use super::{SchedulingPolicy, TaskQueue};

/// Task scheduler
pub struct Scheduler {
    policy: SchedulingPolicy,
    queue: TaskQueue,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new(policy: SchedulingPolicy) -> Self {
        Self {
            policy,
            queue: TaskQueue::new(),
        }
    }

    /// Get scheduling policy
    pub fn policy(&self) -> &SchedulingPolicy {
        &self.policy
    }

    /// Get task queue
    pub fn queue(&self) -> &TaskQueue {
        &self.queue
    }

    /// Get mutable task queue
    pub fn queue_mut(&mut self) -> &mut TaskQueue {
        &mut self.queue
    }
}
