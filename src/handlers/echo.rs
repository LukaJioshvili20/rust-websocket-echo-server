use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;
use tracing::{error, info};

pub async fn handle(
    ws_stream: WebSocketStream<TcpStream>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Echo handler invoked");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(message) => {
                info!("Received: {:?}", message);
                match message {
                    Message::Text(text) => {
                        write.send(Message::Text(format!("Echo: {}", text))).await?;
                    }
                    Message::Binary(bin) => {
                        write.send(Message::Binary(bin)).await?;
                    }
                    _ => {
                        write
                            .send(Message::Text("Unsupported message type".into()))
                            .await?;
                    }
                }
            }
            Err(e) => {
                error!("Error receiving message: {}", e);
                break;
            }
        }
    }

    info!("Echo connection closed");
    Ok(())
}
