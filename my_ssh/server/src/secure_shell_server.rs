use std::{fs::File, io::BufReader, sync::Arc};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::{
    TlsAcceptor,
    rustls::{Certificate, PrivateKey, ServerConfig},
};

pub struct SecureShellServer {
    certs: Vec<Certificate>,
    key: PrivateKey,
    ip_port: String,
    listener: Option<TcpListener>,
    acceptor:Option<TlsAcceptor>
}

impl SecureShellServer {
    pub fn new(cert_path: &str, key_path: &str, ip_port: &str) -> Self {
        let certs = SecureShellServer::load_certs(cert_path).unwrap_or_else(|e| panic!("Error: Certifcate {:?}", e));
        let key = SecureShellServer::load_private_key(key_path).unwrap_or_else(|e| panic!("Error: Key {:?}", e));
        SecureShellServer {
            certs: certs,
            key: key,
            ip_port: ip_port.to_string(),
            listener: None,
            acceptor:None
        }
    }
    pub async fn bind_and_listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(self.certs.clone(), self.key.clone())?;
        self.acceptor = Some(TlsAcceptor::from(Arc::new(config)));

        self.listener = Some(TcpListener::bind(self.ip_port.clone()).await?);
        println!("Server TLS ascultă la {}", self.ip_port);

        Ok(())
    }
    pub async fn accept_wait(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = self.listener.as_mut().unwrap_or_else(||{
            panic!("error , TcpListener not binded")
        });
        let acceptor=self.acceptor.as_mut().unwrap_or_else(||{
            panic!("error , acceptor not configured")
        });
        loop {
            let (stream, addr) = listener.accept().await?;
            let acceptor = acceptor.clone();
            tokio::spawn(async move {
                let mut tls_stream = match acceptor.accept(stream).await {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("TLS handshake failed: {:?}", e);
                        return;
                    }
                };
                println!("Client TLS conectat: {}", addr);

                let mut buf = vec![0u8; 1024];
                loop {
                    match tls_stream.read(&mut buf).await {
                        Ok(n) if n == 0 => 
                        {
                            println!("Clientul s-a deconectat");
                            break;
                        },
                        Ok(n) => {
                            let received = String::from_utf8_lossy(&buf[..n]);
                            println!("Am primit: {}", received);

                            let reply = format!("Am primit: {}", received);
                            if let Err(e) = tls_stream.write_all(reply.as_bytes()).await {
                                eprintln!("Eroare la scriere: {:?}", e);
                            }
                        }
                        Err(e) => eprintln!("Eroare la citire: {:?}", e),
                    }
                }
            });
        }
        Ok(())
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
            return Err("Cheia privată nu a fost găsită".into());
        }
        Ok(PrivateKey(keys[0].clone()))
    }
}
