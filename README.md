# WebSocket Echo Server

A lightweight WebSocket echo server built in **Rust** using **Tokio** and **tokio-tungstenite**. This server listens for WebSocket connections, receives messages from clients, and sends the same messages back as an "echo."

---

## Features

- **Concurrent WebSocket Clients**: Supports multiple WebSocket clients simultaneously.
- **Echo Functionality**: Automatically echoes back text and binary messages.
- **Asynchronous and Efficient**: Powered by **Tokio** for high performance.
- **Docker Support**: Easily deployable using Docker for any environment.
- **Modular and Extensible**: Clean Rust codebase for adding new features.

---

## Requirements

- **Rust** (version `1.70+`)
- **Cargo** (Rust's package manager)
- **Docker** (for containerized deployment)

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

## Running Locally

1. Start the WebSocket server:

   ```bash
   cargo run
   ```

2. The server will start on `ws://0.0.0.0:8765`.

3. Test the server using a WebSocket client. For example, with [wscat](https://github.com/websockets/wscat):

   ```bash
   npm install -g wscat
   wscat -c ws://0.0.0.0:8765/echo
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

## Running with Docker

You can also run the server in a Docker container:

1. Build the Docker image:

   ```bash
   docker build -t websocket-echo-server .
   ```

2. Run the container:

   ```bash
   docker run -p 8765:8765 websocket-echo-server
   ```

3. The server will be available at `ws://<your-docker-host>:8765`.

---

## Dependencies

This project uses the following Rust crates:

- [`tokio`](https://crates.io/crates/tokio): Asynchronous runtime.
- [`tokio-tungstenite`](https://crates.io/crates/tokio-tungstenite): WebSocket implementation for Tokio.
- [`futures-util`](https://crates.io/crates/futures-util): Utilities for working with asynchronous streams.
- [`tracing`](https://crates.io/crates/tracing): For structured logging.

---

## Code Structure

- **`main.rs`**: Contains the main WebSocket server logic, including:
  - Setting up the TCP listener.
  - Accepting WebSocket connections.
  - Echoing messages back to clients.

---

## Extending the Server

Enhance the server by:

1. **Adding New Endpoints**:
   - Introduce new WebSocket paths like `/chat`.
2. **TLS (WSS)**:
   - Use the `tokio-rustls` crate for secure WebSocket connections.
3. **Structured Logging**:
   - Already integrated with the `tracing` crate for debugging and monitoring.
4. **Custom Message Processing**:
   - Modify the message handling logic to filter, transform, or broadcast messages.

---

## Example Output

When running the server, the output will look like this:

```plaintext
INFO  WebSocket server started successfully on ws://0.0.0.0:8765
INFO  /echo is ready to be listened to...
INFO  Incoming connection on path: /echo
INFO  Echo handler invoked
INFO  Received: Text("Hello, WebSocket!")
INFO  Echo connection closed
```

---

## Contributions

Contributions are welcome! Feel free to submit issues or pull requests to improve the project.

---

## License

This project is licensed under the [MIT License](LICENSE).
