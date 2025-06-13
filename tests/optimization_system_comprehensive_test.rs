/// Comprehensive tests for the CURSED optimization system
/// 
/// Tests incremental compilation, benchmarking, adaptive optimization,
/// and integration between different optimization components.

use cursed::optimization::*;
use cursed::error::*;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn test_optimization_config_creation() {
    let config = OptimizationConfig::default();
    
    assert!(config.enable_advanced_llvm);
    assert!(config.enable_parallel_compilation);
    assert!(config.enable_incremental_compilation);
    assert!(config.enable_jit_optimization);
    assert!(config.enable_memory_optimization);
    assert!(config.enable_caching);
    assert_eq!(config.optimization_level, 2);
    assert!(config.max_parallel_threads > 0);
}

#[test]
fn test_optimization_manager_creation() {
    let config = OptimizationConfig::default();
    let manager = OptimizationManager::new(config);
    
    assert!(manager.is_ok());
    let manager = manager.unwrap();
    
    assert!(manager.llvm_optimizer().is_some());
    assert!(manager.speed_optimizer().is_some());
    assert!(manager.jit_optimizer().is_some());
    assert!(manager.memory_optimizer().is_some());
    assert!(manager.parallel_compiler().is_some());
    assert!(manager.incremental_compiler().is_some());
    assert!(manager.cache_manager().is_some());
}

#[test]
fn test_incremental_compilation_basic() {
    let temp_dir = TempDir::new().unwrap();
    let config = IncrementalConfig {
        cache_dir: temp_dir.path().join(".cache"),
        enable_dependency_tracking: true,
        enable_fine_grained_detection: true,
        parallel_incremental: false, // Keep simple for test
        ..Default::default()
    };
    
    let mut detector = ChangeDetector::new(config).unwrap();
    
    // Create a test file
    let test_file = temp_dir.path().join("test.csd");
    std::fs::write(&test_file, "fn main() { println(\"hello\"); }").unwrap();
    
    detector.analyze_file(&test_file).unwrap();
    let changed = detector.get_changed_files();
    
    // New file should be detected as changed
    assert!(changed.contains(&test_file));
    
    // Save metadata
    detector.save_metadata_cache().unwrap();
    
    // Create new detector and load cache
    let mut detector2 = ChangeDetector::new(detector.config.clone()).unwrap();
    detector2.analyze_file(&test_file).unwrap();
    
    // File should not be changed now (same content)
    let changed2 = detector2.get_changed_files();
    assert!(!changed2.contains(&test_file));
}

#[test]
fn test_dependency_graph_functionality() {
    let mut graph = DependencyGraph::default();
    
    let file_a = PathBuf::from("a.csd");
    let file_b = PathBuf::from("b.csd");
    let file_c = PathBuf::from("c.csd");
    
    // a depends on b, b depends on c
    graph.add_dependency(file_a.clone(), file_b.clone());
    graph.add_dependency(file_b.clone(), file_c.clone());
    
    // Check dependencies
    let deps_a = graph.get_dependencies(&file_a);
    assert!(deps_a.contains(&file_b));
    
    let deps_b = graph.get_dependencies(&file_b);
    assert!(deps_b.contains(&file_c));
    
    // Check dependents (reverse)
    let dependents_c = graph.get_dependents(&file_c);
    assert!(dependents_c.contains(&file_b));
    
    let dependents_b = graph.get_dependents(&file_b);
    assert!(dependents_b.contains(&file_a));
    
    // Topological sort
    let order = graph.topological_sort().unwrap();
    
    // c should come before b, b should come before a
    let pos_c = order.iter().position(|f| f == &file_c).unwrap();
    let pos_b = order.iter().position(|f| f == &file_b).unwrap();
    let pos_a = order.iter().position(|f| f == &file_a).unwrap();
    
    assert!(pos_c < pos_b);
    assert!(pos_b < pos_a);
}

#[test]
fn test_compilation_cache_functionality() {
    let temp_dir = TempDir::new().unwrap();
    let config = IncrementalConfig {
        cache_dir: temp_dir.path().to_path_buf(),
        max_cache_size: 1024 * 1024, // 1MB
        ..Default::default()
    };
    
    let cache = CompilationCache::new(config).unwrap();
    
    // Create a fake artifact file
    let artifact_path = temp_dir.path().join("output.o");
    std::fs::write(&artifact_path, b"compiled code").unwrap();
    
    // Store in cache
    cache.store(
        "hash123".to_string(),
        "deps456".to_string(),
        artifact_path.clone(),
        Duration::from_millis(100),
    ).unwrap();
    
    // Retrieve from cache
    let entry = cache.get("hash123", "deps456");
    assert!(entry.is_some());
    
    let entry = entry.unwrap();
    assert_eq!(entry.artifact_path, artifact_path);
    assert_eq!(entry.compilation_time, Duration::from_millis(100));
    
    // Cache miss with different dependencies
    let miss = cache.get("hash123", "deps789");
    assert!(miss.is_none());
    
    // Test cache statistics
    let stats = cache.stats();
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 1);
    assert!(stats.total_size > 0);
}

#[test]
fn test_incremental_compiler_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let opt_config = OptimizationConfig::default();
    
    let mut compiler = IncrementalCompiler::new(&opt_config).unwrap();
    
    // Create test files
    let main_file = temp_dir.path().join("main.csd");
    let util_file = temp_dir.path().join("util.csd");
    
    std::fs::write(&main_file, r#"
        import "util.csd"
        fn main() { 
            println(helper_function());
        }
    "#).unwrap();
    
    std::fs::write(&util_file, r#"
        fn helper_function() -> String {
            "Hello from util"
        }
    "#).unwrap();
    
    // Mock compilation function
    let compile_fn = |path: &std::path::Path| -> Result<(PathBuf, Duration)> {
        let output_path = path.with_extension("o");
        std::fs::write(&output_path, b"compiled object").unwrap();
        Ok((output_path, Duration::from_millis(50)))
    };
    
    // First compilation (everything should be compiled)
    let result = compiler.compile_directory(temp_dir.path(), compile_fn).unwrap();
    assert!(result.compiled_files.len() >= 2);
    assert_eq!(result.cache_hits, 0); // First time, no cache hits
    
    // Second compilation (should use cache)
    let result2 = compiler.compile_directory(temp_dir.path(), compile_fn).unwrap();
    assert!(result2.cache_hits > 0); // Should have cache hits now
    
    // Modify one file and recompile
    std::fs::write(&util_file, r#"
        fn helper_function() -> String {
            "Modified helper function"
        }
    "#).unwrap();
    
    let result3 = compiler.compile_directory(temp_dir.path(), compile_fn).unwrap();
    assert!(result3.files_changed > 0);
    assert!(result3.files_affected > 0);
}

#[test]
fn test_benchmark_suite_creation() {
    let temp_dir = TempDir::new().unwrap();
    let config = BenchmarkConfig {
        output_dir: temp_dir.path().to_path_buf(),
        iterations: 3,
        warmup_iterations: 1,
        max_time: Duration::from_secs(5),
        ..Default::default()
    };
    
    let suite = BenchmarkSuite::new(config);
    assert!(suite.is_ok());
}

#[test]
fn test_benchmark_statistics() {
    let measurements = vec![
        BenchmarkMeasurement {
            name: "test".to_string(),
            duration: Duration::from_millis(100),
            memory_usage: Some(1024),
            cpu_usage: Some(50.0),
            custom_metrics: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now(),
        },
        BenchmarkMeasurement {
            name: "test".to_string(),
            duration: Duration::from_millis(110),
            memory_usage: Some(1024),
            cpu_usage: Some(55.0),
            custom_metrics: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now(),
        },
        BenchmarkMeasurement {
            name: "test".to_string(),
            duration: Duration::from_millis(90),
            memory_usage: Some(1024),
            cpu_usage: Some(45.0),
            custom_metrics: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now(),
        },
    ];
    
    let stats = BenchmarkStatistics::from_measurements("test".to_string(), &measurements);
    
    assert_eq!(stats.count, 3);
    assert_eq!(stats.mean, Duration::from_millis(100));
    assert_eq!(stats.median, Duration::from_millis(100));
    assert_eq!(stats.min, Duration::from_millis(90));
    assert_eq!(stats.max, Duration::from_millis(110));
    assert!(stats.coefficient_of_variation > 0.0);
    assert!(stats.is_stable(0.2)); // Should be stable with low variance
}

#[test]
fn test_performance_comparison() {
    let stats1 = BenchmarkStatistics {
        name: "test".to_string(),
        count: 10,
        mean: Duration::from_millis(100),
        median: Duration::from_millis(100),
        std_dev: Duration::from_millis(5),
        min: Duration::from_millis(90),
        max: Duration::from_millis(110),
        coefficient_of_variation: 0.05,
        p95: Duration::from_millis(108),
        p99: Duration::from_millis(110),
        confidence_interval: (Duration::from_millis(95), Duration::from_millis(105)),
        outliers: vec![],
    };
    
    let stats2 = BenchmarkStatistics {
        name: "test".to_string(),
        count: 10,
        mean: Duration::from_millis(120),
        median: Duration::from_millis(120),
        std_dev: Duration::from_millis(5),
        min: Duration::from_millis(110),
        max: Duration::from_millis(130),
        coefficient_of_variation: 0.04,
        p95: Duration::from_millis(128),
        p99: Duration::from_millis(130),
        confidence_interval: (Duration::from_millis(115), Duration::from_millis(125)),
        outliers: vec![],
    };
    
    let comparison = stats2.compare(&stats1);
    match comparison {
        PerformanceComparison::Regression(percent) => {
            assert!(percent > 15.0 && percent < 25.0);
        },
        _ => panic!("Expected regression"),
    }
}

#[test]
fn test_built_in_benchmarks() {
    let temp_dir = TempDir::new().unwrap();
    let opt_config = OptimizationConfig::default();
    let benchmarks = OptimizationBenchmarks::new(opt_config);
    
    let benchmark_config = BenchmarkConfig {
        output_dir: temp_dir.path().to_path_buf(),
        iterations: 2,
        warmup_iterations: 1,
        max_time: Duration::from_secs(5),
        ..Default::default()
    };
    
    let suite = benchmarks.create_suite(benchmark_config).unwrap();
    let results = suite.run_all().unwrap();
    
    // Should have several built-in benchmarks
    assert!(results.results.contains_key("compilation_speed"));
    assert!(results.results.contains_key("optimization_passes"));
    assert!(results.results.contains_key("memory_usage"));
    assert!(results.results.contains_key("code_quality"));
    
    // Check that results are reasonable
    for (name, stats) in &results.results {
        assert!(stats.count >= 2, "Benchmark {} should have at least 2 measurements", name);
        assert!(stats.mean > Duration::default(), "Benchmark {} should have non-zero mean", name);
    }
}

#[test]
fn test_adaptive_optimizer_basic() {
    let opt_config = OptimizationConfig::default();
    let optimizer = AdaptiveOptimizer::new(&opt_config).unwrap();
    
    // Record some execution feedback
    let feedback = OptimizationFeedback {
        name: "test_function".to_string(),
        execution_time: Duration::from_millis(100),
        memory_usage: 1024,
        success: true,
        error: None,
        timestamp: std::time::SystemTime::now(),
    };
    
    optimizer.record_execution(feedback).unwrap();
    
    let summary = optimizer.get_summary();
    assert_eq!(summary.total_functions, 1);
    assert_eq!(summary.total_executions, 1);
}

#[test]
fn test_execution_profile_updates() {
    let mut profile = ExecutionProfile::new("test_function".to_string());
    
    // Update with execution data
    profile.update(Duration::from_millis(100), 1024, 10);
    profile.update(Duration::from_millis(110), 1024, 10);
    profile.update(Duration::from_millis(90), 1024, 10);
    
    assert_eq!(profile.execution_count, 3);
    assert_eq!(profile.performance_history.len(), 3);
    assert!(profile.average_time.as_millis() > 95);
    assert!(profile.average_time.as_millis() < 105);
    assert!(profile.hotness_score >= 0.0);
}

#[test]
fn test_performance_trend_detection() {
    let mut profile = ExecutionProfile::new("test_function".to_string());
    
    // Add improving trend data (decreasing execution times)
    for i in 0..20 {
        let time = Duration::from_millis(200 - i * 5); // Getting faster
        profile.update(time, 1024, 100);
    }
    
    let trend = profile.get_performance_trend();
    assert_eq!(trend, PerformanceTrend::Improving);
    
    // Test degrading trend
    let mut profile2 = ExecutionProfile::new("degrading_function".to_string());
    for i in 0..20 {
        let time = Duration::from_millis(100 + i * 5); // Getting slower
        profile2.update(time, 1024, 100);
    }
    
    let trend2 = profile2.get_performance_trend();
    assert_eq!(trend2, PerformanceTrend::Degrading);
}

#[test]
fn test_optimization_recommendations() {
    let config = AdaptiveConfig {
        min_execution_count: 5,
        confidence_threshold: 0.5,
        ..Default::default()
    };
    
    let optimizer = LearningOptimizer::new(config);
    
    // Record feedback for a function to make it hot
    for i in 0..20 {
        let feedback = OptimizationFeedback {
            name: "hot_function".to_string(),
            execution_time: Duration::from_millis(100),
            memory_usage: 1024,
            success: true,
            error: None,
            timestamp: std::time::SystemTime::now(),
        };
        
        optimizer.record_feedback(feedback).unwrap();
    }
    
    let recommendations = optimizer.get_recommendations().unwrap();
    
    // Should generate recommendations for hot function
    assert!(!recommendations.is_empty());
    
    let hot_rec = recommendations.iter()
        .find(|r| r.function_name == "hot_function");
    assert!(hot_rec.is_some());
    
    let rec = hot_rec.unwrap();
    assert!(rec.priority > 0.0);
    assert!(rec.confidence > 0.0);
}

#[test]
fn test_success_rate_tracking() {
    let mut success_rate = SuccessRate::new();
    
    // Record successful optimizations
    success_rate.update(true, 0.1);   // 10% improvement
    success_rate.update(true, 0.2);   // 20% improvement
    success_rate.update(false, -0.05); // 5% regression
    success_rate.update(true, 0.15);  // 15% improvement
    
    assert_eq!(success_rate.total_applications, 4);
    assert_eq!(success_rate.successful_applications, 3);
    assert_eq!(success_rate.success_rate(), 0.75);
    assert!(success_rate.average_improvement > 0.0);
    assert!(success_rate.confidence > 0.0);
}

#[test]
fn test_optimization_integration() {
    // Test that all optimization components work together
    let temp_dir = TempDir::new().unwrap();
    
    let config = OptimizationConfig {
        enable_advanced_llvm: true,
        enable_parallel_compilation: true,
        enable_incremental_compilation: true,
        enable_jit_optimization: true,
        enable_memory_optimization: true,
        enable_profiling: true,
        enable_caching: true,
        enable_adaptive_optimization: true,
        optimization_level: 2,
        ..Default::default()
    };
    
    // Create optimization manager
    let manager = OptimizationManager::new(config).unwrap();
    
    // Verify all components are initialized
    assert!(manager.llvm_optimizer().is_some());
    assert!(manager.speed_optimizer().is_some());
    assert!(manager.jit_optimizer().is_some());
    assert!(manager.memory_optimizer().is_some());
    assert!(manager.parallel_compiler().is_some());
    assert!(manager.incremental_compiler().is_some());
    assert!(manager.cache_manager().is_some());
    assert!(manager.adaptive_optimizer().is_some());
    assert!(manager.profiler().is_some());
    
    // Test configuration changes
    let mut new_config = manager.config().clone();
    new_config.optimization_level = 3;
    manager.update_config(new_config).unwrap();
    
    assert_eq!(manager.config().optimization_level, 3);
}

#[test]
fn test_optimization_manager_summary() {
    let config = OptimizationConfig::default();
    let manager = OptimizationManager::new(config).unwrap();
    
    // Should not panic and should produce some output
    manager.print_comprehensive_summary();
}

#[test]
fn test_circular_dependency_detection() {
    let mut graph = DependencyGraph::default();
    
    let file_a = PathBuf::from("a.csd");
    let file_b = PathBuf::from("b.csd");
    let file_c = PathBuf::from("c.csd");
    
    // Create circular dependency: a -> b -> c -> a
    graph.add_dependency(file_a.clone(), file_b.clone());
    graph.add_dependency(file_b.clone(), file_c.clone());
    graph.add_dependency(file_c.clone(), file_a.clone()); // Creates cycle
    
    // Topological sort should fail
    let result = graph.topological_sort();
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    match error {
        Error::Parse(msg) => {
            assert!(msg.contains("Circular dependency"));
        },
        _ => panic!("Expected Parse error for circular dependency"),
    }
}

#[test]
fn test_regression_detection() {
    let temp_dir = TempDir::new().unwrap();
    let benchmark_config = BenchmarkConfig {
        output_dir: temp_dir.path().to_path_buf(),
        iterations: 3,
        warmup_iterations: 1,
        ..Default::default()
    };
    
    let mut suite = BenchmarkSuite::new(benchmark_config).unwrap();
    
    // Add a simple test benchmark
    struct SimpleBenchmark {
        duration: Duration,
    }
    
    impl Benchmark for SimpleBenchmark {
        fn name(&self) -> &str { "simple_test" }
        fn description(&self) -> &str { "Simple test benchmark" }
        fn run_once(&self) -> Result<BenchmarkMeasurement> {
            Ok(BenchmarkMeasurement {
                name: self.name().to_string(),
                duration: self.duration,
                memory_usage: Some(1024),
                cpu_usage: Some(50.0),
                custom_metrics: std::collections::HashMap::new(),
                timestamp: chrono::Utc::now(),
            })
        }
    }
    
    // Register benchmark with good performance
    suite.register_benchmark(Box::new(SimpleBenchmark { 
        duration: Duration::from_millis(100) 
    }));
    
    let results1 = suite.run_all().unwrap();
    
    // Simulate regression by increasing duration
    suite.register_benchmark(Box::new(SimpleBenchmark { 
        duration: Duration::from_millis(150) // 50% slower
    }));
    
    let results2 = suite.run_all().unwrap();
    
    // Check for regressions
    let regressions = suite.detect_regressions(&results2).unwrap();
    
    // Should detect regression
    assert!(!regressions.is_empty());
    let regression = &regressions[0];
    assert_eq!(regression.benchmark_name, "simple_test");
    
    match &regression.comparison {
        PerformanceComparison::Regression(percent) => {
            assert!(*percent > 40.0); // Should detect significant regression
        },
        _ => panic!("Expected regression detection"),
    }
}

#[test]
fn test_memory_profile_updates() {
    let mut profile = MemoryProfile::default();
    
    profile.update(1024);
    profile.update(2048);
    profile.update(512);
    
    assert_eq!(profile.allocation_count, 3);
    assert_eq!(profile.total_allocated, 1024 + 2048 + 512);
    assert_eq!(profile.peak_usage, 2048);
    assert_eq!(profile.average_usage, (1024 + 2048 + 512) / 3);
}
