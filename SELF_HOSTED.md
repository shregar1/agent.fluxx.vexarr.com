# Fluxx — Self-hosted deploy

One binary. One port. No phoning home.

## 30-second quickstart

```bash
./install.sh           # or: curl -fsSL https://get.fluxx.dev/self-hosted | bash
```

Brings up `fluxx-agent` + ClickHouse via Docker Compose, prints the
admin token, opens `http://localhost:7007/` in your browser. Paste
the token into the sign-in prompt.

```
                 ┌─────────────────────────────┐
                 │       fluxx (port 7007)      │
                 │                              │
   browser  ───▶ │  /              dashboard    │
   SDKs    ───▶ │  /v1/*           API          │
   OTLP    ───▶ │  4317/4318       traces       │
   StatsD  ───▶ │  8125/udp        metrics      │
                 └──────────┬───────────────────┘
                            │
                            ▼ (optional)
                  ┌──────────────────────┐
                  │      ClickHouse      │
                  │  persisted errors    │
                  └──────────────────────┘
```

The dashboard and the API are the same process — the React/Vite bundle
in `node.dashboard.fluxx.dev/dist/` is embedded into the Rust binary at
compile time via `rust-embed`. There is no second service to deploy.

---

## Quickstart with Docker Compose

```bash
docker compose up -d
open http://localhost:7007/
```

That's it. Send error events at `/v1/rum`, `/v1/logs`, or OTLP traces
at `:4317`/`:4318`; they show up in the dashboard.

ClickHouse persistence is on by default in the included compose file.
Comment out the `APP_CLICKHOUSE_ERROR_EVENTS__*` env vars on
`fluxx-agent` to run with in-memory error tracking only (faster, but
lost on restart).

---

## Self-hosted mode

Fluxx enters self-hosted mode automatically when **neither**
`FLUXX_API_KEY` **nor** `FLUXX_API_URL` is set. Set
`FLUXX_SELF_HOSTED=true` to force it on regardless.

Self-hosted mode disables:

| What | Why it's off |
|---|---|
| License check (api.fluxx.dev) | No control plane to call. |
| Cloud forwarder | Nowhere to forward to. |
| Runtime grants | Premium features are gated by tier in the agent; a self-hosted user has explicit access. |

It does **not** disable:

- ClickHouse persistence (controlled separately by the CH config)
- OTLP / RUM / StatsD ingest
- The dashboard
- The `fluxx errors` CLI subcommand (talks to the local agent only)

---

## Configuration

All runtime knobs are env-var driven. The full list lives in
[`src/config/mod.rs`](src/config/mod.rs); the ones you'll touch:

```bash
# HTTP API + dashboard bind
APP_SERVER__HOST=0.0.0.0
APP_SERVER__PORT=7007

# OTLP receiver
APP_OTLP__GRPC_PORT=4317
APP_OTLP__HTTP_PORT=4318

# Log store backend: "memory" | "file" | "wal"
APP_LOGS__BACKEND=wal
APP_LOGS__WAL__WAL_DIR=/var/lib/fluxx/log-wal

# ClickHouse-backed error persistence (off by default)
APP_CLICKHOUSE_ERROR_EVENTS__ENABLED=true
APP_CLICKHOUSE_ERROR_EVENTS__HOST=clickhouse
APP_CLICKHOUSE_ERROR_EVENTS__PORT=8123
APP_CLICKHOUSE_ERROR_EVENTS__DATABASE=fluxx
APP_CLICKHOUSE_ERROR_EVENTS__FLUSH_INTERVAL_SECS=5
APP_CLICKHOUSE_ERROR_EVENTS__FLUSH_BATCH_SIZE=1000
APP_CLICKHOUSE_ERROR_EVENTS__QUEUE_CAPACITY=100000
```

---

## Operator surface

| Path | Purpose |
|---|---|
| `GET /` | Dashboard (React SPA) |
| `GET /v1/status` | Health probe — used by the container `HEALTHCHECK` |
| `GET /v1/stats` | Storage stats (event count, WAL segments, etc.) |
| `GET /v1/errors` | Error groups, last 24h by default |
| `GET /v1/errors/top?n=10` | Noisiest errors |
| `GET /v1/errors/trend` | 24-bucket hourly histogram |
| `POST /v1/errors/:fp/resolve` | Mark a fingerprint resolved (in-memory only on the agent) |

### From the CLI

```bash
fluxx errors top -n 10
fluxx errors trend
fluxx errors stats
fluxx errors list --since-hours 168 --json | jq '.data[] | select(.count > 50)'
```

The CLI talks to `http://127.0.0.1:7007` by default. Override with
`--url` or `FLUXX_AGENT_URL`.

---

## SDKs

The agent ingests events from any OTLP-compatible source. Native SDKs
with `release` tagging support live in:

```
sdks/browser   fluxx-rum.js     (RUM, errors, vitals)
sdks/node      fluxx.js
sdks/python    fluxx.py
sdks/go        fluxx.go
```

All four accept a `release` option that propagates as the
`service.version` resource attribute on OTLP exports and as a
top-level `release` field on the agent's `/v1/logs` and `/v1/events`
endpoints.

---

## Hardening

The defaults assume a trusted internal network. Before exposing port
7007 to the public internet:

1. Put a reverse proxy in front (nginx, Caddy, Cloudflare Tunnel) for
   TLS, gzip, and rate limits.
2. Set `APP_SERVER__HOST=127.0.0.1` and let the proxy bridge to it.
3. The agent's CORS is permissive by default; tighten via
   `corsOriginList` in [`src/api/server.rs`](src/api/server.rs) for
   production.
4. ClickHouse: keep it on the docker-internal network (don't publish
   `8123` externally). The included compose file binds CH only to
   `127.0.0.1` for that reason.
5. Auth defaults to a bootstrap admin token. To require auth on the
   SDK ingest endpoints too (`/v1/rum`, `/v1/logs`, `/v1/events`,
   `/v1/otlp/*`), set `FLUXX_REQUIRE_INGEST_AUTH=true` — your SDKs
   then need to send the token as `X-API-Key`.
6. Webhook alerts (`FLUXX_WEBHOOK_URL`): set `FLUXX_WEBHOOK_SECRET`
   and verify `X-Fluxx-Signature: sha256=<hex>` on the receiver. The
   header value is `sha256(secret + body)` — the standard webhook
   signing pattern.

### TLS

The agent speaks plain HTTP — TLS is delegated to a reverse proxy so
operators can use whichever cert lifecycle they already trust
(Let's Encrypt via Caddy, an internal CA via nginx, Cloudflare Origin
certs, etc.).

**Caddy** (auto-TLS via Let's Encrypt):

```caddy
fluxx.example.com {
    reverse_proxy 127.0.0.1:7007
}
```

That's it — drop into `Caddyfile`, `caddy run`, point DNS, done.

**nginx + certbot:**

```nginx
server {
    listen 443 ssl http2;
    server_name fluxx.example.com;

    ssl_certificate     /etc/letsencrypt/live/fluxx.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/fluxx.example.com/privkey.pem;

    # The agent's API server reads X-Forwarded-For when `trust_proxy`
    # is on (default in docker). Pass it through so rate limits work.
    location / {
        proxy_pass         http://127.0.0.1:7007;
        proxy_http_version 1.1;
        proxy_set_header   Host              $host;
        proxy_set_header   X-Real-IP         $remote_addr;
        proxy_set_header   X-Forwarded-For   $proxy_add_x_forwarded_for;
        proxy_set_header   X-Forwarded-Proto $scheme;
        proxy_set_header   Upgrade           $http_upgrade;
        proxy_set_header   Connection        "upgrade";
        proxy_read_timeout 600s;
    }
}
```

For browser RUM specifically: the SDK must point at the public HTTPS
hostname, not the localhost agent. Set `endpoint:
'https://fluxx.example.com'` in your `FluxxRUM.init()` config.

---

## Upgrades

```bash
docker compose pull && docker compose up -d
```

Migrations are idempotent (`CREATE … IF NOT EXISTS`) so the agent
re-runs them on every boot. The in-memory error tracker preloads from
ClickHouse's `error_events` table over the last 24h, so a restart
doesn't blank the dashboard.

---

## Troubleshooting

**Dashboard shows "bundle not built into this binary"** — you compiled
the Rust binary without first running `npm run build` in
`node.dashboard.fluxx.dev/`. The included Dockerfile does this in a
stage; if you're building locally, run:

```bash
cd node.dashboard.fluxx.dev && npm install && npm run build
cd .. && cargo build --release
```

**Agent boots but errors don't persist across restarts** —
ClickHouse persistence is opt-in. Check that
`APP_CLICKHOUSE_ERROR_EVENTS__ENABLED=true` and that the CH host is
reachable from the agent's container.

**"FLUXX_API_KEY environment variable is not set" on startup** — you
have `FLUXX_API_URL` set but no key. Either set both (hosted mode) or
unset `FLUXX_API_URL` to fall into self-hosted mode.
