FROM rust:1.70

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8765

CMD ["./target/release/rust-websocket-echo-server"]
