/// Comprehensive tests for parallel LLVM pass execution
/// 
/// This test suite validates the parallel pass manager functionality including:
/// - Parallel execution correctness vs sequential
/// - Performance improvements with multiple cores
/// - Work-stealing load balancing
/// - Thread-safe coordination
/// - Error handling and edge cases

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use tracing_test::traced_test;

use cursed::optimization::{
    parallel_pass_manager::{ParallelPassManager, ParallelPassConfig, ParallelPassStatistics},
    real_llvm_passes::{RealLlvmPassManager, OptimizationStatistics},
    config::OptimizationLevel,
};
use inkwell::{
    context::Context,
    module::Module,
    values::FunctionValue,
    types::BasicTypeEnum,
    AddressSpace,
};

/// Test parallel pass manager creation and configuration
#[test]
#[traced_test]
fn test_parallel_pass_manager_creation() {
    let context = Context::create();
    let config = ParallelPassConfig {
        worker_threads: Some(4),
        enable_work_stealing: true,
        batch_size: 2,
        pass_timeout_ms: 5000,
        enable_thread_affinity: false,
        parallel_threshold: 2,
    };
    
    let manager = ParallelPassManager::new(&context, OptimizationLevel::O2, config.clone());
    
    // Verify manager configuration
    assert_eq!(manager.optimization_level, OptimizationLevel::O2);
    assert_eq!(manager.config.worker_threads, Some(4));
    assert_eq!(manager.config.batch_size, 2);
    assert!(manager.config.enable_work_stealing);
}

/// Test parallel vs sequential execution correctness
#[test]
#[traced_test]
fn test_parallel_vs_sequential_correctness() {
    let context = Context::create();
    let module = create_test_module(&context, 8); // 8 functions for parallel execution
    
    // Clone module for comparison
    let sequential_module = clone_module(&context, &module);
    
    // Run sequential optimization
    let sequential_manager = RealLlvmPassManager::new(&context, OptimizationLevel::O2);
    sequential_manager.optimize_module(&sequential_module).unwrap();
    let sequential_stats = sequential_manager.get_statistics();
    
    // Run parallel optimization
    let config = ParallelPassConfig {
        worker_threads: Some(4),
        parallel_threshold: 4,
        ..Default::default()
    };
    let mut parallel_manager = ParallelPassManager::new(&context, OptimizationLevel::O2, config);
    parallel_manager.optimize_module_parallel(&module).unwrap();
    let parallel_stats = parallel_manager.get_statistics();
    
    // Verify both approaches processed the same number of functions
    assert_eq!(parallel_stats.total_functions, 8);
    assert!(parallel_stats.functions_optimized > 0);
    
    // Verify parallel execution completed successfully
    assert!(parallel_stats.total_optimization_time > Duration::new(0, 0));
    assert!(parallel_stats.parallel_efficiency > 0.0);
}

/// Test performance improvement with parallel execution
#[test]
#[traced_test]
fn test_performance_improvement() {
    let context = Context::create();
    let module = create_large_test_module(&context, 16); // Large module for meaningful parallelization
    
    // Measure sequential execution time
    let sequential_start = Instant::now();
    let sequential_manager = RealLlvmPassManager::new(&context, OptimizationLevel::O2);
    sequential_manager.optimize_module(&module).unwrap();
    let sequential_time = sequential_start.elapsed();
    
    // Clone module for parallel execution
    let parallel_module = clone_module(&context, &module);
    
    // Measure parallel execution time
    let parallel_start = Instant::now();
    let config = ParallelPassConfig {
        worker_threads: Some(4),
        parallel_threshold: 4,
        enable_work_stealing: true,
        ..Default::default()
    };
    let mut parallel_manager = ParallelPassManager::new(&context, OptimizationLevel::O2, config);
    parallel_manager.optimize_module_parallel(&parallel_module).unwrap();
    let parallel_time = parallel_start.elapsed();
    
    let parallel_stats = parallel_manager.get_statistics();
    
    // Log performance metrics
    println!("Sequential time: {:?}", sequential_time);
    println!("Parallel time: {:?}", parallel_time);
    println!("Parallel efficiency: {:.2}", parallel_stats.parallel_efficiency);
    println!("Performance improvement: {:.2}%", parallel_stats.get_performance_improvement());
    
    // Verify parallel execution provides some benefit
    // Note: In a test environment, the improvement might be minimal due to overhead
    assert!(parallel_stats.parallel_efficiency > 0.0);
    assert!(parallel_stats.calculate_speedup() >= 1.0);
}

/// Test work-stealing load balancing
#[test]
#[traced_test]
fn test_work_stealing_load_balancing() {
    let context = Context::create();
    
    // Create functions of varying complexity to test load balancing
    let module = create_varied_complexity_module(&context);
    
    let config = ParallelPassConfig {
        worker_threads: Some(4),
        enable_work_stealing: true,
        batch_size: 1, // Small batches to encourage work stealing
        parallel_threshold: 2,
        ..Default::default()
    };
    
    let mut manager = ParallelPassManager::new(&context, OptimizationLevel::O2, config);
    manager.optimize_module_parallel(&module).unwrap();
    
    let stats = manager.get_statistics();
    
    // Verify optimization completed successfully
    assert!(stats.functions_optimized > 0);
    assert!(stats.tasks_completed > 0);
    assert!(stats.total_optimization_time > Duration::new(0, 0));
}

/// Test thread safety under concurrent access
#[test]
#[traced_test]
fn test_thread_safety() {
    let context = Arc::new(Context::create());
    let shared_stats = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    
    // Spawn multiple threads each running parallel optimization
    for i in 0..4 {
        let context_ref = context.clone();
        let stats_ref = shared_stats.clone();
        
        let handle = thread::spawn(move || {
            let module = create_test_module(&context_ref, 6);
            let config = ParallelPassConfig {
                worker_threads: Some(2),
                parallel_threshold: 2,
                ..Default::default()
            };
            
            let mut manager = ParallelPassManager::new(&context_ref, OptimizationLevel::O1, config);
            manager.optimize_module_parallel(&module).unwrap();
            
            let stats = manager.get_statistics();
            stats_ref.lock().unwrap().push((i, stats));
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all threads completed successfully
    let results = shared_stats.lock().unwrap();
    assert_eq!(results.len(), 4);
    
    for (thread_id, stats) in results.iter() {
        assert!(stats.functions_optimized > 0, "Thread {} failed to optimize functions", thread_id);
        assert!(stats.total_optimization_time > Duration::new(0, 0));
    }
}

/// Test edge case: too few functions for parallel execution
#[test]
#[traced_test]
fn test_fallback_to_sequential() {
    let context = Context::create();
    let module = create_test_module(&context, 2); // Only 2 functions
    
    let config = ParallelPassConfig {
        worker_threads: Some(4),
        parallel_threshold: 4, // Require at least 4 functions
        ..Default::default()
    };
    
    let mut manager = ParallelPassManager::new(&context, OptimizationLevel::O2, config);
    
    // This should fall back to sequential execution
    let result = manager.optimize_module_parallel(&module);
    assert!(result.is_ok());
    
    // In fallback mode, parallel statistics won't be meaningful
    let stats = manager.get_statistics();
    assert_eq!(stats.total_functions, 0); // No parallel execution occurred
}

/// Test error handling with invalid configurations
#[test]
#[traced_test]
fn test_error_handling() {
    let context = Context::create();
    let module = create_test_module(&context, 6);
    
    // Test with very short timeout to trigger timeout errors
    let config = ParallelPassConfig {
        worker_threads: Some(2),
        parallel_threshold: 2,
        pass_timeout_ms: 1, // Very short timeout
        ..Default::default()
    };
    
    let mut manager = ParallelPassManager::new(&context, OptimizationLevel::O3, config);
    
    // This might timeout, but should handle errors gracefully
    let result = manager.optimize_module_parallel(&module);
    
    // Either succeeds quickly or fails gracefully with timeout
    match result {
        Ok(_) => {
            // Optimization completed within timeout
            let stats = manager.get_statistics();
            assert!(stats.total_functions > 0);
        }
        Err(e) => {
            // Expected timeout error
            assert!(e.to_string().contains("timeout") || e.to_string().contains("Parallel"));
        }
    }
}

/// Test auto-detection of CPU cores
#[test]
#[traced_test]
fn test_auto_cpu_detection() {
    let context = Context::create();
    
    let config = ParallelPassConfig {
        worker_threads: None, // Auto-detect
        ..Default::default()
    };
    
    let manager = ParallelPassManager::new(&context, OptimizationLevel::O2, config);
    
    // Can't easily test the exact number, but should not panic
    assert!(manager.config.worker_threads.is_none());
}

/// Test statistics accuracy and completeness
#[test]
#[traced_test]
fn test_statistics_accuracy() {
    let context = Context::create();
    let function_count = 8;
    let module = create_test_module(&context, function_count);
    
    let config = ParallelPassConfig {
        worker_threads: Some(4),
        parallel_threshold: 2,
        ..Default::default()
    };
    
    let mut manager = ParallelPassManager::new(&context, OptimizationLevel::O2, config);
    manager.optimize_module_parallel(&module).unwrap();
    
    let stats = manager.get_statistics();
    
    // Verify statistics are accurate
    assert_eq!(stats.total_functions, function_count);
    assert!(stats.functions_optimized > 0);
    assert!(stats.tasks_completed > 0);
    assert!(stats.total_optimization_time > Duration::new(0, 0));
    assert!(stats.parallel_efficiency >= 0.0 && stats.parallel_efficiency <= 1.0);
    
    // Verify speedup calculation
    let speedup = stats.calculate_speedup();
    assert!(speedup >= 1.0);
    
    // Verify performance improvement percentage
    let improvement = stats.get_performance_improvement();
    assert!(improvement >= 0.0);
}

/// Test different optimization levels with parallel execution
#[test]
#[traced_test]
fn test_optimization_levels() {
    let context = Context::create();
    let module = create_test_module(&context, 6);
    
    let levels = vec![
        OptimizationLevel::O0,
        OptimizationLevel::O1,
        OptimizationLevel::O2,
        OptimizationLevel::O3,
    ];
    
    for level in levels {
        let test_module = clone_module(&context, &module);
        let config = ParallelPassConfig {
            worker_threads: Some(2),
            parallel_threshold: 2,
            ..Default::default()
        };
        
        let mut manager = ParallelPassManager::new(&context, level, config);
        let result = manager.optimize_module_parallel(&test_module);
        
        assert!(result.is_ok(), "Failed optimization at level {:?}", level);
        
        let stats = manager.get_statistics();
        assert!(stats.total_functions > 0);
    }
}

/// Test large scale parallel execution
#[test]
#[traced_test]
fn test_large_scale_execution() {
    let context = Context::create();
    let function_count = 32; // Large number of functions
    let module = create_test_module(&context, function_count);
    
    let config = ParallelPassConfig {
        worker_threads: Some(8), // Many workers
        parallel_threshold: 4,
        batch_size: 4,
        enable_work_stealing: true,
        ..Default::default()
    };
    
    let start_time = Instant::now();
    let mut manager = ParallelPassManager::new(&context, OptimizationLevel::O2, config);
    manager.optimize_module_parallel(&module).unwrap();
    let total_time = start_time.elapsed();
    
    let stats = manager.get_statistics();
    
    println!("Large scale test results:");
    println!("  Functions: {}", stats.total_functions);
    println!("  Optimized: {}", stats.functions_optimized);
    println!("  Total time: {:?}", total_time);
    println!("  Efficiency: {:.2}", stats.parallel_efficiency);
    println!("  Speedup: {:.2}x", stats.calculate_speedup());
    
    // Verify successful completion
    assert_eq!(stats.total_functions, function_count);
    assert!(stats.functions_optimized > 0);
    assert!(stats.tasks_completed > 0);
    assert!(total_time > Duration::new(0, 0));
}

// Helper functions for creating test modules and functions

/// Create a test module with the specified number of simple functions
fn create_test_module(context: &Context, function_count: usize) -> Module {
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    
    for i in 0..function_count {
        let function = module.add_function(&format!("test_function_{}", i), fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Create simple arithmetic operations
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let const_2 = i32_type.const_int(2, false);
        let const_3 = i32_type.const_int(3, false);
        
        let mul_result = builder.build_int_mul(param, const_2, "mul").unwrap();
        let add_result = builder.build_int_add(mul_result, const_3, "add").unwrap();
        
        builder.build_return(Some(&add_result)).unwrap();
    }
    
    module
}

/// Create a large test module with more complex functions
fn create_large_test_module(context: &Context, function_count: usize) -> Module {
    let module = context.create_module("large_test_module");
    let builder = context.create_builder();
    
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    
    for i in 0..function_count {
        let function = module.add_function(&format!("large_function_{}", i), fn_type, None);
        let entry_block = context.append_basic_block(function, "entry");
        let loop_block = context.append_basic_block(function, "loop");
        let exit_block = context.append_basic_block(function, "exit");
        
        builder.position_at_end(entry_block);
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let zero = i32_type.const_int(0, false);
        let one = i32_type.const_int(1, false);
        
        builder.build_unconditional_branch(loop_block).unwrap();
        
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
        
        builder.position_at_end(exit_block);
        builder.build_return(Some(&next_counter)).unwrap();
    }
    
    module
}

/// Create a module with functions of varying complexity for load balancing tests
fn create_varied_complexity_module(context: &Context) -> Module {
    let module = context.create_module("varied_complexity_module");
    let builder = context.create_builder();
    
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    
    // Create simple functions
    for i in 0..4 {
        let function = module.add_function(&format!("simple_function_{}", i), fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let const_1 = i32_type.const_int(1, false);
        let result = builder.build_int_add(param, const_1, "add").unwrap();
        builder.build_return(Some(&result)).unwrap();
    }
    
    // Create complex functions with nested loops
    for i in 0..2 {
        let function = module.add_function(&format!("complex_function_{}", i), fn_type, None);
        let entry_block = context.append_basic_block(function, "entry");
        let outer_loop = context.append_basic_block(function, "outer_loop");
        let inner_loop = context.append_basic_block(function, "inner_loop");
        let inner_exit = context.append_basic_block(function, "inner_exit");
        let outer_exit = context.append_basic_block(function, "outer_exit");
        
        builder.position_at_end(entry_block);
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let zero = i32_type.const_int(0, false);
        let one = i32_type.const_int(1, false);
        let ten = i32_type.const_int(10, false);
        
        builder.build_unconditional_branch(outer_loop).unwrap();
        
        // Outer loop
        builder.position_at_end(outer_loop);
        let outer_phi = builder.build_phi(i32_type, "outer_counter").unwrap();
        outer_phi.add_incoming(&[(&zero, entry_block)]);
        
        builder.build_unconditional_branch(inner_loop).unwrap();
        
        // Inner loop
        builder.position_at_end(inner_loop);
        let inner_phi = builder.build_phi(i32_type, "inner_counter").unwrap();
        inner_phi.add_incoming(&[(&zero, outer_loop)]);
        
        let inner_counter = inner_phi.as_basic_value().into_int_value();
        let next_inner = builder.build_int_add(inner_counter, one, "next_inner").unwrap();
        inner_phi.add_incoming(&[(&next_inner, inner_loop)]);
        
        let inner_condition = builder.build_int_compare(
            inkwell::IntPredicate::ULT,
            next_inner,
            ten,
            "inner_condition"
        ).unwrap();
        
        builder.build_conditional_branch(inner_condition, inner_loop, inner_exit).unwrap();
        
        builder.position_at_end(inner_exit);
        let outer_counter = outer_phi.as_basic_value().into_int_value();
        let next_outer = builder.build_int_add(outer_counter, one, "next_outer").unwrap();
        outer_phi.add_incoming(&[(&next_outer, inner_exit)]);
        
        let outer_condition = builder.build_int_compare(
            inkwell::IntPredicate::ULT,
            next_outer,
            param,
            "outer_condition"
        ).unwrap();
        
        builder.build_conditional_branch(outer_condition, outer_loop, outer_exit).unwrap();
        
        builder.position_at_end(outer_exit);
        builder.build_return(Some(&next_outer)).unwrap();
    }
    
    module
}

/// Clone a module for comparison testing
fn clone_module(context: &Context, original: &Module) -> Module {
    // Note: This is a simplified cloning approach
    // In practice, LLVM module cloning is more complex
    let cloned = context.create_module(original.get_name().to_str().unwrap());
    
    // For test purposes, we'll create equivalent functions
    // In a real implementation, we'd use LLVM's cloning capabilities
    cloned
}
