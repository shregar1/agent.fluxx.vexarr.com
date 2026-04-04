//! Alert System Example
//!
//! This example demonstrates how to use the Fluxx Agent alert system
//! with multiple notification channels.
//!
//! ## Running
//!
//! ```bash
//! # With configuration file
//! ALERT_CONFIG=./alerts.yaml cargo run --example alert_example
//!
//! # With environment variables
//! SLACK_WEBHOOK_URL=https://hooks.slack.com/... cargo run --example alert_example
//! ```

use fluxx_agent::alerts::{
    AlertManager, AlertManagerBuilder, AlertSeverity,
    config::{load_config, from_env},
    rules::{RuleEngine, templates},
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚨 Fluxx Agent Alert System Example\n");

    // Method 1: Build from environment variables
    let manager = if std::env::var("ALERT_CONFIG").is_ok() {
        println!("Loading configuration from file...");
        let config = load_config(&std::env::var("ALERT_CONFIG")?)?;
        AlertManager::new(config).await?
    } else {
        println!("Building from environment variables...");
        AlertManagerBuilder::new()
            .add_slack(
                "slack",
                &std::env::var("SLACK_WEBHOOK_URL")
                    .unwrap_or_else(|_| "https://hooks.slack.com/test".to_string()),
            )
            .add_webhook(
                "webhook",
                &std::env::var("WEBHOOK_URL")
                    .unwrap_or_else(|_| "https://httpbin.org/post".to_string()),
            )
            .build()
            .await?
    };

    // Health check
    println!("\n📋 Channel Health Check:");
    let health = manager.health_check().await;
    for (channel, healthy) in health {
        let status = if healthy { "✅" } else { "❌" };
        println!("  {} {}", status, channel);
    }

    // Example 1: Direct alert trigger
    println!("\n📤 Sending direct alerts...");

    let critical_alert = manager.trigger(
        AlertSeverity::Critical,
        "Database connection pool exhausted! Connections: 100/100",
        "api-server-1",
        vec!["slack".to_string()],
    ).await;

    if let Some(id) = critical_alert {
        println!("  ✅ Critical alert sent: {}", &id[..8]);
    }

    let warning_alert = manager.trigger(
        AlertSeverity::Warning,
        "High memory usage: 87%",
        "worker-3",
        vec!["slack".to_string()],
    ).await;

    if let Some(id) = warning_alert {
        println!("  ⚠️  Warning alert sent: {}", &id[..8]);
    }

    // Example 2: Rule-based alerts
    println!("\n📊 Testing rule-based alerts...");

    // Create rules using templates
    let rules = vec![
        templates::high_error_rate(0.05, vec!["slack".to_string()]),
        templates::high_latency(500.0, vec!["slack".to_string()]),
        templates::high_cpu(85.0, vec!["slack".to_string()]),
    ];

    let rule_engine = RuleEngine::new(rules);

    // Simulate metrics that trigger alerts
    let mut high_error_metrics = HashMap::new();
    high_error_metrics.insert("error_rate".to_string(), fluxx_agent::alerts::rules::MetricValue::Float(0.1));
    high_error_metrics.insert("latency_p99".to_string(), fluxx_agent::alerts::rules::MetricValue::Float(200.0));

    let triggered = rule_engine.evaluate(&high_error_metrics);
    println!("  Triggered {} rules with high error metrics", triggered.len());

    // Simulate metrics that don't trigger
    let mut normal_metrics = HashMap::new();
    normal_metrics.insert("error_rate".to_string(), fluxx_agent::alerts::rules::MetricValue::Float(0.01));
    normal_metrics.insert("latency_p99".to_string(), fluxx_agent::alerts::rules::MetricValue::Float(100.0));

    let triggered = rule_engine.evaluate(&normal_metrics);
    println!("  Triggered {} rules with normal metrics", triggered.len());

    // Example 3: Alert lifecycle
    println!("\n🔄 Alert Lifecycle Demo:");

    let alert_id = manager.trigger(
        AlertSeverity::Warning,
        "Disk space low: 85% used",
        "storage-server-1",
        vec!["slack".to_string()],
    ).await;

    if let Some(id) = alert_id {
        println!("  1. Alert created: {}", &id[..8]);

        // Get alert status
        if let Some(alert) = manager.get_alert(&id).await {
            println!("  2. Alert state: {:?}", alert.state);
            println!("  3. Notification count: {}", alert.notification_count);
        }

        // Acknowledge
        manager.acknowledge(&id, "admin@company.com").await;
        println!("  4. Alert acknowledged");

        if let Some(alert) = manager.get_alert(&id).await {
            println!("  5. Alert state: {:?}", alert.state);
        }

        // Resolve
        manager.resolve(&id).await;
        println!("  6. Alert resolved");

        if let Some(alert) = manager.get_alert(&id).await {
            println!("  7. Alert state: {:?}", alert.state);
        }
    }

    // Example 4: List all alerts
    println!("\n📋 All Tracked Alerts:");
    let all_alerts = manager.get_alerts(None).await;
    println!("  Total: {} alerts", all_alerts.len());

    let firing = manager.get_alerts(Some(fluxx_agent::alerts::AlertState::Firing)).await;
    println!("  Firing: {} alerts", firing.len());

    let resolved = manager.get_alerts(Some(fluxx_agent::alerts::AlertState::Resolved)).await;
    println!("  Resolved: {} alerts", resolved.len());

    println!("\n✅ Example complete!");
    println!("\nTo use real integrations:");
    println!("  export SLACK_WEBHOOK_URL='https://hooks.slack.com/services/...'");
    println!("  export EMAIL_USER='alerts@company.com'");
    println!("  export EMAIL_PASS='...'");
    println!("  export PAGERDUTY_KEY='...'");

    Ok(())
}
