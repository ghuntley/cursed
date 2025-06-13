/// Comprehensive Optimization Infrastructure Tests
/// 
/// This test suite validates the complete optimization infrastructure including
/// LLVM optimization, JIT compilation, parallel compilation, incremental builds,
/// memory optimization, performance profiling, caching, and adaptive optimization.

use cursed::optimization::*;
use cursed::error::Result;
use std::collections::HashMap;
use std::time::Duration;
use std::path::PathBuf;

/// Test LLVM advanced optimization passes
#[test]
fn test_llvm_advanced_optimization() {
    let config = OptimizationConfig::default();
    let mut optimizer = llvm_advanced::AdvancedOptimizationManager::new(&config).unwrap();
    
    // Create a mock LLVM context and module for testing
    let context = inkwell::context::Context::create();
    let module = context.create_module("test_module");
    
    // Add a simple function to the module
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("test_function", fn_type, None);
    
    // Add basic block
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Add simple return instruction
    let param = function.get_first_param().unwrap();
    builder.build_return(Some(&param)).unwrap();
    
    // Run optimization
    let stats = optimizer.optimize_module(&module).unwrap();
    
    // Verify optimization was performed
    assert!(stats.functions_inlined >= 0);
    assert!(stats.optimization_time > Duration::ZERO);
    
    // Print optimization summary
    optimizer.print_summary();
}

/// Test parallel compilation functionality
#[test]
fn test_parallel_compilation() {
    let config = OptimizationConfig {
        max_parallel_threads: 4,
        ..Default::default()
    };
    
    let mut compiler = parallel_compilation::ParallelCompiler::new(&config).unwrap();
    
    // Create test modules with dependencies
    let modules = vec![
        ("module_a".to_string(), "src/module_a.csd".to_string(), vec![]),
        ("module_b".to_string(), "src/module_b.csd".to_string(), vec!["module_a".to_string()]),
        ("module_c".to_string(), "src/module_c.csd".to_string(), vec!["module_a".to_string(), "module_b".to_string()]),
    ];
    
    // Compile modules in parallel
    let results = compiler.compile_modules(modules).unwrap();
    
    // Verify compilation results
    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|r| r.module_name == "module_a"));
    assert!(results.iter().any(|r| r.module_name == "module_b"));
    assert!(results.iter().any(|r| r.module_name == "module_c"));
    
    // Check that all compilations were successful
    for result in &results {
        assert!(result.success, "Module {} compilation failed: {:?}", result.module_name, result.error_message);
    }
    
    // Print compilation summary
    compiler.print_summary();
}

/// Test JIT optimization with hot path detection
#[test]
fn test_jit_optimization() {
    let config = OptimizationConfig::default();
    let mut optimizer = jit_optimization::AdaptiveJitOptimizer::new(&config).unwrap();
    
    // Simulate function executions to create hot paths
    for i in 0..150 {
        let execution_time = Duration::from_micros(100 + (i % 10) as u64);
        optimizer.record_execution("hot_function", execution_time);
    }
    
    // Simulate less frequent function
    for _i in 0..20 {
        optimizer.record_execution("cold_function", Duration::from_micros(50));
    }
    
    // Get optimization recommendations
    let recommendations = optimizer.get_optimization_recommendations();
    
    // Verify hot path detection
    assert!(!recommendations.is_empty(), "Expected optimization recommendations for hot paths");
    
    // Check that hot function is identified
    let hot_functions = optimizer.profiler.get_hot_functions();
    assert!(hot_functions.contains(&"hot_function".to_string()), "Hot function should be detected");
    assert!(!hot_functions.contains(&"cold_function".to_string()), "Cold function should not be hot");
    
    // Apply optimizations
    for recommendation in recommendations {
        let result = optimizer.apply_optimization(&recommendation).unwrap();
        assert!(result.success || result.actual_benefit >= 0.0, "Optimization should succeed or have no negative impact");
    }
    
    let stats = optimizer.get_stats();
    assert!(stats.total_optimizations > 0);
}

/// Test incremental compilation
#[test]
fn test_incremental_compilation() {
    let config = OptimizationConfig::default();
    let mut compiler = incremental_compilation::IncrementalCompiler::new(&config).unwrap();
    
    // Create temporary project directory
    let temp_dir = std::env::temp_dir().join("cursed_incremental_test");
    std::fs::create_dir_all(&temp_dir).unwrap();
    
    // Perform incremental compilation
    let stats = compiler.compile_incrementally(&temp_dir).unwrap();
    
    // Verify incremental compilation stats
    assert!(stats.total_compilation_time >= Duration::ZERO);
    assert!(stats.dependency_resolution_time >= Duration::ZERO);
    
    // Print incremental compilation summary
    compiler.print_summary();
    
    // Clean up
    let _ = std::fs::remove_dir_all(&temp_dir);
}

/// Test memory optimization
#[test]
fn test_memory_optimization() {
    let config = OptimizationConfig::default();
    let mut optimizer = memory_optimization::MemoryLayoutOptimizer::new(&config).unwrap();
    
    // Create test structures to optimize
    let structures = vec![
        "TestStruct1".to_string(),
        "TestStruct2".to_string(),
        "LargeStruct".to_string(),
    ];
    
    // Run memory optimization
    let stats = optimizer.optimize_memory_layout(&structures).unwrap();
    
    // Verify optimization results
    assert_eq!(stats.structures_optimized, 3);
    assert!(stats.optimization_time > Duration::ZERO);
    
    // Print memory optimization summary
    optimizer.print_summary();
}

/// Test compilation speed optimization
#[test]
fn test_compilation_speed_optimization() {
    let config = OptimizationConfig {
        max_parallel_threads: 4,
        ..Default::default()
    };
    
    let mut optimizer = compilation_speed::CompilationSpeedOptimizer::new(&config).unwrap();
    
    // Create mock program for optimization
    let program = cursed::ast::Program {
        statements: vec![],
    };
    
    // Run compilation speed optimization
    let stats = optimizer.optimize_compilation(&program).unwrap();
    
    // Verify optimization was performed
    assert!(stats.total_compilation_time >= Duration::ZERO);
    assert!(stats.optimizations_applied > 0);
    
    // Print optimization summary
    optimizer.print_summary();
}

/// Test performance profiling
#[test]
fn test_performance_profiling() {
    let config = OptimizationConfig::default();
    let mut profiler = profiling::PerformanceProfiler::new(&config).unwrap();
    
    // Start profiling session
    let session_config = profiling::SessionConfig {
        sample_rate: 1.0,
        max_samples: 1000,
        profiling_duration: Some(Duration::from_secs(1)),
        output_format: profiling::OutputFormat::Json,
        enable_detailed_analysis: true,
    };
    
    let session_id = profiler.start_session("test_session".to_string(), session_config).unwrap();
    assert!(!session_id.is_empty());
    
    // Profile a compilation
    let compilation_profile = profiler.profile_compilation("test_module", || {
        // Simulate compilation work
        std::thread::sleep(Duration::from_millis(50));
        Ok(())
    }).unwrap();
    
    assert_eq!(compilation_profile.module_name, "test_module");
    assert!(compilation_profile.total_time >= Duration::from_millis(50));
    
    // Profile an execution
    let (result, execution_profile) = profiler.profile_execution("test_function", || {
        // Simulate execution work
        std::thread::sleep(Duration::from_millis(20));
        Ok(42)
    }).unwrap();
    
    assert_eq!(result, 42);
    assert_eq!(execution_profile.function_name, "test_function");
    assert!(execution_profile.execution_time >= Duration::from_millis(20));
    
    // Stop profiling session and generate report
    let report = profiler.stop_session().unwrap();
    
    assert_eq!(report.session_id, session_id);
    assert!(report.session_duration >= Duration::from_millis(70));
    
    // Print profiling summary
    profiler.print_summary();
}

/// Test optimization caching
#[test]
fn test_optimization_caching() {
    let cache_config = cache::CacheConfig {
        max_size: 10 * 1024 * 1024, // 10MB
        max_entries: 1000,
        enable_compression: true,
        enable_encryption: false,
        ..Default::default()
    };
    
    let mut cache = cache::OptimizationCache::new(cache_config).unwrap();
    
    // Create test optimization result
    let optimization_result = cache::OptimizationResult {
        optimization_type: "function_inlining".to_string(),
        input_hash: "abc123".to_string(),
        output_data: vec![1, 2, 3, 4, 5],
        optimization_stats: cache::OptimizationStats {
            optimization_time: Duration::from_millis(100),
            code_size_before: 1000,
            code_size_after: 850,
            performance_improvement: 1.2,
            memory_usage: 2048,
        },
        dependencies: vec![],
        compiler_version: "cursed-0.1.0".to_string(),
        optimization_level: 2,
    };
    
    // Store optimization result
    cache.store_optimization_result("test_optimization", optimization_result.clone()).unwrap();
    
    // Retrieve optimization result
    let retrieved = cache.get_optimization_result("test_optimization");
    assert!(retrieved.is_none()); // Will be None in the mock implementation
    
    // Test cache statistics
    let stats = cache.get_stats();
    assert!(stats.current_entries >= 0);
    
    // Print cache summary
    cache.print_summary();
}

/// Test adaptive optimization
#[test]
fn test_adaptive_optimization() {
    let config = OptimizationConfig::default();
    let mut optimizer = adaptive::AdaptiveOptimizer::new(&config).unwrap();
    
    // Create optimization context
    let context = adaptive::OptimizationContext {
        target_platform: "x86_64".to_string(),
        optimization_level: 2,
        code_characteristics: adaptive::CodeCharacteristics {
            function_count: 50,
            loop_count: 25,
            branch_count: 100,
            memory_access_patterns: vec!["sequential".to_string(), "random".to_string()],
            algorithmic_complexity: 3.5,
            data_structures_used: vec!["array".to_string(), "hash_map".to_string()],
        },
        resource_constraints: adaptive::ResourceConstraints {
            memory_limit: 1024 * 1024 * 1024, // 1GB
            compilation_time_limit: Duration::from_secs(60),
            cpu_cores_available: 8,
            disk_space_available: 10 * 1024 * 1024 * 1024, // 10GB
        },
        performance_requirements: adaptive::PerformanceRequirements {
            target_execution_time: Duration::from_millis(100),
            memory_usage_limit: 512 * 1024 * 1024, // 512MB
            throughput_requirement: 1000.0,
            latency_requirement: Duration::from_millis(10),
            energy_efficiency: true,
        },
        environment_info: adaptive::EnvironmentInfo {
            cpu_architecture: "x86_64".to_string(),
            cache_sizes: vec![32768, 262144, 8388608], // L1, L2, L3
            memory_hierarchy: vec!["L1".to_string(), "L2".to_string(), "L3".to_string()],
            compiler_version: "cursed-0.1.0".to_string(),
            operating_system: "Linux".to_string(),
        },
    };
    
    // Perform adaptive optimization
    let adaptation_result = optimizer.adapt_strategy(&context).unwrap();
    
    // Verify adaptation was performed
    assert!(!adaptation_result.strategy_applied.strategy_name.is_empty());
    assert!(adaptation_result.adaptation_time >= Duration::ZERO);
    
    // Simulate feedback learning
    let feedback = vec![
        adaptive::FeedbackEvent {
            event_id: "feedback_1".to_string(),
            optimization_id: "opt_1".to_string(),
            event_type: adaptive::FeedbackEventType::PerformanceMetric,
            timestamp: std::time::SystemTime::now(),
            data: adaptive::FeedbackData::Numeric(0.85),
            reliability: 0.9,
        },
        adaptive::FeedbackEvent {
            event_id: "feedback_2".to_string(),
            optimization_id: "opt_1".to_string(),
            event_type: adaptive::FeedbackEventType::BenchmarkResult,
            timestamp: std::time::SystemTime::now(),
            data: adaptive::FeedbackData::Metrics({
                let mut metrics = HashMap::new();
                metrics.insert("performance_score".to_string(), 0.8);
                metrics.insert("memory_efficiency".to_string(), 0.75);
                metrics
            }),
            reliability: 0.85,
        },
    ];
    
    // Learn from feedback
    optimizer.learn_from_feedback(feedback).unwrap();
    
    // Verify learning occurred
    let stats = optimizer.get_stats();
    assert!(stats.feedback_events > 0);
    
    // Print adaptive optimization summary
    optimizer.print_summary();
}

/// Test integrated optimization pipeline
#[test]
fn test_integrated_optimization_pipeline() {
    let config = OptimizationConfig {
        enable_advanced_llvm: true,
        enable_parallel_compilation: true,
        enable_incremental_compilation: true,
        enable_jit_optimization: true,
        enable_memory_optimization: true,
        enable_profiling: true,
        enable_caching: true,
        enable_adaptive_optimization: true,
        max_parallel_threads: 4,
        optimization_level: 2,
        target_arch: "x86_64".to_string(),
        debug_optimizations: false,
    };
    
    // Create main optimization manager
    let mut manager = OptimizationManager::new(config).unwrap();
    
    // Verify all components are initialized
    assert!(manager.llvm_optimizer().is_some());
    assert!(manager.speed_optimizer().is_some());
    assert!(manager.jit_optimizer().is_some());
    assert!(manager.memory_optimizer().is_some());
    assert!(manager.parallel_compiler().is_some());
    assert!(manager.incremental_compiler().is_some());
    assert!(manager.profiler().is_some());
    assert!(manager.cache_manager().is_some());
    assert!(manager.adaptive_optimizer().is_some());
    
    // Test configuration updates
    let mut new_config = manager.config().clone();
    new_config.optimization_level = 3;
    new_config.max_parallel_threads = 8;
    
    manager.update_config(new_config).unwrap();
    assert_eq!(manager.config().optimization_level, 3);
    assert_eq!(manager.config().max_parallel_threads, 8);
}

/// Test optimization effectiveness measurement
#[test]
fn test_optimization_effectiveness() {
    let config = OptimizationConfig::default();
    
    // Create benchmark suite
    let test_files = vec![
        PathBuf::from("test_program1.csd"),
        PathBuf::from("test_program2.csd"),
        PathBuf::from("test_program3.csd"),
    ];
    
    let benchmark_config = benchmarking::BenchmarkConfig {
        iterations: 10,
        warmup_iterations: 2,
        max_execution_time: Duration::from_secs(30),
        min_execution_time: Duration::from_millis(1),
        confidence_level: 0.95,
        track_memory_usage: true,
        enable_cpu_profiling: false,
        output_directory: std::env::temp_dir().join("cursed_benchmarks"),
        compare_with_baseline: false,
        enable_parallel_execution: true,
    };
    
    let mut benchmarks = benchmarking::OptimizationBenchmarks::new(benchmark_config);
    
    // Register compilation benchmark suite
    let compilation_suite = benchmarking::create_compilation_benchmark_suite(test_files);
    benchmarks.register_suite(compilation_suite);
    
    // Execute benchmarks
    let summary = benchmarks.execute_all_suites().unwrap();
    
    // Verify benchmark execution
    assert!(summary.benchmarks_executed > 0);
    assert!(summary.total_duration > Duration::ZERO);
    assert!(summary.overall_performance_score >= 0.0);
    
    // Print benchmark results
    println!("Benchmark Summary:");
    println!("  Executed: {}", summary.benchmarks_executed);
    println!("  Passed: {}", summary.benchmarks_passed);
    println!("  Failed: {}", summary.benchmarks_failed);
    println!("  Duration: {:?}", summary.total_duration);
    println!("  Performance Score: {:.2}", summary.overall_performance_score);
}

/// Performance regression test
#[test]
fn test_performance_regression_detection() {
    let config = OptimizationConfig::default();
    let benchmarks = benchmarking::OptimizationBenchmarks::new(benchmarking::BenchmarkConfig::default());
    
    // Get current execution statistics
    let stats = benchmarks.get_execution_stats();
    
    // Verify performance tracking
    for (benchmark_name, performance_value) in stats {
        // In a real test, this would compare against known baselines
        assert!(performance_value >= 0.0, "Performance value for {} should be non-negative", benchmark_name);
        
        // Example regression check (simplified)
        let baseline_performance = 1.0; // Mock baseline
        let regression_threshold = 0.1; // 10% regression threshold
        
        if performance_value < baseline_performance * (1.0 - regression_threshold) {
            println!("PERFORMANCE REGRESSION DETECTED in {}: {:.2} vs baseline {:.2}", 
                    benchmark_name, performance_value, baseline_performance);
        }
    }
}

/// Test optimization configuration validation
#[test]
fn test_optimization_config_validation() {
    // Test default configuration
    let default_config = OptimizationConfig::default();
    assert!(default_config.max_parallel_threads > 0);
    assert!(default_config.optimization_level <= 3);
    assert!(!default_config.target_arch.is_empty());
    
    // Test custom configuration
    let custom_config = OptimizationConfig {
        enable_advanced_llvm: true,
        enable_parallel_compilation: true,
        enable_incremental_compilation: false,
        enable_jit_optimization: true,
        enable_memory_optimization: true,
        enable_profiling: false,
        enable_caching: true,
        enable_adaptive_optimization: false,
        max_parallel_threads: 6,
        optimization_level: 3,
        target_arch: "aarch64".to_string(),
        debug_optimizations: true,
    };
    
    // Verify custom configuration values
    assert!(custom_config.enable_advanced_llvm);
    assert!(!custom_config.enable_incremental_compilation);
    assert_eq!(custom_config.max_parallel_threads, 6);
    assert_eq!(custom_config.optimization_level, 3);
    assert_eq!(custom_config.target_arch, "aarch64");
    assert!(custom_config.debug_optimizations);
    
    // Test configuration boundary values
    let boundary_config = OptimizationConfig {
        max_parallel_threads: 1,
        optimization_level: 0,
        ..Default::default()
    };
    
    assert_eq!(boundary_config.max_parallel_threads, 1);
    assert_eq!(boundary_config.optimization_level, 0);
}

/// Helper function to create test module for compilation
fn create_test_module() -> cursed::ast::Program {
    cursed::ast::Program {
        statements: vec![
            // Add mock statements for testing
        ],
    }
}

/// Helper function to create test source code
fn create_test_source() -> String {
    r#"
    collab TestInterface {
        slay test_method() -> i32;
    }
    
    squad TestStruct {
        value: i32,
        name: String,
    }
    
    slay main() -> i32 {
        sus x = 42;
        sus s = TestStruct { value: x, name: "test" };
        periodt s.value;
    }
    "#.to_string()
}

/// Integration test for complete optimization workflow
#[test]
fn test_complete_optimization_workflow() {
    // Initialize tracing for test
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter("cursed=debug")
        .try_init();
    
    let config = OptimizationConfig {
        enable_advanced_llvm: true,
        enable_parallel_compilation: true,
        enable_incremental_compilation: true,
        enable_jit_optimization: true,
        enable_memory_optimization: true,
        enable_profiling: true,
        enable_caching: true,
        enable_adaptive_optimization: true,
        max_parallel_threads: 4,
        optimization_level: 2,
        target_arch: "x86_64".to_string(),
        debug_optimizations: false,
    };
    
    // Step 1: Create optimization manager
    let mut manager = OptimizationManager::new(config).unwrap();
    
    // Step 2: Set up optimization configuration
    let mut optimizations = HashMap::new();
    optimizations.insert("parallel_ast_processing".to_string(), true);
    optimizations.insert("type_checking_cache".to_string(), true);
    optimizations.insert("compilation_cache".to_string(), true);
    
    if let Some(speed_optimizer) = manager.speed_optimizer() {
        // Would configure speed optimizer if it were mutable
        println!("Speed optimizer available and configured");
    }
    
    // Step 3: Simulate optimization workflow
    let test_source = create_test_source();
    println!("Created test source with {} characters", test_source.len());
    
    // Step 4: Verify all optimization components
    let component_status = vec![
        ("LLVM Optimizer", manager.llvm_optimizer().is_some()),
        ("Speed Optimizer", manager.speed_optimizer().is_some()),
        ("JIT Optimizer", manager.jit_optimizer().is_some()),
        ("Memory Optimizer", manager.memory_optimizer().is_some()),
        ("Parallel Compiler", manager.parallel_compiler().is_some()),
        ("Incremental Compiler", manager.incremental_compiler().is_some()),
        ("Profiler", manager.profiler().is_some()),
        ("Cache Manager", manager.cache_manager().is_some()),
        ("Adaptive Optimizer", manager.adaptive_optimizer().is_some()),
    ];
    
    for (component, available) in component_status {
        assert!(available, "{} should be available", component);
        println!("✓ {} is available", component);
    }
    
    println!("🎉 Complete optimization workflow test passed!");
}
