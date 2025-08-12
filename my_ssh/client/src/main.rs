mod client;
mod response_handlers;
use client::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::new("certificate/cert.pem", "localhost", "192.168.100.237:12345");

    if let Err(e) = client.connect_to_server().await {
        eprintln!("Error At Connect: {:?}", e);
        return;
    }

    if let Err(e) = client.send_and_receive().await {
        eprintln!("Error at communication: {:?}", e);
    }
}
