pub mod event_store;
pub mod in_memory;
pub mod processor;

pub use event_store::{EventStore, TimeWindow};
pub use in_memory::InMemoryEventStore;
pub use processor::StorageProcessor;
