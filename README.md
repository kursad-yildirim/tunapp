# TunApp - SSH Tunnel Application

A command-line SSH tunneling application written in Rust that creates secure SSH tunnels for port forwarding.

## Features

- SSH connection with private key authentication
- Local-to-remote port forwarding tunnel creation
- Automatic certificate acceptance (no host key verification)
- Cross-platform support (Linux, Windows, macOS)
- Single binary deployment with no dependencies
- Support for OpenSSH key formats (RSA, Ed25519, ECDSA)

## Prerequisites

### All Platforms
- SSH private key file (OpenSSH format)
- Network access to SSH server and target host

### For Building from Source
- Rust toolchain (1.70+ recommended)
- Cargo package manager

## Installation

### Option 1: Build from Source
```bash
git clone <repository>
cd tunapp
cargo build --release
```

### Option 2: Download Pre-built Binaries
Download the appropriate binary for your platform from the releases page.

## Usage

### Basic Syntax
```bash
tunapp --host <SSH_HOST> --username <USERNAME> --key <PRIVATE_KEY_PATH> --remote-host <REMOTE_IP> [OPTIONS]
```

### Required Arguments

- `--host, -h`: SSH server host/IP address to connect to
- `--username, -u`: SSH username for authentication
- `--key, -k`: Path to SSH private key file
- `--remote-host, -r`: Target host IP for tunneling (can be different from SSH host)

### Optional Arguments

- `--port, -p`: SSH server port (default: 22)
- `--local-port, -l`: Local port to bind and listen on (default: 443)
- `--remote-port, -P`: Target host port for tunneling (default: 443)

### Platform-Specific Examples

#### Linux/macOS
```bash
# Basic tunnel
./tunapp --host 192.168.1.100 --username admin --key ~/.ssh/id_rsa --remote-host 10.0.0.50

# Custom ports
./tunapp --host ssh.example.com --username user --key /home/user/.ssh/id_ed25519 \
         --remote-host 192.168.10.20 --local-port 8443 --remote-port 443 --port 2222
```

#### Windows (Command Prompt)
```cmd
tunapp.exe --host 192.168.1.100 --username admin --key C:\Users\Admin\.ssh\id_rsa --remote-host 10.0.0.50
```

#### Windows (PowerShell)
```powershell
.\tunapp.exe --host 192.168.1.100 --username admin --key $env:USERPROFILE\.ssh\id_rsa --remote-host 10.0.0.50
```

### Use Case Example

Creating a tunnel to access a web service running on an internal server:

```bash
tunapp --host bastion.company.com --username jumpuser --key ~/.ssh/company_key \
       --remote-host 10.internal.company.com --local-port 8080 --remote-port 80
```

This creates a tunnel where:
1. Connects to SSH server at `bastion.company.com:22`
2. Binds to local port `8080`
3. Forwards traffic to `10.internal.company.com:80`
4. Access via `http://localhost:8080` in your browser

## Building

### Native Build
```bash
cargo build --release
```
Binary location: `target/release/tunapp` (Linux/macOS) or `target/release/tunapp.exe` (Windows)

### Cross-Compilation

#### From Linux to Other Platforms

**For Windows:**
```bash
# Install target
rustup target add x86_64-pc-windows-gnu

# Install cross-compiler (Ubuntu/Debian)
sudo apt install gcc-mingw-w64-x86-64

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

**For macOS:**
```bash
# Install target
rustup target add x86_64-apple-darwin

# Build (requires macOS SDK)
cargo build --release --target x86_64-apple-darwin
```

#### From macOS to Other Platforms

**For Linux:**
```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

**For Windows:**
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

#### From Windows to Other Platforms

**For Linux:**
```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

**For macOS:**
```bash
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

### Binary Locations After Cross-Compilation
- Windows: `target/x86_64-pc-windows-gnu/release/tunapp.exe`
- Linux: `target/x86_64-unknown-linux-gnu/release/tunapp`
- macOS: `target/x86_64-apple-darwin/release/tunapp`

## SSH Key Formats

Supported private key formats:
- OpenSSH (recommended): `ssh-keygen -t ed25519` or `ssh-keygen -t rsa`
- PKCS#8
- PKCS#1 (RSA)

### Key Generation Examples
```bash
# Ed25519 (recommended)
ssh-keygen -t ed25519 -f ~/.ssh/tunapp_key

# RSA 4096-bit
ssh-keygen -t rsa -b 4096 -f ~/.ssh/tunapp_key

# ECDSA
ssh-keygen -t ecdsa -b 256 -f ~/.ssh/tunapp_key
```

## Troubleshooting

### Common Issues

**Permission denied (publickey)**
- Ensure the private key file has correct permissions (600 on Unix)
- Verify the corresponding public key is in `~/.ssh/authorized_keys` on the SSH server
- Check SSH server logs for authentication failures

**Connection refused**
- Verify SSH server is running and accessible
- Check firewall rules on both client and server
- Ensure correct SSH port (default 22)

**Address already in use**
- Another process is using the local port
- Use a different `--local-port` value
- Check with `netstat -tulpn | grep <port>` (Linux) or `netstat -an | findstr <port>` (Windows)

### Platform-Specific Notes

**Linux:**
- May require `sudo` for ports < 1024
- Check SELinux/AppArmor policies if connection issues occur

**Windows:**
- Windows Defender may require exception for the binary
- Use forward slashes or double backslashes in paths: `C:\\Users\\...` or `C:/Users/...`

**macOS:**
- May require adding binary to Security & Privacy settings
- Use absolute paths for key files

## Security Considerations

⚠️ **Important Security Notes:**

1. **Host Key Verification Disabled**: This application automatically accepts all SSH host keys without verification
2. **Use Case**: Suitable for controlled environments, internal networks, or temporary tunnels
3. **Not Recommended**: Production environments without proper security controls
4. **Key Security**: Protect private key files with appropriate permissions (600/rw-------)
5. **Network Security**: Use only on trusted networks

## License

[Add your license information here]

## Contributing

[Add contribution guidelines here]