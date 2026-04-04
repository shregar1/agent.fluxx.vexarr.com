use serde::{Deserialize, Serialize};

/// MetricPoint: A point-in-time snapshot of service performance.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricPoint {
    pub avg_latency_us: u64,
    pub error_rate: f64,
    pub throughput_rps: u64,
}

/// EnvDiff: High-fidelity comparison between two deployment environments.
/// Enforces structural identification of performance regressions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvDiff {
    pub source_env: String,
    pub target_env: String,
    pub latency_delta_us: i64,
    pub error_rate_delta: f64,
    pub throughput_delta: i64,
}

impl EnvDiff {
    pub fn calculate(source: &MetricPoint, target: &MetricPoint, s_name: &str, t_name: &str) -> Self {
        Self {
            source_env: s_name.to_string(),
            target_env: t_name.to_string(),
            latency_delta_us: (target.avg_latency_us as i64) - (source.avg_latency_us as i64),
            error_rate_delta: target.error_rate - source.error_rate,
            throughput_delta: (target.throughput_rps as i64) - (source.throughput_rps as i64),
        }
    }
}
