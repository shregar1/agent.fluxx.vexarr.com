# Integration Development Guide

This guide explains how to add new integrations to the Fluxx Agent Integrations Framework.

## Prerequisites

- Rust 1.70+
- Understanding of async Rust (Tokio)
- Familiarity with the target service's API

## Quick Start

To add a new integration, you'll need to:

1. Create a new module file
2. Implement the `Integration` trait
3. Add tests
4. Export in the category module
5. Add to `IntegrationManager`

## Step-by-Step Guide

### Step 1: Choose a Category

Decide which category your integration belongs to:

- `db/` - Databases (PostgreSQL, Redis, etc.)
- `cloud/` - Cloud providers (AWS, Azure, etc.)
- `queue/` - Message queues (Kafka, RabbitMQ, etc.)
- `containers/` - Container platforms (Docker, Kubernetes)
- `cicd/` - CI/CD platforms (ServiceNow Actions, Jenkins)
- `cache/` - Caching systems (Memcached, Varnish)
- `observability/` - Monitoring tools (Prometheus, Grafana)
- `security/` - Security tools (Vault, LDAP)
- `network/` - Network monitoring (built-in)

### Step 2: Create Module File

Create a new file in the appropriate category:

```bash
touch src/integrations/category/myservice.rs
```

### Step 3: Implement the Integration

Here's a template for a new integration:

```rust
//! MyService Integration
//!
//! Brief description of what this integration monitors.

use super::super::framework::*;
use async_trait::async_trait;
use std::collections::HashMap;

/// MyService integration
pub struct MyServiceIntegration {
    config: IntegrationConfig,
    // Add your fields here
    host: String,
    port: u16,
    api_token: Option<String>,
}

impl MyServiceIntegration {
    /// Create new MyService integration
    pub fn new(host: &str, port: u16) -> Result<Self, IntegrationError> {
        let mut tags = HashMap::new();
        tags.insert("service_type".to_string(), "myservice".to_string());

        let config = IntegrationConfig {
            name: "myservice".to_string(),
            interval_secs: 60,
            tags,
            timeout_secs: 10,
        };

        Ok(Self {
            config,
            host: host.to_string(),
            port,
            api_token: None,
        })
    }

    /// Create with API token
    pub fn with_token(mut self, token: &str) -> Self {
        self.api_token = Some(token.to_string());
        self
    }

    /// Check if MyService is accessible
    async fn check_access(&self) -> bool {
        // Implement connectivity check
        // Return true if service is accessible, false otherwise
        let timeout = tokio::time::Duration::from_secs(5);
        tokio::time::timeout(
            timeout,
            tokio::net::TcpStream::connect((self.host.as_str(), self.port)),
        )
        .await
        .is_ok_and(|r| r.is_ok())
    }

    /// Collect metrics from MyService
    async fn collect_service_metrics(&self) -> Result<Vec<Metric>, IntegrationError> {
        let mut metrics = vec![];

        // Implement metric collection
        // Example:
        metrics.push(Metric::gauge(
            "myservice.connections.active",
            42.0,
            self.name(),
        ));

        Ok(metrics)
    }

    /// Collect all metrics
    async fn collect_all_metrics(&self) -> Result<Vec<Metric>, IntegrationError> {
        let mut all_metrics = vec![];

        // Check connectivity first
        if !self.check_access().await {
            all_metrics.push(Metric::gauge(
                "myservice.connection.status",
                0.0,
                self.name(),
            ));
            return Ok(all_metrics);
        }

        all_metrics.push(Metric::gauge(
            "myservice.connection.status",
            1.0,
            self.name(),
        ));

        match self.collect_service_metrics().await {
            Ok(metrics) => all_metrics.extend(metrics),
            Err(e) => tracing::warn!("Failed to collect metrics: {}", e),
        }

        Ok(all_metrics)
    }

    /// Collect events from MyService
    async fn collect_service_events(&self) -> Result<Vec<Event>, IntegrationError> {
        let mut events = vec![];

        // Check connectivity
        if !self.check_access().await {
            events.push(
                Event::new(
                    "MyService Unavailable",
                    format!("Cannot connect to MyService at {}:{}", self.host, self.port),
                    EventSeverity::Critical,
                    self.name(),
                    "myservice_unavailable",
                ),
            );
            return Ok(events);
        }

        // Detect other events
        // Example:
        if self.some_condition().await? {
            events.push(
                Event::new(
                    "MyService High Load",
                    "Load is above 90%".to_string(),
                    EventSeverity::Warning,
                    self.name(),
                    "high_load",
                ),
            );
        }

        Ok(events)
    }
}

#[async_trait]
impl Integration for MyServiceIntegration {
    fn name(&self) -> &str {
        &self.config.name
    }

    async fn collect_metrics(&self) -> Result<Vec<Metric>, IntegrationError> {
        self.collect_all_metrics().await
    }

    async fn collect_events(&self) -> Result<Vec<Event>, IntegrationError> {
        self.collect_service_events().await
    }

    async fn health_check(&self) -> Result<(), IntegrationError> {
        if self.check_access().await {
            Ok(())
        } else {
            Err(IntegrationError::Connection(format!(
                "Cannot connect to MyService at {}:{}",
                self.host, self.port
            )))
        }
    }

    fn config(&self) -> &IntegrationConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_myservice_integration_creation() {
        let service = MyServiceIntegration::new("localhost", 8080);
        assert!(service.is_ok());
        assert_eq!(service.unwrap().name(), "myservice");
    }

    #[test]
    fn test_myservice_with_token() {
        let service = MyServiceIntegration::new("localhost", 8080)
            .unwrap()
            .with_token("secret-token");
        assert_eq!(service.api_token, Some("secret-token".to_string()));
    }
}
```

### Step 4: Export in Category Module

Add your integration to the category's `mod.rs`:

```rust
// src/integrations/category/mod.rs

pub mod myservice;
pub use myservice::MyServiceIntegration;
```

### Step 5: Add to IntegrationManager

Add a builder method to `IntegrationManager`:

```rust
// src/integrations/mod.rs

/// Add MyService integration
pub fn with_myservice(&mut self, host: &str, port: u16) -> &mut Self {
    match category::myservice::MyServiceIntegration::new(host, port) {
        Ok(service) => {
            self.register("myservice", service);
        }
        Err(e) => {
            error!(error = %e, "Failed to create MyService integration");
        }
    }
    self
}
```

## Best Practices

### 1. Connection Health

Always implement a `check_access()` method that:
- Returns quickly (within 5 seconds)
- Doesn't require authentication if possible
- Handles network timeouts gracefully

### 2. Metric Naming

Follow the naming convention: `<integration>.<resource>.<metric>`

Examples:
- `postgresql.connections.active`
- `aws.ec2.instances.running`
- `kubernetes.pods.running`

### 3. Error Handling

- Use `IntegrationError` for all errors
- Provide meaningful error messages
- Log warnings for non-fatal errors
- Don't fail entire collection for one bad metric

### 4. Event Detection

Generate events for:
- Connection failures (Critical)
- High resource usage (Warning)
- Performance degradation (Warning)
- Anomalous behavior (Info/Warning)

### 5. Testing

Include tests for:
- Integration creation
- Configuration
- Metric collection (mocked)
- Event detection (mocked)

Example:

```rust
#[tokio::test]
async fn test_metric_collection() {
    let service = MyServiceIntegration::new("localhost", 8080).unwrap();
    let metrics = service.collect_metrics().await.unwrap();
    assert!(!metrics.is_empty());
}
```

### 6. Documentation

Add module-level documentation:

```rust
//! MyService Integration
//!
//! Monitor MyService for:
//! - Connection health
//! - Request rates
//! - Error rates
//! - Resource usage
//!
//! ## Quick Start
//!
//! ```rust
//! let service = MyServiceIntegration::new("localhost", 8080)?;
//! ```
```

## Common Patterns

### HTTP API Integration

```rust
async fn collect_api_metrics(&self) -> Result<Vec<Metric>, IntegrationError> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/metrics", self.base_url);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", self.token))
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| IntegrationError::Connection(e.to_string()))?;

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| IntegrationError::Other(e.to_string()))?;

    let mut metrics = vec![];

    if let Some(value) = data["active_connections"].as_f64() {
        metrics.push(Metric::gauge(
            "myservice.connections.active",
            value,
            self.name(),
        ));
    }

    Ok(metrics)
}
```

### CLI-based Integration

```rust
async fn collect_cli_metrics(&self) -> Result<Vec<Metric>, IntegrationError> {
    let output = tokio::process::Command::new("myservice-cli")
        .args(["status", "--format", "json"])
        .output()
        .await
        .map_err(|e| IntegrationError::Connection(e.to_string()))?;

    if !output.status.success() {
        return Err(IntegrationError::Other(
            "CLI command failed".to_string()
        ));
    }

    let data: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| IntegrationError::Other(e.to_string()))?;

    // Parse metrics from output
    Ok(vec![])
}
```

### TCP-based Integration

```rust
async fn check_tcp_access(&self) -> bool {
    let timeout = Duration::from_secs(5);
    tokio::time::timeout(
        timeout,
        tokio::net::TcpStream::connect((self.host.as_str(), self.port)),
    )
    .await
    .is_ok_and(|r| r.is_ok())
}
```

## Testing Your Integration

Run tests:

```bash
cargo test myservice --lib
```

Build and verify:

```bash
cargo build --lib
```

Check for warnings:

```bash
cargo clippy --lib
```

## Submitting Your Integration

1. Ensure all tests pass
2. Add documentation
3. Update the integrations README
4. Create a PR with:
   - Description of the integration
   - List of metrics collected
   - Example usage
   - Any special configuration needed

## Example: Complete Integration

See existing integrations for complete examples:

- Simple HTTP API: `src/integrations/db/redis.rs`
- CLI-based: `src/integrations/cloud/aws.rs`
- Complex with events: `src/integrations/queue/rabbitmq.rs`

## Questions?

- Join our [Discord](https://discord.gg/fluxx)
- Open an [issue](https://servicenow.com/fluxx/agent/issues)
- Email: dev@fluxx.dev

Thank you for contributing to Fluxx Agent!
