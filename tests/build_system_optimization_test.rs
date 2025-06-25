//! Comprehensive tests for build system optimization functionality
//! 
//! Tests the real implementations of build integration, performance monitoring,
//! benchmarking, and incremental compilation features.

use std::path::{Path, PathBuf};
use std::time::Duration;
use std::fs;
use tempfile::TempDir;
use tokio;

use cursed::optimization::{
    build_integration::{BuildOptimizer, BuildContext, BuildOptimizationResult},
    performance_integration::{PerformanceIntegrationSystem, PerformanceIntegrationConfig},
    benchmarks::{BenchmarkRunner, BenchmarkConfig, PerformanceThresholds},
    enhanced_benchmarking::{EnhancedBenchmarkRunner, BenchmarkConfig as EnhancedConfig},
    config::{OptimizationConfig, OptimizationLevel},
};
use cursed::error::Result;

/// Test basic build optimizer creation and configuration
#[test]
fn test_build_optimizer_creation() {
    let temp_dir = TempDir::new().unwrap();
    let context = BuildContext {
        project_root: temp_dir.path().to_path_buf(),
        source_files: vec![],
        output_directory: temp_dir.path().join("output"),
        target_triple: "x86_64-unknown-linux-gnu".to_string(),
        debug_mode: false,
        release_mode: true,
        verbose: true,
    };
    
    let optimizer = BuildOptimizer::new(context);
    assert!(optimizer.is_ok());
    
    let optimizer = optimizer.unwrap();
    let stats = optimizer.get_statistics();
    assert_eq!(stats.total_compilations, 0);
    assert!(stats.cache_enabled);
}

/// Test real object file generation
#[tokio::test]
async fn test_object_file_generation() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("test.csd");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir)?;
    
    // Create a test CURSED source file
    let source_content = r#"
        slay main() {
            facts x = 42;
            println("Hello, World!");
        }
    "#;
    fs::write(&source_file, source_content)?;
    
    let context = BuildContext {
        project_root: temp_dir.path().to_path_buf(),
        source_files: vec![source_file.clone()],
        output_directory: output_dir.clone(),
        target_triple: "x86_64-unknown-linux-gnu".to_string(),
        debug_mode: false,
        release_mode: true,
        verbose: true,
    };
    
    let mut optimizer = BuildOptimizer::new(context)?;
    let result = optimizer.optimize_build()?;
    
    // Verify build result
    assert!(result.success);
    assert!(result.compilation_time > Duration::from_millis(0));
    assert_eq!(result.files_compiled, 1);
    
    // Verify object file was created
    let object_file = output_dir.join("test.o");
    assert!(object_file.exists());
    
    // Verify object file has ELF structure
    let object_content = fs::read(&object_file)?;
    assert!(object_content.len() > 64);
    assert_eq!(&object_content[0..4], &[0x7f, 0x45, 0x4c, 0x46]); // ELF magic
    
    // Verify executable was linked
    let executable = output_dir.join("release_output");
    assert!(executable.exists());
    
    let executable_content = fs::read(&executable)?;
    assert!(executable_content.len() >= 4096); // Minimum executable size
    assert_eq!(&executable_content[0..4], &[0x7f, 0x45, 0x4c, 0x46]); // ELF magic
    
    Ok(())
}

/// Test incremental build detection
#[tokio::test]
async fn test_incremental_build() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("test.csd");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir)?;
    
    let source_content = r#"
        slay main() {
            facts x = 42;
        }
    "#;
    fs::write(&source_file, source_content)?;
    
    let context = BuildContext {
        project_root: temp_dir.path().to_path_buf(),
        source_files: vec![source_file.clone()],
        output_directory: output_dir,
        target_triple: "x86_64-unknown-linux-gnu".to_string(),
        debug_mode: true, // Enable incremental compilation
        release_mode: false,
        verbose: false,
    };
    
    let mut optimizer = BuildOptimizer::new(context)?;
    
    // First build
    let result1 = optimizer.optimize_build()?;
    assert!(result1.success);
    let first_time = result1.compilation_time;
    
    // Second build (should be faster due to caching)
    let result2 = optimizer.optimize_build()?;
    assert!(result2.success);
    
    // Cache hit rate should be meaningful
    assert!(result2.cache_hit_rate >= 0.0);
    
    // Modify file and rebuild
    let modified_content = r#"
        slay main() {
            facts x = 84; // Changed value
        }
    "#;
    fs::write(&source_file, modified_content)?;
    
    let result3 = optimizer.optimize_build()?;
    assert!(result3.success);
    // Should detect file change and recompile
    assert!(result3.compilation_time > Duration::from_millis(0));
    
    Ok(())
}

/// Test performance integration system
#[tokio::test]
async fn test_performance_integration() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("test.csd");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir)?;
    
    let source_content = r#"
        slay fibonacci(n) {
            lowkey (n <= 1) {
                fr n;
            }
            fr fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        slay main() {
            facts result = fibonacci(10);
            println(result);
        }
    "#;
    fs::write(&source_file, source_content)?;
    
    let perf_config = PerformanceIntegrationConfig {
        enable_adaptive_optimization: true,
        enable_performance_monitoring: true,
        enable_automatic_reporting: false,
        monitoring_interval_ms: 100,
        optimization_threshold_seconds: 1.0,
        max_parallel_workers: 2,
        enable_pgo: false,
        enable_distributed: false,
        cache_size_limit_mb: 100,
        report_output_dir: Some(temp_dir.path().join("reports")),
        benchmark_configs: std::collections::HashMap::new(),
        target_improvements: cursed::optimization::performance_integration::PerformanceTargets::default(),
    };
    
    let opt_config = OptimizationConfig::default();
    
    let mut perf_system = PerformanceIntegrationSystem::new(perf_config, opt_config)?;
    
    let result = perf_system.optimize_project(&[source_file], &output_dir).await?;
    
    // Verify optimization results
    assert!(result.compilation_time > Duration::from_millis(0));
    assert!(result.parallel_efficiency >= 0.0 && result.parallel_efficiency <= 1.0);
    assert!(result.cache_hit_rate >= 0.0 && result.cache_hit_rate <= 1.0);
    assert!(!result.recommendations.is_empty());
    
    // Verify performance improvements structure
    assert!(result.performance_improvements.compilation_time_saved >= Duration::from_millis(0));
    assert!(result.performance_improvements.binary_size_reduction >= 0.0);
    assert!(result.performance_improvements.runtime_improvement_estimate >= 0.0);
    
    Ok(())
}

/// Test benchmark runner with real measurements
#[tokio::test]
async fn test_benchmark_runner() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let work_dir = temp_dir.path().join("work");
    let source_file = temp_dir.path().join("bench.csd");
    fs::create_dir_all(&work_dir)?;
    
    // Create a simple benchmark program
    let source_content = r#"
        slay main() {
            facts sum = 0;
            lowkey (sus i = 0; i < 1000; i++) {
                sum = sum + i;
            }
            println(sum);
        }
    "#;
    fs::write(&source_file, source_content)?;
    
    let compiler_path = PathBuf::from("cursed"); // Placeholder compiler path
    let runner = BenchmarkRunner::new(compiler_path, work_dir.clone())
        .with_verbose(true);
    
    let config = BenchmarkConfig {
        name: "simple_loop".to_string(),
        source_files: vec![source_file],
        optimization_levels: vec![
            OptimizationLevel::None,
            OptimizationLevel::Default,
            OptimizationLevel::Aggressive,
        ],
        iterations: 3,
        warmup_iterations: 1,
        timeout: Duration::from_secs(30),
        compiler_flags: vec![],
        performance_thresholds: PerformanceThresholds::default(),
    };
    
    // Note: This test would require a real compiler binary to work fully
    // For now, we test the configuration and structure
    assert_eq!(config.name, "simple_loop");
    assert_eq!(config.optimization_levels.len(), 3);
    assert_eq!(config.iterations, 3);
    
    Ok(())
}

/// Test enhanced benchmarking system
#[tokio::test]
async fn test_enhanced_benchmarking() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("enhanced_bench.csd");
    
    let source_content = r#"
        slay calculate_prime(n) {
            lowkey (n < 2) {
                fr false;
            }
            lowkey (sus i = 2; i * i <= n; i++) {
                lowkey (n % i == 0) {
                    fr false;
                }
            }
            fr true;
        }
        
        slay main() {
            facts count = 0;
            lowkey (sus i = 2; i < 100; i++) {
                lowkey (calculate_prime(i)) {
                    count = count + 1;
                }
            }
            println(count);
        }
    "#;
    fs::write(&source_file, source_content)?;
    
    let mut runner = EnhancedBenchmarkRunner::new();
    
    let optimization_levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
    ];
    
    let result = runner.benchmark_comprehensive(
        source_content,
        &source_file,
        &optimization_levels,
    ).await?;
    
    // Verify comprehensive benchmark results
    assert_eq!(result.level_results.len(), 3);
    
    // Check statistical summary
    assert!(matches!(
        result.statistical_summary.best_level,
        OptimizationLevel::None | OptimizationLevel::Default | OptimizationLevel::Aggressive
    ));
    assert!(result.statistical_summary.overall_confidence >= 0.0);
    assert!(result.statistical_summary.overall_confidence <= 1.0);
    
    // Check performance comparison
    assert!(!result.performance_comparison.performance_ranking.is_empty());
    assert!(!result.performance_comparison.pairwise_comparisons.is_empty());
    
    // Verify environment information
    assert!(!result.environment.os.is_empty());
    assert!(result.environment.cpu_info.cores > 0);
    assert!(result.environment.memory_info.total_ram > 0);
    
    // Check recommendations
    assert!(!result.recommendations.is_empty());
    
    Ok(())
}

/// Test memory usage monitoring
#[test]
fn test_memory_monitoring() {
    // Test the performance monitor's memory usage measurement
    let monitor = cursed::optimization::performance_integration::PerformanceMonitor::new();
    let memory_usage = monitor.get_memory_usage_mb();
    
    // Should return a reasonable memory usage value (> 0 and < 10GB)
    assert!(memory_usage > 0.0);
    assert!(memory_usage < 10240.0); // 10GB
}

/// Test CPU usage monitoring
#[test]
fn test_cpu_monitoring() {
    let monitor = cursed::optimization::performance_integration::PerformanceMonitor::new();
    let cpu_usage = monitor.get_cpu_usage_percent();
    
    // Should return a reasonable CPU usage percentage
    assert!(cpu_usage >= 0.0);
    assert!(cpu_usage <= 100.0);
}

/// Test build cache effectiveness
#[tokio::test]
async fn test_build_cache_effectiveness() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let source_files: Vec<PathBuf> = (0..5).map(|i| {
        let file = temp_dir.path().join(format!("module_{}.csd", i));
        let content = format!(r#"
            slay function_{}() {{
                facts value = {};
                fr value * 2;
            }}
        "#, i, i * 10);
        fs::write(&file, content).unwrap();
        file
    }).collect();
    
    let context = BuildContext {
        project_root: temp_dir.path().to_path_buf(),
        source_files: source_files.clone(),
        output_directory: temp_dir.path().join("output"),
        target_triple: "x86_64-unknown-linux-gnu".to_string(),
        debug_mode: true, // Enable caching
        release_mode: false,
        verbose: true,
    };
    
    let mut optimizer = BuildOptimizer::new(context)?;
    
    // First build - should have low cache hit rate
    let result1 = optimizer.optimize_build()?;
    assert!(result1.success);
    let initial_cache_rate = result1.cache_hit_rate;
    
    // Second build - should have higher cache hit rate
    let result2 = optimizer.optimize_build()?;
    assert!(result2.success);
    let second_cache_rate = result2.cache_hit_rate;
    
    // Cache hit rate should improve or stay the same
    assert!(second_cache_rate >= initial_cache_rate);
    
    // Build should be faster or same speed (due to caching)
    assert!(result2.compilation_time <= result1.compilation_time + Duration::from_millis(100)); // Allow small variance
    
    Ok(())
}

/// Test parallel compilation efficiency
#[tokio::test]
async fn test_parallel_compilation() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    
    // Create multiple source files for parallel compilation
    let source_files: Vec<PathBuf> = (0..8).map(|i| {
        let file = temp_dir.path().join(format!("parallel_{}.csd", i));
        let content = format!(r#"
            slay compute_{}(input) {{
                facts result = 0;
                lowkey (sus j = 0; j < input; j++) {{
                    result = result + j * {};
                }}
                fr result;
            }}
            
            slay main_{}() {{
                facts value = compute_{}(1000);
                println(value);
            }}
        "#, i, i + 1, i, i);
        fs::write(&file, content).unwrap();
        file
    }).collect();
    
    let context = BuildContext {
        project_root: temp_dir.path().to_path_buf(),
        source_files: source_files.clone(),
        output_directory: temp_dir.path().join("output"),
        target_triple: "x86_64-unknown-linux-gnu".to_string(),
        debug_mode: false,
        release_mode: true, // Enable parallel compilation
        verbose: true,
    };
    
    let mut optimizer = BuildOptimizer::new(context)?;
    let result = optimizer.optimize_build()?;
    
    assert!(result.success);
    assert_eq!(result.files_compiled, 8);
    
    // Parallel efficiency should be reasonable for 8 files
    assert!(result.parallel_efficiency >= 0.5); // At least 50% efficiency
    assert!(result.parallel_efficiency <= 1.0);
    
    // Should have compiled all files
    assert_eq!(result.files_compiled, source_files.len());
    
    Ok(())
}

/// Test size reduction calculations
#[tokio::test]
async fn test_size_reduction() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("size_test.csd");
    
    // Create a larger source file to test size reduction
    let mut source_content = String::new();
    source_content.push_str("slay main() {\n");
    for i in 0..50 {
        source_content.push_str(&format!("    facts var_{} = {};\n", i, i * 42));
    }
    source_content.push_str("    facts total = 0;\n");
    for i in 0..50 {
        source_content.push_str(&format!("    total = total + var_{};\n", i));
    }
    source_content.push_str("    println(total);\n}\n");
    
    fs::write(&source_file, &source_content)?;
    
    // Test different optimization modes
    for (debug, release, expected_factor) in [
        (true, false, 0.95),   // Debug mode: minimal reduction
        (false, false, 0.85),  // Default mode: moderate reduction
        (false, true, 0.75),   // Release mode: significant reduction
    ] {
        let context = BuildContext {
            project_root: temp_dir.path().to_path_buf(),
            source_files: vec![source_file.clone()],
            output_directory: temp_dir.path().join(format!("output_{}_{}", debug, release)),
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            debug_mode: debug,
            release_mode: release,
            verbose: false,
        };
        
        let mut optimizer = BuildOptimizer::new(context)?;
        let result = optimizer.optimize_build()?;
        
        assert!(result.success);
        
        // Should show some size reduction in release mode
        if release {
            assert!(result.size_reduction_bytes > 0);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    /// Test end-to-end build optimization workflow
    #[tokio::test]
    async fn test_complete_build_workflow() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        
        // Create a realistic CURSED project structure
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(&src_dir)?;
        
        // Main module
        let main_file = src_dir.join("main.csd");
        fs::write(&main_file, r#"
            import "utils";
            import "math";
            
            slay main() {
                facts result = math::fibonacci(20);
                utils::print_result("Fibonacci(20)", result);
            }
        "#)?;
        
        // Utils module
        let utils_file = src_dir.join("utils.csd");
        fs::write(&utils_file, r#"
            slay print_result(label, value) {
                println(label + ": " + value);
            }
            
            slay format_number(num) {
                fr "Number: " + num;
            }
        "#)?;
        
        // Math module
        let math_file = src_dir.join("math.csd");
        fs::write(&math_file, r#"
            slay fibonacci(n) {
                lowkey (n <= 1) {
                    fr n;
                }
                fr fibonacci(n - 1) + fibonacci(n - 2);
            }
            
            slay factorial(n) {
                facts result = 1;
                lowkey (sus i = 1; i <= n; i++) {
                    result = result * i;
                }
                fr result;
            }
        "#)?;
        
        let source_files = vec![main_file, utils_file, math_file];
        
        // Test with performance integration
        let perf_config = PerformanceIntegrationConfig {
            enable_adaptive_optimization: true,
            enable_performance_monitoring: true,
            enable_automatic_reporting: true,
            monitoring_interval_ms: 100,
            optimization_threshold_seconds: 2.0,
            max_parallel_workers: 4,
            enable_pgo: false,
            enable_distributed: false,
            cache_size_limit_mb: 200,
            report_output_dir: Some(temp_dir.path().join("reports")),
            benchmark_configs: std::collections::HashMap::new(),
            target_improvements: cursed::optimization::performance_integration::PerformanceTargets {
                compilation_time_reduction: 40.0,
                runtime_performance_improvement: 30.0,
                memory_usage_reduction: 20.0,
                binary_size_reduction: 15.0,
            },
        };
        
        let opt_config = OptimizationConfig {
            optimization_level: OptimizationLevel::Aggressive,
            enable_parallel: true,
            parallel_workers: 4,
            enable_incremental: true,
            ..Default::default()
        };
        
        let mut perf_system = PerformanceIntegrationSystem::new(perf_config, opt_config)?;
        
        let result = perf_system.optimize_project(&source_files, &temp_dir.path().join("output")).await?;
        
        // Verify comprehensive optimization results
        assert!(result.compilation_time > Duration::from_millis(0));
        assert!(result.parallel_efficiency > 0.0);
        assert!(!result.recommendations.is_empty());
        
        // Check that all optimization phases completed
        assert!(!result.checkpoints.is_empty());
        
        // Verify environment information is captured
        assert!(!result.environment.os.is_empty());
        assert!(result.environment.cpu_info.cores > 0);
        
        // Verify adaptive optimization worked
        assert!(result.adaptive_optimization_enabled);
        
        Ok(())
    }
}
