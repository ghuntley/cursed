/// LLVM Optimization Passes for CURSED
/// 
/// This module contains all optimization pass implementations specific to the CURSED language,
/// as well as wrappers for standard LLVM passes with CURSED-specific configurations.

pub mod dead_code_elimination;
pub mod constant_propagation;
pub mod loop_optimization;
pub mod inlining;
pub mod pass_registry;
pub mod pass_pipeline;
pub mod performance_monitoring;

// Critical optimization passes
pub mod sccp;
pub mod licm;
pub mod gvn;
pub mod sroa;
pub mod mem2reg;
pub mod tail_call;
pub mod jump_threading;

// Re-export main pass types
pub use dead_code_elimination::{DeadCodeEliminationPass, DeadCodeAnalyzer};
pub use constant_propagation::{ConstantPropagationPass, ConstantFolder};
pub use loop_optimization::{LoopOptimizationPass, LoopUnroller, LoopVectorizer};
pub use inlining::{InliningPass, InliningHeuristics, CallSiteAnalyzer};
pub use pass_registry::{PassRegistry, PassRegistration, PassDependency};
pub use pass_pipeline::{OptimizationPipeline, PipelineBuilder, OptimizationStage, PassExecutionContext};
pub use performance_monitoring::{PerformanceMonitor, OptimizationMetrics, PassExecutionStats};

// Critical optimization passes
pub use sccp::SccpPass;
pub use licm::LicmPass;
pub use gvn::GvnPass;
pub use sroa::SroaPass;
pub use mem2reg::Mem2RegPass;
pub use tail_call::TailCallPass;
pub use jump_threading::JumpThreadingPass;

use crate::error::{Error, Result};
use inkwell::{
    context::Context,
    module::Module,
    values::FunctionValue,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument};

/// Base trait for all optimization passes
pub trait OptimizationPass<'ctx> {
    /// Get the name of this pass
    fn name(&self) -> &str;
    
    /// Get the description of what this pass does
    fn description(&self) -> &str;
    
    /// Get the pass dependencies
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }
    
    /// Check if this pass should run given the current configuration
    fn should_run(&self, config: &PassConfiguration) -> bool;
    
    /// Run the pass on a module
    fn run_on_module(&mut self, module: &Module<'ctx>, context: &'ctx Context) -> Result<PassResult>;
    
    /// Run the pass on a function
    fn run_on_function(&mut self, function: &FunctionValue<'ctx>, context: &'ctx Context) -> Result<PassResult> {
        // Default implementation - not all passes work on functions
        Ok(PassResult::unchanged())
    }
    
    /// Get optimization level requirements
    fn required_optimization_level(&self) -> OptimizationLevel {
        OptimizationLevel::Basic
    }
    
    /// Get estimated execution time
    fn estimated_execution_time(&self) -> Duration {
        Duration::from_millis(100)
    }
    
    /// Reset pass state for reuse
    fn reset(&mut self) {}
    
    /// Get pass statistics
    fn get_statistics(&self) -> PassStatistics {
        PassStatistics::default()
    }
}

/// Configuration for optimization passes
#[derive(Debug, Clone)]
pub struct PassConfiguration {
    pub optimization_level: OptimizationLevel,
    pub target_cpu: Option<String>,
    pub target_features: Vec<String>,
    pub enable_aggressive_optimizations: bool,
    pub enable_size_optimizations: bool,
    pub enable_debug_info_preservation: bool,
    pub max_inline_size: usize,
    pub max_unroll_count: usize,
    pub enable_vectorization: bool,
    pub enable_loop_unrolling: bool,
    pub enable_dead_code_elimination: bool,
    pub enable_constant_propagation: bool,
    pub enable_memory_optimizations: bool,
    pub time_budget: Duration,
}

impl Default for PassConfiguration {
    fn default() -> Self {
        Self {
            optimization_level: OptimizationLevel::Default,
            target_cpu: None,
            target_features: Vec::new(),
            enable_aggressive_optimizations: false,
            enable_size_optimizations: false,
            enable_debug_info_preservation: true,
            max_inline_size: 1000,
            max_unroll_count: 8,
            enable_vectorization: true,
            enable_loop_unrolling: true,
            enable_dead_code_elimination: true,
            enable_constant_propagation: true,
            enable_memory_optimizations: true,
            time_budget: Duration::from_secs(30),
        }
    }
}

/// Result of running an optimization pass
#[derive(Debug, Clone)]
pub struct PassResult {
    pub changed: bool,
    pub instructions_eliminated: usize,
    pub functions_inlined: usize,
    pub loops_unrolled: usize,
    pub constants_folded: usize,
    pub memory_allocations_eliminated: usize,
    pub branches_eliminated: usize,
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub metrics: HashMap<String, f64>,
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
            memory_allocations_eliminated: 0,
            branches_eliminated: 0,
            execution_time: Duration::from_secs(0),
            memory_usage: 0,
            errors: Vec::new(),
            warnings: Vec::new(),
            metrics: HashMap::new(),
        }
    }
    
    /// Create a new pass result indicating changes were made
    pub fn changed() -> Self {
        Self {
            changed: true,
            ..Self::unchanged()
        }
    }
    
    /// Merge two pass results
    pub fn merge(mut self, other: PassResult) -> Self {
        self.changed = self.changed || other.changed;
        self.instructions_eliminated += other.instructions_eliminated;
        self.functions_inlined += other.functions_inlined;
        self.loops_unrolled += other.loops_unrolled;
        self.constants_folded += other.constants_folded;
        self.memory_allocations_eliminated += other.memory_allocations_eliminated;
        self.branches_eliminated += other.branches_eliminated;
        self.execution_time += other.execution_time;
        self.memory_usage = self.memory_usage.max(other.memory_usage);
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        
        for (key, value) in other.metrics {
            let current = self.metrics.entry(key).or_insert(0.0);
            *current += value;
        }
        
        self
    }
    
    /// Calculate optimization effectiveness score
    pub fn effectiveness_score(&self) -> f64 {
        if self.execution_time.as_millis() == 0 {
            return 0.0;
        }
        
        let optimizations = (self.instructions_eliminated + 
                           self.functions_inlined + 
                           self.loops_unrolled +
                           self.constants_folded +
                           self.memory_allocations_eliminated +
                           self.branches_eliminated) as f64;
        
        optimizations / self.execution_time.as_millis() as f64
    }
}

/// Statistics for a pass execution
#[derive(Debug, Clone, Default)]
pub struct PassStatistics {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub total_execution_time: Duration,
    pub average_execution_time: Duration,
    pub total_instructions_eliminated: u64,
    pub total_functions_inlined: u64,
    pub total_optimizations_applied: u64,
    pub peak_memory_usage: usize,
}

impl PassStatistics {
    /// Update statistics with a new execution result
    pub fn update(&mut self, result: &PassResult) {
        self.total_executions += 1;
        if result.errors.is_empty() {
            self.successful_executions += 1;
        }
        
        self.total_execution_time += result.execution_time;
        self.average_execution_time = 
            self.total_execution_time / self.total_executions as u32;
            
        self.total_instructions_eliminated += result.instructions_eliminated as u64;
        self.total_functions_inlined += result.functions_inlined as u64;
        self.total_optimizations_applied += (result.instructions_eliminated +
                                           result.functions_inlined +
                                           result.loops_unrolled +
                                           result.constants_folded) as u64;
        self.peak_memory_usage = self.peak_memory_usage.max(result.memory_usage);
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

/// Optimization levels for passes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptimizationLevel {
    None,
    Basic,
    Default,
    Aggressive,
    Size,
    MinSize,
}

impl OptimizationLevel {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "0" | "o0" | "none" => Ok(OptimizationLevel::None),
            "1" | "o1" | "basic" => Ok(OptimizationLevel::Basic),
            "2" | "o2" | "default" => Ok(OptimizationLevel::Default),
            "3" | "o3" | "aggressive" => Ok(OptimizationLevel::Aggressive),
            "s" | "os" | "size" => Ok(OptimizationLevel::Size),
            "z" | "oz" | "minsize" => Ok(OptimizationLevel::MinSize),
            _ => Err(Error::Internal(format!("Invalid optimization level: {}", s))),
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            OptimizationLevel::None => "O0",
            OptimizationLevel::Basic => "O1",
            OptimizationLevel::Default => "O2",
            OptimizationLevel::Aggressive => "O3",
            OptimizationLevel::Size => "Os",
            OptimizationLevel::MinSize => "Oz",
        }
    }
    
    /// Get default configuration for this optimization level
    pub fn default_config(&self) -> PassConfiguration {
        let mut config = PassConfiguration::default();
        config.optimization_level = *self;
        
        match self {
            OptimizationLevel::None => {
                config.enable_aggressive_optimizations = false;
                config.enable_vectorization = false;
                config.enable_loop_unrolling = false;
                config.max_inline_size = 0;
                config.max_unroll_count = 0;
            }
            OptimizationLevel::Basic => {
                config.enable_aggressive_optimizations = false;
                config.max_inline_size = 500;
                config.max_unroll_count = 4;
            }
            OptimizationLevel::Default => {
                config.max_inline_size = 1000;
                config.max_unroll_count = 8;
            }
            OptimizationLevel::Aggressive => {
                config.enable_aggressive_optimizations = true;
                config.max_inline_size = 2000;
                config.max_unroll_count = 16;
            }
            OptimizationLevel::Size | OptimizationLevel::MinSize => {
                config.enable_size_optimizations = true;
                config.enable_vectorization = false;
                config.max_inline_size = 200;
                config.max_unroll_count = 2;
            }
        }
        
        config
    }
}

/// Utility functions for pass management
pub mod utils {
    use super::*;
    
    /// Estimate the complexity of a module for optimization planning
    pub fn estimate_module_complexity(module: &Module) -> ModuleComplexity {
        let function_count = module.get_functions().count();
        let mut total_instructions = 0;
        let mut total_basic_blocks = 0;
        
        for function in module.get_functions() {
            total_basic_blocks += function.count_basic_blocks();
            for basic_block in function.get_basic_blocks() {
                total_instructions += basic_block.get_instructions().count();
            }
        }
        
        ModuleComplexity {
            function_count,
            instruction_count: total_instructions,
            basic_block_count: total_basic_blocks,
            estimated_optimization_time: Duration::from_millis(
                (total_instructions as u64 * 10) + (function_count as u64 * 50)
            ),
        }
    }
    
    /// Check if a pass should be skipped based on module characteristics
    pub fn should_skip_pass(pass_name: &str, complexity: &ModuleComplexity, config: &PassConfiguration) -> bool {
        // Skip expensive passes on large modules if time budget is tight
        if complexity.estimated_optimization_time > config.time_budget {
            matches!(pass_name, "aggressive_inlining" | "whole_program_optimization")
        } else {
            false
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
