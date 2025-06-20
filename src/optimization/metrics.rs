//! Optimization metrics collection and analysis
//!
//! This module provides comprehensive metrics for optimization performance tracking,
//! compilation statistics, and system resource monitoring.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Compilation unit metrics for tracking individual compilation tasks
#[derive(Debug, Clone)]
pub struct CompilationUnit {
    pub name: String,
    pub start_time: Instant,
    pub duration: Option<Duration>,
    pub size_bytes: usize,
    pub optimization_level: u32,
    pub success: bool,
}

impl CompilationUnit {
    pub fn new(name: String, size_bytes: usize, optimization_level: u32) -> Self {
        Self {
            name,
            start_time: Instant::now(),
            duration: None,
            size_bytes,
            optimization_level,
            success: false,
        }
    }

    pub fn complete(&mut self, success: bool) {
        self.duration = Some(self.start_time.elapsed());
        self.success = success;
    }
}

/// System statistics for resource monitoring during optimization
#[derive(Debug, Clone, Default)]
pub struct SystemStatistics {
    pub cpu_usage_percent: f64,
    pub memory_usage_bytes: u64,
    pub available_memory_bytes: u64,
    pub disk_io_read_bytes: u64,
    pub disk_io_write_bytes: u64,
    pub network_io_bytes: u64,
    pub load_average: f64,
    pub active_threads: usize,
}

/// Resource statistics for optimization processes
#[derive(Debug, Clone, Default)]
pub struct ResourceStatistics {
    pub compilation_time: Duration,
    pub peak_memory_usage: u64,
    pub total_cpu_time: Duration,
    pub file_io_operations: u64,
    pub cache_hit_rate: f64,
    pub parallel_efficiency: f64,
}

/// Compilation statistics aggregator
#[derive(Debug, Default)]
pub struct CompilationStatistics {
    pub total_units: usize,
    pub successful_units: usize,
    pub failed_units: usize,
    pub total_time: Duration,
    pub average_time_per_unit: Duration,
    pub total_size_bytes: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

impl CompilationStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_unit(&mut self, unit: &CompilationUnit) {
        self.total_units += 1;
        self.total_size_bytes += unit.size_bytes;
        
        if let Some(duration) = unit.duration {
            self.total_time += duration;
            self.average_time_per_unit = self.total_time / self.total_units as u32;
        }

        if unit.success {
            self.successful_units += 1;
        } else {
            self.failed_units += 1;
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_units == 0 {
            0.0
        } else {
            self.successful_units as f64 / self.total_units as f64
        }
    }

    pub fn cache_hit_rate(&self) -> f64 {
        let total_requests = self.cache_hits + self.cache_misses;
        if total_requests == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total_requests as f64
        }
    }
}

/// Metrics collector for optimization operations
#[derive(Debug)]
pub struct MetricsCollector {
    pub compilation_units: Vec<CompilationUnit>,
    pub system_stats: SystemStatistics,
    pub resource_stats: ResourceStatistics,
    pub compilation_stats: CompilationStatistics,
    pub custom_metrics: HashMap<String, f64>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            compilation_units: Vec::new(),
            system_stats: SystemStatistics::default(),
            resource_stats: ResourceStatistics::default(),
            compilation_stats: CompilationStatistics::new(),
            custom_metrics: HashMap::new(),
        }
    }

    pub fn start_compilation(&mut self, name: String, size_bytes: usize, optimization_level: u32) -> usize {
        let unit = CompilationUnit::new(name, size_bytes, optimization_level);
        self.compilation_units.push(unit);
        self.compilation_units.len() - 1
    }

    pub fn complete_compilation(&mut self, index: usize, success: bool) {
        if let Some(unit) = self.compilation_units.get_mut(index) {
            unit.complete(success);
            self.compilation_stats.add_unit(unit);
        }
    }

    pub fn record_cache_hit(&mut self) {
        self.compilation_stats.cache_hits += 1;
    }

    pub fn record_cache_miss(&mut self) {
        self.compilation_stats.cache_misses += 1;
    }

    pub fn update_system_stats(&mut self, stats: SystemStatistics) {
        self.system_stats = stats;
    }

    pub fn record_custom_metric(&mut self, name: String, value: f64) {
        self.custom_metrics.insert(name, value);
    }

    pub fn get_summary(&self) -> MetricsSummary {
        MetricsSummary {
            total_compilations: self.compilation_stats.total_units,
            successful_compilations: self.compilation_stats.successful_units,
            success_rate: self.compilation_stats.success_rate(),
            total_time: self.compilation_stats.total_time,
            average_time: self.compilation_stats.average_time_per_unit,
            cache_hit_rate: self.compilation_stats.cache_hit_rate(),
            peak_memory: self.resource_stats.peak_memory_usage,
            current_cpu_usage: self.system_stats.cpu_usage_percent,
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of optimization metrics
#[derive(Debug, Clone)]
pub struct MetricsSummary {
    pub total_compilations: usize,
    pub successful_compilations: usize,
    pub success_rate: f64,
    pub total_time: Duration,
    pub average_time: Duration,
    pub cache_hit_rate: f64,
    pub peak_memory: u64,
    pub current_cpu_usage: f64,
}

impl std::fmt::Display for MetricsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Optimization Metrics Summary:\n\
             Compilations: {}/{} ({:.1}% success)\n\
             Total time: {:.2}s\n\
             Average time: {:.2}ms\n\
             Cache hit rate: {:.1}%\n\
             Peak memory: {:.1} MB\n\
             CPU usage: {:.1}%",
            self.successful_compilations,
            self.total_compilations,
            self.success_rate * 100.0,
            self.total_time.as_secs_f64(),
            self.average_time.as_millis(),
            self.cache_hit_rate * 100.0,
            self.peak_memory as f64 / 1024.0 / 1024.0,
            self.current_cpu_usage
        )
    }
}
