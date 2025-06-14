use anyhow::{Result, Context};
use russh::*;
use russh::client::Msg;
use russh_keys::*;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::pin::Pin;
use std::future::Future;

pub struct SshClient {
    session: russh::client::Handle<Client>,
}

struct Client;

impl russh::client::Handler for Client {
    type Error = russh::Error;

    fn check_server_key<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        _server_public_key: &'life1 key::PublicKey,
    ) -> Pin<Box<dyn Future<Output = Result<bool, Self::Error>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move { Ok(true) })
    }
}

impl SshClient {
    pub async fn connect(
        host: &str,
        port: u16,
        username: &str,
        private_key_path: &str,
    ) -> Result<Self> {
        let config = russh::client::Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
            ..Default::default()
        };

        let config = Arc::new(config);
        let mut session = russh::client::connect(config, (host, port), Client).await?;

        let key_pair = load_secret_key(private_key_path, None)
            .context("Failed to load private key")?;

        let auth_res = session
            .authenticate_publickey(username, Arc::new(key_pair))
            .await?;

        if !auth_res {
            anyhow::bail!("Authentication failed");
        }

        Ok(SshClient { session })
    }

    pub async fn create_tunnel(
        &mut self,
        local_port: u16,
        remote_host: &str,
        remote_port: u16,
    ) -> Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", local_port)).await?;
        println!("Tunnel created: 127.0.0.1:{} -> {}:{}", local_port, remote_host, remote_port);

        loop {
            let (local_stream, _) = listener.accept().await?;
            let remote_host = remote_host.to_string();
            
            // Create a new channel for each connection
            let channel = self.session
                .channel_open_direct_tcpip(&remote_host, remote_port as u32, "127.0.0.1", 0)
                .await?;

            tokio::spawn(async move {
                if let Err(e) = handle_tunnel_connection(local_stream, channel).await {
                    eprintln!("Tunnel connection error: {}", e);
                }
            });
        }
    }
}

async fn handle_tunnel_connection(
    mut local_stream: TcpStream,
    mut channel: Channel<Msg>,
) -> Result<()> {
    let (mut local_read, mut local_write) = local_stream.split();
    let mut local_to_remote_buf = vec![0u8; 8192];
    
    loop {
        tokio::select! {
            // Read from local and write to remote
            result = local_read.read(&mut local_to_remote_buf) => {
                match result {
                    Ok(0) => {
                        // Local connection closed, close remote
                        let _ = channel.eof().await;
                        break;
                    }
                    Ok(n) => {
                        if let Err(e) = channel.data(&local_to_remote_buf[..n]).await {
                            eprintln!("Error sending data to remote: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from local: {}", e);
                        break;
                    }
                }
            }
            
            // Read from remote and write to local
            msg = channel.wait() => {
                match msg {
                    Some(ChannelMsg::Data { data }) => {
                        if let Err(e) = local_write.write_all(&data).await {
                            eprintln!("Error writing to local: {}", e);
                            break;
                        }
                    }
                    Some(ChannelMsg::Eof) => {
                        // Remote connection closed
                        break;
                    }
                    Some(ChannelMsg::Close) => {
                        // Channel closed
                        break;
                    }
                    None => break,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}