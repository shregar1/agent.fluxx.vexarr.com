use crate::sessions::store::SessionStore;
use crate::sessions::models::Session;
use crate::models::event::Event;
use crate::core::traits::Processor;
use crate::core::error::Result;
use async_trait::async_trait;
use tracing::debug;
use std::sync::Arc;

/// SessionManager: Protocol-agnostic orchestrator for global session lifecycle.
/// Decoupled from the underlying storage backend through the SessionStore interface.
#[derive(Debug)]
pub struct SessionManager {
    store: Arc<dyn SessionStore>,
}

impl SessionManager {
    pub fn new(store: Arc<dyn SessionStore>) -> Self {
        Self { store }
    }

    /// Fetches a session's history and metrics.
    pub async fn get_session(&self, session_id: &str) -> Option<Session> {
        self.store.get(session_id).await
    }

    /// Lists active monitored sessions.
    pub async fn list_sessions(&self) -> Vec<Session> {
        self.store.list().await
    }

    /// Executes protocol-agnostic lifecycle maintenance.
    pub async fn perform_eviction(&self) {
        self.store.evict().await;
    }
}

/// SessionProcessor: Generic middleware that orchestrates event correlation.
/// Zero protocol-specific logic; relies on Event metadata for session linkage.
#[derive(Debug, Clone)]
pub struct SessionProcessor {
    store: Arc<dyn SessionStore>,
}

impl SessionProcessor {
    pub fn new(store: Arc<dyn SessionStore>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl Processor for SessionProcessor {
    async fn process(&self, mut event: Event) -> Result<Option<Event>> {
        // Core Logic: Extracting the session key (reusable across HTTP, WS, etc.)
        let session_id = event.session_id.clone()
            .or_else(|| event.metadata.extra.get("session_id").cloned())
            .unwrap_or_else(|| "default_session".to_string());

        event.session_id = Some(session_id.clone());

        // Correlation: Persisting via the abstract storage interface
        debug!(id = %event.id, session = %session_id, "SessionProcessor: Correlating event.");
        self.store.insert_or_update(session_id, event.clone()).await;

        Ok(Some(event))
    }

    fn name(&self) -> &str {
        "SessionProcessor"
    }
}
