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

pub struct Client {
    cert: Certificate,
    ip_port: String,
    server_name: ServerName,
    tls_stream: Option<TlsStream<TcpStream>>,
}
impl Client {
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
