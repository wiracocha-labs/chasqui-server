# 🐳 Dockerfile for Chasqui Server

# Stage 1: Build
FROM rust:1.85-slim-bookworm AS builder

# Create a new empty shell project
RUN USER=root cargo new --bin chasqui-server
WORKDIR /chasqui-server

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache our dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy our source code
COPY ./src ./src

# Build for release
# We touch the main file to ensure it's rebuilt
RUN touch src/main.rs
RUN cargo build --release

# Stage 2: Run
FROM debian:bookworm-slim

# Install runtime dependencies (OpenSSL used by many crates)
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /chasqui-server/target/release/chasqui-server /usr/local/bin/chasqui-server

# Set environment variables
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080

# Expose the port (Railway will provide a PORT at runtime)
EXPOSE 8080

# At runtime forward Railway's PORT to SERVER_PORT (default to the Dockerfile value)
# This ensures the app listens on the port Railway expects while keeping the default for local runs.
CMD ["sh", "-c", "export SERVER_PORT=${PORT:-$SERVER_PORT}; exec /usr/local/bin/chasqui-server"]
