# Metrics Reference

This document provides a comprehensive reference of all metrics collected by the Fluxx Agent Integrations Framework.

## Metric Naming Convention

All metrics follow the pattern:

```
<integration>.<resource>.<metric>
```

Examples:
- `postgresql.connections.active`
- `aws.ec2.instances.running`
- `kubernetes.pods.running`

## Common Metrics

These metrics are available across most integrations:

| Metric | Type | Description |
|--------|------|-------------|
| `*.connection.status` | gauge | 1 if connected, 0 if not |
| `*.health.status` | gauge | 1 if healthy, 0 if not |
| `*.uptime_seconds` | gauge | Service uptime in seconds |

## Database Metrics

### PostgreSQL

| Metric | Type | Description |
|--------|------|-------------|
| `postgresql.connection.status` | gauge | Connection health |
| `postgresql.active_connections` | gauge | Currently active connections |
| `postgresql.idle_connections` | gauge | Idle connections |
| `postgresql.transactions.commited` | counter | Committed transactions |
| `postgresql.transactions.rolled_back` | counter | Rolled back transactions |
| `postgresql.blocks.read` | counter | Disk blocks read |
| `postgresql.blocks.hit` | counter | Cache hits |

### MySQL

| Metric | Type | Description |
|--------|------|-------------|
| `mysql.connection.status` | gauge | Connection health |
| `mysql.threads_connected` | gauge | Connected threads |
| `mysql.threads_running` | gauge | Running threads |
| `mysql.questions` | counter | Queries executed |
| `mysql.slow_queries` | counter | Slow queries |

### Redis

| Metric | Type | Description |
|--------|------|-------------|
| `redis.connection.status` | gauge | Connection health |
| `redis.connected_clients` | gauge | Connected clients |
| `redis.used_memory` | gauge | Memory used in bytes |
| `redis.total_commands_processed` | counter | Commands processed |
| `redis.keyspace_hits` | counter | Cache hits |
| `redis.keyspace_misses` | counter | Cache misses |

### MongoDB

| Metric | Type | Description |
|--------|------|-------------|
| `mongodb.connection.status` | gauge | Connection health |
| `mongodb.connections.current` | gauge | Current connections |
| `mongodb.opcounters.query` | counter | Query operations |
| `mongodb.opcounters.insert` | counter | Insert operations |
| `mongodb.opcounters.update` | counter | Update operations |
| `mongodb.opcounters.delete` | counter | Delete operations |

### Elasticsearch

| Metric | Type | Description |
|--------|------|-------------|
| `elasticsearch.connection.status` | gauge | Connection health |
| `elasticsearch.cluster.status` | gauge | 0=green, 1=yellow, 2=red |
| `elasticsearch.cluster.nodes.total` | gauge | Total nodes |
| `elasticsearch.cluster.nodes.data` | gauge | Data nodes |
| `elasticsearch.cluster.shards.active` | gauge | Active shards |
| `elasticsearch.cluster.shards.unassigned` | gauge | Unassigned shards |
| `elasticsearch.indices.docs.count` | gauge | Total documents |

### Cassandra

| Metric | Type | Description |
|--------|------|-------------|
| `cassandra.connection.status` | gauge | Connection health |
| `cassandra.nodes.total` | gauge | Total nodes |
| `cassandra.nodes.up` | gauge | Nodes up |
| `cassandra.nodes.down` | gauge | Nodes down |
| `cassandra.compaction.pending_tasks` | gauge | Pending compactions |

### ClickHouse

| Metric | Type | Description |
|--------|------|-------------|
| `clickhouse.connection.status` | gauge | Connection health |
| `clickhouse.queries.running` | gauge | Running queries |
| `clickhouse.tables.total` | gauge | Total tables |
| `clickhouse.tables.total_rows` | gauge | Total rows across tables |

## Cloud Provider Metrics

### AWS

| Metric | Type | Description |
|--------|------|-------------|
| `aws.connection.status` | gauge | API access health |
| `aws.ec2.instances.running` | gauge | Running EC2 instances |
| `aws.ec2.instances.stopped` | gauge | Stopped EC2 instances |
| `aws.rds.databases.total` | gauge | RDS instances |
| `aws.lambda.functions.total` | gauge | Lambda functions |
| `aws.s3.buckets.total` | gauge | S3 buckets |

### Azure

| Metric | Type | Description |
|--------|------|-------------|
| `azure.connection.status` | gauge | API access health |
| `azure.vm.total` | gauge | Total VMs |
| `azure.vm.running` | gauge | Running VMs |
| `azure.aks.clusters.total` | gauge | AKS clusters |
| `azure.sql.servers.total` | gauge | SQL servers |

### GCP

| Metric | Type | Description |
|--------|------|-------------|
| `gcp.connection.status` | gauge | API access health |
| `gcp.compute.instances.total` | gauge | Total instances |
| `gcp.compute.instances.running` | gauge | Running instances |
| `gcp.gke.clusters.total` | gauge | GKE clusters |
| `gcp.storage.buckets.total` | gauge | Storage buckets |

### DigitalOcean

| Metric | Type | Description |
|--------|------|-------------|
| `digitalocean.connection.status` | gauge | API access health |
| `digitalocean.droplets.total` | gauge | Total droplets |
| `digitalocean.droplets.running` | gauge | Running droplets |
| `digitalocean.kubernetes.clusters.total` | gauge | K8s clusters |
| `digitalocean.databases.total` | gauge | Managed databases |

## Container Metrics

### Docker

| Metric | Type | Description |
|--------|------|-------------|
| `docker.connection.status` | gauge | Docker daemon health |
| `docker.containers.total` | gauge | Total containers |
| `docker.images.total` | gauge | Total images |
| `docker.volumes.total` | gauge | Total volumes |
| `docker.memory.current_bytes` | gauge | Memory usage |

### Kubernetes

| Metric | Type | Description |
|--------|------|-------------|
| `kubernetes.connection.status` | gauge | API server health |
| `kubernetes.nodes.total` | gauge | Total nodes |
| `kubernetes.nodes.ready` | gauge | Ready nodes |
| `kubernetes.nodes.not_ready` | gauge | Not ready nodes |
| `kubernetes.pods.total` | gauge | Total pods |
| `kubernetes.pods.running` | gauge | Running pods |
| `kubernetes.deployments.total` | gauge | Total deployments |

## Message Queue Metrics

### Kafka

| Metric | Type | Description |
|--------|------|-------------|
| `kafka.connection.status` | gauge | Connection health |
| `kafka.topics.total` | gauge | Total topics |
| `kafka.consumer_lag.total` | gauge | Total consumer lag |
| `kafka.messages_in_per_sec` | gauge | Messages in/sec |

### RabbitMQ

| Metric | Type | Description |
|--------|------|-------------|
| `rabbitmq.connection.status` | gauge | Connection health |
| `rabbitmq.connections.total` | gauge | Total connections |
| `rabbitmq.queues.total` | gauge | Total queues |
| `rabbitmq.messages.total` | gauge | Total messages |
| `rabbitmq.messages.ready` | gauge | Ready messages |
| `rabbitmq.consumers.total` | gauge | Total consumers |

### AWS SQS

| Metric | Type | Description |
|--------|------|-------------|
| `aws.sqs.connection.status` | gauge | API access health |
| `aws.sqs.queues.total` | gauge | Total queues |
| `aws.sqs.messages.total` | gauge | Total messages |
| `aws.sqs.messages.visible` | gauge | Visible messages |
| `aws.sqs.messages.delayed` | gauge | Delayed messages |

## CI/CD Metrics

### ServiceNow Actions

| Metric | Type | Description |
|--------|------|-------------|
| `servicenow_actions.connection.status` | gauge | API access health |
| `servicenow_actions.workflows.total` | gauge | Total workflows |
| `servicenow_actions.runs.total` | gauge | Total runs |
| `servicenow_actions.runs.success` | gauge | Successful runs |
| `servicenow_actions.runs.failure` | gauge | Failed runs |
| `servicenow_actions.success_rate` | gauge | Success percentage |

### GitLab CI

| Metric | Type | Description |
|--------|------|-------------|
| `gitlab.connection.status` | gauge | API access health |
| `gitlab.pipelines.total` | gauge | Total pipelines |
| `gitlab.pipelines.success` | gauge | Successful pipelines |
| `gitlab.pipelines.failed` | gauge | Failed pipelines |
| `gitlab.runners.total` | gauge | Total runners |
| `gitlab.runners.active` | gauge | Active runners |

### Jenkins

| Metric | Type | Description |
|--------|------|-------------|
| `jenkins.connection.status` | gauge | Jenkins health |
| `jenkins.jobs.total` | gauge | Total jobs |
| `jenkins.jobs.success` | gauge | Successful jobs |
| `jenkins.jobs.failed` | gauge | Failed jobs |
| `jenkins.executors.total` | gauge | Total executors |
| `jenkins.executors.busy` | gauge | Busy executors |
| `jenkins.queue.size` | gauge | Queue depth |

## Cache Metrics

### Memcached

| Metric | Type | Description |
|--------|------|-------------|
| `memcached.connection.status` | gauge | Connection health |
| `memcached.connections.current` | gauge | Current connections |
| `memcached.items.current` | gauge | Current items |
| `memcached.memory.bytes_used` | gauge | Memory used |
| `memcached.gets.hit_ratio` | gauge | Cache hit ratio |
| `memcached.evictions` | counter | Items evicted |

### Varnish

| Metric | Type | Description |
|--------|------|-------------|
| `varnish.connection.status` | gauge | Connection health |
| `varnish.cache.hits` | counter | Cache hits |
| `varnish.cache.misses` | counter | Cache misses |
| `varnish.cache.hit_ratio` | gauge | Hit ratio percentage |
| `varnish.threads` | gauge | Thread count |
| `varnish.backend.failures` | counter | Backend failures |

## Observability Metrics

### Prometheus

| Metric | Type | Description |
|--------|------|-------------|
| `prometheus.connection.status` | gauge | Server health |
| `prometheus.targets.total` | gauge | Total targets |
| `prometheus.targets.up` | gauge | Up targets |
| `prometheus.targets.down` | gauge | Down targets |
| `prometheus.timeseries.total` | gauge | Total time series |

### Grafana

| Metric | Type | Description |
|--------|------|-------------|
| `grafana.connection.status` | gauge | Server health |
| `grafana.datasources.total` | gauge | Total datasources |
| `grafana.health.database` | gauge | Database health |

### Datadog

| Metric | Type | Description |
|--------|------|-------------|
| `datadog.connection.status` | gauge | API access health |
| `datadog.monitors.total` | gauge | Total monitors |
| `datadog.monitors.ok` | gauge | OK monitors |
| `datadog.monitors.alert` | gauge | Alerting monitors |
| `datadog.monitors.warn` | gauge | Warning monitors |

## Security Metrics

### HashiCorp Vault

| Metric | Type | Description |
|--------|------|-------------|
| `vault.connection.status` | gauge | Server health |
| `vault.health.status` | gauge | Health status (0-1) |
| `vault.sealed` | gauge | 1 if sealed, 0 if not |
| `vault.initialized` | gauge | 1 if initialized |
| `vault.standby` | gauge | 1 if standby mode |

### LDAP

| Metric | Type | Description |
|--------|------|-------------|
| `ldap.connection.status` | gauge | Server health |
| `ldap.ssl` | gauge | 1 if using SSL |

## Metric Types

### Gauge
Current value that can go up or down.

Examples:
- Current connections
- Memory usage
- Queue depth

### Counter
Cumulative value that only increases.

Examples:
- Total requests
- Cache hits
- Transactions

## Tags

Metrics include tags for filtering and grouping:

```rust
Metric {
    name: "postgresql.connections.active",
    value: Gauge(42.0),
    tags: {
        "host": "db-primary",
        "database": "production",
        "integration": "postgresql"
    },
}
```

Common tags:
- `host` - Hostname or IP
- `region` - Cloud region
- `database` - Database name
- `table` - Table name
- `index` - Index name
- `queue` - Queue name
- `integration` - Integration name

## Querying Metrics

### By Integration

```rust
let postgres_metrics: Vec<_> = metrics
    .iter()
    .filter(|m| m.name.starts_with("postgresql"))
    .collect();
```

### By Tag

```rust
let production_metrics: Vec<_> = metrics
    .iter()
    .filter(|m| m.tags.get("environment") == Some(&"production".to_string()))
    .collect();
```

### Aggregation

```rust
let total_connections: f64 = metrics
    .iter()
    .filter(|m| m.name == "postgresql.connections.active")
    .filter_map(|m| m.value.as_f64())
    .sum();
```

## Exporting Metrics

Metrics can be exported to various backends:

### Prometheus

```rust
for metric in metrics {
    println!("# HELP {} {}", metric.name, metric.description);
    println!("# TYPE {} {}", metric.name, metric.metric_type);
    print!("{}", metric.name);
    for (key, value) in &metric.tags {
        print!("{}=\"{}\"", key, value);
    }
    println!(" {}", metric.value);
}
```

### InfluxDB Line Protocol

```rust
for metric in metrics {
    print!("{}", metric.name);
    for (key, value) in &metric.tags {
        print!(",{}={}", key, value);
    }
    println!(" value={} {}", metric.value, timestamp);
}
```

## See Also

- [Integrations Overview](./README.md)
- [Event Reference](./EVENTS.md)
- [API Documentation](../api/README.md)
