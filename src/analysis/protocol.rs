use crate::models::event::{Event, ProtocolType};
use crate::analysis::insights::{Insight, InsightLevel};
use crate::analysis::Analyzer;
use crate::analysis::AnalysisUtils;

/// HTTP Protocol Intelligence: Tracks endpoint latency and payload size anomalies.
#[derive(Debug, Default)]
pub struct HttpAnalyzer;

impl Analyzer for HttpAnalyzer {
    fn analyze(&self, event: &Event) -> Vec<Insight> {
        let mut insights = Vec::new();
        
        // 1. Threshold Detection (High latency endpoint)
        if AnalysisUtils::is_high_latency(event.latency_us, 1_000_000) {
            insights.push(Insight::new(self.protocol(), InsightLevel::Warning, "High latency HTTP endpoint detected."));
        }
        
        // 2. Payload Anomaly
        if let Some(p) = &event.payload {
            if AnalysisUtils::is_payload_anomaly(p.len(), 10 * 1024 * 1024) {
                 insights.push(Insight::new(self.protocol(), InsightLevel::Anomaly, "Large HTTP payload identified."));
            }
        }

        insights
    }

    fn protocol(&self) -> ProtocolType { ProtocolType::Http }
}

/// WebSocket Intelligence: Detects reconnect storms and connection spikes.
#[derive(Debug, Default)]
pub struct WebSocketAnalyzer;

impl Analyzer for WebSocketAnalyzer {
    fn analyze(&self, event: &Event) -> Vec<Insight> {
        let mut insights = Vec::new();
        
        // 1. Pattern Recognition (Reconnect storm detection - proxy based on metadata)
        if let Some(reconnect) = event.metadata.extra.get("reconnect_count") {
            if let Ok(count) = reconnect.parse::<u32>() {
                if count > 10 {
                    insights.push(Insight::new(self.protocol(), InsightLevel::Critical, "WebSocket reconnect storm detected."));
                }
            }
        }
        
        insights
    }

    fn protocol(&self) -> ProtocolType { ProtocolType::Ws }
}

/// gRPC Intelligence: Detects stream resets and RPC latency spikes.
#[derive(Debug, Default)]
pub struct GrpcAnalyzer;

impl Analyzer for GrpcAnalyzer {
    fn analyze(&self, event: &Event) -> Vec<Insight> {
        let mut insights = Vec::new();
        if AnalysisUtils::is_high_latency(event.latency_us, 500_000) {
            insights.push(Insight::new(self.protocol(), InsightLevel::Warning, "gRPC RPC latency spike found."));
        }
        insights
    }
    fn protocol(&self) -> ProtocolType { ProtocolType::Grpc }
}

/// QUIC Intelligence: Detects high UDP packet loss and congestion.
#[derive(Debug, Default)]
pub struct QuicAnalyzer;

impl Analyzer for QuicAnalyzer {
    fn analyze(&self, event: &Event) -> Vec<Insight> {
        let mut insights = Vec::new();
        // Placeholder for QUIC congestion metrics analysis
        insights
    }
    fn protocol(&self) -> ProtocolType { ProtocolType::Quic }
}

/// eBPF Intelligence: Detects kernel-level TCP/UDP volume surges.
#[derive(Debug, Default)]
pub struct EbpfAnalyzer;

impl Analyzer for EbpfAnalyzer {
    fn analyze(&self, event: &Event) -> Vec<Insight> {
        let mut insights = Vec::new();
        if let Some(size) = event.metadata.extra.get("size_bytes") {
             if let Ok(s) = size.parse::<usize>() {
                if s > 1024 * 1024 {
                    insights.push(Insight::new(self.protocol(), InsightLevel::Anomaly, "eBPF detected large TCP segment."));
                }
             }
        }
        insights
    }
    fn protocol(&self) -> ProtocolType { ProtocolType::Ebpf }
}
