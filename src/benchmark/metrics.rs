//! Metrics collection for benchmarks

use std::time::Duration;
use std::fmt::{self, Debug};

/// Metric type that can be used to categorize metrics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    /// Timing-related metrics
    Timing,
    /// Memory-related metrics
    Memory,
    /// Garbage collection metrics
    GarbageCollection,
    /// Throughput metrics
    Throughput,
    /// Custom metrics
    Custom,
}

/// Value for a metric
#[derive(Debug, Clone)]
pub enum MetricValue {
    /// Duration value in nanoseconds
    Duration(Duration),
    /// Integer value
    Integer(i64),
    /// Unsigned integer value
    UInteger(u64),
    /// Floating point value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// String value
    String(String),
}

impl fmt::Display for MetricValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetricValue::Duration(d) => write!(f, "{:?}", d),
            MetricValue::Integer(i) => write!(f, "{}", i),
            MetricValue::UInteger(u) => write!(f, "{}", u),
            MetricValue::Float(fl) => write!(f, "{:.6}", fl),
            MetricValue::Boolean(b) => write!(f, "{}", b),
            MetricValue::String(s) => write!(f, "{}", s),
        }
    }
}

/// Trait for metrics that can be collected during benchmarks
pub trait Metric: Debug + Send + Sync {
    /// Get the name of the metric
    fn name(&self) -> &str;
    
    /// Get the type of the metric
    fn metric_type(&self) -> MetricType;
    
    /// Get the value for this metric
    fn value(&self) -> MetricValue;
    
    /// Get a unique key for this metric (used for aggregation)
    fn key(&self) -> String {
        format!("{}/{}", self.metric_type().to_string(), self.name())
    }
    
    /// Whether this metric is better when higher
    fn higher_is_better(&self) -> bool {
        false // Default to lower is better
    }
    
    /// Get unit of measurement
    fn unit(&self) -> &str;
    
    /// Clone this metric
    fn box_clone(&self) -> Box<dyn Metric>;
}

impl Clone for Box<dyn Metric> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetricType::Timing => write!(f, "timing"),
            MetricType::Memory => write!(f, "memory"),
            MetricType::GarbageCollection => write!(f, "gc"),
            MetricType::Throughput => write!(f, "throughput"),
            MetricType::Custom => write!(f, "custom"),
        }
    }
}

/// Timing-related metric
#[derive(Debug, Clone)]
pub struct TimingMetric {
    /// Name of the metric
    pub name: String,
    /// Duration value
    pub duration: Duration,
}

impl Metric for TimingMetric {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn metric_type(&self) -> MetricType {
        MetricType::Timing
    }
    
    fn value(&self) -> MetricValue {
        MetricValue::Duration(self.duration)
    }
    
    fn higher_is_better(&self) -> bool {
        false // Lower timing is better
    }
    
    fn unit(&self) -> &str {
        "ns"
    }
    
    fn box_clone(&self) -> Box<dyn Metric> {
        Box::new(self.clone())
    }
}

/// Memory-related metric
#[derive(Debug, Clone)]
pub struct MemoryMetric {
    /// Name of the metric
    pub name: String,
    /// Object count before operation
    pub before_object_count: usize,
    /// Object count after operation
    pub after_object_count: usize,
    /// Total heap size before operation
    pub before_total_size: usize,
    /// Total heap size after operation
    pub after_total_size: usize,
    /// Bytes allocated during operation
    pub allocated: usize,
    /// Bytes collected during operation
    pub collected: usize,
    /// Time spent in GC during operation (milliseconds)
    pub collection_time_ms: u128,
}

impl Metric for MemoryMetric {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn metric_type(&self) -> MetricType {
        MetricType::Memory
    }
    
    fn value(&self) -> MetricValue {
        // Return allocated bytes as the primary value
        MetricValue::UInteger(self.allocated as u64)
    }
    
    fn higher_is_better(&self) -> bool {
        false // Lower memory usage is better
    }
    
    fn unit(&self) -> &str {
        "bytes"
    }
    
    fn box_clone(&self) -> Box<dyn Metric> {
        Box::new(self.clone())
    }
}

/// Throughput-related metric
#[derive(Debug, Clone)]
pub struct ThroughputMetric {
    /// Name of the metric
    pub name: String,
    /// Operations performed
    pub operations: u64,
    /// Time spent
    pub duration: Duration,
    /// Unit of operations (e.g., "requests", "objects")
    pub operation_unit: String,
}

impl Metric for ThroughputMetric {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn metric_type(&self) -> MetricType {
        MetricType::Throughput
    }
    
    fn value(&self) -> MetricValue {
        // Calculate operations per second
        let seconds = self.duration.as_secs_f64();
        let ops_per_second = if seconds > 0.0 {
            self.operations as f64 / seconds
        } else {
            0.0
        };
        MetricValue::Float(ops_per_second)
    }
    
    fn higher_is_better(&self) -> bool {
        true // Higher throughput is better
    }
    
    fn unit(&self) -> &str {
        &self.operation_unit
    }
    
    fn box_clone(&self) -> Box<dyn Metric> {
        Box::new(self.clone())
    }
}