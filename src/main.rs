mod status;
mod packet;
mod connection;
mod config;

use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config = Arc::new(config::ConfigManager::new().await);

    let listener = TcpListener::bind("0.0.0.0:25565").await.expect("Error starting the server");
    println!("Started server on {}", listener.local_addr().unwrap().to_string().as_str());

    loop {
        let conn = listener.accept().await.unwrap();

        let mut player = connection::PlayerConnection::new(conn.0, config.clone()).await;

        tokio::spawn(async move {
            player.handle().await;
        });
    }
}
