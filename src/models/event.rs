use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use bytes::Bytes;

/// Common metadata fields for all Fluxx events.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Metadata {
    pub source_ip: Option<String>,
    pub destination_ip: Option<String>,
    pub request_id: Option<String>,
    pub user_agent: Option<String>,
    
    /// Any protocol-specific extra metadata.
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.extra.insert(key.to_string(), value.to_string());
    }
}

/// Strong typed protocol identifiers for all Fluxx network traffic.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProtocolType {
    Http,
    Ws,
    Tcp,
    Udp,
    Grpc,
    Quic,
    Ebpf,
    Mqtt,
    Modbus,
    Other,
}

/// The centralized, high-performance event definition for Fluxx.
/// Using 'Bytes' for payloads to enable zero-copy sharing between transporters.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: String,
    pub session_id: Option<String>, 
    pub project_id: Option<String>,
    pub environment_id: Option<String>,
    pub service_id: Option<String>,
    pub timestamp_us: u64,
    pub protocol: ProtocolType,
    pub latency_us: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Bytes>, // Optimized for zero-copy
    pub metadata: Metadata,
}

impl Event {
    pub fn new(id: String, protocol: ProtocolType) -> Self {
        Self {
            id,
            session_id: None,
            timestamp_us: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_micros() as u64,
            protocol,
            latency_us: 0,
            payload: None,
            metadata: Metadata::new(),
        }
    }

    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_payload(mut self, data: impl Into<Bytes>) -> Self {
        self.payload = Some(data.into());
        self
    }
}
