/// Performance monitoring for LLVM code generation
/// 
/// This module provides performance monitoring capabilities for the LLVM backend.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance metrics for LLVM compilation
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub compilation_time: Duration,
    pub optimization_time: Duration,
    pub code_size: usize,
    pub memory_usage: usize,
}

/// Performance monitor for LLVM operations
pub struct PerformanceMonitor {
    metrics: HashMap<String, PerformanceMetrics>,
    start_time: Option<Instant>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
            start_time: None,
        }
    }
    
    /// Start monitoring a compilation phase
    pub fn start_phase(&mut self, phase_name: &str) {
        self.start_time = Some(Instant::now());
    }
    
    /// End monitoring and record metrics
    pub fn end_phase(&mut self, phase_name: &str, code_size: usize, memory_usage: usize) {
        if let Some(start) = self.start_time.take() {
            let elapsed = start.elapsed();
            let metrics = PerformanceMetrics {
                compilation_time: elapsed,
                optimization_time: Duration::from_millis(0), // TODO: track separately
                code_size,
                memory_usage,
            };
            self.metrics.insert(phase_name.to_string(), metrics);
        }
    }
    
    /// Get metrics for a specific phase
    pub fn get_metrics(&self, phase_name: &str) -> Option<&PerformanceMetrics> {
        self.metrics.get(phase_name)
    }
    
    /// Get all recorded metrics
    pub fn get_all_metrics(&self) -> &HashMap<String, PerformanceMetrics> {
        &self.metrics
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}
