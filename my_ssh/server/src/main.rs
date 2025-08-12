mod command_system;
mod login;
mod secure_shell_server;
use secure_shell_server::SecureShellServer;

#[tokio::main]
async fn main() {
    let mut server = SecureShellServer::new(
        "certificate/cert.pem",
        "certificate/key.pem",
        "0.0.0.0:12345",
        "C:\\Users\\teodo\\Desktop",
        "C:\\Users\\teodo\\Desktop\\passwords.txt",
    );
    server
        .bind_and_listen()
        .await
        .unwrap_or_else(|e| panic!("{}", e));
    server
        .accept_wait()
        .await
        .unwrap_or_else(|e| panic!("{}", e));
}
