use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub env: String,
    pub debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            env: "development".into(),
            debug: true,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
