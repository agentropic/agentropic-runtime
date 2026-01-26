use super::Collector;
use std::collections::HashMap;

/// Metrics registry
pub struct MetricsRegistry {
    collectors: HashMap<String, Collector>,
}

impl MetricsRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            collectors: HashMap::new(),
        }
    }

    /// Register collector
    pub fn register(&mut self, name: impl Into<String>, collector: Collector) {
        self.collectors.insert(name.into(), collector);
    }

    /// Get collector
    pub fn get(&self, name: &str) -> Option<&Collector> {
        self.collectors.get(name)
    }

    /// Get mutable collector
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Collector> {
        self.collectors.get_mut(name)
    }

    /// Get all collectors
    pub fn collectors(&self) -> impl Iterator<Item = (&String, &Collector)> {
        self.collectors.iter()
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}
