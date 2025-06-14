mod ssh;
mod config;

use clap::Parser;
use config::{Args, TunnelConfig};
use ssh::SshClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = TunnelConfig::from(args);

    if let Err(e) = config.validate() {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    }

    println!("Connecting to {}@{}:{}", config.username, config.ssh_host, config.ssh_port);
    println!("Creating tunnel: 127.0.0.1:{} -> {}:{}", 
             config.local_port, config.remote_host, config.remote_port);

    let mut client = SshClient::connect(
        &config.ssh_host,
        config.ssh_port,
        &config.username,
        &config.private_key_path,
    ).await?;

    println!("SSH connection established successfully");
    
    client.create_tunnel(
        config.local_port,
        &config.remote_host,
        config.remote_port,
    ).await?;

    Ok(())
}
