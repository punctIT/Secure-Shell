# Secure Shell in Rust 🦀🔒

A secure shell implementation written in Rust that provides TLS-encrypted remote shell access with user authentication and command execution capabilities.

> **⚠️ Educational Purpose Disclaimer**  
> This project is developed for **educational and learning purposes only**. It demonstrates secure communication protocols, authentication systems, and system programming concepts in Rust. 

## 📋 Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Features](#features)
- [Quick Start](#quick-start)
- [Documentation](#documentation)

## 🎯 Overview

This project implements a secure shell server and client using Rust's async capabilities and TLS encryption. The system provides:

- **Secure Communication**: All communication encrypted using TLS 1.3
- **User Authentication**: BCrypt-hashed password authentication
- **Command Execution**: Support for various Unix-like commands
- **Session Management**: Multi-user concurrent session support
- **Formatted Output**: Custom response formatting with color support

## 📁 Project Structure

```
my_ssh/
├── client/
│   ├── src/
│   │   ├── main.rs                    # Client entry point
│   │   ├── client.rs                  # Core client implementation
│   │   ├── client_configure.rs        # Client configuration
│   │   └── response_handlers/
│   │       └── response_formatter.rs  # Response formatting and display
│   ├── Cargo.toml
│   └── README.md                      # Client documentation
└── server/
    ├── src/
    │   ├── main.rs                    # Server entry point
    │   ├── secure_shell_server.rs     # Core server implementation
    │   ├── server_configure.rs        # Server configuration
    │   ├── login.rs                   # Authentication system
    │   └── command_system/
    │       ├── common.rs              # Command parsing utilities
    │       ├── command_handler.rs     # Command execution handler
    │       └── commands/              # Individual command implementations
    │           ├── mod.rs
    │           ├── echo.rs
    │           ├── users.rs
    │           ├── list_files.rs
    │           ├── change_directory.rs
    │           ├── concatenate.rs
    │           ├── executable_files.rs
    │           ├── global_regular_expresion_print.rs
    │           ├── make_director.rs
    │           ├── move_class.rs
    │           ├── remove_director.rs
    │           ├── remove_file.rs
    │           └── word_count.rs
    ├── Cargo.toml
    └── README.md                      # Server documentation
```

## ✨ Features

### 🔐 Security Features
- **TLS 1.3 Encryption**: All communication encrypted end-to-end
- **Certificate-based Authentication**: Server verification using X.509 certificates
- **BCrypt Password Hashing**: Secure password storage and verification

### 🖥️ Client Features
- **Interactive Shell**: Full interactive command-line interface
- **Color-coded Output**: Enhanced readability with colored responses
- **Cross-platform Support**: Works on Windows, Linux, and macOS

### 🖧 Server Features
- **Concurrent Connections**: Multiple simultaneous user sessions
- **User Management**: Multi-user support with individual authentication
- **Command Execution**: Comprehensive command set implementation
- **Path Management**: Secure file system navigation
- **Resource Management**: Efficient memory and connection handling

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ with Cargo
- OpenSSL development libraries
- TLS certificates (self-signed or CA-issued)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/punctIT/Secure-Shell-in-Rust.git
cd Secure-Shell-in-Rust

# Build both server and client
cd my_ssh/server && cargo build --release
cd ../client && cargo build --release
```

### Generate Test Certificates

```bash
# Generate self-signed certificate (for testing)
openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -nodes
```

## 📚 Documentation

### 🖧 [Server Documentation](./my_ssh/server/README.MD)

### 🖥️ [Client Documentation](./my_ssh/client/README.md)

## 🎓 Educational Goals

This project serves as a comprehensive learning resource for:

### 🦀 Rust Programming Concepts
- **Async Programming**: Using `tokio` for asynchronous I/O operations
- **Concurrency**: Safe concurrent programming with `Arc<RwLock<T>>`
- **Error Handling**: Robust error handling with `Result<T, E>`
- **Module System**: Organizing large projects with Rust's module system

### 🖧 Network Programming
- **TCP Socket Programming**: Low-level network communication
- **Protocol Design**: Custom application-layer protocol implementation
- **Client-Server Architecture**: Building scalable network applications
- **Connection Management**: Handling multiple concurrent connections



**Author**: punctIT  
**Version**: 1.0.0  

**Last Updated**: August 12, 2025
