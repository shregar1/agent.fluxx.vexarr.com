use crate::core::Agent;
use crate::config::Config;
use crate::protocols::http::HttpCapturer;
use crate::protocols::quic::QuicCapturer;
use crate::protocols::grpc::GrpcCapturer;
use crate::protocols::ebpf::EbpfCapturer;
use crate::processing::{TagProcessor, LatencyProcessor, LatencyBucketProcessor, FilterProcessor};
use crate::transport::{StdoutTransporter, HttpTransport, ManagedTransporter};
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, error};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Lifecycle {
    token: CancellationToken,
}

impl Lifecycle {
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
        }
    }

    pub async fn wait_for_shutdown(&self) {
        self.token.cancelled().await;
        info!("Runtime: Shutdown finalized.");
    }

    pub fn shutdown(&self) {
        info!("Runtime: Initiating shutdown...");
        self.token.cancel();
    }

    pub fn token(&self) -> CancellationToken {
        self.token.clone()
    }
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::new()
    }
}

/// AgentService: High-level business logic for orchestrating the Fluxx agent.
pub struct AgentService {
    agent: Agent,
}

impl AgentService {
    pub fn new() -> Self {
        Self {
            agent: Agent::new(),
        }
    }

    pub async fn run_agent(&mut self, config: Config) -> anyhow::Result<()> {
        info!("Runtime: Configuring agent '{}'", config.agent_name);
        
        let github_zen = Arc::new(HttpCapturer::new("https://api.github.com/zen", "GithubZen"));
        self.agent.add_capturer(github_zen);
        self.agent.add_capturer(Arc::new(QuicCapturer::new("QuicPrimary", "0.0.0.0:4433")));
        self.agent.add_capturer(Arc::new(GrpcCapturer::new("GrpcInterseptor", "0.0.0.0:50051")));
        self.agent.add_capturer(Arc::new(EbpfCapturer::new("eth0")));

        self.agent.add_processor(Arc::new(TagProcessor::new(HashMap::from([
            ("env".to_string(), "production".to_string()),
            ("agent".to_string(), config.agent_name.clone()),
        ]))));
        
        self.agent.add_processor(Arc::new(LatencyProcessor::new()));
        self.agent.add_processor(Arc::new(LatencyBucketProcessor::default_buckets()));
        
        self.agent.add_processor(Arc::new(FilterProcessor::new("PayloadFilter", |e| {
            e.payload.is_some() || e.protocol == crate::models::event::ProtocolType::Http
        })));

        self.agent.add_transporter(Arc::new(StdoutTransporter));

        let http_net = Arc::new(HttpTransport::new(&format!("http://{}:{}/events", config.server.host, config.server.port)));
        let managed_http = Arc::new(ManagedTransporter::new(http_net));
        self.agent.add_transporter(managed_http.clone());
        self.agent.add_protocol(managed_http);

        info!("Runtime: Starting agent pipeline...");
        let agent_handle = Arc::new(std::mem::replace(&mut self.agent, Agent::new()));
        let agent_run = agent_handle.clone();

        tokio::select! {
            res = agent_run.start() => {
                if let Err(e) = res {
                    error!("Runtime: Agent failure: {:?}", e);
                    return Err(e.into());
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Runtime: Shutdown signal received.");
                agent_handle.shutdown();
            }
        }

        Ok(())
    }

    pub fn inspect_target(&self, target: &str) -> String {
        format!("Target '{}' is currently monitored and active.", target)
    }

    pub fn get_config_summary(&self, config: &Config) -> String {
        format!("{:#?}", config)
    }
}
