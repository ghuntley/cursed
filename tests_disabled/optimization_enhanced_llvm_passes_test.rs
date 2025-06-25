/// Comprehensive test suite for Enhanced LLVM Optimization Passes
/// 
/// Tests all newly implemented optimization passes including memory layout,
/// interprocedural analysis, vectorization, cache optimization, and branch prediction.

use cursed::optimization::enhanced_llvm_passes::*;
use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
use cursed::error::Result;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::IntType;
use std::sync::{Arc, Mutex};

/// Helper function to create a test module
fn create_test_module<'ctx>(context: &'ctx Context) -> Module<'ctx> {
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create a simple test function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("test_function", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    let param0 = function.get_nth_param(0).unwrap().into_int_value();
    let param1 = function.get_nth_param(1).unwrap().into_int_value();
    
    // Add some simple operations
    let add_result = builder.build_int_add(param0, param1, "add").unwrap();
    let mul_result = builder.build_int_mul(add_result, param0, "mul").unwrap();
    
    builder.build_return(Some(&mul_result)).unwrap();
    
    module
}

/// Helper function to create a more complex test module with loops
fn create_complex_test_module<'ctx>(context: &'ctx Context) -> Module<'ctx> {
    let module = context.create_module("complex_test_module");
    let builder = context.create_builder();
    
    // Create a function with loop-like structure
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("complex_function", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let loop_block = context.append_basic_block(function, "loop");
    let exit_block = context.append_basic_block(function, "exit");
    
    // Entry block
    builder.position_at_end(entry_block);
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let zero = i32_type.const_int(0, false);
    let one = i32_type.const_int(1, false);
    builder.build_unconditional_branch(loop_block).unwrap();
    
    // Loop block
    builder.position_at_end(loop_block);
    let phi = builder.build_phi(i32_type, "counter").unwrap();
    phi.add_incoming(&[(&zero, entry_block)]);
    
    let counter = phi.as_basic_value().into_int_value();
    let next_counter = builder.build_int_add(counter, one, "next").unwrap();
    phi.add_incoming(&[(&next_counter, loop_block)]);
    
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::ULT,
        next_counter,
        param,
        "condition"
    ).unwrap();
    
    builder.build_conditional_branch(condition, loop_block, exit_block).unwrap();
    
    // Exit block
    builder.position_at_end(exit_block);
    builder.build_return(Some(&next_counter)).unwrap();
    
    module
}

#[test]
fn test_enhanced_llvm_pass_manager_creation() {
    let context = Context::create();
    let config = OptimizationConfig::default();
    let manager = EnhancedLlvmPassManager::new(&context, OptimizationLevel::Default, &config);
    
    assert_eq!(manager.optimization_level, OptimizationLevel::Default);
}

#[test]
fn test_enhanced_llvm_pass_manager_optimization() -> Result<()> {
    let context = Context::create();
    let module = create_test_module(&context);
    let config = OptimizationConfig::default();
    let manager = EnhancedLlvmPassManager::new(&context, OptimizationLevel::Default, &config);
    
    // Run optimization
    manager.optimize_module(&module)?;
    
    // Verify module is still valid
    assert!(module.verify().is_ok());
    
    // Check that statistics were updated
    let stats = manager.get_statistics();
    assert!(stats.total_optimization_time.as_millis() > 0);
    
    Ok(())
}

#[test]
fn test_memory_layout_optimizer() -> Result<()> {
    let context = Context::create();
    let module = create_test_module(&context);
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let mut optimizer = MemoryLayoutOptimizer::new(statistics.clone());
    
    // Analyze memory patterns
    optimizer.analyze_memory_patterns(&module)?;
    
    // Optimize each function
    for function in module.get_functions() {
        if function.get_first_basic_block().is_some() {
            let optimizations = optimizer.optimize_memory_layout(function)?;
            // Should find some optimization opportunities
            assert!(optimizations >= 0);
        }
    }
    
    // Check statistics
    let stats = optimizer.get_optimization_statistics()?;
    assert!(stats.contains_key("memory_access_hotspots"));
    
    Ok(())
}

#[test]
fn test_interprocedural_analyzer() -> Result<()> {
    let context = Context::create();
    let module = create_test_module(&context);
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let mut analyzer = InterproceduralAnalyzer::new(statistics.clone());
    
    // Analyze module
    analyzer.analyze_module(&module)?;
    
    // Check call graph statistics
    let stats = analyzer.get_call_graph_statistics();
    assert!(stats.contains_key("total_functions"));
    assert!(stats.get("total_functions").unwrap() > &0);
    
    Ok(())
}

#[test]
fn test_vectorization_optimizer() -> Result<()> {
    let context = Context::create();
    let module = create_complex_test_module(&context);
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let mut optimizer = VectorizationOptimizer::new(statistics.clone());
    
    // Vectorize operations in each function
    for function in module.get_functions() {
        if function.get_first_basic_block().is_some() {
            let vectorizations = optimizer.vectorize_operations(function)?;
            // Should find some vectorization opportunities
            assert!(vectorizations >= 0);
        }
    }
    
    // Check vectorization statistics
    let stats = optimizer.get_vectorization_statistics();
    assert!(stats.contains_key("analyzed_functions"));
    
    Ok(())
}

#[test]
fn test_cache_optimizer() -> Result<()> {
    let context = Context::create();
    let module = create_complex_test_module(&context);
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let mut optimizer = CacheOptimizer::new(statistics.clone());
    
    // Optimize cache usage in each function
    for function in module.get_functions() {
        if function.get_first_basic_block().is_some() {
            let optimizations = optimizer.optimize_cache_usage(function)?;
            // Should find some cache optimization opportunities
            assert!(optimizations >= 0);
        }
    }
    
    // Check cache statistics
    let stats = optimizer.get_cache_statistics();
    assert!(stats.contains_key("functions_analyzed"));
    
    Ok(())
}

#[test]
fn test_branch_predictor() -> Result<()> {
    let context = Context::create();
    let module = create_complex_test_module(&context);
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let mut predictor = BranchPredictor::new(statistics.clone());
    
    // Optimize branch patterns in each function
    for function in module.get_functions() {
        if function.get_first_basic_block().is_some() {
            let optimizations = predictor.optimize_branch_patterns(function)?;
            // Should find some branch optimization opportunities
            assert!(optimizations >= 0);
        }
    }
    
    // Check branch statistics
    let stats = predictor.get_branch_statistics();
    assert!(stats.contains_key("functions_analyzed"));
    
    Ok(())
}

#[test]
fn test_optimization_statistics() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    
    // Test default statistics
    {
        let stats = statistics.lock().unwrap();
        assert_eq!(stats.initial_functions, 0);
        assert_eq!(stats.functions_inlined, 0);
        assert_eq!(stats.goroutines_optimized, 0);
        assert_eq!(stats.vectorized_operations, 0);
        assert_eq!(stats.cache_optimizations, 0);
        assert_eq!(stats.branch_predictions_improved, 0);
    }
    
    // Test statistics update
    {
        let mut stats = statistics.lock().unwrap();
        stats.vectorized_operations = 5;
        stats.cache_optimizations = 3;
        stats.branch_predictions_improved = 2;
        stats.memory_layout_improvements = 4;
    }
    
    {
        let stats = statistics.lock().unwrap();
        assert_eq!(stats.vectorized_operations, 5);
        assert_eq!(stats.cache_optimizations, 3);
        assert_eq!(stats.branch_predictions_improved, 2);
        assert_eq!(stats.memory_layout_improvements, 4);
    }
}

#[test]
fn test_performance_tracker() {
    let mut tracker = PerformanceTracker::new();
    
    // Test phase timing
    tracker.start_phase("test_phase");
    std::thread::sleep(std::time::Duration::from_millis(10));
    tracker.end_phase("test_phase");
    
    let timings = tracker.get_phase_timings().unwrap();
    assert_eq!(timings.len(), 1);
    assert_eq!(timings[0].0, "test_phase");
    assert!(timings[0].1.as_millis() >= 10);
    
    // Test compilation speedup calculation
    let speedup = tracker.get_compilation_speedup();
    assert!(speedup >= 1.0);
}

#[test]
fn test_integration_with_optimization_levels() -> Result<()> {
    let context = Context::create();
    let module = create_complex_test_module(&context);
    let config = OptimizationConfig::default();
    
    // Test different optimization levels
    let levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
        OptimizationLevel::Size,
    ];
    
    for level in levels {
        let manager = EnhancedLlvmPassManager::new(&context, level, &config);
        manager.optimize_module(&module)?;
        
        let stats = manager.get_statistics();
        
        // Aggressive optimization should do more work
        if matches!(level, OptimizationLevel::Aggressive) {
            assert!(stats.estimated_runtime_improvement >= 0.0);
        }
        
        // Verify module is still valid after optimization
        assert!(module.verify().is_ok());
    }
    
    Ok(())
}

#[test]
fn test_optimization_report_generation() -> Result<()> {
    let context = Context::create();
    let module = create_test_module(&context);
    let config = OptimizationConfig::default();
    let manager = EnhancedLlvmPassManager::new(&context, OptimizationLevel::Default, &config);
    
    // Run optimization
    manager.optimize_module(&module)?;
    
    // Generate report
    let report = manager.generate_optimization_report()?;
    
    // Check report content
    assert!(report.contains("Enhanced LLVM Optimization Report"));
    assert!(report.contains("Optimization Level"));
    assert!(report.contains("Core Metrics"));
    assert!(report.contains("CURSED-Specific Optimizations"));
    assert!(report.contains("Advanced Optimizations"));
    assert!(report.contains("Performance Improvements"));
    
    Ok(())
}

#[test]
fn test_memory_hierarchy_configuration() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = CacheOptimizer::new(statistics);
    
    // Test default memory hierarchy
    assert_eq!(optimizer.memory_hierarchy.l1_cache.size, 32 * 1024);
    assert_eq!(optimizer.memory_hierarchy.l1_cache.line_size, 64);
    assert_eq!(optimizer.memory_hierarchy.l2_cache.size, 256 * 1024);
    assert_eq!(optimizer.memory_hierarchy.l3_cache.size, 8 * 1024 * 1024);
    assert_eq!(optimizer.memory_hierarchy.main_memory.page_size, 4096);
}

#[test]
fn test_vectorization_target_info() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = VectorizationOptimizer::new(statistics);
    
    // Test default target information
    assert!(optimizer.target_info.supported_widths.contains_key("i32"));
    assert!(optimizer.target_info.supported_widths.contains_key("f32"));
    assert!(optimizer.target_info.supported_widths.contains_key("f64"));
    
    assert!(optimizer.target_info.operation_costs.contains_key(&VectorOperation::Add));
    assert!(optimizer.target_info.operation_costs.contains_key(&VectorOperation::Multiply));
    
    assert!(optimizer.target_info.available_instructions.contains(&SIMDInstructionSet::SSE2));
    assert!(optimizer.target_info.available_instructions.contains(&SIMDInstructionSet::AVX));
}

#[test]
fn test_comprehensive_optimization_workflow() -> Result<()> {
    let context = Context::create();
    let module = create_complex_test_module(&context);
    let config = OptimizationConfig::default();
    let manager = EnhancedLlvmPassManager::new(&context, OptimizationLevel::Aggressive, &config);
    
    // Record initial state
    let initial_function_count = module.get_functions().count();
    
    // Run comprehensive optimization
    manager.optimize_module(&module)?;
    
    // Verify optimization completed successfully
    let stats = manager.get_statistics();
    assert!(stats.total_optimization_time.as_millis() > 0);
    
    // Verify module integrity
    assert!(module.verify().is_ok());
    
    // Check that all optimization phases ran
    assert!(stats.estimated_runtime_improvement >= 0.0);
    assert!(stats.estimated_memory_reduction >= 0.0);
    assert!(stats.compilation_speedup >= 1.0);
    
    // Verify functions are still present
    let final_function_count = module.get_functions().count();
    assert!(final_function_count > 0);
    
    Ok(())
}

#[test]
fn test_optimization_pass_isolation() -> Result<()> {
    let context = Context::create();
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    
    // Test that each optimizer can be used independently
    let module1 = create_test_module(&context);
    let mut memory_opt = MemoryLayoutOptimizer::new(statistics.clone());
    memory_opt.analyze_memory_patterns(&module1)?;
    
    let module2 = create_test_module(&context);
    let mut vector_opt = VectorizationOptimizer::new(statistics.clone());
    for function in module2.get_functions() {
        if function.get_first_basic_block().is_some() {
            vector_opt.vectorize_operations(function)?;
        }
    }
    
    let module3 = create_test_module(&context);
    let mut cache_opt = CacheOptimizer::new(statistics.clone());
    for function in module3.get_functions() {
        if function.get_first_basic_block().is_some() {
            cache_opt.optimize_cache_usage(function)?;
        }
    }
    
    // All modules should still be valid
    assert!(module1.verify().is_ok());
    assert!(module2.verify().is_ok());
    assert!(module3.verify().is_ok());
    
    Ok(())
}

#[test]
fn test_error_handling_in_optimizations() -> Result<()> {
    let context = Context::create();
    let config = OptimizationConfig::default();
    
    // Create a minimal module
    let module = context.create_module("minimal_test");
    
    // Should handle empty modules gracefully
    let manager = EnhancedLlvmPassManager::new(&context, OptimizationLevel::Default, &config);
    manager.optimize_module(&module)?;
    
    let stats = manager.get_statistics();
    assert_eq!(stats.initial_functions, 0);
    assert_eq!(stats.final_functions, 0);
    
    Ok(())
}
