use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use tokio::spawn;
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tracing::{error, info, warn};

mod handlers;

type SharedClients = Arc<Mutex<Vec<(String, tokio::sync::mpsc::UnboundedSender<Message>)>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let addr = "0.0.0.0:8765";
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket server started on ws://{}", addr);

    let global_chat_clients: SharedClients = Arc::new(Mutex::new(Vec::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let global_chat_clients = Arc::clone(&global_chat_clients);

        spawn(async move {
            if let Err(err) = handle_connection(stream, global_chat_clients).await {
                error!("Error handling connection: {}", err);
            }
        });
    }

    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    global_chat_clients: SharedClients,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut path = String::new();

    let ws_stream = accept_hdr_async(stream, |req: &http::Request<()>, res| {
        if let Some(uri) = req.uri().path_and_query() {
            path = uri.to_string();
            info!("Incoming connection on path: {}", path);
        }
        Ok(res)
    })
    .await?;

    match path.as_str() {
        "/echo" => handlers::echo::handle(ws_stream).await,
        "/math" => handlers::math::handle(ws_stream).await,
        "/global-chat" => handlers::global_chat::handle(ws_stream, global_chat_clients).await,
        _ => {
            warn!("Unsupported path: {}", path);
            Ok(())
        }
    }
}
