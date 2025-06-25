/// Comprehensive Integration Tests for CURSED Optimization System
/// 
/// Tests the complete optimization pipeline including parallel compilation,
/// incremental builds, caching, profiling, and analysis.

use cursed::optimization::*;
use cursed::optimization::config::*;
use cursed::optimization::profiler::*;
use cursed::optimization::analysis::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn test_optimization_system_creation() {
    let temp_dir = TempDir::new().unwrap();
    let config = OptimizationConfig {
        cache_directory: Some(temp_dir.path().to_path_buf()),
        parallel_workers: 2,
        enable_profiling: true,
        ..Default::default()
    };
    
    let system = OptimizationSystem::new(config).unwrap();
    assert_eq!(system.config().parallel_workers, 2);
    assert!(system.config().enable_profiling);
}

#[test]
fn test_optimization_configuration_from_args() {
    let args = OptimizationArgs {
        profile: Some(OptimizationProfile::Performance),
        optimization_level: Some("O3".to_string()),
        parallel_workers: Some(8),
        enable_profiling: Some(true),
        verbose: Some(true),
        target_cpu: Some("native".to_string()),
        target_features: vec!["sse4.2".to_string(), "avx".to_string()],
        ..Default::default()
    };
    
    let config = OptimizationConfig::from_args(&args).unwrap();
    assert_eq!(config.optimization_level, OptimizationLevel::Aggressive);
    assert_eq!(config.parallel_workers, 8);
    assert!(config.enable_profiling);
    assert!(config.verbose_optimization);
    assert_eq!(config.target_cpu, Some("native".to_string()));
    assert_eq!(config.target_features, vec!["sse4.2", "avx"]);
}

#[test]
fn test_optimization_profiles() {
    // Test development profile
    let dev_config = OptimizationProfile::Development.to_config();
    assert_eq!(dev_config.optimization_level, OptimizationLevel::Less);
    assert!(dev_config.debug_mode);
    assert!(!dev_config.llvm_passes.enable_vectorization);
    
    // Test release profile
    let release_config = OptimizationProfile::Release.to_config();
    assert_eq!(release_config.optimization_level, OptimizationLevel::Aggressive);
    assert!(!release_config.debug_mode);
    assert!(release_config.llvm_passes.enable_link_time_optimization);
    
    // Test performance profile
    let perf_config = OptimizationProfile::Performance.to_config();
    assert_eq!(perf_config.optimization_level, OptimizationLevel::Aggressive);
    assert!(perf_config.profile_guided);
    assert!(perf_config.enable_profiling);
}

#[test]
fn test_parallel_compiler_basic_functionality() {
    let compiler = ParallelCompiler::new(2);
    assert_eq!(compiler.worker_count, 2);
    assert_eq!(compiler.active_workers(), 0);
    
    let stats = compiler.get_stats();
    assert_eq!(stats.jobs_queued, 0);
    assert_eq!(stats.jobs_completed, 0);
}

#[test]
fn test_incremental_builder_creation() {
    let temp_dir = TempDir::new().unwrap();
    let builder = IncrementalBuilder::new(temp_dir.path()).unwrap();
    
    assert!(builder.cache_directory.exists());
    
    let stats = builder.get_cache_stats();
    assert_eq!(stats["total_files"], 0);
}

#[test]
fn test_compilation_cache_operations() {
    let temp_dir = TempDir::new().unwrap();
    let mut cache = CompilationCache::new(temp_dir.path()).unwrap();
    
    // Create test source file
    let source_file = temp_dir.path().join("test.csd");
    std::fs::write(&source_file, "facts x = 42;").unwrap();
    
    // Generate cache key
    let key = cache.generate_key(
        &source_file,
        &[],
        &["--optimize".to_string()],
        CacheType::CompiledObject,
    ).unwrap();
    
    // Test storage and retrieval
    let test_data = b"compiled object data";
    cache.store(
        &key,
        test_data,
        &source_file,
        &[],
        &["--optimize".to_string()],
        CacheType::CompiledObject,
    ).unwrap();
    
    assert!(cache.contains(&key));
    
    let retrieved = cache.retrieve(&key).unwrap();
    assert_eq!(retrieved.unwrap(), test_data);
    
    // Test statistics
    let stats = cache.get_stats();
    assert_eq!(stats["entry_count"], 1);
    assert!(stats["hits"] >= 1);
}

#[test]
fn test_performance_profiler_sessions() {
    let profiler = PerformanceProfiler::new();
    
    profiler.start_session("test_session");
    
    // Test timer operations
    profiler.start_timer("test_session", "operation1");
    std::thread::sleep(Duration::from_millis(10));
    profiler.end_timer("test_session", "operation1", ProfileCategory::Parsing);
    
    profiler.start_timer("test_session", "operation2");
    std::thread::sleep(Duration::from_millis(20));
    profiler.end_timer("test_session", "operation2", ProfileCategory::CodeGeneration);
    
    // Test results
    let stats = profiler.get_session_stats("test_session").unwrap();
    assert_eq!(stats.len(), 2);
    assert_eq!(stats[0].name, "operation1");
    assert_eq!(stats[1].name, "operation2");
    assert_eq!(stats[0].category, ProfileCategory::Parsing);
    assert_eq!(stats[1].category, ProfileCategory::CodeGeneration);
    
    // Test category breakdown
    let breakdown = profiler.get_category_breakdown("test_session");
    assert!(breakdown.contains_key(&ProfileCategory::Parsing));
    assert!(breakdown.contains_key(&ProfileCategory::CodeGeneration));
    
    // Test slowest operations
    let slowest = profiler.get_slowest_operations("test_session", 1);
    assert_eq!(slowest.len(), 1);
    assert_eq!(slowest[0].name, "operation2"); // Should be the slower one
}

#[test]
fn test_scoped_timer() {
    let profiler = PerformanceProfiler::new();
    profiler.start_session("scoped_test");
    
    {
        let _timer = ScopedTimer::new(
            &profiler,
            "scoped_test",
            "scoped_operation",
            ProfileCategory::Optimization,
        );
        std::thread::sleep(Duration::from_millis(5));
    } // Timer automatically ends here
    
    let stats = profiler.get_session_stats("scoped_test").unwrap();
    assert_eq!(stats.len(), 1);
    assert_eq!(stats[0].name, "scoped_operation");
    assert_eq!(stats[0].category, ProfileCategory::Optimization);
    assert!(stats[0].duration >= Duration::from_millis(5));
}

#[test]
fn test_performance_analyzer_basic() {
    let profiler = PerformanceProfiler::new();
    let mut analyzer = PerformanceAnalyzer::new();
    
    profiler.start_session("analysis_test");
    
    // Add some profile points
    profiler.record_point(
        "analysis_test",
        "slow_operation",
        ProfileCategory::Parsing,
        Duration::from_millis(1000),
        HashMap::new(),
    );
    
    profiler.record_point(
        "analysis_test",
        "fast_operation",
        ProfileCategory::TypeChecking,
        Duration::from_millis(100),
        HashMap::new(),
    );
    
    // Analyze performance
    let report = analyzer.analyze(&profiler, "analysis_test").unwrap();
    
    // Verify summary
    assert_eq!(report.summary.files_processed, 2);
    assert!(report.summary.total_compilation_time >= Duration::from_millis(1100));
    
    // Verify bottlenecks were identified
    assert!(!report.bottlenecks.is_empty());
    let parsing_bottleneck = report.bottlenecks.iter()
        .find(|b| b.category == ProfileCategory::Parsing);
    assert!(parsing_bottleneck.is_some());
    
    // Verify recommendations were generated
    assert!(!report.recommendations.is_empty());
}

#[test]
fn test_optimization_session_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    let config = OptimizationConfig {
        cache_directory: Some(temp_dir.path().to_path_buf()),
        enable_profiling: true,
        ..Default::default()
    };
    
    let system = Arc::new(OptimizationSystem::new(config).unwrap());
    
    {
        let session = OptimizationSession::new(system.clone(), "lifecycle_test".to_string());
        assert_eq!(session.id(), "lifecycle_test");
        assert!(session.duration().as_nanos() > 0);
        
        // Simulate some work
        std::thread::sleep(Duration::from_millis(10));
        
    } // Session automatically ends here
    
    // Verify session was properly closed
    // Note: In a real implementation, we'd verify the profiler session was ended
}

#[test]
fn test_optimization_result_operations() {
    let mut result = OptimizationResult::success();
    
    // Test setting various metrics
    result.set_timing(Duration::from_millis(500));
    result.set_memory_usage(1024 * 1024); // 1MB
    result.set_cache_stats(80, 20);
    result.set_processing_stats(10, 25);
    result.set_improvements(15.5, 22.3);
    
    // Verify all values were set correctly
    assert!(result.success);
    assert_eq!(result.duration, Duration::from_millis(500));
    assert_eq!(result.memory_used, 1024 * 1024);
    assert_eq!(result.cache_hits, 80);
    assert_eq!(result.cache_misses, 20);
    assert_eq!(result.files_processed, 10);
    assert_eq!(result.optimizations_applied, 25);
    assert_eq!(result.code_size_reduction, 15.5);
    assert_eq!(result.compilation_speed_improvement, 22.3);
    
    // Test adding errors and warnings
    result.add_error("Test error".to_string());
    result.add_warning("Test warning".to_string());
    
    assert!(!result.success); // Should become false when error is added
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.warnings.len(), 1);
}

#[test]
fn test_cache_eviction_policies() {
    let temp_dir = TempDir::new().unwrap();
    let config = CacheConfig {
        max_entries: 2, // Very small to trigger eviction
        eviction_strategy: EvictionStrategy::LeastRecentlyUsed,
        ..Default::default()
    };
    
    let mut cache = CompilationCache::with_config(temp_dir.path(), config).unwrap();
    
    // Create test files
    for i in 0..3 {
        let source_file = temp_dir.path().join(format!("test{}.csd", i));
        std::fs::write(&source_file, format!("facts x = {};", i)).unwrap();
        
        let key = cache.generate_key(
            &source_file,
            &[],
            &[],
            CacheType::CompiledObject,
        ).unwrap();
        
        cache.store(
            &key,
            &format!("data{}", i).as_bytes(),
            &source_file,
            &[],
            &[],
            CacheType::CompiledObject,
        ).unwrap();
    }
    
    // Should have evicted at least one entry
    let stats = cache.get_stats();
    assert!(stats["entry_count"] <= 2);
    assert!(stats["evictions"] >= 1);
}

#[test]
fn test_comprehensive_integration_workflow() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test source files
    let source_files = vec![
        temp_dir.path().join("main.csd"),
        temp_dir.path().join("utils.csd"),
        temp_dir.path().join("types.csd"),
    ];
    
    for (i, file) in source_files.iter().enumerate() {
        std::fs::write(file, format!("facts value_{} = {};", i, i * 10)).unwrap();
    }
    
    // Set up optimization system
    let config = OptimizationConfig {
        cache_directory: Some(temp_dir.path().join("cache")),
        parallel_workers: 2,
        enable_profiling: true,
        enable_incremental: true,
        generate_reports: true,
        verbose_optimization: true,
        ..Default::default()
    };
    
    let system = Arc::new(OptimizationSystem::new(config).unwrap());
    let session = OptimizationSession::new(system.clone(), "integration_test".to_string());
    
    // Simulate compilation workflow
    let profiler = session.system().profiler();
    
    // Phase 1: Parsing
    profiler.start_timer(session.id(), "parsing_phase");
    for (i, file) in source_files.iter().enumerate() {
        profiler.start_timer(session.id(), &format!("parse_file_{}", i));
        std::thread::sleep(Duration::from_millis(20));
        profiler.end_timer(session.id(), &format!("parse_file_{}", i), ProfileCategory::Parsing);
    }
    profiler.end_timer(session.id(), "parsing_phase", ProfileCategory::Parsing);
    
    // Phase 2: Type checking
    profiler.start_timer(session.id(), "typecheck_phase");
    std::thread::sleep(Duration::from_millis(50));
    profiler.end_timer(session.id(), "typecheck_phase", ProfileCategory::TypeChecking);
    
    // Phase 3: Optimization
    profiler.start_timer(session.id(), "optimization_phase");
    std::thread::sleep(Duration::from_millis(80));
    profiler.end_timer(session.id(), "optimization_phase", ProfileCategory::Optimization);
    
    // Phase 4: Code generation
    profiler.start_timer(session.id(), "codegen_phase");
    std::thread::sleep(Duration::from_millis(60));
    profiler.end_timer(session.id(), "codegen_phase", ProfileCategory::CodeGeneration);
    
    // Analyze results
    let mut analyzer = PerformanceAnalyzer::new();
    let report = analyzer.analyze(&profiler, session.id()).unwrap();
    
    // Verify comprehensive analysis
    assert!(report.summary.files_processed > 0);
    assert!(report.summary.total_compilation_time >= Duration::from_millis(200));
    assert!(!report.detailed_metrics.phase_breakdown.is_empty());
    
    // Check that all major phases were captured
    let phases = &report.detailed_metrics.phase_breakdown;
    assert!(phases.contains_key("Parsing"));
    assert!(phases.contains_key("Type Checking"));
    assert!(phases.contains_key("Optimization"));
    assert!(phases.contains_key("Code Generation"));
    
    // Verify cache operations
    let cache_stats = session.system().cache_stats();
    // Note: In this test, cache stats might be 0 since we're not actually storing anything
    
    // Verify incremental builder was initialized
    assert!(session.system().incremental_builder().cache_directory.exists());
    
    // Test report generation
    let report_file = temp_dir.path().join("test_report.md");
    analyzer.generate_report(&profiler, &report_file).unwrap();
    assert!(report_file.exists());
    
    let report_content = std::fs::read_to_string(&report_file).unwrap();
    assert!(report_content.contains("# CURSED Compiler Performance Analysis Report"));
    assert!(report_content.contains("## Performance Bottlenecks"));
    assert!(report_content.contains("## Optimization Recommendations"));
}

#[test]
fn test_error_handling_and_recovery() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test with invalid cache directory
    let config = OptimizationConfig {
        cache_directory: Some(PathBuf::from("/invalid/path/that/does/not/exist")),
        ..Default::default()
    };
    
    // This should fail gracefully
    let result = OptimizationSystem::new(config);
    assert!(result.is_err());
    
    // Test with valid but empty configuration
    let valid_config = OptimizationConfig {
        cache_directory: Some(temp_dir.path().to_path_buf()),
        parallel_workers: 1, // Minimum workers
        ..Default::default()
    };
    
    let system = OptimizationSystem::new(valid_config).unwrap();
    assert_eq!(system.config().effective_workers(), 1);
}

#[test]
fn test_configuration_validation() {
    let mut config = OptimizationConfig::default();
    
    // Valid configuration should pass
    assert!(config.validate().is_ok());
    
    // Invalid parallel workers
    config.parallel_workers = 0;
    assert!(config.validate().is_err());
    
    config.parallel_workers = 4; // Fix it
    
    // Invalid cache size
    config.cache_max_size = 5; // Too small
    assert!(config.validate().is_err());
    
    config.cache_max_size = 100; // Fix it
    
    // Invalid benchmark iterations
    config.benchmark_iterations = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_optimization_level_conversions() {
    // Test string to optimization level conversion
    assert_eq!(OptimizationLevel::from_str("O0").unwrap(), OptimizationLevel::None);
    assert_eq!(OptimizationLevel::from_str("o1").unwrap(), OptimizationLevel::Less);
    assert_eq!(OptimizationLevel::from_str("default").unwrap(), OptimizationLevel::Default);
    assert_eq!(OptimizationLevel::from_str("AGGRESSIVE").unwrap(), OptimizationLevel::Aggressive);
    assert_eq!(OptimizationLevel::from_str("size").unwrap(), OptimizationLevel::Size);
    
    // Test invalid conversion
    assert!(OptimizationLevel::from_str("invalid").is_err());
    
    // Test string representation
    assert_eq!(OptimizationLevel::None.as_str(), "O0");
    assert_eq!(OptimizationLevel::Aggressive.as_str(), "O3");
    assert_eq!(OptimizationLevel::Size.as_str(), "Os");
    
    // Test descriptions
    assert!(OptimizationLevel::None.description().contains("No optimization"));
    assert!(OptimizationLevel::Aggressive.description().contains("Aggressive"));
}

#[test]
fn test_multi_threaded_profiling() {
    use std::thread;
    use std::sync::Arc;
    
    let profiler = Arc::new(PerformanceProfiler::new());
    profiler.start_session("multi_thread_test");
    
    let mut handles = Vec::new();
    
    // Spawn multiple threads that do profiling
    for i in 0..4 {
        let profiler_clone = profiler.clone();
        let handle = thread::spawn(move || {
            profiler_clone.start_timer("multi_thread_test", &format!("thread_operation_{}", i));
            thread::sleep(Duration::from_millis(10));
            profiler_clone.end_timer("multi_thread_test", &format!("thread_operation_{}", i), ProfileCategory::Other);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all operations were recorded
    let stats = profiler.get_session_stats("multi_thread_test").unwrap();
    assert_eq!(stats.len(), 4);
    
    // Verify each thread's operation was recorded
    for i in 0..4 {
        let found = stats.iter().any(|s| s.name == format!("thread_operation_{}", i));
        assert!(found, "Thread {} operation not found", i);
    }
}
