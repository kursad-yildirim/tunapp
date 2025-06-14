use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "tunapp")]
#[command(about = "SSH tunnel application")]
#[command(version = "0.1.0")]
pub struct Args {
    #[arg(short, long, help = "SSH host/IP address")]
    pub host: String,

    #[arg(short, long, default_value = "22", help = "SSH port")]
    pub port: u16,

    #[arg(short, long, help = "SSH username")]
    pub username: String,

    #[arg(short, long, help = "Path to private key file")]
    pub key: String,

    #[arg(short, long, default_value = "443", help = "Local port to bind")]
    pub local_port: u16,

    #[arg(short, long, help = "Remote host IP for tunneling")]
    pub remote_host: String,

    #[arg(short = 'P', long, default_value = "443", help = "Remote port for tunneling")]
    pub remote_port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TunnelConfig {
    pub ssh_host: String,
    pub ssh_port: u16,
    pub username: String,
    pub private_key_path: String,
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
}

impl From<Args> for TunnelConfig {
    fn from(args: Args) -> Self {
        TunnelConfig {
            ssh_host: args.host,
            ssh_port: args.port,
            username: args.username,
            private_key_path: args.key,
            local_port: args.local_port,
            remote_host: args.remote_host,
            remote_port: args.remote_port,
        }
    }
}

impl TunnelConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.ssh_host.is_empty() {
            return Err("SSH host cannot be empty".to_string());
        }
        
        if self.username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        
        if self.private_key_path.is_empty() {
            return Err("Private key path cannot be empty".to_string());
        }
        
        if self.remote_host.is_empty() {
            return Err("Remote host cannot be empty".to_string());
        }
        
        if !std::path::Path::new(&self.private_key_path).exists() {
            return Err(format!("Private key file does not exist: {}", self.private_key_path));
        }
        
        Ok(())
    }
}