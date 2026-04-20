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

    async fn list_events(&self, project_id: Option<String>, protocol: Option<crate::models::event::ProtocolType>, severity: Option<crate::models::event::EventSeverity>, service_id: Option<String>) -> Vec<Event> {
        let map = self.events.read().expect("RwLock poisoned");
        
        map.values()
            .flat_map(|evs| evs.iter())
            .filter(|ev| {
                let project_match = project_id.is_none() || ev.project_id == project_id;
                let protocol_match = protocol.is_none() || Some(ev.protocol) == protocol;
                let severity_match = severity.is_none() || Some(ev.severity) == severity;
                let service_match = service_id.is_none() || ev.service_id == service_id;
                project_match && protocol_match && severity_match && service_match
            })
            .cloned()
            .collect()
    }

    async fn get_stats(&self) -> serde_json::Value {
        let map = self.events.read().expect("RwLock poisoned");
        let total = map.values().map(|v| v.len()).sum::<usize>();
        
        let mut protocol_counts = std::collections::HashMap::new();
        for evs in map.values() {
            for ev in evs {
                *protocol_counts.entry(format!("{:?}", ev.protocol)).or_insert(0) += 1;
            }
        }

        serde_json::json!({
            "total_events": total,
            "unique_time_buckets": map.len(),
            "protocols": protocol_counts
        })
    }
}
