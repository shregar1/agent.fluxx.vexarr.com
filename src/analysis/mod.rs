use crate::models::event::Event;
use crate::analysis::insights::Insight;
use crate::core::error::Result;
use async_trait::async_trait;
use std::fmt::Debug;

pub mod insights;
pub mod protocol;
pub mod industrial;

/// Protocol Intelligence Interface: Performs stateless analysis of network events.
/// Enforces modular detection of anomalies (surges, high latency, reconnects).
#[async_trait]
pub trait Analyzer: Send + Sync + Debug {
    /// Inspects an event and emits one or more insights based on the analysis.
    fn analyze(&self, event: &Event) -> Vec<Insight>;
    
    /// Identifies the specific protocol this analyzer is specialized for.
    fn protocol(&self) -> crate::models::event::ProtocolType;
}

/// Shared Analysis Logic: Utilities for common threshold detection across all protocols.
pub struct AnalysisUtils;

impl AnalysisUtils {
    pub fn is_high_latency(latency_us: u64, threshold_us: u64) -> bool {
        latency_us > threshold_us
    }

    pub fn is_payload_anomaly(payload_size: usize, threshold_bytes: usize) -> bool {
        payload_size > threshold_bytes
    }
}
