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
}

impl Default for OptimizationStats {
    fn default() -> Self {
        Self::new()
    }
}
