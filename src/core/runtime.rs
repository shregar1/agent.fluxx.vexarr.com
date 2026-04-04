use crate::core::Agent;
use crate::config::Config;
use crate::protocols::http::HttpCapturer;
use crate::protocols::websocket::WebSocketCapturer;
use crate::protocols::quic::QuicCapturer;
use crate::protocols::grpc::GrpcCapturer;
use crate::protocols::ebpf::EbpfCapturer;
use crate::processing::{TagProcessor, LatencyProcessor, LatencyBucketProcessor, FilterProcessor};
use crate::sessions::{SessionProcessor, InMemorySessionStore, SessionManager};
use crate::storage::{StorageProcessor, InMemoryEventStore, ReplayEngine};
use crate::transport::{StdoutTransporter, HttpTransport, ManagedTransporter};
use crate::api::{ApiServer, ApiState};
use crate::models::event::Event;
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, error, debug};
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

/// Managed Lifecycle for the agent daemon.
#[derive(Debug, Clone, Default)]
pub struct Lifecycle {
    token: CancellationToken,
}

impl Lifecycle {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn wait_for_shutdown(&self) {
        self.token.cancelled().await;
        info!("Runtime: Pipeline context closed.");
    }

    pub fn shutdown(&self) {
        info!("Runtime: Signaling global shutdown.");
        self.token.cancel();
    }

    pub fn token(&self) -> CancellationToken {
        self.token.clone()
    }
}

/// AgentService: Orchestrator for setting up the Fluxx agent and its UI API layer.
pub struct AgentService {
    agent: Agent,
    event_bus: broadcast::Sender<Arc<Event>>,
}

impl AgentService {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self {
            agent: Agent::new(),
            event_bus: tx,
        }
    }

    pub async fn run_agent(&mut self, config: Config) -> anyhow::Result<()> {
        info!("Runtime: Configuring Fluxx agent '{}'", config.agent_name);
        
        let session_store = Arc::new(InMemorySessionStore::new(600)); 
        let event_store = Arc::new(InMemoryEventStore::new());
        let session_manager = Arc::new(SessionManager::new(session_store.clone()));

        // 1. Configure UI API API Stack
        let api_state = Arc::new(ApiState {
            session_manager,
            event_store: event_store.clone(),
            event_bus: self.event_bus.clone(),
        });

        let api_server = ApiServer::new(api_state, "0.0.0.0:3000");

        // 2. Configure Ingestion (Capture Layer)
        self.agent.add_capturer(Arc::new(HttpCapturer::new("https://api.github.com/zen", "GithubHealth")));
        self.agent.add_capturer(Arc::new(WebSocketCapturer::new("WsInbound", "0.0.0.0:8080")));
        self.agent.add_capturer(Arc::new(QuicCapturer::new("QuicFabric", "0.0.0.0:4433")));
        self.agent.add_capturer(Arc::new(GrpcCapturer::new("GrpcTrace", "0.0.0.0:50051")));
        self.agent.add_capturer(Arc::new(EbpfCapturer::new("eth0")));

        // 3. Configure Pipeline (In-Memory Processing)
        self.agent.add_processor(Arc::new(StorageProcessor::new(event_store)));
        self.agent.add_processor(Arc::new(SessionProcessor::new(session_store)));
        self.agent.add_processor(Arc::new(LatencyProcessor::new()));
        self.agent.add_processor(Arc::new(LatencyBucketProcessor::default_buckets()));
        self.agent.add_processor(Arc::new(TagProcessor::new(HashMap::from([
            ("env".to_string(), "production".to_string()),
            ("agent".to_string(), config.agent_name.clone()),
        ]))));

        // 4. Configure Transport Sinks
        self.agent.add_transporter(Arc::new(StdoutTransporter));
        
        let upstream_url = format!("http://{}:{}/events", config.server.host, config.server.port);
        let http_net = Arc::new(HttpTransport::new(&upstream_url));
        let managed_http = Arc::new(ManagedTransporter::new(http_net));
        self.agent.add_transporter(managed_http.clone());
        self.agent.add_protocol(managed_http);

        // 5. Fire up the high-performance UI API server
        tokio::spawn(async move {
            if let Err(e) = api_server.start().await {
                error!("Runtime: API Server failure: {}", e);
            }
        });

        // 6. Launch Orchestration
        info!("Runtime: Starting core agent...");
        let agent_ref = Arc::new(std::mem::replace(&mut self.agent, Agent::new()));
        
        tokio::select! {
            res = agent_ref.start() => res?,
            _ = tokio::signal::ctrl_c() => agent_ref.shutdown(),
        }

        Ok(())
    }

    pub fn inspect_target(&self, target: &str) -> String {
        format!("Inspection target '{}' has healthy operational status.", target)
    }

    pub fn get_config_summary(&self, config: &Config) -> String {
        format!("Agent identity: '{}', UI server port: 3000", config.agent_name)
    }
}
