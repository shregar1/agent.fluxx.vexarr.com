use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub agent_name: String,
    pub log_level: String,
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            agent_name: "fluxx-agent-01".into(),
            log_level: "info".into(),
            server: ServerConfig {
                host: "127.0.0.1".into(),
                port: 8080,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = config::Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(config::File::with_name("config/default").required(false))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                config::File::with_name(&format!("config/{}", run_mode))
                    .required(false),
            )
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        s.try_deserialize().context("Failed to deserialize configuration")
    }

    pub fn load_from_path(path: &str) -> Result<Self> {
        let s = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()
            .map_err(|e| anyhow::anyhow!("Config build failed: {}", e))?;

        s.try_deserialize().context("Failed to deserialize configuration from path")
    }
}
