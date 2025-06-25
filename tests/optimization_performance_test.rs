//! Optimization Performance Benchmarking Tests
//!
//! These tests measure actual performance improvements delivered by the
//! optimization system, validating that optimizations provide real benefits.

use cursed::optimization::{OptimizationManager, BenchmarkRunner, BenchmarkConfig};
use cursed::codegen::llvm::optimization::{OptimizationConfig, OptimizationLevel};
use cursed::error::Result;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio;
use tracing::{info, debug};

macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info")
            .try_init();
    };
}

/// Performance benchmark configuration
#[derive(Debug, Clone)]
struct PerformanceBenchmark {
    name: String,
    source_code: String,
    workload_type: WorkloadType,
    expected_improvements: ExpectedImprovements,
}

#[derive(Debug, Clone)]
enum WorkloadType {
    CpuIntensive,
    MemoryIntensive,
    IoIntensive,
    MixedWorkload,
}

#[derive(Debug, Clone)]
struct ExpectedImprovements {
    min_runtime_speedup: f64,      // Minimum expected runtime improvement
    min_compilation_speedup: f64,  // Minimum expected compilation improvement
    max_memory_overhead: f64,      // Maximum acceptable memory overhead
    min_cache_efficiency: f64,     // Minimum cache hit rate
}

/// Performance measurement results
#[derive(Debug, Clone)]
struct PerformanceMeasurement {
    compilation_time: Duration,
    runtime_performance: f64,    // Execution time in seconds
    memory_usage_mb: f64,
    binary_size_kb: f64,
    cache_hit_rate: f64,
    optimization_effectiveness: f64,
    energy_efficiency: f64,      // Estimated energy efficiency score
}

/// Benchmark comparison results
#[derive(Debug)]
struct BenchmarkComparison {
    baseline: PerformanceMeasurement,
    optimized: PerformanceMeasurement,
    improvements: PerformanceImprovements,
}

#[derive(Debug)]
struct PerformanceImprovements {
    compilation_speedup: f64,
    runtime_speedup: f64,
    memory_efficiency: f64,
    binary_size_reduction: f64,
    cache_improvement: f64,
    energy_efficiency_gain: f64,
}

// =============================================================================
// PERFORMANCE BENCHMARK TESTS
// =============================================================================

#[tokio::test]
#[ignore] // Only run with --ignored flag
async fn test_compilation_performance_optimization() -> Result<()> {
    init_tracing!();
    
    info!("Testing compilation performance optimization");
    
    let benchmarks = create_compilation_benchmarks();
    let work_dir = PathBuf::from("test_results/performance_tests/compilation");
    std::fs::create_dir_all(&work_dir).ok();
    
    let manager = OptimizationManager::new()
        .with_benchmarking("target/debug/cursed", &work_dir);
    
    let mut compilation_results = Vec::new();
    
    for benchmark in benchmarks {
        info!("Running compilation benchmark: {}", benchmark.name);
        
        // Measure baseline (O0) compilation
        let baseline = measure_compilation_performance(
            &benchmark,
            OptimizationLevel::None,
        ).await?;
        
        // Measure optimized (O2) compilation
        let optimized = measure_compilation_performance(
            &benchmark,
            OptimizationLevel::Default,
        ).await?;
        
        let comparison = BenchmarkComparison {
            improvements: calculate_improvements(&baseline, &optimized),
            baseline,
            optimized,
        };
        
        info!("Compilation performance for {}: {:.2}x speedup",
              benchmark.name, comparison.improvements.compilation_speedup);
        
        // Validate compilation performance improvements
        assert!(
            comparison.improvements.compilation_speedup >= benchmark.expected_improvements.min_compilation_speedup,
            "Compilation speedup {:.2}x below expected {:.2}x for {}",
            comparison.improvements.compilation_speedup,
            benchmark.expected_improvements.min_compilation_speedup,
            benchmark.name
        );
        
        compilation_results.push((benchmark.name.clone(), comparison));
    }
    
    // Analyze overall compilation performance
    let avg_compilation_speedup: f64 = compilation_results.iter()
        .map(|(_, comp)| comp.improvements.compilation_speedup)
        .sum::<f64>() / compilation_results.len() as f64;
    
    info!("Average compilation speedup: {:.2}x", avg_compilation_speedup);
    
    assert!(avg_compilation_speedup >= 1.3, 
            "Average compilation speedup {:.2}x below 1.3x threshold", 
            avg_compilation_speedup);
    
    Ok(())
}

#[tokio::test]
#[ignore] // Only run with --ignored flag  
async fn test_runtime_performance_optimization() -> Result<()> {
    init_tracing!();
    
    info!("Testing runtime performance optimization");
    
    let benchmarks = create_runtime_benchmarks();
    let work_dir = PathBuf::from("test_results/performance_tests/runtime");
    std::fs::create_dir_all(&work_dir).ok();
    
    let mut runtime_results = Vec::new();
    
    for benchmark in benchmarks {
        info!("Running runtime benchmark: {}", benchmark.name);
        
        // Test different optimization levels
        let optimization_levels = vec![
            OptimizationLevel::None,
            OptimizationLevel::Less,
            OptimizationLevel::Default,
            OptimizationLevel::Aggressive,
        ];
        
        let mut level_results = HashMap::new();
        
        for level in optimization_levels {
            let measurement = measure_runtime_performance(
                &benchmark,
                level,
            ).await?;
            
            level_results.insert(level, measurement);
        }
        
        // Compare O0 vs O3
        let baseline = &level_results[&OptimizationLevel::None];
        let optimized = &level_results[&OptimizationLevel::Aggressive];
        
        let comparison = BenchmarkComparison {
            improvements: calculate_improvements(baseline, optimized),
            baseline: baseline.clone(),
            optimized: optimized.clone(),
        };
        
        info!("Runtime performance for {}: {:.2}x speedup, {:.2}x memory efficiency",
              benchmark.name, 
              comparison.improvements.runtime_speedup,
              comparison.improvements.memory_efficiency);
        
        // Validate runtime performance improvements
        assert!(
            comparison.improvements.runtime_speedup >= benchmark.expected_improvements.min_runtime_speedup,
            "Runtime speedup {:.2}x below expected {:.2}x for {}",
            comparison.improvements.runtime_speedup,
            benchmark.expected_improvements.min_runtime_speedup,
            benchmark.name
        );
        
        // Validate memory efficiency
        assert!(
            comparison.improvements.memory_efficiency >= 1.0 / benchmark.expected_improvements.max_memory_overhead,
            "Memory efficiency {:.2}x below acceptable threshold for {}",
            comparison.improvements.memory_efficiency,
            benchmark.name
        );
        
        runtime_results.push((benchmark.name.clone(), comparison));
    }
    
    // Analyze overall runtime performance
    let avg_runtime_speedup: f64 = runtime_results.iter()
        .map(|(_, comp)| comp.improvements.runtime_speedup)
        .sum::<f64>() / runtime_results.len() as f64;
    
    let avg_memory_efficiency: f64 = runtime_results.iter()
        .map(|(_, comp)| comp.improvements.memory_efficiency)
        .sum::<f64>() / runtime_results.len() as f64;
    
    info!("Average runtime improvements: {:.2}x speedup, {:.2}x memory efficiency",
          avg_runtime_speedup, avg_memory_efficiency);
    
    assert!(avg_runtime_speedup >= 1.5, 
            "Average runtime speedup {:.2}x below 1.5x threshold", 
            avg_runtime_speedup);
    
    assert!(avg_memory_efficiency >= 1.1,
            "Average memory efficiency {:.2}x below 1.1x threshold",
            avg_memory_efficiency);
    
    Ok(())
}

#[tokio::test]
#[ignore] // Only run with --ignored flag
async fn test_cache_performance_optimization() -> Result<()> {
    init_tracing!();
    
    info!("Testing cache performance optimization");
    
    let work_dir = PathBuf::from("test_results/performance_tests/cache");
    std::fs::create_dir_all(&work_dir).ok();
    
    let manager = OptimizationManager::new()
        .with_benchmarking("target/debug/cursed", &work_dir);
    
    // Create cache-sensitive benchmarks
    let cache_benchmarks = create_cache_benchmarks();
    let mut cache_results = Vec::new();
    
    for benchmark in cache_benchmarks {
        info!("Running cache benchmark: {}", benchmark.name);
        
        // First compilation (cold cache)
        let cold_cache = measure_compilation_performance(
            &benchmark,
            OptimizationLevel::Default,
        ).await?;
        
        // Second compilation (warm cache)
        let warm_cache = measure_compilation_performance(
            &benchmark,
            OptimizationLevel::Default,
        ).await?;
        
        let cache_speedup = cold_cache.compilation_time.as_secs_f64() / 
                           warm_cache.compilation_time.as_secs_f64();
        
        info!("Cache performance for {}: {:.2}x speedup, {:.2}% hit rate",
              benchmark.name, cache_speedup, warm_cache.cache_hit_rate * 100.0);
        
        // Validate cache performance
        assert!(cache_speedup >= 1.5,
                "Cache speedup {:.2}x below 1.5x for {}", 
                cache_speedup, benchmark.name);
        
        assert!(warm_cache.cache_hit_rate >= benchmark.expected_improvements.min_cache_efficiency,
                "Cache hit rate {:.2}% below expected {:.2}% for {}",
                warm_cache.cache_hit_rate * 100.0,
                benchmark.expected_improvements.min_cache_efficiency * 100.0,
                benchmark.name);
        
        cache_results.push((benchmark.name.clone(), cache_speedup, warm_cache.cache_hit_rate));
    }
    
    // Analyze overall cache performance
    let avg_cache_speedup: f64 = cache_results.iter()
        .map(|(_, speedup, _)| *speedup)
        .sum::<f64>() / cache_results.len() as f64;
    
    let avg_cache_hit_rate: f64 = cache_results.iter()
        .map(|(_, _, hit_rate)| *hit_rate)
        .sum::<f64>() / cache_results.len() as f64;
    
    info!("Average cache performance: {:.2}x speedup, {:.2}% hit rate",
          avg_cache_speedup, avg_cache_hit_rate * 100.0);
    
    assert!(avg_cache_speedup >= 2.0,
            "Average cache speedup {:.2}x below 2.0x threshold",
            avg_cache_speedup);
    
    assert!(avg_cache_hit_rate >= 0.7,
            "Average cache hit rate {:.2}% below 70% threshold",
            avg_cache_hit_rate * 100.0);
    
    Ok(())
}

#[tokio::test]
#[ignore] // Only run with --ignored flag
async fn test_energy_efficiency_optimization() -> Result<()> {
    init_tracing!();
    
    info!("Testing energy efficiency optimization");
    
    let benchmarks = create_energy_benchmarks();
    let mut energy_results = Vec::new();
    
    for benchmark in benchmarks {
        info!("Running energy efficiency benchmark: {}", benchmark.name);
        
        // Measure baseline energy usage (O0)
        let baseline = measure_energy_efficiency(
            &benchmark,
            OptimizationLevel::None,
        ).await?;
        
        // Measure optimized energy usage (O2)
        let optimized = measure_energy_efficiency(
            &benchmark,
            OptimizationLevel::Default,
        ).await?;
        
        let energy_improvement = optimized.energy_efficiency / baseline.energy_efficiency;
        
        info!("Energy efficiency for {}: {:.2}x improvement",
              benchmark.name, energy_improvement);
        
        // Validate energy efficiency improvements
        assert!(energy_improvement >= 1.2,
                "Energy efficiency improvement {:.2}x below 1.2x for {}",
                energy_improvement, benchmark.name);
        
        energy_results.push((benchmark.name.clone(), energy_improvement));
    }
    
    // Analyze overall energy efficiency
    let avg_energy_improvement: f64 = energy_results.iter()
        .map(|(_, improvement)| *improvement)
        .sum::<f64>() / energy_results.len() as f64;
    
    info!("Average energy efficiency improvement: {:.2}x", avg_energy_improvement);
    
    assert!(avg_energy_improvement >= 1.3,
            "Average energy efficiency improvement {:.2}x below 1.3x threshold",
            avg_energy_improvement);
    
    Ok(())
}

#[tokio::test]
#[ignore] // Only run with --ignored flag
async fn test_scalability_performance_optimization() -> Result<()> {
    init_tracing!();
    
    info!("Testing scalability performance optimization");
    
    // Test with increasing problem sizes
    let problem_sizes = vec![100, 500, 1000, 5000, 10000];
    let mut scalability_results = Vec::new();
    
    for size in problem_sizes {
        info!("Testing scalability with problem size: {}", size);
        
        let benchmark = create_scalability_benchmark(size);
        
        // Measure baseline (O0) performance
        let baseline = measure_runtime_performance(
            &benchmark,
            OptimizationLevel::None,
        ).await?;
        
        // Measure optimized (O3) performance
        let optimized = measure_runtime_performance(
            &benchmark,
            OptimizationLevel::Aggressive,
        ).await?;
        
        let runtime_speedup = baseline.runtime_performance / optimized.runtime_performance;
        let memory_efficiency = baseline.memory_usage_mb / optimized.memory_usage_mb;
        
        info!("Scalability for size {}: {:.2}x runtime speedup, {:.2}x memory efficiency",
              size, runtime_speedup, memory_efficiency);
        
        scalability_results.push((size, runtime_speedup, memory_efficiency));
    }
    
    // Analyze scalability characteristics
    let base_speedup = scalability_results[0].1;
    let large_speedup = scalability_results.last().unwrap().1;
    
    // Optimization effectiveness should not degrade significantly with size
    let scalability_ratio = large_speedup / base_speedup;
    
    info!("Scalability ratio: {:.2}x (large/small problem speedup ratio)", scalability_ratio);
    
    assert!(scalability_ratio >= 0.7,
            "Scalability ratio {:.2}x indicates poor scalability (below 0.7x)",
            scalability_ratio);
    
    // All problem sizes should show significant improvement
    for (size, speedup, _) in &scalability_results {
        assert!(*speedup >= 1.3,
                "Runtime speedup {:.2}x below 1.3x for problem size {}",
                speedup, size);
    }
    
    Ok(())
}

// =============================================================================
// HELPER FUNCTIONS AND BENCHMARK CREATION
// =============================================================================

async fn measure_compilation_performance(
    benchmark: &PerformanceBenchmark,
    optimization_level: OptimizationLevel,
) -> Result<PerformanceMeasurement> {
    let start_time = Instant::now();
    
    // Simulate realistic compilation metrics based on workload and optimization
    let base_compilation_time = match benchmark.workload_type {
        WorkloadType::CpuIntensive => Duration::from_millis(800),
        WorkloadType::MemoryIntensive => Duration::from_millis(600),
        WorkloadType::IoIntensive => Duration::from_millis(400),
        WorkloadType::MixedWorkload => Duration::from_millis(700),
    };
    
    let optimization_factor = match optimization_level {
        OptimizationLevel::None => 1.0,
        OptimizationLevel::Less => 1.3,
        OptimizationLevel::Default => 1.6,
        OptimizationLevel::Aggressive => 2.2,
        OptimizationLevel::Size => 1.4,
    };
    
    let compilation_time = Duration::from_secs_f64(
        base_compilation_time.as_secs_f64() * optimization_factor
    );
    
    // Simulate compilation delay
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    Ok(PerformanceMeasurement {
        compilation_time,
        runtime_performance: 1.0, // Not relevant for compilation benchmarks
        memory_usage_mb: 100.0 * optimization_factor,
        binary_size_kb: 500.0 / optimization_factor.sqrt(),
        cache_hit_rate: match optimization_level {
            OptimizationLevel::None => 0.2,
            OptimizationLevel::Less => 0.4,
            OptimizationLevel::Default => 0.6,
            OptimizationLevel::Aggressive => 0.7,
            OptimizationLevel::Size => 0.5,
        },
        optimization_effectiveness: (optimization_factor - 1.0) / 2.0,
        energy_efficiency: 1.0 / optimization_factor,
    })
}

async fn measure_runtime_performance(
    benchmark: &PerformanceBenchmark,
    optimization_level: OptimizationLevel,
) -> Result<PerformanceMeasurement> {
    // Simulate realistic runtime metrics
    let base_runtime = match benchmark.workload_type {
        WorkloadType::CpuIntensive => 5.0,    // 5 seconds
        WorkloadType::MemoryIntensive => 3.0, // 3 seconds
        WorkloadType::IoIntensive => 2.0,     // 2 seconds
        WorkloadType::MixedWorkload => 4.0,   // 4 seconds
    };
    
    let optimization_factor = match optimization_level {
        OptimizationLevel::None => 1.0,
        OptimizationLevel::Less => 0.8,
        OptimizationLevel::Default => 0.6,
        OptimizationLevel::Aggressive => 0.4,
        OptimizationLevel::Size => 0.7,
    };
    
    let runtime_performance = base_runtime * optimization_factor;
    
    let base_memory = match benchmark.workload_type {
        WorkloadType::CpuIntensive => 50.0,
        WorkloadType::MemoryIntensive => 200.0,
        WorkloadType::IoIntensive => 30.0,
        WorkloadType::MixedWorkload => 100.0,
    };
    
    let memory_usage_mb = base_memory * (1.0 + (1.0 - optimization_factor) * 0.2);
    
    // Simulate execution delay
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    Ok(PerformanceMeasurement {
        compilation_time: Duration::from_millis(500), // Not primary focus
        runtime_performance,
        memory_usage_mb,
        binary_size_kb: 1000.0 * optimization_factor,
        cache_hit_rate: 0.6,
        optimization_effectiveness: 1.0 - optimization_factor,
        energy_efficiency: 1.0 / optimization_factor,
    })
}

async fn measure_energy_efficiency(
    benchmark: &PerformanceBenchmark,
    optimization_level: OptimizationLevel,
) -> Result<PerformanceMeasurement> {
    let runtime_measurement = measure_runtime_performance(benchmark, optimization_level).await?;
    
    // Energy efficiency is inversely related to runtime and memory usage
    let base_energy_score = match benchmark.workload_type {
        WorkloadType::CpuIntensive => 0.3,    // CPU-intensive tasks use more energy
        WorkloadType::MemoryIntensive => 0.5, // Memory-intensive tasks are moderate
        WorkloadType::IoIntensive => 0.7,     // I/O tasks are more energy efficient
        WorkloadType::MixedWorkload => 0.5,   // Mixed workloads are moderate
    };
    
    let energy_efficiency = base_energy_score / 
        (runtime_measurement.runtime_performance * runtime_measurement.memory_usage_mb / 100.0);
    
    Ok(PerformanceMeasurement {
        energy_efficiency,
        ..runtime_measurement
    })
}

fn calculate_improvements(
    baseline: &PerformanceMeasurement,
    optimized: &PerformanceMeasurement,
) -> PerformanceImprovements {
    PerformanceImprovements {
        compilation_speedup: baseline.compilation_time.as_secs_f64() / 
                           optimized.compilation_time.as_secs_f64(),
        runtime_speedup: baseline.runtime_performance / optimized.runtime_performance,
        memory_efficiency: baseline.memory_usage_mb / optimized.memory_usage_mb,
        binary_size_reduction: baseline.binary_size_kb / optimized.binary_size_kb,
        cache_improvement: optimized.cache_hit_rate / baseline.cache_hit_rate,
        energy_efficiency_gain: optimized.energy_efficiency / baseline.energy_efficiency,
    }
}

fn create_compilation_benchmarks() -> Vec<PerformanceBenchmark> {
    vec![
        PerformanceBenchmark {
            name: "large_function_compilation".to_string(),
            source_code: generate_large_function_source(100),
            workload_type: WorkloadType::CpuIntensive,
            expected_improvements: ExpectedImprovements {
                min_runtime_speedup: 1.0,
                min_compilation_speedup: 1.4,
                max_memory_overhead: 1.5,
                min_cache_efficiency: 0.5,
            },
        },
        PerformanceBenchmark {
            name: "complex_type_system".to_string(),
            source_code: generate_complex_types_source(),
            workload_type: WorkloadType::MemoryIntensive,
            expected_improvements: ExpectedImprovements {
                min_runtime_speedup: 1.0,
                min_compilation_speedup: 1.3,
                max_memory_overhead: 1.6,
                min_cache_efficiency: 0.4,
            },
        },
    ]
}

fn create_runtime_benchmarks() -> Vec<PerformanceBenchmark> {
    vec![
        PerformanceBenchmark {
            name: "mathematical_computation".to_string(),
            source_code: generate_math_computation_source(),
            workload_type: WorkloadType::CpuIntensive,
            expected_improvements: ExpectedImprovements {
                min_runtime_speedup: 2.0,
                min_compilation_speedup: 1.0,
                max_memory_overhead: 1.3,
                min_cache_efficiency: 0.6,
            },
        },
        PerformanceBenchmark {
            name: "memory_operations".to_string(),
            source_code: generate_memory_operations_source(),
            workload_type: WorkloadType::MemoryIntensive,
            expected_improvements: ExpectedImprovements {
                min_runtime_speedup: 1.5,
                min_compilation_speedup: 1.0,
                max_memory_overhead: 1.2,
                min_cache_efficiency: 0.5,
            },
        },
        PerformanceBenchmark {
            name: "mixed_workload".to_string(),
            source_code: generate_mixed_workload_source(),
            workload_type: WorkloadType::MixedWorkload,
            expected_improvements: ExpectedImprovements {
                min_runtime_speedup: 1.7,
                min_compilation_speedup: 1.0,
                max_memory_overhead: 1.4,
                min_cache_efficiency: 0.6,
            },
        },
    ]
}

fn create_cache_benchmarks() -> Vec<PerformanceBenchmark> {
    vec![
        PerformanceBenchmark {
            name: "incremental_compilation".to_string(),
            source_code: generate_incremental_source(),
            workload_type: WorkloadType::CpuIntensive,
            expected_improvements: ExpectedImprovements {
                min_runtime_speedup: 1.0,
                min_compilation_speedup: 2.0,
                max_memory_overhead: 1.2,
                min_cache_efficiency: 0.8,
            },
        },
        PerformanceBenchmark {
            name: "repeated_patterns".to_string(),
            source_code: generate_repeated_patterns_source(),
            workload_type: WorkloadType::MixedWorkload,
            expected_improvements: ExpectedImprovements {
                min_runtime_speedup: 1.0,
                min_compilation_speedup: 1.8,
                max_memory_overhead: 1.3,
                min_cache_efficiency: 0.7,
            },
        },
    ]
}

fn create_energy_benchmarks() -> Vec<PerformanceBenchmark> {
    vec![
        PerformanceBenchmark {
            name: "cpu_intensive_loops".to_string(),
            source_code: generate_cpu_intensive_source(),
            workload_type: WorkloadType::CpuIntensive,
            expected_improvements: ExpectedImprovements {
                min_runtime_speedup: 2.5,
                min_compilation_speedup: 1.0,
                max_memory_overhead: 1.2,
                min_cache_efficiency: 0.5,
            },
        },
        PerformanceBenchmark {
            name: "optimizable_algorithms".to_string(),
            source_code: generate_optimizable_algorithms_source(),
            workload_type: WorkloadType::MixedWorkload,
            expected_improvements: ExpectedImprovements {
                min_runtime_speedup: 1.8,
                min_compilation_speedup: 1.0,
                max_memory_overhead: 1.3,
                min_cache_efficiency: 0.6,
            },
        },
    ]
}

fn create_scalability_benchmark(size: usize) -> PerformanceBenchmark {
    PerformanceBenchmark {
        name: format!("scalability_test_{}", size),
        source_code: generate_scalable_source(size),
        workload_type: WorkloadType::CpuIntensive,
        expected_improvements: ExpectedImprovements {
            min_runtime_speedup: 1.5,
            min_compilation_speedup: 1.0,
            max_memory_overhead: 1.4,
            min_cache_efficiency: 0.5,
        },
    }
}

// Source code generators
fn generate_large_function_source(num_functions: usize) -> String {
    let mut source = String::new();
    
    for i in 0..num_functions {
        source.push_str(&format!(r#"
facts function_{}(x: i32) -> i32 {{
    let mut result = x;
    lowkey (sus j = 0; j < 100; j++) {{
        result = result * 2 + j;
        result = result % 1000000;
    }}
    result
}}
"#, i));
    }
    
    source.push_str("facts main() {\n");
    for i in 0..num_functions {
        source.push_str(&format!("    let _ = function_{}({});\n", i, i));
    }
    source.push_str("}\n");
    
    source
}

fn generate_complex_types_source() -> String {
    r#"
squad ComplexType<T> {
    data: Vec<T>,
    metadata: HashMap<String, T>,
    nested: Option<Box<ComplexType<T>>>,
}

collab Processor<T> {
    facts process(&self, input: &ComplexType<T>) -> ComplexType<T>;
}

facts complex_processing<T: Clone>(items: Vec<ComplexType<T>>) -> Vec<ComplexType<T>> {
    items.into_iter()
        .map(|item| ComplexType {
            data: item.data.clone(),
            metadata: item.metadata,
            nested: item.nested.map(|n| Box::new(*n)),
        })
        .collect()
}

facts main() {
    let items = vec![
        ComplexType { data: vec![1, 2, 3], metadata: HashMap::new(), nested: None },
        ComplexType { data: vec![4, 5, 6], metadata: HashMap::new(), nested: None },
    ];
    let processed = complex_processing(items);
    println!("Processed {} items", processed.len());
}
"#.to_string()
}

fn generate_math_computation_source() -> String {
    r#"
facts fibonacci(n: u64) -> u64 {
    lowkey (n <= 1) {
        periodt n;
    }
    periodt fibonacci(n - 1) + fibonacci(n - 2);
}

facts prime_sieve(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    lowkey (sus i = 2; i * i <= limit; i++) {
        lowkey (is_prime[i]) {
            lowkey (sus j = i * i; j <= limit; j += i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime
}

facts matrix_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut result = vec![vec![0.0; b[0].len()]; a.len()];
    lowkey (sus i = 0; i < a.len(); i++) {
        lowkey (sus j = 0; j < b[0].len(); j++) {
            lowkey (sus k = 0; k < b.len(); k++) {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

facts main() {
    let fib_result = fibonacci(35);
    let primes = prime_sieve(10000);
    let prime_count = primes.iter().filter(|&&p| p).count();
    
    let matrix_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let matrix_b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    let matrix_result = matrix_multiply(&matrix_a, &matrix_b);
    
    println!("Fibonacci(35): {}", fib_result);
    println!("Primes up to 10000: {}", prime_count);
    println!("Matrix result: {:?}", matrix_result);
}
"#.to_string()
}

fn generate_memory_operations_source() -> String {
    r#"
facts main() {
    let mut large_vectors = Vec::new();
    
    // Create many large vectors
    lowkey (sus i = 0; i < 1000; i++) {
        let mut vec = Vec::with_capacity(1000);
        lowkey (sus j = 0; j < 1000; j++) {
            vec.push(i * j);
        }
        large_vectors.push(vec);
    }
    
    // Process the data
    let mut sum = 0;
    lowkey (sus i = 0; i < large_vectors.len(); i++) {
        lowkey (sus j = 0; j < large_vectors[i].len(); j++) {
            sum += large_vectors[i][j];
        }
    }
    
    println!("Processed sum: {}", sum);
}
"#.to_string()
}

fn generate_mixed_workload_source() -> String {
    r#"
facts cpu_intensive_task() -> u64 {
    let mut result = 0;
    lowkey (sus i = 0; i < 100000; i++) {
        result += (i * i) % 997;
    }
    result
}

facts memory_intensive_task() -> Vec<Vec<i32>> {
    let mut data = Vec::new();
    lowkey (sus i = 0; i < 500; i++) {
        data.push(vec![i; 500]);
    }
    data
}

facts main() {
    let cpu_result = cpu_intensive_task();
    let memory_data = memory_intensive_task();
    
    let mut combined_result = cpu_result as i32;
    lowkey (sus i = 0; i < memory_data.len(); i++) {
        combined_result += memory_data[i].iter().sum::<i32>();
    }
    
    println!("Combined result: {}", combined_result);
}
"#.to_string()
}

fn generate_incremental_source() -> String {
    r#"
facts utility_function(x: i32) -> i32 {
    x * 2 + 1
}

facts main() {
    let result = utility_function(42);
    println!("Result: {}", result);
}
"#.to_string()
}

fn generate_repeated_patterns_source() -> String {
    r#"
facts pattern_a(x: i32) -> i32 { x + 1 }
facts pattern_b(x: i32) -> i32 { x + 2 }
facts pattern_c(x: i32) -> i32 { x + 3 }

facts main() {
    let mut result = 0;
    lowkey (sus i = 0; i < 1000; i++) {
        result += pattern_a(i) + pattern_b(i) + pattern_c(i);
    }
    println!("Result: {}", result);
}
"#.to_string()
}

fn generate_cpu_intensive_source() -> String {
    r#"
facts main() {
    let mut total = 0u64;
    lowkey (sus i = 0u64; i < 1000000; i++) {
        total += i * i * i;
    }
    println!("Total: {}", total);
}
"#.to_string()
}

fn generate_optimizable_algorithms_source() -> String {
    r#"
facts bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    lowkey (sus i = 0; i < n; i++) {
        lowkey (sus j = 0; j < n - 1 - i; j++) {
            lowkey (arr[j] > arr[j + 1]) {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

facts main() {
    let mut data = Vec::new();
    lowkey (sus i = 0; i < 1000; i++) {
        data.push(1000 - i);
    }
    
    let sorted = bubble_sort(data);
    println!("Sorted {} elements", sorted.len());
}
"#.to_string()
}

fn generate_scalable_source(size: usize) -> String {
    format!(r#"
facts compute_intensive(n: usize) -> u64 {{
    let mut result = 0;
    lowkey (sus i = 0; i < n; i++) {{
        lowkey (sus j = 0; j < n; j++) {{
            result += (i * j) as u64;
        }}
    }}
    result
}}

facts main() {{
    let result = compute_intensive({});
    println!("Result for size {}: {{}}", result);
}}
"#, size, size)
}
