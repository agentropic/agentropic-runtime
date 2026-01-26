use std::time::Duration;

/// Exponential backoff
pub struct ExponentialBackoff {
    current_delay: Duration,
    max_delay: Duration,
    multiplier: f64,
    retries: u32,
}

impl ExponentialBackoff {
    /// Create a new exponential backoff
    pub fn new(initial_delay: Duration, max_delay: Duration) -> Self {
        Self {
            current_delay: initial_delay,
            max_delay,
            multiplier: 2.0,
            retries: 0,
        }
    }

    /// Get next delay
    pub fn next_delay(&mut self) -> Duration {
        let delay = self.current_delay;
        self.current_delay = Duration::from_secs_f64(
            (self.current_delay.as_secs_f64() * self.multiplier).min(self.max_delay.as_secs_f64()),
        );
        self.retries += 1;
        delay
    }

    /// Reset backoff
    pub fn reset(&mut self) {
        self.current_delay = Duration::from_secs(1);
        self.retries = 0;
    }

    /// Get retry count
    pub fn retries(&self) -> u32 {
        self.retries
    }
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self::new(Duration::from_secs(1), Duration::from_secs(60))
    }
}
