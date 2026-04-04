use crate::models::event::Event;
use crate::storage::event_store::{EventStore, TimeWindow};
use crate::core::error::Result;
use async_trait::async_trait;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

/// InMemoryEventStore: Chronological persistence using a BTreeMap index.
/// Focused purely on pluggable storage and range retrieval efficiency.
#[derive(Debug, Default)]
pub struct InMemoryEventStore {
    events: Arc<RwLock<BTreeMap<u64, Vec<Event>>>>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn insert(&self, event: Event) -> Result<()> {
        let mut map = self.events.write().expect("RwLock poisoned");
        map.entry(event.timestamp_us).or_default().push(event);
        Ok(())
    }

    async fn query(&self, window: TimeWindow) -> Result<Vec<Event>> {
        let map = self.events.read().expect("RwLock poisoned");
        
        // BTreeMap provides an efficient O(log N) range scan for time windows.
        let events = map.range(window.start_us..=window.end_us)
            .flat_map(|(_, evs)| evs.clone())
            .collect();
            
        Ok(events)
    }

    async fn len(&self) -> Result<usize> {
        let map = self.events.read().expect("RwLock poisoned");
        Ok(map.values().map(|v| v.len()).sum())
    }
}
