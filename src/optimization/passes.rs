//! High-level optimization pass orchestration

use crate::error::{CursedError, Result};
use crate::optimization::config::{OptimizationConfig, OptimizationLevel};
use crate::optimization::llvm_passes::{LlvmPassManager, OptimizationStatistics};
use inkwell::{context::Context, module::Module};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// High-level pass manager that orchestrates all optimizations
pub struct PassManager<'ctx> {
    llvm_manager: LlvmPassManager<'ctx>,
    config: OptimizationConfig,
    statistics: PassStatistics,
    custom_passes: HashMap<String, Box<dyn CustomPass + 'ctx>>,
}

impl<'ctx> PassManager<'ctx> {
    /// Create a new pass manager
    pub fn new(context: &'ctx Context, config: OptimizationConfig) -> Result<Self> {
        let llvm_manager = LlvmPassManager::new(context, config.clone())?;
        
        Ok(Self {
            llvm_manager,
            config,
            statistics: PassStatistics::default(),
            custom_passes: HashMap::new(),
        })
    }
    
    /// Register a custom optimization pass
    pub fn register_custom_pass(&mut self, name: String, pass: Box<dyn CustomPass + 'ctx>) {
        self.custom_passes.insert(name, pass);
    }
    
    /// Run all optimization passes on a module
    pub fn optimize(&mut self, module: &Module<'ctx>) -> Result<OptimizationResult> {
        let start_time = Instant::now();
        
        // Validate module before optimization
        if let Err(err) = module.verify() {
            return Err(CursedError::runtime_error(&format!("Module validation failed: {}", err)));
        }
        
        let mut result = OptimizationResult::new();
        
        // Run pre-optimization analysis
        let initial_metrics = self.analyze_module(module)?;
        result.initial_metrics = Some(initial_metrics);
        
        // Run custom passes first
        if self.config.pass_manager_config.enable_function_passes {
            for (name, pass) in &mut self.custom_passes {
                let pass_start = Instant::now();
                let pass_result = pass.run(module)?;
                let pass_time = pass_start.elapsed();
                
                result.passes_run.push(PassExecutionInfo {
                    name: name.clone(),
                    execution_time: pass_time,
                    changed: pass_result.changed,
                    metrics: pass_result.metrics,
                });
                
                if pass_result.changed {
                    result.total_changes += 1;
                }
            }
        }
        
        // Run LLVM optimization passes
        let llvm_start = Instant::now();
        let llvm_changed = self.llvm_manager.optimize_module(module)?;
        let llvm_time = llvm_start.elapsed();
        
        result.passes_run.push(PassExecutionInfo {
            name: "LLVM Standard Passes".to_string(),
            execution_time: llvm_time,
            changed: llvm_changed,
            metrics: HashMap::new(),
        });
        
        if llvm_changed {
            result.total_changes += 1;
        }
        
        // Run post-optimization analysis
        let final_metrics = self.analyze_module(module)?;
        result.final_metrics = Some(final_metrics);
        
        // Calculate optimization effectiveness
        if let (Some(initial), Some(final_metrics)) = (&result.initial_metrics, &result.final_metrics) {
            result.size_reduction = initial.module_size.saturating_sub(final_metrics.module_size) as f64 / initial.module_size as f64;
            result.complexity_reduction = initial.complexity_score - final_metrics.complexity_score;
        }
        
        result.total_time = start_time.elapsed();
        
        // Update statistics
        self.statistics.update(&result);
        
        // Validate module after optimization
        if let Err(err) = module.verify() {
            return Err(CursedError::runtime_error(&format!("Module validation failed after optimization: {}", err)));
        }
        
        Ok(result)
    }
    
    /// Analyze module metrics
    fn analyze_module(&self, module: &Module<'ctx>) -> Result<ModuleMetrics> {
        let mut metrics = ModuleMetrics::default();
        
        // Count functions and instructions
        for function in module.get_functions() {
            metrics.function_count += 1;
            metrics.basic_block_count += function.count_basic_blocks() as u32;
            
            for basic_block in function.get_basic_blocks() {
                let instruction_count = basic_block.get_instructions().count() as u32;
                metrics.instruction_count += instruction_count;
                
                // Analyze instruction types
                for instruction in basic_block.get_instructions() {
                    match instruction.get_opcode() {
                        inkwell::values::InstructionOpcode::Call => metrics.call_count += 1,
                        inkwell::values::InstructionOpcode::Load => metrics.load_count += 1,
                        inkwell::values::InstructionOpcode::Store => metrics.store_count += 1,
                        inkwell::values::InstructionOpcode::Br | 
                        inkwell::values::InstructionOpcode::Br => metrics.branch_count += 1,
                        _ => {}
                    }
                }
            }
        }
        
        // Estimate module size (simplified)
        metrics.module_size = (metrics.instruction_count * 4) as usize; // Rough estimate
        
        // Calculate complexity score
        metrics.complexity_score = (metrics.function_count as f64 * 10.0) + 
                                  (metrics.instruction_count as f64) + 
                                  (metrics.basic_block_count as f64 * 5.0) +
                                  (metrics.call_count as f64 * 2.0);
        
        Ok(metrics)
    }
    
    /// Get optimization statistics
    pub fn get_statistics(&self) -> &PassStatistics {
        &self.statistics
    }
    
    /// Get LLVM-specific statistics
    pub fn get_llvm_statistics(&self) -> OptimizationStatistics {
        self.llvm_manager.get_statistics()
    }
    
    /// Run specific optimization pipeline based on level
    pub fn run_pipeline(&mut self, module: &Module<'ctx>, level: OptimizationLevel) -> Result<OptimizationResult> {
        // Temporarily override configuration
        let original_level = self.config.level.clone();
        self.config.level = level;
        
        // Create new LLVM manager with updated config
        let context_ref = module.get_context();
        self.llvm_manager = LlvmPassManager::new(context_ref, self.config.clone())?;
        
        let result = self.optimize(module);
        
        // Restore original configuration
        self.config.level = original_level;
        
        result
    }
}

/// Trait for custom optimization passes
pub trait CustomPass {
    /// Get the name of this pass
    fn name(&self) -> &str;
    
    /// Run the pass on a module
    fn run(&mut self, module: &Module) -> Result<CustomPassResult>;
    
    /// Check if this pass should run given the current configuration
    fn should_run(&self, config: &OptimizationConfig) -> bool {
        true
    }
    
    /// Get pass dependencies
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }
}

/// Result from a custom optimization pass
#[derive(Debug)]
pub struct CustomPassResult {
    pub changed: bool,
    pub metrics: HashMap<String, f64>,
}

impl CustomPassResult {
    pub fn unchanged() -> Self {
        Self {
            changed: false,
            metrics: HashMap::new(),
        }
    }
    
    pub fn changed() -> Self {
        Self {
            changed: true,
            metrics: HashMap::new(),
        }
    }
    
    pub fn with_metric(mut self, name: String, value: f64) -> Self {
        self.metrics.insert(name, value);
        self
    }
}

/// Complete optimization result
#[derive(Debug)]
pub struct OptimizationResult {
    pub total_time: Duration,
    pub total_changes: u32,
    pub passes_run: Vec<PassExecutionInfo>,
    pub initial_metrics: Option<ModuleMetrics>,
    pub final_metrics: Option<ModuleMetrics>,
    pub size_reduction: f64,
    pub complexity_reduction: f64,
}

impl OptimizationResult {
    fn new() -> Self {
        Self {
            total_time: Duration::default(),
            total_changes: 0,
            passes_run: Vec::new(),
            initial_metrics: None,
            final_metrics: None,
            size_reduction: 0.0,
            complexity_reduction: 0.0,
        }
    }
    
    /// Calculate optimization effectiveness score
    pub fn effectiveness_score(&self) -> f64 {
        if self.total_time.as_millis() == 0 {
            return 0.0;
        }
        
        let benefit = self.size_reduction + (self.complexity_reduction / 1000.0);
        let time_cost = self.total_time.as_millis() as f64 / 1000.0; // Convert to seconds
        
        benefit / time_cost.max(0.001) // Avoid division by zero
    }
}

/// Information about a single pass execution
#[derive(Debug)]
pub struct PassExecutionInfo {
    pub name: String,
    pub execution_time: Duration,
    pub changed: bool,
    pub metrics: HashMap<String, f64>,
}

/// Module analysis metrics
#[derive(Debug, Default, Clone)]
pub struct ModuleMetrics {
    pub function_count: u32,
    pub basic_block_count: u32,
    pub instruction_count: u32,
    pub call_count: u32,
    pub load_count: u32,
    pub store_count: u32,
    pub branch_count: u32,
    pub module_size: usize,
    pub complexity_score: f64,
}

/// Statistics tracked across optimization runs
#[derive(Debug, Default)]
pub struct PassStatistics {
    pub total_optimizations: u32,
    pub total_time: Duration,
    pub average_time: Duration,
    pub total_size_reduction: f64,
    pub total_complexity_reduction: f64,
    pub passes_run_count: HashMap<String, u32>,
    pub effectiveness_scores: Vec<f64>,
}

impl PassStatistics {
    /// Update statistics with a new optimization result
    pub fn update(&mut self, result: &OptimizationResult) {
        self.total_optimizations += 1;
        self.total_time += result.total_time;
        self.average_time = self.total_time / self.total_optimizations;
        self.total_size_reduction += result.size_reduction;
        self.total_complexity_reduction += result.complexity_reduction;
        
        // Track individual pass statistics
        for pass_info in &result.passes_run {
            *self.passes_run_count.entry(pass_info.name.clone()).or_insert(0) += 1;
        }
        
        self.effectiveness_scores.push(result.effectiveness_score());
    }
    
    /// Get average effectiveness score
    pub fn average_effectiveness(&self) -> f64 {
        if self.effectiveness_scores.is_empty() {
            0.0
        } else {
            self.effectiveness_scores.iter().sum::<f64>() / self.effectiveness_scores.len() as f64
        }
    }
    
    /// Get most frequently run pass
    pub fn most_frequent_pass(&self) -> Option<(&String, &u32)> {
        self.passes_run_count.iter().max_by_key(|(_, count)| *count)
    }
}

/// Built-in custom passes
pub mod builtin_passes {
    use super::*;
    
    /// CURSED-specific function optimization pass
    pub struct CursedFunctionPass {
        aggressive_inlining: bool,
    }
    
    impl CursedFunctionPass {
        pub fn new(config: &OptimizationConfig) -> Self {
            Self {
                aggressive_inlining: matches!(config.level, OptimizationLevel::Aggressive),
            }
        }
    }
    
    impl CustomPass for CursedFunctionPass {
        fn name(&self) -> &str {
            "cursed-function-optimization"
        }
        
        fn run(&mut self, module: &Module) -> Result<CustomPassResult> {
            let mut changed = false;
            let mut metrics = HashMap::new();
            let mut inlined_functions = 0u32;
            
            // Analyze functions for CURSED-specific optimizations
            for function in module.get_functions() {
                // Skip external functions
                if function.get_first_basic_block().is_none() {
                    continue;
                }
                
                let instruction_count = function.get_basic_blocks()
                .into_iter().map(|bb| bb.get_instructions().count())
                    .sum::<usize>();
                
                // Mark small functions for aggressive inlining
                if self.aggressive_inlining && instruction_count < 10 {
                    // In a real implementation, we'd set inlining attributes
                    inlined_functions += 1;
                    changed = true;
                }
            }
            
            metrics.insert("functions_marked_for_inlining".to_string(), inlined_functions as f64);
            
            if changed {
                Ok(CustomPassResult::changed().with_metric("inlined_functions".to_string(), inlined_functions as f64))
            } else {
                Ok(CustomPassResult::unchanged())
            }
        }
    }
    
    /// CURSED error handling optimization pass
    pub struct CursedErrorOptimizationPass;
    
    impl CustomPass for CursedErrorOptimizationPass {
        fn name(&self) -> &str {
            "cursed-error-optimization"
        }
        
        fn run(&mut self, module: &Module) -> Result<CustomPassResult> {
            // Optimize CURSED-specific error handling patterns
            // This would analyze Result<T, E> usage patterns and optimize them
            
            let mut error_paths_optimized = 0u32;
            
            for function in module.get_functions() {
                if function.get_first_basic_block().is_none() {
                    continue;
                }
                
                // Look for error handling patterns in the IR
                for basic_block in function.get_basic_blocks() {
                    for instruction in basic_block.get_instructions() {
                        // In a real implementation, we'd analyze the IR for error patterns
                        // and optimize cold error paths
                        if instruction.get_opcode() == inkwell::values::InstructionOpcode::Call {
                            // Check if this is an error-handling call
                            error_paths_optimized += 1;
                        }
                    }
                }
            }
            
            if error_paths_optimized > 0 {
                Ok(CustomPassResult::changed()
                    .with_metric("error_paths_optimized".to_string(), error_paths_optimized as f64))
            } else {
                Ok(CustomPassResult::unchanged())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_pass_manager_creation() {
        let context = Context::create();
        let config = OptimizationConfig::debug();
        
        let manager = PassManager::new(&context, config);
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_custom_pass_result() {
        let result = CustomPassResult::changed()
            .with_metric("test".to_string(), 42.0);
        
        assert!(result.changed);
        assert_eq!(result.metrics.get("test"), Some(&42.0));
    }
    
    #[test]
    fn test_optimization_result_effectiveness() {
        let mut result = OptimizationResult::new();
        result.size_reduction = 0.1; // 10% size reduction
        result.complexity_reduction = 100.0;
        result.total_time = Duration::from_millis(100);
        
        let score = result.effectiveness_score();
        assert!(score > 0.0);
    }
}
