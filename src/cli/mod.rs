pub mod dev;

use clap::{Parser, Subcommand};
use crate::config::Config;
use crate::utils::init_logging;
use crate::core::AgentService;
use crate::cli::dev::DevManager;
use tracing::info;

/// CLI interface: Parser and Entry Point for the Fluxx agent.
#[derive(Parser)]
#[command(name = "fluxx")]
#[command(version, about = "Fluxx: High-performance observability for the edge.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Dev mode: Automates local service discovery and live telemetry streaming
    Dev,
    
    /// Initialises and runs the agent service
    Start {
        /// Optional path to any configuration file
        #[arg(short, long)]
        config: Option<String>,
    },
    
    /// Inspects the current state of any specified target
    Inspect {
        /// The identifier for the target to inspect
        target: String,
    },

    /// Returns the current configuration summary
    Config,
}

pub async fn run() -> anyhow::Result<()> {
    init_logging();
    let cli = Cli::parse();
    let mut service = AgentService::new();

    match cli.command {
        Commands::Dev => {
            let mut dev = DevManager::new();
            dev.run().await?;
        }
        Commands::Start { config } => {
            let mut service = AgentService::new();
            info!("CLI: Initializing Start flow...");
            let cfg = match config {
                Some(path) => Config::load_from_path(&path).unwrap_or_default(),
                None => Config::load().unwrap_or_default(),
            };
            service.run_agent(cfg).await?;
        }
        Commands::Inspect { target } => {
            let service = AgentService::new();
            let res = service.inspect_target(&target);
            println!("{}", res);
        }
        Commands::Config => {
            let service = AgentService::new();
            let cfg = Config::load().unwrap_or_default();
            let res = service.get_config_summary(&cfg);
            println!("{}", res);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing_start() {
        let cli = Cli::try_parse_from(&["fluxx", "start", "--config", "env.json"]).unwrap();
        match cli.command {
            Commands::Start { config } => assert_eq!(config, Some("env.json".to_string())),
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_cli_parsing_inspect() {
        let cli = Cli::try_parse_from(&["fluxx", "inspect", "http"]).unwrap();
        match cli.command {
            Commands::Inspect { target } => assert_eq!(target, "http"),
            _ => panic!("Expected Inspect command"),
        }
    }

    #[test]
    fn test_cli_parsing_config() {
        let cli = Cli::try_parse_from(&["fluxx", "config"]).unwrap();
        match cli.command {
            Commands::Config => {},
            _ => panic!("Expected Config command"),
        }
    }
}
