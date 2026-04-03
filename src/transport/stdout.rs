use async_trait::async_trait;
use crate::core::error::Result;
use crate::core::traits::{Event, Transporter};
use tracing::info;

#[derive(Debug)]
pub struct StdoutTransporter;

#[async_trait]
impl Transporter for StdoutTransporter {
    async fn send(&self, event: Event) -> Result<()> {
        info!(">>> [{:?}] Event ID={}, Latency={}us", 
            event.protocol, event.id, event.latency_us);
        
        if let Some(ref p) = event.payload {
            let payload_str = String::from_utf8_lossy(p);
            info!("PAYLOAD ({} bytes): {}", p.len(), payload_str);
        }
        
        info!("METADATA: source={:?}, ua={:?}, extras={:?}", 
            event.metadata.source_ip, 
            event.metadata.user_agent,
            event.metadata.extra);
        
        Ok(())
    }

    fn name(&self) -> &str {
        "StdoutTransporter"
    }
}
