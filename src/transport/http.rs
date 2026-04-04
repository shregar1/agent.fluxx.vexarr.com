use async_trait::async_trait;
use crate::core::error::{Error, Result};
use crate::transport::network::NetworkLayer;
use tracing::debug;
use reqwest::Client;

#[derive(Debug)]
pub struct HttpTransport {
    url: String,
    client: Client,
}

impl HttpTransport {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap_or_default(),
        }
    }
}

#[async_trait]
impl NetworkLayer for HttpTransport {
    async fn write(&self, data: Vec<u8>) -> Result<()> {
        debug!("HTTP Delivery: Posting bytes to {}", self.url);
        
        let response = self.client.post(&self.url)
            .body(data) // Raw bytes, serialization handled by ManagedTransporter
            .send()
            .await
            .map_err(|e| Error::Transport(format!("HTTP POST failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Transport(format!(
                "HTTP endpoint error: {}", response.status()
            )));
        }

        Ok(())
    }

    fn protocol_name(&self) -> &str {
        "http"
    }
}
