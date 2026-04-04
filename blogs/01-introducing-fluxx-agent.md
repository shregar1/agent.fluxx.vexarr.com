# Fluxx Agent: A Developer's Guide to 18× Cheaper Observability

**Published:** April 2026  
**Reading time:** 8 minutes  
**Who this is for:** Engineers tired of $18M Datadog bills

---

## The Problem in Code

You're debugging a production issue. You need to check logs:

```bash
# Datadog: Wait for indexing (2-5 minutes delay)
datadog query "status:error service:payments"
# "Results will be available shortly..."

# Meanwhile, your customers are waiting
```

Or you want to add a custom metric:

```python
# Datadog: Hope they support it
datadog.distribution("payment.latency", latency)
# $5 per custom metric × 100 metrics = $500/month extra
```

**The developer experience is terrible and expensive.**

---

## Meet Fluxx Agent

A Rust-powered observability engine that runs on your infrastructure. Same features as Datadog. 1/18th the cost. 600× faster.

```bash
# One-line install
curl -sSL https://fluxx.dev/install.sh | bash

# Start collecting
cargo run --release

# Query instantly
fluxx query "SELECT * FROM events WHERE level = 'error'"
```

---

## Quick Start (5 minutes)

### 1. Install

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/fluxx-dev/fluxx-agent
cd fluxx-agent && cargo build --release
```

### 2. Configure

```yaml
# fluxx.yaml
server:
  bind: 0.0.0.0:8080
  
ingestion:
  buffer_size: 10000
  compression: zstd
  
storage:
  backend: s3
  bucket: my-observability
  region: us-east-1
  
retention:
  hot: 24h      # SSD
  warm: 7d      # EBS
  cold: 30d     # S3
```

### 3. Run

```bash
# Start the agent
./target/release/fluxx-agent --config fluxx.yaml

# Send a test event
curl -X POST http://localhost:8080/v1/events \
  -H "Content-Type: application/json" \
  -d '{
    "id": "evt-123",
    "service": "api-gateway",
    "level": "error",
    "message": "Database connection failed",
    "timestamp": 1712234567890
  }'

# Query immediately
fluxx query "SELECT * FROM events WHERE service = 'api-gateway'"
```

---

## Developer Experience Comparison

### Adding Custom Metrics

**Datadog:**
```python
from datadog import statsd

# Hope this metric name doesn't conflict
# Hope you don't exceed your custom metric limit
# Hope you can afford it
statsd.gauge('myapp.users.active', 42)
```

**Fluxx Agent:**
```rust
// Add any metric, any time, no limits
use fluxx_agent::metrics;

metrics::gauge!("myapp.users.active", 42.0,
    "region" => "us-east-1",
    "version" => "v1.2.3"
);

// Or from your application
curl -X POST http://localhost:8080/v1/metrics \
  -d '{
    "name": "myapp.users.active",
    "value": 42,
    "tags": {"region": "us-east-1"}
  }'
```

**No limits. No extra cost.**

### Querying Data

**Datadog:**
```python
# Datadog Query Language (limited)
"avg:system.cpu.user{service:api} by {host}"

# Complex queries? Write a Python script.
# Export to CSV? Pay for API access.
```

**Fluxx Agent:**
```sql
-- Full SQL support
SELECT 
    host,
    AVG(cpu_user) as avg_cpu,
    PERCENTILE(cpu_user, 99) as p99_cpu,
    COUNT(*) as sample_count
FROM metrics
WHERE service = 'api'
    AND timestamp > NOW() - INTERVAL '1 hour'
GROUP BY host
HAVING avg_cpu > 80
ORDER BY p99_cpu DESC;
```

```bash
# Query from CLI
fluxx query "SELECT * FROM logs WHERE level = 'error' LIMIT 10"

# Query from code
let errors = fluxx.query(r#"
    SELECT * FROM logs 
    WHERE level = 'error' 
    AND service = ?
"#, &["payments"]).await?;
```

### Debugging Production Issues

**Datadog:**
```
1. Find the trace
2. Wait for indexing
3. Click through UI
4. Can't find the log
5. Escalate to support
6. Wait 24 hours
```

**Fluxx Agent:**
```bash
# SSH into your server (you own the data)
ssh production-server

# Query logs directly
fluxx query "SELECT * FROM logs 
  WHERE trace_id = 'abc-123' 
  ORDER BY timestamp"

# Or grep the raw files (they're just compressed)
zcat /var/lib/fluxx/logs/2026-04-01/0001.parquet.zst | \
  jq 'select(.trace_id == "abc-123")'
```

---

## Architecture for Developers

```
┌─────────────────────────────────────────────────────────────────┐
│                        YOUR APPLICATION                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐      │
│  │   App Code   │───▶│ OpenTelemetry│───▶│  Fluxx Agent │      │
│  └──────────────┘    └──────────────┘    └──────┬───────┘      │
│                                                 │               │
│                                                 ▼               │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐      │
│  │   Grafana    │◄───│  Query API   │◄───│   Storage    │      │
│  └──────────────┘    └──────────────┘    └──────────────┘      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                        YOUR INFRASTRUCTURE
```

**Everything runs in your VPC. You control everything.**

---

## Code Examples

### Rust Integration

```rust
use fluxx_agent::{Event, FluxxClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = FluxxClient::new("http://localhost:8080");
    
    // Send structured event
    let event = Event::new("payment-processed")
        .with_service("payments")
        .with_duration_ms(150)
        .with_tags(&[
            ("user_id", "user-123"),
            ("amount", "99.99"),
            ("currency", "USD"),
        ]);
    
    client.send(event).await?;
    
    // Query data back
    let slow_payments = client.query(r#"
        SELECT * FROM events 
        WHERE service = 'payments'
        AND duration_ms > 100
        ORDER BY timestamp DESC
        LIMIT 100
    "#).await?;
    
    println!("Found {} slow payments", slow_payments.len());
    
    Ok(())
}
```

### Python Integration

```python
from fluxx import FluxxClient

client = FluxxClient("http://localhost:8080")

# Send event
client.send_event({
    "name": "api-request",
    "service": "auth-service",
    "duration_ms": 45,
    "tags": {
        "endpoint": "/login",
        "method": "POST",
        "status_code": 200
    }
})

# Query with SQL
errors = client.query("""
    SELECT 
        endpoint,
        COUNT(*) as error_count,
        AVG(duration_ms) as avg_latency
    FROM events
    WHERE service = 'auth-service'
    AND status_code >= 500
    AND timestamp > NOW() - INTERVAL '1 hour'
    GROUP BY endpoint
    ORDER BY error_count DESC
""")

for row in errors:
    print(f"{row['endpoint']}: {row['error_count']} errors")
```

### Go Integration

```go
package main

import (
    "context"
    "log"
    "time"
    
    "github.com/fluxx-dev/fluxx-go"
)

func main() {
    client, err := fluxx.NewClient("http://localhost:8080")
    if err != nil {
        log.Fatal(err)
    }
    
    // Send event
    ctx := context.Background()
    err = client.SendEvent(ctx, fluxx.Event{
        Name:      "request-completed",
        Service:   "api-gateway",
        Timestamp: time.Now(),
        Duration:  150 * time.Millisecond,
        Tags: map[string]string{
            "path":   "/api/users",
            "method": "GET",
        },
    })
    
    if err != nil {
        log.Printf("Failed to send event: %v", err)
    }
}
```

---

## Configuration as Code

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: fluxx-agent
spec:
  selector:
    matchLabels:
      app: fluxx-agent
  template:
    metadata:
      labels:
        app: fluxx-agent
    spec:
      containers:
      - name: fluxx-agent
        image: fluxx/agent:latest
        resources:
          requests:
            memory: "256Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "1000m"
        volumeMounts:
        - name: config
          mountPath: /etc/fluxx
        - name: data
          mountPath: /var/lib/fluxx
      volumes:
      - name: config
        configMap:
          name: fluxx-config
      - name: data
        hostPath:
          path: /var/lib/fluxx
```

### Terraform

```hcl
resource "aws_instance" "fluxx" {
  ami           = "ami-12345678"
  instance_type = "c6i.16xlarge"
  
  user_data = <<-EOF
              #!/bin/bash
              curl -sSL https://fluxx.dev/install.sh | bash
              systemctl start fluxx-agent
              EOF
  
  tags = {
    Name = "fluxx-observability"
  }
}

resource "aws_s3_bucket" "fluxx_storage" {
  bucket = "my-company-observability"
}
```

---

## API Reference

### Ingestion API

```bash
# POST /v1/events
# Content-Type: application/json

{
  "id": "string (optional, auto-generated)",
  "timestamp": "int64 (microseconds since epoch)",
  "service": "string (required)",
  "name": "string (required)",
  "level": "enum: debug|info|warn|error|fatal",
  "message": "string",
  "duration_ms": "float64 (optional)",
  "tags": "map<string,string> (optional)",
  "metadata": "map<string,any> (optional)"
}

# Response: 201 Created
{
  "id": "evt-abc-123",
  "status": "accepted"
}
```

### Query API

```bash
# POST /v1/query
# Content-Type: application/json

{
  "query": "SELECT * FROM events WHERE service = 'api' LIMIT 100",
  "format": "json"  // or "csv", "parquet"
}

# Response: 200 OK
{
  "columns": ["id", "timestamp", "service", "message"],
  "rows": [
    ["evt-1", 1712234567890, "api", "Request completed"],
    // ...
  ],
  "row_count": 100,
  "query_time_ms": 45
}
```

### Health Check

```bash
# GET /health
{
  "status": "healthy",
  "version": "1.2.3",
  "uptime_seconds": 86400,
  "metrics": {
    "events_ingested_total": 1000000000,
    "storage_used_bytes": 1099511627776,
    "query_latency_ms_p99": 50
  }
}
```

---

## Troubleshooting

### High Memory Usage

```bash
# Check memory usage
fluxx-cli stats memory

# Solution: Reduce buffer size
# In fluxx.yaml:
ingestion:
  buffer_size: 1000  # default: 10000
```

### Slow Queries

```bash
# Enable query profiling
fluxx-cli query-profile --enable

# Check slow queries
fluxx-cli query-log --slow-only

# Solution: Add indexes
fluxx-cli index create --table events --column service
```

### Data Not Appearing

```bash
# Check ingestion
fluxx-cli ingestion status

# Verify event format
curl -X POST http://localhost:8080/v1/events \
  -d '{"test": "malformed"}' -v

# Check logs
tail -f /var/log/fluxx/agent.log
```

---

## Migration from Datadog

### Step 1: Export Your Dashboards

```bash
# Export all Datadog dashboards
datadog dashboard list | jq -r '.id' | while read id; do
  datadog dashboard get "$id" > "dashboard-$id.json"
done

# Convert to Grafana format
fluxx-convert dashboards --from datadog --to grafana
```

### Step 2: Update Your Code

```python
# Before (Datadog)
from datadog import statsd
statsd.increment('api.requests')

# After (Fluxx)
from fluxx import FluxxClient
client = FluxxClient()
client.counter('api.requests', 1)
```

### Step 3: Dual-Write During Transition

```python
# Write to both during migration
from datadog import statsd
from fluxx import FluxxClient

datadog_client = statsd
fluxx_client = FluxxClient()

def record_metric(name, value):
    datadog_client.gauge(name, value)  # Legacy
    fluxx_client.gauge(name, value)     # New
```

---

## Performance Tuning

### For High Throughput (>1M events/sec)

```yaml
# fluxx-high-throughput.yaml
server:
  workers: 16  # Increase parallelism
  
ingestion:
  buffer_size: 100000
  batch_size: 1000
  compression:
    enabled: true
    level: 1  # Speed over compression
    
storage:
  backend: s3
  s3:
    multipart_upload: true
    part_size: 100MB
    
query:
  cache:
    enabled: true
    size: 10GB
```

### For Low Latency (<10ms queries)

```yaml
# fluxx-low-latency.yaml
storage:
  hot_tier:
    backend: local_ssd
    retention: 24h
    
query:
  cache:
    enabled: true
    preload_hot_data: true
    
indexes:
  - table: events
    columns: [service, timestamp]
  - table: events
    columns: [trace_id]
```

---

## Community

- **GitHub:** [github.com/fluxx-dev/fluxx-agent](https://github.com/fluxx-dev/fluxx-agent)
- **Discord:** [discord.gg/fluxx](https://discord.gg/fluxx)
- **Docs:** [docs.fluxx.dev](https://docs.fluxx.dev)
- **Examples:** [github.com/fluxx-dev/examples](https://github.com/fluxx-dev/examples)

---

## Get Started

```bash
# Clone and run
git clone https://github.com/fluxx-dev/fluxx-agent
cd fluxx-agent
cargo build --release
./target/release/fluxx-agent

# Or use Docker
docker run -p 8080:8080 fluxx/agent:latest
```

**Stop paying $18M for observability. Build it yourself for $1M.**

---

*Questions? Open an issue on GitHub or ask in Discord.*
