#!/usr/bin/env bash
# Fluxx self-hosted bootstrap.
#
# Stands up the agent + dashboard + ClickHouse via docker compose,
# then opens the dashboard. Idempotent: re-running brings the stack
# up to date without clobbering data volumes.
#
# Usage:
#   curl -fsSL https://get.fluxx.dev/self-hosted | bash
#   # or, from a clone of the repo:
#   ./install.sh

set -euo pipefail

FLUXX_DIR="${FLUXX_DIR:-${HOME}/.fluxx}"
COMPOSE_FILE="${FLUXX_DIR}/docker-compose.yaml"

say() { printf '\033[1;36m• %s\033[0m\n' "$*"; }
ok()  { printf '\033[1;32m✔\033[0m %s\n' "$*"; }
warn(){ printf '\033[1;33m!\033[0m %s\n' "$*"; }
die() { printf '\033[1;31m✘\033[0m %s\n' "$*" >&2; exit 1; }

# ── Prerequisites ──────────────────────────────────────────────────
command -v docker >/dev/null 2>&1 || die "Docker not found. Install Docker Desktop or docker-engine first."
docker info >/dev/null 2>&1 || die "Docker daemon isn't reachable. Start it and retry."

if docker compose version >/dev/null 2>&1; then
  DC=(docker compose)
elif command -v docker-compose >/dev/null 2>&1; then
  DC=(docker-compose)
else
  die "Neither 'docker compose' nor 'docker-compose' is available."
fi

# ── Scaffold ───────────────────────────────────────────────────────
say "Preparing ${FLUXX_DIR}"
mkdir -p "$FLUXX_DIR"

cat >"$COMPOSE_FILE" <<'YAML'
services:
  fluxx-agent:
    image: ghcr.io/fluxx/agent:latest
    container_name: fluxx-agent
    restart: unless-stopped
    environment:
      - RUST_LOG=info
      - APP_CLICKHOUSE_ERROR_EVENTS__ENABLED=true
      - APP_CLICKHOUSE_ERROR_EVENTS__HOST=clickhouse
      - APP_CLICKHOUSE_ERROR_EVENTS__PORT=8123
      - APP_CLICKHOUSE_ERROR_EVENTS__DATABASE=fluxx
    ports:
      - "7007:7007"
      - "4317:4317"
      - "4318:4318"
      - "8125:8125/udp"
    volumes:
      - fluxx-wal:/var/lib/fluxx
    depends_on:
      clickhouse: { condition: service_healthy, required: false }

  clickhouse:
    image: clickhouse/clickhouse-server:24.8
    container_name: fluxx-clickhouse
    restart: unless-stopped
    environment:
      - CLICKHOUSE_SKIP_USER_SETUP=1
    ports:
      - "127.0.0.1:8123:8123"
    volumes:
      - fluxx-ch:/var/lib/clickhouse
    healthcheck:
      test: ["CMD", "wget", "--no-verbose", "--tries=1", "--spider", "http://127.0.0.1:8123/ping"]
      interval: 5s
      timeout: 3s
      retries: 12
    ulimits:
      nofile: { soft: 262144, hard: 262144 }

volumes:
  fluxx-wal:
  fluxx-ch:
YAML

# ── Bring it up ────────────────────────────────────────────────────
say "Pulling images (this is the slow part on first run)"
(cd "$FLUXX_DIR" && "${DC[@]}" pull --quiet) || warn "Image pull had warnings; continuing"

say "Starting the stack"
(cd "$FLUXX_DIR" && "${DC[@]}" up -d)

say "Waiting for the agent to come up"
for i in {1..60}; do
  if curl -fsS http://127.0.0.1:7007/v1/status >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

if ! curl -fsS http://127.0.0.1:7007/v1/status >/dev/null 2>&1; then
  warn "Agent isn't responding at http://127.0.0.1:7007 after 60s."
  warn "Check logs:  (cd $FLUXX_DIR && ${DC[*]} logs --tail=50 fluxx-agent)"
  exit 1
fi

ok "Fluxx is up at http://127.0.0.1:7007/"

# ── Surface the admin token ────────────────────────────────────────
# The agent prints its bootstrap token to stdout exactly once,
# wrapped in ANSI escape codes for the boot banner. Strip those
# before grepping so we don't accidentally capture an `[1m` prefix
# along with the real token bytes.
TOKEN=$( (cd "$FLUXX_DIR" && "${DC[@]}" logs --tail=400 fluxx-agent 2>/dev/null) \
  | sed -E 's/\x1b\[[0-9;]*m//g' \
  | awk '/Fluxx admin token/{getline; print $1}' \
  | head -1 || true)

if [[ -n "${TOKEN:-}" ]]; then
  printf '\n\033[1;33m🔑 Admin token:\033[0m %s\n' "$TOKEN"
  printf '   Paste it into the dashboard sign-in prompt, or:\n'
  printf '   export FLUXX_ADMIN_TOKEN=%s\n\n' "$TOKEN"
else
  warn "Could not extract the admin token from logs."
  warn "Print it manually:  (cd $FLUXX_DIR && ${DC[*]} logs fluxx-agent | grep -A 2 'admin token')"
fi

if command -v open >/dev/null 2>&1; then
  open http://127.0.0.1:7007/ >/dev/null 2>&1 || true
elif command -v xdg-open >/dev/null 2>&1; then
  xdg-open http://127.0.0.1:7007/ >/dev/null 2>&1 || true
fi

ok "Done. Next:"
echo "  - Send a test span:  fluxx register --service my-service"
echo "  - Read the docs:     SELF_HOSTED.md"
echo "  - Stop the stack:    (cd $FLUXX_DIR && ${DC[*]} down)"
