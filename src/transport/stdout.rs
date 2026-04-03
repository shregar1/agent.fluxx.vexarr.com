use async_trait::async_trait;
use crate::core::error::Result;
use crate::core::traits::{Event, Transporter};
use tracing::info;

#[derive(Debug)]
pub struct StdoutTransporter;

#[async_trait]
impl Transporter for StdoutTransporter {
    async fn send(&self, event: Event) -> Result<()> {
        info!(">>> Sending event to STDOUT: ID={}, TS={}, METADATA={:?}", 
            event.id, event.timestamp, event.metadata);
        
        let payload_str = String::from_utf8_lossy(&event.payload);
        info!("PAYLOAD: {}", payload_str);
        
        Ok(())
    }

    fn name(&self) -> &str {
        "StdoutTransporter"
    }
}
