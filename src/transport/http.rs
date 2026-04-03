use async_trait::async_trait;
use crate::core::error::{Error, Result};
use crate::core::event::Event;
use crate::core::traits::Transporter;
use tracing::{debug, info};
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
impl Transporter for HttpTransport {
    async fn send(&self, event: Event) -> Result<()> {
        debug!("HTTP Transport: Sending event {} to {}", event.id, self.url);
        
        let response = self.client.post(&self.url)
            .json(&event)
            .send()
            .await
            .map_err(|e| Error::Transport(format!("HTTP delivery failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Transport(format!(
                "Remote server returned error: {}", 
                response.status()
            )));
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "HttpTransport"
    }
}
