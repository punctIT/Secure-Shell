use crate::command_system::command_handler::CommandHandler;
use crate::login::UserLogin;
use std::{fs::File, io::BufReader, sync::Arc};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio_rustls::{
    TlsAcceptor,
    rustls::{Certificate, PrivateKey, ServerConfig},
};

/// A secure shell server implementation using TLS encryption.
///
/// `SecureShellServer` provides a TLS-encrypted remote shell service that allows
/// authenticated users to execute commands in a secure environment. The server
/// handles SSL/TLS certificate management, user authentication, and command processing.
///
/// # Examples
///
/// ```rust
/// let mut server = SecureShellServer::new(
///     "/path/to/cert.pem",
///     "/path/to/key.pem",
///     "127.0.0.1:8443",
///     "/home/secure",
///     "/etc/passwords.txt"
/// );
///
/// server.bind_and_listen().await?;
/// server.accept_wait().await?;
/// ```
pub struct SecureShellServer {
    certs: Vec<Certificate>,
    key: PrivateKey,
    ip_port: String,
    listener: Option<TcpListener>,
    acceptor: Option<TlsAcceptor>,
    root_path: std::path::PathBuf,
    password_path: std::path::PathBuf,
    users: Arc<RwLock<Vec<String>>>,
}

impl SecureShellServer {
    /// Creates a new `SecureShellServer` instance with the specified configuration.
    ///
    /// # Parameters
    ///
    /// - `cert_path`: Path to the TLS certificate file (PEM format)
    /// - `key_path`: Path to the private key file (PEM format)
    /// - `ip_port`: IP address and port to bind to (e.g., "127.0.0.1:8443")
    /// - `root`: Root directory path for user operations (sandboxing)
    /// - `password_path`: Path to the password file for authentication
    ///
    /// # Returns
    ///
    /// A new `SecureShellServer` instance ready for binding and listening.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - Certificate file cannot be loaded or is invalid
    /// - Private key file cannot be loaded or is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// let server = SecureShellServer::new(
    ///     "/etc/ssl/certs/server.pem",
    ///     "/etc/ssl/private/server.key",
    ///     "0.0.0.0:8443",
    ///     "/home/secure_shell",
    ///     "/etc/secure_shell/passwords"
    /// );
    /// ```
    pub fn new(
        cert_path: &str,
        key_path: &str,
        ip_port: &str,
        root: &str,
        password_path: &str,
    ) -> Self {
        let certs = SecureShellServer::load_certs(cert_path)
            .unwrap_or_else(|e| panic!("Error: Certifcate {:?}", e));
        let key = SecureShellServer::load_private_key(key_path)
            .unwrap_or_else(|e| panic!("Error: Key {:?}", e));
        SecureShellServer {
            certs,
            key,
            ip_port: ip_port.to_string(),
            listener: None,
            acceptor: None,
            root_path: std::path::PathBuf::from(root),
            password_path: std::path::PathBuf::from(password_path),
            users: Arc::new(RwLock::new(Vec::new())),
        }
    }
    /// Binds the server to the specified IP and port and prepares it for accepting connections.
    ///
    /// This method sets up the TLS configuration using the loaded certificates and private key,
    /// creates a TCP listener, and prepares the TLS acceptor for handling incoming connections.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the server is successfully bound and configured
    /// - `Err(Box<dyn std::error::Error>)` if binding fails or TLS configuration is invalid
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The IP address and port are already in use
    /// - Invalid TLS certificate or private key
    /// - Network interface is not available
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut server = SecureShellServer::new(/* ... */);
    /// match server.bind_and_listen().await {
    ///     Ok(()) => println!("Server bound successfully"),
    ///     Err(e) => eprintln!("Failed to bind server: {}", e),
    /// }
    /// ```
    pub async fn bind_and_listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(self.certs.clone(), self.key.clone())?;
        self.acceptor = Some(TlsAcceptor::from(Arc::new(config)));

        self.listener = Some(TcpListener::bind(self.ip_port.clone()).await?);
        println!("Server TLS listener at {}", self.ip_port);

        Ok(())
    }
        /// Accepts and handles incoming client connections in an infinite loop.
    /// 
    /// This method continuously accepts new TCP connections, performs TLS handshakes,
    /// and spawns separate async tasks to handle each client session. Each client
    /// session includes authentication and command processing capabilities.
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - This method runs indefinitely and should not return under normal operation
    /// - `Err(Box<dyn std::error::Error>)` if a critical server error occurs
    /// 
    /// # Panics
    /// 
    /// Panics if `bind_and_listen()` has not been called first, as the listener
    /// and acceptor must be configured before accepting connections.
    /// 
    /// # Client Session Flow
    /// 
    /// 1. Accept TCP connection and perform TLS handshake
    /// 2. Send welcome message to client
    /// 3. Wait for login command with username and password
    /// 4. Authenticate user against password file
    /// 5. Process commands for authenticated users
    /// 6. Clean up user session on disconnect
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let mut server = SecureShellServer::new(/* ... */);
    /// server.bind_and_listen().await?;
    /// 
    /// // This will run indefinitely, handling client connections
    /// server.accept_wait().await?;
    /// ```
    /// 
    /// # Note
    /// 
    /// Each client connection is handled in a separate async task, allowing
    /// concurrent sessions from multiple users.
    pub async fn accept_wait(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = self
            .listener
            .as_mut()
            .unwrap_or_else(|| panic!("error , acceptor not configured"));
        let acceptor = self
            .acceptor
            .as_mut()
            .unwrap_or_else(|| panic!("error , acceptor not configured"));

        loop {
            let (stream, addr) = listener.accept().await?;
            let acceptor = acceptor.clone();
            let root_path = self.root_path.clone();
            let users = self.users.clone();

            let password_path = self.password_path.clone();
            tokio::spawn(async move {
                let mut tls_stream = match acceptor.accept(stream).await {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("TLS handshake failed: {:?}", e);
                        return;
                    }
                };
                println!("Client TLS :connected {}", addr);

                if let Err(e) = tls_stream
                    .write_all("?&NWelcome\nThis is a secure shell , use >: login [USERNAME] [PASSWORD][-]".as_bytes())
                    .await
                {
                    eprintln!("Write Error: {:?}", e);
                }
                let mut server_path = root_path.clone();
                let mut user: Option<String> = None;
                let mut buf = vec![0u8; 1024];
                loop {
                    match tls_stream.read(&mut buf).await {
                        Ok(0) => {
                            println!("client disconnected {}", addr);
                            if user.is_some() {
                                let mut vec_lock = users.write().await;
                                vec_lock.retain(|u| u != user.as_ref().unwrap());
                            }
                            break;
                        }
                        Ok(n) => {
                            let received = String::from_utf8_lossy(&buf[..n]);
                            println!(
                                "{}:{} sent: {}",
                                user.clone().unwrap_or(String::from("")),
                                addr,
                                received.trim()
                            );
                            if user.is_none() {
                                let login = UserLogin::new(
                                    received.to_string(),
                                    password_path.clone(),
                                    users.clone(),
                                );
                                match login.get_login_status().await {
                                    Ok(user_name) => {
                                        dbg!("login succesful");
                                        let mut vec_lock = users.write().await;
                                        vec_lock.push(user_name.clone());
                                        user = Some(user_name);
                                        if let Err(err) = tls_stream
                                            .write_all(
                                                format!(
                                                    "{}[-]:{}[-]\r\n\r\n",
                                                    "?&NSuccesful login", ""
                                                )
                                                .as_bytes(),
                                            )
                                            .await
                                        {
                                            eprintln!("Write Error: {:?}", err);
                                        }
                                    }
                                    Err(e) => {
                                        //dbg!(&e);
                                        if let Err(err) = tls_stream.write_all(e.as_bytes()).await {
                                            eprintln!("Write Error: {:?}", err);
                                        }
                                    }
                                }
                            } else {
                                let mut command_handler = CommandHandler::new(
                                    received.to_string(),
                                    root_path.clone(),
                                    server_path.clone(),
                                    users.clone(),
                                );
                                let (reply, new_server_path) = command_handler.get_output().await;
                                server_path = new_server_path;
                                //dbg!(&reply);
                                if let Err(e) = tls_stream.write_all(reply.as_bytes()).await {
                                    eprintln!("Write Error: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Read Error: {:?}", e);
                            let mut vec_lock = users.write().await;
                            if let Some(ref username) = user {
                                vec_lock.retain(|u| u != username);
                            }
                            break;
                        }
                    }
                }
            });
        }
    }
    fn load_certs(path: &str) -> Result<Vec<Certificate>, Box<dyn std::error::Error>> {
        let certfile = File::open(path)?;
        let mut reader = BufReader::new(certfile);
        let certs = rustls_pemfile::certs(&mut reader)?
            .into_iter()
            .map(Certificate)
            .collect();
        Ok(certs)
    }

    fn load_private_key(path: &str) -> Result<PrivateKey, Box<dyn std::error::Error>> {
        let keyfile = File::open(path)?;
        let mut reader = BufReader::new(keyfile);
        let keys = rustls_pemfile::pkcs8_private_keys(&mut reader)?;
        if keys.is_empty() {
            return Err("Private key not found".into());
        }
        Ok(PrivateKey(keys[0].clone()))
    }
}
