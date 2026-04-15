pub mod server;
pub mod response;
pub mod logs;
pub mod metrics;
pub mod service_map;
pub mod dashboard;
pub mod annotations;
pub mod promql;
pub mod cluster;

use crate::sessions::SessionManager;
use crate::storage::event_store::EventStore;
use crate::storage::metric_store::MetricStore;
use crate::storage::log_store::LogStore;
use crate::models::event::Event;
use crate::models::metric::Metric;
use crate::models::log_entry::LogEntry;
use crate::api::annotations::AnnotationStore;
use crate::analysis::error_tracking::ErrorTracker;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct ApiState {
    pub session_manager: Arc<SessionManager>,
    pub event_store: Arc<dyn EventStore>,
    pub event_bus: broadcast::Sender<Arc<Event>>,
    pub metric_store: Arc<dyn MetricStore>,
    pub metric_bus: broadcast::Sender<Arc<Metric>>,
    pub log_store: Arc<dyn LogStore>,
    pub log_bus: broadcast::Sender<Arc<LogEntry>>,
    pub annotation_store: AnnotationStore,
    pub error_tracker: Arc<ErrorTracker>,
}

pub use server::ApiServer;
pub use response::StandardResponse;
pub use crate::analysis::insights::Insight;
