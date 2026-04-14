//! Fluxx Agent: High-performance network agent.

pub mod config;
pub mod core;
pub mod protocols;
pub mod plugins;
pub mod security;
pub mod graph;
pub mod analysis;
pub mod alerts;
pub mod explain;
pub mod comparison;
pub mod api;
pub mod replay;
pub mod sessions;
pub mod storage;
pub mod transport;
pub mod processing;
pub mod models;
pub mod cli;
pub mod utils;

// High-performance optimizations module
pub mod perf;

// Distributed mesh networking
pub mod mesh;

// AI-powered diagnostic and remediation
pub mod diagnostic;

// AI assistant integrations
pub mod ai;

// Synthetic monitoring (HTTP checks, API tests, SSL)
pub mod synthetic;

// 500+ integrations framework
pub mod integrations;

// Re-exports
pub use core::Agent;
pub use core::traits::{Capturer, Processor, Transporter, Protocol};
pub use models::event::{Event, ProtocolType};
pub use config::Config;
pub use core::runtime::{AgentService, Lifecycle};
