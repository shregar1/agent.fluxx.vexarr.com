# Fluxx Configuration Guide

Complete guide to configuring Fluxx Agent.

## Configuration Sources

Fluxx reads configuration from (in order of precedence):

1. **Environment variables** (highest priority)
2. **Configuration file** (YAML)
3. **Default values** (lowest priority)

## Configuration File

Default locations (checked in order):

1. `./fluxx.yaml` (current directory)
2. `~/.config/fluxx/config.yaml`
3. `/etc/fluxx/config.yaml`
4. Path specified by `--config` flag

## Complete Configuration Reference

```yaml
# fluxx.yaml - Complete Configuration Example

# =============================================================================
# CORE SETTINGS
# =============================================================================

# Node identification
node_id: "node-1"  # Unique identifier for this node

# API server configuration
server:
  bind: "0.0.0.0:8080"  # Address to bind API server
  tls:
    enabled: false
    cert_file: "/etc/fluxx/tls/cert.pem"
    key_file: "/etc/fluxx/tls/key.pem"
  
  # Authentication
  auth:
    type: "api_key"  # or "jwt", "none"
    api_key_header: "X-API-Key"
    
  # Rate limiting
  rate_limit:
    enabled: true
    requests_per_minute: 1000

# =============================================================================
# LOGGING
# =============================================================================

logging:
  level: "info"  # trace, debug, info, warn, error
  format: "json"  # json, pretty
  output: "stdout"  # stdout, stderr, file
  
  # File output (if output is "file")
  file:
    path: "/var/log/fluxx/agent.log"
    rotation: "daily"  # hourly, daily, never
    max_files: 7
    
  # Structured fields added to every log
  fields:
    service: "fluxx-agent"
    environment: "production"

# =============================================================================
# STORAGE
# =============================================================================

storage:
  # Primary storage backend
  type: "local"  # local, s3, memory
  
  # Local filesystem storage
  local:
    path: "/var/lib/fluxx/data"
    
  # S3-compatible storage
  s3:
    bucket: "my-fluxx-data"
    region: "us-east-1"
    prefix: "events/"
    # Credentials from env: AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY
    
  # Retention policies
  retention:
    hot:
      duration: "7d"
      max_size: "100GB"
    warm:
      duration: "30d"
      max_size: "500GB"
    cold:
      duration: "365d"
      storage: "s3"

# =============================================================================
# MESH NETWORKING
# =============================================================================

mesh:
  enabled: true
  
  # Bind address for mesh communication
  bind: "0.0.0.0:7946"
  
  # Known peers (optional, auto-discovery via gossip)
  peers:
    - "node-2:7946"
    - "node-3:7946"
    
  # Gossip protocol settings
  gossip:
    interval: "100ms"
    fanout: 3
    
  # TLS for mesh communication
  tls:
    enabled: true
    cert: "/etc/fluxx/mesh/cert.pem"
    key: "/etc/fluxx/mesh/key.pem"
    ca: "/etc/fluxx/mesh/ca.pem"
    
  # Query federation
  federation:
    enabled: true
    timeout: "30s"

# =============================================================================
# INTEGRATIONS
# =============================================================================

integrations:
  # ---------------------------------------------------------------------------
  # DATABASES
  # ---------------------------------------------------------------------------
  
  - type: postgres
    name: "postgres-primary"
    enabled: true
    connection_string: "${POSTGRES_URL}"
    interval: "30s"
    timeout: "10s"
    tags:
      environment: "production"
      tier: "critical"
    
  - type: mysql
    name: "mysql-analytics"
    enabled: true
    connection_string: "${MYSQL_URL}"
    interval: "60s"
    
  - type: redis
    name: "redis-cache"
    enabled: true
    host: "localhost"
    port: 6379
    interval: "30s"
    
  # ---------------------------------------------------------------------------
  # CLOUD PROVIDERS
  # ---------------------------------------------------------------------------
  
  - type: aws
    name: "aws-production"
    enabled: true
    region: "us-east-1"
    # Credentials from env: AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY
    services:
      - "ec2"
      - "rds"
      - "elb"
    interval: "60s"
    
  - type: gcp
    name: "gcp-production"
    enabled: true
    project: "my-project"
    # Credentials from env: GOOGLE_APPLICATION_CREDENTIALS
    interval: "60s"
    
  # ---------------------------------------------------------------------------
  # CONTAINERS
  # ---------------------------------------------------------------------------
  
  - type: kubernetes
    name: "k8s-production"
    enabled: true
    kubeconfig: "${HOME}/.kube/config"
    # or use in-cluster config
    # in_cluster: true
    namespaces:
      - "default"
      - "production"
    interval: "30s"
    
  - type: docker
    name: "docker-local"
    enabled: true
    socket: "/var/run/docker.sock"
    interval: "30s"
    
  # ---------------------------------------------------------------------------
  # QUEUES
  # ---------------------------------------------------------------------------
  
  - type: kafka
    name: "kafka-main"
    enabled: true
    bootstrap_servers: "kafka-1:9092,kafka-2:9092"
    interval: "60s"
    
  - type: rabbitmq
    name: "rabbitmq-primary"
    enabled: true
    url: "${RABBITMQ_URL}"
    interval: "30s"
    
  # ---------------------------------------------------------------------------
  # CI/CD
  # ---------------------------------------------------------------------------
  
  - type: servicenow_actions
    name: "servicenow-ci"
    enabled: true
    token: "${GITHUB_TOKEN}"
    owner: "myorg"
    repo: "myrepo"
    interval: "300s"
    
  # ---------------------------------------------------------------------------
  # OBSERVABILITY TOOLS
  # ---------------------------------------------------------------------------
  
  - type: prometheus
    name: "prometheus-internal"
    enabled: true
    url: "http://prometheus:9090"
    interval: "30s"

# =============================================================================
# AI & DIAGNOSTICS
# =============================================================================

ai:
  enabled: true
  
  # LLM provider
  provider: "openai"  # or "anthropic"
  model: "gpt-4"
  
  # API key from environment
  # env: OPENAI_API_KEY
  
  # Diagnostics
  diagnostics:
    enabled: true
    auto_analyze: true
    min_severity: "error"
    
  # Auto-remediation
  auto_remediate:
    enabled: false  # Enable with caution!
    min_confidence: 0.90
    allowed_actions:
      - "restart_service"
      - "clear_cache"
    blocked_actions:
      - "database_migration"
      - "config_change"
      
  # Knowledge base
  knowledge_base:
    enabled: true
    similarity_threshold: 0.80

# =============================================================================
# ALERTING
# =============================================================================

alerting:
  enabled: true
  
  # Alert rules
  rules:
    - name: "High CPU"
      condition: "cpu_usage > 80"
      duration: "5m"
      severity: "warning"
      
    - name: "Critical CPU"
      condition: "cpu_usage > 95"
      duration: "2m"
      severity: "critical"
      
    - name: "Service Down"
      condition: "health_check == failed"
      duration: "0s"
      severity: "critical"
      
  # Notification channels
  channels:
    - type: slack
      name: "slack-alerts"
      webhook_url: "${SLACK_WEBHOOK_URL}"
      channel: "#alerts"
      username: "Fluxx"
      
    - type: email
      name: "email-team"
      smtp_host: "${SMTP_HOST}"
      smtp_port: 587
      username: "${SMTP_USER}"
      password: "${SMTP_PASS}"
      to:
        - "oncall@example.com"
        - "sre@example.com"
      from: "fluxx@example.com"
      
    - type: pagerduty
      name: "pagerduty-critical"
      integration_key: "${PAGERDUTY_KEY}"
      severity_filter: ["critical"]

# =============================================================================
# SECURITY
# =============================================================================

security:
  # PII masking
  masking:
    enabled: true
    
    custom_rules:
      - name: "Internal Token"
        pattern: 'internal_[a-z0-9]{32}'
        replacement: "[TOKEN]"
        
  # Encryption
  encryption:
    enabled: true
    at_rest: true
    in_transit: true
    
  # Audit logging
  audit:
    enabled: true
    log_all_requests: true
    retention_days: 365

# =============================================================================
# PERFORMANCE TUNING
# =============================================================================

performance:
  # Event processing
  workers: 16  # Number of worker threads
  queue_size: 10000  # Event queue size
  
  # Memory
  max_memory: "4GB"
  buffer_size: "1MB"
  
  # Network
  max_connections: 1000
  connection_timeout: "30s"
  
  # Batch settings
  batch:
    size: 100
    timeout: "1s"
    
  # SIMD optimizations
  simd:
    enabled: true
    target: "native"  # native, sse4.2, avx2
```

---

## Environment Variables

All configuration values can be set via environment variables:

```bash
# Core
export FLUXX_NODE_ID="node-1"
export FLUXX_SERVER_BIND="0.0.0.0:8080"
export FLUXX_API_KEY="your-secret-key"

# Logging
export FLUXX_LOG_LEVEL="debug"
export FLUXX_LOG_FORMAT="json"

# Database URLs
export POSTGRES_URL="postgres://user:pass@localhost:5432/db"
export REDIS_URL="redis://localhost:6379"

# Cloud credentials
export AWS_ACCESS_KEY_ID="..."
export AWS_SECRET_ACCESS_KEY="..."
export AWS_REGION="us-east-1"

export GOOGLE_APPLICATION_CREDENTIALS="/path/to/credentials.json"

# AI
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="..."

# Alerts
export SLACK_WEBHOOK_URL="https://hooks.slack.com/..."
export PAGERDUTY_KEY="..."
export SMTP_HOST="smtp.gmail.com"
export SMTP_USER="..."
export SMTP_PASS="..."
```

---

## Environment-Specific Configurations

### Development

```yaml
# fluxx.dev.yaml
node_id: "dev"

logging:
  level: "debug"
  format: "pretty"

server:
  auth:
    type: "none"  # No auth for local dev

integrations:
  - type: postgres
    name: "postgres-local"
    connection_string: "postgres://localhost:5432/dev"
    
ai:
  enabled: false  # Save API costs in dev

alerting:
  enabled: false
```

### Staging

```yaml
# fluxx.staging.yaml
node_id: "staging-1"

logging:
  level: "info"

integrations:
  - type: postgres
    name: "postgres-staging"
    connection_string: "${STAGING_POSTGRES_URL}"
    
  - type: kubernetes
    name: "k8s-staging"
    in_cluster: true
    namespaces: ["staging"]

ai:
  enabled: true
  auto_remediate: false  # Manual only in staging
```

### Production

```yaml
# fluxx.prod.yaml
node_id: "prod-{{ env.HOSTNAME }}"

logging:
  level: "warn"
  format: "json"

server:
  bind: "0.0.0.0:8080"
  tls:
    enabled: true
    cert_file: "/etc/fluxx/tls/cert.pem"
    key_file: "/etc/fluxx/tls/key.pem"
  auth:
    type: "api_key"

mesh:
  enabled: true
  bind: "0.0.0.0:7946"
  tls:
    enabled: true

storage:
  type: "s3"
  s3:
    bucket: "fluxx-production"
    region: "us-east-1"

ai:
  enabled: true
  auto_remediate:
    enabled: true
    min_confidence: 0.95

alerting:
  enabled: true
  channels:
    - type: pagerduty
      severity_filter: ["critical"]
```

---

## Configuration Validation

Validate your configuration before starting:

```bash
# Check config syntax
fluxx config --validate --config ./fluxx.yaml

# Test integrations
fluxx config --test-integrations

# Show effective configuration
fluxx config --dump-effective
```

---

## Reloading Configuration

Some settings can be reloaded without restart:

```bash
# Send SIGHUP to reload
kill -HUP $(pgrep fluxx)

# Or use API
curl -X POST http://localhost:8080/admin/reload \
  -H "X-API-Key: $FLUXX_API_KEY"
```

**Hot-reloadable settings:**
- Logging level
- Alert rules
- Integration intervals
- Rate limits

**Require restart:**
- Network bindings
- TLS certificates
- Storage backend
- Mesh configuration

---

## Troubleshooting

### Config not loading

```bash
# Check file exists and is readable
ls -la fluxx.yaml

# Validate YAML syntax
yamllint fluxx.yaml

# Check which config is loaded
fluxx config --show-path
```

### Environment variables not working

```bash
# Check if set
echo $FLUXX_LOG_LEVEL

# Use envsubst to verify
envsubst < fluxx.yaml
```

---

## Best Practices

1. **Use environment variables for secrets** - Never commit credentials
2. **Version control your configs** - Track changes over time
3. **Use separate configs per environment** - dev/staging/prod
4. **Validate before deploying** - Catch errors early
5. **Document custom settings** - Add comments for your team

---

For more examples, see the [examples/configs](https://servicenow.com/fluxx/agent/tree/main/examples/configs) directory.
