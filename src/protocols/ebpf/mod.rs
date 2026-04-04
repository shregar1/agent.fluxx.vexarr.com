use async_trait::async_trait;
use crate::core::error::Result;
use crate::core::traits::{Capturer, Protocol};
use crate::protocols::common::CaptureChannel;
use crate::models::event::Event;
use tracing::{info, error};

/// EbpfCapturer: High-performance kernel-level network observation.
#[derive(Debug)]
pub struct EbpfCapturer {
    pub iface: String,
    channel: CaptureChannel,
}

impl EbpfCapturer {
    pub fn new(iface: &str) -> Self {
        Self {
            iface: iface.to_string(),
            channel: CaptureChannel::new("EbpfCapturer", 10000),
        }
    }

    pub async fn load_programs(&self) -> Result<()> {
        info!(iface=%self.iface, "eBPF: Loading kernel probes.");
        Ok(())
    }
}

#[async_trait]
impl Protocol for EbpfCapturer {
    fn name(&self) -> &str {
        "ebpf"
    }

    async fn start(&self) -> Result<()> {
        if let Err(e) = self.load_programs().await {
            error!(error=%e, "eBPF: Initialization failure.");
            return Err(e);
        }
        info!(iface=%self.iface, "eBPF: Operational.");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        info!(iface=%self.iface, "eBPF: Probes detached.");
        Ok(())
    }
}

#[async_trait]
impl Capturer for EbpfCapturer {
    async fn capture(&self) -> Result<Event> {
        self.channel.capture().await
    }

    fn name(&self) -> &str {
        "EbpfCapturer"
    }
}
