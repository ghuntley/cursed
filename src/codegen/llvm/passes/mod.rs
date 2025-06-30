/// LLVM Optimization Passes for CURSED
/// 
/// This module contains all optimization pass implementations specific to the CURSED language,
/// as well as wrappers for standard LLVM passes with CURSED-specific configurations.

// Core optimization passes
pub mod dead_code_elimination;
pub mod constant_propagation;
pub mod loop_optimization;
pub mod loop_optimization_old;
pub mod inlining;
pub mod gvn;

// Critical optimization passes (minimally implemented)
pub mod pass_registry;
pub mod pass_pipeline;
pub mod performance_monitoring;
pub mod sccp;
pub mod licm;
pub mod sroa;
pub mod mem2reg;
pub mod tail_call;
pub mod jump_threading;

// Re-export main pass types with their results
pub use dead_code_elimination::{DeadCodeEliminationPass, DeadCodeAnalyzer, DeadCodeResult};
pub use constant_propagation::ConstantPropagationPass;
pub use loop_optimization::{LoopOptimizationPass, LoopInfo};
pub use loop_optimization_old::LoopOptimizationResult;
pub use inlining::{InliningPass, InliningHeuristics, CallSiteAnalyzer, InliningResult};
pub use gvn::{GvnPass, GvnResult};

// Placeholder exports for other passes
pub use pass_registry::{PassRegistry, PassRegistration, PassDependency};
pub use pass_pipeline::{OptimizationPipeline, PipelineBuilder};
pub use performance_monitoring::{PerformanceMonitor, OptimizationMetrics, PassExecutionStats};

// Optimization passes - restored from minimal implementations
pub use sccp::SccpPass;
pub use licm::LicmPass;
pub use sroa::SroaPass;
pub use mem2reg::Mem2RegPass;
pub use tail_call::TailCallPass;
pub use jump_threading::JumpThreadingPass;

use crate::error::{CursedError, Result};
use std::collections::HashMap;
use std::time::Duration;

/// Optimization levels
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationLevel {
    O0,
    O1,
    O2,
    O3,
}

/// Configuration for optimization passes
#[derive(Debug, Clone)]
pub struct PassConfiguration {
    pub optimization_level: OptimizationLevel,
    pub time_budget: Duration,
}

impl Default for PassConfiguration {
    fn default() -> Self {
        Self {
            optimization_level: OptimizationLevel::O2,
            time_budget: Duration::from_secs(30),
        }
    }
}

/// Result of running an optimization pass
#[derive(Debug, Clone)]
pub struct PassResult {
    pub changed: bool,
    pub instructions_eliminated: u32,
    pub functions_inlined: u32,
    pub loops_unrolled: u32,
    pub constants_folded: u32,
    pub execution_time: Duration,
}

impl PassResult {
    /// Create a new pass result indicating no changes
    pub fn unchanged() -> Self {
        Self {
            changed: false,
            instructions_eliminated: 0,
            functions_inlined: 0,
            loops_unrolled: 0,
            constants_folded: 0,
            execution_time: Duration::default(),
        }
    }
    
    /// Create a new pass result indicating changes were made
    pub fn changed() -> Self {
        Self {
            changed: true,
            ..Self::unchanged()
        }
    }
}

/// Statistics for a pass execution
#[derive(Debug, Clone, Default)]
pub struct PassStatistics {
    pub total_executions: u32,
    pub successful_executions: u32,
    pub total_execution_time: Duration,
    pub average_execution_time: Duration,
}

impl PassStatistics {
    /// Update statistics with a new execution result
    pub fn update(&mut self, result: &PassResult) {
        self.total_executions += 1;
        self.successful_executions += 1; // Assume success for now
        self.total_execution_time += result.execution_time;
        self.average_execution_time = 
            self.total_execution_time / self.total_executions.max(1);
    }
    
    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            self.successful_executions as f64 / self.total_executions as f64
        }
    }
}

/// Module complexity metrics
#[derive(Debug, Clone)]
pub struct ModuleComplexity {
    pub function_count: usize,
    pub instruction_count: usize,
    pub basic_block_count: usize,
    pub estimated_optimization_time: Duration,
}

impl ModuleComplexity {
    /// Calculate complexity score
    pub fn complexity_score(&self) -> f64 {
        (self.function_count as f64 * 10.0) + 
        (self.instruction_count as f64) + 
        (self.basic_block_count as f64 * 5.0)
    }
    
    /// Check if module is considered large
    pub fn is_large_module(&self) -> bool {
        self.function_count > 100 || self.instruction_count > 10000
    }
}
