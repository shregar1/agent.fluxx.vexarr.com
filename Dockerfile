# Multi-stage Dockerfile for the self-hosted Fluxx single-binary deploy.
#
# Stage 1 (dashboard): build the React/Vite per-node UI to `dist/`.
# Stage 2 (rust):      compile the Rust agent — rust-embed pulls the
#                      previous stage's `dist/` straight into the binary.
# Stage 3 (runtime):   minimal Debian image with just the binary + TLS
#                      roots. One process listens on 7007 and serves
#                      both /v1/* (API) and / (dashboard).

# --- Stage 1: dashboard build ---
FROM node:20-bookworm-slim AS dashboard
WORKDIR /dashboard
COPY node.dashboard.fluxx.dev/package.json node.dashboard.fluxx.dev/package-lock.json ./
RUN npm ci --no-audit --no-fund
COPY node.dashboard.fluxx.dev/ ./
RUN npm run build

# --- Stage 2: rust build ---
FROM rust:1.83-slim-bookworm AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libpcap-dev \
    git \
    make \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache the dependency build by copying manifests first.
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --profile dist || true

# Bring in the real source, plus the dashboard bundle that rust-embed
# expects to find at `node.dashboard.fluxx.dev/dist/`. The empty SDK
# directories under `sdks/` are also kept so future SDK-bundled builds
# don't fail.
COPY . .
COPY --from=dashboard /dashboard/dist ./node.dashboard.fluxx.dev/dist
RUN touch src/main.rs && cargo build --profile dist

# --- Stage 3: runtime ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libpcap0.8 \
    libssl3 \
    wget \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/dist/fluxx /usr/local/bin/fluxx

# WAL + cold storage default to /var/lib/fluxx; mount a volume here in
# production if you need durability across restarts.
RUN mkdir -p /var/lib/fluxx /etc/fluxx
VOLUME ["/var/lib/fluxx"]

# 7007 = API + dashboard. 4317/4318 = OTLP gRPC/HTTP. 8125 = StatsD.
EXPOSE 7007 4317 4318 8125/udp

HEALTHCHECK --interval=15s --timeout=3s --start-period=10s --retries=3 \
  CMD wget -q -O - http://127.0.0.1:7007/v1/status >/dev/null 2>&1 || exit 1

LABEL org.opencontainers.image.source="https://github.com/fluxx/agent"
LABEL org.opencontainers.image.description="Fluxx — self-hosted observability agent + dashboard, one binary"

# `start` honours config from FLUXX_* env vars; no on-disk config file
# is required for a typical self-hosted deploy.
ENTRYPOINT ["fluxx"]
CMD ["start"]
