# Multi-stage Dockerfile for Fluxx Agent
# Optimized for high-performance and minimal footprint

# --- Build Stage ---
FROM rust:1.70-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpcap-dev \
    git \
    make \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy dependency manifests first for caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --profile dist

# Copy full source and rebuild
COPY . .
RUN cargo build --profile dist

# --- Runtime Stage ---
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpcap0.8 \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/dist/fluxx /usr/local/bin/

# Create config directory
RUN mkdir -p /etc/fluxx

# Default configuration
COPY config/default.json /etc/fluxx/config.json

# Expose API and Mesh ports
EXPOSE 3000 8080 7946/tcp 7946/udp

# Standard labels
LABEL org.opencontainers.image.source="https://github.com/fluxx/agent"
LABEL org.opencontainers.image.description="Fluxx: High-performance observability agent"

# Start the agent
ENTRYPOINT ["fluxx"]
CMD ["start", "--config", "/etc/fluxx/config.json"]
