use futures_util::{Sink, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{tungstenite::Error as TungsteniteError, WebSocketStream};
use tracing::{error, info, warn};

pub async fn handle(ws_stream: WebSocketStream<TcpStream>) -> Result<(), TungsteniteError> {
    info!("Echo handler invoked");

    let (mut writer, mut reader) = ws_stream.split();

    while let Some(message_result) = reader.next().await {
        match message_result {
            Ok(message) => {
                if let Err(e) = process_message(&mut writer, message).await {
                    warn!("Error processing message: {:?}", e);
                    break;
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

async fn process_message<W>(writer: &mut W, message: Message) -> Result<(), TungsteniteError>
where
    W: Sink<Message, Error = TungsteniteError> + Unpin,
{
    match message {
        Message::Text(text) => {
            info!("Received text message: {}", text);
            writer
                .send(Message::Text(format!("Echo: {}", text)))
                .await?;
        }
        Message::Binary(binary_data) => {
            info!("Received binary message");
            writer.send(Message::Binary(binary_data)).await?;
        }
        Message::Close(close_frame) => {
            info!("Received close frame: {:?}", close_frame);
            writer.send(Message::Close(close_frame)).await.ok();
            return Err(TungsteniteError::ConnectionClosed);
        }
        other => {
            warn!("Unsupported message type received: {:?}", other);
        }
    }

    Ok(())
}
