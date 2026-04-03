use anyhow::Result;
use fluxx_agent::Agent;
use fluxx_agent::protocols::http::HttpCapturer;
use fluxx_agent::transport::stdout::StdoutTransporter;
use fluxx_agent::config::Config;
use fluxx_agent::utils::init_logging;
use std::sync::Arc;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging using our utils
    init_logging();

    info!("Starting fluxx-agent daemon...");

    // Load configuration
    let config = Config::load().unwrap_or_default();
    info!("Configuration loaded for agent: {}", config.agent_name);

    // Initialize Agent core
    let mut agent = Agent::new();

    // Setup modular components (Dependency Injection)
    let github_zen = Arc::new(HttpCapturer::new("https://api.github.com/zen", "GithubZen"));
    agent.add_capturer(github_zen);

    let stdout_transport = Arc::new(StdoutTransporter);
    agent.add_transporter(stdout_transport);

    // Orchestrate startup and graceful shutdown
    let agent_handle = Arc::new(agent);
    let agent_run = agent_handle.clone();

    tokio::select! {
        res = agent_run.start() => {
            if let Err(e) = res {
                error!("Agent runtime error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Ctrl+C received! Initiating graceful shutdown...");
            agent_handle.shutdown();
        }
    }

    info!("Fluxx agent shutdown complete. Goodbye!");
    Ok(())
}
