//! Library documentation for agent-fluxx-dev.

pub mod config;
pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

pub use config::Config;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
