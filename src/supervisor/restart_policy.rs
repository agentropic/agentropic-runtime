use serde::{Deserialize, Serialize};

/// Restart strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestartStrategy {
    /// Never restart
    Never,
    /// Always restart
    Always,
    /// Restart on failure
    OnFailure,
    /// Restart with exponential backoff
    ExponentialBackoff,
}

/// Restart policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestartPolicy {
    strategy: RestartStrategy,
    max_retries: Option<u32>,
    backoff_seconds: u64,
}

impl RestartPolicy {
    /// Create a new restart policy
    pub fn new(strategy: RestartStrategy) -> Self {
        Self {
            strategy,
            max_retries: None,
            backoff_seconds: 1,
        }
    }

    /// Set max retries
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    /// Set backoff duration
    pub fn with_backoff_seconds(mut self, seconds: u64) -> Self {
        self.backoff_seconds = seconds;
        self
    }

    /// Get strategy
    pub fn strategy(&self) -> RestartStrategy {
        self.strategy
    }

    /// Get max retries
    pub fn max_retries(&self) -> Option<u32> {
        self.max_retries
    }

    /// Get backoff seconds
    pub fn backoff_seconds(&self) -> u64 {
        self.backoff_seconds
    }
}

impl Default for RestartPolicy {
    fn default() -> Self {
        Self::new(RestartStrategy::OnFailure)
    }
}
