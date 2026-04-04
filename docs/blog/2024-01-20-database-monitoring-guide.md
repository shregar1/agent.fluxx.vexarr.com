# Database Monitoring Guide: 11 Databases, One Interface

**Date:** January 20, 2024  
**Author:** Fluxx Agent Team  
**Category:** Database Monitoring

Managing multiple databases in production is challenging. Each has different monitoring needs, query syntaxes, and metric formats. Today, we'll show you how Fluxx Agent's Integrations Framework simplifies database monitoring with support for 11 popular databases through a single, unified interface.

## Why Unified Database Monitoring Matters

Consider a typical microservices architecture:
- **PostgreSQL** for transactional data
- **Redis** for caching
- **MongoDB** for document storage
- **Elasticsearch** for search
- **ClickHouse** for analytics
- **Kafka** for event streaming

Without unified monitoring, you'd need:
- 6 different monitoring tools
- 6 different dashboards
- 6 different alerting systems
- 6 different credential management approaches

With Fluxx Agent, you get one tool for all of them.

## Supported Databases

### Relational Databases

**PostgreSQL**
```rust
manager.with_postgres("postgres://user:pass@localhost/mydb");
```
Key metrics: active connections, transactions/second, cache hit ratio, slow queries

**MySQL**
```rust
manager.with_mysql("mysql://user:pass@localhost/mydb");
```
Key metrics: threads connected, slow queries, replication lag

**CockroachDB**
```rust
manager.with_cockroachdb("localhost", 26257);
```
Key metrics: node status, range distribution, replication health

**TimescaleDB**
```rust
manager.with_timescaledb("localhost", 5432, "mydb");
```
Key metrics: hypertable size, chunk compression, continuous aggregates

### NoSQL Databases

**MongoDB**
```rust
manager.with_mongodb("mongodb://localhost:27017/mydb");
```
Key metrics: opcounters, connections, memory usage, replication lag

**Redis**
```rust
manager.with_redis("localhost", 6379);
```
Key metrics: memory usage, hit ratio, commands/sec, connected clients

**Cassandra**
```rust
manager.with_cassandra(&["node1", "node2", "node3"], 9042);
```
Key metrics: node status, compaction pending, read/write latency

**Neo4j**
```rust
manager.with_neo4j("localhost", 7474, "neo4j", "password");
```
Key metrics: node/relationship counts, transaction rates, page cache

### Search & Analytics

**Elasticsearch**
```rust
manager.with_elasticsearch("http://localhost:9200")
    .with_auth("user", "password");
```
Key metrics: cluster health, index size, search latency, shard distribution

**ClickHouse**
```rust
manager.with_clickhouse("localhost", 8123, "default");
```
Key metrics: query performance, merge operations, parts count

**InfluxDB**
```rust
manager.with_influxdb("http://localhost:8086", "myorg", "token");
```
Key metrics: bucket sizes, write rates, query duration

## Common Metrics Across All Databases

Every database integration provides:

| Metric | Description |
|--------|-------------|
| `connection.status` | Connectivity health (1=healthy, 0=unhealthy) |
| Active connections | Current client connections |
| Query rate | Queries/second |
| Error rate | Failed queries/second |
| Resource usage | CPU, memory, disk |

## Automatic Event Detection

All database integrations automatically detect:

**Connection Issues**
```
Event: Database Connection Failed
Severity: CRITICAL
Description: Cannot connect to PostgreSQL at localhost:5432
```

**Performance Problems**
```
Event: High Connection Count
Severity: WARNING
Description: 150 active connections (limit: 100)
```

**Resource Constraints**
```
Event: Redis Memory High
Severity: WARNING
Description: Memory usage at 85% of max limit
```

## Best Practices

### 1. Use Connection Pooling

For production databases, configure connection pooling to avoid overwhelming your database:

```rust
let config = IntegrationConfig {
    interval_secs: 60,  // Collect every minute
    timeout_secs: 30,   // 30-second timeout
    ..Default::default()
};
```

### 2. Monitor Replicas

Always monitor both primary and replica databases:

```rust
manager.with_postgres("postgres://primary.internal/db");
manager.with_postgres("postgres://replica.internal/db");
```

### 3. Alert on Lag

For replicated databases, monitor replication lag:

```rust
// Automatic detection of high replication lag
events.push(
    Event::new(
        "High Replication Lag",
        "MySQL replica lag is 45 seconds",
        EventSeverity::Warning,
        "mysql",
        "replication_lag",
    )
);
```

### 4. Track Slow Queries

Enable slow query monitoring:

```rust
// PostgreSQL
.with_postgres("postgres://localhost/db?slow_query_threshold=100ms")

// MySQL
.with_mysql("mysql://localhost/db?slow_query_log=true")
```

## Real-World Example

Here's a complete database monitoring setup for a microservices architecture:

```rust
use fluxx_agent::integrations::IntegrationManager;
use std::env;

let mut manager = IntegrationManager::new();

// Primary databases
manager
    .with_postgres(&env::var("PRIMARY_DB_URL")?)
    .with_redis(&env::var("REDIS_HOST")?, 6379)
    .with_mongodb(&env::var("MONGO_URL")?);

// Analytics databases
manager
    .with_clickhouse("analytics.internal", 8123, "events")
    .with_elasticsearch("http://search.internal:9200")
    .with_influxdb("http://metrics.internal:8086", "prod", &env::var("INFLUX_TOKEN")?);

// Event streaming
manager.with_kafka("kafka.internal:9092");

// Cache layer
manager.with_memcached("cache.internal", 11211);

// Run monitoring
let metrics = manager.collect_metrics().await;
let events = manager.collect_events().await;

// Check health
let health = manager.health_check().await;
for (name, status) in health {
    println!("{}: {:?}", name, status);
}
```

## Performance Impact

Our database integrations are designed for minimal overhead:

- **PostgreSQL**: Uses `pg_stat_*` views (no additional load)
- **Redis**: Uses `INFO` command (negligible impact)
- **MongoDB**: Uses `db.serverStatus()` (low impact)
- **Elasticsearch**: Uses `_cluster/health` and `_stats` APIs

Typical monitoring overhead: < 1% CPU on database servers

## Troubleshooting

### Connection Refused

```
Error: Cannot connect to PostgreSQL at localhost:5432
Solution: 
1. Check PostgreSQL is running: sudo systemctl status postgresql
2. Verify connection string
3. Check firewall rules
```

### Authentication Failed

```
Error: Authentication failed for user 'monitoring'
Solution:
1. Verify username/password
2. Check pg_hba.conf for access rules
3. Ensure user has required permissions
```

### High Query Latency

```
Error: Query timeout after 30 seconds
Solution:
1. Increase timeout: timeout_secs: 60
2. Check database performance
3. Monitor during off-peak hours
```

## What's Next?

We're working on:
- Query-level monitoring for all databases
- Automatic index recommendations
- Query plan analysis
- Cross-database correlation

## Get Started

```bash
cargo add fluxx-agent
```

See the [full documentation](../integrations/README.md) for detailed setup instructions for each database.

---

*Monitoring 11 databases has never been easier. Try Fluxx Agent today and unify your database monitoring.*
