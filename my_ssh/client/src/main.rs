mod client;
mod client_configure;
mod response_handlers;
use client::Client;

#[tokio::main]
async fn main() {
    let client_cfg = client_configure::Configure::new();

    let mut client = Client::new(
        &client_cfg.set_cert_path(),
        "localhost",
        &client_cfg.set_ip_path(),
    );

    if let Err(e) = client.connect_to_server().await {
        eprintln!("Error At Connect: {:?}", e);
        return;
    }

    if let Err(e) = client.send_and_receive().await {
        eprintln!("Error at communication: {:?}", e);
    }
}
