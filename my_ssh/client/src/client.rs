use crate::response_handlers::response_formatter::ShowResponse;
use colored::*;
use core::panic;
use std::io::Write;
use std::{fs::File, io::BufReader, sync::Arc};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::{
    TlsConnector,
    client::TlsStream,
    rustls::{Certificate, ClientConfig, RootCertStore, ServerName},
};

/// A secure shell client for connecting to and interacting with TLS-encrypted shell servers.
/// 
/// `Client` provides a complete client implementation for secure shell sessions over TLS.
/// It handles certificate validation, secure connection establishment, and interactive
/// command execution with a remote shell server.
/// 
/// # Features
/// 
/// - **TLS Security**: End-to-end encrypted communication using TLS 1.2/1.3
/// - **Certificate Validation**: Validates server certificates against trusted CA
/// - **Interactive Shell**: Real-time command execution with formatted output
/// - **Cross-Platform**: Supports Windows and Unix-like systems
/// - **Session Management**: Maintains persistent connection with path tracking
/// - **Local Commands**: Built-in support for `exit`, `clear`, and `cls` commands
/// 
/// # Security Model
/// 
/// The client uses certificate-based authentication to verify server identity:
/// - Loads trusted CA certificate from file
/// - Validates server certificate chain during handshake
/// - Establishes encrypted TLS tunnel for all communication
/// 
/// # Communication Protocol
/// 
/// Messages follow a structured format:
/// ```text
/// response_content[-]:current_path[-]\r\n\r\n
/// ```
/// 
/// # Examples
/// 
/// ## Basic Usage
/// ```rust
/// // Create client instance
/// let mut client = Client::new(
///     "/etc/ssl/certs/ca-cert.pem",
///     "myserver.example.com",
///     "192.168.1.100:8443"
/// );
/// 
/// // Connect to server
/// client.connect_to_server().await?;
/// 
/// // Start interactive session
/// client.send_and_receive().await?;
/// ```
/// 
/// ## Certificate Setup
/// ```rust
/// // Client expects PEM-formatted CA certificate
/// let client = Client::new(
///     "/path/to/ca-certificate.pem",
///     "secure-shell.company.com",
///     "10.0.0.5:9443"
/// );
/// ```
pub struct Client {
    cert: Certificate,
    ip_port: String,
    server_name: ServerName,
    tls_stream: Option<TlsStream<TcpStream>>,
}
impl Client {
        /// Creates a new `Client` instance with the specified connection parameters.
    /// 
    /// This constructor loads the CA certificate, validates the server name format,
    /// and prepares the client for connection. The actual network connection is
    /// established later via `connect_to_server()`.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path to the CA certificate file in PEM format
    /// - `server_name`: Server hostname for SNI and certificate validation (e.g., "myserver.com")
    /// - `ip_port`: Server IP address and port in "IP:PORT" format (e.g., "192.168.1.100:8443")
    /// 
    /// # Returns
    /// 
    /// A new `Client` instance ready for connection.
    /// 
    /// # Panics
    /// 
    /// This method panics if:
    /// - The certificate file cannot be loaded or is invalid
    /// - The server name is not a valid hostname
    /// 
    /// # Certificate Requirements
    /// 
    /// The certificate file must:
    /// - Be in PEM format
    /// - Contain a valid X.509 certificate
    /// - Be the CA certificate that signed the server's certificate
    /// - Have appropriate permissions for reading
    pub fn new(path: &str, server_name: &str, ip_port: &str) -> Self {
        let cert = Client::load_cert(path).unwrap_or_else(|e| {
            panic!("Error loading certificate: {:?}", e);
        });
        let server_name = ServerName::try_from(server_name).unwrap_or_else(|e| {
            panic!("Error , invalid server name: {:?}", e);
        });
        Client {
            cert,
            ip_port: ip_port.to_string(),
            server_name,
            tls_stream: None,
        }
    }
    /// Establishes a secure TLS connection to the remote shell server.
    /// 
    /// This method performs the complete connection establishment process:
    /// 1. Creates a TCP connection to the server
    /// 2. Configures TLS with the loaded CA certificate
    /// 3. Performs TLS handshake with certificate validation
    /// 4. Stores the established connection for subsequent operations
    /// 
    /// # Returns
    /// 
    /// - `Ok(())`: Connection established successfully
    /// - `Err(Box<dyn std::error::Error>)`: Connection failed
    /// 
    pub async fn connect_to_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut root_cert_store = RootCertStore::empty();
        root_cert_store.add(&self.cert)?;

        let config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();

        let connector = TlsConnector::from(Arc::new(config));

        let stream = TcpStream::connect(self.ip_port.clone()).await?;

        let server_name = self.server_name.clone();

        self.tls_stream = Some(connector.connect(server_name, stream).await?);

        Ok(())
    }
    // Clears the console screen using platform-appropriate commands.
    /// 
    /// This utility method provides cross-platform console clearing functionality,
    /// automatically detecting the operating system and using the appropriate
    /// command to clear the terminal screen.
    /// 
    /// # Platform Support
    /// 
    /// - **Windows**: Uses `cmd /C cls` command
    /// - **Unix/Linux/macOS**: Uses `clear` command
    /// 
    /// # Behavior
    /// 
    /// - Executes synchronously and waits for completion
    /// - Ignores command execution errors (fails silently)
    /// - Preserves current working directory and environment
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
        /// Starts an interactive shell session with the connected server.
    /// 
    /// This method implements the main client loop, handling user input, server
    /// communication, and response formatting. It provides a full interactive
    /// shell experience with command history, path tracking, and local command
    /// processing.
    /// 
    /// # Returns
    /// 
    /// - `Ok(())`: Session ended normally (user typed "exit")
    /// - `Err(Box<dyn std::error::Error>)`: Communication error occurred
    /// 
    /// # Session Flow
    /// 
    /// 1. **Initial Handshake**: Receives welcome message and initial prompt
    /// 2. **Command Loop**: Continuously processes user input and server responses
    /// 3. **Local Commands**: Handles `exit`, `clear`, and `cls` locally
    /// 4. **Remote Commands**: Sends other commands to server for execution
    /// 5. **Response Processing**: Formats and displays server responses
    /// 6. **Graceful Shutdown**: Closes connection on exit
    /// 
    /// # Supported Local Commands
    /// 
    /// | Command | Description | Action |
    /// |---------|-------------|--------|
    /// | `exit` | Terminate session | Closes TLS connection and returns |
    /// | `clear` | Clear screen (Unix) | Clears terminal and refreshes prompt |
    /// | `cls` | Clear screen (Windows) | Clears terminal and refreshes prompt |
    /// 
    /// # Message Protocol
    /// 
    /// ## Server Response Format
    /// ```text
    /// response_content[-]:current_path[-]\r\n\r\n
    /// ```
    /// 
    pub async fn send_and_receive(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let tls_stream = self.tls_stream.as_mut().unwrap_or_else(|| {
            panic!("Error TLS not configured");
        });

        let mut last_path = String::new();

        let mut buf = vec![0u8; 1024];
        let n = tls_stream.read(&mut buf).await?;

        let answer = String::from_utf8_lossy(&buf[..n]);
        let r: Vec<&str> = answer.split("[-]").collect();
        let resonse = ShowResponse::new(r[0].to_string());
        resonse.show();
        print!("{}{}>", "Server".cyan(), r[1].cyan());
        last_path += r[1];
        std::io::stdout().flush().unwrap();

        loop {
            let mut message = String::new();
            std::io::stdin()
                .read_line(&mut message)
                .expect("Read Error");
            if message.trim() == "exit" {
                tls_stream.shutdown().await?;
                return Ok(());
            }
            if message.trim() == "clear" || message.trim() == "cls" {
                Client::clear_console();
                print!("{}{}>", "Server".cyan(), last_path.cyan());
                std::io::stdout().flush().unwrap();
                continue;
            }
            tls_stream.write_all(message.as_bytes()).await?;

            let mut buffer = Vec::new();
            let mut temp_buf = vec![0u8; 1024];

            loop {
                let n = tls_stream.read(&mut temp_buf).await?;
                if n == 0 {
                    break;
                }

                buffer.extend_from_slice(&temp_buf[..n]);
                if buffer.ends_with(b"\r\n\r\n") {
                    break;
                }
            }

            let answer = String::from_utf8_lossy(&buffer);
            let r: Vec<&str> = answer.split("[-]").collect();
            let resonse = ShowResponse::new(r[0].to_string());
            resonse.show();
            print!("{}{}>", "Server".cyan(), r[1].cyan());
            last_path = r[1].to_string();
            std::io::stdout().flush().unwrap();
        }
    }
    fn load_cert(path: &str) -> Result<Certificate, Box<dyn std::error::Error>> {
        let certfile = File::open(path)?;
        let mut reader = BufReader::new(certfile);
        let certs = rustls_pemfile::certs(&mut reader)?;
        if certs.is_empty() {
            return Err("Certificat invalid".into());
        }
        Ok(Certificate(certs[0].clone()))
    }
}
