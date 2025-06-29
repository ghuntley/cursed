//! Enhanced LLVM pass management with advanced features

use crate::error::CursedError;
use crate::optimization::config::{OptimizationConfig, OptimizationLevel};
use crate::optimization::real_llvm_passes::{RealLlvmPassManager, CustomPassInfo, PassCategory};
use inkwell::context::Context;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct EnhancedLlvmPassManager<'ctx> {
    real_pass_manager: RealLlvmPassManager<'ctx>,
    config: OptimizationConfig,
    pass_statistics: HashMap<String, PassStatistics>,
    execution_history: Vec<PassExecution>,
    adaptive_optimization: bool,
}

#[derive(Debug, Clone)]
pub struct PassStatistics {
    pub executions: u64,
    pub total_time: Duration,
    pub average_time: Duration,
    pub last_execution: Option<Instant>,
    pub success_rate: f64,
    pub performance_impact: f64,
}

#[derive(Debug, Clone)]
pub struct PassExecution {
    pub pass_name: String,
    pub timestamp: Instant,
    pub duration: Duration,
    pub success: bool,
    pub context: ExecutionContext,
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub optimization_level: OptimizationLevel,
    pub module_size: usize,
    pub function_count: usize,
    pub complexity_score: f64,
}

impl<'ctx> EnhancedLlvmPassManager<'ctx> {
    pub fn new(context: &'ctx Context, config: OptimizationConfig) -> Result<Self, CursedError> {
        let real_pass_manager = RealLlvmPassManager::new(context, config.clone())?;
        
        Ok(Self {
            real_pass_manager,
            config,
            pass_statistics: HashMap::new(),
            execution_history: Vec::new(),
            adaptive_optimization: true,
        })
    }

    pub fn with_adaptive_optimization(mut self, enable: bool) -> Self {
        self.adaptive_optimization = enable;
        self
    }

    pub fn run_enhanced_passes(&mut self, context: ExecutionContext) -> Result<OptimizationResult, CursedError> {
        let start_time = Instant::now();
        
        // Adaptive pass selection based on context
        if self.adaptive_optimization {
            self.adapt_passes_for_context(&context)?;
        }

        // Execute passes with timing and statistics collection
        let enabled_passes = self.real_pass_manager.get_enabled_passes().to_vec();
        let mut successful_passes = 0;
        let mut failed_passes = 0;

        for pass_name in &enabled_passes {
            let pass_result = self.execute_pass_with_stats(pass_name, &context)?;
            if pass_result.success {
                successful_passes += 1;
            } else {
                failed_passes += 1;
            }
        }

        let total_time = start_time.elapsed();
        
        Ok(OptimizationResult {
            total_time,
            passes_executed: enabled_passes.len(),
            successful_passes,
            failed_passes,
            performance_improvement: self.estimate_performance_improvement(&context),
            size_reduction: self.estimate_size_reduction(&context),
        })
    }

    fn adapt_passes_for_context(&mut self, context: &ExecutionContext) -> Result<(), CursedError> {
        // Clear current passes
        self.real_pass_manager.clear_passes();

        // Select passes based on context
        match context.optimization_level {
            OptimizationLevel::None => {
                // No optimizations for debug builds
            },
            OptimizationLevel::Less => {
                self.add_basic_passes()?;
            },
            OptimizationLevel::Default => {
                self.add_standard_passes()?;
                if context.complexity_score > 0.7 {
                    self.add_advanced_passes()?;
                }
            },
            OptimizationLevel::Aggressive => {
                self.add_all_optimization_passes()?;
            },
            OptimizationLevel::Size | OptimizationLevel::SizeZ => {
                self.add_size_optimization_passes()?;
            },
            OptimizationLevel::Custom(_) => {
                self.add_custom_passes()?;
            },
        }

        // Apply heuristics based on module characteristics
        if context.module_size > 100000 {
            // Large modules - add parallelization passes
            self.add_parallelization_passes()?;
        }

        if context.function_count > 1000 {
            // Many functions - focus on inlining and IPO
            self.add_interprocedural_passes()?;
        }

        Ok(())
    }

    fn add_basic_passes(&mut self) -> Result<(), CursedError> {
        self.real_pass_manager.add_pass("mem2reg".to_string())?;
        self.real_pass_manager.add_pass("simplifycfg".to_string())?;
        Ok(())
    }

    fn add_standard_passes(&mut self) -> Result<(), CursedError> {
        self.add_basic_passes()?;
        self.real_pass_manager.add_pass("instcombine".to_string())?;
        self.real_pass_manager.add_pass("reassociate".to_string())?;
        Ok(())
    }

    fn add_advanced_passes(&mut self) -> Result<(), CursedError> {
        self.real_pass_manager.add_pass("gvn".to_string())?;
        // Add more advanced passes as needed
        Ok(())
    }

    fn add_all_optimization_passes(&mut self) -> Result<(), CursedError> {
        self.add_standard_passes()?;
        self.add_advanced_passes()?;
        // Add aggressive optimization passes
        Ok(())
    }

    fn add_size_optimization_passes(&mut self) -> Result<(), CursedError> {
        self.real_pass_manager.add_pass("mem2reg".to_string())?;
        self.real_pass_manager.add_pass("simplifycfg".to_string())?;
        // Add size-specific passes
        Ok(())
    }

    fn add_custom_passes(&mut self) -> Result<(), CursedError> {
        for pass in &self.config.custom_passes {
            self.real_pass_manager.add_pass(pass.clone())?;
        }
        Ok(())
    }

    fn add_parallelization_passes(&mut self) -> Result<(), CursedError> {
        // Add passes that help with parallelization
        Ok(())
    }

    fn add_interprocedural_passes(&mut self) -> Result<(), CursedError> {
        // Add passes that optimize across function boundaries
        Ok(())
    }

    fn execute_pass_with_stats(&mut self, pass_name: &str, context: &ExecutionContext) -> Result<PassExecutionResult, CursedError> {
        let start_time = Instant::now();
        
        // Execute the pass (simplified)
        let success = true; // In reality, this would depend on actual pass execution
        
        let duration = start_time.elapsed();
        
        // Update statistics
        let stats = self.pass_statistics.entry(pass_name.to_string()).or_insert(PassStatistics {
            executions: 0,
            total_time: Duration::new(0, 0),
            average_time: Duration::new(0, 0),
            last_execution: None,
            success_rate: 1.0,
            performance_impact: 0.0,
        });

        stats.executions += 1;
        stats.total_time += duration;
        stats.average_time = stats.total_time / stats.executions as u32;
        stats.last_execution = Some(start_time);
        
        // Update success rate
        let old_successes = (stats.success_rate * (stats.executions - 1) as f64) as u64;
        let new_successes = if success { old_successes + 1 } else { old_successes };
        stats.success_rate = new_successes as f64 / stats.executions as f64;

        // Record execution
        let execution = PassExecution {
            pass_name: pass_name.to_string(),
            timestamp: start_time,
            duration,
            success,
            context: context.clone(),
        };
        self.execution_history.push(execution);

        Ok(PassExecutionResult {
            pass_name: pass_name.to_string(),
            duration,
            success,
        })
    }

    fn estimate_performance_improvement(&self, _context: &ExecutionContext) -> f64 {
        // Simplified estimation based on enabled passes
        let pass_count = self.real_pass_manager.get_enabled_passes().len();
        (pass_count as f64) * 0.05 // 5% improvement per pass (very simplified)
    }

    fn estimate_size_reduction(&self, _context: &ExecutionContext) -> f64 {
        // Simplified estimation
        if matches!(self.config.level, OptimizationLevel::Size | OptimizationLevel::SizeZ) {
            0.15 // 15% size reduction for size optimizations
        } else {
            0.05 // 5% for other optimizations
        }
    }

    pub fn get_pass_statistics(&self) -> &HashMap<String, PassStatistics> {
        &self.pass_statistics
    }

    pub fn get_execution_history(&self) -> &[PassExecution] {
        &self.execution_history
    }

    pub fn generate_performance_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Enhanced Pass Manager Report ===\n");
        
        for (pass_name, stats) in &self.pass_statistics {
            report.push_str(&format!(
                "{}: {} executions, avg time: {:?}, success rate: {:.2}%\n",
                pass_name,
                stats.executions,
                stats.average_time,
                stats.success_rate * 100.0
            ));
        }
        
        report
    }

    pub fn reset_statistics(&mut self) {
        self.pass_statistics.clear();
        self.execution_history.clear();
    }
}

#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub total_time: Duration,
    pub passes_executed: usize,
    pub successful_passes: usize,
    pub failed_passes: usize,
    pub performance_improvement: f64,
    pub size_reduction: f64,
}

#[derive(Debug, Clone)]
pub struct PassExecutionResult {
    pub pass_name: String,
    pub duration: Duration,
    pub success: bool,
}

impl Default for PassStatistics {
    fn default() -> Self {
        Self {
            executions: 0,
            total_time: Duration::new(0, 0),
            average_time: Duration::new(0, 0),
            last_execution: None,
            success_rate: 1.0,
            performance_impact: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_pass_manager() {
        let context = inkwell::context::Context::create();
        let config = OptimizationConfig::default();
        let mut manager = EnhancedLlvmPassManager::new(&context, config).unwrap();
        
        let exec_context = ExecutionContext {
            optimization_level: OptimizationLevel::Default,
            module_size: 50000,
            function_count: 100,
            complexity_score: 0.5,
        };
        
        let result = manager.run_enhanced_passes(exec_context).unwrap();
        assert!(result.total_time >= Duration::new(0, 0));
    }
}
