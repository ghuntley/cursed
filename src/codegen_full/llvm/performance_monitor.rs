// Performance monitoring for LLVM code generation
use crate::error::CursedError;
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Performance monitor for LLVM compilation
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    pub enabled: bool,
    start_time: Option<Instant>,
    metrics: HashMap<String, Duration>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            enabled: true,
            start_time: None,
            metrics: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn record_metric(&mut self, name: &str, duration: Duration) {
        self.metrics.insert(name.to_string(), duration);
    }

    pub fn get_metrics(&self) -> &HashMap<String, Duration> {
        &self.metrics
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for performance monitoring
#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub collect_detailed_metrics: bool,
    pub sample_rate: f64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collect_detailed_metrics: false,
            sample_rate: 1.0,
        }
    }
}

/// Code metrics collected during compilation
#[derive(Debug, Clone)]
pub struct CodeMetrics {
    pub lines_of_code: usize,
    pub function_count: usize,
    pub instruction_count: usize,
    pub optimization_passes: usize,
}

impl Default for CodeMetrics {
    fn default() -> Self {
        Self {
            lines_of_code: 0,
            function_count: 0,
            instruction_count: 0,
            optimization_passes: 0,
        }
    }
}

/// Baseline metrics for performance comparison
#[derive(Debug, Clone)]
pub struct BaselineMetrics {
    pub compilation_time: Duration,
    pub memory_usage: usize,
    pub output_size: usize,
}

impl Default for BaselineMetrics {
    fn default() -> Self {
        Self {
            compilation_time: Duration::from_secs(0),
            memory_usage: 0,
            output_size: 0,
        }
    }
}

/// Performance report containing all metrics
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub metrics: CodeMetrics,
    pub baseline: BaselineMetrics,
    pub compilation_time: Duration,
    pub optimizations_applied: Vec<String>,
}

impl Default for PerformanceReport {
    fn default() -> Self {
        Self {
            metrics: CodeMetrics::default(),
            baseline: BaselineMetrics::default(),
            compilation_time: Duration::from_secs(0),
            optimizations_applied: Vec::new(),
        }
    }
}
