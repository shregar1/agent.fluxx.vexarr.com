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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::insights::{Insight, InsightLevel};

    // Test 171: AlertSeverity equality
    #[test]
    fn test_alert_severity_equality() {
        assert_eq!(AlertSeverity::Info, AlertSeverity::Info);
        assert_eq!(AlertSeverity::Warning, AlertSeverity::Warning);
        assert_eq!(AlertSeverity::Critical, AlertSeverity::Critical);
    }

    // Test 172: AlertSeverity inequality
    #[test]
    fn test_alert_severity_inequality() {
        assert_ne!(AlertSeverity::Info, AlertSeverity::Warning);
        assert_ne!(AlertSeverity::Critical, AlertSeverity::Info);
    }

    // Test 173: AlertRule matches protocol
    #[test]
    fn test_alert_rule_matches_protocol() {
        let rule = AlertRule {
            name: "Test".to_string(),
            target_protocol: Some(ProtocolType::Http),
            min_severity: InsightLevel::Warning,
        };
        let insight = Insight::new(ProtocolType::Http, InsightLevel::Warning, "Test");
        assert!(rule.matches(&insight));
    }

    // Test 174: AlertRule no match different protocol
    #[test]
    fn test_alert_rule_no_match_protocol() {
        let rule = AlertRule {
            name: "Test".to_string(),
            target_protocol: Some(ProtocolType::Http),
            min_severity: InsightLevel::Warning,
        };
        let insight = Insight::new(ProtocolType::Ws, InsightLevel::Warning, "Test");
        assert!(!rule.matches(&insight));
    }

    // Test 175: AlertRule matches any protocol when None
    #[test]
    fn test_alert_rule_matches_any_protocol() {
        let rule = AlertRule {
            name: "Test".to_string(),
            target_protocol: None,
            min_severity: InsightLevel::Warning,
        };
        let insight = Insight::new(ProtocolType::Grpc, InsightLevel::Warning, "Test");
        assert!(rule.matches(&insight));
    }

    // Test 176: AlertRule matches critical regardless
    #[test]
    fn test_alert_rule_matches_critical() {
        let rule = AlertRule {
            name: "Test".to_string(),
            target_protocol: Some(ProtocolType::Http),
            min_severity: InsightLevel::Warning,
        };
        let insight = Insight::new(ProtocolType::Http, InsightLevel::Critical, "Test");
        assert!(rule.matches(&insight));
    }

    // Test 177: AlertSeverity serialization
    #[test]
    fn test_alert_severity_serialization() {
        let severity = AlertSeverity::Critical;
        let json = serde_json::to_string(&severity).unwrap();
        assert!(json.contains("Critical"));
    }

    // Test 178: Alert serialization
    #[test]
    fn test_alert_serialization() {
        let alert = Alert {
            id: "alert-1".to_string(),
            severity: AlertSeverity::Warning,
            message: "Test alert".to_string(),
            source_insight: Insight::new(ProtocolType::Http, InsightLevel::Warning, "Test"),
            timestamp_us: 12345,
        };
        let json = serde_json::to_string(&alert).unwrap();
        assert!(json.contains("alert-1"));
        assert!(json.contains("Test alert"));
    }
}
