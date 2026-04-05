//! System Resource Capturer for Host Observability
//! Collects CPU, memory, and disk telemetry from the standalone server.

use crate::core::traits::Capturer;
use crate::models::event::{Event, EventSeverity, ProtocolType, Metadata};
use crate::core::error::Result;
use async_trait::async_trait;
use sysinfo::{System, CpuRefreshKind, RefreshKind};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, debug, error};
use std::time::SystemTime;

/// SystemCapturer: High-performance cross-platform system resource monitor.
pub struct SystemCapturer {
    name: String,
    interval_secs: u64,
}

impl SystemCapturer {
    pub fn new(name: &str, interval_secs: u64) -> Self {
        Self {
            name: name.to_string(),
            interval_secs,
        }
    }
}

#[async_trait]
impl Capturer for SystemCapturer {
    fn name(&self) -> &str {
        &self.name
    }

    async fn capture(&self, tx: mpsc::Sender<Arc<Event>>) -> Result<()> {
        info!("SystemCapturer: Activating host-level resource monitoring ({}s interval)...", self.interval_secs);
        
        let mut sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory()
        );

        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(self.interval_secs));
        let agent_name = whoami::hostname();

        loop {
            interval.tick().await;
            
            // Refresh system data
            sys.refresh_all();

            // 1. CPU Usage
            let cpu_usage = sys.global_cpu_info().cpu_usage();
            
            // 2. Memory Usage
            let total_mem = sys.total_memory();
            let used_mem = sys.used_memory();
            let mem_pct = (used_mem as f32 / total_mem as f32) * 100.0;

            debug!(cpu = %cpu_usage, mem_pct = %mem_pct, "System metrics collected");

            let mut metadata = Metadata::new();
            metadata.add("cpu_usage_pct", &format!("{:.2}", cpu_usage));
            metadata.add("memory_used_bytes", &used_mem.to_string());
            metadata.add("memory_total_bytes", &total_mem.to_string());
            metadata.add("memory_usage_pct", &format!("{:.2}", mem_pct));
            metadata.add("load_avg_1m", &sys.load_average().one.to_string());
            metadata.add("uptime_h", &format!("{:.2}", sys.uptime() as f64 / 3600.0));
            metadata.user_agent = Some(format!("fluxx-agent/{}", agent_name));

            let severity = if cpu_usage > 90.0 || mem_pct > 90.0 {
                EventSeverity::Critical
            } else if cpu_usage > 70.0 || mem_pct > 70.0 {
                EventSeverity::High
            } else {
                EventSeverity::Info
            };

            let timestamp_us = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_micros() as u64;

            let event = Event {
                id: uuid::Uuid::new_v4().to_string(),
                session_id: Some("default_system_session".to_string()),
                project_id: Some("standalone".to_string()),
                environment_id: Some("production".to_string()),
                service_id: Some("host-health".to_string()),
                timestamp_us,
                protocol: ProtocolType::System,
                latency_us: 0,
                severity,
                payload: Some(format!("System Monitor: CPU {:.1}% | RAM {:.1}%", cpu_usage, mem_pct)),
                metadata,
            };

            if let Err(e) = tx.send(Arc::new(event)).await {
                error!("SystemCapturer: Failed to dispatch telemetry: {}", e);
                // Exit loop if receiver is closed
                break;
            }
        }

        Ok(())
    }
}
