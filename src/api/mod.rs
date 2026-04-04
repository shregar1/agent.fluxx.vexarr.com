pub mod server;
pub mod response;

use crate::sessions::SessionManager;
use crate::storage::event_store::EventStore;
use crate::models::event::Event;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct ApiState {
    pub session_manager: Arc<SessionManager>,
    pub event_store: Arc<dyn EventStore>,
    pub event_bus: broadcast::Sender<Arc<Event>>, 
}

pub use server::ApiServer;
pub use response::StandardResponse;
pub use crate::analysis::insights::Insight;
