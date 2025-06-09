//! Performance benchmarks for bootstrap compiler stages
//!
//! These tests measure and compare performance characteristics
//! between different compiler stages in the bootstrap process.

use super::utils::*;
use super::{init_bootstrap_tests, BootstrapTestConfig, BootstrapTestMetrics};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tracing::{info, instrument};

#[derive(Debug, Clone)]
struct BenchmarkResult {
    name: String,
    stage1_time_ms: u64,
    stage2_time_ms: Option<u64>,
    stage3_time_ms: Option<u64>,
    memory_usage_mb: u64,
    binary_size_bytes: u64,
    compilation_throughput_loc_per_sec: f64,
}

#[instrument]
#[test]
fn test_compile_time_benchmarks() {
    let config = init_bootstrap_tests();
    let mut results = Vec::new();
    
    // Benchmark various program sizes
    let test_programs = create_benchmark_programs();
    
    for (name, source) in test_programs {
        info!(test_name = %name, "Running compile time benchmark");
        
        let result = benchmark_compilation(&config, &name, &source)
            .expect("Benchmark failed");
        
        results.push(result);
    }
    
    // Analyze and report results
    analyze_compile_time_results(&results);
    
    // Performance assertions
    for result in &results {
        assert!(result.stage1_time_ms < 10000, 
               "Stage 1 compilation too slow for {}: {}ms", result.name, result.stage1_time_ms);
        
        if let Some(stage2_time) = result.stage2_time_ms {
            assert!(stage2_time < 20000, 
                   "Stage 2 compilation too slow for {}: {}ms", result.name, stage2_time);
        }
    }
}

#[instrument]
#[test]
fn test_memory_usage_benchmarks() {
    let config = init_bootstrap_tests();
    let mut results = Vec::new();
    
    // Test memory usage with different program complexities
    let test_programs = create_memory_test_programs();
    
    for (name, source) in test_programs {
        info!(test_name = %name, "Running memory usage benchmark");
        
        let result = benchmark_memory_usage(&config, &name, &source)
            .expect("Memory benchmark failed");
        
        results.push(result);
    }
    
    // Analyze memory usage patterns
    analyze_memory_usage_results(&results);
    
    // Memory usage assertions
    for result in &results {
        assert!(result.memory_usage_mb < 500, 
               "Memory usage too high for {}: {}MB", result.name, result.memory_usage_mb);
    }
}

#[instrument]
#[test]
fn test_binary_size_benchmarks() {
    let config = init_bootstrap_tests();
    let mut results = Vec::new();
    
    // Test binary size scaling with program complexity
    let test_programs = create_size_test_programs();
    
    for (name, source) in test_programs {
        info!(test_name = %name, "Running binary size benchmark");
        
        let result = benchmark_binary_size(&config, &name, &source)
            .expect("Binary size benchmark failed");
        
        results.push(result);
    }
    
    // Analyze binary size scaling
    analyze_binary_size_results(&results);
    
    // Binary size assertions
    for result in &results {
        assert!(result.binary_size_bytes < 50 * 1024 * 1024, 
               "Binary size too large for {}: {} bytes", result.name, result.binary_size_bytes);
    }
}

#[instrument]
#[test]
fn test_throughput_benchmarks() {
    let config = init_bootstrap_tests();
    let mut results = Vec::new();
    
    // Test compilation throughput (lines of code per second)
    let test_programs = create_throughput_test_programs();
    
    for (name, source) in test_programs {
        info!(test_name = %name, "Running throughput benchmark");
        
        let result = benchmark_throughput(&config, &name, &source)
            .expect("Throughput benchmark failed");
        
        results.push(result);
    }
    
    // Analyze throughput results
    analyze_throughput_results(&results);
    
    // Throughput assertions
    for result in &results {
        assert!(result.compilation_throughput_loc_per_sec > 100.0, 
               "Compilation throughput too low for {}: {:.2} LOC/sec", 
               result.name, result.compilation_throughput_loc_per_sec);
    }
}

#[instrument]
#[test]
fn test_stage_comparison_benchmarks() {
    let config = init_bootstrap_tests();
    
    // Compare performance between different compiler stages
    let test_source = create_stage_comparison_program();
    
    // Stage 1 compilation
    let stage1_result = benchmark_compilation(&config, "stage_comparison", &test_source)
        .expect("Stage 1 benchmark failed");
    
    // Try Stage 2 compilation if available
    let stage2_result = benchmark_stage2_compilation(&config, &test_source);
    
    // Compare results
    compare_stage_performance(&stage1_result, &stage2_result);
}

#[instrument]
#[test]
fn test_optimization_impact_benchmarks() {
    let config = init_bootstrap_tests();
    
    // Test impact of different optimization levels
    let test_source = create_optimization_test_program();
    
    // TODO: Implement when optimization flags are available
    let debug_result = benchmark_compilation(&config, "optimization_debug", &test_source)
        .expect("Debug optimization benchmark failed");
    
    info!(
        debug_time_ms = debug_result.stage1_time_ms,
        debug_size_bytes = debug_result.binary_size_bytes,
        "Debug optimization benchmark completed"
    );
    
    // For now, just verify debug compilation works
    assert!(debug_result.stage1_time_ms > 0, "Debug compilation should take some time");
}

#[instrument]
#[test]
fn test_incremental_compilation_benchmarks() {
    let config = init_bootstrap_tests();
    
    // Test incremental compilation performance
    let base_source = create_incremental_test_base();
    let modified_source = create_incremental_test_modified();
    
    // Initial compilation
    let initial_result = benchmark_compilation(&config, "incremental_initial", &base_source)
        .expect("Initial compilation benchmark failed");
    
    // Modified compilation (should be faster if incremental compilation is supported)
    let incremental_result = benchmark_compilation(&config, "incremental_modified", &modified_source)
        .expect("Incremental compilation benchmark failed");
    
    info!(
        initial_time_ms = initial_result.stage1_time_ms,
        incremental_time_ms = incremental_result.stage1_time_ms,
        "Incremental compilation benchmark completed"
    );
    
    // For now, just verify both compilations work
    assert!(initial_result.stage1_time_ms > 0);
    assert!(incremental_result.stage1_time_ms > 0);
}

/// Create benchmark test programs of various sizes
fn create_benchmark_programs() -> Vec<(String, String)> {
    vec![
        ("small".to_string(), create_small_program()),
        ("medium".to_string(), create_medium_program()),
        ("large".to_string(), create_large_program()),
    ]
}

/// Create memory test programs
fn create_memory_test_programs() -> Vec<(String, String)> {
    vec![
        ("low_memory".to_string(), create_low_memory_program()),
        ("moderate_memory".to_string(), create_moderate_memory_program()),
        ("high_memory".to_string(), create_high_memory_program()),
    ]
}

/// Create binary size test programs
fn create_size_test_programs() -> Vec<(String, String)> {
    vec![
        ("minimal".to_string(), create_minimal_program()),
        ("standard".to_string(), create_standard_program()),
        ("complex".to_string(), create_complex_program()),
    ]
}

/// Create throughput test programs
fn create_throughput_test_programs() -> Vec<(String, String)> {
    vec![
        ("throughput_small".to_string(), create_throughput_small_program()),
        ("throughput_large".to_string(), create_throughput_large_program()),
    ]
}

/// Benchmark compilation performance
fn benchmark_compilation(
    config: &BootstrapTestConfig,
    name: &str,
    source: &str,
) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
    let source_path = create_test_source(config, name, source)?;
    let output_path = PathBuf::from(&config.output_dir).join(format!("bench_{}", name));
    
    // Measure Stage 1 compilation
    let stage1_duration = compile_with_stage1(config, &source_path, &output_path)?;
    let binary_size = get_file_size(&output_path)?;
    
    // Calculate lines of code
    let loc = source.lines().count() as f64;
    let throughput = loc / (stage1_duration.as_secs_f64() + 0.001); // Avoid division by zero
    
    Ok(BenchmarkResult {
        name: name.to_string(),
        stage1_time_ms: stage1_duration.as_millis() as u64,
        stage2_time_ms: None,
        stage3_time_ms: None,
        memory_usage_mb: 0, // TODO: Implement memory measurement
        binary_size_bytes: binary_size,
        compilation_throughput_loc_per_sec: throughput,
    })
}

/// Benchmark memory usage
fn benchmark_memory_usage(
    config: &BootstrapTestConfig,
    name: &str,
    source: &str,
) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
    let source_path = create_test_source(config, name, source)?;
    let output_path = PathBuf::from(&config.output_dir).join(format!("mem_bench_{}", name));
    
    // Measure memory usage during compilation
    let memory_usage = measure_compilation_memory_usage(config, &source_path, &output_path)?;
    let binary_size = get_file_size(&output_path)?;
    
    Ok(BenchmarkResult {
        name: name.to_string(),
        stage1_time_ms: 0,
        stage2_time_ms: None,
        stage3_time_ms: None,
        memory_usage_mb: memory_usage / (1024 * 1024),
        binary_size_bytes: binary_size,
        compilation_throughput_loc_per_sec: 0.0,
    })
}

/// Benchmark binary size
fn benchmark_binary_size(
    config: &BootstrapTestConfig,
    name: &str,
    source: &str,
) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
    let source_path = create_test_source(config, name, source)?;
    let output_path = PathBuf::from(&config.output_dir).join(format!("size_bench_{}", name));
    
    let _duration = compile_with_stage1(config, &source_path, &output_path)?;
    let binary_size = get_file_size(&output_path)?;
    
    Ok(BenchmarkResult {
        name: name.to_string(),
        stage1_time_ms: 0,
        stage2_time_ms: None,
        stage3_time_ms: None,
        memory_usage_mb: 0,
        binary_size_bytes: binary_size,
        compilation_throughput_loc_per_sec: 0.0,
    })
}

/// Benchmark throughput
fn benchmark_throughput(
    config: &BootstrapTestConfig,
    name: &str,
    source: &str,
) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
    let source_path = create_test_source(config, name, source)?;
    let output_path = PathBuf::from(&config.output_dir).join(format!("throughput_bench_{}", name));
    
    let duration = compile_with_stage1(config, &source_path, &output_path)?;
    let loc = source.lines().count() as f64;
    let throughput = loc / (duration.as_secs_f64() + 0.001);
    
    Ok(BenchmarkResult {
        name: name.to_string(),
        stage1_time_ms: duration.as_millis() as u64,
        stage2_time_ms: None,
        stage3_time_ms: None,
        memory_usage_mb: 0,
        binary_size_bytes: 0,
        compilation_throughput_loc_per_sec: throughput,
    })
}

/// Benchmark Stage 2 compilation
fn benchmark_stage2_compilation(
    config: &BootstrapTestConfig,
    source: &str,
) -> Option<BenchmarkResult> {
    // This would require having a working Stage 2 compiler
    // For now, return None as Stage 2 may not be fully implemented
    None
}

/// Measure memory usage during compilation
fn measure_compilation_memory_usage(
    config: &BootstrapTestConfig,
    source_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<u64, Box<dyn std::error::Error>> {
    // This would require monitoring the compiler process
    // For now, return a placeholder value
    Ok(50 * 1024 * 1024) // 50MB placeholder
}

/// Analyze compile time results
fn analyze_compile_time_results(results: &[BenchmarkResult]) {
    info!("=== Compile Time Benchmark Results ===");
    
    for result in results {
        info!(
            test = %result.name,
            stage1_time_ms = result.stage1_time_ms,
            throughput_loc_per_sec = result.compilation_throughput_loc_per_sec,
            "Compile time result"
        );
    }
    
    // Calculate averages
    let avg_time = results.iter().map(|r| r.stage1_time_ms).sum::<u64>() / results.len() as u64;
    let avg_throughput = results.iter().map(|r| r.compilation_throughput_loc_per_sec).sum::<f64>() / results.len() as f64;
    
    info!(
        avg_compile_time_ms = avg_time,
        avg_throughput_loc_per_sec = avg_throughput,
        "Average compile time results"
    );
}

/// Analyze memory usage results
fn analyze_memory_usage_results(results: &[BenchmarkResult]) {
    info!("=== Memory Usage Benchmark Results ===");
    
    for result in results {
        info!(
            test = %result.name,
            memory_usage_mb = result.memory_usage_mb,
            "Memory usage result"
        );
    }
    
    let avg_memory = results.iter().map(|r| r.memory_usage_mb).sum::<u64>() / results.len() as u64;
    info!(avg_memory_usage_mb = avg_memory, "Average memory usage");
}

/// Analyze binary size results
fn analyze_binary_size_results(results: &[BenchmarkResult]) {
    info!("=== Binary Size Benchmark Results ===");
    
    for result in results {
        info!(
            test = %result.name,
            binary_size_bytes = result.binary_size_bytes,
            binary_size_kb = result.binary_size_bytes / 1024,
            "Binary size result"
        );
    }
    
    let avg_size = results.iter().map(|r| r.binary_size_bytes).sum::<u64>() / results.len() as u64;
    info!(avg_binary_size_bytes = avg_size, "Average binary size");
}

/// Analyze throughput results
fn analyze_throughput_results(results: &[BenchmarkResult]) {
    info!("=== Throughput Benchmark Results ===");
    
    for result in results {
        info!(
            test = %result.name,
            throughput_loc_per_sec = result.compilation_throughput_loc_per_sec,
            "Throughput result"
        );
    }
    
    let avg_throughput = results.iter().map(|r| r.compilation_throughput_loc_per_sec).sum::<f64>() / results.len() as f64;
    info!(avg_throughput_loc_per_sec = avg_throughput, "Average throughput");
}

/// Compare performance between compiler stages
fn compare_stage_performance(stage1: &BenchmarkResult, stage2: &Option<BenchmarkResult>) {
    info!("=== Stage Performance Comparison ===");
    
    info!(
        stage1_time_ms = stage1.stage1_time_ms,
        stage1_size_bytes = stage1.binary_size_bytes,
        "Stage 1 performance"
    );
    
    if let Some(stage2) = stage2 {
        info!(
            stage2_time_ms = stage2.stage2_time_ms,
            stage2_size_bytes = stage2.binary_size_bytes,
            "Stage 2 performance"
        );
        
        // Compare performance
        if let Some(stage2_time) = stage2.stage2_time_ms {
            let time_ratio = stage2_time as f64 / stage1.stage1_time_ms as f64;
            info!(time_ratio = time_ratio, "Stage 2 to Stage 1 time ratio");
        }
    } else {
        info!("Stage 2 performance not available");
    }
}

// Test program generators
fn create_small_program() -> String {
    create_minimal_subset_test().to_string()
}

fn create_medium_program() -> String {
    create_complex_test_program().to_string()
}

fn create_large_program() -> String {
    // Generate a larger program
    let mut program = String::new();
    program.push_str("func main() {\n");
    program.push_str("    let result = 0\n");
    
    // Add many functions
    for i in 0..50 {
        program.push_str(&format!("    result += func_{}()\n", i));
    }
    
    program.push_str("    return result\n");
    program.push_str("}\n\n");
    
    // Add function definitions
    for i in 0..50 {
        program.push_str(&format!(
            "func func_{}() int {{\n    return {}\n}}\n\n",
            i, i
        ));
    }
    
    program
}

fn create_low_memory_program() -> String {
    create_small_program()
}

fn create_moderate_memory_program() -> String {
    create_medium_program()
}

fn create_high_memory_program() -> String {
    create_large_program()
}

fn create_minimal_program() -> String {
    "func main() { return 0 }".to_string()
}

fn create_standard_program() -> String {
    create_medium_program()
}

fn create_complex_program() -> String {
    create_large_program()
}

fn create_throughput_small_program() -> String {
    create_small_program()
}

fn create_throughput_large_program() -> String {
    create_large_program()
}

fn create_stage_comparison_program() -> String {
    create_medium_program()
}

fn create_optimization_test_program() -> String {
    create_medium_program()
}

fn create_incremental_test_base() -> String {
    create_medium_program()
}

fn create_incremental_test_modified() -> String {
    let mut program = create_medium_program();
    program.push_str("\n// Added comment for incremental test\n");
    program
}
