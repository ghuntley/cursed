/// Integration tests for the LLVM Optimization System
/// 
/// Tests the complete integration of optimization passes, pipelines, and 
/// performance monitoring with the CURSED compiler.

use cursed::codegen::llvm::{
    LlvmCodeGenerator, OptimizationEngine, OptimizationEngineConfig, 
    PassConfiguration, OptimizationLevel as PassOptLevel,
    PassRegistry, PipelineBuilder, PerformanceMonitor, MonitoringConfig,
    DeadCodeEliminationPass, ConstantPropagationPass
};
use cursed::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn test_optimization_engine_integration() {
    // Create LLVM context and module for testing
    let context = inkwell::context::Context::create();
    let module = context.create_module("test_module");
    
    // Create optimization engine configuration
    let config = OptimizationEngineConfig {
        optimization_level: PassOptLevel::Default,
        enable_performance_monitoring: true,
        optimization_time_budget: Duration::from_secs(5),
        ..OptimizationEngineConfig::default()
    };
    
    // Create optimization engine
    let result = OptimizationEngine::new(&context, config);
    assert!(result.is_ok(), "Failed to create optimization engine: {:?}", result.err());
    
    let mut engine = result.unwrap();
    
    // Test optimization of a simple module
    let optimization_result = engine.optimize_module(&module);
    assert!(optimization_result.is_ok(), "Optimization failed: {:?}", optimization_result.err());
    
    let result = optimization_result.unwrap();
    // The module is empty, so optimization should succeed but not change anything
    assert!(result.success);
}

#[test]
fn test_pass_registry_functionality() {
    let config = PassConfiguration::default();
    let mut registry = PassRegistry::new(config.clone());
    
    // Register dead code elimination pass
    let dce_pass = DeadCodeEliminationPass::new(config.clone());
    let result = registry.register_pass(dce_pass);
    assert!(result.is_ok(), "Failed to register DCE pass: {:?}", result.err());
    
    // Register constant propagation pass
    let cp_pass = ConstantPropagationPass::new(config);
    let result = registry.register_pass(cp_pass);
    assert!(result.is_ok(), "Failed to register CP pass: {:?}", result.err());
    
    // Check registered passes
    let registered_passes = registry.get_registered_passes();
    assert_eq!(registered_passes.len(), 2);
    assert!(registered_passes.contains(&"dead_code_elimination".to_string()));
    assert!(registered_passes.contains(&"constant_propagation".to_string()));
    
    // Validate dependencies
    let issues = registry.validate_dependencies().unwrap();
    assert!(issues.is_empty(), "Dependency validation failed: {:?}", issues);
}

#[test]
fn test_optimization_pipeline_builder() {
    let config = PassConfiguration::default();
    let registry = Arc::new(Mutex::new(PassRegistry::new(config.clone())));
    
    // Build pipeline using builder pattern
    let pipeline = PipelineBuilder::new(registry, config)
        .with_optimization_level(PassOptLevel::Default)
        .build();
    
    // Check that pipeline has stages
    assert!(!pipeline.get_execution_order().is_empty());
    
    // Test pipeline estimation
    let estimated_time = pipeline.estimate_execution_time();
    assert!(estimated_time > Duration::from_secs(0));
}

#[test]
fn test_performance_monitoring() {
    let config = MonitoringConfig::default();
    let mut monitor = PerformanceMonitor::new(config);
    
    // Test monitoring functionality
    assert_eq!(monitor.get_all_metrics().len(), 0);
    
    // Generate a performance report
    let result = monitor.generate_report();
    assert!(result.is_ok(), "Failed to generate performance report: {:?}", result.err());
    
    let report = result.unwrap();
    assert_eq!(report.total_passes_monitored, 0);
    assert_eq!(report.total_executions, 0);
}

#[test]
fn test_code_generator_optimization_integration() {
    let result = LlvmCodeGenerator::new();
    assert!(result.is_ok(), "Failed to create code generator: {:?}", result.err());
    
    let mut generator = result.unwrap();
    
    // Test optimization configuration
    let opt_config = OptimizationEngineConfig::for_development();
    let result = generator.set_optimization_config(opt_config);
    assert!(result.is_ok(), "Failed to set optimization config: {:?}", result.err());
    
    // Test optimization enable/disable
    assert!(generator.optimization_enabled());
    generator.set_optimization_enabled(false);
    assert!(!generator.optimization_enabled());
    generator.set_optimization_enabled(true);
    assert!(generator.optimization_enabled());
    
    // Test getting statistics (should be None initially)
    let stats = generator.get_optimization_statistics();
    assert!(stats.is_some());
}

#[test]
fn test_optimization_levels() {
    // Test optimization level configurations
    let dev_config = OptimizationEngineConfig::for_development();
    assert_eq!(dev_config.optimization_level, PassOptLevel::Basic);
    assert!(!dev_config.enable_aggressive_optimizations);
    
    let release_config = OptimizationEngineConfig::for_release();
    assert_eq!(release_config.optimization_level, PassOptLevel::Aggressive);
    assert!(release_config.enable_aggressive_optimizations);
    
    let size_config = OptimizationEngineConfig::for_size();
    assert_eq!(size_config.optimization_level, PassOptLevel::Size);
    assert!(size_config.enable_size_optimizations);
}

#[test]
fn test_pass_execution_context() {
    use cursed::codegen::llvm::passes::{PassExecutionContext, PassConfiguration, OptimizationLevel};
    
    let config = PassConfiguration {
        optimization_level: OptimizationLevel::Default,
        time_budget: Duration::from_secs(10),
        ..PassConfiguration::default()
    };
    
    let mut context = PassExecutionContext::new(&config);
    
    // Test time budget management
    assert_eq!(context.passes_executed, 0);
    assert_eq!(context.remaining_time, Duration::from_secs(10));
    
    let pass_time = Duration::from_millis(500);
    assert!(context.has_time_for_pass(pass_time));
    
    context.set_current_pass("test_pass".to_string());
    assert_eq!(context.current_pass, Some("test_pass".to_string()));
    
    context.update_after_pass(pass_time);
    assert_eq!(context.passes_executed, 1);
    assert!(context.remaining_time < Duration::from_secs(10));
}

#[test]
fn test_module_complexity_estimation() {
    use cursed::codegen::llvm::passes::utils;
    
    let context = inkwell::context::Context::create();
    let module = context.create_module("test_module");
    
    // Add a simple function to the module
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    
    // Create a basic block
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Add a return instruction
    let zero = i32_type.const_zero();
    builder.build_return(Some(&zero)).unwrap();
    
    // Test complexity estimation
    let complexity = utils::estimate_module_complexity(&module);
    assert_eq!(complexity.function_count, 1);
    assert!(complexity.instruction_count > 0);
    assert!(complexity.basic_block_count > 0);
    assert!(complexity.estimated_optimization_time > Duration::from_secs(0));
    
    // Test complexity score calculation
    let score = complexity.complexity_score();
    assert!(score > 0.0);
    
    // Test large module detection
    assert!(!complexity.is_large_module()); // Single function should not be considered large
}

#[test]
fn test_error_handling_in_optimization() {
    // Test error handling when optimization engine is not initialized properly
    let result = LlvmCodeGenerator::new();
    assert!(result.is_ok());
    
    let mut generator = result.unwrap();
    
    // Disable optimization to test fallback behavior
    generator.set_optimization_enabled(false);
    assert!(!generator.optimization_enabled());
    
    // Test that compilation still works without optimization
    let test_source = "slay main() normie { return 0; }";
    let ir_result = generator.generate_ir(test_source);
    assert!(ir_result.is_ok(), "IR generation failed: {:?}", ir_result.err());
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_end_to_end_optimization() {
        // Create a more comprehensive test that exercises the entire optimization pipeline
        let context = inkwell::context::Context::create();
        let module = context.create_module("comprehensive_test");
        
        // Create a function with some code that can be optimized
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function("add_one", fn_type, None);
        
        let entry_bb = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(entry_bb);
        
        // Get function parameter
        let param = function.get_nth_param(0).unwrap().into_int_value();
        
        // Add 1 to parameter
        let one = i32_type.const_int(1, false);
        let result = builder.build_int_add(param, one, "add_result").unwrap();
        
        // Return result
        builder.build_return(Some(&result)).unwrap();
        
        // Create optimization engine with aggressive settings
        let config = OptimizationEngineConfig::for_release();
        let mut engine = OptimizationEngine::new(&context, config).unwrap();
        
        // Set baseline metrics
        engine.set_baseline_metrics(&module);
        
        // Run optimization
        let result = engine.optimize_module(&module);
        assert!(result.is_ok(), "Optimization failed: {:?}", result.err());
        
        let optimization_result = result.unwrap();
        assert!(optimization_result.success);
        
        // Get performance report
        let report_result = engine.get_performance_report();
        assert!(report_result.is_ok(), "Failed to get performance report: {:?}", report_result.err());
        
        let report = report_result.unwrap();
        // Report should have some baseline comparison since we set baseline metrics
        assert!(report.baseline_comparison.is_some());
        
        // Get engine statistics
        let stats = engine.get_statistics();
        assert!(stats.total_optimizations > 0);
    }
}
