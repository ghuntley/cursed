/// Comprehensive Performance Optimization System Tests
/// 
/// Tests all aspects of the performance optimization system including:
/// - Smart optimization defaults
/// - Adaptive optimization
/// - Compilation speed improvements
/// - Runtime optimizations
/// - Performance profiling

use cursed::optimization::{
    performance_system::{
        PerformanceOptimizationSystem, PerformanceSystemConfig, PerformanceMonitoringLevel,
        ParallelConfig, CacheConfig, AdaptiveDecisionType, RecommendationType,
    },
    BuildProfile, OptimizationLevel,
    compilation_speed::{CompilationUnit, CompilationStatus},
    benchmarking::BenchmarkType,
};
use std::time::{Duration, SystemTime};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_performance_system_initialization() {
    let config = PerformanceSystemConfig::default();
    let system = PerformanceOptimizationSystem::new(config).unwrap();
    
    assert_eq!(system.get_config().build_profile, BuildProfile::Release);
    assert!(system.get_config().enable_adaptive_optimization);
    assert!(system.get_config().enable_compilation_speed_optimizations);
    assert!(system.get_config().enable_advanced_runtime_optimizations);
}

#[test]
fn test_build_profile_optimization_defaults() {
    // Test development profile
    let dev_config = PerformanceSystemConfig {
        build_profile: BuildProfile::Development,
        ..Default::default()
    };
    let dev_system = PerformanceOptimizationSystem::new(dev_config).unwrap();
    assert_eq!(dev_system.get_config().build_profile, BuildProfile::Development);

    // Test production profile
    let prod_config = PerformanceSystemConfig {
        build_profile: BuildProfile::Production,
        ..Default::default()
    };
    let prod_system = PerformanceOptimizationSystem::new(prod_config).unwrap();
    assert_eq!(prod_system.get_config().build_profile, BuildProfile::Production);

    // Test debug profile
    let debug_config = PerformanceSystemConfig {
        build_profile: BuildProfile::Debug,
        ..Default::default()
    };
    let debug_system = PerformanceOptimizationSystem::new(debug_config).unwrap();
    assert_eq!(debug_system.get_config().build_profile, BuildProfile::Debug);
}

#[test]
fn test_compilation_time_budget_adaptation() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = PerformanceSystemConfig {
        build_profile: BuildProfile::Production,
        compilation_time_budget: 1.0, // Very short budget (1 second)
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let system = PerformanceOptimizationSystem::new(config).unwrap();
    
    // Create a large compilation unit that would exceed budget
    let large_unit = CompilationUnit {
        id: "large_module".to_string(),
        source_path: PathBuf::from("large.csd"),
        module_name: "large".to_string(),
        source_code: generate_large_source_code(10000), // 10k lines
        dependencies: vec![],
        last_modified: SystemTime::now(),
        status: CompilationStatus::Pending,
        priority: 1,
        content_hash: String::new(),
    };
    
    let session_id = system.start_session("budget_test".to_string()).unwrap();
    
    // Compile with time budget constraint
    let results = system.compile_with_smart_optimization(vec![large_unit]).unwrap();
    
    // Should have made adaptive decisions to meet budget
    assert!(!results.adaptive_decisions.is_empty());
    
    // Check if build profile was adjusted
    let has_profile_change = results.adaptive_decisions.iter().any(|decision| {
        matches!(decision.decision_type, AdaptiveDecisionType::BuildProfileChange { .. })
    });
    assert!(has_profile_change);
    
    system.end_session().unwrap();
}

#[test]
fn test_adaptive_optimization_learning() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = PerformanceSystemConfig {
        enable_adaptive_optimization: true,
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let system = PerformanceOptimizationSystem::new(config).unwrap();
    
    // Simulate multiple compilation sessions to train adaptive system
    for i in 0..5 {
        let session_id = system.start_session(format!("adaptive_test_{}", i)).unwrap();
        
        let unit = CompilationUnit {
            id: format!("module_{}", i),
            source_path: PathBuf::from(format!("test_{}.csd", i)),
            module_name: format!("test_{}", i),
            source_code: generate_test_source_code(i * 100 + 50),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
            content_hash: String::new(),
        };
        
        let results = system.compile_with_smart_optimization(vec![unit]).unwrap();
        
        // System should learn and make adaptive decisions
        if i > 2 {
            // After a few iterations, should start making adaptive decisions
            assert!(!results.adaptive_decisions.is_empty());
        }
        
        system.end_session().unwrap();
    }
}

#[test]
fn test_parallel_compilation_optimization() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = PerformanceSystemConfig {
        enable_compilation_speed_optimizations: true,
        parallel_config: ParallelConfig {
            max_threads: 4,
            enable_parallel_parsing: true,
            enable_parallel_type_checking: true,
            enable_parallel_optimization: true,
            ..Default::default()
        },
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let system = PerformanceOptimizationSystem::new(config).unwrap();
    
    // Create multiple independent compilation units
    let units: Vec<CompilationUnit> = (0..8).map(|i| {
        CompilationUnit {
            id: format!("parallel_module_{}", i),
            source_path: PathBuf::from(format!("parallel_{}.csd", i)),
            module_name: format!("parallel_{}", i),
            source_code: generate_test_source_code(200),
            dependencies: vec![], // Independent units for parallel compilation
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
            content_hash: String::new(),
        }
    }).collect();
    
    let session_id = system.start_session("parallel_test".to_string()).unwrap();
    
    let start_time = std::time::Instant::now();
    let results = system.compile_with_smart_optimization(units).unwrap();
    let compilation_time = start_time.elapsed();
    
    // Compilation should complete successfully
    assert_eq!(results.compilation_results.len(), 8);
    
    // All compilations should succeed
    let successful_compilations = results.compilation_results.iter()
        .filter(|(_, result)| result.is_ok())
        .count();
    assert_eq!(successful_compilations, 8);
    
    // Parallel efficiency should be reasonable
    assert!(results.performance_metrics.parallel_efficiency > 0.0);
    
    system.end_session().unwrap();
}

#[test]
fn test_incremental_compilation_caching() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = PerformanceSystemConfig {
        enable_compilation_speed_optimizations: true,
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            enable_ast_caching: true,
            enable_type_cache: true,
            enable_optimization_cache: true,
            max_cache_size_mb: 100,
            ..Default::default()
        },
        ..Default::default()
    };
    
    let system = PerformanceOptimizationSystem::new(config).unwrap();
    
    let unit = CompilationUnit {
        id: "cache_test".to_string(),
        source_path: PathBuf::from("cache_test.csd"),
        module_name: "cache_test".to_string(),
        source_code: generate_test_source_code(500),
        dependencies: vec![],
        last_modified: SystemTime::now(),
        status: CompilationStatus::Pending,
        priority: 1,
        content_hash: String::new(),
    };
    
    // First compilation - should populate cache
    let session1_id = system.start_session("cache_test_1".to_string()).unwrap();
    let results1 = system.compile_with_smart_optimization(vec![unit.clone()]).unwrap();
    let first_compile_time = results1.performance_metrics.total_time;
    system.end_session().unwrap();
    
    // Second compilation - should use cache
    let session2_id = system.start_session("cache_test_2".to_string()).unwrap();
    let results2 = system.compile_with_smart_optimization(vec![unit]).unwrap();
    let second_compile_time = results2.performance_metrics.total_time;
    system.end_session().unwrap();
    
    // Second compilation should be faster due to caching
    assert!(second_compile_time < first_compile_time);
    
    // Cache hit rate should be reasonable
    assert!(results2.performance_metrics.cache_hit_rate > 0.0);
}

#[test]
fn test_performance_monitoring_levels() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test minimal monitoring
    let minimal_config = PerformanceSystemConfig {
        performance_monitoring_level: PerformanceMonitoringLevel::Minimal,
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().join("minimal"),
            ..Default::default()
        },
        ..Default::default()
    };
    let minimal_system = PerformanceOptimizationSystem::new(minimal_config).unwrap();
    
    // Test comprehensive monitoring
    let comprehensive_config = PerformanceSystemConfig {
        performance_monitoring_level: PerformanceMonitoringLevel::Comprehensive,
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().join("comprehensive"),
            ..Default::default()
        },
        ..Default::default()
    };
    let comprehensive_system = PerformanceOptimizationSystem::new(comprehensive_config).unwrap();
    
    // Both should initialize successfully
    assert_eq!(minimal_system.get_config().performance_monitoring_level, PerformanceMonitoringLevel::Minimal);
    assert_eq!(comprehensive_system.get_config().performance_monitoring_level, PerformanceMonitoringLevel::Comprehensive);
}

#[test]
fn test_performance_recommendations_generation() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = PerformanceSystemConfig {
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let system = PerformanceOptimizationSystem::new(config).unwrap();
    
    // Create compilation scenario that would generate recommendations
    let units = vec![
        CompilationUnit {
            id: "slow_module".to_string(),
            source_path: PathBuf::from("slow.csd"),
            module_name: "slow".to_string(),
            source_code: generate_large_source_code(5000), // Large enough to be slow
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
            content_hash: String::new(),
        }
    ];
    
    let session_id = system.start_session("recommendation_test".to_string()).unwrap();
    let results = system.compile_with_smart_optimization(units).unwrap();
    system.end_session().unwrap();
    
    // Should generate performance recommendations
    assert!(!results.recommendations.is_empty());
    
    // Check recommendation types
    let has_compilation_speed_rec = results.recommendations.iter().any(|rec| {
        matches!(rec.recommendation_type, RecommendationType::CompilationSpeed)
    });
    let has_caching_rec = results.recommendations.iter().any(|rec| {
        matches!(rec.recommendation_type, RecommendationType::Caching)
    });
    
    // Should have at least one useful recommendation
    assert!(has_compilation_speed_rec || has_caching_rec);
}

#[test]
fn test_benchmark_integration() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = PerformanceSystemConfig {
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let system = PerformanceOptimizationSystem::new(config).unwrap();
    
    // Run compilation benchmark
    let benchmark_results = system.run_performance_benchmark(BenchmarkType::Compilation).unwrap();
    
    // Benchmark should complete successfully
    assert!(benchmark_results.iterations > 0);
    assert!(benchmark_results.average_time > Duration::from_nanos(1));
    assert!(benchmark_results.throughput >= 0.0);
}

#[test]
fn test_performance_report_generation() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = PerformanceSystemConfig {
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let system = PerformanceOptimizationSystem::new(config).unwrap();
    
    // Run a compilation session
    let session_id = system.start_session("report_test".to_string()).unwrap();
    
    let unit = CompilationUnit {
        id: "report_module".to_string(),
        source_path: PathBuf::from("report.csd"),
        module_name: "report".to_string(),
        source_code: generate_test_source_code(100),
        dependencies: vec![],
        last_modified: SystemTime::now(),
        status: CompilationStatus::Pending,
        priority: 1,
        content_hash: String::new(),
    };
    
    let results = system.compile_with_smart_optimization(vec![unit]).unwrap();
    system.end_session().unwrap();
    
    // Generate performance report
    let report = system.generate_performance_report();
    
    // Report should contain key sections
    assert!(report.contains("# CURSED Compiler Performance Report"));
    assert!(report.contains("## Current Session"));
    assert!(report.contains("## Compilation Performance"));
    assert!(report.contains("## Performance Recommendations"));
    assert!(report.contains("## Build Profile Analysis"));
    assert!(report.contains("## Configuration Summary"));
}

#[test]
fn test_cache_management() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = PerformanceSystemConfig {
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            max_cache_size_mb: 50, // Small cache for testing
            enable_ast_caching: true,
            enable_type_cache: true,
            enable_optimization_cache: true,
            ..Default::default()
        },
        ..Default::default()
    };
    
    let system = PerformanceOptimizationSystem::new(config).unwrap();
    
    // Populate cache with multiple compilations
    for i in 0..5 {
        let session_id = system.start_session(format!("cache_test_{}", i)).unwrap();
        
        let unit = CompilationUnit {
            id: format!("cache_module_{}", i),
            source_path: PathBuf::from(format!("cache_{}.csd", i)),
            module_name: format!("cache_{}", i),
            source_code: generate_test_source_code(200),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
            content_hash: String::new(),
        };
        
        let results = system.compile_with_smart_optimization(vec![unit]).unwrap();
        system.end_session().unwrap();
    }
    
    // Clear caches
    system.clear_caches().unwrap();
    
    // Should succeed without errors
}

#[test]
fn test_session_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = PerformanceSystemConfig {
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let system = PerformanceOptimizationSystem::new(config).unwrap();
    
    // Start session
    let session_id = system.start_session("lifecycle_test".to_string()).unwrap();
    assert!(!session_id.is_empty());
    assert!(session_id.contains("lifecycle_test"));
    
    // Perform some work
    let unit = CompilationUnit {
        id: "lifecycle_module".to_string(),
        source_path: PathBuf::from("lifecycle.csd"),
        module_name: "lifecycle".to_string(),
        source_code: "facts x = 42;".to_string(),
        dependencies: vec![],
        last_modified: SystemTime::now(),
        status: CompilationStatus::Pending,
        priority: 1,
        content_hash: String::new(),
    };
    
    let results = system.compile_with_smart_optimization(vec![unit]).unwrap();
    assert_eq!(results.compilation_results.len(), 1);
    
    // End session
    let session = system.end_session().unwrap();
    assert!(session.is_some());
    
    let session = session.unwrap();
    assert_eq!(session.name, "lifecycle_test");
    assert!(session.start_time.elapsed() > Duration::from_nanos(1));
}

#[test]
fn test_configuration_updates() {
    let temp_dir = TempDir::new().unwrap();
    
    let initial_config = PerformanceSystemConfig {
        build_profile: BuildProfile::Development,
        compilation_time_budget: 10.0,
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let mut system = PerformanceOptimizationSystem::new(initial_config).unwrap();
    
    // Update configuration
    let updated_config = PerformanceSystemConfig {
        build_profile: BuildProfile::Production,
        compilation_time_budget: 60.0,
        enable_adaptive_optimization: false,
        cache_config: CacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        },
        ..Default::default()
    };
    
    system.update_config(updated_config).unwrap();
    
    // Verify configuration was updated
    assert_eq!(system.get_config().build_profile, BuildProfile::Production);
    assert_eq!(system.get_config().compilation_time_budget, 60.0);
    assert!(!system.get_config().enable_adaptive_optimization);
}

// Helper functions for generating test code

fn generate_test_source_code(lines: usize) -> String {
    let mut code = String::new();
    
    // Add some imports
    code.push_str("// CURSED test module\n");
    code.push_str("import \"stdlib::io\";\n");
    code.push_str("import \"stdlib::math\";\n\n");
    
    // Add variable declarations
    for i in 0..lines.min(50) {
        code.push_str(&format!("facts var_{} = {};\n", i, i * 2));
    }
    
    code.push_str("\n");
    
    // Add function definitions
    let func_count = (lines / 10).max(1);
    for i in 0..func_count {
        code.push_str(&format!(
            "slay function_{}(x: i32) -> i32 {{\n    lowkey (x > 0) {{\n        return x * 2;\n    }} highkey {{\n        return 0;\n    }}\n}}\n\n",
            i
        ));
    }
    
    // Add a main function
    code.push_str("slay main() {\n");
    for i in 0..lines.min(20) {
        code.push_str(&format!("    println(\"Line {}\");\n", i));
    }
    code.push_str("}\n");
    
    code
}

fn generate_large_source_code(lines: usize) -> String {
    let mut code = String::new();
    
    // Generate a large amount of code to simulate real projects
    code.push_str("// Large CURSED module for performance testing\n\n");
    
    // Add many variable declarations
    for i in 0..lines / 4 {
        code.push_str(&format!("facts large_var_{} = {} + {} * {};\n", i, i, i * 2, i % 100));
    }
    
    code.push_str("\n");
    
    // Add many function definitions
    for i in 0..(lines / 10) {
        code.push_str(&format!(
            "slay large_function_{}(a: i32, b: i32, c: i32) -> i32 {{\n",
            i
        ));
        code.push_str("    facts result = 0;\n");
        code.push_str("    lowkey (a > b) {\n");
        code.push_str("        result = a + c;\n");
        code.push_str("    } highkey lowkey (b > c) {\n");
        code.push_str("        result = b + a;\n");
        code.push_str("    } highkey {\n");
        code.push_str("        result = c + a + b;\n");
        code.push_str("    }\n");
        code.push_str("    return result;\n");
        code.push_str("}\n\n");
    }
    
    // Add complex control structures
    for i in 0..(lines / 20) {
        code.push_str(&format!("slay complex_function_{}() {{\n", i));
        code.push_str("    periodt (sus j = 0; j < 100; j++) {\n");
        code.push_str("        lowkey (j % 2 == 0) {\n");
        code.push_str("            println(\"Even\");\n");
        code.push_str("        } highkey {\n");
        code.push_str("            println(\"Odd\");\n");
        code.push_str("        }\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");
    }
    
    code
}
