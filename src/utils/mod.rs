use tracing_subscriber::{fmt, EnvFilter};
use tracing::Level;

pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_max_level(Level::DEBUG) // Explicit max level
        .init();
}
