mod secure_shell_server;
use secure_shell_server::SecureShellServer;


#[tokio::main]
async fn main() {
    let mut server = SecureShellServer::new("certificate/cert.pem", "certificate/key.pem", "127.0.0.1:12345");
    server.bind_and_listen().await.unwrap_or_else(|e|{
        panic!("{}",e)
    });
    server.accept_wait().await.unwrap_or_else(|e|{
        panic!("{}",e)
    });
}