//! Optimization result types and statistics

use std::time::Duration;
use std::collections::HashMap;
use crate::error::CursedError;

#[derive(Debug, Clone)]
pub struct OptimizationStats {
    pub passes_run: u32,
    pub total_time: Duration,
    pub memory_saved: usize,
    pub performance_improvement: f64,
    pub code_size_reduction: f64,
    pub pass_statistics: HashMap<String, PassStats>,
}

#[derive(Debug, Clone)]
pub struct PassStats {
    pub executions: u32,
    pub total_time: Duration,
    pub success_rate: f64,
}

#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub success: bool,
    pub stats: OptimizationStats,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl OptimizationStats {
    pub fn new() -> Self {
        Self {
            passes_run: 0,
            total_time: Duration::new(0, 0),
            memory_saved: 0,
            performance_improvement: 0.0,
            code_size_reduction: 0.0,
            pass_statistics: HashMap::new(),
        }
    }

    pub fn add_pass_result(&mut self, pass_name: String, duration: Duration, success: bool) {
        self.passes_run += 1;
        self.total_time += duration;
        
        let stats = self.pass_statistics.entry(pass_name).or_insert(PassStats {
            executions: 0,
            total_time: Duration::new(0, 0),
            success_rate: 1.0,
        });
        
        stats.executions += 1;
        stats.total_time += duration;
        
        // Update success rate
        let old_successes = (stats.success_rate * (stats.executions - 1) as f64) as u32;
        let new_successes = if success { old_successes + 1 } else { old_successes };
        stats.success_rate = new_successes as f64 / stats.executions as f64;
    }
}

impl OptimizationResult {
    pub fn success(stats: OptimizationStats) -> Self {
        Self {
            success: true,
            stats,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn failure(errors: Vec<String>) -> Self {
        Self {
            success: false,
            stats: OptimizationStats::new(),
            errors,
            warnings: Vec::new(),
        }
    }

    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }

    /// Set timing information
    pub fn set_timing(&mut self, duration: std::time::Duration) {
        self.stats.total_time = duration;
    }

    /// Set memory usage information
    pub fn set_memory_usage(&mut self, memory_mb: u64) {
        self.stats.memory_saved = memory_mb as usize;
    }

    /// Set cache statistics
    pub fn set_cache_stats(&mut self, hit_rate: u32, miss_rate: u32) {
        // Store in performance improvement for now
        self.stats.performance_improvement = hit_rate as f64 / (hit_rate + miss_rate) as f64;
    }

    /// Set processing statistics
    pub fn set_processing_stats(&mut self, processed: u32, skipped: u32) {
        self.stats.passes_run = processed;
    }

    /// Set performance improvements
    pub fn set_improvements(&mut self, speed_improvement: f64, size_reduction: f64) {
        self.stats.performance_improvement = speed_improvement;
        self.stats.code_size_reduction = size_reduction;
    }

    /// Print summary of optimization results
    pub fn print_summary(&self) {
        println!("🎯 Optimization Results Summary:");
        println!("   Success: {}", self.success);
        println!("   Passes Run: {}", self.stats.passes_run);
        println!("   Total Time: {:?}", self.stats.total_time);
        println!("   Performance Improvement: {:.1}%", self.stats.performance_improvement);
        println!("   Code Size Reduction: {:.1}%", self.stats.code_size_reduction);
        if !self.warnings.is_empty() {
            println!("   Warnings: {}", self.warnings.len());
        }
        if !self.errors.is_empty() {
            println!("   Errors: {}", self.errors.len());
        }
    }

    /// Get primary result (for compatibility)
    pub fn primary(&self) -> bool {
        self.success
    }
}

impl Default for OptimizationStats {
    fn default() -> Self {
        Self::new()
    }
}
