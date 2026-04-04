use fluxx_agent::models::event::{Event, ProtocolType};
use fluxx_agent::processing::TopologyProcessor;
use fluxx_agent::core::traits::Processor;
use fluxx_agent::analysis::protocol::HttpAnalyzer;
use fluxx_agent::analysis::Analyzer;
use fluxx_agent::alerts::AlertEngine;
use fluxx_agent::alerts::models::AlertRule;
use fluxx_agent::analysis::insights::InsightLevel;
use std::sync::Arc;

#[tokio::test]
async fn test_high_fidelity_pipeline_enrichment() {
    // 1. Initialise Topology Processor
    let topo = TopologyProcessor::new("fluxx-project", "staging", "auth-service");
    let mut event = Event::new("ev-1".to_string(), ProtocolType::Http);
    event.latency_us = 2_000_000; // Trigger high-latency anomaly

    // 2. Process event (Enrichment check)
    let enriched = topo.process(event).await.unwrap().unwrap();
    
    assert_eq!(enriched.project_id.unwrap(), "fluxx-project");
    assert_eq!(enriched.environment_id.unwrap(), "staging");
    assert_eq!(enriched.service_id.unwrap(), "auth-service");
}

#[tokio::test]
async fn test_anomaly_to_alert_flow() {
    // 1. Initialise Intelligence/Alerting
    let analyzer = HttpAnalyzer::default();
    let mut alert_engine = AlertEngine::default_rules();
    
    // 2. Create Event triggering anomaly (Latency > 1s)
    let mut event = Event::new("ev-anom".to_string(), ProtocolType::Http);
    event.latency_us = 1_500_000;

    // 3. Analyze to Insights
    let insights = analyzer.analyze(&event);
    assert!(!insights.is_empty());
    assert_eq!(insights[0].level, InsightLevel::Warning);

    // 4. Alert Engine matching
    // Let's manually add a rule for HTTP Warnings to ensure it triggers
    alert_engine.add_rule(AlertRule {
        name: "HttpLatencyBreach".to_string(),
        target_protocol: Some(ProtocolType::Http),
        min_severity: InsightLevel::Warning,
    });

    let alert = alert_engine.process_insight(insights[0].clone());
    assert!(alert.is_some());
    assert_eq!(alert.unwrap().source_insight.protocol, ProtocolType::Http);
}
