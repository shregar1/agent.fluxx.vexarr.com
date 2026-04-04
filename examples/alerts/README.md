# Fluxx Agent Alert System Examples

This directory contains examples demonstrating the Fluxx Agent alert system with multiple notification channels.

## Quick Start

### 1. Slack Integration

```bash
# Set your Slack webhook URL
export SLACK_WEBHOOK_URL="https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX"

# Run the example
cargo run --example alert_example
```

**Setting up Slack webhook:**
1. Go to https://api.slack.com/apps
2. Create New App → From scratch
3. Enable "Incoming Webhooks"
4. Add New Webhook to Workspace
5. Copy the webhook URL

### 2. Email Integration

```bash
# Set email credentials
export SMTP_HOST="smtp.gmail.com"
export SMTP_USER="your-email@gmail.com"
export SMTP_PASS="your-app-password"
export ALERT_EMAIL_TO="ops@company.com,oncall@company.com"
export ALERT_EMAIL_FROM="alerts@company.com"

# Run
cargo run --example alert_example
```

### 3. PagerDuty Integration

```bash
export PAGERDUTY_KEY="your-integration-key"
cargo run --example alert_example
```

**Setting up PagerDuty:**
1. In PagerDuty, go to Services → Service Directory
2. Select or create a service
3. Add Integration → Events API v2
4. Copy the Integration Key

### 4. Discord Integration

```bash
export DISCORD_WEBHOOK_URL="https://discord.com/api/webhooks/..."
cargo run --example alert_example
```

**Setting up Discord webhook:**
1. In Discord, go to Server Settings → Integrations
2. Create Webhook
3. Copy the webhook URL

### 5. Microsoft Teams Integration

```bash
export TEAMS_WEBHOOK_URL="https://outlook.office.com/webhook/..."
cargo run --example alert_example
```

**Setting up Teams webhook:**
1. In Teams, go to channel → ... → Connectors
2. Configure "Incoming Webhook"
3. Copy the webhook URL

## Configuration File

Create `alerts.yaml`:

```yaml
alerts:
  enabled: true
  cooldown_seconds: 300
  max_alerts_per_hour: 100

  rules:
    - name: high_error_rate
      description: "Error rate > 5%"
      condition: "error_rate > 0.05"
      severity: critical
      channels: [slack, pagerduty]

    - name: high_latency
      description: "P99 latency > 500ms"
      condition: "latency_p99 > 500"
      severity: warning
      channels: [slack]

  channels:
    slack:
      type: slack
      webhook_url: "${SLACK_WEBHOOK_URL}"
      channel: "#alerts"
      username: "Fluxx Agent"

    pagerduty:
      type: pagerduty
      integration_key: "${PAGERDUTY_KEY}"
```

Run with:
```bash
export ALERT_CONFIG=./alerts.yaml
cargo run --example alert_example
```

## Condition Syntax

Alert conditions support:

```yaml
# Simple comparisons
condition: "error_rate > 0.05"
condition: "cpu_percent >= 90"
condition: "memory_used < 1GB"

# Logical operators
condition: "error_rate > 0.05 AND cpu_percent > 80"
condition: "status_code == 500 OR status_code == 503"

# Units
condition: "disk_free < 10GB"      # GB, MB, KB
condition: "latency_p99 > 500ms"   # ms, s
```

## Pre-built Templates

```rust
use fluxx_agent::alerts::rules::templates;

// High error rate
let rule = templates::high_error_rate(
    0.05,                              // 5% threshold
    vec!["slack".to_string()]          // channels
);

// High latency
let rule = templates::high_latency(
    500.0,                             // 500ms threshold
    vec!["slack".to_string(), "pagerduty".to_string()]
);

// Low disk space
let rule = templates::low_disk_space(
    10.0,                              // 10% free
    vec!["email".to_string()]
);
```

## Rate Limiting & Deduplication

The alert system includes built-in protection:

- **Rate limiting**: Max alerts per hour (default: 100)
- **Cooldown**: Time between identical alerts (default: 5 minutes)
- **Deduplication**: Same alert won't fire repeatedly during cooldown

Configure in YAML:
```yaml
alerts:
  cooldown_seconds: 300        # 5 minutes
  max_alerts_per_hour: 100
```

Or per-rule:
```yaml
rules:
  - name: frequent_metric
    cooldown_seconds: 60       # 1 minute for this rule only
```

## Testing Webhooks

For testing, you can use httpbin.org:

```bash
export WEBHOOK_URL="https://httpbin.org/post"
cargo run --example alert_example
```

This will show the exact JSON payload being sent.

## Alert Lifecycle

```
Triggered → Firing → Acknowledged → Resolved
    ↓
Suppressed (if deduplicated)
```

Track state:
```rust
// Get all firing alerts
let firing = manager.get_alerts(Some(AlertState::Firing)).await;

// Acknowledge
manager.acknowledge(&alert_id, "admin@company.com").await;

// Resolve
manager.resolve(&alert_id).await;
```

## Multiple Channels

Send to multiple channels simultaneously:

```rust
manager.trigger(
    AlertSeverity::Critical,
    "Database down",
    "db-primary",
    vec![
        "slack".to_string(),
        "pagerduty".to_string(),
        "email".to_string(),
    ],
).await;
```

## Environment Variables

All configuration supports environment variable substitution:

```yaml
channels:
  slack:
    webhook_url: "${SLACK_WEBHOOK_URL}"
    channel: "${SLACK_CHANNEL:-#alerts}"  # With default
```

## Troubleshooting

### Alerts not sending

1. Check channel health:
```rust
let health = manager.health_check().await;
```

2. Enable debug logging:
```bash
RUST_LOG=debug cargo run --example alert_example
```

3. Verify environment variables are set

### Rate limiting

If you see "Alert dropped due to rate limiting":
- Increase `max_alerts_per_hour`
- Increase `cooldown_seconds`
- Check for alert storms (many similar alerts)

### Webhook failures

Webhooks automatically retry (default: 3 attempts) with exponential backoff.

## Security

- Never commit webhook URLs to git
- Use environment variables for credentials
- Rotate webhook URLs periodically
- Use TLS for all channels (enforced)

## Next Steps

- See [alerts.yaml](alerts.yaml) for full configuration example
- Read the [alert system documentation](../../docs/ALERTS.md)
- Check [API documentation](../../docs/API.md) for HTTP endpoints
