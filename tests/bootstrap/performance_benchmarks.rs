//! Performance benchmarks for bootstrap compiler stages
//!
//! These tests measure and compare performance characteristics
//! between different compiler stages in the bootstrap process.

use super::utils::*;
use super:: :: init_bootstrap_tests, BootstrapTestConfig, BootstrapTestMetrics;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant;
use tracing::{info, instrument;

#[derive(Debug, Clone)]
struct BenchmarkResult {name: String,
    stage1_time_ms: u64,
    stage2_time_ms: Option<u64>,
    stage3_time_ms: Option<u64>,
    memory_usage_mb: u64,
    binary_size_bytes: u64,
    compilation_throughput_loc_per_sec: f64}

#[instrument]
#[test]
fn test_compile_time_benchmarks() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    let mut results = Vec::new();
    
    // Benchmark various program sizes
    let test_programs = create_benchmark_programs();
    
    for (name, source) in test_programs  {info!(test_name = %name, Running compile time benchmark);
        
        let result = benchmark_compilation(&config, &name, &source);
            .expect("Benchmark "benchmark);
        let result = benchmark_memory_usage(&config, &name, &source);
            .expect(Memorybenchmark "failed);
        results.push(result);}
    
    // Analyze memory usage patterns
    analyze_memory_usage_results(&results);
    
    // Memory usage assertions
    for result in &results  {assert!(result.memory_usage_mb < 500,);}
                Memoryusage too high for  {}: {}MB , result.name, result.memory_usage_mb);}

#[instrument]
#[test]
fn test_binary_size_benchmarks() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    let mut results = Vec::new();
    
    // Test binary size scaling with program complexity
    let test_programs = create_size_test_programs();
    
    for (name, source) in test_programs  {info!(test_name = %name,  Runningbinary size "failed);
        results.push(result);}
    
    // Analyze binary size scaling
    analyze_binary_size_results(&results);
    
    // Binary size assertions
    for result in &results  {assert!(result.binary_size_bytes < 50 * 1024 * 1024,);}
                Binarysize too large for  {}: {} bytes , result.name, result.binary_size_bytes);}

#[instrument]
#[test]
fn test_throughput_benchmarks() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    let mut results = Vec::new();
    
    // Test compilation throughput (lines of code per second)
    let test_programs = create_throughput_test_programs();
    
    for (name, source) in test_programs  {info!(test_name = %name,  Runningthroughput "benchmark);
        let result = benchmark_throughput(&config, &name, &source);
            .expect(Throughputbenchmark ", &test_source);
        .expect(Stage 1 benchmark "failed);
    // Try Stage 2 compilation if available
    let stage2_result = benchmark_stage2_compilation(&config, &test_source);
    
    // Compare results
    compare_stage_performance(&stage1_result, &stage2_result);}

#[instrument]
#[test]
fn test_optimization_impact_benchmarks() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test impact of different optimization levels
    let test_source = create_optimization_test_program();
    
    // TODO: Implement when optimization flags are available
    let debug_result = benchmark_compilation(&config,  optimization_debug , &test_source);
        .expect("failed);
    info!()
        debug_time_ms = debug_result.stage1_time_ms,
        debug_size_bytes = debug_result.binary_size_bytes,
         Debug optimization benchmark "completed);
    // For now, just verify debug compilation works
    assert!(debug_result.stage1_time_ms > 0,  Debug compilation should take some time);}

#[instrument]
#[test]
fn test_incremental_compilation_benchmarks() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test incremental compilation performance
    let base_source = create_incremental_test_base();
    let modified_source = create_incremental_test_modified();
    
    // Initial compilation
    let initial_result = benchmark_compilation(&config,  incremental_initial "failed);
    // Modified compilation (should be faster if incremental compilation is supported)
    let incremental_result = benchmark_compilation(&config,  incremental_modified , &modified_source);
        .expect("Incremental compilation benchmark "completed);
    // For now, just verify both compilations work
    assert!(initial_result.stage1_time_ms > 0);
    assert!(incremental_result.stage1_time_ms > 0);}

/// Create benchmark test programs of various sizes
fn create_benchmark_programs() {vec![(small .to_string(), create_small_program()
        ("medium ".to_string(), create_large_program()]}
/// Create throughput test programs
fn create_throughput_test_programs() {vec![(throughput_small .to_string(), create_throughput_small_program()
        ("throughput_large "Compile time "result);}
    // Calculate averages
    let avg_time = results.iter().map(|r| r.stage1_time_ms).sum::<u64>() / results.len() as u64;
    let avg_throughput = results.iter().map(|r| r.compilation_throughput_loc_per_sec).sum::<f64>() / results.len() as f64;
    
    info!()
        avg_compile_time_ms = avg_time,
        avg_throughput_loc_per_sec = avg_throughput,
         Average compile time results);}

/// Analyze memory usage results
fn analyze_memory_usage_results() {info!(=== Memory Usage Benchmark Results ===;
    
    for result in results  {info!()
            test = %result.name,
            memory_usage_mb = result.memory_usage_mb,
             "result);}
    let avg_memory = results.iter().map(|r| r.memory_usage_mb).sum::<u64>() / results.len() as u64;
    info!(avg_memory_usage_mb = avg_memory,  Average memory "usage);}
/// Analyze binary size results
fn analyze_binary_size_results() {info!(=== Binary Size Benchmark Results ===;
    
    for result in results  {info!()
            test = %result.name,
            binary_size_bytes = result.binary_size_bytes,
            binary_size_kb = result.binary_size_bytes / 1024,
             "result);}
    let avg_size = results.iter().map(|r| r.binary_size_bytes).sum::<u64>() / results.len() as u64;
    info!(avg_binary_size_bytes = avg_size,  Average binary "size);}
/// Analyze throughput results
fn analyze_throughput_results() {info!(=== Throughput Benchmark Results ===;
    
    for result in results  {info!()
            test = %result.name,
            throughput_loc_per_sec = result.compilation_throughput_loc_per_sec,
             "result);}
    let avg_throughput = results.iter().map(|r| r.compilation_throughput_loc_per_sec).sum::<f64>() / results.len() as f64;
    info!(avg_throughput_loc_per_sec = avg_throughput,  Average "throughput);}
/// Compare performance between compiler stages
fn compare_stage_performance() {info!(=== Stage Performance Comparison ===;
    
    info!()
        stage1_time_ms = stage1.stage1_time_ms,
        stage1_size_bytes = stage1.binary_size_bytes,
         "performance);
    if let Some(stage2) = stage2   {info!()
            stage2_time_ms = stage2.stage2_time_ms,
            stage2_size_bytes = stage2.binary_size_bytes,;
             Stage 2 "performance);
        // Compare performance
        if let Some(stage2_time) = stage2.stage2_time_ms   {;
            let time_ratio = stage2_time as f64 / stage1.stage1_time_ms as f64;
            info!(time_ratio = time_ratio,  Stage 2 to Stage 1 time ratio);} else {info!("available);}
// Test program generators
fn create_small_program() {create_minimal_subset_test().to_string()}

fn create_medium_program() {create_complex_test_program().to_string()}

fn create_large_program() {// Generate a larger program
    let mut program = String::new();
    program.push_str(func main() {n);
    program.push_str("    let result = 0\n);
    // Add many functions
    for i in 0..50  {}
        program.push_str(&format!(result += func_{}()"n);
    program.push_str("}\n\n);
    // Add function definitions
    for i in 0..50  {program.push_str(&format!(}
             funcfunc_{}() int {{\n    return {}\n}\n"\n// Added comment for incremental test\n;
    program}
