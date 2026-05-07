# Fluxx Agent Documentation

Welcome to the Fluxx Agent documentation! Fluxx Agent is a high-performance, cost-effective observability platform capable of processing 218M events/second at 95% lower cost than competitors.

## Getting Started

- [Quick Start Guide](quickstart.md) - Get up and running in 5 minutes
- [Installation](installation.md) - Detailed installation instructions
- [Configuration](configuration.md) - Configure Fluxx Agent for your environment

## Core Features

### Integrations Framework (45+ Integrations)

Monitor all your infrastructure with a unified interface:

- [Integrations Overview](integrations/README.md) - Complete guide to all integrations
- [Metrics Reference](integrations/METRICS.md) - All metrics collected by each integration
- [Events Reference](integrations/EVENTS.md) - Events and alerting reference

**Supported Categories:**
- **Databases (11)**: PostgreSQL, MySQL, Redis, MongoDB, Cassandra, Elasticsearch, ClickHouse, Neo4j, InfluxDB, TimescaleDB, CockroachDB
- **Cloud Providers (8)**: AWS, Azure, GCP, DigitalOcean, Linode, IBM Cloud, Oracle Cloud, Alibaba Cloud
- **Message Queues (6)**: Kafka, RabbitMQ, SQS, NATS, Pulsar, ActiveMQ
- **CI/CD (7)**: ServiceNow Actions, GitLab CI, Jenkins, CircleCI, Travis CI, TeamCity, Bamboo
- **Containers**: Docker, Kubernetes
- **Cache**: Memcached, Varnish
- **Observability**: Prometheus, Grafana, Jaeger, Zipkin, Datadog, New Relic
- **Security**: HashiCorp Vault, LDAP

### AI-Powered Diagnostics

- [Diagnostic System](DIAGNOSTIC.md) - AI-powered root cause analysis
- [Auto-Remediation](auto-remediation.md) - Automatic issue resolution
- [Ticketing Integration](ticketing.md) - Jira, ServiceNow, Linear integration

### Distributed Mesh

- [Mesh Deployment](MESH.md) - Zero-code distributed clusters
- [Query Federation](mesh/query-federation.md) - Cross-cluster querying
- [Deployment Patterns](mesh/deployment.md) - Docker, Kubernetes, bare metal

### Alert System

- [Alert Configuration](alerts/README.md) - Configure alert rules
- [Notification Channels](alerts/channels.md) - Slack, Email, PagerDuty, etc.
- [Rate Limiting](alerts/rate-limiting.md) - Prevent alert fatigue

## Architecture

- [Overview](architecture/overview.md) - High-level architecture
- [Performance](architecture/performance.md) - 218M events/sec design
- [Security](architecture/security.md) - Security architecture

## API Reference

- [REST API](api/rest.md) - HTTP API documentation
- [WebSocket API](api/websocket.md) - Real-time event streaming
- [GraphQL API](api/graphql.md) - Query interface

## Blog

Read our latest articles:

- [Introducing the Integrations Framework](blog/2024-01-15-introducing-integrations-framework.md)
- [Database Monitoring Guide](blog/2024-01-20-database-monitoring-guide.md)
- [Unified Cloud Monitoring](blog/2024-01-25-cloud-monitoring-unified.md)
- [CI/CD Monitoring Guide](blog/2024-02-01-cicd-monitoring-guide.md)

[View all blog posts →](blog/README.md)

## Examples

### Basic Monitoring Setup

```rust
use fluxx_agent::integrations::IntegrationManager;

let mut manager = IntegrationManager::new();

// Add your infrastructure
manager
    .with_postgres("postgres://localhost/mydb")
    .with_redis("localhost", 6379)
    .with_docker()
    .with_kubernetes()
    .with_aws("us-east-1");

// Collect metrics
let metrics = manager.collect_metrics().await;
println!("Collected {} metrics", metrics.len());

// Check for events
let events = manager.collect_events().await;
for event in events {
    println!("{}: {}", event.severity, event.title);
}
```

### AI Diagnostics

```rust
use fluxx_agent::diagnostic::DiagnosticEngine;

let engine = DiagnosticEngine::new(config);

// Analyze an error
let report = engine.diagnose(error).await?;
println!("Root cause: {}", report.root_cause);
println!("Confidence: {:.0}%", report.confidence * 100.0);
println!("Suggested fixes: {:?}", report.suggested_fixes);
```

### Distributed Mesh

```rust
use fluxx_agent::mesh::MeshNode;

// Join a mesh cluster
let node = MeshNode::new(config)
    .join_seeds(&["10.0.1.10:7946", "10.0.1.11:7946"])
    .start()
    .await?;

// Query across the mesh
let results = node.query("SELECT * FROM events WHERE severity='critical'")
    .federated()
    .await?;
```

## Performance

| Metric | Value |
|--------|-------|
| Events/sec per server | 218M |
| Latency (p99) | < 1ms |
| Memory overhead | < 100MB |
| Cost vs Datadog | 95% savings |

## Contributing

We welcome contributions! See:

- [Contributing Guide](CONTRIBUTING.md)
- [Development Setup](development.md)
- [Adding Integrations](integrations/DEVELOPMENT.md)

## Support

- [Discord](https://discord.gg/fluxx) - Community chat
- [ServiceNow Issues](https://servicenow.com/fluxx/agent/issues) - Bug reports
- [Email](mailto:support@fluxx.vexarr.com) - Direct support

## License

Fluxx Agent is licensed under the MIT License. See [LICENSE](../LICENSE) for details.

---

**Quick Links:**
[Home](https://fluxx.vexarr.com) |
[ServiceNow](https://servicenow.com/fluxx/agent) |
[Twitter](https://twitter.com/fluxxagent) |
[Docker Hub](https://hub.docker.com/r/fluxx/agent)
