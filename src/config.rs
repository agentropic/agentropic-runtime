use serde::{Deserialize, Serialize};

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Maximum number of worker threads
    pub max_workers: usize,

    /// Enable metrics collection
    pub enable_metrics: bool,

    /// Enable tracing
    pub enable_tracing: bool,

    /// Default timeout in milliseconds
    pub default_timeout_ms: u64,
}

impl RuntimeConfig {
    /// Create a new runtime configuration
    pub fn new() -> Self {
        Self {
            max_workers: num_cpus::get(),
            enable_metrics: true,
            enable_tracing: true,
            default_timeout_ms: 30000,
        }
    }

    /// Set maximum workers
    pub fn with_max_workers(mut self, workers: usize) -> Self {
        self.max_workers = workers;
        self
    }

    /// Enable/disable metrics
    pub fn with_metrics(mut self, enable: bool) -> Self {
        self.enable_metrics = enable;
        self
    }

    /// Enable/disable tracing
    pub fn with_tracing(mut self, enable: bool) -> Self {
        self.enable_tracing = enable;
        self
    }

    /// Set default timeout
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.default_timeout_ms = timeout_ms;
        self
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self::new()
    }
}
