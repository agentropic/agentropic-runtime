use serde::{Deserialize, Serialize};

/// Type of metric
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    /// Counter (monotonically increasing)
    Counter,
    /// Gauge (can go up or down)
    Gauge,
    /// Histogram
    Histogram,
}

/// Metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    name: String,
    metric_type: MetricType,
    value: f64,
    labels: Vec<(String, String)>,
}

impl Metric {
    /// Create a new metric
    pub fn new(name: impl Into<String>, metric_type: MetricType, value: f64) -> Self {
        Self {
            name: name.into(),
            metric_type,
            value,
            labels: Vec::new(),
        }
    }

    /// Add label
    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.push((key.into(), value.into()));
        self
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get type
    pub fn metric_type(&self) -> MetricType {
        self.metric_type
    }

    /// Get value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get labels
    pub fn labels(&self) -> &[(String, String)] {
        &self.labels
    }
}

/// Metrics collector
pub struct Collector {
    metrics: Vec<Metric>,
}

impl Collector {
    /// Create a new collector
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
        }
    }

    /// Record metric
    pub fn record(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }

    /// Get all metrics
    pub fn metrics(&self) -> &[Metric] {
        &self.metrics
    }

    /// Clear metrics
    pub fn clear(&mut self) {
        self.metrics.clear();
    }
}

impl Default for Collector {
    fn default() -> Self {
        Self::new()
    }
}
