FROM rust:1.72 AS builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/rust-websocket-echo-server .

EXPOSE 8765

ENV RUST_LOG=info

CMD ["./rust-websocket-echo-server"]
