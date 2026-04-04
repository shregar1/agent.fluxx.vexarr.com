use serde::{Deserialize, Serialize};

/// Explanation: A high-fidelity, structured interpretation of a network anomaly 
/// or complex request. Designed for architectural clarity in the UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredExplanation {
    pub summary: String,
    pub root_cause: String,
    pub recommendation: String,
    pub impacted_services: Vec<String>,
}

impl StructuredExplanation {
    pub fn mock(message: &str) -> Self {
        Self {
            summary: format!("Fluxx AI analyzed the event: {}", message),
            root_cause: "High-latency observed at the edge due to potential network congestion.".to_string(),
            recommendation: "Check upstream service health and potential kernel-level dropped packets via eBPF.".to_string(),
            impacted_services: vec!["AuthV2".to_string(), "PaymentGateway".to_string()],
        }
    }
}
