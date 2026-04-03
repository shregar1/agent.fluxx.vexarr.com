use async_trait::async_trait;
use crate::core::error::Result;
use crate::core::event::Event;
use std::fmt::Debug;

/// High-level trait for specialized network protocols.
/// Each protocol implementation should focus purely on its unique logic.
#[async_trait]
pub trait Protocol: Send + Sync + Debug {
    /// Unique identifier for the protocol
    fn name(&self) -> &str;

    /// Specific protocol startup logic
    async fn start(&self) -> Result<()>;

    /// Specific protocol shutdown logic
    async fn stop(&self) -> Result<()>;
}

/// The capture layer: Responsibility for data ingestion.
#[async_trait]
pub trait Capturer: Send + Sync + Debug {
    /// Captures a new Fluxx event from the protocol source.
    async fn capture(&self) -> Result<Event>;
    fn name(&self) -> &str;
}

/// The processing layer: Responsibility for data transformation.
#[async_trait]
pub trait Processor: Send + Sync + Debug {
    /// Transforms or enriches a Fluxx event.
    async fn process(&self, event: Event) -> Result<Option<Event>>;
    fn name(&self) -> &str;
}

/// The transport layer: Responsibility for data delivery.
#[async_trait]
pub trait Transporter: Send + Sync + Debug {
    /// Delivers the final Fluxx event to its destination.
    async fn send(&self, event: Event) -> Result<()>;
    fn name(&self) -> &str;
}
