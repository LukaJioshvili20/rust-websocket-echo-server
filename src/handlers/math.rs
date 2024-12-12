use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;
use tracing::{error, info};

pub async fn handle(
    ws_stream: WebSocketStream<TcpStream>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Math handler invoked");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(message) => {
                if let Message::Text(expression) = message {
                    match evaluate_expression(&expression) {
                        Ok(result) => {
                            write
                                .send(Message::Text(format!("Result: {}", result)))
                                .await?;
                        }
                        Err(e) => {
                            write.send(Message::Text(format!("Error: {}", e))).await?;
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error receiving message: {}", e);
                break;
            }
        }
    }

    info!("Math connection closed");
    Ok(())
}

fn evaluate_expression(expression: &str) -> Result<f64, &'static str> {
    if let Some(num_part) = expression.strip_suffix('!') {
        let num = num_part
            .parse::<u64>()
            .map_err(|_| "Invalid number for factorial")?;
        let result = factorial(num)?;
        return Ok(result as f64);
    }

    let tokens: Vec<&str> = expression.split_whitespace().collect();

    if tokens.len() == 3 {
        let left = tokens[0].parse::<f64>().map_err(|_| "Invalid number")?;
        let right = tokens[2].parse::<f64>().map_err(|_| "Invalid number")?;
        match tokens[1] {
            "+" => Ok(left + right),
            "-" => Ok(left - right),
            "*" => Ok(left * right),
            "/" => Ok(left / right),
            "%" => Ok(left % right),
            "^" => Ok(left.powf(right)),
            _ => Err("Unsupported operator"),
        }
    } else {
        Err("Invalid format")
    }
}

fn factorial(n: u64) -> Result<u64, &'static str> {
    if n > 20 {
        return Err("Factorial is too large. Maximum allowed is 20!");
    }
    Ok((1..=n).product())
}
