use anyhow::{Context, Result};
use tracing::{info, error};

mod config;
mod core;
mod protocols;
mod transport;
mod utils;

use crate::config::Config;
use crate::utils::init_logging;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging using our utils
    init_logging();

    info!("Starting fluxx-agent...");

    // Load configuration
    match Config::load() {
        Ok(cfg) => {
            info!("Configuration loaded successfully for agent: {}", cfg.agent_name);
            info!("Agent listening on {}:{}", cfg.server.host, cfg.server.port);
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            // Fall back to default config if necessary
            info!("Falling back to default configuration...");
        }
    }

    // TODO: Initialize core engine and protocols
    info!("Initializing network protocols and core engine...");

    // Placeholder for actual agent loop
    info!("Fluxx agent is now operational.");

    // Simple async sleep to simulate execution
    tokio::signal::ctrl_c().await.context("Failed to wait for Ctrl+C")?;
    
    info!("Shutting down fluxx-agent...");
    Ok(())
}
