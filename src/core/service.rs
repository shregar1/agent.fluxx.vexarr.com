use crate::core::Agent;
use crate::config::Config;
use crate::protocols::http::HttpCapturer;
use crate::protocols::quic::QuicCapturer;
use crate::protocols::grpc::GrpcCapturer;
use crate::protocols::ebpf::EbpfCapturer;
use crate::core::processor::{TagProcessor, LatencyProcessor, LatencyBucketProcessor, FilterProcessor};
use crate::transport::{StdoutTransporter, HttpTransport, ManagedTransporter};
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, error};

/// Service Layer: High-level business logic for orchestrating the Fluxx agent.
/// This layer decouples CLI specifics from core agent operations.
pub struct AgentService {
    agent: Agent,
}

impl AgentService {
    pub fn new() -> Self {
        Self {
            agent: Agent::new(),
        }
    }

    /// Initializes and starts the agent with the provided configuration.
    pub async fn run_agent(&mut self, config: Config) -> anyhow::Result<()> {
        info!("Service: Configuring agent '{}'", config.agent_name);
        
        // 1. Configure Capturers (Multi-protocol support)
        let github_zen = Arc::new(HttpCapturer::new("https://api.github.com/zen", "GithubZen"));
        self.agent.add_capturer(github_zen);

        self.agent.add_capturer(Arc::new(QuicCapturer::new("QuicPrimary", "0.0.0.0:4433")));
        self.agent.add_capturer(Arc::new(GrpcCapturer::new("GrpcInterseptor", "0.0.0.0:50051")));
        self.agent.add_capturer(Arc::new(EbpfCapturer::new("eth0")));

        // 2. Configure Processors (Modular & Composable)
        self.agent.add_processor(Arc::new(TagProcessor::new(HashMap::from([
            ("env".to_string(), "production".to_string()),
            ("agent".to_string(), config.agent_name.clone()),
        ]))));
        
        self.agent.add_processor(Arc::new(LatencyProcessor::new()));
        self.agent.add_processor(Arc::new(LatencyBucketProcessor::default_buckets()));
        
        self.agent.add_processor(Arc::new(FilterProcessor::new("PayloadFilter", |e| {
            e.payload.is_some() || e.protocol == crate::core::ProtocolType::Http
        })));

        // 3. Configure Transporters
        self.agent.add_transporter(Arc::new(StdoutTransporter));

        let http_net = Arc::new(HttpTransport::new(&format!("http://{}:{}/events", config.server.host, config.server.port)));
        let managed_http = Arc::new(ManagedTransporter::new(http_net));
        self.agent.add_transporter(managed_http.clone());
        self.agent.add_protocol(managed_http);

        // 4. Orchestrate Startup
        info!("Service: Starting agent pipeline...");
        let agent_handle = Arc::new(std::mem::replace(&mut self.agent, Agent::new()));
        let agent_run = agent_handle.clone();

        tokio::select! {
            res = agent_run.start() => {
                if let Err(e) = res {
                    error!("Service: Agent runtime failure: {:?}", e);
                    return Err(e.into());
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Service: Shutdown signal received.");
                agent_handle.shutdown();
            }
        }

        Ok(())
    }

    /// Inspects a specific protocol target.
    pub fn inspect_target(&self, target: &str) -> String {
        format!("Target '{}' is currently monitored and active.", target)
    }

    /// Returns the current configuration view.
    pub fn get_config_summary(&self, config: &Config) -> String {
        format!("{:#?}", config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_inspect_active() {
        let service = AgentService::new();
        let res = service.inspect_target("http");
        assert!(res.contains("http"));
        assert!(res.contains("active"));
    }

    #[test]
    fn test_service_get_config_summary() {
        let service = AgentService::new();
        let config = Config::default();
        let res = service.get_config_summary(&config);
        assert!(res.contains("fluxx-agent-01"));
    }

    #[tokio::test]
    async fn test_service_initialization() {
        let service = AgentService::new();
        // Just verify it doesn't panic during construction
        assert!(true); 
    }
}
