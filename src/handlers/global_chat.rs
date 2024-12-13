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
    let (tx, mut rx) = mpsc::unbounded_channel();

    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.push((client_id.clone(), tx));
    }
    info!("Client {} connected to /global-chat", client_id);

    let broadcast_clients = Arc::clone(&clients);
    let client_id_clone = client_id.clone();

    spawn(async move {
        while let Some(msg_result) = ws_receiver.next().await {
            let msg = match msg_result {
                Ok(msg) if msg.is_text() || msg.is_binary() => msg,
                Ok(_) => continue,
                Err(e) => {
                    error!(
                        "Error receiving message from client {}: {}",
                        client_id_clone, e
                    );
                    return;
                }
            };

            broadcast_message(&client_id_clone, msg, &broadcast_clients);
        }
    });

    while let Some(msg) = rx.recv().await {
        if let Err(e) = ws_sender.send(msg).await {
            error!("Failed to send message to client {}: {}", client_id, e);
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

fn broadcast_message(sender_id: &str, message: Message, clients: &SharedClients) {
    let clients_lock = clients.lock().unwrap();
    for (id, client) in clients_lock.iter() {
        if id == sender_id {
            continue;
        }

        if let Err(e) = client.send(message.clone()) {
            error!("Failed to send message to client {}: {}", id, e);
        }
    }
}
