use std::time::Duration;

/// Round robin scheduler
pub struct RoundRobinScheduler {
    time_slice: Duration,
    current_index: usize,
}

impl RoundRobinScheduler {
    /// Create a new round robin scheduler
    pub fn new(time_slice: Duration) -> Self {
        Self {
            time_slice,
            current_index: 0,
        }
    }

    /// Get time slice
    pub fn time_slice(&self) -> Duration {
        self.time_slice
    }

    /// Get current index
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Advance to next
    pub fn next(&mut self) {
        self.current_index += 1;
    }

    /// Reset index
    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}

impl Default for RoundRobinScheduler {
    fn default() -> Self {
        Self::new(Duration::from_millis(100))
    }
}
