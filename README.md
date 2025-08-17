# Secure Shell in Rust 🦀🔒

A secure shell implementation written in Rust that provides TLS-encrypted remote shell access with user authentication and command execution capabilities.

> **⚠️ Educational Purpose Disclaimer**  
> This project is developed for **educational and learning purposes only**. It demonstrates secure communication protocols, authentication systems, and system programming concepts in Rust. **Do not use in production environments.**

> **🔗 Companion Project**  
> This Rust server is **specifically designed** to work with the custom PyQt6 GUI client also included in this repository. Both components use a **proprietary communication protocol** and form a complete educational SSH ecosystem.
> 
## 🎯 Overview

This project implements a secure shell server and client using Rust's async capabilities and TLS encryption. The system provides:

- **Secure Communication**: All communication encrypted using TLS 1.3
- **User Authentication**: BCrypt-hashed password authentication
- **Command Execution**: Support for various Unix-like commands
- **Session Management**: Multi-user concurrent session support
- **Formatted Output**: Custom response formatting with color support
- **GUI Integration**: Seamless communication with the PyQt6 client interface

## 📁 Project Structure

```
Secure-Shell/
├── my_ssh/                            # Rust implementation
│   ├── client/
│   │   ├── src/
│   │   │   ├── main.rs                # Client entry point
│   │   │   ├── client.rs              # Core client implementation
│   │   │   ├── client_configure.rs    # Client configuration
│   │   │   └── response_handlers/
│   │   │       └── response_formatter.rs # Response formatting and display
│   │   ├── Cargo.toml
│   │   └── README.md                  # Client documentation
│   └── server/
│       ├── src/
│       │   ├── main.rs                # Server entry point
│       │   ├── secure_shell_server.rs # Core server implementation
│       │   ├── server_configure.rs    # Server configuration
│       │   ├── login.rs               # Authentication system
│       │   └── command_system/
│       │       ├── common.rs          # Command parsing utilities
│       │       ├── command_handler.rs # Command execution handler
│       │       └── commands/          # Individual command implementations
│       │           ├── mod.rs
│       │           ├── echo.rs
│       │           ├── users.rs
│       │           ├── list_files.rs
│       │           ├── change_directory.rs
│       │           ├── concatenate.rs
│       │           ├── executable_files.rs
│       │           ├── global_regular_expresion_print.rs
│       │           ├── make_director.rs
│       │           ├── move_class.rs
│       │           ├── remove_director.rs
│       │           ├── remove_file.rs
│       │           └── word_count.rs
│       ├── Cargo.toml
│       └── README.md                  # Server documentation
└── client_GUI-PyQt6/                  # Python GUI client (companion project)
    ├── main.py                        # GUI entry point
    ├── backend/
    │   └── client.py                  # TLS client for GUI
    ├── graphic_user_interface/        # PyQt6 interface components
    │   ├── client_gui.py              # Main window controller
    │   ├── windows/
    │   │   ├── connect_window.py      # Server connection interface
    │   │   ├── login_window.py        # User authentication interface
    │   │   └── secure_shell/
    │   │       ├── secure_shell_window.py # Main SSH interface
    │   │       ├── top_menu.py        # Navigation and user menu
    │   │       ├── left_menu.py       # Quick access menu
    │   │       ├── file_area.py       # File browser interface
    │   │       ├── content_menu.py    # File content viewer
    │   │       └── console.py         # Interactive command console
    │   ├── styles/                    # CSS styling files
    │   │   ├── connect_window.css     # Connection interface styling
    │   │   ├── login_window.css       # Authentication interface styling
    │   │   └── ssh_window.css         # Main SSH interface styling
    │   └── Assets/
    │       └── Icons/                 # Application icons
    └── images-git-readme/             # Visual documentation
```

## ✨ Features

### 🔐 Security Features
- **TLS 1.3 Encryption**: All communication encrypted end-to-end
- **Certificate-based Authentication**: Server verification using X.509 certificates
- **BCrypt Password Hashing**: Secure password storage and verification
- **Custom Protocol**: Proprietary secure communication protocol

### 🖥️ Command Line Client Features
- **Interactive Shell**: Full interactive command-line interface
- **Color-coded Output**: Enhanced readability with colored responses
- **Cross-platform Support**: Works on Windows, Linux, and macOS

### 🖧 Server Features
- **Concurrent Connections**: Multiple simultaneous user sessions
- **User Management**: Multi-user support with individual authentication
- **Command Execution**: Comprehensive command set implementation
- **Path Management**: Secure file system navigation
- **Resource Management**: Efficient memory and connection handling

### 🎨 GUI Client Integration
- **Modern Interface**: Beautiful PyQt6-based graphical interface
- **File Browser**: Visual file system navigation
- **Interactive Console**: Embedded terminal for command execution
- **User Management**: Active user monitoring and session control

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ with Cargo
- Python 3.7+ (for GUI client)
- PyQt6 (for GUI client)
- OpenSSL development libraries
- TLS certificates (self-signed or CA-issued)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/punctIT/Secure-Shell.git
cd Secure-Shell

# Build Rust server and client
cd my_ssh/server && cargo build --release
cd ../client && cargo build --release

# Install GUI client dependencies
cd ../../client_GUI-PyQt6
pip install PyQt6
```

### Generate Test Certificates

```bash
# Generate self-signed certificate (for testing)
openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -nodes
```

### Running the System

```bash
# 1. Start the Rust server
cd my_ssh/server
cargo run --release

# 2. Use either client:

# Option A: Command-line client
cd ../client
cargo run --release

# Option B: GUI client 
cd ../../client_GUI-PyQt6
python main.py
```

## 📸 Visual Demonstration

### GUI Client Workflow

#### Connection Setup
![Connection Process](client_GUI-PyQt6/images-git-readme/connect.gif)
*Secure server connection with certificate validation*

#### User Authentication
![Login Process](client_GUI-PyQt6/images-git-readme/login.gif)
*User authentication with password visibility controls*

#### File Management & Console
![SSH Interface](client_GUI-PyQt6/images-git-readme/ssh.gif)
*Complete file system navigation and interactive command execution*

## 📚 Documentation

### 🖧 [Server Documentation](./my_ssh/server/README.MD)
Comprehensive guide for the Rust server implementation

### 🖥️ [Command Line Client Documentation](./my_ssh/client/README.MD)
Details for the Rust terminal client

### 🎨 [GUI Client Documentation](./client_GUI-PyQt6/README.md)
Complete guide for the PyQt6 graphical interface

## 🎓 Educational Goals

This project serves as a comprehensive learning resource for:

### 🦀 Rust Programming Concepts
- **Async Programming**: Using `tokio` for asynchronous I/O operations
- **Concurrency**: Safe concurrent programming with `Arc<RwLock<T>>`
- **Error Handling**: Robust error handling with `Result<T, E>`
- **Module System**: Organizing large projects with Rust's module system
- **Memory Safety**: Rust's ownership system in network programming

### 🖧 Network Programming
- **TCP Socket Programming**: Low-level network communication
- **Protocol Design**: Custom application-layer protocol implementation
- **Client-Server Architecture**: Building scalable network applications
- **Connection Management**: Handling multiple concurrent connections
- **TLS Integration**: Implementing secure communication protocols

### 🎨 GUI Development
- **PyQt6 Framework**: Modern desktop application development
- **Event-Driven Programming**: Handling user interactions and network events
- **Cross-Language Communication**: Python client with Rust server
- **User Experience Design**: Creating intuitive interfaces for technical tools

### 🔒 Security Concepts
- **Cryptographic Protocols**: TLS/SSL implementation and best practices
- **Authentication Systems**: Secure user verification methods
- **Certificate Management**: Public key infrastructure concepts

## 📦 Dependencies

### Rust Server/Client
- `tokio` - Async runtime
- `tokio-rustls` - TLS implementation
- `serde` - Serialization framework
- `bcrypt` - Password hashing
- `colored` - Terminal colors

### Python GUI Client
- `PyQt6` - Modern GUI framework
- `ssl` - TLS/SSL support
- `threading` - Background operations

**Author**: punctIT  
**Version**: 1.0.0  
**Last Updated**: August 17, 2025
