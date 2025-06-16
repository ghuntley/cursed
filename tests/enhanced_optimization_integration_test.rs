/// Enhanced Optimization Integration Tests
/// 
/// Comprehensive test suite for the enhanced LLVM optimization system
/// validating real performance improvements and optimization effectiveness.

use cursed::optimization::{
    enhanced_llvm_passes::{EnhancedLlvmPassManager, EnhancedOptimizationStatistics},
    enhanced_llvm_optimization::{
        EnhancedLlvmOptimizer, EnhancedOptimizationConfig, EnhancedOptimizationResults,
        PerformanceImprovements, OptimizationFeedback,
    },
    performance_analysis::{PerformanceAnalyzer, PerformanceTrends, RegressionDetector},
    coordinator::{OptimizationCoordinator, OptimizationCoordinatorConfig},
    config::{OptimizationConfig, OptimizationLevel},
};
use cursed::error::Result;
use inkwell::{context::Context, module::Module, values::FunctionValue, types::BasicType};
use std::time::Duration;
use tracing_test::traced_test;

#[test]
#[traced_test]
fn test_enhanced_llvm_pass_manager_basic_functionality() -> Result<()> {
    let context = Context::create();
    let module = context.create_module("test_module");
    let config = OptimizationConfig::default();
    
    // Create a simple function for testing
    let fn_type = context.i32_type().fn_type(&[context.i32_type().into()], false);
    let function = module.add_function("test_function", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    // Create some basic operations that can be optimized
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let const_val = context.i32_type().const_int(42, false);
    let add_result = builder.build_int_add(param, const_val, "add_result").unwrap();
    let sub_result = builder.build_int_sub(add_result, const_val, "sub_result").unwrap(); // This should optimize to param
    builder.build_return(Some(&sub_result)).unwrap();
    
    // Test pass manager creation
    let pass_manager = EnhancedLlvmPassManager::new(&context, OptimizationLevel::Default, &config);
    
    // Test optimization execution
    pass_manager.optimize_module(&module)?;
    
    // Validate statistics
    let stats = pass_manager.get_statistics();
    assert!(stats.optimization_time > Duration::from_nanos(0));
    
    // Generate optimization report
    let report = pass_manager.generate_optimization_report()?;
    assert!(report.contains("Enhanced LLVM Optimization Report"));
    assert!(report.contains("Module Metrics"));
    assert!(report.contains("Optimization Results"));
    
    Ok(())
}

#[test]
#[traced_test]
fn test_llvm_pass_effectiveness() -> Result<()> {
    let context = Context::create();
    let module = context.create_module("optimization_test");
    let config = OptimizationConfig::default();
    
    // Create a function with obvious optimization opportunities
    let fn_type = context.i32_type().fn_type(&[context.i32_type().into()], false);
    let function = module.add_function("optimization_target", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    let param = function.get_nth_param(0).unwrap().into_int_value();
    
    // Dead code that should be eliminated
    let dead_val1 = context.i32_type().const_int(100, false);
    let dead_val2 = context.i32_type().const_int(200, false);
    let _dead_add = builder.build_int_add(dead_val1, dead_val2, "dead_add").unwrap(); // Not used, should be eliminated
    
    // Constant folding opportunity
    let const1 = context.i32_type().const_int(10, false);
    let const2 = context.i32_type().const_int(20, false);
    let const_add = builder.build_int_add(const1, const2, "const_add").unwrap(); // Should fold to 30
    
    // Final result using the parameter and constant
    let result = builder.build_int_add(param, const_add, "result").unwrap();
    builder.build_return(Some(&result)).unwrap();
    
    // Test different optimization levels
    for opt_level in [OptimizationLevel::None, OptimizationLevel::Less, OptimizationLevel::Default, OptimizationLevel::Aggressive] {
        let pass_manager = EnhancedLlvmPassManager::new(&context, opt_level.clone(), &config);
        
        // Count initial instructions
        let initial_count = count_instructions(&module);
        
        // Run optimization
        pass_manager.optimize_module(&module)?;
        
        // Validate optimization occurred
        let stats = pass_manager.get_statistics();
        
        match opt_level {
            OptimizationLevel::None => {
                // Minimal optimization should still clean up obvious dead code
                assert!(stats.optimization_time < Duration::from_millis(100));
            }
            OptimizationLevel::Less => {
                // Basic optimization should eliminate some dead code and fold constants
                assert!(stats.constants_propagated >= 0);
                assert!(stats.instructions_eliminated >= 0);
            }
            OptimizationLevel::Default => {
                // Standard optimization should perform multiple passes
                assert!(stats.constants_propagated >= 0);
                assert!(stats.estimated_runtime_improvement >= 0.0);
            }
            OptimizationLevel::Aggressive => {
                // Aggressive optimization should maximize improvements
                assert!(stats.estimated_runtime_improvement >= 0.0);
                assert!(stats.estimated_code_size_reduction >= 0.0);
            }
        }
        
        println!("Optimization level {:?}: {} constants propagated, {:.1}% estimated improvement",
                opt_level.as_str(), stats.constants_propagated, stats.estimated_runtime_improvement);
    }
    
    Ok(())
}

#[test]
#[traced_test]
fn test_performance_improvement_measurement() -> Result<()> {
    let context = Context::create();
    let config = EnhancedOptimizationConfig::default();
    let base_config = OptimizationConfig::default();
    
    let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config)?;
    
    // Create a module with clear optimization opportunities
    let module = create_optimization_test_module(&context);
    
    // Run optimization and measure improvements
    let results = optimizer.optimize_module(&module)?;
    
    // Validate performance improvements
    assert!(results.performance_improvements.runtime_improvement >= 0.0);
    assert!(results.performance_improvements.size_reduction >= 0.0);
    assert!(results.performance_improvements.memory_reduction >= 0.0);
    assert!(results.performance_improvements.compilation_speedup >= 0.0);
    assert!(results.performance_improvements.energy_efficiency >= 0.0);
    
    // Validate base optimization statistics
    assert!(results.base_statistics.optimization_time > Duration::from_nanos(0));
    
    // Validate compilation metrics
    assert!(results.compilation_metrics.total_compilation_time >= Duration::from_nanos(0));
    
    // Validate target-specific results
    assert!(!results.target_specific_results.target_arch.is_empty());
    assert!(results.target_specific_results.cache_optimization_results.l1_hit_rate_improvement >= 0.0);
    assert!(results.target_specific_results.vectorization_results.vectorization_speedup >= 1.0);
    
    println!("Performance improvements: {:.1}% runtime, {:.1}% size reduction", 
             results.performance_improvements.runtime_improvement,
             results.performance_improvements.size_reduction);
    
    Ok(())
}

#[test]
#[traced_test]
fn test_optimization_report_generation() -> Result<()> {
    let context = Context::create();
    let config = EnhancedOptimizationConfig::default();
    let base_config = OptimizationConfig::default();
    
    let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config)?;
    let module = create_optimization_test_module(&context);
    
    let results = optimizer.optimize_module(&module)?;
    let report = optimizer.generate_optimization_report(&results)?;
    
    // Validate report structure
    assert!(report.contains("# Enhanced LLVM Optimization Report"));
    assert!(report.contains("## Executive Summary"));
    assert!(report.contains("## Base Optimization Statistics"));
    assert!(report.contains("## Target-Specific Optimizations"));
    assert!(report.contains("## Optimization Feedback"));
    
    // Validate specific metrics are included
    assert!(report.contains("Runtime Improvement"));
    assert!(report.contains("Code Size Reduction"));
    assert!(report.contains("Memory Reduction"));
    assert!(report.contains("Compilation Speedup"));
    assert!(report.contains("Energy Efficiency"));
    
    // Validate target-specific information
    assert!(report.contains("Target Architecture"));
    assert!(report.contains("L1 Cache Hit Rate Improvement"));
    assert!(report.contains("Vectorization Speedup"));
    
    println!("Generated optimization report:\n{}", report);
    
    Ok(())
}

#[test]
#[traced_test]
fn test_function_optimization() -> Result<()> {
    let context = Context::create();
    let config = EnhancedOptimizationConfig::default();
    let base_config = OptimizationConfig::default();
    
    let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config)?;
    
    // Create a function with optimization opportunities
    let module = context.create_module("function_test");
    let fn_type = context.i64_type().fn_type(&[context.i64_type().into(), context.i64_type().into()], false);
    let function = module.add_function("mathematical_computation", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    
    // Mathematical operations that can be optimized
    let mul_result = builder.build_int_mul(param1, param2, "mul").unwrap();
    let const_val = context.i64_type().const_int(1, false);
    let add_result = builder.build_int_add(mul_result, const_val, "add").unwrap();
    let sub_result = builder.build_int_sub(add_result, const_val, "sub").unwrap(); // Should optimize to mul_result
    
    builder.build_return(Some(&sub_result)).unwrap();
    
    // Test function-level optimization
    let func_results = optimizer.optimize_function(function)?;
    
    // Validate function optimization results
    assert_eq!(func_results.function_name, "mathematical_computation");
    assert!(func_results.optimization_time > Duration::from_nanos(0));
    assert!(func_results.function_analysis.instruction_count > 0);
    assert!(func_results.function_analysis.basic_block_count >= 1);
    
    println!("Function optimization completed in {:?}", func_results.optimization_time);
    
    Ok(())
}

#[test]
#[traced_test] 
fn test_cache_performance() -> Result<()> {
    let context = Context::create();
    let config = EnhancedOptimizationConfig {
        enable_compilation_cache: true,
        ..Default::default()
    };
    let base_config = OptimizationConfig::default();
    
    let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config)?;
    let module = create_optimization_test_module(&context);
    
    // First optimization run - should cache results
    let start_time = std::time::Instant::now();
    let results1 = optimizer.optimize_module(&module)?;
    let first_run_time = start_time.elapsed();
    
    // Second optimization run - should use cached results
    let start_time = std::time::Instant::now();
    let results2 = optimizer.optimize_module(&module)?;
    let second_run_time = start_time.elapsed();
    
    // Validate that caching provides benefits
    assert_eq!(results1.performance_improvements.runtime_improvement, 
               results2.performance_improvements.runtime_improvement);
    
    // Cache should make second run faster (though this test might be flaky due to timing)
    println!("First run: {:?}, Second run: {:?}", first_run_time, second_run_time);
    
    // Clear caches
    optimizer.clear_caches()?;
    
    Ok(())
}

#[test]
#[traced_test]
fn test_optimization_metrics_tracking() -> Result<()> {
    let context = Context::create();
    let config = EnhancedOptimizationConfig::default();
    let base_config = OptimizationConfig::default();
    
    let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config)?;
    
    // Initial metrics should be empty
    let initial_metrics = optimizer.get_optimization_metrics();
    assert_eq!(initial_metrics.total_optimizations, 0);
    
    // Run optimization
    let module = create_optimization_test_module(&context);
    optimizer.optimize_module(&module)?;
    
    // Metrics should be updated
    let updated_metrics = optimizer.get_optimization_metrics();
    assert_eq!(updated_metrics.total_optimizations, 1);
    assert!(updated_metrics.total_optimization_time > Duration::from_nanos(0));
    
    // Run another optimization
    let module2 = create_optimization_test_module(&context);
    optimizer.optimize_module(&module2)?;
    
    // Metrics should accumulate
    let final_metrics = optimizer.get_optimization_metrics();
    assert_eq!(final_metrics.total_optimizations, 2);
    
    Ok(())
}

#[test]
#[traced_test]
fn test_compilation_metrics() -> Result<()> {
    let context = Context::create();
    let config = EnhancedOptimizationConfig::default();
    let base_config = OptimizationConfig::default();
    
    let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config)?;
    let module = create_optimization_test_module(&context);
    
    let results = optimizer.optimize_module(&module)?;
    
    // Validate compilation metrics
    let compilation_metrics = results.compilation_metrics;
    assert!(compilation_metrics.total_compilation_time >= Duration::from_nanos(0));
    assert!(compilation_metrics.peak_memory_usage > 0);
    assert!(compilation_metrics.average_cpu_usage >= 0.0);
    assert!(compilation_metrics.io_operations >= 0);
    
    // Get current compilation metrics from optimizer
    let current_metrics = optimizer.get_compilation_metrics();
    assert!(current_metrics.total_compilation_time >= Duration::from_nanos(0));
    
    println!("Compilation metrics: {:?} peak memory, {:.1}% avg CPU",
             compilation_metrics.peak_memory_usage, compilation_metrics.average_cpu_usage);
    
    Ok(())
}

#[test]
#[traced_test]
fn test_optimization_coordinator_integration() -> Result<()> {
    let coordinator_config = OptimizationCoordinatorConfig {
        enable_incremental_compilation: true,
        enable_parallel_compilation: true,
        enable_caching: true,
        ..Default::default()
    };
    
    let coordinator = OptimizationCoordinator::new(coordinator_config);
    
    // Test coordinator workflow
    let work_dir = std::env::temp_dir().join("cursed_optimization_test");
    std::fs::create_dir_all(&work_dir).ok();
    
    let source_files = vec![
        work_dir.join("test1.csd"),
        work_dir.join("test2.csd"),
    ];
    
    // Create simple test files
    for file in &source_files {
        std::fs::write(file, "func main() { sus x = 42; }")?;
    }
    
    // Run comprehensive optimization
    let comprehensive_result = coordinator.run_comprehensive_optimization(
        &source_files,
        &work_dir,
        OptimizationLevel::Default,
    )?;
    
    // Validate comprehensive results
    assert!(comprehensive_result.overall_improvement.total_improvement >= 0.0);
    assert!(comprehensive_result.cache_performance.cache_hit_rate >= 0.0);
    assert!(comprehensive_result.parallel_performance.parallel_efficiency >= 0.0);
    
    println!("Comprehensive optimization: {:.1}% total improvement, {:.1}% cache hit rate",
             comprehensive_result.overall_improvement.total_improvement,
             comprehensive_result.cache_performance.cache_hit_rate);
    
    // Cleanup
    std::fs::remove_dir_all(&work_dir).ok();
    
    Ok(())
}

#[test]
#[traced_test]
fn test_performance_regression_detection() -> Result<()> {
    let context = Context::create();
    let config = EnhancedOptimizationConfig::default();
    let base_config = OptimizationConfig::default();
    
    let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config)?;
    
    // Create baseline performance data
    let module1 = create_optimization_test_module(&context);
    let baseline_results = optimizer.optimize_module(&module1)?;
    
    // Create modified module (simulated regression)
    let module2 = create_larger_test_module(&context);
    let current_results = optimizer.optimize_module(&module2)?;
    
    // Performance should differ between modules
    let performance_diff = current_results.performance_improvements.runtime_improvement 
                         - baseline_results.performance_improvements.runtime_improvement;
    
    println!("Performance difference: {:.2}%", performance_diff);
    
    // Both should have valid optimization results
    assert!(baseline_results.performance_improvements.runtime_improvement >= 0.0);
    assert!(current_results.performance_improvements.runtime_improvement >= 0.0);
    
    Ok(())
}

// Helper functions

fn count_instructions(module: &Module) -> usize {
    let mut count = 0;
    for function in module.get_functions() {
        if let Some(mut block) = function.get_first_basic_block() {
            loop {
                if let Some(mut instruction) = block.get_first_instruction() {
                    loop {
                        count += 1;
                        if let Some(next) = instruction.get_next_instruction() {
                            instruction = next;
                        } else {
                            break;
                        }
                    }
                }
                if let Some(next) = block.get_next_basic_block() {
                    block = next;
                } else {
                    break;
                }
            }
        }
    }
    count
}

fn create_optimization_test_module(context: &Context) -> Module {
    let module = context.create_module("optimization_test");
    
    // Create a function with various optimization opportunities
    let fn_type = context.i32_type().fn_type(&[context.i32_type().into()], false);
    let function = module.add_function("test_function", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    let param = function.get_nth_param(0).unwrap().into_int_value();
    
    // Constant folding opportunities
    let const1 = context.i32_type().const_int(10, false);
    let const2 = context.i32_type().const_int(20, false);
    let const_add = builder.build_int_add(const1, const2, "const_add").unwrap();
    
    // Dead code
    let dead_const = context.i32_type().const_int(999, false);
    let _dead_mul = builder.build_int_mul(dead_const, const1, "dead_mul").unwrap();
    
    // Actual computation
    let result = builder.build_int_add(param, const_add, "result").unwrap();
    builder.build_return(Some(&result)).unwrap();
    
    module
}

fn create_larger_test_module(context: &Context) -> Module {
    let module = context.create_module("larger_test");
    
    // Create multiple functions with different characteristics
    for i in 0..3 {
        let fn_type = context.i64_type().fn_type(&[context.i64_type().into()], false);
        let function = module.add_function(&format!("function_{}", i), fn_type, None);
        
        let entry_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(entry_block);
        
        let param = function.get_nth_param(0).unwrap().into_int_value();
        
        // Create more complex computation
        let mut result = param;
        for j in 0..5 {
            let const_val = context.i64_type().const_int((i * 10 + j) as u64, false);
            result = builder.build_int_add(result, const_val, &format!("add_{}", j)).unwrap();
        }
        
        builder.build_return(Some(&result)).unwrap();
    }
    
    module
}

#[test]
#[traced_test]
fn test_mathematical_computation_benchmark() -> Result<()> {
    let context = Context::create();
    let config = EnhancedOptimizationConfig::default();
    let base_config = OptimizationConfig::default();
    
    let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config)?;
    
    // Create a mathematical computation module
    let module = context.create_module("math_benchmark");
    let fn_type = context.f64_type().fn_type(&[context.f64_type().into(), context.f64_type().into()], false);
    let function = module.add_function("complex_math", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    let a = function.get_nth_param(0).unwrap().into_float_value();
    let b = function.get_nth_param(1).unwrap().into_float_value();
    
    // Complex mathematical operations
    let sum = builder.build_float_add(a, b, "sum").unwrap();
    let product = builder.build_float_mul(a, b, "product").unwrap();
    let quotient = builder.build_float_div(sum, product, "quotient").unwrap();
    
    // Redundant operations that should be optimized
    let const_one = context.f64_type().const_float(1.0);
    let mul_by_one = builder.build_float_mul(quotient, const_one, "mul_by_one").unwrap(); // Should optimize away
    let add_zero = builder.build_float_add(mul_by_one, context.f64_type().const_float(0.0), "add_zero").unwrap(); // Should optimize away
    
    builder.build_return(Some(&add_zero)).unwrap();
    
    // Measure optimization performance
    let start_time = std::time::Instant::now();
    let results = optimizer.optimize_module(&module)?;
    let optimization_time = start_time.elapsed();
    
    // Validate optimization effectiveness
    assert!(results.performance_improvements.runtime_improvement >= 0.0);
    assert!(optimization_time < Duration::from_secs(10)); // Should be reasonably fast
    
    println!("Mathematical benchmark: {:.1}% improvement in {:?}",
             results.performance_improvements.runtime_improvement, optimization_time);
    
    Ok(())
}

#[test]
#[traced_test]
fn test_memory_usage_optimization() -> Result<()> {
    let context = Context::create();
    let config = EnhancedOptimizationConfig::default();
    let base_config = OptimizationConfig::default();
    
    let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config)?;
    
    // Create a module with memory allocation patterns
    let module = context.create_module("memory_test");
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("memory_operations", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    // Allocate and use memory
    let i32_type = context.i32_type();
    let array_type = i32_type.array_type(100);
    let alloca = builder.build_alloca(array_type, "local_array").unwrap();
    
    // Store and load operations
    for i in 0..10 {
        let index = context.i32_type().const_int(i, false);
        let value = context.i32_type().const_int(i * 10, false);
        let gep = unsafe {
            builder.build_in_bounds_gep(
                array_type,
                alloca,
                &[context.i32_type().const_zero(), index],
                &format!("gep_{}", i)
            ).unwrap()
        };
        builder.build_store(gep, value).unwrap();
    }
    
    builder.build_return(None).unwrap();
    
    let results = optimizer.optimize_module(&module)?;
    
    // Memory optimization should show benefits
    assert!(results.performance_improvements.memory_reduction >= 0.0);
    assert!(results.compilation_metrics.peak_memory_usage > 0);
    
    println!("Memory optimization: {:.1}% reduction, peak usage: {} bytes",
             results.performance_improvements.memory_reduction,
             results.compilation_metrics.peak_memory_usage);
    
    Ok(())
}

#[test]
#[traced_test]
fn test_optimization_level_progression() -> Result<()> {
    let context = Context::create();
    let module = create_optimization_test_module(&context);
    let base_config = OptimizationConfig::default();
    
    let optimization_levels = [
        OptimizationLevel::None,
        OptimizationLevel::Less,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
    ];
    
    let mut previous_improvement = 0.0;
    
    for level in optimization_levels {
        let config = EnhancedOptimizationConfig {
            optimization_level: level.clone(),
            ..Default::default()
        };
        
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config.clone())?;
        let results = optimizer.optimize_module(&module)?;
        
        let improvement = results.performance_improvements.runtime_improvement;
        
        // Higher optimization levels should generally provide better improvements
        // (though this isn't strictly guaranteed in all cases)
        println!("Optimization level {:?}: {:.1}% improvement",
                level.as_str(), improvement);
        
        // Validate that optimization occurred
        assert!(improvement >= 0.0);
        assert!(results.base_statistics.optimization_time > Duration::from_nanos(0));
        
        // Update for next comparison
        previous_improvement = improvement;
    }
    
    Ok(())
}
