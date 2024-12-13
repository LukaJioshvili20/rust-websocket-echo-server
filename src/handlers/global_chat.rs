use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;
use tracing::{error, info};

use std::sync::{Arc, Mutex};

type SharedClients = Arc<Mutex<Vec<mpsc::UnboundedSender<Message>>>>;

pub async fn handle(
    ws_stream: WebSocketStream<tokio::net::TcpStream>,
    clients: SharedClients,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let (tx, mut rx) = mpsc::unbounded_channel();

    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.push(tx.clone());
    }
    info!("Client connected to /global-chat");

    let broadcast_clients = Arc::clone(&clients);
    tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if msg.is_text() || msg.is_binary() {
                let clients_lock = broadcast_clients.lock().unwrap();
                for client in clients_lock.iter() {
                    if let Err(e) = client.send(msg.clone()) {
                        error!("Failed to send message to client: {}", e);
                    }
                }
            }
        }
    });

    while let Some(msg) = rx.recv().await {
        if ws_sender.send(msg).await.is_err() {
            break;
        }
    }

    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.retain(|client| !client.same_channel(&tx));
    }

    info!("Client disconnected from /global-chat");
    Ok(())
}
