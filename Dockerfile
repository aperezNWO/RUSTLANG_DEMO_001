# Build Stage - Use the latest stable Rust toolchain on Alpine
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev build-base

WORKDIR /app

# Safely copy manifests
COPY Cargo.toml Cargo.loc[k] ./

# Copy source code and build binary
COPY src ./src
RUN cargo build --release

# Final Stage
FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache ca-certificates libgcc

# Copy compiled binary from builder
COPY --from=builder /app/target/release/rust-ping-api /app/rust-ping-api

EXPOSE 8080

CMD ["./rust-ping-api"]