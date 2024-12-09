use tokio::net::TcpListener;
use tokio::spawn;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8765";

    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        spawn(async move {
            if let Err(err) = handle_connection(stream).await {
                eprintln!("Error handling connection: {}", err);
            }
        });
    }

    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;
    println!("New WebSocket connection established");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(message) => {
                println!("Received: {:?}", message);
                if let Message::Text(text) = message {
                    write.send(Message::Text(format!("Echo: {}", text))).await?;
                } else if let Message::Binary(bin) = message {
                    write.send(Message::Binary(bin)).await?;
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }

    println!("Connection closed");
    Ok(())
}
