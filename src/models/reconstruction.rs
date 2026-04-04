use crate::models::event::Event;
use crate::models::event::ProtocolType;

/// Unified logic for reconstructing protocols from events to avoid logic duplication
/// across capture and replay modules.
pub struct EventReconstruction;

impl EventReconstruction {
    /// Maps generic Fluxx events to human-readable protocol descriptions.
    pub fn describe(event: &Event) -> String {
        match event.protocol {
            ProtocolType::Http => format!("HTTP POST ({} bytes)", event.payload.as_ref().map(|p| p.len()).unwrap_or(0)),
            ProtocolType::Ws => format!("WebSocket Message ({} bytes)", event.payload.as_ref().map(|p| p.len()).unwrap_or(0)),
            ProtocolType::Tcp => format!("TCP Segment from {}", event.metadata.source_ip.as_deref().unwrap_or("unknown")),
            ProtocolType::Udp => format!("UDP Datagram from {}", event.metadata.source_ip.as_deref().unwrap_or("unknown")),
            _ => format!("Protocol {:?} event", event.protocol),
        }
    }

    /// Future: Reconstruct full multi-event HTTP streams or WebSocket sessions here.
}
