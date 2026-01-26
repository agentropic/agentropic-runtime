use serde::{Deserialize, Serialize};

/// Type of scheduling policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyType {
    /// Fair share scheduling
    FairShare,
    /// Priority-based scheduling
    Priority,
    /// Round robin scheduling
    RoundRobin,
    /// First-come-first-served
    FCFS,
}

/// Scheduling policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingPolicy {
    policy_type: PolicyType,
    parameters: Vec<(String, f64)>,
}

impl SchedulingPolicy {
    /// Create a new scheduling policy
    pub fn new(policy_type: PolicyType) -> Self {
        Self {
            policy_type,
            parameters: Vec::new(),
        }
    }

    /// Add parameter
    pub fn with_parameter(mut self, name: impl Into<String>, value: f64) -> Self {
        self.parameters.push((name.into(), value));
        self
    }

    /// Get policy type
    pub fn policy_type(&self) -> PolicyType {
        self.policy_type
    }

    /// Get parameters
    pub fn parameters(&self) -> &[(String, f64)] {
        &self.parameters
    }
}
