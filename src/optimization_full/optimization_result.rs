/// Optimization result types for the CURSED compiler
/// 
/// Provides unified result types for all optimization operations

use crate::error::{CursedError, Result};

use std::collections::HashMap;
use std::time::Duration;

/// General optimization result
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub success: bool,
    pub performance_improvement: f64,
    pub compilation_time_saved: Duration,
    pub optimizations_applied: Vec<String>,
    pub metrics: HashMap<String, f64>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl Default for OptimizationResult {
    fn default() -> Self {
        Self {
            success: true,
            performance_improvement: 0.0,
            compilation_time_saved: Duration::from_secs(0),
            optimizations_applied: Vec::new(),
            metrics: HashMap::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }
}

impl OptimizationResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_improvement(mut self, improvement: f64) -> Self {
        self.performance_improvement = improvement;
        self
    }

    pub fn with_time_saved(mut self, time_saved: Duration) -> Self {
        self.compilation_time_saved = time_saved;
        self
    }

    pub fn add_optimization(mut self, optimization: String) -> Self {
        self.optimizations_applied.push(optimization);
        self
    }

    pub fn add_metric(mut self, name: String, value: f64) -> Self {
        self.metrics.insert(name, value);
        self
    }

    pub fn add_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    pub fn add_error(mut self, error: String) -> Self {
        self.errors.push(error);
        self.success = false;
        self
    }
}

// Incremental optimization results
pub use OptimizationResult as IncrementalResult;

// Adaptive optimization results  
#[derive(Debug, Clone)]
pub struct AdaptiveResults {
    pub base_result: OptimizationResult,
    pub adaptation_strategy: String,
    pub adaptation_effectiveness: f64,
}

#[derive(Debug, Clone)]
pub struct AdaptiveStrategy {
    pub name: String,
    pub parameters: HashMap<String, f64>,
    pub expected_improvement: f64,
}

// Memory optimization results
#[derive(Debug, Clone)]
pub struct MemoryOptimizer {
    pub enabled: bool,
    pub strategy: String,
}

#[derive(Debug, Clone)]  
pub struct MemoryOptimizationResults {
    pub base_result: OptimizationResult,
    pub memory_saved: usize,
    pub allocation_optimizations: usize,
}

// Build optimization results
#[derive(Debug, Clone)]
pub struct BuildOptimizer {
    pub parallel_enabled: bool,
    pub cache_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct BuildOptimizationResults {
    pub base_result: OptimizationResult,
    pub build_time_saved: Duration,
    pub cache_hit_rate: f64,
}

// Parallel compilation results
#[derive(Debug, Clone)]
pub struct ParallelCompilationResults {
    pub base_result: OptimizationResult,
    pub parallelization_factor: f64,
    pub thread_efficiency: f64,
}

// Profiler results
#[derive(Debug, Clone)]
pub struct OptimizationProfiler {
    pub enabled: bool,
    pub sampling_rate: f64,
}

#[derive(Debug, Clone)]
pub struct ProfilerResults {
    pub base_result: OptimizationResult,
    pub hotspots_identified: usize,
    pub profiling_overhead: f64,
}

// Runtime optimization results
#[derive(Debug, Clone)]
pub struct RuntimeOptimizer {
    pub jit_enabled: bool,
    pub adaptive_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct RuntimeOptimizationResults {
    pub base_result: OptimizationResult,
    pub runtime_improvement: f64,
    pub jit_effectiveness: f64,
}

// Profiling config
#[derive(Debug, Clone)]
pub struct ProfilingConfig {
    pub enabled: bool,
    pub sample_rate: f64,
    pub output_format: String,
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sample_rate: 1000.0, // samples per second
            output_format: "json".to_string(),
        }
    }
}
