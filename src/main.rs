use futures_util::{SinkExt, StreamExt};
use http::Request;
use tokio::net::TcpListener;
use tokio::spawn;
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::Response;
use tokio_tungstenite::tungstenite::protocol::Message;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let addr = "0.0.0.0:8765";
    let listener = TcpListener::bind(addr).await?;

    info!("WebSocket server started successfully on ws://{}", addr);
    info!("/echo is ready to be listened to...");

    while let Ok((stream, _)) = listener.accept().await {
        spawn(async move {
            if let Err(err) = handle_connection(stream).await {
                error!("Error handling connection: {}", err);
            }
        });
    }

    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut path = String::new();

    let ws_stream = accept_hdr_async(stream, |req: &Request<()>, res: Response| {
        if let Some(uri) = req.uri().path_and_query() {
            path = uri.to_string(); // Capture the path
            info!("Incoming connection on path: {}", path);
        }
        Ok(res) // Accept the connection
    })
    .await?;

    match path.as_str() {
        "/echo" => handle_echo(ws_stream).await,
        _ => {
            warn!("Unsupported path: {}", path);
            Ok(())
        }
    }
}

async fn handle_echo(
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Echo handler invoked");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(message) => {
                info!("Received: {:?}", message);
                if let Message::Text(text) = message {
                    write.send(Message::Text(format!("Echo: {}", text))).await?;
                } else if let Message::Binary(bin) = message {
                    write.send(Message::Binary(bin)).await?;
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
