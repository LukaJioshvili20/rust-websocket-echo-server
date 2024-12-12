FROM rust:1.72 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04

WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-websocket-echo-server .

EXPOSE 8765

CMD ["./rust-websocket-echo-server"]
