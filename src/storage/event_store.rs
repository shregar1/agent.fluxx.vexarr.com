use crate::models::event::Event;
use crate::core::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// TimeWindow: Defines the boundaries for chronological event processing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start_us: u64,
    pub end_us: u64,
}

impl TimeWindow {
    pub fn new(start_us: u64, end_us: u64) -> Self {
        Self { start_us, end_us }
    }
}

/// EventStore: Interface for high-performance event persistence.
/// Purely focused on storage and range retrieval to ensure pluggable backends.
#[async_trait]
pub trait EventStore: Send + Sync + std::fmt::Debug {
    /// Inserts an event into the store.
    async fn insert(&self, event: Event) -> Result<()>;
    
    /// Queries a block of events within the specified time range.
    async fn query(&self, window: TimeWindow) -> Result<Vec<Event>>;
    
    /// Returns the total volume of events in storage.
    async fn len(&self) -> Result<usize>;
}
