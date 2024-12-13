use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;
use tracing::{error, info};

pub async fn handle(
    ws_stream: WebSocketStream<TcpStream>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Math handler invoked");

    let (mut writer, mut reader) = ws_stream.split();

    while let Some(msg) = reader.next().await {
        match msg {
            Ok(Message::Text(expression)) => {
                let response = match evaluate_expression(&expression) {
                    Ok(result) => format!("Result: {}", result),
                    Err(e) => format!("Error: {}", e),
                };
                if let Err(e) = writer.send(Message::Text(response)).await {
                    error!("Failed to send response: {}", e);
                    break;
                }
            }
            Ok(_) => continue,
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
    if let Some(result) = parse_factorial(expression)? {
        return Ok(result as f64);
    }

    let tokens = parse_tokens(expression)?;
    execute_operation(tokens)
}

fn parse_factorial(expression: &str) -> Result<Option<u64>, &'static str> {
    if let Some(num_part) = expression.strip_suffix('!') {
        let num = num_part
            .parse::<u64>()
            .map_err(|_| "Invalid number for factorial")?;
        let result = factorial(num)?;
        return Ok(Some(result));
    }
    Ok(None)
}

fn parse_tokens(expression: &str) -> Result<(&str, f64, f64), &'static str> {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    if tokens.len() != 3 {
        return Err("Invalid format. Expected: `<num> <operator> <num>`");
    }

    let left = tokens[0].parse::<f64>().map_err(|_| "Invalid number")?;
    let right = tokens[2].parse::<f64>().map_err(|_| "Invalid number")?;
    Ok((tokens[1], left, right))
}

fn execute_operation((operator, left, right): (&str, f64, f64)) -> Result<f64, &'static str> {
    match operator {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => Ok(left / right),
        "%" => Ok(left % right),
        "^" => Ok(left.powf(right)),
        _ => Err("Unsupported operator"),
    }
}

fn factorial(n: u64) -> Result<u64, &'static str> {
    if n > 20 {
        return Err("Factorial is too large. Maximum allowed is 20!");
    }
    Ok((1..=n).product())
}
