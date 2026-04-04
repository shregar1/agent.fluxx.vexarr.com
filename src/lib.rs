//! Fluxx Agent: High-performance network agent.

pub mod config;
pub mod core;
pub mod protocols;
pub mod replay;
pub mod sessions;
pub mod storage;
pub mod transport;
pub mod processing;
pub mod models;
pub mod cli;
pub mod utils;

// Re-exports
pub use core::Agent;
pub use core::traits::{Capturer, Processor, Transporter, Protocol};
pub use models::event::{Event, ProtocolType};
pub use config::Config;
pub use core::runtime::{AgentService, Lifecycle};
