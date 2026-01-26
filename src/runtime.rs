use crate::{RuntimeConfig, RuntimeError, RuntimeHandle};
use agentropic_core::AgentId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Shared AgentEntry struct (make it public for handle.rs)
#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct AgentEntry {
    pub id: AgentId,
    pub name: String,
}

/// Agent runtime engine
pub struct Runtime {
    config: RuntimeConfig,
    agents: Arc<RwLock<HashMap<AgentId, AgentEntry>>>,
    running: Arc<RwLock<bool>>,
}

impl Runtime {
    /// Create a new runtime with default configuration
    pub fn new() -> Self {
        Self::with_config(RuntimeConfig::default())
    }

    /// Create a runtime with custom configuration
    pub fn with_config(config: RuntimeConfig) -> Self {
        Self {
            config,
            agents: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Get runtime configuration
    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }

    /// Spawn an agent
    pub async fn spawn(
        &self,
        agent_id: AgentId,
        name: impl Into<String>,
    ) -> Result<(), RuntimeError> {
        let mut agents = self.agents.write().await;

        let entry = AgentEntry {
            id: agent_id,
            name: name.into(),
        };

        agents.insert(agent_id, entry);
        Ok(())
    }

    /// Get agent count
    pub async fn agent_count(&self) -> usize {
        self.agents.read().await.len()
    }

    /// Check if agent exists
    pub async fn has_agent(&self, agent_id: &AgentId) -> bool {
        self.agents.read().await.contains_key(agent_id)
    }

    /// Start the runtime
    pub async fn start(&self) -> Result<(), RuntimeError> {
        let mut running = self.running.write().await;
        *running = true;
        tracing::info!("Runtime started");
        Ok(())
    }

    /// Stop the runtime
    pub async fn stop(&self) -> Result<(), RuntimeError> {
        let mut running = self.running.write().await;
        *running = false;
        tracing::info!("Runtime stopped");
        Ok(())
    }

    /// Check if runtime is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Get runtime handle
    pub fn handle(&self) -> RuntimeHandle {
        RuntimeHandle::new(self.agents.clone(), self.running.clone())
    }

    /// Shutdown the run-time
    pub async fn shutdown(self) -> Result<(), RuntimeError> {
        self.stop().await?;
        let mut agents = self.agents.write().await;
        agents.clear();
        tracing::info!("Runtime shutdown complete");
        Ok(())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
