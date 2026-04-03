use crate::core::error::Result;
use crate::core::traits::{Capturer, Processor, Transporter, Protocol};
use crate::core::pipeline::Pipeline;
use crate::core::lifecycle::Lifecycle;
use std::sync::Arc;
use tracing::{info, error, debug};

/// Agent: The high-level orchestrator for the Fluxx network agent.
pub struct Agent {
    capturers: Vec<Arc<dyn Capturer>>,
    processors: Vec<Arc<dyn Processor>>,
    transporters: Vec<Arc<dyn Transporter>>,
    protocols: Vec<Arc<dyn Protocol>>, // Lifecycle-managed protocols
    lifecycle: Lifecycle,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            capturers: Vec::new(),
            processors: Vec::new(),
            transporters: Vec::new(),
            protocols: Vec::new(),
            lifecycle: Lifecycle::new(),
        }
    }

    /// Registers a specialized network protocol for management.
    pub fn add_protocol(&mut self, protocol: Arc<dyn Protocol>) {
        info!("Agent: Registering protocol '{}'", protocol.name());
        self.protocols.push(protocol);
    }

    pub fn add_capturer(&mut self, capturer: Arc<dyn Capturer>) {
        self.capturers.push(capturer);
    }

    pub fn add_processor(&mut self, processor: Arc<dyn Processor>) {
        self.processors.push(processor);
    }

    pub fn add_transporter(&mut self, transporter: Arc<dyn Transporter>) {
        self.transporters.push(transporter);
    }

    /// Orchestrates the startup of the agent daemon.
    pub async fn start(&self) -> Result<()> {
        info!("Agent: Coordinating startup for {} protocols...", self.protocols.len());

        // Lifecycle Orchestration: Common behavior for ALL protocols (Logs, Status tracking)
        for protocol in &self.protocols {
            info!("Agent: Protocol '{}' starting...", protocol.name());
            if let Err(e) = protocol.start().await {
                error!("Agent: Failed to start protocol '{}': {}", protocol.name(), e);
                return Err(e);
            }
            debug!("Agent: Protocol '{}' is now active.", protocol.name());
        }

        // Pipeline Orchestration: Data flow management
        let pipeline = Pipeline::new(
            self.capturers.clone(),
            self.processors.clone(),
            self.transporters.clone(),
        );

        pipeline.run(self.lifecycle.token()).await?;

        // Wait for shutdown trigger
        self.lifecycle.wait_for_shutdown().await;

        // Lifecycle Orchestration: Common behavior for ALL protocols (Graceful cleanup)
        for protocol in &self.protocols {
            info!("Agent: Protocol '{}' stopping...", protocol.name());
            if let Err(e) = protocol.stop().await {
                error!("Agent: Graceful stop failed for '{}': {}", protocol.name(), e);
            }
        }

        info!("Agent: All operations completed.");
        Ok(())
    }

    pub fn shutdown(&self) {
        info!("Agent: Global shutdown initiated.");
        self.lifecycle.shutdown();
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}
