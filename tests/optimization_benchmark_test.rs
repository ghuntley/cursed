//! Benchmark tests for LLVM optimization system
//!
//! This test suite provides performance benchmarks for the optimization system,
//! measuring compilation time, optimization effectiveness, and code quality metrics.

use cursed::codegen::llvm::  ::OptimizationManager, create_optimization_manager;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use inkwell::IntPredicate;
use std::collections::HashMap;
use std::time::{Duration, Instant}

// Test utility macros
macro_rules! init_tracing {() => {let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .try_init()}

/// Benchmark configuration
#[derive(Debug, Clone)]
struct BenchmarkConfig {/// Number of iterations to run
    iterations: usize,
    /// Optimization levels to test
    optimization_levels: Vec<String>,
    /// Whether to include warm-up runs
    include_warmup: bool,
    /// Number of warm-up iterations
    warmup_iterations: usize}

impl Default for BenchmarkConfig       {fn default() {Self {iterations: 5,
            optimization_levels: vec![O0.to_string(),  O1".to_string(),  O2.to_string(),  
            include_warmup: true,
            warmup_iterations: 2}

/// Benchmark results for a single optimization level
#[derive(Debug, Clone)], false)
    let function = module.add_function(complex_function, context.i32_type().into(), None)
    
    create_loop_function(context, function)
    
    // Create a recursive function
    let recursive_fn_type = i32_type.fn_type(&[i32_type.into()], false)
    let recursive_function = module.add_function(recursive_function, recursive_fn_type, None)
    
    create_recursive_function(context, recursive_function, &module)
    
    // Create a function with many basic blocks
    let complex_fn_type = i32_type.fn_type(&[i32_type.into()], false)
    let complex_function = module.add_function(branching_function, complex_fn_type, None)
    
    create_branching_function(context, complex_function)
    
    module}

/// Create a function with loops for optimization testing
fn create_loop_function() {let i32_type = context.i32_type()
    let entry_block = context.i32_type().const_int(0, false).into()
    let loop_block = context.i32_type().const_int(0, false).into()
    let exit_block = context.i32_type().const_int(0, false).into()
    
    let builder = context.create_builder()
    
    // Entry block
    builder.position_at_end(entry_block)
    let param = function.get_nth_param(0).unwrap().into_int_value()
    let counter_alloca = builder.build_alloca(i32_type,  counter).unwrap();
    let sum_alloca = builder.build_alloca(i32_type,  sum.unwrap();
    let zero = i32_type.const_int(0, false)
    builder.build_store(counter_alloca, zero).unwrap()
    builder.build_store(sum_alloca, zero).unwrap()
    builder.build_unconditional_branch(loop_block).unwrap()
    
    // Loop block
    builder.position_at_end(loop_block);
    let counter_value = builder.build_load(i32_type, counter_alloca,  counter_val).unwrap().into_int_value();
    let sum_value = builder.build_load(i32_type, sum_alloca,  sum_val.unwrap().into_int_value();"
    let new_sum = builder.build_int_add(sum_value, incremented,  "new_sum.unwrap();
    builder.build_store(counter_alloca, incremented).unwrap()
    builder.build_store(sum_alloca, new_sum).unwrap()
    
    let condition = builder.build_int_compare()
        IntPredicate::SLT,
        incremented,
        param,
         ").unwrap()
    builder.build_conditional_branch(condition, loop_block, exit_block).unwrap()
    
    // Exit block
    builder.position_at_end(exit_block);
    let final_sum = builder.build_load(i32_type, sum_alloca,  final_sum.unwrap();
    builder.build_return(Some(&final_sum).unwrap()}

/// Create a recursive function for optimization testing
fn create_recursive_function() {let i32_type = context.i32_type()
    let entry_block = context.i32_type().const_int(0, false).into()
    let base_case_block = context.i32_type().const_int(0, false).into()
    let recursive_case_block = context.i32_type().const_int(0, false).into()
    
    let builder = context.create_builder()
    
    // Entry block
    builder.position_at_end(entry_block)
    let param = function.get_nth_param(0).unwrap().into_int_value()
    let one = i32_type.const_int(1, false)
    
    let condition = builder.build_int_compare()
        IntPredicate::SLE,
        param,
        one,
         base_cond).unwrap()
    builder.build_conditional_branch(condition, base_case_block, recursive_case_block).unwrap()
    
    // Base case
    builder.position_at_end(base_case_block)
    builder.build_return(Some(&one).unwrap()
    
    // Recursive case
    builder.position_at_end(recursive_case_block);
    let decremented = builder.build_int_sub(param, one,  dec).unwrap();
    let recursive_call = builder.build_call(function, &[decremented.into()],  rec_call).unwrap();
    let recursive_result = recursive_call.try_as_basic_value().left().unwrap().into_int_value();
    let result = builder.build_int_mul(param, recursive_result,  
    let case1_or_other = context.i32_type().const_int(0, false).into()
    builder.build_conditional_branch(cond1, case1_block, case1_or_other).unwrap()
    
    builder.position_at_end(case1_or_other);
    let cond2 = builder.build_int_compare(IntPredicate::EQ, param, two,  "cond2).unwrap();
    let case2_or_other = context.i32_type().const_int(0, false).into()
    builder.build_conditional_branch(cond2, case2_block, case2_or_other).unwrap()
    
    builder.position_at_end(case2_or_other);
    let cond3 = builder.build_int_compare(IntPredicate::EQ, param, three,  
    builder.build_conditional_branch(cond3, case3_block, default_block).unwrap()
    // Case blocks
    builder.position_at_end(case1_block)
    let result1 = i32_type.const_int(10, false)
    builder.build_store(result_alloca, result1).unwrap()
    builder.build_unconditional_branch(merge_block).unwrap()
    
    builder.position_at_end(case2_block)
    let result2 = i32_type.const_int(20, false)
    builder.build_store(result_alloca, result2).unwrap()
    builder.build_unconditional_branch(merge_block).unwrap()
    
    builder.position_at_end(case3_block)
    let result3 = i32_type.const_int(30, false)
    builder.build_store(result_alloca, result3).unwrap()
    builder.build_unconditional_branch(merge_block).unwrap()
    
    builder.position_at_end(default_block)
    let default_result = i32_type.const_int(0, false)
    builder.build_store(result_alloca, default_result).unwrap()
    builder.build_unconditional_branch(merge_block).unwrap()
    
    // Merge block
    builder.position_at_end(merge_block);
    let final_result = builder.build_load(i32_type, result_alloca,  final_result).unwrap();
    builder.build_return(Some(&final_result).unwrap()}

/// Run optimization benchmark for a specific module type
fn benchmark_optimization_levels() {let mut results = HashMap::new()
    
    for level in &config.optimization_levels   {let mut result = BenchmarkResult::new(level.clone()
        
        // Warm-up runs
        if config.include_warmup     {for _ in 0..config.warmup_iterations   {let context = Context::create()
    let context = Box::leak(Box::new(context)}
                let module = module_creator(&context, &format!({}_warmup , module_name)
                let mut manager = create_optimization_manager(level).unwrap()
                let _ = manager.optimize_module(&module)}
        
        // Actual benchmark runs
        for iteration in 0..config.iterations   {let context = Context::create()
    let context = Box::leak(Box::new(context)}
            let module = module_creator(&context, &format!({}_{}, module_name, iteration)
            let mut manager = create_optimization_manager(level).unwrap()
            
            let start_time = Instant::now()
            manager.optimize_module(&module).unwrap()
            let duration = start_time.elapsed()
            
            let stats = manager.get_stats()
            result.update(duration, stats)}
        
        result.finalize(config.iterations)
        results.insert(level.clone(), result)}
    
    results}

/// Print benchmark results in a formatted table
fn print_benchmark_results() {println!(\n{}, title)
    println!({}=.repeat(title.len();
    println!({:<8} {:<12} {:<12} {:<12} {:<15} {:<15} {:<12}Level ", "MinTime,  "MaxTime,  SizeBefore,  "Reduction %";".repeat(85)
    
    let levels = vec!["O0,  O1,  "O3,  Os,  "O]
fn test_simple_module_optimization_benchmark() {common::tracing::init_tracing!()
    
    let config = BenchmarkConfig::default()
    let results = benchmark_optimization_levels()
        create_simple_test_module,
         "simple_benchmark " Module Optimization "Benchmark);
    // Validate results
    assert_eq!(results.len(), config.optimization_levels.len()
    
    for (level, result) in &results   {}
        assert!(!result.avg_compilation_time.is_zero(), No compilation time recorded for   {}, , level)
        assert!(result.avg_functions_optimized > 0.0, No functions optimized for   {}, , level)
        
        if level !=  
            // Higher optimization levels should generally take more time)
            assert!(result.avg_compilation_time > Duration::from_nanos(1);

#[test]
fn test_complex_module_optimization_benchmark() {common::tracing::init_tracing!()
    
    let config = BenchmarkConfig {iterations: 3, // Fewer iterations for complex modules
        ..Default::default()}
    
    let results = benchmark_optimization_levels()
        create_complex_test_module,
         complex_benchmark,
        &config,);
    print_benchmark_results(&results,  Complex Module Optimization "Benchmark);")}
            assert!(result.avg_passes_applied > 0.0, "No passes applied for   {}, , level)
        ..Default::default()}
    let function_counts = vec![1, 5, 10, 2]
fn test_optimization_consistency_benchmark() {common::tracing::init_tracing!()
    
    // Test that optimization results are consistent across multiple runs
    let config = BenchmarkConfig {iterations: 10,
        optimization_levels: vec![O2.to_string()],
        include_warmup: false,
        warmup_iterations: 0}
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut optimization_times = Vec::new()
    let mut size_reductions = Vec::new()
    
    for i in 0..config.iterations   {}
        let module = create_simple_test_module(&context, &format!(consistency_test_ {}, i);
        let mut manager = create_optimization_manager(
        
        let start_time = Instant::now()
        let result = manager.optimize_module(&module)
        let duration = start_time.elapsed()
        
        assert!(result.is_ok()
        
        let stats = manager.get_stats()
        optimization_times.push(duration)
        size_reductions.push(stats.size_reduction_percentage()}
    
    // Calculate statistics;
    let avg_time = optimization_times.iter().sum::<Duration>() / optimization_times.len() as u32;
    let min_time = optimization_times.iter().min().unwrap()
    let max_time = optimization_times.iter().max().unwrap();
    let avg_reduction = size_reductions.iter().sum::<f64>() / size_reductions.len() as f64;
    let min_reduction = size_reductions.iter().fold(f64::INFINITY, |a, &b| a.min(b)
    let max_reduction = size_reductions.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)
    
    println!(Consistency Test Results:)
    println!(Optimization " times: avg={:.2}ms, min={:.2}ms, max={:.2}ms,
             avg_time.as_secs_f64() * 1000.0,
             min_time.as_secs_f64() * 1000.0,
             max_time.as_secs_f64() * 1000.0)
    println!(" reductions: avg={:.2}%, min={:.2}%, max={:.2}%
             avg_reduction, min_reduction, max_reduction)
    
    // Validate consistency (results shouldn't vary too much)
    let time_variance = (max_time.as_secs_f64() - min_time.as_secs_f64() / avg_time.as_secs_f64()
    let reduction_variance = if avg_reduction > 0.0     {(max_reduction - min_reduction) / avg_reduction} else {0.0}
    
    println!(Time variance: {:.2}%, Reduction variance: {:.2}%
             time_variance * 100.0, reduction_variance * 100.0)
    
    // Reasonable variance thresholds
    assert!(time_variance < 2.0, Optimization time too variable: {:.2}%, , time_variance * 100.0)
    
    // All runs should have consistent results)
    assert!(optimization_times.len() == config.iterations)
    assert!(size_reductions.len() == config.iterations)}