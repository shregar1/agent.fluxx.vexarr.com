# Fluxx Agent Integrations Framework

## Overview

The Fluxx Agent Integrations Framework provides a unified, pluggable architecture for monitoring 45+ infrastructure components, cloud services, databases, message queues, CI/CD systems, caches, observability tools, and security systems.

## Architecture

### Core Design Principles

1. **Unified Interface**: All integrations implement the `Integration` trait
2. **Async-First**: Built on Tokio for high-performance concurrent monitoring
3. **Health-Aware**: Every integration includes connectivity validation
4. **Event-Driven**: Automatic anomaly detection and alerting
5. **Extensible**: Easy to add new integrations following established patterns

### Integration Trait

```rust
#[async_trait]
pub trait Integration: Send + Sync {
    fn name(&self) -> &str;
    async fn collect_metrics(&self) -> Result<Vec<Metric>, IntegrationError>;
    async fn collect_events(&self) -> Result<Vec<Event>, IntegrationError>;
    async fn health_check(&self) -> Result<(), IntegrationError>;
    fn config(&self) -> &IntegrationConfig;
}
```

## Supported Integrations (45+)

### Databases (11)

| Integration | Description | Key Metrics |
|-------------|-------------|-------------|
| **PostgreSQL** | Query performance, connections, locks | `postgresql.active_connections`, `postgresql.transactions.commited` |
| **MySQL** | Slow queries, replication lag | `mysql.threads_connected`, `mysql.slow_queries` |
| **Redis** | Memory usage, command stats | `redis.used_memory`, `redis.keyspace_hits` |
| **MongoDB** | Operations, replication | `mongodb.opcounters.query`, `mongodb.connections.current` |
| **Cassandra** | Cluster status, compaction | `cassandra.nodes.up`, `cassandra.compaction.pending_tasks` |
| **Elasticsearch** | Cluster health, indices | `elasticsearch.cluster.status`, `elasticsearch.indices.docs.count` |
| **ClickHouse** | Query performance, tables | `clickhouse.queries.running`, `clickhouse.tables.total_rows` |
| **Neo4j** | Graph metrics, transactions | `neo4j.nodes.total`, `neo4j.relationships.total` |
| **InfluxDB** | Buckets, writes | `influxdb.buckets.total`, `influxdb.writes.total` |
| **TimescaleDB** | Hypertables, chunks | `timescaledb.hypertables.total`, `timescaledb.chunks.total` |
| **CockroachDB** | Distributed SQL metrics | `cockroachdb.nodes.live`, `cockroachdb.ranges.total` |

### Cloud Providers (8)

| Provider | Services Monitored |
|----------|-------------------|
| **AWS** | EC2, RDS, ELB, Lambda, S3, CloudWatch |
| **Azure** | VMs, AKS, SQL, Storage, Functions |
| **GCP** | Compute, GKE, Cloud SQL, Storage, Functions |
| **DigitalOcean** | Droplets, Kubernetes, Databases, Load Balancers |
| **Linode** | Instances, LKE (Kubernetes) |
| **IBM Cloud** | Resource instances, services |
| **Oracle Cloud** | Compute, databases, networking |
| **Alibaba Cloud** | ECS, RDS, OSS |

### Message Queues (6)

| Integration | Key Metrics |
|-------------|-------------|
| **Kafka** | Consumer lag, topics, partitions |
| **RabbitMQ** | Queue depth, connections, nodes |
| **AWS SQS** | Queue depth, message age, visibility |
| **NATS** | Connections, subscriptions, messages |
| **Apache Pulsar** | Tenants, namespaces, topics |
| **ActiveMQ** | Queue depth, consumers, memory |

### Containers & Orchestration (2)

| Integration | Metrics |
|-------------|---------|
| **Docker** | Container stats, images, volumes, system |
| **Kubernetes** | Pods, nodes, deployments, services |

### CI/CD (7)

| Integration | Metrics |
|-------------|---------|
| **ServiceNow Actions** | Workflow runs, deployment tracking |
| **GitLab CI** | Pipelines, jobs, runners |
| **Jenkins** | Jobs, executors, queue, nodes |
| **CircleCI** | Pipelines, workflows |
| **Travis CI** | Build status, repository metrics |
| **TeamCity** | Build configs, agents, queue |
| **Bamboo** | Plans, deployments, agents |

### Cache (2)

| Integration | Key Metrics |
|-------------|-------------|
| **Memcached** | Connections, hit ratio, evictions |
| **Varnish** | Hit rates, backends, threads, ESI |

### Observability (6)

| Integration | Purpose |
|-------------|---------|
| **Prometheus** | Metrics collection, target health |
| **Grafana** | Dashboards, datasources |
| **Jaeger** | Distributed tracing |
| **Zipkin** | Distributed tracing |
| **Datadog** | Monitor health, alert status |
| **New Relic** | APM, infrastructure monitoring |

### Security (2)

| Integration | Purpose |
|-------------|---------|
| **HashiCorp Vault** | Seal status, health, mounts |
| **LDAP/AD** | Connection health, authentication |

### Network

- TCP/UDP flow monitoring
- DNS resolution tracking
- Interface statistics
- Socket statistics

## Quick Start

### Basic Usage

```rust
use fluxx_agent::integrations::IntegrationManager;

let mut manager = IntegrationManager::new();

// Add integrations
manager
    .with_postgres("postgres://user:pass@localhost/mydb")
    .with_redis("localhost", 6379)
    .with_docker()
    .with_kubernetes()
    .with_aws("us-east-1")
    .with_prometheus("http://localhost:9090");

// Collect metrics
let metrics = manager.collect_metrics().await;
println!("Collected {} metrics", metrics.len());

// Collect events
let events = manager.collect_events().await;
for event in events {
    println!("Event: {} - {}", event.title, event.severity);
}

// Health check
let health = manager.health_check().await;
for (name, status) in health {
    println!("{}: {}", name, status);
}
```

### Configuration-Based Setup

```yaml
# integrations.yaml
integrations:
  databases:
    postgres:
      connection_string: "postgres://user:pass@localhost/mydb"
      interval_secs: 60
    redis:
      host: "localhost"
      port: 6379
  
  cloud:
    aws:
      region: "us-east-1"
      profile: "production"
  
  containers:
    docker: {}
    kubernetes:
      namespace: "default"
  
  observability:
    prometheus:
      url: "http://localhost:9090"
    grafana:
      url: "http://localhost:3000"
      api_token: "${GRAFANA_TOKEN}"
```

## Integration Categories

### Database Integrations

All database integrations provide:
- Connection health monitoring
- Query performance metrics
- Resource utilization (CPU, memory, disk)
- Slow query detection
- Connection pool tracking

Example with PostgreSQL:

```rust
manager.with_postgres("postgres://localhost/mydb");
```

Metrics collected:
- `postgresql.active_connections`
- `postgresql.idle_connections`
- `postgresql.transactions.commited`
- `postgresql.transactions.rolled_back`
- `postgresql.blocks.read`
- `postgresql.blocks.hit`

### Cloud Provider Integrations

Cloud integrations use official CLI tools or APIs:
- AWS: Uses AWS CLI
- Azure: Uses Azure CLI
- GCP: Uses gcloud CLI
- DigitalOcean: Uses REST API
- Others: REST APIs or CLIs

Example with multiple clouds:

```rust
manager
    .with_aws("us-east-1")
    .with_azure("subscription-id")
    .with_gcp("project-id")
    .with_digitalocean("api-token");
```

### Container Orchestration

Docker integration uses:
- `/var/run/docker.sock` for container stats
- `docker system df` for resource usage
- cgroup v1/v2 for metrics

Kubernetes integration uses:
- `kubectl` command
- Parse JSON output for resources
- Node, pod, deployment metrics

### CI/CD Integrations

All CI/CD integrations track:
- Build/pipeline status
- Duration metrics
- Success/failure rates
- Queue depth
- Agent/runner status

Example with ServiceNow Actions:

```rust
manager.with_servicenow_actions("token", "owner", "repo");
```

### Message Queue Integrations

Message queue integrations monitor:
- Queue depth
- Consumer lag
- Connection counts
- Message throughput
- Error rates

Example with Kafka:

```rust
manager.with_kafka("localhost:9092");
```

Key metrics:
- `kafka.consumer_lag.total`
- `kafka.topics.total`
- `kafka.messages_in_per_sec`

## Event System

Integrations automatically generate events for:
- Connection failures
- High resource usage
- Error conditions
- Performance degradation

Event severity levels:
- `Info`: Informational events
- `Warning`: Potential issues
- `Error`: Errors requiring attention
- `Critical`: Immediate action required

Example event:

```rust
Event::new(
    "High Consumer Lag",
    "Consumer group X on topic Y has lag of 10000",
    EventSeverity::Warning,
    "kafka",
    "high_consumer_lag",
)
.with_metadata("group", "my-group")
.with_metadata("topic", "my-topic")
.with_metadata("lag", "10000");
```

## Health Checking

Each integration provides a `health_check()` method:

```rust
match integration.health_check().await {
    Ok(()) => println!("Integration is healthy"),
    Err(e) => println!("Integration unhealthy: {}", e),
}
```

The IntegrationManager can check all integrations:

```rust
let health = manager.health_check().await;
for (name, status) in health {
    match status {
        IntegrationStatus::Healthy => println!("✓ {}", name),
        IntegrationStatus::Unhealthy(reason) => println!("✗ {}: {}", name, reason),
        IntegrationStatus::Unknown => println!("? {}", name),
    }
}
```

## Best Practices

### 1. Timeout Configuration

Set appropriate timeouts for each integration:

```rust
let config = IntegrationConfig {
    interval_secs: 60,
    timeout_secs: 30,
    ..Default::default()
};
```

### 2. Error Handling

Integrations should fail gracefully:

```rust
match integration.collect_metrics().await {
    Ok(metrics) => send_to_storage(metrics),
    Err(e) => {
        tracing::warn!("Failed to collect metrics: {}", e);
        // Continue with other integrations
    }
}
```

### 3. Resource Management

- Use connection pooling where applicable
- Close connections after collection
- Respect rate limits of APIs

### 4. Security

- Store credentials securely (environment variables, secrets management)
- Use read-only permissions when possible
- Rotate API tokens regularly

## Adding New Integrations

### Step 1: Create Module

```bash
touch src/integrations/category/myintegration.rs
```

### Step 2: Implement Integration Trait

```rust
use super::super::framework::*;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct MyIntegration {
    config: IntegrationConfig,
    // ... fields
}

impl MyIntegration {
    pub fn new(...) -> Result<Self, IntegrationError> {
        // ... initialization
    }
}

#[async_trait]
impl Integration for MyIntegration {
    fn name(&self) -> &str { &self.config.name }
    
    async fn collect_metrics(&self) -> Result<Vec<Metric>, IntegrationError> {
        // ... collect metrics
    }
    
    async fn collect_events(&self) -> Result<Vec<Event>, IntegrationError> {
        // ... collect events
    }
    
    async fn health_check(&self) -> Result<(), IntegrationError> {
        // ... check health
    }
    
    fn config(&self) -> &IntegrationConfig { &self.config }
}
```

### Step 3: Export in Category Module

```rust
// src/integrations/category/mod.rs
pub mod myintegration;
pub use myintegration::MyIntegration;
```

### Step 4: Add to IntegrationManager

```rust
pub fn with_my_integration(&mut self, ...) -> &mut Self {
    match category::myintegration::MyIntegration::new(...) {
        Ok(integration) => self.register("my_integration", integration),
        Err(e) => error!("Failed to create integration: {}", e),
    }
    self
}
```

## Troubleshooting

### Common Issues

1. **Connection Refused**
   - Verify service is running
   - Check firewall rules
   - Verify correct port

2. **Authentication Errors**
   - Check API tokens/credentials
   - Verify permissions
   - Check token expiration

3. **Timeout Errors**
   - Increase timeout value
   - Check network latency
   - Verify service responsiveness

### Debug Mode

Enable debug logging:

```bash
RUST_LOG=fluxx_agent::integrations=debug cargo run
```

## Performance Considerations

- Each integration runs independently
- Use concurrent collection for multiple integrations
- Cache results when appropriate
- Respect API rate limits
- Use connection pooling

## Contributing

When adding new integrations:

1. Follow existing patterns
2. Include comprehensive tests
3. Document all metrics collected
4. Add examples to documentation
5. Update this README

## License

See LICENSE file in project root.
