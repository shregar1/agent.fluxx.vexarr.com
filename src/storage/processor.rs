use crate::storage::event_store::EventStore;
use crate::models::event::Event;
use crate::core::traits::Processor;
use crate::core::error::Result;
use async_trait::async_trait;
use tracing::debug;
use std::sync::Arc;

/// StorageProcessor: A middleware component that persists events into an `EventStore` 
/// as they flow through the synchronous processing pipeline.
/// Enables time-travel debugging by providing a historical stream of all handled data.
#[derive(Debug, Clone)]
pub struct StorageProcessor {
    store: Arc<dyn EventStore>,
}

impl StorageProcessor {
    pub fn new(store: Arc<dyn EventStore>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl Processor for StorageProcessor {
    async fn process(&self, event: Event) -> Result<Option<Event>> {
        // Persist the event chronologically for time-travel debugging.
        // This operation happens inside the pipeline, ensuring no data loss.
        debug!(id = %event.id, "StorageProcessor: Persisting event to store.");
        if let Err(e) = self.store.insert(event.clone()).await {
            // We log the error but don't stop the pipeline; persistence shouldn't 
            // break the live delivery flow.
            tracing::error!(error=%e, "StorageProcessor: Failed to persist event.");
        }

        // Return the original event to continue downstream (e.g. to transporters).
        Ok(Some(event))
    }

    fn name(&self) -> &str {
        "StorageProcessor"
    }
}
