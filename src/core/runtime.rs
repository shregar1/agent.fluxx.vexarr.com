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
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, error, debug};
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

/// AgentService: High-level orchestrator for setting up the Fluxx agent from a configuration.
/// Integrates all sub-modules (Capture, Processing, Storage, Sessions) into a single runtime.
pub struct AgentService {
    agent: Agent,
    session_manager: Option<Arc<SessionManager>>,
    replay_engine: Option<Arc<ReplayEngine>>,
}

impl AgentService {
    pub fn new() -> Self {
        Self {
            agent: Agent::new(),
            session_manager: None,
            replay_engine: None,
        }
    }

    pub async fn run_agent(&mut self, config: Config) -> anyhow::Result<()> {
        info!("Runtime: Initializing Fluxx agent '{}'", config.agent_name);
        
        // 1. Storage & Session Persistence (Core Foundation)
        let session_store = Arc::new(InMemorySessionStore::new(600)); // 10-minute session TTL
        let event_store = Arc::new(InMemoryEventStore::new());
        
        self.session_manager = Some(Arc::new(SessionManager::new(session_store.clone())));
        self.replay_engine = Some(Arc::new(ReplayEngine::new(event_store.clone())));

        // 2. Configure Ingestion (Capture Layer)
        self.agent.add_capturer(Arc::new(HttpCapturer::new("https://api.github.com/zen", "GithubHealth")));
        self.agent.add_capturer(Arc::new(WebSocketCapturer::new("WsInbound", "0.0.0.0:8080")));
        self.agent.add_capturer(Arc::new(QuicCapturer::new("QuicPrimary", "0.0.0.0:4433")));
        self.agent.add_capturer(Arc::new(GrpcCapturer::new("GrpcInterseptor", "0.0.0.0:50051")));
        self.agent.add_capturer(Arc::new(EbpfCapturer::new("eth0")));

        // 3. Configure Pipeline (Middleware / Processing Layer)
        // Order: Storage & Sessioning -> Latency -> Bucketization -> Delivery Tags
        self.agent.add_processor(Arc::new(StorageProcessor::new(event_store)));
        self.agent.add_processor(Arc::new(SessionProcessor::new(session_store)));
        
        self.agent.add_processor(Arc::new(LatencyProcessor::new()));
        self.agent.add_processor(Arc::new(LatencyBucketProcessor::default_buckets()));
        
        self.agent.add_processor(Arc::new(TagProcessor::new(HashMap::from([
            ("env".to_string(), "production".to_string()),
            ("agent_id".to_string(), config.agent_name.clone()),
        ]))));
        
        self.agent.add_processor(Arc::new(FilterProcessor::new("EventSanitize", |e| e.payload.is_some())));

        // 4. Configure Sinks (Transport Layer)
        // Local feedback loop (Stdout)
        self.agent.add_transporter(Arc::new(StdoutTransporter));

        // Remote upstream delivery with exponential retries
        let upstream_url = format!("http://{}:{}/events", config.server.host, config.server.port);
        let http_net = Arc::new(HttpTransport::new(&upstream_url));
        let managed_http = Arc::new(ManagedTransporter::new(http_net));
        
        self.agent.add_transporter(managed_http.clone());
        self.agent.add_protocol(managed_http); // Register as lifecycle protocol

        // 5. Execution Orchestration
        info!("Runtime: Starting orchestrator daemon.");
        let agent_ref = Arc::new(std::mem::replace(&mut self.agent, Agent::new()));
        let agent_run = agent_ref.clone();

        tokio::select! {
            res = agent_run.start() => {
                if let Err(e) = res {
                    error!(error=%e, "Runtime: Agent daemon exited with error.");
                    return Err(e.into());
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Runtime: Interrupted by user, signaling clean shutdown.");
                agent_ref.shutdown();
            }
        }

        Ok(())
    }

    pub fn inspect_target(&self, target: &str) -> String {
        format!("Inspection target '{}' has active monitoring enabled.", target)
    }

    pub fn get_config_summary(&self, config: &Config) -> String {
        format!("Configured for agent '{}', delivery target: {}:{}", 
            config.agent_name, config.server.host, config.server.port)
    }
}
