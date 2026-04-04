use serde::{Deserialize, Serialize};
use crate::analysis::insights::Insight;
use crate::models::event::ProtocolType;

/// Alert: A high-priority notification triggered by protocol anomalies or insights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub source_insight: Insight,
    pub timestamp_us: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// AlertRule: Defines the conditions under which an insight triggers an alert.
#[derive(Debug, Clone)]
pub struct AlertRule {
    pub name: String,
    pub target_protocol: Option<ProtocolType>,
    pub min_severity: crate::analysis::insights::InsightLevel,
}

impl AlertRule {
    pub fn matches(&self, insight: &Insight) -> bool {
        // 1. Check protocol match
        if let Some(proto) = self.target_protocol {
            if proto != insight.protocol {
                return false;
            }
        }
        
        // 2. Check severity threshold (simplified)
        // (Assuming InsightLevel has a priority order similar to AlertSeverity)
        insight.level == self.min_severity || insight.level == crate::analysis::insights::InsightLevel::Critical
    }
}
