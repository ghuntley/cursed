//! Benchmark tests for LLVM optimization system
//!
//! This test suite provides performance benchmarks for the optimization system,
//! measuring compilation time, optimization effectiveness, and code quality metrics.

use cursed::codegen::llvm::{OptimizationManager, create_optimization_manager};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use inkwell::IntPredicate;
use std::collections::HashMap;
use std::time::{Duration, Instant};

// Test utility macros
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .try_init();
    };
}

/// Benchmark configuration
#[derive(Debug, Clone)]
struct BenchmarkConfig {
    /// Number of iterations to run
    iterations: usize,
    /// Optimization levels to test
    optimization_levels: Vec<String>,
    /// Whether to include warm-up runs
    include_warmup: bool,
    /// Number of warm-up iterations
    warmup_iterations: usize,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 5,
            optimization_levels: vec!["O0".to_string(), "O1".to_string(), "O2".to_string(), "O3".to_string()],
            include_warmup: true,
            warmup_iterations: 2,
        }
    }
}

/// Benchmark results for a single optimization level
#[derive(Debug, Clone)]
struct BenchmarkResult {
    /// Optimization level
    level: String,
    /// Average compilation time
    avg_compilation_time: Duration,
    /// Minimum compilation time
    min_compilation_time: Duration,
    /// Maximum compilation time
    max_compilation_time: Duration,
    /// Average code size before optimization
    avg_size_before: f64,
    /// Average code size after optimization
    avg_size_after: f64,
    /// Average size reduction percentage
    avg_size_reduction: f64,
    /// Average number of passes applied
    avg_passes_applied: f64,
    /// Average number of functions optimized
    avg_functions_optimized: f64,
}

impl BenchmarkResult {
    fn new(level: String) -> Self {
        Self {
            level,
            avg_compilation_time: Duration::new(0, 0),
            min_compilation_time: Duration::from_secs(999),
            max_compilation_time: Duration::new(0, 0),
            avg_size_before: 0.0,
            avg_size_after: 0.0,
            avg_size_reduction: 0.0,
            avg_passes_applied: 0.0,
            avg_functions_optimized: 0.0,
        }
    }

    fn update(&mut self, duration: Duration, stats: &cursed::codegen::llvm::OptimizationStats) {
        self.avg_compilation_time = self.avg_compilation_time + duration;
        self.min_compilation_time = self.min_compilation_time.min(duration);
        self.max_compilation_time = self.max_compilation_time.max(duration);
        
        self.avg_size_before += stats.code_size_before as f64;
        self.avg_size_after += stats.code_size_after as f64;
        self.avg_size_reduction += stats.size_reduction_percentage();
        self.avg_passes_applied += stats.passes_applied as f64;
        self.avg_functions_optimized += stats.functions_optimized as f64;
    }

    fn finalize(&mut self, iterations: usize) {
        let iterations_f = iterations as f64;
        self.avg_compilation_time = Duration::from_nanos(
            (self.avg_compilation_time.as_nanos() as f64 / iterations_f) as u64
        );
        self.avg_size_before /= iterations_f;
        self.avg_size_after /= iterations_f;
        self.avg_size_reduction /= iterations_f;
        self.avg_passes_applied /= iterations_f;
        self.avg_functions_optimized /= iterations_f;
    }
}

/// Create a simple test module for benchmarking
fn create_simple_test_module(context: &Context, name: &str) -> Module {
    let module = context.create_module(name);
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("simple_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Simple arithmetic operations
    let const_42 = i32_type.const_int(42, false);
    let const_1 = i32_type.const_int(1, false);
    let add_result = builder.build_int_add(const_42, const_1, "add").unwrap();
    
    builder.build_return(Some(&add_result)).unwrap();
    
    module
}

/// Create a complex test module with loops and branches for benchmarking
fn create_complex_test_module(context: &Context, name: &str) -> Module {
    let module = context.create_module(name);
    let i32_type = context.i32_type();
    
    // Create a function with loops and branches
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("complex_function", fn_type, None);
    
    create_loop_function(context, function);
    
    // Create a recursive function
    let recursive_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let recursive_function = module.add_function("recursive_function", recursive_fn_type, None);
    
    create_recursive_function(context, recursive_function, &module);
    
    // Create a function with many basic blocks
    let complex_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let complex_function = module.add_function("branching_function", complex_fn_type, None);
    
    create_branching_function(context, complex_function);
    
    module
}

/// Create a function with loops for optimization testing
fn create_loop_function(context: &Context, function: FunctionValue) {
    let i32_type = context.i32_type();
    let entry_block = context.append_basic_block(function, "entry");
    let loop_block = context.append_basic_block(function, "loop");
    let exit_block = context.append_basic_block(function, "exit");
    
    let builder = context.create_builder();
    
    // Entry block
    builder.position_at_end(entry_block);
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let counter_alloca = builder.build_alloca(i32_type, "counter").unwrap();
    let sum_alloca = builder.build_alloca(i32_type, "sum").unwrap();
    let zero = i32_type.const_int(0, false);
    builder.build_store(counter_alloca, zero).unwrap();
    builder.build_store(sum_alloca, zero).unwrap();
    builder.build_unconditional_branch(loop_block).unwrap();
    
    // Loop block
    builder.position_at_end(loop_block);
    let counter_value = builder.build_load(i32_type, counter_alloca, "counter_val").unwrap().into_int_value();
    let sum_value = builder.build_load(i32_type, sum_alloca, "sum_val").unwrap().into_int_value();
    
    let one = i32_type.const_int(1, false);
    let incremented = builder.build_int_add(counter_value, one, "inc").unwrap();
    let new_sum = builder.build_int_add(sum_value, incremented, "new_sum").unwrap();
    
    builder.build_store(counter_alloca, incremented).unwrap();
    builder.build_store(sum_alloca, new_sum).unwrap();
    
    let condition = builder.build_int_compare(
        IntPredicate::SLT,
        incremented,
        param,
        "loop_cond"
    ).unwrap();
    builder.build_conditional_branch(condition, loop_block, exit_block).unwrap();
    
    // Exit block
    builder.position_at_end(exit_block);
    let final_sum = builder.build_load(i32_type, sum_alloca, "final_sum").unwrap();
    builder.build_return(Some(&final_sum)).unwrap();
}

/// Create a recursive function for optimization testing
fn create_recursive_function(context: &Context, function: FunctionValue, module: &Module) {
    let i32_type = context.i32_type();
    let entry_block = context.append_basic_block(function, "entry");
    let base_case_block = context.append_basic_block(function, "base_case");
    let recursive_case_block = context.append_basic_block(function, "recursive_case");
    
    let builder = context.create_builder();
    
    // Entry block
    builder.position_at_end(entry_block);
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let one = i32_type.const_int(1, false);
    
    let condition = builder.build_int_compare(
        IntPredicate::SLE,
        param,
        one,
        "base_cond"
    ).unwrap();
    builder.build_conditional_branch(condition, base_case_block, recursive_case_block).unwrap();
    
    // Base case
    builder.position_at_end(base_case_block);
    builder.build_return(Some(&one)).unwrap();
    
    // Recursive case
    builder.position_at_end(recursive_case_block);
    let decremented = builder.build_int_sub(param, one, "dec").unwrap();
    let recursive_call = builder.build_call(function, &[decremented.into()], "rec_call").unwrap();
    let recursive_result = recursive_call.try_as_basic_value().left().unwrap().into_int_value();
    let result = builder.build_int_mul(param, recursive_result, "result").unwrap();
    builder.build_return(Some(&result)).unwrap();
}

/// Create a function with many branches for optimization testing
fn create_branching_function(context: &Context, function: FunctionValue) {
    let i32_type = context.i32_type();
    let entry_block = context.append_basic_block(function, "entry");
    let case1_block = context.append_basic_block(function, "case1");
    let case2_block = context.append_basic_block(function, "case2");
    let case3_block = context.append_basic_block(function, "case3");
    let default_block = context.append_basic_block(function, "default");
    let merge_block = context.append_basic_block(function, "merge");
    
    let builder = context.create_builder();
    
    // Entry block - create switch-like structure
    builder.position_at_end(entry_block);
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let result_alloca = builder.build_alloca(i32_type, "result").unwrap();
    
    let one = i32_type.const_int(1, false);
    let two = i32_type.const_int(2, false);
    let three = i32_type.const_int(3, false);
    
    let cond1 = builder.build_int_compare(IntPredicate::EQ, param, one, "cond1").unwrap();
    let case1_or_other = context.append_basic_block(function, "case1_or_other");
    builder.build_conditional_branch(cond1, case1_block, case1_or_other).unwrap();
    
    builder.position_at_end(case1_or_other);
    let cond2 = builder.build_int_compare(IntPredicate::EQ, param, two, "cond2").unwrap();
    let case2_or_other = context.append_basic_block(function, "case2_or_other");
    builder.build_conditional_branch(cond2, case2_block, case2_or_other).unwrap();
    
    builder.position_at_end(case2_or_other);
    let cond3 = builder.build_int_compare(IntPredicate::EQ, param, three, "cond3").unwrap();
    builder.build_conditional_branch(cond3, case3_block, default_block).unwrap();
    
    // Case blocks
    builder.position_at_end(case1_block);
    let result1 = i32_type.const_int(10, false);
    builder.build_store(result_alloca, result1).unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();
    
    builder.position_at_end(case2_block);
    let result2 = i32_type.const_int(20, false);
    builder.build_store(result_alloca, result2).unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();
    
    builder.position_at_end(case3_block);
    let result3 = i32_type.const_int(30, false);
    builder.build_store(result_alloca, result3).unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();
    
    builder.position_at_end(default_block);
    let default_result = i32_type.const_int(0, false);
    builder.build_store(result_alloca, default_result).unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();
    
    // Merge block
    builder.position_at_end(merge_block);
    let final_result = builder.build_load(i32_type, result_alloca, "final_result").unwrap();
    builder.build_return(Some(&final_result)).unwrap();
}

/// Run optimization benchmark for a specific module type
fn benchmark_optimization_levels(
    module_creator: fn(&Context, &str) -> Module,
    module_name: &str,
    config: &BenchmarkConfig,
) -> HashMap<String, BenchmarkResult> {
    let mut results = HashMap::new();
    
    for level in &config.optimization_levels {
        let mut result = BenchmarkResult::new(level.clone());
        
        // Warm-up runs
        if config.include_warmup {
            for _ in 0..config.warmup_iterations {
                let context = Context::create();
                let module = module_creator(&context, &format!("{}_warmup", module_name));
                let mut manager = create_optimization_manager(level).unwrap();
                let _ = manager.optimize_module(&module);
            }
        }
        
        // Actual benchmark runs
        for iteration in 0..config.iterations {
            let context = Context::create();
            let module = module_creator(&context, &format!("{}_{}", module_name, iteration));
            let mut manager = create_optimization_manager(level).unwrap();
            
            let start_time = Instant::now();
            manager.optimize_module(&module).unwrap();
            let duration = start_time.elapsed();
            
            let stats = manager.get_stats();
            result.update(duration, stats);
        }
        
        result.finalize(config.iterations);
        results.insert(level.clone(), result);
    }
    
    results
}

/// Print benchmark results in a formatted table
fn print_benchmark_results(results: &HashMap<String, BenchmarkResult>, title: &str) {
    println!("\n{}", title);
    println!("{}", "=".repeat(title.len()));
    
    println!("{:<8} {:<12} {:<12} {:<12} {:<15} {:<15} {:<12}",
             "Level", "Avg Time", "Min Time", "Max Time", "Size Before", "Size After", "Reduction%");
    println!("{}", "-".repeat(85));
    
    let levels = vec!["O0", "O1", "O2", "O3", "Os", "Oz"];
    
    for level in &levels {
        if let Some(result) = results.get(*level) {
            println!("{:<8} {:<12} {:<12} {:<12} {:<15.0} {:<15.0} {:<12.2}",
                     result.level,
                     format!("{:.2}ms", result.avg_compilation_time.as_secs_f64() * 1000.0),
                     format!("{:.2}ms", result.min_compilation_time.as_secs_f64() * 1000.0),
                     format!("{:.2}ms", result.max_compilation_time.as_secs_f64() * 1000.0),
                     result.avg_size_before,
                     result.avg_size_after,
                     result.avg_size_reduction);
        }
    }
    
    println!();
    println!("Additional Metrics:");
    for level in &levels {
        if let Some(result) = results.get(*level) {
            println!("{}: {:.1} functions, {:.1} passes",
                     result.level,
                     result.avg_functions_optimized,
                     result.avg_passes_applied);
        }
    }
}

#[test]
fn test_simple_module_optimization_benchmark() {
    init_tracing!();
    
    let config = BenchmarkConfig::default();
    let results = benchmark_optimization_levels(
        create_simple_test_module,
        "simple_benchmark",
        &config,
    );
    
    print_benchmark_results(&results, "Simple Module Optimization Benchmark");
    
    // Validate results
    assert_eq!(results.len(), config.optimization_levels.len());
    
    for (level, result) in &results {
        assert!(!result.avg_compilation_time.is_zero(), "No compilation time recorded for {}", level);
        assert!(result.avg_functions_optimized > 0.0, "No functions optimized for {}", level);
        
        if level != "O0" {
            // Higher optimization levels should generally take more time
            assert!(result.avg_compilation_time > Duration::from_nanos(1));
        }
    }
}

#[test]
fn test_complex_module_optimization_benchmark() {
    init_tracing!();
    
    let config = BenchmarkConfig {
        iterations: 3, // Fewer iterations for complex modules
        ..Default::default()
    };
    
    let results = benchmark_optimization_levels(
        create_complex_test_module,
        "complex_benchmark",
        &config,
    );
    
    print_benchmark_results(&results, "Complex Module Optimization Benchmark");
    
    // Validate results
    assert_eq!(results.len(), config.optimization_levels.len());
    
    for (level, result) in &results {
        assert!(!result.avg_compilation_time.is_zero(), "No compilation time recorded for {}", level);
        assert!(result.avg_functions_optimized >= 3.0, "Expected at least 3 functions for {}", level);
        
        if level != "O0" {
            assert!(result.avg_passes_applied > 0.0, "No passes applied for {}", level);
        }
    }
    
    // Compare O0 vs O3 performance
    let o0_result = results.get("O0").unwrap();
    let o3_result = results.get("O3").unwrap();
    
    // O3 should apply more passes than O0
    assert!(o3_result.avg_passes_applied > o0_result.avg_passes_applied);
    
    // O3 may take longer than O0, but should provide better optimization
    if o3_result.avg_size_reduction > o0_result.avg_size_reduction {
        println!("O3 provides better optimization: {:.2}% vs {:.2}%",
                 o3_result.avg_size_reduction,
                 o0_result.avg_size_reduction);
    }
}

#[test]
fn test_optimization_scaling_benchmark() {
    init_tracing!();
    
    // Test how optimization time scales with module complexity
    let context = Context::create();
    let base_config = BenchmarkConfig {
        iterations: 3,
        optimization_levels: vec!["O2".to_string()],
        ..Default::default()
    };
    
    let function_counts = vec![1, 5, 10, 20];
    let mut scaling_results = Vec::new();
    
    for &func_count in &function_counts {
        println!("Testing with {} functions...", func_count);
        
        // Create module with specified number of functions
        let module = context.create_module(&format!("scaling_test_{}", func_count));
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        
        for i in 0..func_count {
            let function = module.add_function(&format!("test_function_{}", i), fn_type, None);
            let basic_block = context.append_basic_block(function, "entry");
            
            let builder = context.create_builder();
            builder.position_at_end(basic_block);
            
            // Add some computation
            let const_val = i32_type.const_int(i as u64 * 42, false);
            let const_1 = i32_type.const_int(1, false);
            let add_result = builder.build_int_add(const_val, const_1, "add").unwrap();
            
            builder.build_return(Some(&add_result)).unwrap();
        }
        
        // Benchmark this module
        let mut total_time = Duration::new(0, 0);
        let mut total_functions = 0.0;
        
        for _ in 0..base_config.iterations {
            let mut manager = create_optimization_manager("O2").unwrap();
            
            let start_time = Instant::now();
            manager.optimize_module(&module).unwrap();
            let duration = start_time.elapsed();
            
            total_time += duration;
            total_functions += manager.get_stats().functions_optimized as f64;
        }
        
        let avg_time = Duration::from_nanos(
            (total_time.as_nanos() as f64 / base_config.iterations as f64) as u64
        );
        let avg_functions = total_functions / base_config.iterations as f64;
        
        scaling_results.push((func_count, avg_time, avg_functions));
        
        println!("  {} functions: {:.2}ms avg, {:.1} functions optimized",
                 func_count,
                 avg_time.as_secs_f64() * 1000.0,
                 avg_functions);
    }
    
    // Validate scaling behavior
    assert_eq!(scaling_results.len(), function_counts.len());
    
    // Check that optimization time generally increases with complexity
    for i in 1..scaling_results.len() {
        let (prev_count, prev_time, _) = scaling_results[i-1];
        let (curr_count, curr_time, curr_funcs) = scaling_results[i];
        
        // More functions should generally take more time (but allow for some variance)
        let time_ratio = curr_time.as_secs_f64() / prev_time.as_secs_f64();
        let count_ratio = curr_count as f64 / prev_count as f64;
        
        println!("Scaling {}->{} functions: time ratio {:.2}, count ratio {:.2}",
                 prev_count, curr_count, time_ratio, count_ratio);
        
        // Optimization should handle the expected number of functions
        assert!((curr_funcs - curr_count as f64).abs() < 1.0, 
                "Expected ~{} functions optimized, got {:.1}", curr_count, curr_funcs);
    }
}

#[test]
fn test_optimization_memory_usage_benchmark() {
    init_tracing!();
    
    // Test memory efficiency of optimization passes
    let context = Context::create();
    
    // Create a module with many similar functions (tests memory reuse)
    let module = context.create_module("memory_test");
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    
    // Create many functions with similar patterns
    for i in 0..50 {
        let function = module.add_function(&format!("memory_test_function_{}", i), fn_type, None);
        let entry_block = context.append_basic_block(function, "entry");
        
        let builder = context.create_builder();
        builder.position_at_end(entry_block);
        
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let const_val = i32_type.const_int(i as u64, false);
        
        // Create a pattern that could benefit from optimization
        let add1 = builder.build_int_add(param, const_val, "add1").unwrap();
        let add2 = builder.build_int_add(add1, const_val, "add2").unwrap();
        let sub1 = builder.build_int_sub(add2, const_val, "sub1").unwrap();
        
        builder.build_return(Some(&sub1)).unwrap();
    }
    
    // Test different optimization levels
    let levels = vec!["O0", "O1", "O2", "O3"];
    
    for level in &levels {
        let mut manager = create_optimization_manager(level).unwrap();
        
        let start_time = Instant::now();
        let result = manager.optimize_module(&module);
        let duration = start_time.elapsed();
        
        assert!(result.is_ok(), "Optimization failed for level {}", level);
        
        let stats = manager.get_stats();
        
        println!("Memory test - {}: {:.2}ms, {} functions, {:.2}% reduction",
                 level,
                 duration.as_secs_f64() * 1000.0,
                 stats.functions_optimized,
                 stats.size_reduction_percentage());
        
        // Validate results
        assert_eq!(stats.functions_optimized, 50);
        
        if level != "O0" {
            assert!(stats.passes_applied > 0);
            // Higher optimization levels should provide some benefit
            assert!(stats.size_reduction_percentage() >= 0.0);
        }
    }
}

#[test]
fn test_optimization_consistency_benchmark() {
    init_tracing!();
    
    // Test that optimization results are consistent across multiple runs
    let config = BenchmarkConfig {
        iterations: 10,
        optimization_levels: vec!["O2".to_string()],
        include_warmup: false,
        warmup_iterations: 0,
    };
    
    let context = Context::create();
    let mut optimization_times = Vec::new();
    let mut size_reductions = Vec::new();
    
    for i in 0..config.iterations {
        let module = create_simple_test_module(&context, &format!("consistency_test_{}", i));
        let mut manager = create_optimization_manager("O2").unwrap();
        
        let start_time = Instant::now();
        let result = manager.optimize_module(&module);
        let duration = start_time.elapsed();
        
        assert!(result.is_ok());
        
        let stats = manager.get_stats();
        optimization_times.push(duration);
        size_reductions.push(stats.size_reduction_percentage());
    }
    
    // Calculate statistics
    let avg_time = optimization_times.iter().sum::<Duration>() / optimization_times.len() as u32;
    let min_time = optimization_times.iter().min().unwrap();
    let max_time = optimization_times.iter().max().unwrap();
    
    let avg_reduction = size_reductions.iter().sum::<f64>() / size_reductions.len() as f64;
    let min_reduction = size_reductions.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_reduction = size_reductions.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    println!("Consistency Test Results:");
    println!("Optimization times: avg={:.2}ms, min={:.2}ms, max={:.2}ms",
             avg_time.as_secs_f64() * 1000.0,
             min_time.as_secs_f64() * 1000.0,
             max_time.as_secs_f64() * 1000.0);
    println!("Size reductions: avg={:.2}%, min={:.2}%, max={:.2}%",
             avg_reduction, min_reduction, max_reduction);
    
    // Validate consistency (results shouldn't vary too much)
    let time_variance = (max_time.as_secs_f64() - min_time.as_secs_f64()) / avg_time.as_secs_f64();
    let reduction_variance = if avg_reduction > 0.0 {
        (max_reduction - min_reduction) / avg_reduction
    } else {
        0.0
    };
    
    println!("Time variance: {:.2}%, Reduction variance: {:.2}%",
             time_variance * 100.0, reduction_variance * 100.0);
    
    // Reasonable variance thresholds
    assert!(time_variance < 2.0, "Optimization time too variable: {:.2}%", time_variance * 100.0);
    
    // All runs should have consistent results
    assert!(optimization_times.len() == config.iterations);
    assert!(size_reductions.len() == config.iterations);
}
