pub mod server;

use crate::sessions::SessionManager;
use crate::storage::event_store::EventStore;
use crate::models::event::Event;
use std::sync::Arc;
use tokio::sync::broadcast;

/// ApiState: Shared high-performance state for the Axum REST and WebSocket endpoints.
pub struct ApiState {
    pub session_manager: Arc<SessionManager>,
    pub event_store: Arc<dyn EventStore>,
    pub event_bus: broadcast::Sender<Arc<Event>>, // Live fan-out bus for WebSockets
}

pub use server::ApiServer;
