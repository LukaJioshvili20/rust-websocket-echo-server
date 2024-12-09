# WebSocket Echo Server

A lightweight WebSocket echo server built in **Rust** using **Tokio** and **tokio-tungstenite**. The server listens for WebSocket connections, receives messages from clients, and sends the same messages back as an "echo."

## Features

- Supports multiple WebSocket clients concurrently.
- Automatically echoes back text and binary messages.
- Built with **Tokio** for asynchronous and efficient I/O.
- Uses **tokio-tungstenite** for WebSocket support.
- Beginner-friendly, modular, and clean Rust code.

---

## Requirements

- Rust (version `1.70+`)
- Cargo (Rust's package manager)

Install Rust using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Dependencies

This project uses the following Rust crates:

- [`tokio`](https://crates.io/crates/tokio): Asynchronous runtime.
- [`tokio-tungstenite`](https://crates.io/crates/tokio-tungstenite): WebSocket implementation for Tokio.
- [`futures-util`](https://crates.io/crates/futures-util): Utilities for working with asynchronous streams.

Dependencies are managed in `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1.42.0", features = ["full"] }
tokio-tungstenite = "0.24.0"
futures-util = "0.3"
```

---

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/LukaJioshvili20/rust-websocket-echo-server.git
   cd rust-websocket-echo-server
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

---

## Usage

1. Run the server:

   ```bash
   cargo run
   ```

2. The server will start on `ws://127.0.0.1:8765`.

3. Test the server using a WebSocket client. For example, using [wscat](https://github.com/websockets/wscat):

   ```bash
   npm install -g wscat
   wscat -c ws://127.0.0.1:8765
   ```

4. Send a message to the server:

   ```text
   > Hello, WebSocket!
   ```

   The server will echo it back:

   ```text
   < Echo: Hello, WebSocket!
   ```

---

## Code Structure

- `main.rs`: Contains the main logic for the WebSocket server, including:
  - Setting up the TCP listener.
  - Accepting WebSocket connections.
  - Echoing messages back to clients.

---

## Extending the Server

You can enhance the server further by:

- **Adding TLS (WSS)**: Use the `tokio-rustls` crate for secure WebSocket connections.
- **Message Filtering**: Implement custom logic to process messages before echoing them.
- **Logging**: Integrate structured logging using crates like `log` and `env_logger`.

---

## Example Output

When running the server, the output will look like this:

```text
WebSocket server started on ws://127.0.0.1:8765
New WebSocket connection established
Received: "Hello, WebSocket!"
Connection closed
```

---

## License

This project is licensed under the [MIT License](LICENSE).

---

## Contributions

Contributions are welcome! Feel free to submit issues or pull requests to improve the project.
