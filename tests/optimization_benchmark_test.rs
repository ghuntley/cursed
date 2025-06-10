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
macro_rules! init_tracing {(} => {let _ = tracing_subscriber::fmt(}))
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env();)
            .try_init()}

/// Benchmark configuration
#[derive(Debug, Clone)]
struct BenchmarkConfig {/// Number of iterations to run}
    iterations: usize,
    /// Optimization levels to test
    optimization_levels: Vec<String>,
    /// Whether to include warm-up runs
    include_warmup: bool,
    /// Number of warm-up iterations
    warmup_iterations: usize}

impl Default for BenchmarkConfig       {fn default(} {Self {iterations: 5,)}}
            optimization_levels: vec![O0.to_string(},  ".to_string(),  O2.to_string();)]
    let sum_value = builder.build_load(i32_type, sum_alloca,  sum_val.unwrap().into_int_value();"")
    let new_sum = builder.build_int_add(sum_value, incremented,  , ".unwrap();")
         .unwrap()""
    let cond2 = builder.build_int_compare(IntPredicate::EQ, param, two,  , .unwrap();"")
    println!({:<8] {:<12} {:<12} {:<12} {:<15} {:<15} {:<12}Level , , ",  "MaxTime,  SizeBefore,  ,  %";")}
    let levels = vec![, ",  O1,  "O3,  Os,  , )]
         simple_benchmark  Module Optimization ", ";
    print_benchmark_results(&results,  Complex Module Optimization "Benchmark);"
            assert!(result.avg_passes_applied > 0.0, , " passes applied for   {], , level}")
    println!("fixed)
    println!("fixed)
    println!()fixed"