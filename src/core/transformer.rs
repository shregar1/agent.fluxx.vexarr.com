use async_trait::async_trait;
use crate::core::error::Result;
use crate::core::traits::{Event, Processor};
use tracing::debug;

#[derive(Debug)]
pub struct JsonTransformer;

#[async_trait]
impl Processor for JsonTransformer {
    async fn process(&self, mut event: Event) -> Result<Option<Event>> {
        debug!("Processing event {}", event.id);

        // Simple transformation: add a 'processed_at' tag to metadata
        event.metadata.insert(
            "processed_at".to_string(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
        );

        Ok(Some(event))
    }

    fn name(&self) -> &str {
        "JsonTransformer"
    }
}
