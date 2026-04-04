use crate::models::event::{Event, ProtocolType};
use crate::analysis::insights::{Insight, InsightLevel};
use crate::analysis::{Analyzer, AnalysisUtils};

/// IoT/Sensor Intelligence: Detects stale sensor data or telemetry drift.
#[derive(Debug, Default)]
pub struct SensorAnalyzer;

impl Analyzer for SensorAnalyzer {
    fn analyze(&self, event: &Event) -> Vec<Insight> {
        let mut insights = Vec::new();
        if let Some(drift) = event.metadata.extra.get("drift_factor") {
            if let Ok(f) = drift.parse::<f64>() {
                if f > 0.15 {
                    insights.push(Insight::new(self.protocol(), InsightLevel::Warning, "IoT: Potential sensor drift detected in edge stream."));
                }
            }
        }
        insights
    }
    fn protocol(&self) -> ProtocolType { ProtocolType::Other } // Future: Map to Mqtt
}

/// Factory Automation Intelligence: Tracks actuator response and cycle times.
#[derive(Debug, Default)]
pub struct ActuatorAnalyzer;

impl Analyzer for ActuatorAnalyzer {
    fn analyze(&self, event: &Event) -> Vec<Insight> {
        let mut insights = Vec::new();
        if AnalysisUtils::is_high_latency(event.latency_us, 500_000) {
            insights.push(Insight::new(self.protocol(), InsightLevel::Critical, "Factory: PLC Actuator response breach (latency > 500ms)."));
        }
        insights
    }
    fn protocol(&self) -> ProtocolType { ProtocolType::Modbus }
}

/// Automobile Intelligence: Detects unusual CAN/Ethernet traffic patterns.
#[derive(Debug, Default)]
pub struct VehicleAnalyzer;

impl Analyzer for VehicleAnalyzer {
    fn analyze(&self, event: &Event) -> Vec<Insight> {
        let mut insights = Vec::new();
        if let Some(restarts) = event.metadata.extra.get("ecu_resets") {
             insights.push(Insight::new(self.protocol(), InsightLevel::Critical, "Vehicle: ECU reset detected on CAN/SomeIP bus."));
        }
        insights
    }
    fn protocol(&self) -> ProtocolType { ProtocolType::Ebpf }
}
