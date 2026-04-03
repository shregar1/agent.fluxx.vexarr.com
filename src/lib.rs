//! Fluxx Agent: High-performance network agent.

pub mod config;
pub mod core;
pub mod protocols;
pub mod transport;
pub mod utils;

// Re-exports
pub use core::Engine;
pub use core::traits::{Capturer, Event, Processor, Transporter};
pub use config::Config;
