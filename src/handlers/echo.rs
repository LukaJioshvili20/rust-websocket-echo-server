use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;
use tracing::{error, info, warn};

pub async fn handle(
    ws_stream: WebSocketStream<TcpStream>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Echo handler invoked");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(message) => match message {
                Message::Text(text) => {
                    info!("Received: {}", text);
                    if let Err(e) = write.send(Message::Text(format!("Echo: {}", text))).await {
                        warn!("Failed to send message: {}", e);
                        break;
                    }
                }
                Message::Binary(bin) => {
                    info!("Received binary message");
                    if let Err(e) = write.send(Message::Binary(bin)).await {
                        warn!("Failed to send binary message: {}", e);
                        break;
                    }
                }
                Message::Close(_) => {
                    info!("Received close frame");
                    break;
                }
                _ => {
                    warn!("Unsupported message type received");
                }
            },
            Err(e) => {
                error!("Error receiving message: {}", e);
                break;
            }
        }
    }

    info!("Echo connection closed");
    Ok(())
}
