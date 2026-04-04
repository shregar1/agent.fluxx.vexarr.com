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

#[cfg(test)]
mod tests {
    use super::*;

    // Test 71: Config default values
    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.agent_name, "fluxx-agent-01");
        assert_eq!(config.log_level, "info");
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
    }

    // Test 72: ServerConfig default
    #[test]
    fn test_server_config_default() {
        let server: ServerConfig = Config::default().server;
        assert_eq!(server.host, "127.0.0.1");
        assert_eq!(server.port, 8080);
    }

    // Test 73: Config clone
    #[test]
    fn test_config_clone() {
        let config = Config::default();
        let cloned = config.clone();
        assert_eq!(config.agent_name, cloned.agent_name);
        assert_eq!(config.log_level, cloned.log_level);
    }

    // Test 74: Config serialization
    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("fluxx-agent-01"));
        assert!(json.contains("8080"));
    }

    // Test 75: ServerConfig serialization
    #[test]
    fn test_server_config_serialization() {
        let server = ServerConfig { host: "0.0.0.0".to_string(), port: 3000 };
        let json = serde_json::to_string(&server).unwrap();
        assert!(json.contains("0.0.0.0"));
        assert!(json.contains("3000"));
    }

    // Test 76: Config deserialization
    #[test]
    fn test_config_deserialization() {
        let json = r#"{"agent_name":"test-agent","log_level":"debug","server":{"host":"0.0.0.0","port":3000}}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.agent_name, "test-agent");
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.server.port, 3000);
    }

    // Test 77: Custom config values
    #[test]
    fn test_custom_config() {
        let config = Config {
            agent_name: "custom-agent".to_string(),
            log_level: "debug".to_string(),
            server: ServerConfig { host: "0.0.0.0".to_string(), port: 9090 },
        };
        assert_eq!(config.agent_name, "custom-agent");
        assert_eq!(config.server.port, 9090);
    }
}
