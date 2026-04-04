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

    /// Provisions and deploys the agent to a specified fleet or cloud provider
    Deploy {
        /// The cloud or infrastructure provider (aws, azure, gcp, local)
        #[arg(short, long)]
        provider: String,

        /// The region for the provider (AWS specific)
        #[arg(short, long)]
        region: Option<String>,

        /// The deployment environment (e.g. production, staging)
        #[arg(short, long)]
        env: Option<String>,

        /// The IP/Host for a local self-hosted hub
        #[arg(long)]
        hub_ip: Option<String>,

        /// The cluster URI for an Azure Data Explorer hub
        #[arg(long)]
        cluster_uri: Option<String>,

        /// The project ID for a GCP deployment
        #[arg(long)]
        project: Option<String>,

        /// The instance ID for a GCP deployment
        #[arg(long)]
        instance: Option<String>,
    },
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
            info!("CLI: Initializing Start flow...");
            let cfg = match config {
                Some(path) => Config::load_from_path(&path).unwrap_or_default(),
                None => Config::load().unwrap_or_default(),
            };
            service.run_agent(cfg).await?;
        }
        Commands::Inspect { target } => {
            let res = service.inspect_target(&target);
            println!("{}", res);
        }
        Commands::Config => {
            let cfg = Config::load().unwrap_or_default();
            let res = service.get_config_summary(&cfg);
            println!("{}", res);
        }
        Commands::Deploy { 
            provider, region, env, hub_ip, cluster_uri, project, instance 
        } => {
            let res = service.deploy_fleet(provider, region, env, hub_ip, cluster_uri, project, instance).await?;
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

    #[test]
    fn test_cli_parsing_deploy() {
        let cli = Cli::try_parse_from(&["fluxx", "deploy", "--provider", "aws", "--region", "us-west-2"]).unwrap();
        match cli.command {
            Commands::Deploy { provider, region, .. } => {
                assert_eq!(provider, "aws");
                assert_eq!(region, Some("us-west-2".to_string()));
            },
            _ => panic!("Expected Deploy command"),
        }
    }
}
