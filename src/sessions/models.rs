use serde::{Deserialize, Serialize};
use crate::models::event::Event;
use std::time::SystemTime;

/// Session: Defines a logical grouping of network events.
/// This could represent a single WebSocket connection or a chain of HTTP requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub created_at_us: u64,
    pub last_seen_us: u64,
    pub events: Vec<Event>, // Timeline of events in this session
}

impl Session {
    pub fn new(id: String) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;

        Self {
            id,
            created_at_us: now,
            last_seen_us: now,
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.last_seen_us = event.timestamp_us;
        self.events.push(event);
    }
}
