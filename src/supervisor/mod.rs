//! Supervision and fault tolerance

/// Exponential backoff
pub mod backoff;
/// Circuit breaker
pub mod circuit_breaker;
/// Supervisor engine
pub mod engine;
/// Health checks
pub mod health_check;
/// Restart policies
pub mod restart_policy;

pub use backoff::ExponentialBackoff;
pub use circuit_breaker::{CircuitBreaker, CircuitState};
pub use engine::Supervisor;
pub use health_check::{HealthCheck, HealthStatus};
pub use restart_policy::{RestartPolicy, RestartStrategy};
