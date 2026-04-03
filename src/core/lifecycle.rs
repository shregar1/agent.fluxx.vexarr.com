use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Debug, Clone)]
pub struct Lifecycle {
    token: CancellationToken,
}

impl Lifecycle {
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
        }
    }

    pub async fn wait_for_shutdown(&self) {
        self.token.cancelled().await;
        info!("Lifecycle: Shutdown finalized.");
    }

    pub fn shutdown(&self) {
        info!("Lifecycle: Initiating shutdown...");
        self.token.cancel();
    }

    pub fn token(&self) -> CancellationToken {
        self.token.clone()
    }
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::new()
    }
}
