use super::{HealthCheck, RestartPolicy};
use agentropic_core::AgentId;
use std::collections::HashMap;

/// Supervisor for agent fault tolerance
pub struct Supervisor {
    name: String,
    policies: HashMap<AgentId, RestartPolicy>,
    health_checks: HashMap<AgentId, HealthCheck>,
}

impl Supervisor {
    /// Create a new supervisor
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            policies: HashMap::new(),
            health_checks: HashMap::new(),
        }
    }

    /// Add agent to supervision
    pub fn supervise(&mut self, agent_id: AgentId, policy: RestartPolicy) {
        self.policies.insert(agent_id, policy);
        self.health_checks.insert(agent_id, HealthCheck::new());
    }

    /// Get restart policy
    pub fn get_policy(&self, agent_id: &AgentId) -> Option<&RestartPolicy> {
        self.policies.get(agent_id)
    }

    /// Get health check
    pub fn get_health_check(&self, agent_id: &AgentId) -> Option<&HealthCheck> {
        self.health_checks.get(agent_id)
    }

    /// Get mutable health check
    pub fn get_health_check_mut(&mut self, agent_id: &AgentId) -> Option<&mut HealthCheck> {
        self.health_checks.get_mut(agent_id)
    }

    /// Get supervisor name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get supervised agent count
    pub fn supervised_count(&self) -> usize {
        self.policies.len()
    }
}
