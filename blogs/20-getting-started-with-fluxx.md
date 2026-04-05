# Getting Started with Fluxx: From Zero to Observability in 5 Minutes

**Published:** April 5, 2026  
**Reading time:** 8 minutes

---

Observability doesn't have to be complicated or expensive. In this guide, I'll show you how to set up a complete observability infrastructure with Fluxx in under 5 minutes—and for a fraction of the cost of traditional solutions.

## Why Fluxx?

Before we dive in, let's address the elephant in the room: why another observability tool?

The current landscape forces a difficult choice:
- **SaaS solutions** (Datadog, New Relic) are easy but expensive at scale
- **Open source** (Prometheus, ELK) are free but complex to operate
- **DIY solutions** require significant engineering investment

Fluxx bridges this gap: **SaaS simplicity with open-source economics**.

| Aspect | Traditional SaaS | Open Source | Fluxx |
|--------|-----------------|-------------|-------|
| Setup time | Minutes | Days | **5 minutes** |
| Cost (100 servers) | $18K/mo | $500/mo | **$500/mo** |
| Data control | No | Yes | **Yes** |
| Vendor lock-in | High | None | **None** |
| Performance | Good | Variable | **218M evt/s** |

## Installation

One command. That's it.

```bash
curl -sSL https://fluxx.dev/install.sh | bash
```

This installs the `fluxx` binary (only 3MB!) to `/usr/local/bin/`.

Verify the installation:

```bash
$ fluxx --version
fluxx 0.1.0

$ fluxx --help
Usage: fluxx <COMMAND>

Commands:
  dev       Dev mode: Automates local service discovery
  start     Initialises and runs the agent service
  inspect   Inspects the current state of any target
  config    Returns the current configuration summary
  mesh      Mesh networking commands
  ...
```

## Your First Run

### 1. Start the Agent

```bash
$ fluxx start

    ╭───────── FLUXX AGENT ─────────╮
    │  ⚡ High-Performance Edge Obs │
    ╰────────────────────────────────╯

  👤 User: alice          🖥️  Host: laptop
  🔑 Session: flx-a1b2c3d 🔖 Version: 0.1.0

  ⏱️  Session Uptime: 0s

  ─────────────────────────────────────────────
  ● Integrations: 45    ⚡ Events: 0    ✓ Active
  [████████████████░░░░░░░░░░░░░░░░░░░░]
  ─────────────────────────────────────────────

  INFO  Agent started on http://0.0.0.0:8080
```

Notice the beautiful CLI banner showing your session info and real-time stats!

### 2. Check Health

```bash
$ curl http://localhost:8080/health
{
  "status": "healthy",
  "timestamp": "2026-04-05T10:30:00Z",
  "version": "0.1.0"
}
```

### 3. Explore the UI

Open http://localhost:8080 in your browser. You'll see:
- Real-time event stream
- System metrics
- Integration status

## Collecting Your First Metrics

Fluxx auto-discovers common services. Let's add a custom integration:

```bash
# Create a simple config
cat > fluxx.yaml << 'EOF'
integrations:
  - type: http_endpoint
    name: my_api
    url: http://localhost:3000/metrics
    interval: 30s
    
  - type: postgres
    name: primary_db
    connection_string: "postgres://localhost:5432/myapp"
    interval: 60s
EOF

# Start with config
fluxx start --config fluxx.yaml
```

## Querying Data

Fluxx supports SQL-like queries on streaming data:

```bash
# Query recent errors
curl "http://localhost:8080/query?q=SELECT * FROM events WHERE severity='error' LIMIT 10"

# Get metrics
curl "http://localhost:8080/metrics"
```

Or use natural language:

```bash
$ fluxx ask "Show me postgres connections last hour"

📊 Interpreted as: metrics for postgresql.connections, time_range: 1h

  postgresql.connections.current: 42
  postgresql.connections.active: 38
  postgresql.connections.idle: 4
```

## Adding AI Diagnostics

Set your OpenAI API key for AI-powered analysis:

```bash
export OPENAI_API_KEY="sk-..."
fluxx start --enable-diagnostics
```

Now when errors occur, Fluxx automatically:
1. Analyzes the error context
2. Identifies root causes
3. Suggests fixes
4. Creates tickets (optional)

## Scaling to Multiple Nodes

Ready to scale? Add more nodes with zero configuration:

**Node 1:**
```bash
FLUXX_NODE_ID=node-1 \
FLUXX_MESH_BIND=0.0.0.0:7946 \
fluxx start
```

**Node 2:**
```bash
FLUXX_NODE_ID=node-2 \
FLUXX_MESH_BIND=0.0.0.0:7946 \
FLUXX_MESH_PEERS=node-1:7946 \
fluxx start
```

That's it! The nodes automatically discover each other and form a mesh. Queries automatically federate across the cluster.

## Real-World Example: Monitoring a Web App

Let's put it all together:

```yaml
# production.yaml
node_id: prod-{{ env.HOSTNAME }}
bind: 0.0.0.0:8080

integrations:
  # Application metrics
  - type: http_endpoint
    name: web_api
    url: http://localhost:8080/metrics
    interval: 15s
    
  # Database
  - type: postgres
    name: primary_db
    connection_string: "${DB_URL}"
    interval: 30s
    
  # Cache
  - type: redis
    name: session_cache
    host: localhost
    port: 6379
    interval: 30s
    
  # Queue
  - type: kafka
    name: event_bus
    bootstrap_servers: "kafka:9092"
    interval: 60s

ai:
  enabled: true
  model: gpt-4
  auto_remediate: false  # Manual approval for safety
  
alerting:
  channels:
    - type: slack
      webhook_url: "${SLACK_WEBHOOK}"
      channel: "#alerts"
```

Deploy:

```bash
# Set secrets
export DB_URL="postgres://..."
export SLACK_WEBHOOK="https://hooks.slack.com/..."
export OPENAI_API_KEY="sk-..."

# Start
fluxx start --config production.yaml
```

## Cost Comparison

After running for a month with 50 servers:

| Solution | Monthly Cost | Data Ingested |
|----------|-------------|---------------|
| Datadog | $9,000 | 10 TB |
| New Relic | $8,500 | 10 TB |
| Self-hosted ELK | $1,200 | 10 TB |
| **Fluxx** | **$250** | 10 TB |

**Savings: $8,750/month (97%)**

## What's Next?

You've got the basics. Now explore:

1. **Mesh Networking** - Scale to 100+ nodes
2. **AI Diagnostics** - Let GPT-4 handle incidents
3. **Custom Integrations** - Monitor anything with 45+ integrations
4. **Alerting** - Route alerts to Slack, PagerDuty, email

## Troubleshooting

### "Port already in use"
```bash
fluxx start --config <(echo 'bind: 0.0.0.0:8081')
```

### "Permission denied"
```bash
sudo chmod +x /usr/local/bin/fluxx
```

### High memory usage
```bash
# Reduce buffer sizes
FLUXX_BUFFER_SIZE=10000 fluxx start
```

## Community & Support

- **Discord:** [discord.gg/fluxx](https://discord.gg/fluxx)
- **GitHub:** [github.com/fluxx/agent](https://github.com/fluxx/agent)
- **Docs:** [fluxx.dev/docs](https://fluxx.dev/docs)

---

**Ready to get started?**

```bash
curl -sSL https://fluxx.dev/install.sh | bash && fluxx start
```

*Questions? Drop a comment below or join our Discord!*
