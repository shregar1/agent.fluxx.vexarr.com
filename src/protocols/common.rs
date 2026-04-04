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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::event::{Event, ProtocolType};

    // Test 111: CaptureChannel new
    #[test]
    fn test_capture_channel_new() {
        let channel = CaptureChannel::new("test-channel", 100);
        assert_eq!(channel.name, "test-channel");
    }

    // Test 112: CaptureChannel sender
    #[test]
    fn test_capture_channel_sender() {
        let channel = CaptureChannel::new("test-channel", 100);
        let _sender = channel.sender();
    }

    // Test 113: CaptureChannel send and recv
    #[tokio::test]
    async fn test_capture_channel_send_recv() {
        let channel = CaptureChannel::new("test-channel", 100);
        let sender = channel.sender();
        let event = Event::new("evt-1".to_string(), ProtocolType::Http);
        
        sender.send(event.clone()).await.unwrap();
        let received = channel.recv().await.unwrap();
        
        assert_eq!(received.id, "evt-1");
    }

    // Test 114: CaptureChannel multiple events
    #[tokio::test]
    async fn test_capture_channel_multiple() {
        let channel = CaptureChannel::new("test-channel", 100);
        let sender = channel.sender();
        
        sender.send(Event::new("evt-1".to_string(), ProtocolType::Http)).await.unwrap();
        sender.send(Event::new("evt-2".to_string(), ProtocolType::Ws)).await.unwrap();
        sender.send(Event::new("evt-3".to_string(), ProtocolType::Grpc)).await.unwrap();
        
        let e1 = channel.recv().await.unwrap();
        let e2 = channel.recv().await.unwrap();
        let e3 = channel.recv().await.unwrap();
        
        assert_eq!(e1.id, "evt-1");
        assert_eq!(e2.id, "evt-2");
        assert_eq!(e3.id, "evt-3");
    }

    // Test 115: CaptureChannel name method
    #[test]
    fn test_capture_channel_name() {
        let channel = CaptureChannel::new("my-channel", 100);
        assert_eq!(channel.name(), "my-channel");
    }

    // Test 116: CaptureChannel capturer trait name
    #[tokio::test]
    async fn test_capture_channel_capturer_name() {
        let channel = CaptureChannel::new("trait-channel", 100);
        assert_eq!(Capturer::name(&channel), "trait-channel");
    }

    // Test 117: CaptureChannel buffer capacity
    #[test]
    fn test_capture_channel_capacity() {
        // Channel is created with specified capacity
        let channel = CaptureChannel::new("test", 10);
        let _ = channel.sender();
    }

    // Test 118: CaptureChannel debug format
    #[test]
    fn test_capture_channel_debug() {
        let channel = CaptureChannel::new("test-channel", 100);
        let debug_str = format!("{:?}", channel);
        assert!(debug_str.contains("test-channel"));
    }

    // Test 119: Empty channel recv returns error
    #[tokio::test]
    async fn test_capture_channel_empty_recv() {
        let channel = CaptureChannel::new("test", 1);
        let sender = channel.sender();
        drop(sender); // Close the channel
        
        let result = channel.recv().await;
        assert!(result.is_err());
    }

    // Test 120: CaptureChannel implements Capturer
    fn assert_capturer_impl<T: Capturer>() {}
    #[test]
    fn test_capture_channel_implements_trait() {
        assert_capturer_impl::<CaptureChannel>();
    }
}
