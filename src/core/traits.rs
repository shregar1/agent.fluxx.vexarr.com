use async_trait::async_trait;
use crate::core::error::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: String,
    pub timestamp: u64,
    pub metadata: std::collections::HashMap<String, String>,
    pub payload: Vec<u8>,
}

#[async_trait]
pub trait Capturer: Send + Sync + Debug {
    async fn capture(&self) -> Result<Event>;
    fn name(&self) -> &str;
}

#[async_trait]
pub trait Processor: Send + Sync + Debug {
    async fn process(&self, event: Event) -> Result<Option<Event>>;
    fn name(&self) -> &str;
}

#[async_trait]
pub trait Transporter: Send + Sync + Debug {
    async fn send(&self, event: Event) -> Result<()>;
    fn name(&self) -> &str;
}
