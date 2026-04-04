use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Capture failed: {0}")]
    Capture(String),

    #[error("Processing failed: {0}")]
    Processing(String),

    #[error("Transport failed: {0}")]
    Transport(String),

    #[error("Protocol logic error ('{0}'): {1}")]
    Protocol(String, String),

    #[error("Configuration fault: {0}")]
    Config(String),

    #[error("Batch processing error: {0}")]
    Batch(String),

    #[error("Internal I/O failure: {0}")]
    Io(#[from] std::io::Error),

    #[error("Network runtime failure: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Internal structural error: {0}")]
    Internal(String),

    #[error("Unexpected event loop error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
