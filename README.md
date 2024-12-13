# WebSocket Echo Server

A lightweight WebSocket echo server built in **Rust** using **Tokio** and **tokio-tungstenite**. This server listens for WebSocket connections, processes messages based on the endpoint, and provides multiple functionalities.

---

## Features

- **Concurrent WebSocket Clients**: Supports multiple WebSocket clients simultaneously.
- **Echo Functionality**: Automatically echoes back text and binary messages.
- **Mathematical Operations**: Provides an endpoint for solving basic math operations.
- **Global Chat**: Facilitates a simple chat feature for connected clients.
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
   ```

4. Connect to different endpoints:

   ### `/echo`

   - Establish a connection:

     ```bash
     wscat -c ws://0.0.0.0:8765/echo
     ```

   - Send a message:

     ```text
     > Hello, WebSocket!
     ```

     The server will echo it back:

     ```text
     < Echo: Hello, WebSocket!
     ```

   ### `/math`

   - Perform mathematical operations:

     ```bash
     wscat -c ws://0.0.0.0:8765/math
     ```

   - Example interactions:

     ```text
     > 8 - 2
     < Result: 6
     > 10 + 2
     < Result: 12
     > 10 * 2
     < Result: 20
     > 3!
     < Result: 6
     ```

   ### `/global-chat`

   - Engage in a global chat:

     ```bash
     wscat -c ws://0.0.0.0:8765/global-chat
     ```

   - Example chat:

     ```text
     > Hello Somebody
     < Greeting from somebody
     > My name is Jeff
     < My name is John
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
- [`http`](https://crates.io/crates/http): Utilities for working with HTTP requests and responses.
- [`uuid`](https://crates.io/crates/uuid): For generating and handling UUIDs, especially useful for unique client or session identifiers.

---

## Code Structure

- **`main.rs`**: Contains the main WebSocket server logic, including:
  - Setting up the TCP listener.
  - Accepting WebSocket connections.
  - Handling `/echo`, `/math`, and `/global-chat` endpoints.

---

## Extending the Server

Enhance the server by:

1. **Secure Communication**:
   - Add WSS support with `tokio-rustls`.

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
