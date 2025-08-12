# SSH Client Documentation 🖥️

Complete documentation for the Secure Shell Client implementation in Rust.

## 📋 Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Features](#features)
- [API Reference](#api-reference)

## 🎯 Overview

The SSH client provides a secure, interactive terminal interface for connecting to the SSH server. Built with Rust's async capabilities, it features TLS encryption, colored output formatting, and cross-platform support.

## 🏗️ Architecture


### Module Structure

```
client/src/
├── main.rs                        # Client entry point
├── client.rs                      # Core client implementation
├── client_configure.rs            # Configuration management
└── response_handlers/
    └── response_formatter.rs      # Response formatting and display
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

### Building the Client

```bash
cd my_ssh/client
cargo build --release
```

## 📖 Usage

### Starting the Client

```bash
# Run the client
cd my_ssh/client
cargo run --release
```

### Connection Process

1. **Certificate Configuration**: Enter path to server certificate
2. **Server Address**: Enter server IP and port (e.g., `192.168.1.100:12345`)
3. **TLS Handshake**: Client verifies server certificate and establishes secure connection
4. **Authentication**: Login using `login username password`
5. **Interactive Session**: Execute commands in the secure shell

### Authentication

```bash
# After connecting, authenticate
login alice mypassword

```

### Command Usage

```bash
# File operations
ls                           # List current directory
ls /path/to/directory       # List specific directory
cat filename.txt            # Display file contents
mkdir new_folder            # Create directory
rm file.txt                 # Remove file
mv old_name new_name        # Move/rename file

# Navigation
cd /path/to/directory       # Change directory
cd ..                       # Go to parent directory
cd                          # Go to home directory

# Text operations
echo "Hello World"          # Display text
grep "pattern" file.txt     # Search in file
wc file.txt                 # Count words/lines/characters

# System operations
users                       # List active users
clear                       # Clear screen (local)
exit                        # Disconnect from server
```

### Command Chaining

```bash
# Execute multiple commands
ls && echo "Listed successfully" || echo "Failed to list"

# Pipe operations
cat file.txt | grep "pattern"

# Output redirection
echo "Hello" > output.txt

# Input redirection
wc < input.txt

# Sequential execution
mkdir test; cd test; echo "In test directory"
```

## ✨ Features

### 🎨 Color-Coded Output

The client automatically formats server responses with colors:

- **Directory Listings**: Structured formatting with file/directory indicators
- **Search Results**: Highlighted search patterns
- **Normal Output**: Clean, readable text formatting

### 🖥️ Cross-Platform Support

```rust
fn clear_console() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}
```

### 📱 Interactive Features

- **Real-time Response**: Immediate server response display
- **Path Tracking**: Current directory shown in prompt

### 🔧 Local Commands

| Command | Description |
|---------|-------------|
| `clear` or `cls` | Clear client terminal |
| `exit` | Disconnect and quit |


## 🎨 Response Formatting

### Formatting Codes

The client interprets special formatting codes from the server:

| Code | Purpose | Example |
|------|---------|---------|
| `?&E` | Error messages | `?&EFile not found` |
| `?&L` | Directory listings | `?&Lfile1.txt\nfolder1/` |
| `?&C` | Colored output | `?&C^@Error~~^#Success~~` |
| `?&N` | Normal text | `?&NHello World` |



**Author**: punctIT  
**Version**: 1.0.0  
**Last Updated**: August 12, 2025