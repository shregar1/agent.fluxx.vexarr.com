use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Capture failed: {0}")]
    Capture(String),

    #[error("Processing failed: {0}")]
    Processing(String),

    #[error("Transport failed: {0}")]
    Transport(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Unexpected error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
