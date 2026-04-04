use async_trait::async_trait;
use crate::core::error::Result;
use crate::models::event::Event;
use std::fmt::Debug;
use std::sync::Arc;

/// High-level trait for specialized network protocols.
#[async_trait]
pub trait Protocol: Send + Sync + Debug {
    fn name(&self) -> &str;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
}

/// The capture layer: Optimized for zero-copy data ingestion.
#[async_trait]
pub trait Capturer: Send + Sync + Debug {
    /// Captures a Fluxx event. Uses 'Bytes' inside Event for zero-copy.
    async fn capture(&self) -> Result<Event>;
    fn name(&self) -> &str;
}

/// The processing layer: Sequential transformations and enrichments.
#[async_trait]
pub trait Processor: Send + Sync + std::fmt::Debug {
    /// Transforms the event. Returns Some(Event) to continue or None to drop.
    async fn process(&self, event: Event) -> Result<Option<Event>>;
    fn name(&self) -> &str;
}

/// The transport layer: Concurrent fan-out using reference sharing.
#[async_trait]
pub trait Transporter: Send + Sync + Debug {
    /// Delivers the event. Takes Arc<Event> to support zero-copy fan-out across sinks.
    async fn send(&self, event: Arc<Event>) -> Result<()>;
    fn name(&self) -> &str;
}
