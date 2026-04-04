use crate::sessions::models::Session;
use crate::models::event::Event;
use async_trait::async_trait;

/// Interface for session data persistence.
/// Enables future support for Redis, Disk, or Cloud storage backends.
#[async_trait]
pub trait SessionStore: Send + Sync + std::fmt::Debug {
    /// Inserts a new event or appends it to an existing session timeline.
    async fn insert_or_update(&self, session_id: String, event: Event);
    
    /// Retrieves a session by its unique identifier.
    async fn get(&self, id: &str) -> Option<Session>;
    
    /// Lists all active sessions currently within the storage bounds.
    async fn list(&self) -> Vec<Session>;
    
    /// Prunes stale session data according to the backend's policy.
    async fn evict(&self);
}

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use tracing::info;

/// Optimized in-memory implementation of the SessionStore.
#[derive(Debug)]
pub struct InMemorySessionStore {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    max_age_us: u64,
}

impl InMemorySessionStore {
    pub fn new(max_age_seconds: u64) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            max_age_us: max_age_seconds * 1_000_000,
        }
    }
}

#[async_trait]
impl SessionStore for InMemorySessionStore {
    async fn insert_or_update(&self, session_id: String, event: Event) {
        let mut map = self.sessions.write().expect("RwLock poisoned");
        let session = map.entry(session_id.clone()).or_insert_with(|| Session::new(session_id));
        session.add_event(event);
    }

    async fn get(&self, id: &str) -> Option<Session> {
        let map = self.sessions.read().expect("RwLock poisoned");
        map.get(id).cloned()
    }

    async fn list(&self) -> Vec<Session> {
        let map = self.sessions.read().expect("RwLock poisoned");
        map.values().cloned().collect()
    }

    async fn evict(&self) {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_micros() as u64;
        let mut map = self.sessions.write().expect("RwLock poisoned");
        let initial_size = map.len();
        map.retain(|_, session| (now - session.last_seen_us) < self.max_age_us);
        if map.len() < initial_size {
            info!(count = initial_size - map.len(), "InMemorySessionStore: Evicted stale sessions.");
        }
    }
}
