use async_trait::async_trait;
use crate::core::error::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: String,
    pub timestamp: u64,
    pub metadata: std::collections::HashMap<String, String>,
    pub payload: Vec<u8>,
}

/// The minimal, focused trait for network protocols.
/// Each protocol implementation should focus on its unique initialization and cleanup.
#[async_trait]
pub trait Protocol: Send + Sync + Debug {
    /// Unique name of the protocol.
    fn name(&self) -> &str;

    /// Protocol-specific startup logic.
    async fn start(&self) -> Result<()>;

    /// Protocol-specific shutdown logic.
    async fn stop(&self) -> Result<()>;
}

// Keeping existing traits compatible
#[async_trait]
pub trait Capturer: Send + Sync + Debug {
    async fn capture(&self) -> Result<Event>;
    fn name(&self) -> &str;
}

#[async_trait]
pub trait Processor: Send + Sync + Debug {
    async fn process(&self, event: Event) -> Result<Option<Event>>;
    fn name(&self) -> &str;
}

#[async_trait]
pub trait Transporter: Send + Sync + Debug {
    async fn send(&self, event: Event) -> Result<()>;
    fn name(&self) -> &str;
}
