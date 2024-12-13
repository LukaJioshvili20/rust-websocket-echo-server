use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::spawn;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;
use tracing::{error, info};
use uuid::Uuid;

type SharedClients = Arc<Mutex<Vec<(String, mpsc::UnboundedSender<Message>)>>>;

pub async fn handle(
    ws_stream: WebSocketStream<tokio::net::TcpStream>,
    clients: SharedClients,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let client_id = Uuid::new_v4().to_string();

    let client_id_for_task = client_id.clone();

    let (tx, mut rx) = mpsc::unbounded_channel();

    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.push((client_id.clone(), tx.clone()));
    }
    info!("Client {} connected to /global-chat", client_id);

    let broadcast_clients = Arc::clone(&clients);
    spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if msg.is_text() || msg.is_binary() {
                let clients_lock = broadcast_clients.lock().unwrap();
                for (id, client) in clients_lock.iter() {
                    if id != &client_id_for_task {
                        if let Err(e) = client.send(msg.clone()) {
                            error!("Failed to send message to client {}: {}", id, e);
                        }
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
        clients_lock.retain(|(id, _)| id != &client_id);
    }

    info!("Client {} disconnected from /global-chat", client_id);
    Ok(())
}
