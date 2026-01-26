use super::{IsolationConfig, ResourceLimits};
use agentropic_core::AgentId;

/// Sandbox environment for isolated agent execution
pub struct Sandbox {
    agent_id: AgentId,
    config: IsolationConfig,
    limits: ResourceLimits,
}

impl Sandbox {
    /// Create a new sandbox
    pub fn new(agent_id: AgentId, config: IsolationConfig, limits: ResourceLimits) -> Self {
        Self {
            agent_id,
            config,
            limits,
        }
    }

    /// Get agent ID
    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }

    /// Get configuration
    pub fn config(&self) -> &IsolationConfig {
        &self.config
    }

    /// Get resource limits
    pub fn limits(&self) -> &ResourceLimits {
        &self.limits
    }

    /// Check if sandbox is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}
