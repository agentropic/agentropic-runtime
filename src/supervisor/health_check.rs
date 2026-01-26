use std::time::Instant;

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}

/// Health check
pub struct HealthCheck {
    status: HealthStatus,
    last_check: Option<Instant>,
    failures: u32,
}

impl HealthCheck {
    /// Create a new health check
    pub fn new() -> Self {
        Self {
            status: HealthStatus::Unknown,
            last_check: None,
            failures: 0,
        }
    }

    /// Record healthy check
    pub fn record_healthy(&mut self) {
        self.status = HealthStatus::Healthy;
        self.last_check = Some(Instant::now());
        self.failures = 0;
    }

    /// Record unhealthy check
    pub fn record_unhealthy(&mut self) {
        self.status = HealthStatus::Unhealthy;
        self.last_check = Some(Instant::now());
        self.failures += 1;
    }

    /// Get status
    pub fn status(&self) -> HealthStatus {
        self.status
    }

    /// Get failure count
    pub fn failures(&self) -> u32 {
        self.failures
    }

    /// Check if healthy
    pub fn is_healthy(&self) -> bool {
        self.status == HealthStatus::Healthy
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self::new()
    }
}
