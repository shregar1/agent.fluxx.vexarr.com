use async_trait::async_trait;
use crate::core::error::Result;
use crate::models::event::Event;
use crate::core::traits::Transporter;
use tracing::info;

#[derive(Debug)]
pub struct StdoutTransporter;

#[async_trait]
impl Transporter for StdoutTransporter {
    async fn send(&self, event: Arc<Event>) -> Result<()> {
        info!(
            protocol=?event.protocol, 
            id=%event.id, 
            latency=%event.latency_us,
            "Stdout: Dispatching event."
        );
        
        if let Some(p) = event.payload.as_ref() {
            let payload_str = String::from_utf8_lossy(p);
            debug!(size=p.len(), "Payload preview: {}", payload_str);
        }
        
        debug!(
            source=?event.metadata.source_ip, 
            ua=?event.metadata.user_agent, 
            extras=?event.metadata.extra,
            "Metadata details."
        );
        
        Ok(())
    }

    fn name(&self) -> &str {
        "StdoutTransporter"
    }
}
