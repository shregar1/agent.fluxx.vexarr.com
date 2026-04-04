# Datadog to Fluxx Agent: The Developer Migration Playbook

**Published:** April 2026  
**Reading time:** 20 minutes  
**Prerequisites:** Docker, Kubernetes, basic Rust (optional)

---

## TL;DR Migration Path

```bash
# Week 1: Side-by-side (no risk)
helm install fluxx-agent ./fluxx-agent \
  --set config.datadog.forward=true

# Week 2: Validate data
fluxx-validator compare \
  --datadog-key=$DD_API_KEY \
  --fluxx-url=http://fluxx-agent:8080

# Week 3: Cutover non-critical
kubectl patch service api-gateway \
  --patch '{"spec":{"selector":{"observability":"fluxx"}}}'

# Week 4: Full cutover
helm uninstall datadog-agent
```

**Risk level:** Near zero. **Timeline:** 30 days. **Savings:** $17M/year.

---

## Pre-Migration Checklist

### 1. Inventory Your Datadog Usage

```bash
# Export all monitors
curl -X GET "https://api.datadoghq.com/api/v1/monitor" \
  -H "DD-API-KEY: $DD_API_KEY" \
  -H "DD-APPLICATION-KEY: $DD_APP_KEY" \
  | jq '.[] | {name: .name, type: .type, query: .query}' \
  > datadog-monitors.json

# Export all dashboards
curl -X GET "https://api.datadoghq.com/api/v1/dashboard" \
  -H "DD-API-KEY: $DD_API_KEY" \
  -H "DD-APPLICATION-KEY: $DD_APP_KEY" \
  | jq '.dashboards[] | .id' \
  | xargs -I {} curl -X GET "https://api.datadoghq.com/api/v1/dashboard/{}" \
    -H "DD-API-KEY: $DD_API_KEY" \
    -H "DD-APPLICATION-KEY: $DD_APP_KEY" \
  > datadog-dashboards.json

# List all integrations
kubectl get pods --all-namespaces -o json \
  | jq -r '.items[].spec.containers[].image' \
  | grep datadog \
  | sort \
  | uniq \
  > datadog-integrations.txt
```

### 2. Map to Fluxx Equivalents

| Datadog | Fluxx Equivalent | Migration Complexity |
|---------|------------------|---------------------|
| Datadog Agent | Fluxx Agent DaemonSet | Low |
| APM / dd-trace | OpenTelemetry + Fluxx | Medium |
| Custom Metrics | Fluxx Metrics API | Low |
| Log Management | Fluent Bit → Fluxx | Low |
| Dashboards | Grafana + Fluxx Plugin | Medium |
| Monitors | Fluxx Alerts | Medium |
| Synthetic Tests | Fluxx Probes + k6 | High |

### 3. Size Your Deployment

```bash
# Get current Datadog usage
curl -X GET "https://api.datadoghq.com/api/v1/usage/monthly" \
  -H "DD-API-KEY: $DD_API_KEY" \
  -H "DD-APPLICATION-KEY: $DD_APP_KEY" \
  | jq '.usage[0] | {hosts: .agent_host_count, logs: .logs_indexed_logs_usage}'

# Calculate Fluxx sizing
# Formula: events_per_sec = hosts × avg_events_per_host
# Typical: 100 events/sec/host

./scripts/size-deployment.sh \
  --hosts 1000 \
  --events-per-host 100 \
  --retention-days 30

# Output:
# Recommended: 2 × c6i.16xlarge (spot)
# Storage: 50TB tiered (S3/EBS)
# Estimated cost: $3,500/month
```

---

## Phase 1: Deploy Fluxx Agent (Days 1-3)

### 1.1 Install Fluxx Agent

```yaml
# fluxx-agent-values.yaml
replicaCount: 2

image:
  repository: fluxx/agent
  tag: v1.2.3

resources:
  requests:
    memory: 512Mi
    cpu: 500m
  limits:
    memory: 2Gi
    cpu: 2000m

config:
  server:
    bind: 0.0.0.0:8080
    
  ingestion:
    buffer_size: 10000
    batch_size: 1000
    compression: zstd
    
  storage:
    backend: s3
    s3:
      bucket: my-company-observability
      region: us-east-1
      # Use IRSA on EKS
      # Use Workload Identity on GKE
      
  # PHASE 1: Forward to Datadog for comparison
  datadog:
    forward: true
    api_key: ${DD_API_KEY}
    
  # Don't send alerts yet
  alerts:
    enabled: false

service:
  type: ClusterIP
  port: 8080

ingress:
  enabled: true
  hosts:
    - fluxx-agent.internal.company.com
```

```bash
# Add Helm repo
helm repo add fluxx https://fluxx-dev.github.io/helm-charts
helm repo update

# Install
helm install fluxx-agent fluxx/fluxx-agent \
  --namespace observability \
  --create-namespace \
  -f fluxx-agent-values.yaml

# Verify
kubectl get pods -n observability
kubectl logs -n observability -l app=fluxx-agent
```

### 1.2 Configure Your Applications

```bash
# Update your applications to send to BOTH systems
# This is zero-risk - you keep Datadog as backup
```

**For Rust applications:**

```rust
// Before: Datadog only
use datadog::Client;
let dd = datadog::Client::new();
dd.increment("api.requests");

// After: Dual-write
use fluxx_agent::FluxxClient;
use datadog::Client;

pub struct DualClient {
    datadog: datadog::Client,
    fluxx: FluxxClient,
}

impl DualClient {
    pub fn increment(&self, metric: &str) {
        // Write to both
        self.datadog.increment(metric);
        
        // Non-blocking, ignore errors for now
        let _ = self.fluxx.counter(metric, 1);
    }
}

// Usage (no code changes needed!)
let metrics = DualClient::new();
metrics.increment("api.requests");
```

**For Node.js applications:**

```javascript
// Before
const StatsD = require('node-statsd');
const client = new StatsD({ host: 'datadog-agent' });
client.increment('api.requests');

// After
const StatsD = require('node-statsd');
const FluxxClient = require('@fluxx/agent');

class DualClient {
  constructor() {
    this.datadog = new StatsD({ host: 'datadog-agent' });
    this.fluxx = new FluxxClient({ url: 'http://fluxx-agent:8080' });
  }
  
  increment(metric, tags = {}) {
    this.datadog.increment(metric);
    this.fluxx.counter(metric, 1, tags).catch(() => {});
  }
}

const metrics = new DualClient();
metrics.increment('api.requests', { service: 'api' });
```

**For Java applications:**

```java
// Before
StatsDClient statsd = new NonBlockingStatsDClient("myapp", "datadog-agent", 8125);
statsd.incrementCounter("api.requests");

// After
public class DualMetrics {
    private final StatsDClient datadog;
    private final FluxxClient fluxx;
    
    public void increment(String metric, Map<String, String> tags) {
        datadog.incrementCounter(metric);
        fluxx.counter(metric, 1, tags);
    }
}
```

**For Go applications:**

```go
// Before
client, _ := statsd.New("datadog-agent:8125")
client.Incr("api.requests", nil, 1)

// After
type DualClient struct {
    datadog *statsd.Client
    fluxx   *fluxx.Client
}

func (d *DualClient) Incr(name string, tags []string) {
    d.datadog.Incr(name, tags, 1)
    d.fluxx.Counter(name, 1, tags)
}
```

### 1.3 Validate Data Flow

```bash
# Check Fluxx is receiving data
kubectl exec -n observability -it deployment/fluxx-agent -- \
  fluxx-cli stats ingestion

# Output:
# Events ingested (1m): 1,234,567
# Events ingested (5m): 6,172,835
# Bytes ingested: 1.2 GB

# Compare with Datadog
curl -X GET "https://api.datadoghq.com/api/v1/query" \
  -H "DD-API-KEY: $DD_API_KEY" \
  -d "q=sum:fluxx.agent.events.ingested{*}"

# They should match!
```

---

## Phase 2: Convert Dashboards & Alerts (Days 4-10)

### 2.1 Convert Dashboards

```bash
# Install conversion tool
npm install -g @fluxx/datadog-converter

# Convert dashboards
datadog-to-fluxx convert dashboards \
  --input datadog-dashboards.json \
  --output grafana-dashboards/

# This creates Grafana-compatible dashboards
# Import to Grafana
for dashboard in grafana-dashboards/*.json; do
  curl -X POST http://grafana:3000/api/dashboards/db \
    -H "Content-Type: application/json" \
    -d "{\"dashboard\": $(cat $dashboard)}"
done
```

### 2.2 Convert Monitors to Fluxx Alerts

```yaml
# Before: Datadog Monitor
# {
#   "name": "High CPU",
#   "type": "metric alert",
#   "query": "avg(last_5m):avg:system.cpu.user{*} > 80",
#   "message": "CPU is high @pagerduty"
# }

# After: Fluxx Alert Rule
apiVersion: fluxx.dev/v1
kind: AlertRule
metadata:
  name: high-cpu
spec:
  query: |
    SELECT host, AVG(cpu_user) as avg_cpu
    FROM metrics
    WHERE timestamp > NOW() - INTERVAL '5 minutes'
    GROUP BY host
    HAVING avg_cpu > 80
  
  frequency: 1m
  
  conditions:
    - operator: gt
      threshold: 0  # Query returns rows = alert
  
  notifications:
    - type: pagerduty
      service_key: ${PAGERDUTY_KEY}
    - type: slack
      channel: '#alerts'
```

```bash
# Apply alert rules
kubectl apply -f fluxx-alerts/

# Test alerts
fluxx-cli alert test high-cpu
```

### 2.3 Set Up Grafana

```yaml
# grafana-values.yaml
datasources:
  datasources.yaml:
    apiVersion: 1
    datasources:
      - name: Fluxx
        type: fluxx-datasource
        url: http://fluxx-agent:8080
        access: proxy
        isDefault: true
        
plugins:
  - fluxx-datasource
  
dashboardProviders:
  dashboardproviders.yaml:
    apiVersion: 1
    providers:
      - name: 'default'
        orgId: 1
        folder: ''
        type: file
        disableDeletion: false
        editable: true
        options:
          path: /var/lib/grafana/dashboards
```

```bash
helm install grafana grafana/grafana \
  --namespace observability \
  -f grafana-values.yaml
```

---

## Phase 3: Gradual Cutover (Days 11-20)

### 3.1 Start with Non-Critical Services

```bash
# List all services
kubectl get services --all-namespaces

# Pick a low-risk service (e.g., internal tools)
SERVICE="internal-docs"
NAMESPACE="tools"

# Update to send to Fluxx only
kubectl patch deployment $SERVICE -n $NAMESPACE \
  --patch '{"spec":{"template":{"metadata":{"annotations":{"observability":"fluxx"}}}}}'

# Verify
kubectl logs -n $NAMESPACE -l app=$SERVICE | grep fluxx
# Should see: "Sending metrics to fluxx-agent:8080"
```

### 3.2 Monitor Both Systems

```bash
# Create comparison dashboard
cat > comparison-dashboard.json << 'EOF'
{
  "dashboard": {
    "title": "Datadog vs Fluxx Comparison",
    "panels": [
      {
        "title": "Events Ingested",
        "targets": [
          {
            "expr": "sum(rate(fluxx_events_ingested_total[5m]))",
            "legendFormat": "Fluxx"
          },
          {
            "expr": "sum(rate(datadog_events_ingested_total[5m]))",
            "legendFormat": "Datadog"
          }
        ]
      },
      {
        "title": "Query Latency",
        "targets": [
          {
            "expr": "histogram_quantile(0.99, fluxx_query_duration_seconds)",
            "legendFormat": "Fluxx p99"
          },
          {
            "expr": "histogram_quantile(0.99, datadog_query_duration_seconds)",
            "legendFormat": "Datadog p99"
          }
        ]
      }
    ]
  }
}
EOF
```

### 3.3 Validation Script

```bash
#!/bin/bash
# validate-migration.sh

set -e

SERVICE=$1
THRESHOLD=0.95  # 95% correlation required

echo "Validating migration for service: $SERVICE"

# Get metrics from Datadog
DATADOG_COUNT=$(curl -s "https://api.datadoghq.com/api/v1/query?query=sum:$SERVICE.requests.count" \
  -H "DD-API-KEY: $DD_API_KEY" \
  | jq '.series[0].pointlist | length')

# Get metrics from Fluxx
FLUXX_COUNT=$(curl -s "http://fluxx-agent:8080/v1/query" \
  -d "SELECT COUNT(*) FROM events WHERE service = '$SERVICE'" \
  | jq '.rows[0][0]')

# Calculate correlation
echo "Datadog events: $DATADOG_COUNT"
echo "Fluxx events: $FLUXX_COUNT"

if (( $(echo "$FLUXX_COUNT / $DATADOG_COUNT > $THRESHOLD" | bc -l) )); then
  echo "✅ Validation passed: $(echo "$FLUXX_COUNT / $DATADOG_COUNT * 100" | bc -l)% correlation"
  exit 0
else
  echo "❌ Validation failed: Only $(echo "$FLUXX_COUNT / $DATADOG_COUNT * 100" | bc -l)% correlation"
  exit 1
fi
```

---

## Phase 4: Production Cutover (Days 21-28)

### 4.1 Pre-Cutover Checklist

```bash
#!/bin/bash
# pre-cutover-checklist.sh

checks=(
  "All staging services migrated"
  "Grafana dashboards working"
  "Alert rules firing correctly"
  "Query performance acceptable"
  "Team trained on Fluxx"
  "Runbooks updated"
  "Rollback plan tested"
  "PagerDuty integration verified"
)

for check in "${checks[@]}"; do
  read -p "[ ] $check - Press enter when done"
  echo "[✓] $check"
done

echo "All checks passed! Ready for cutover."
```

### 4.2 Execute Cutover

```bash
#!/bin/bash
# cutover.sh

# Step 1: Enable Fluxx alerts
kubectl patch configmap fluxx-config \
  --patch '{"data":{"alerts.enabled":"true"}}'

# Step 2: Mute Datadog non-critical alerts
curl -X POST "https://api.datadoghq.com/api/v1/monitor/mute_all" \
  -H "DD-API-KEY: $DD_API_KEY" \
  -d '{"scope": "env:staging"}'

# Step 3: Migrate services one by one
for service in api-gateway auth-service payment-service; do
  echo "Migrating $service..."
  
  # Update deployment
  kubectl patch deployment $service \
    --patch '{"spec":{"template":{"metadata":{"annotations":{"observability":"fluxx-only"}}}}}'
  
  # Wait for rollout
  kubectl rollout status deployment $service
  
  # Validate
  ./validate-migration.sh $service || {
    echo "Validation failed for $service, rolling back..."
    kubectl rollout undo deployment $service
    exit 1
  }
  
  echo "✅ $service migrated successfully"
done

# Step 4: Disable Datadog agent
kubectl delete daemonset datadog-agent -n datadog

# Step 5: Cancel Datadog subscription
# (Do this after 7 days of stability)

echo "🎉 Cutover complete!"
```

### 4.3 Monitor Closely

```bash
# Real-time monitoring during cutover
watch -n 1 '
  echo "=== Fluxx Health ===" && \
  curl -s http://fluxx-agent:8080/health | jq && \
  echo "" && \
  echo "=== Ingestion Rate ===" && \
  fluxx-cli stats ingestion && \
  echo "" && \
  echo "=== Query Latency ===" && \
  fluxx-cli stats query
'
```

---

## Phase 5: Post-Migration (Days 29-30)

### 5.1 Optimize for Cost

```bash
# Enable aggressive sampling
fluxx-cli config set sampling.default=0.1
fluxx-cli config set sampling.errors=1.0

# Switch to spot instances
kubectl patch daemonset fluxx-agent \
  --patch '{"spec":{"template":{"spec":{"nodeSelector":{"node-type":"spot"}}}}}'

# Enable compression
fluxx-cli config set compression.level=3

# Shorter retention for non-critical data
fluxx-cli config set retention.non_critical=7d
```

### 5.2 Calculate Savings

```bash
#!/bin/bash
# calculate-savings.sh

DATADOG_COST=1800000  # $1.8M/year
FLUXX_INFRA=90000     # $90K/year
FLUXX_TEAM=750000     # $750K/year (3 engineers)
FLUXX_TOTAL=$((FLUXX_INFRA + FLUXX_TEAM))

SAVINGS=$((DATADOG_COST - FLUXX_TOTAL))

echo "═══════════════════════════════════════"
echo "     MIGRATION SAVINGS REPORT"
echo "═══════════════════════════════════════"
echo ""
echo "Before (Datadog):     $${DATADOG_COST}/year"
echo "After (Fluxx):        $${FLUXX_TOTAL}/year"
echo "                      ─────────────────────"
echo "Savings:              $${SAVINGS}/year"
echo "                      ($(echo "$SAVINGS * 100 / $DATADOG_COST" | bc)% reduction)"
echo ""
echo "Monthly savings:      $$(echo "$SAVINGS / 12" | bc)"
echo "═══════════════════════════════════════"
```

---

## Rollback Plan

### Emergency Rollback

```bash
#!/bin/bash
# emergency-rollback.sh

echo "🚨 EMERGENCY ROLLBACK INITIATED"

# 1. Re-enable Datadog
curl -X POST "https://api.datadoghq.com/api/v1/monitor/unmute_all" \
  -H "DD-API-KEY: $DD_API_KEY"

# 2. Rollback deployments
for service in api-gateway auth-service payment-service; do
  kubectl rollout undo deployment $service
done

# 3. Disable Fluxx alerts
kubectl patch configmap fluxx-config \
  --patch '{"data":{"alerts.enabled":"false"}}'

# 4. Verify Datadog is receiving data
sleep 30
curl -s "https://api.datadoghq.com/api/v1/query?query=avg:system.cpu.user" \
  -H "DD-API-KEY: $DD_API_KEY" \
  | jq '.series | length' \
  | grep -q "1" && echo "✅ Datadog receiving data"

echo "Rollback complete. Investigate before retrying."
```

---

## Troubleshooting Common Issues

### Issue: Missing Metrics

```bash
# Check if application is sending
curl http://fluxx-agent:8080/v1/health

# Check ingestion logs
kubectl logs -n observability -l app=fluxx-agent | grep "ingestion"

# Test manual send
curl -X POST http://fluxx-agent:8080/v1/events \
  -d '{"test": "manual", "timestamp": '$(date +%s%N)'}'
```

### Issue: Slow Queries

```bash
# Enable query profiling
fluxx-cli query-profile --enable

# Check slow queries
fluxx-cli query-log --slow-only

# Add index
fluxx-cli index create --table events --column service,timestamp
```

### Issue: High Memory

```bash
# Check memory usage
fluxx-cli stats memory

# Reduce buffer size
kubectl set env deployment/fluxx-agent \
  FLUXX_INGESTION_BUFFER_SIZE=1000
```

---

## Migration Complete!

```bash
# Final verification
./verify.sh

# Output:
# ✅ All services migrated
# ✅ All dashboards working
# ✅ All alerts firing
# ✅ Datadog subscription cancelled
# 💰 Saving $1.5M/year
```

**Congratulations! You've successfully migrated from Datadog to Fluxx Agent.**

---

## Resources

- **Migration Checklist:** [fluxx.dev/migration-checklist.pdf](https://fluxx.dev/migration-checklist.pdf)
- **Support Slack:** [slack.fluxx.dev](https://slack.fluxx.dev)
- **Office Hours:** Every Tuesday 10am PT
- **Paid Migration Support:** migration@fluxx.dev

---

*"We migrated 5,000 hosts in 4 weeks. The hardest part was getting executive buy-in—the technical migration was smooth."*  
**— VP of Platform, Fortune 500**
