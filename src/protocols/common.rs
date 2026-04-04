use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::event::Event;
use crate::core::error::{Result, Error};
use async_trait::async_trait;
use crate::core::traits::Capturer;

/// Shared component for protocols that use internal buffering (MPSC)
/// to bridge between async collectors and the synchronous pipeline demand.
#[derive(Debug)]
pub struct CaptureChannel {
    pub name: String,
    tx: mpsc::Sender<Event>,
    rx: Arc<Mutex<mpsc::Receiver<Event>>>,
}

impl CaptureChannel {
    pub fn new(name: &str, capacity: usize) -> Self {
        let (tx, rx) = mpsc::channel(capacity);
        Self {
            name: name.to_string(),
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }

    pub fn sender(&self) -> mpsc::Sender<Event> {
        self.tx.clone()
    }

    pub async fn recv(&self) -> Result<Event> {
        let mut rx = self.rx.lock().await;
        rx.recv().await.ok_or_else(|| Error::Capture(format!("Channel for {} closed", self.name)))
    }
}

/// Generic implementation allows protocols to easily adhere to the Capturer trait.
#[async_trait]
impl Capturer for CaptureChannel {
    async fn capture(&self) -> Result<Event> {
        self.recv().await
    }

    fn name(&self) -> &str {
        &self.name
    }
}
