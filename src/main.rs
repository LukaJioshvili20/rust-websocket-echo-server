use http::Request;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::spawn;
use tokio_tungstenite::accept_hdr_async;
use tracing::{error, info, warn};

mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let addr = "0.0.0.0:8765";
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket server started on ws://{}", addr);

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
    stream: TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut path = String::new();

    let ws_stream = accept_hdr_async(stream, |req: &Request<()>, res| {
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
        _ => {
            warn!("Unsupported path: {}", path);
            Ok(())
        }
    }
}
