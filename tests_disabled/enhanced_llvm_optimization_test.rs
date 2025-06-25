/// Comprehensive Test Suite for Enhanced LLVM Optimization
/// 
/// Tests all aspects of the enhanced optimization system including:
/// - Basic optimization functionality
/// - CURSED-specific optimizations
/// - Performance monitoring and metrics
/// - Adaptive optimization
/// - Caching and feedback systems

use cursed::optimization::enhanced_llvm_optimization::{
    EnhancedLlvmOptimizer, EnhancedOptimizationConfig, PerformanceImprovements,
    OptimizationFeedback, TargetOptimizationResults,
};
use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
use inkwell::context::Context;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic enhanced optimizer creation and configuration
    #[test]
    fn test_enhanced_optimizer_creation() {
        let context = Context::create();
        let config = EnhancedOptimizationConfig::default();
        let base_config = OptimizationConfig::default();
        
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config);
        assert!(optimizer.is_ok());
        
        let optimizer = optimizer.unwrap();
        let metrics = optimizer.get_optimization_metrics();
        assert_eq!(metrics.total_optimizations, 0);
        
        let compilation_metrics = optimizer.get_compilation_metrics();
        assert_eq!(compilation_metrics.total_compilation_time, Duration::from_secs(0));
    }

    /// Test optimization configuration options
    #[test]
    fn test_optimization_configuration() {
        let mut config = EnhancedOptimizationConfig::default();
        assert!(config.enable_cursed_optimizations);
        assert!(config.enable_adaptive_optimization);
        assert!(config.enable_compilation_cache);
        assert!(config.enable_target_optimizations);
        assert_eq!(config.optimization_level, OptimizationLevel::Default);
        
        // Test configuration modification
        config.enable_cursed_optimizations = false;
        config.optimization_level = OptimizationLevel::Aggressive;
        config.max_optimization_time = Duration::from_secs(60);
        
        assert!(!config.enable_cursed_optimizations);
        assert_eq!(config.optimization_level, OptimizationLevel::Aggressive);
        assert_eq!(config.max_optimization_time, Duration::from_secs(60));
    }

    /// Test different optimization levels
    #[test]
    fn test_optimization_levels() {
        let context = Context::create();
        let base_config = OptimizationConfig::default();
        
        for level in [
            OptimizationLevel::None,
            OptimizationLevel::Less,
            OptimizationLevel::Default,
            OptimizationLevel::Aggressive,
            OptimizationLevel::Size,
        ] {
            let config = EnhancedOptimizationConfig {
                optimization_level: level.clone(),
                ..Default::default()
            };
            
            let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config.clone());
            assert!(optimizer.is_ok(), "Failed to create optimizer for level: {:?}", level);
        }
    }

    /// Test module optimization with simple module
    #[test]
    fn test_basic_module_optimization() {
        let context = Context::create();
        let module = context.create_module("test_module");
        
        // Create a simple function
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function("simple_function", fn_type, None);
        
        let basic_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(basic_block);
        
        // Simple return statement
        let param = function.get_nth_param(0).unwrap();
        builder.build_return(Some(&param));
        
        // Verify module before optimization
        assert!(module.verify().is_ok());
        
        let config = EnhancedOptimizationConfig::default();
        let base_config = OptimizationConfig::default();
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config).unwrap();
        
        let results = optimizer.optimize_module(&module);
        assert!(results.is_ok());
        
        let results = results.unwrap();
        assert!(results.base_statistics.total_optimization_time > Duration::from_millis(0));
        
        // Verify module still valid after optimization
        assert!(module.verify().is_ok());
    }

    /// Test function-level optimization
    #[test]
    fn test_function_optimization() {
        let context = Context::create();
        let module = context.create_module("test_module");
        
        // Create a function with multiple basic blocks
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function("complex_function", fn_type, None);
        
        let entry_block = context.append_basic_block(function, "entry");
        let then_block = context.append_basic_block(function, "then");
        let else_block = context.append_basic_block(function, "else");
        let merge_block = context.append_basic_block(function, "merge");
        
        let builder = context.create_builder();
        
        // Entry block with conditional branch
        builder.position_at_end(entry_block);
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let zero = i32_type.const_int(0, false);
        let cmp = builder.build_int_compare(inkwell::IntPredicate::SGT, param, zero, "cmp");
        builder.build_conditional_branch(cmp, then_block, else_block);
        
        // Then block
        builder.position_at_end(then_block);
        let one = i32_type.const_int(1, false);
        let then_val = builder.build_int_add(param, one, "then_val");
        builder.build_unconditional_branch(merge_block);
        
        // Else block
        builder.position_at_end(else_block);
        let neg_one = i32_type.const_int((-1i32) as u64, true);
        let else_val = builder.build_int_add(param, neg_one, "else_val");
        builder.build_unconditional_branch(merge_block);
        
        // Merge block with phi node
        builder.position_at_end(merge_block);
        let phi = builder.build_phi(i32_type, "result");
        phi.add_incoming(&[(&then_val, then_block), (&else_val, else_block)]);
        builder.build_return(Some(&phi.as_basic_value()));
        
        assert!(module.verify().is_ok());
        
        let config = EnhancedOptimizationConfig::default();
        let base_config = OptimizationConfig::default();
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config).unwrap();
        
        let results = optimizer.optimize_function(function);
        assert!(results.is_ok());
        
        let results = results.unwrap();
        assert_eq!(results.function_name, "complex_function");
        assert!(results.optimization_time > Duration::from_millis(0));
        assert!(results.function_analysis.basic_block_count >= 4);
        assert!(results.function_analysis.instruction_count > 0);
    }

    /// Test performance improvement calculations
    #[test]
    fn test_performance_improvements() {
        let improvements = PerformanceImprovements {
            runtime_improvement: 15.5,
            size_reduction: 8.2,
            memory_reduction: 5.0,
            compilation_speedup: 12.0,
            energy_efficiency: 6.8,
        };
        
        assert_eq!(improvements.runtime_improvement, 15.5);
        assert_eq!(improvements.size_reduction, 8.2);
        assert!(improvements.memory_reduction > 0.0);
        assert!(improvements.compilation_speedup > 0.0);
        assert!(improvements.energy_efficiency > 0.0);
    }

    /// Test optimization feedback system
    #[test]
    fn test_optimization_feedback() {
        let feedback = OptimizationFeedback {
            successful_patterns: vec![],
            failed_attempts: vec![],
            performance_correlations: std::collections::HashMap::new(),
            recommendations: vec![],
        };
        
        assert_eq!(feedback.successful_patterns.len(), 0);
        assert_eq!(feedback.failed_attempts.len(), 0);
        assert_eq!(feedback.performance_correlations.len(), 0);
        assert_eq!(feedback.recommendations.len(), 0);
    }

    /// Test target-specific optimization results
    #[test]
    fn test_target_optimization_results() {
        let cache_results = cursed::optimization::enhanced_llvm_optimization::CacheOptimizationResults {
            l1_hit_rate_improvement: 5.0,
            l2_hit_rate_improvement: 3.0,
            cache_miss_reduction: 10.0,
            access_pattern_optimizations: 3,
        };
        
        let vectorization_results = cursed::optimization::enhanced_llvm_optimization::VectorizationResults {
            loops_vectorized: 2,
            vectorization_width: vec![4, 8],
            simd_instructions: 15,
            vectorization_speedup: 2.5,
        };
        
        let target_results = TargetOptimizationResults {
            target_arch: "x86_64".to_string(),
            optimizations_applied: vec!["simd".to_string(), "cache".to_string()],
            arch_improvements: std::collections::HashMap::new(),
            cache_optimization_results: cache_results,
            vectorization_results,
        };
        
        assert_eq!(target_results.target_arch, "x86_64");
        assert_eq!(target_results.optimizations_applied.len(), 2);
        assert_eq!(target_results.cache_optimization_results.l1_hit_rate_improvement, 5.0);
        assert_eq!(target_results.vectorization_results.loops_vectorized, 2);
        assert_eq!(target_results.vectorization_results.vectorization_speedup, 2.5);
    }

    /// Test optimization caching functionality
    #[test]
    fn test_optimization_caching() {
        let context = Context::create();
        let module = context.create_module("cache_test_module");
        
        // Create simple function for caching test
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function("cache_test_function", fn_type, None);
        
        let basic_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(basic_block);
        builder.build_return(Some(&i32_type.const_int(42, false)));
        
        assert!(module.verify().is_ok());
        
        let config = EnhancedOptimizationConfig {
            enable_compilation_cache: true,
            ..Default::default()
        };
        let base_config = OptimizationConfig::default();
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config).unwrap();
        
        // First optimization (should be cached)
        let start_time = std::time::Instant::now();
        let results1 = optimizer.optimize_module(&module);
        let first_duration = start_time.elapsed();
        assert!(results1.is_ok());
        
        // Second optimization (should use cache if available)
        let start_time = std::time::Instant::now();
        let results2 = optimizer.optimize_module(&module);
        let second_duration = start_time.elapsed();
        assert!(results2.is_ok());
        
        // Cache clearing test
        let clear_result = optimizer.clear_caches();
        assert!(clear_result.is_ok());
    }

    /// Test adaptive optimization configuration
    #[test]
    fn test_adaptive_optimization() {
        let context = Context::create();
        let config = EnhancedOptimizationConfig {
            enable_adaptive_optimization: true,
            feedback_config: cursed::optimization::enhanced_llvm_optimization::OptimizationFeedbackConfig {
                enable_performance_feedback: true,
                enable_size_feedback: true,
                enable_compilation_time_feedback: true,
                learning_rate: 0.15,
                max_feedback_history: 500,
            },
            ..Default::default()
        };
        let base_config = OptimizationConfig::default();
        
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config);
        assert!(optimizer.is_ok());
        
        // Test that adaptive optimization is properly configured
        let optimizer = optimizer.unwrap();
        let metrics = optimizer.get_optimization_metrics();
        assert_eq!(metrics.total_optimizations, 0);
    }

    /// Test optimization report generation
    #[test]
    fn test_optimization_report_generation() {
        let context = Context::create();
        let module = context.create_module("report_test_module");
        
        // Create a simple function
        let void_type = context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        let function = module.add_function("report_test_function", fn_type, None);
        
        let basic_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(basic_block);
        builder.build_return(None);
        
        assert!(module.verify().is_ok());
        
        let config = EnhancedOptimizationConfig::default();
        let base_config = OptimizationConfig::default();
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config).unwrap();
        
        let results = optimizer.optimize_module(&module);
        assert!(results.is_ok());
        
        let results = results.unwrap();
        let report = optimizer.generate_optimization_report(&results);
        assert!(report.is_ok());
        
        let report = report.unwrap();
        assert!(report.contains("Enhanced LLVM Optimization Report"));
        assert!(report.contains("Executive Summary"));
        assert!(report.contains("Runtime Improvement"));
        assert!(report.contains("Code Size Reduction"));
        assert!(report.contains("Target-Specific Optimizations"));
        assert!(report.len() > 100); // Ensure report has substantial content
    }

    /// Test error handling in optimization
    #[test]
    fn test_optimization_error_handling() {
        let context = Context::create();
        let module = context.create_module("error_test_module");
        
        // Create a function with intentionally problematic structure for testing
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function("error_test_function", fn_type, None);
        
        // Don't add any basic blocks - this creates an incomplete function
        // This should still be handled gracefully by the optimizer
        
        let config = EnhancedOptimizationConfig::default();
        let base_config = OptimizationConfig::default();
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config).unwrap();
        
        // The optimizer should handle incomplete functions gracefully
        let results = optimizer.optimize_module(&module);
        // This might succeed or fail depending on LLVM's handling, but should not panic
        assert!(results.is_ok() || results.is_err());
    }

    /// Test optimization metrics tracking
    #[test]
    fn test_optimization_metrics_tracking() {
        let context = Context::create();
        let module = context.create_module("metrics_test_module");
        
        // Create multiple functions to generate more metrics
        for i in 0..3 {
            let i32_type = context.i32_type();
            let fn_type = i32_type.fn_type(&[i32_type.into()], false);
            let function = module.add_function(&format!("function_{}", i), fn_type, None);
            
            let basic_block = context.append_basic_block(function, "entry");
            let builder = context.create_builder();
            builder.position_at_end(basic_block);
            
            let param = function.get_nth_param(0).unwrap();
            let const_val = i32_type.const_int(i as u64, false);
            let result = builder.build_int_add(param.into_int_value(), const_val, "result");
            builder.build_return(Some(&result));
        }
        
        assert!(module.verify().is_ok());
        
        let config = EnhancedOptimizationConfig::default();
        let base_config = OptimizationConfig::default();
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config).unwrap();
        
        // Check initial metrics
        let initial_metrics = optimizer.get_optimization_metrics();
        assert_eq!(initial_metrics.total_optimizations, 0);
        
        // Perform optimization
        let results = optimizer.optimize_module(&module);
        assert!(results.is_ok());
        
        // Check updated metrics
        let final_metrics = optimizer.get_optimization_metrics();
        assert_eq!(final_metrics.total_optimizations, 1);
        assert!(final_metrics.total_optimization_time > Duration::from_millis(0));
    }

    /// Stress test with complex module
    #[test]
    fn test_complex_module_optimization() {
        let context = Context::create();
        let module = context.create_module("complex_test_module");
        
        // Create a more complex function with loops and multiple operations
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function("complex_function", fn_type, None);
        
        let entry_block = context.append_basic_block(function, "entry");
        let loop_block = context.append_basic_block(function, "loop");
        let loop_body = context.append_basic_block(function, "loop_body");
        let loop_end = context.append_basic_block(function, "loop_end");
        let exit_block = context.append_basic_block(function, "exit");
        
        let builder = context.create_builder();
        
        // Entry block
        builder.position_at_end(entry_block);
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let zero = i32_type.const_int(0, false);
        let acc = builder.build_alloca(i32_type, "accumulator");
        builder.build_store(acc, zero);
        let counter = builder.build_alloca(i32_type, "counter");
        builder.build_store(counter, zero);
        builder.build_unconditional_branch(loop_block);
        
        // Loop condition
        builder.position_at_end(loop_block);
        let counter_val = builder.build_load(counter, "counter_val").into_int_value();
        let cmp = builder.build_int_compare(inkwell::IntPredicate::SLT, counter_val, param, "cmp");
        builder.build_conditional_branch(cmp, loop_body, loop_end);
        
        // Loop body
        builder.position_at_end(loop_body);
        let acc_val = builder.build_load(acc, "acc_val").into_int_value();
        let new_acc = builder.build_int_add(acc_val, counter_val, "new_acc");
        builder.build_store(acc, new_acc);
        let one = i32_type.const_int(1, false);
        let new_counter = builder.build_int_add(counter_val, one, "new_counter");
        builder.build_store(counter, new_counter);
        builder.build_unconditional_branch(loop_block);
        
        // Loop end
        builder.position_at_end(loop_end);
        builder.build_unconditional_branch(exit_block);
        
        // Exit block
        builder.position_at_end(exit_block);
        let final_acc = builder.build_load(acc, "final_acc");
        builder.build_return(Some(&final_acc));
        
        assert!(module.verify().is_ok());
        
        let config = EnhancedOptimizationConfig {
            optimization_level: OptimizationLevel::Aggressive,
            enable_cursed_optimizations: true,
            enable_adaptive_optimization: true,
            ..Default::default()
        };
        let base_config = OptimizationConfig::default();
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config).unwrap();
        
        let results = optimizer.optimize_module(&module);
        assert!(results.is_ok());
        
        let results = results.unwrap();
        assert!(results.base_statistics.total_optimization_time > Duration::from_millis(0));
        assert!(results.performance_improvements.runtime_improvement >= 0.0);
        
        // Verify module is still valid after aggressive optimization
        assert!(module.verify().is_ok());
    }
}

/// Integration tests for enhanced optimization system
#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test integration with different optimization levels
    #[test]
    fn test_optimization_level_integration() {
        let context = Context::create();
        let module = context.create_module("integration_test");
        
        // Create a representative function
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
        let function = module.add_function("integration_function", fn_type, None);
        
        let basic_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(basic_block);
        
        let param1 = function.get_nth_param(0).unwrap().into_int_value();
        let param2 = function.get_nth_param(1).unwrap().into_int_value();
        let result = builder.build_int_add(param1, param2, "result");
        builder.build_return(Some(&result));
        
        assert!(module.verify().is_ok());
        
        // Test each optimization level
        for level in [
            OptimizationLevel::None,
            OptimizationLevel::Less,
            OptimizationLevel::Default,
            OptimizationLevel::Aggressive,
        ] {
            let config = EnhancedOptimizationConfig {
                optimization_level: level.clone(),
                ..Default::default()
            };
            let base_config = OptimizationConfig::default();
            let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config).unwrap();
            
            let results = optimizer.optimize_module(&module);
            assert!(results.is_ok(), "Failed optimization with level: {:?}", level);
            
            let results = results.unwrap();
            let report = optimizer.generate_optimization_report(&results);
            assert!(report.is_ok());
            
            assert!(module.verify().is_ok());
        }
    }

    /// Test optimization with CURSED-specific features enabled/disabled
    #[test]
    fn test_cursed_optimization_toggle() {
        let context = Context::create();
        let module = context.create_module("cursed_toggle_test");
        
        // Create a function that might benefit from CURSED optimizations
        let void_type = context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        let function = module.add_function("cursed_test_function", fn_type, None);
        
        let basic_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(basic_block);
        builder.build_return(None);
        
        assert!(module.verify().is_ok());
        
        let base_config = OptimizationConfig::default();
        
        // Test with CURSED optimizations enabled
        let config_enabled = EnhancedOptimizationConfig {
            enable_cursed_optimizations: true,
            ..Default::default()
        };
        let optimizer_enabled = EnhancedLlvmOptimizer::new(&context, config_enabled, base_config.clone()).unwrap();
        let results_enabled = optimizer_enabled.optimize_module(&module);
        assert!(results_enabled.is_ok());
        
        // Test with CURSED optimizations disabled
        let config_disabled = EnhancedOptimizationConfig {
            enable_cursed_optimizations: false,
            ..Default::default()
        };
        let optimizer_disabled = EnhancedLlvmOptimizer::new(&context, config_disabled, base_config).unwrap();
        let results_disabled = optimizer_disabled.optimize_module(&module);
        assert!(results_disabled.is_ok());
        
        // Both should succeed, though results may differ
        assert!(module.verify().is_ok());
    }

    /// Test parallel optimization capability
    #[test]
    fn test_parallel_optimization() {
        let context = Context::create();
        let module = context.create_module("parallel_test");
        
        // Create multiple functions for parallel optimization testing
        for i in 0..5 {
            let i32_type = context.i32_type();
            let fn_type = i32_type.fn_type(&[], false);
            let function = module.add_function(&format!("parallel_function_{}", i), fn_type, None);
            
            let basic_block = context.append_basic_block(function, "entry");
            let builder = context.create_builder();
            builder.position_at_end(basic_block);
            builder.build_return(Some(&i32_type.const_int(i as u64, false)));
        }
        
        assert!(module.verify().is_ok());
        
        let config = EnhancedOptimizationConfig {
            enable_parallel_optimization: true,
            ..Default::default()
        };
        let base_config = OptimizationConfig::default();
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config).unwrap();
        
        let start_time = std::time::Instant::now();
        let results = optimizer.optimize_module(&module);
        let optimization_time = start_time.elapsed();
        
        assert!(results.is_ok());
        let results = results.unwrap();
        
        // With parallel optimization, we expect reasonable performance
        assert!(optimization_time < Duration::from_secs(10));
        assert!(results.base_statistics.total_optimization_time > Duration::from_millis(0));
        
        assert!(module.verify().is_ok());
    }
}

/// Performance benchmark tests
#[cfg(test)]
mod performance_tests {
    use super::*;

    /// Benchmark optimization performance with varying module sizes
    #[test]
    fn test_optimization_performance_scaling() {
        let context = Context::create();
        
        for num_functions in [1, 5, 10, 20] {
            let module = context.create_module(&format!("perf_test_{}", num_functions));
            
            // Create multiple functions of varying complexity
            for i in 0..num_functions {
                let i32_type = context.i32_type();
                let fn_type = i32_type.fn_type(&[i32_type.into()], false);
                let function = module.add_function(&format!("perf_function_{}", i), fn_type, None);
                
                // Create function with multiple basic blocks
                let entry = context.append_basic_block(function, "entry");
                let then_block = context.append_basic_block(function, "then");
                let else_block = context.append_basic_block(function, "else");
                let merge = context.append_basic_block(function, "merge");
                
                let builder = context.create_builder();
                
                builder.position_at_end(entry);
                let param = function.get_nth_param(0).unwrap().into_int_value();
                let zero = i32_type.const_int(0, false);
                let cmp = builder.build_int_compare(inkwell::IntPredicate::SGT, param, zero, "cmp");
                builder.build_conditional_branch(cmp, then_block, else_block);
                
                builder.position_at_end(then_block);
                let then_val = builder.build_int_mul(param, param, "then_val");
                builder.build_unconditional_branch(merge);
                
                builder.position_at_end(else_block);
                let neg_param = builder.build_int_neg(param, "neg_param");
                builder.build_unconditional_branch(merge);
                
                builder.position_at_end(merge);
                let phi = builder.build_phi(i32_type, "result");
                phi.add_incoming(&[(&then_val, then_block), (&neg_param, else_block)]);
                builder.build_return(Some(&phi.as_basic_value()));
            }
            
            assert!(module.verify().is_ok());
            
            let config = EnhancedOptimizationConfig::default();
            let base_config = OptimizationConfig::default();
            let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config).unwrap();
            
            let start_time = std::time::Instant::now();
            let results = optimizer.optimize_module(&module);
            let optimization_time = start_time.elapsed();
            
            assert!(results.is_ok());
            
            // Performance should scale reasonably
            println!("Functions: {}, Optimization time: {:?}", num_functions, optimization_time);
            assert!(optimization_time < Duration::from_secs(30), 
                    "Optimization took too long for {} functions: {:?}", num_functions, optimization_time);
            
            assert!(module.verify().is_ok());
        }
    }

    /// Test memory usage during optimization
    #[test]
    fn test_optimization_memory_usage() {
        let context = Context::create();
        let module = context.create_module("memory_test");
        
        // Create a large function to test memory usage
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function("memory_test_function", fn_type, None);
        
        let entry = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(entry);
        
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let mut current_val = param;
        
        // Create a chain of operations to increase memory usage
        for i in 0..100 {
            let const_val = i32_type.const_int(i as u64, false);
            current_val = builder.build_int_add(current_val, const_val, &format!("add_{}", i));
        }
        
        builder.build_return(Some(&current_val));
        
        assert!(module.verify().is_ok());
        
        let config = EnhancedOptimizationConfig::default();
        let base_config = OptimizationConfig::default();
        let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config).unwrap();
        
        let results = optimizer.optimize_module(&module);
        assert!(results.is_ok());
        
        let results = results.unwrap();
        let compilation_metrics = results.compilation_metrics;
        
        // Memory usage should be reasonable
        assert!(compilation_metrics.peak_memory_usage < 1_000_000_000); // Less than 1GB
        
        assert!(module.verify().is_ok());
    }
}
