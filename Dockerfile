# --- Stage 1: Build Image ---
FROM rust:1.80-alpine AS builder

WORKDIR /app
RUN apk add --no-libc-dev build-base

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# --- Stage 2: Minimal Runtime Image ---
FROM alpine:latest

WORKDIR /app
RUN apk add --no-cache ca-certificates libgcc

COPY --from=builder /app/target/release/rust-ping-api /app/rust-ping-api

EXPOSE 8080
ENV RUST_LOG=info

CMD ["./rust-ping-api"]