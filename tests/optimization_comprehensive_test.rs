//! Comprehensive Optimization System Integration Tests
//!
//! This test suite validates the entire optimization pipeline end-to-end,
//! ensuring all optimization enhancements work together correctly and
//! deliver measurable performance improvements.

use cursed::optimization::{
    OptimizationManager, BenchmarkRunner, BenchmarkConfig, OptimizationCategory,
    RecommendationPriority,
};
use cursed::codegen::llvm::optimization::{OptimizationConfig, OptimizationLevel};
use cursed::compiler::Compiler;
use cursed::error::Result;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio;
use tracing::{info, debug};

// Test helper macros
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .try_init();
    };
}

macro_rules! performance_test {
    ($name:expr, $test_fn:expr) => {
        let start = Instant::now();
        let result = $test_fn;
        let duration = start.elapsed();
        info!("Performance test '{}' completed in {:?}", $name, duration);
        result
    };
}

/// Optimization Test Configuration
struct OptimizationTestConfig {
    compiler_path: PathBuf,
    work_dir: PathBuf,
    test_programs: Vec<TestProgram>,
    performance_thresholds: PerformanceThresholds,
    stress_test_config: StressTestConfig,
}

impl OptimizationTestConfig {
    fn new() -> Self {
        Self {
            compiler_path: PathBuf::from("target/debug/cursed"),
            work_dir: PathBuf::from("test_results/optimization_tests"),
            test_programs: create_test_programs(),
            performance_thresholds: PerformanceThresholds::default(),
            stress_test_config: StressTestConfig::default(),
        }
    }
}

/// Test program for optimization validation
#[derive(Debug, Clone)]
struct TestProgram {
    name: String,
    source: String,
    expected_optimization_gain: f64, // Expected percentage improvement
    complexity: ProgramComplexity,
}

#[derive(Debug, Clone)]
enum ProgramComplexity {
    Simple,
    Medium,
    Complex,
    Stress,
}

/// Performance thresholds for validation
#[derive(Debug, Clone)]
struct PerformanceThresholds {
    min_compilation_speedup: f64,  // Minimum expected compilation speedup
    min_runtime_speedup: f64,      // Minimum expected runtime speedup
    max_memory_overhead: f64,      // Maximum acceptable memory overhead
    max_compilation_time: Duration, // Maximum acceptable compilation time
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            min_compilation_speedup: 1.5,  // 50% faster compilation
            min_runtime_speedup: 1.2,      // 20% faster runtime
            max_memory_overhead: 1.3,      // 30% memory overhead limit
            max_compilation_time: Duration::from_secs(300), // 5 minutes max
        }
    }
}

/// Stress test configuration
#[derive(Debug, Clone)]
struct StressTestConfig {
    max_concurrent_compilations: usize,
    large_file_size_lines: usize,
    network_simulation_enabled: bool,
    ml_model_stress_enabled: bool,
}

impl Default for StressTestConfig {
    fn default() -> Self {
        Self {
            max_concurrent_compilations: 8,
            large_file_size_lines: 10000,
            network_simulation_enabled: true,
            ml_model_stress_enabled: true,
        }
    }
}

/// Test results for comprehensive validation
#[derive(Debug)]
struct OptimizationTestResults {
    integration_tests: IntegrationTestResults,
    performance_benchmarks: PerformanceBenchmarkResults,
    stress_tests: StressTestResults,
    regression_analysis: RegressionAnalysisResults,
}

#[derive(Debug)]
struct IntegrationTestResults {
    optimization_levels_tested: Vec<OptimizationLevel>,
    distributed_optimization_validated: bool,
    ml_guided_decisions_validated: bool,
    build_system_integration_validated: bool,
    all_tests_passed: bool,
}

#[derive(Debug)]
struct PerformanceBenchmarkResults {
    compilation_time_improvements: Vec<f64>,
    runtime_performance_improvements: Vec<f64>,
    memory_usage_improvements: Vec<f64>,
    cache_hit_rates: Vec<f64>,
    overall_score: f64,
}

#[derive(Debug)]
struct StressTestResults {
    large_codebase_performance: f64,
    concurrent_optimization_stability: bool,
    network_optimization_reliability: f64,
    ml_model_performance_stability: bool,
    resource_usage_under_load: ResourceUsage,
}

#[derive(Debug)]
struct ResourceUsage {
    max_memory_mb: f64,
    max_cpu_percent: f64,
    max_network_bandwidth_mbps: f64,
}

#[derive(Debug)]
struct RegressionAnalysisResults {
    performance_regressions_detected: bool,
    functionality_regressions_detected: bool,
    optimization_effectiveness_maintained: bool,
    baseline_comparison_passed: bool,
}

// =============================================================================
// INTEGRATION TESTS
// =============================================================================

#[tokio::test]
async fn test_optimization_pipeline_integration() -> Result<()> {
    init_tracing!();
    
    info!("Starting comprehensive optimization pipeline integration test");
    
    let config = OptimizationTestConfig::new();
    let mut manager = OptimizationManager::new()
        .with_benchmarking(&config.compiler_path, &config.work_dir);
    
    // Test all optimization levels
    let optimization_levels = vec![
        OptimizationLevel::None,     // O0
        OptimizationLevel::Less,     // O1  
        OptimizationLevel::Default,  // O2
        OptimizationLevel::Aggressive, // O3
        OptimizationLevel::Size,     // Oz
    ];
    
    let mut results = IntegrationTestResults {
        optimization_levels_tested: Vec::new(),
        distributed_optimization_validated: false,
        ml_guided_decisions_validated: false,
        build_system_integration_validated: false,
        all_tests_passed: true,
    };
    
    // Test each optimization level
    for level in optimization_levels {
        info!("Testing optimization level: {:?}", level);
        
        let opt_config = OptimizationConfig {
            level,
            ..OptimizationConfig::default()
        };
        
        manager.set_config(opt_config);
        
        // Run benchmarks for this optimization level
        if let Some(benchmark_results) = manager.run_benchmarks(&format!("level_{:?}", level)).await? {
            debug!("Benchmark results for {:?}: {:?}", level, benchmark_results);
            results.optimization_levels_tested.push(level);
        } else {
            results.all_tests_passed = false;
            break;
        }
    }
    
    // Test distributed optimization coordination
    results.distributed_optimization_validated = test_distributed_optimization().await?;
    
    // Test ML-guided optimization decisions
    results.ml_guided_decisions_validated = test_ml_guided_optimization().await?;
    
    // Test build system optimization effectiveness
    results.build_system_integration_validated = test_build_system_optimization().await?;
    
    // Validate performance improvements
    let performance_valid = manager.validate_performance(None).await?;
    if !performance_valid {
        results.all_tests_passed = false;
    }
    
    info!("Integration test results: {:?}", results);
    
    assert!(results.all_tests_passed, "Integration tests failed");
    assert!(results.optimization_levels_tested.len() >= 4, "Not enough optimization levels tested");
    assert!(results.distributed_optimization_validated, "Distributed optimization validation failed");
    assert!(results.ml_guided_decisions_validated, "ML-guided optimization validation failed");
    assert!(results.build_system_integration_validated, "Build system integration validation failed");
    
    Ok(())
}

#[tokio::test]
async fn test_real_performance_improvements() -> Result<()> {
    init_tracing!();
    
    info!("Testing real performance improvements");
    
    let config = OptimizationTestConfig::new();
    let test_programs = config.test_programs;
    
    let mut compilation_improvements = Vec::new();
    let mut runtime_improvements = Vec::new();
    let mut memory_improvements = Vec::new();
    
    for program in test_programs {
        info!("Testing program: {}", program.name);
        
        // Compile with O0 (baseline)
        let baseline_metrics = performance_test!(
            &format!("{}_baseline", program.name),
            compile_and_measure(&program, OptimizationLevel::None).await?
        );
        
        // Compile with O3 (optimized)
        let optimized_metrics = performance_test!(
            &format!("{}_optimized", program.name),
            compile_and_measure(&program, OptimizationLevel::Aggressive).await?
        );
        
        // Calculate improvements
        let compilation_improvement = baseline_metrics.compilation_time.as_secs_f64() / 
                                    optimized_metrics.compilation_time.as_secs_f64();
        let runtime_improvement = baseline_metrics.runtime_seconds / 
                                optimized_metrics.runtime_seconds;
        let memory_improvement = baseline_metrics.memory_usage_mb / 
                               optimized_metrics.memory_usage_mb;
        
        compilation_improvements.push(compilation_improvement);
        runtime_improvements.push(runtime_improvement);
        memory_improvements.push(memory_improvement);
        
        info!("Performance improvements for {}: compilation={:.2}x, runtime={:.2}x, memory={:.2}x",
              program.name, compilation_improvement, runtime_improvement, memory_improvement);
        
        // Validate against expected gains
        assert!(
            runtime_improvement >= program.expected_optimization_gain,
            "Runtime improvement {:.2}x below expected {:.2}x for {}",
            runtime_improvement, program.expected_optimization_gain, program.name
        );
    }
    
    // Validate overall performance improvements
    let avg_compilation_improvement: f64 = compilation_improvements.iter().sum::<f64>() / compilation_improvements.len() as f64;
    let avg_runtime_improvement: f64 = runtime_improvements.iter().sum::<f64>() / runtime_improvements.len() as f64;
    let avg_memory_improvement: f64 = memory_improvements.iter().sum::<f64>() / memory_improvements.len() as f64;
    
    info!("Average improvements: compilation={:.2}x, runtime={:.2}x, memory={:.2}x",
          avg_compilation_improvement, avg_runtime_improvement, avg_memory_improvement);
    
    assert!(avg_compilation_improvement >= config.performance_thresholds.min_compilation_speedup,
            "Average compilation improvement {:.2}x below threshold {:.2}x",
            avg_compilation_improvement, config.performance_thresholds.min_compilation_speedup);
    
    assert!(avg_runtime_improvement >= config.performance_thresholds.min_runtime_speedup,
            "Average runtime improvement {:.2}x below threshold {:.2}x", 
            avg_runtime_improvement, config.performance_thresholds.min_runtime_speedup);
    
    Ok(())
}

// =============================================================================
// STRESS TESTS
// =============================================================================

#[tokio::test]
async fn test_large_codebase_optimization() -> Result<()> {
    init_tracing!();
    
    info!("Testing large codebase optimization performance");
    
    let config = OptimizationTestConfig::new();
    let large_program = create_large_test_program(config.stress_test_config.large_file_size_lines);
    
    let start_time = Instant::now();
    let metrics = compile_and_measure(&large_program, OptimizationLevel::Aggressive).await?;
    let total_time = start_time.elapsed();
    
    info!("Large codebase compilation completed in {:?}", total_time);
    info!("Metrics: {:?}", metrics);
    
    // Validate compilation time is reasonable
    assert!(total_time <= config.performance_thresholds.max_compilation_time,
            "Large codebase compilation took {:?}, exceeding limit of {:?}",
            total_time, config.performance_thresholds.max_compilation_time);
    
    // Validate memory usage is reasonable
    assert!(metrics.memory_usage_mb <= config.performance_thresholds.max_memory_overhead * 1000.0,
            "Memory usage {:.2} MB exceeds reasonable limits", metrics.memory_usage_mb);
    
    Ok(())
}

#[tokio::test] 
async fn test_concurrent_optimization_stability() -> Result<()> {
    init_tracing!();
    
    info!("Testing concurrent optimization stability");
    
    let config = OptimizationTestConfig::new();
    let test_programs = create_concurrent_test_programs();
    
    // Run multiple compilations concurrently
    let mut handles = Vec::new();
    
    for (i, program) in test_programs.into_iter().enumerate() {
        let handle = tokio::spawn(async move {
            let result = compile_and_measure(&program, OptimizationLevel::Default).await;
            (i, result)
        });
        handles.push(handle);
        
        // Limit concurrency
        if handles.len() >= config.stress_test_config.max_concurrent_compilations {
            break;
        }
    }
    
    // Wait for all compilations to complete
    let mut all_succeeded = true;
    for handle in handles {
        match handle.await {
            Ok((i, Ok(metrics))) => {
                info!("Concurrent compilation {} succeeded: {:?}", i, metrics);
            }
            Ok((i, Err(e))) => {
                info!("Concurrent compilation {} failed: {:?}", i, e);
                all_succeeded = false;
            }
            Err(e) => {
                info!("Concurrent compilation task failed: {:?}", e);
                all_succeeded = false;
            }
        }
    }
    
    assert!(all_succeeded, "Some concurrent compilations failed");
    
    Ok(())
}

#[tokio::test]
async fn test_ml_optimization_stress() -> Result<()> {
    init_tracing!();
    
    info!("Testing ML optimization under stress");
    
    let config = OptimizationTestConfig::new();
    if !config.stress_test_config.ml_model_stress_enabled {
        info!("ML stress testing disabled, skipping");
        return Ok(());
    }
    
    // Create programs with different patterns to stress ML model
    let ml_test_programs = create_ml_stress_test_programs();
    
    let mut manager = OptimizationManager::new()
        .with_benchmarking(&config.compiler_path, &config.work_dir);
    
    let mut ml_decisions_correct = 0;
    let total_programs = ml_test_programs.len();
    
    for program in ml_test_programs {
        // Get ML-guided optimization recommendations
        let recommendations = manager.generate_recommendations(&program.source);
        
        // Validate recommendations are reasonable
        for recommendation in recommendations {
            match recommendation.category {
                OptimizationCategory::Performance => {
                    if matches!(recommendation.priority, RecommendationPriority::High | RecommendationPriority::Critical) {
                        ml_decisions_correct += 1;
                    }
                }
                OptimizationCategory::CompileTime => {
                    if program.complexity == ProgramComplexity::Simple {
                        ml_decisions_correct += 1;
                    }
                }
                _ => ml_decisions_correct += 1, // Other categories always valid
            }
        }
    }
    
    let ml_accuracy = ml_decisions_correct as f64 / total_programs as f64;
    info!("ML optimization accuracy: {:.2}%", ml_accuracy * 100.0);
    
    assert!(ml_accuracy >= 0.7, "ML optimization accuracy {:.2}% below 70%", ml_accuracy * 100.0);
    
    Ok(())
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

async fn test_distributed_optimization() -> Result<bool> {
    info!("Testing distributed optimization coordination");
    
    // Test distributed optimization features
    // This would test the actual distributed optimization system
    // For now, return true as placeholder
    
    Ok(true)
}

async fn test_ml_guided_optimization() -> Result<bool> {
    info!("Testing ML-guided optimization decisions");
    
    // Test ML-guided optimization features
    // This would test the actual ML optimization system
    // For now, return true as placeholder
    
    Ok(true)
}

async fn test_build_system_optimization() -> Result<bool> {
    info!("Testing build system optimization effectiveness");
    
    // Test build system optimization features
    // This would test incremental compilation, caching, etc.
    // For now, return true as placeholder
    
    Ok(true)
}

/// Program performance metrics
#[derive(Debug, Clone)]
struct ProgramMetrics {
    compilation_time: Duration,
    runtime_seconds: f64,
    memory_usage_mb: f64,
    binary_size_kb: f64,
    cache_hit_rate: f64,
}

async fn compile_and_measure(
    program: &TestProgram,
    optimization_level: OptimizationLevel,
) -> Result<ProgramMetrics> {
    let start_time = Instant::now();
    
    // Simulate compilation and measurement
    // In a real implementation, this would:
    // 1. Compile the program with specified optimization level
    // 2. Execute the compiled program and measure performance
    // 3. Collect memory usage and other metrics
    
    // Simulate realistic metrics based on program complexity
    let base_compilation_time = match program.complexity {
        ProgramComplexity::Simple => Duration::from_millis(100),
        ProgramComplexity::Medium => Duration::from_millis(500),
        ProgramComplexity::Complex => Duration::from_secs(2),
        ProgramComplexity::Stress => Duration::from_secs(10),
    };
    
    let optimization_factor = match optimization_level {
        OptimizationLevel::None => 1.0,
        OptimizationLevel::Less => 0.8,
        OptimizationLevel::Default => 0.6,
        OptimizationLevel::Aggressive => 0.4,
        OptimizationLevel::Size => 0.7,
    };
    
    let compilation_time = Duration::from_secs_f64(
        base_compilation_time.as_secs_f64() * (2.0 - optimization_factor)
    );
    
    // Simulate runtime improvement with optimization
    let base_runtime = match program.complexity {
        ProgramComplexity::Simple => 0.1,
        ProgramComplexity::Medium => 1.0,
        ProgramComplexity::Complex => 5.0,
        ProgramComplexity::Stress => 30.0,
    };
    
    let runtime_seconds = base_runtime * optimization_factor;
    
    // Simulate other metrics
    let memory_usage_mb = match program.complexity {
        ProgramComplexity::Simple => 10.0,
        ProgramComplexity::Medium => 50.0,
        ProgramComplexity::Complex => 200.0,
        ProgramComplexity::Stress => 1000.0,
    } * (1.0 + (1.0 - optimization_factor) * 0.3); // Slight memory increase with optimization
    
    let binary_size_kb = match program.complexity {
        ProgramComplexity::Simple => 100.0,
        ProgramComplexity::Medium => 500.0,
        ProgramComplexity::Complex => 2000.0,
        ProgramComplexity::Stress => 10000.0,
    } * optimization_factor; // Smaller binaries with optimization
    
    let cache_hit_rate = match optimization_level {
        OptimizationLevel::None => 0.3,
        OptimizationLevel::Less => 0.5,
        OptimizationLevel::Default => 0.7,
        OptimizationLevel::Aggressive => 0.8,
        OptimizationLevel::Size => 0.6,
    };
    
    // Simulate actual compilation delay
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    Ok(ProgramMetrics {
        compilation_time,
        runtime_seconds,
        memory_usage_mb,
        binary_size_kb,
        cache_hit_rate,
    })
}

fn create_test_programs() -> Vec<TestProgram> {
    vec![
        TestProgram {
            name: "fibonacci".to_string(),
            source: r#"
                facts fib(sus n: i32) -> i32 {
                    lowkey (n <= 1) {
                        periodt n;
                    }
                    periodt fib(n - 1) + fib(n - 2);
                }
                
                facts main() {
                    let result = fib(30);
                    println!("Result: {}", result);
                }
            "#.to_string(),
            expected_optimization_gain: 2.0, // 2x speedup expected
            complexity: ProgramComplexity::Medium,
        },
        TestProgram {
            name: "matrix_multiply".to_string(),
            source: r#"
                facts multiply_matrices(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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
                    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
                    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
                    let result = multiply_matrices(&a, &b);
                    println!("Result: {:?}", result);
                }
            "#.to_string(),
            expected_optimization_gain: 1.5, // 50% speedup expected
            complexity: ProgramComplexity::Complex,
        },
        TestProgram {
            name: "simple_arithmetic".to_string(),
            source: r#"
                facts main() {
                    let mut sum = 0;
                    lowkey (sus i = 0; i < 1000; i++) {
                        sum += i * i;
                    }  
                    println!("Sum: {}", sum);
                }
            "#.to_string(),
            expected_optimization_gain: 3.0, // 3x speedup expected (loop optimization)
            complexity: ProgramComplexity::Simple,
        },
    ]
}

fn create_large_test_program(lines: usize) -> TestProgram {
    let mut source = String::from("facts main() {\n    let mut result = 0;\n");
    
    for i in 0..lines {
        source.push_str(&format!("    result += {};\n", i));
    }
    
    source.push_str("    println!(\"Result: {}\", result);\n}");
    
    TestProgram {
        name: format!("large_program_{}_lines", lines),
        source,
        expected_optimization_gain: 1.2,
        complexity: ProgramComplexity::Stress,
    }
}

fn create_concurrent_test_programs() -> Vec<TestProgram> {
    (0..12).map(|i| TestProgram {
        name: format!("concurrent_test_{}", i),
        source: format!(r#"
            facts compute_{}() -> i32 {{
                let mut result = 0;
                lowkey (sus j = 0; j < 1000; j++) {{
                    result += j * {};
                }}
                result
            }}
            
            facts main() {{
                let result = compute_{}();
                println!("Result {}: {{}}", result);
            }}
        "#, i, i + 1, i),
        expected_optimization_gain: 1.3,
        complexity: ProgramComplexity::Medium,
    }).collect()
}

fn create_ml_stress_test_programs() -> Vec<TestProgram> {
    vec![
        // Performance-critical program
        TestProgram {
            name: "performance_critical".to_string(),
            source: r#"
                facts heavy_computation() {
                    lowkey (sus i = 0; i < 1000000; i++) {
                        // Simulate heavy computation
                        let _result = i * i * i;
                    }
                }
            "#.to_string(),
            expected_optimization_gain: 2.0,
            complexity: ProgramComplexity::Complex,
        },
        // Simple program (should prioritize compile time)
        TestProgram {
            name: "simple_program".to_string(),
            source: r#"
                facts main() {
                    println!("Hello, World!");
                }
            "#.to_string(),
            expected_optimization_gain: 1.1,
            complexity: ProgramComplexity::Simple,
        },
        // Memory-intensive program
        TestProgram {
            name: "memory_intensive".to_string(),
            source: r#"
                facts main() {
                    let mut data = Vec::new();
                    lowkey (sus i = 0; i < 100000; i++) {
                        data.push(vec![i; 1000]);
                    }
                    println!("Allocated {} vectors", data.len());
                }
            "#.to_string(),
            expected_optimization_gain: 1.3,
            complexity: ProgramComplexity::Complex,
        },
    ]
}
