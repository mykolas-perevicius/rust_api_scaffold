FROM rust:1.78 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust_api_scaffold /usr/local/bin/app
EXPOSE 8080
CMD ["app"]
