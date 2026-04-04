use crate::alerts::models::{Alert, AlertRule, AlertSeverity};
use crate::analysis::insights::{Insight, InsightLevel};
use crate::core::error::Result;
use std::sync::{Arc, RwLock};
use tracing::{info, warn};

pub mod models;

/// AlertEngine: Coordinates the transformation of high-level insights into 
/// context-aware alerts based on a pluggable rules system.
#[derive(Debug, Default)]
pub struct AlertEngine {
    rules: Arc<RwLock<Vec<AlertRule>>>,
}

impl AlertEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Default alert rules for common protocol criticalities.
    pub fn default_rules() -> Self {
        let mut engine = Self::new();
        engine.add_rule(AlertRule {
            name: "CriticalReconnects".to_string(),
            target_protocol: Some(crate::models::event::ProtocolType::Ws),
            min_severity: InsightLevel::Critical,
        });
        
        engine.add_rule(AlertRule {
            name: "GenericAnomaly".to_string(),
            target_protocol: None,
            min_severity: InsightLevel::Anomaly,
        });
        
        engine
    }

    pub fn add_rule(&mut self, rule: AlertRule) {
        let mut rules = self.rules.write().expect("RwLock poisoned");
        rules.push(rule);
    }

    /// Evaluates an insight against the rule system and emits an alert if required.
    pub fn process_insight(&self, insight: Insight) -> Option<Alert> {
        let rules = self.rules.read().expect("RwLock poisoned");
        
        for rule in &*rules {
            if rule.matches(&insight) {
                let severity = match insight.level {
                    InsightLevel::Critical => AlertSeverity::Critical,
                    _ => AlertSeverity::Warning,
                };

                let alert = Alert {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity,
                    message: format!("[{}] {}", rule.name, insight.message),
                    source_insight: insight.clone(),
                    timestamp_us: insight.metadata.get("timestamp_us")
                        .and_then(|t| t.parse().ok())
                        .unwrap_or_default(),
                };

                info!(id = %alert.id, msg = %alert.message, "AlertEngine: Triggered alert breach.");
                return Some(alert);
            }
        }

        None
    }
}
