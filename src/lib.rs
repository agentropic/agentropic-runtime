//! Runtime, execution engine, scheduling, and supervision for agents.

//#![warn(missing_docs)]
#![allow(missing_docs)]

pub mod config;
pub mod error;
pub mod executor;
pub mod handle;
pub mod isolation;
pub mod metrics;
pub mod runtime;
pub mod scheduler;
pub mod supervisor;
pub mod tracing;

/// Prelude for convenient imports
pub mod prelude;

// Re-exports
pub use config::RuntimeConfig;
pub use error::RuntimeError;
pub use handle::RuntimeHandle;
pub use runtime::Runtime;
