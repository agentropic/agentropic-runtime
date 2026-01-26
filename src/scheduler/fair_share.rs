/// Fair share scheduler
pub struct FairShareScheduler {
    shares: f64,
}

impl FairShareScheduler {
    /// Create a new fair share scheduler
    pub fn new(shares: f64) -> Self {
        Self { shares }
    }

    /// Get shares
    pub fn shares(&self) -> f64 {
        self.shares
    }

    /// Set shares
    pub fn set_shares(&mut self, shares: f64) {
        self.shares = shares;
    }
}

impl Default for FairShareScheduler {
    fn default() -> Self {
        Self::new(1.0)
    }
}
