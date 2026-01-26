/// Priority scheduler
pub struct PriorityScheduler {
    levels: u32,
}

impl PriorityScheduler {
    /// Create a new priority scheduler
    pub fn new(levels: u32) -> Self {
        Self { levels }
    }

    /// Get priority levels
    pub fn levels(&self) -> u32 {
        self.levels
    }
}

impl Default for PriorityScheduler {
    fn default() -> Self {
        Self::new(5)
    }
}
