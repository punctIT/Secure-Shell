mod command_system;
mod login;
mod secure_shell_server;
mod server_configure;
use secure_shell_server::SecureShellServer;

#[tokio::main]
async fn main() {
    let server_config = server_configure::Configure::new();

    let mut server = SecureShellServer::new(
        &server_config.set_cert_path(),
        &server_config.set_cert_key_path(),
        "0.0.0.0:12345",
        &server_config.set_working_directory(),
        &server_config.set_password_file(),
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
