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
            cert: cert,
            ip_port: ip_port.to_string(),
            server_name: server_name,
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

        let stream = TcpStream::connect(self.ip_port.clone())
            .await
            .unwrap_or_else(|e| panic!("Error , {e}"));

        let server_name = ServerName::try_from(self.server_name.clone()).unwrap();

        self.tls_stream = Some(connector.connect(server_name, stream).await?);

        Ok(())
    }
    pub async fn send_and_receive(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let tls_stream = self.tls_stream.as_mut().unwrap_or_else(|| {
            panic!("Error TLS not configured");
        });
        print!(">");
        std::io::stdout().flush().unwrap();
        loop {
            let mut mesaj = String::new();
            if mesaj.trim() == "exit" {
                break;
            }
            std::io::stdin().read_line(&mut mesaj).expect("Read Error");
            tls_stream.write(mesaj.as_bytes()).await?;
            let mut buf = vec![0u8; 1024];
            let n = tls_stream.read(&mut buf).await?;

            let answer = String::from_utf8_lossy(&buf[..n]);
            let r: Vec<&str> = answer.split("\r\n").collect();
            let resonse = ShowResponse::new(r[0].to_string());
            resonse.show();
            print!("\n{}{}>", "Server".cyan(), r[1].cyan());
            std::io::stdout().flush().unwrap();
        }
        Ok(())
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
