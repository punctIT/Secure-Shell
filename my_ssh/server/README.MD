# SSH Server Documentation 🖧

Complete documentation for the Secure Shell Server implementation in Rust.

## 📋 Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Installation](#installation)
- [Configuration](#configuration)
- [Authentication](#authentication)
- [Command System](#command-system)
- [Security](#security)
- [Deployment](#deployment)

## 🎯 Overview

The SSH server is built using Rust's async capabilities and provides secure remote shell access with TLS encryption. It supports multiple concurrent users, comprehensive command execution, and robust authentication.

## 🏗️ Architecture

### Module Structure

```
server/src/
├── main.rs                          # Server entry point
├── secure_shell_server.rs           # Core server implementation
├── server_configure.rs              # Configuration management
├── login.rs                         # Authentication system
└── command_system/
    ├── common.rs                    # Command parsing utilities
    ├── command_handler.rs           # Command execution handler
    └── commands/                    # Individual command implementations
        ├── mod.rs
        ├── echo.rs                  # Echo command
        ├── users.rs                 # User management
        ├── list_files.rs            # Directory listing
        ├── change_directory.rs      # Directory navigation
        ├── concatenate.rs           # File content display
        ├── executable_files.rs      # Executable file operations
        ├── global_regular_expresion_print.rs  # grep implementation
        ├── make_director.rs         # Directory creation
        ├── move_class.rs            # File/directory moving
        ├── remove_director.rs       # Directory removal
        ├── remove_file.rs           # File removal
        └── word_count.rs            # Word count utility
```

## 🚀 Installation

### Prerequisites

```bash
# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install OpenSSL development libraries
# Ubuntu/Debian:
sudo apt-get install libssl-dev

# CentOS/RHEL:
sudo yum install openssl-devel

# macOS:
brew install openssl
```

### Building the Server

```bash
cd my_ssh/server
cargo build --release
```

## ⚙️ Configuration

### Server Configuration Structure

```rust
pub struct Configure {
    cert_path: String,        // Path to TLS certificate
    cert_key_path: String,    // Path to private key
    working_directory: String, // Server root directory
    password_file: String,    // User credentials file
}
```

### Configuration Methods

```rust
impl Configure {
    pub fn new() -> Self                           // Create new configuration
    pub fn set_cert_path(&self) -> String          // Set certificate path
    pub fn set_cert_key_path(&self) -> String      // Set private key path
    pub fn set_working_directory(&self) -> String  // Set working directory
    pub fn set_password_file(&self) -> String      // Set password file path
}
```

### Generate TLS Certificates

```bash
# Generate self-signed certificate for testing
openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -nodes

# For production, use CA-signed certificates
# Example with Let's Encrypt:
certbot certonly --standalone -d your-domain.com
```

### Password File Format

Create a text file with username and BCrypt hashed passwords:

```
alice $2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8.lwP.CL0PGQF5.2A0y
bob $2b$12$KQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8.lwP.CL0PGQF5.2A0y
charlie $2b$12$MQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8.lwP.CL0PGQF5.2A0y
```

Generate BCrypt hashes:

```bash
# Using Python
python3 -c "import bcrypt; print(bcrypt.hashpw(b'password', bcrypt.gensalt()).decode())"

# Using online tools (for testing only)
# https://bcrypt-generator.com/
```

## 🔐 Authentication

### Authentication Flow

```rust
impl UserLogin {
    pub async fn get_login_status(&self) -> Result<String, String> {
        let cmd = get_commands(self.input.clone());
        let pass_map = self.get_hashmap_password();
        
        if cmd[0].cmd[0] == "login" {
            if cmd[0].cmd.len() == 3 {
                let username = cmd[0].cmd[1].clone();
                if let Some(stored_hash) = pass_map.get(&username) {
                    let password = cmd[0].cmd[2].clone();
                    if verify(password, stored_hash).unwrap_or(false) {
                        // Check for duplicate login
                        let users_lock = self.users_list.read().await;
                        let user_exists = users_lock
                            .iter()
                            .any(|user_entry| user_entry.contains(&username));

                        if user_exists {
                            Err("User already logged in".to_string())
                        } else {
                            Ok(username)
                        }
                    } else {
                        Err("Incorrect password".to_string())
                    }
                } else {
                    Err("Invalid username".to_string())
                }
            } else {
                Err("Invalid format: login [USERNAME] [PASSWORD]".to_string())
            }
        } else {
            Err("You must login first".to_string())
        }
    }
}
```

### Security Features

- **BCrypt Hashing**: Passwords are hashed using BCrypt with salt
- **Duplicate Login Prevention**: Users cannot login multiple times
- **Session Isolation**: Each user session is isolated
- **Secure Password Storage**: Passwords are never stored in plaintext

## 🔧 Command System

### Available Commands

| Command | Description | Implementation |
|---------|-------------|----------------|
| `echo` | Display text | [echo.rs](src/command_system/commands/echo.rs) |
| `ls` | List directory contents | [list_files.rs](src/command_system/commands/list_files.rs) |
| `cd` | Change directory | [change_directory.rs](src/command_system/commands/change_directory.rs) |
| `cat` | Display file contents | [concatenate.rs](src/command_system/commands/concatenate.rs) |
| `mkdir` | Create directory | [make_director.rs](src/command_system/commands/make_director.rs) |
| `rmdir` | Remove directory | [remove_director.rs](src/command_system/commands/remove_director.rs) |
| `rm` | Remove file | [remove_file.rs](src/command_system/commands/remove_file.rs) |
| `mv` | Move/rename files | [move_class.rs](src/command_system/commands/move_class.rs) |
| `grep` | Search in files | [global_regular_expresion_print.rs](src/command_system/commands/global_regular_expresion_print.rs) |
| `wc` | Word count | [word_count.rs](src/command_system/commands/word_count.rs) |
| `users` | List active users | [users.rs](src/command_system/commands/users.rs) |

### Command Structure

```rust
#[derive(Clone, Debug)]
pub struct Command {
    pub cmd: Vec<String>,    // Command and arguments
    pub op: Option<String>,  // Operator for chaining (&&, ||, ;, |, >, <)
}
```

### Command Parser

```rust
pub fn get_commands(client_input: String) -> Vec<Command> {
    let mut cmds: Vec<Command> = Vec::new();
    let op = ["&&", "|", "||", "<", ">", ";"];

    let parsed = match split(client_input.trim()) {
        Ok(v) => v,
        Err(_) => return cmds,
    };

    let mut current_cmd: Vec<String> = Vec::new();
    for token in parsed {
        if op.contains(&token.as_str()) {
            cmds.push(Command {
                cmd: current_cmd.clone(),
                op: Some(token),
            });
            current_cmd.clear();
        } else {
            current_cmd.push(token);
        }
    }

    if !current_cmd.is_empty() {
        cmds.push(Command {
            cmd: current_cmd,
            op: None,
        });
    }

    cmds
}
```

### Response Formatting

The server uses a custom formatting protocol:

```rust
pub enum Format {
    Error,           // ?&E - Error messages
    ListDir,         // ?&L - List formatting
    Color(&'static str), // Color codes
    Split,           // \n\n - Split sections
    Normal,          // ?&N - Normal text
    NormalColored,   // ?&C - Colored output
}
```

Color codes:
- `^!` - Blue text
- `^@` - Red text (with `~~` to stop)
- `^#` - Green text
- `~~` - Stop color formatting

## 🔒 Security

### TLS Configuration

### Security Measures

- **TLS 1.3 Encryption**: All communication is encrypted
- **Perfect Forward Secrecy**: Each session uses unique keys
- **Certificate Verification**: Client must verify server certificate
- **Path Validation**: All file operations are validated against root directory

### Session Management

```rust
// User tracking for concurrent sessions
users: Arc<RwLock<Vec<String>>>

// Session isolation
let mut server_path = root_path.clone();
let mut user: Option<String> = None;

// Cleanup on disconnect
users.write().await.retain(|u| u != &username);
```

## 🚀 Deployment

### Running the Server

```bash
# Development mode
cargo run

# Production mode
cargo run --release

```

---

**Author**: punctIT  
**Version**: 1.0.0  
**Last Updated**: August 12, 2025