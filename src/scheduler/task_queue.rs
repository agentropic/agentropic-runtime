use agentropic_core::AgentId;
use std::collections::VecDeque;

/// Task in the queue
#[derive(Debug, Clone)]
pub struct Task {
    agent_id: AgentId,
    priority: u32,
}

impl Task {
    /// Create a new task
    pub fn new(agent_id: AgentId, priority: u32) -> Self {
        Self { agent_id, priority }
    }

    /// Get agent ID
    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }

    /// Get priority
    pub fn priority(&self) -> u32 {
        self.priority
    }
}

/// Task queue
#[derive(Debug)]
pub struct TaskQueue {
    tasks: VecDeque<Task>,
}

impl TaskQueue {
    /// Create a new task queue
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    /// Push a task
    pub fn push(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    /// Pop a task
    pub fn pop(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Get queue length
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Clear the queue
    pub fn clear(&mut self) {
        self.tasks.clear();
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}
