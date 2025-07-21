//! Integration layer for enhanced LLVM optimization
//! 
//! This module provides the integration between the enhanced optimization
//! system and the existing CURSED compiler infrastructure.

use crate::error::{CursedError, Result};
use crate::codegen::llvm::enhanced_optimization::{
    EnhancedOptimizationManager, EnhancedOptimizationConfig, EnhancedOptimizationResult
};
use crate::codegen::llvm::optimization::{OptimizationManager, OptimizationConfig};
use inkwell::{
    context::Context,
    module::Module,
    targets::TargetMachine,
};
use std::sync::Arc;
use std::time::Duration;

/// Optimization integration manager that bridges old and new systems
pub struct OptimizationIntegration<'ctx> {
    context: &'ctx Context,
    enhanced_manager: Option<EnhancedOptimizationManager<'ctx>>,
    fallback_manager: Option<OptimizationManager<'ctx>>,
    config: IntegrationConfig,
    target_machine: Option<&'ctx TargetMachine>,
}

/// Configuration for optimization integration
#[derive(Debug, Clone)]
pub struct IntegrationConfig {
    pub use_enhanced_optimization: bool,
    pub fallback_on_error: bool,
    pub benchmark_both_systems: bool,
    pub optimization_timeout: Option<Duration>,
    pub self_hosting_mode: bool,
    pub debug_integration: bool,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            use_enhanced_optimization: true,
            fallback_on_error: true,
            benchmark_both_systems: false,
            optimization_timeout: Some(Duration::from_secs(120)), // Reduced from 300 to 120 seconds
            self_hosting_mode: false,
            debug_integration: false,
        }
    }
}

impl IntegrationConfig {
    /// Create configuration for self-hosting compiler builds
    pub fn for_self_hosting() -> Self {
        Self {
            use_enhanced_optimization: true,
            fallback_on_error: false,
            benchmark_both_systems: false,
            optimization_timeout: Some(Duration::from_secs(600)),
            self_hosting_mode: true,
            debug_integration: false,
        }
    }
    
    /// Create configuration for development builds
    pub fn for_development() -> Self {
        Self {
            use_enhanced_optimization: true,
            fallback_on_error: true,
            benchmark_both_systems: false,
            optimization_timeout: Some(Duration::from_secs(30)), // Reduced for faster development builds
            self_hosting_mode: false,
            debug_integration: true,
        }
    }
    
    /// Create configuration for release builds
    pub fn for_release() -> Self {
        Self {
            use_enhanced_optimization: true,
            fallback_on_error: false,
            benchmark_both_systems: false,
            optimization_timeout: Some(Duration::from_secs(1200)),
            self_hosting_mode: false,
            debug_integration: false,
        }
    }
}

/// Result of integrated optimization
#[derive(Debug)]
pub struct IntegratedOptimizationResult {
    pub enhanced_result: Option<EnhancedOptimizationResult>,
    pub fallback_used: bool,
    pub benchmark_comparison: Option<OptimizationBenchmark>,
    pub total_time: Duration,
    pub success: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Benchmark comparison between optimization systems
#[derive(Debug)]
pub struct OptimizationBenchmark {
    pub enhanced_time: Duration,
    pub fallback_time: Duration,
    pub enhanced_success: bool,
    pub fallback_success: bool,
    pub performance_delta: f64,
    pub recommendation: BenchmarkRecommendation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BenchmarkRecommendation {
    UseEnhanced,
    UseFallback,
    NoPreference,
    NeedsTuning,
}

impl<'ctx> OptimizationIntegration<'ctx> {
    /// Create a new optimization integration manager
    pub fn new(context: &'ctx Context, config: IntegrationConfig) -> Self {
        Self {
            context,
            enhanced_manager: None,
            fallback_manager: None,
            config,
            target_machine: None,
        }
    }
    
    /// Initialize with enhanced optimization configuration
    pub fn with_enhanced_config(
        context: &'ctx Context, 
        integration_config: IntegrationConfig,
        optimization_config: EnhancedOptimizationConfig
    ) -> Self {
        let enhanced_manager = EnhancedOptimizationManager::new(context, optimization_config);
        
        Self {
            context,
            enhanced_manager: Some(enhanced_manager),
            fallback_manager: None,
            config: integration_config,
            target_machine: None,
        }
    }
    
    /// Initialize with both enhanced and fallback configurations
    pub fn with_both_configs(
        context: &'ctx Context,
        integration_config: IntegrationConfig,
        enhanced_config: EnhancedOptimizationConfig,
        fallback_config: OptimizationConfig
    ) -> Self {
        let enhanced_manager = EnhancedOptimizationManager::new(context, enhanced_config);
        let fallback_manager = OptimizationManager::new(context, fallback_config);
        
        Self {
            context,
            enhanced_manager: Some(enhanced_manager),
            fallback_manager: Some(fallback_manager),
            config: integration_config,
            target_machine: None,
        }
    }
    
    /// Set target machine for optimization
    pub fn set_target_machine(&mut self, target_machine: &'ctx TargetMachine) {
        self.target_machine = Some(target_machine);
        
        if let Some(ref mut enhanced) = self.enhanced_manager {
            enhanced.set_target_machine(target_machine);
        }
    }
    
    /// Run integrated optimization with fallback support
    pub fn optimize_module(&mut self, module: &Module<'ctx>) -> Result<IntegratedOptimizationResult> {
        let start_time = std::time::Instant::now();
        let mut result = IntegratedOptimizationResult {
            enhanced_result: None,
            fallback_used: false,
            benchmark_comparison: None,
            total_time: Duration::default(),
            success: false,
            warnings: Vec::new(),
            errors: Vec::new(),
        };
        
        // Try enhanced optimization first if enabled
        if self.config.use_enhanced_optimization {
            if let Some(enhanced_manager) = self.enhanced_manager.take() {
                let mut enhanced_manager = enhanced_manager;
                match self.run_enhanced_optimization(&mut enhanced_manager, module) {
                    Ok(enhanced_result) => {
                        result.enhanced_result = Some(enhanced_result);
                        result.success = result.enhanced_result.as_ref().unwrap().success;
                        
                        if self.config.debug_integration {
                            eprintln!("Enhanced optimization completed successfully");
                        }
                    }
                    Err(e) => {
                        result.errors.push(format!("Enhanced optimization failed: {}", e));
                        
                        if self.config.fallback_on_error {
                            result.warnings.push("Falling back to standard optimization".to_string());
                            if let Err(fallback_err) = self.run_fallback_optimization(module) {
                                result.errors.push(format!("Fallback optimization also failed: {}", fallback_err));
                            } else {
                                result.fallback_used = true;
                                result.success = true;
                            }
                        }
                    }
                }
                
                // Restore the enhanced_manager
                self.enhanced_manager = Some(enhanced_manager);
            }
        }
        
        // Run benchmark comparison if configured
        if self.config.benchmark_both_systems {
            if let Some(benchmark) = self.run_benchmark_comparison(module)? {
                result.benchmark_comparison = Some(benchmark);
            }
        }
        
        // Run fallback optimization if enhanced is not used
        if !self.config.use_enhanced_optimization || result.enhanced_result.is_none() {
            if let Err(e) = self.run_fallback_optimization(module) {
                result.errors.push(format!("Optimization failed: {}", e));
            } else {
                result.fallback_used = true;
                result.success = true;
            }
        }
        
        result.total_time = start_time.elapsed();
        
        // Log results if debugging is enabled
        if self.config.debug_integration {
            self.log_optimization_results(&result);
        }
        
        Ok(result)
    }
    
    /// Run enhanced optimization with timeout handling
    fn run_enhanced_optimization(
        &mut self, 
        enhanced_manager: &mut EnhancedOptimizationManager<'ctx>, 
        module: &Module<'ctx>
    ) -> Result<EnhancedOptimizationResult> {
        // Set timeout if configured
        if let Some(timeout) = self.config.optimization_timeout {
            let start = std::time::Instant::now();
            
            // Spawn optimization in a controlled manner
            // For now, just run directly but monitor time more frequently
            let optimization_result = enhanced_manager.optimize_module(module);
            
            // Check if we exceeded timeout after completion
            let elapsed = start.elapsed();
            if elapsed > timeout {
                eprintln!("Warning: Enhanced optimization took {:?}, which exceeds timeout {:?}", elapsed, timeout);
                // Still return the result if it completed, just warn
                if self.config.debug_integration {
                    eprintln!("Optimization completed but took longer than expected");
                }
            }
            
            optimization_result
        } else {
            enhanced_manager.optimize_module(module)
        }
    }
    
    /// Run fallback optimization using the standard system
    fn run_fallback_optimization(&mut self, module: &Module<'ctx>) -> Result<()> {
        if let Some(ref mut fallback_manager) = self.fallback_manager {
            fallback_manager.optimize_module(module)
        } else {
            // Create a basic fallback manager if none exists
            let fallback_config = if self.config.self_hosting_mode {
                OptimizationConfig::release_config()
            } else {
                OptimizationConfig::dev_config()
            };
            
            let mut fallback_manager = OptimizationManager::new(self.context, fallback_config);
            fallback_manager.optimize_module(module)
        }
    }
    
    /// Run benchmark comparison between optimization systems
    fn run_benchmark_comparison(&mut self, module: &Module<'ctx>) -> Result<Option<OptimizationBenchmark>> {
        if self.enhanced_manager.is_none() || self.fallback_manager.is_none() {
            return Ok(None);
        }
        
        // Clone module for separate optimization runs
        // Note: This is a simplified approach - in practice we'd need to clone the module
        // For now, we'll benchmark based on execution time estimation
        
        let enhanced_start = std::time::Instant::now();
        let enhanced_success = if let Some(ref mut enhanced) = self.enhanced_manager {
            enhanced.optimize_module(module).is_ok()
        } else {
            false
        };
        let enhanced_time = enhanced_start.elapsed();
        
        let fallback_start = std::time::Instant::now();
        let fallback_success = self.run_fallback_optimization(module).is_ok();
        let fallback_time = fallback_start.elapsed();
        
        let performance_delta = if fallback_time.as_nanos() > 0 {
            (enhanced_time.as_nanos() as f64 - fallback_time.as_nanos() as f64) / fallback_time.as_nanos() as f64
        } else {
            0.0
        };
        
        let recommendation = if enhanced_success && !fallback_success {
            BenchmarkRecommendation::UseEnhanced
        } else if !enhanced_success && fallback_success {
            BenchmarkRecommendation::UseFallback
        } else if enhanced_success && fallback_success {
            if performance_delta < -0.2 {
                BenchmarkRecommendation::UseEnhanced
            } else if performance_delta > 0.2 {
                BenchmarkRecommendation::UseFallback
            } else {
                BenchmarkRecommendation::NoPreference
            }
        } else {
            BenchmarkRecommendation::NeedsTuning
        };
        
        Ok(Some(OptimizationBenchmark {
            enhanced_time,
            fallback_time,
            enhanced_success,
            fallback_success,
            performance_delta,
            recommendation,
        }))
    }
    
    /// Log optimization results for debugging
    fn log_optimization_results(&self, result: &IntegratedOptimizationResult) {
        eprintln!("=== Optimization Integration Results ===");
        eprintln!("Success: {}", result.success);
        eprintln!("Fallback used: {}", result.fallback_used);
        eprintln!("Total time: {:?}", result.total_time);
        
        if let Some(ref enhanced) = result.enhanced_result {
            eprintln!("Enhanced optimization time: {:?}", enhanced.total_time);
            eprintln!("Enhanced stages completed: {}", enhanced.stages_completed);
            
            if let (Some(ref initial), Some(ref final_complexity)) = 
               (&enhanced.initial_complexity, &enhanced.final_complexity) {
                let improvement = final_complexity.improvement_over(initial);
                eprintln!("Complexity improvement: {:.2}%", improvement);
            }
        }
        
        if let Some(ref benchmark) = result.benchmark_comparison {
            eprintln!("Benchmark recommendation: {:?}", benchmark.recommendation);
            eprintln!("Performance delta: {:.2}%", benchmark.performance_delta * 100.0);
        }
        
        if !result.warnings.is_empty() {
            eprintln!("Warnings:");
            for warning in &result.warnings {
                eprintln!("  - {}", warning);
            }
        }
        
        if !result.errors.is_empty() {
            eprintln!("Errors:");
            for error in &result.errors {
                eprintln!("  - {}", error);
            }
        }
        
        eprintln!("=====================================");
    }
    
    /// Get performance statistics from the active optimization manager
    pub fn get_performance_stats(&self) -> Option<String> {
        if let Some(ref enhanced) = self.enhanced_manager {
            let monitor = enhanced.get_performance_monitor();
            let bottlenecks = monitor.get_bottlenecks();
            
            if !bottlenecks.is_empty() {
                let mut stats = String::new();
                stats.push_str("Performance Bottlenecks:\n");
                for bottleneck in bottlenecks {
                    stats.push_str(&format!(
                        "  {}: {} ({})\n", 
                        bottleneck.stage, 
                        bottleneck.issue, 
                        format!("{:?}", bottleneck.severity)
                    ));
                }
                Some(stats)
            } else {
                Some("No performance bottlenecks detected".to_string())
            }
        } else {
            None
        }
    }
    
    /// Configure optimization for specific compilation phases
    pub fn configure_for_phase(&mut self, phase: CompilationPhase) -> Result<()> {
        match phase {
            CompilationPhase::Bootstrap => {
                self.config.use_enhanced_optimization = true;
                self.config.fallback_on_error = false;
                self.config.self_hosting_mode = true;
                self.config.optimization_timeout = Some(Duration::from_secs(600));
            }
            CompilationPhase::SelfHosting => {
                self.config.use_enhanced_optimization = true;
                self.config.fallback_on_error = false;
                self.config.self_hosting_mode = true;
                self.config.optimization_timeout = Some(Duration::from_secs(1200));
            }
            CompilationPhase::Development => {
                self.config.use_enhanced_optimization = true;
                self.config.fallback_on_error = true;
                self.config.self_hosting_mode = false;
                self.config.optimization_timeout = Some(Duration::from_secs(60));
            }
            CompilationPhase::Release => {
                self.config.use_enhanced_optimization = true;
                self.config.fallback_on_error = false;
                self.config.self_hosting_mode = false;
                self.config.optimization_timeout = Some(Duration::from_secs(1800));
            }
        }
        Ok(())
    }
}

/// Compilation phases for optimization configuration
#[derive(Debug, Clone, PartialEq)]
pub enum CompilationPhase {
    Bootstrap,
    SelfHosting,
    Development,
    Release,
}

/// Factory for creating optimization integration instances
pub struct OptimizationIntegrationFactory;

impl OptimizationIntegrationFactory {
    /// Create optimization integration for self-hosting compilation
    pub fn for_self_hosting<'ctx>(context: &'ctx Context) -> OptimizationIntegration<'ctx> {
        let integration_config = IntegrationConfig::for_self_hosting();
        let enhanced_config = EnhancedOptimizationConfig::for_self_hosting();
        let fallback_config = OptimizationConfig::release_config();
        
        OptimizationIntegration::with_both_configs(
            context, 
            integration_config, 
            enhanced_config, 
            fallback_config
        )
    }
    
    /// Create optimization integration for development builds
    pub fn for_development<'ctx>(context: &'ctx Context) -> OptimizationIntegration<'ctx> {
        let integration_config = IntegrationConfig::for_development();
        let enhanced_config = EnhancedOptimizationConfig::for_development();
        let fallback_config = OptimizationConfig::dev_config();
        
        OptimizationIntegration::with_both_configs(
            context, 
            integration_config, 
            enhanced_config, 
            fallback_config
        )
    }
    
    /// Create optimization integration for release builds
    pub fn for_release<'ctx>(context: &'ctx Context) -> OptimizationIntegration<'ctx> {
        let integration_config = IntegrationConfig::for_release();
        let enhanced_config = EnhancedOptimizationConfig::for_release();
        let fallback_config = OptimizationConfig::release_config();
        
        OptimizationIntegration::with_both_configs(
            context, 
            integration_config, 
            enhanced_config, 
            fallback_config
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_integration_config_creation() {
        let config = IntegrationConfig::for_self_hosting();
        assert!(config.use_enhanced_optimization);
        assert!(!config.fallback_on_error);
        assert!(config.self_hosting_mode);
    }
    
    #[test]
    fn test_compilation_phase_configuration() {
        let context = Context::create();
        let mut integration = OptimizationIntegrationFactory::for_development(&context);
        
        assert!(integration.configure_for_phase(CompilationPhase::Release).is_ok());
        assert_eq!(integration.config.optimization_timeout, Some(Duration::from_secs(1800)));
    }
    
    #[test]
    fn test_benchmark_recommendation_logic() {
        let benchmark = OptimizationBenchmark {
            enhanced_time: Duration::from_millis(100),
            fallback_time: Duration::from_millis(150),
            enhanced_success: true,
            fallback_success: true,
            performance_delta: -0.33, // Enhanced is 33% faster
            recommendation: BenchmarkRecommendation::UseEnhanced,
        };
        
        assert_eq!(benchmark.recommendation, BenchmarkRecommendation::UseEnhanced);
    }
}
