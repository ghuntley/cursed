/// Comprehensive Link-Time Optimization (LTO) Test Suite
/// 
/// Tests the complete LTO system including cross-module optimization,
/// inter-procedural analysis, and whole-program optimization capabilities.

#[path = "common.rs"]
pub mod common;

use cursed::optimization::lto::{
    LtoOptimizer, LtoConfig, LtoLevel, LtoCompilationUnit, LtoStatistics,
    CrossModuleAnalysis, CallGraph, GlobalUsageAnalysis, FunctionUsageAnalysis,
    ConstantPropagationOpportunity, DeadCodeCandidate, InliningOpportunity,
    LtoResult, OptimizationResults
};
use cursed::error::{Error, Result};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::Duration;
use tracing::{info, debug};

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

/// Test basic LTO optimizer creation and configuration
#[test]
fn test_lto_optimizer_creation() {
    init_tracing!();
    
    let config = LtoConfig::default();
    let optimizer = LtoOptimizer::new(config);
    assert!(optimizer.is_ok());
    
    let optimizer = optimizer.unwrap();
    assert_eq!(optimizer.get_config().level, LtoLevel::None);
    assert!(optimizer.get_config().enable_cross_module_inlining);
    assert!(optimizer.get_config().enable_whole_program_dce);
}

/// Test LTO level parsing and configuration
#[test]
fn test_lto_level_configuration() {
    init_tracing!();
    
    // Test level parsing
    assert_eq!(LtoLevel::from_str("none").unwrap(), LtoLevel::None);
    assert_eq!(LtoLevel::from_str("thin").unwrap(), LtoLevel::Thin);
    assert_eq!(LtoLevel::from_str("full").unwrap(), LtoLevel::Full);
    assert_eq!(LtoLevel::from_str("fat").unwrap(), LtoLevel::Full);
    assert!(LtoLevel::from_str("invalid").is_err());
    
    // Test string representation
    assert_eq!(LtoLevel::None.as_str(), "none");
    assert_eq!(LtoLevel::Thin.as_str(), "thin");
    assert_eq!(LtoLevel::Full.as_str(), "full");
}

/// Test compilation unit creation and management
#[test]
fn test_compilation_unit_management() {
    init_tracing!();
    
    let mut unit = LtoCompilationUnit::new(
        "test_module".to_string(),
        PathBuf::from("test_module.bc")
    );
    
    assert_eq!(unit.id, "test_module");
    assert_eq!(unit.module_path, PathBuf::from("test_module.bc"));
    assert!(unit.exported_functions.is_empty());
    assert!(unit.exported_globals.is_empty());
    
    // Add some exports
    unit.exported_functions.insert("main".to_string());
    unit.exported_functions.insert("helper".to_string());
    unit.exported_globals.insert("config".to_string());
    
    assert_eq!(unit.exported_functions.len(), 2);
    assert_eq!(unit.exported_globals.len(), 1);
    assert!(unit.exported_functions.contains("main"));
    assert!(unit.exported_globals.contains("config"));
}

/// Test multiple compilation units in LTO optimizer
#[test]
fn test_multiple_compilation_units() {
    init_tracing!();
    
    let config = LtoConfig {
        level: LtoLevel::Thin,
        ..Default::default()
    };
    let mut optimizer = LtoOptimizer::new(config).unwrap();
    
    // Create multiple compilation units
    for i in 0..5 {
        let mut unit = LtoCompilationUnit::new(
            format!("module_{}", i),
            PathBuf::from(format!("module_{}.bc", i))
        );
        
        // Add some mock functions
        unit.exported_functions.insert(format!("function_{}_main", i));
        unit.exported_functions.insert(format!("function_{}_helper", i));
        
        // Add some mock globals
        unit.exported_globals.insert(format!("global_{}_config", i));
        
        unit.size_estimate = 1000 + i * 200;
        optimizer.add_compilation_unit(unit);
    }
    
    let stats = optimizer.get_statistics();
    assert_eq!(stats.modules_processed, 0); // Not optimized yet
}

/// Test cross-module analysis functionality
#[test]
fn test_cross_module_analysis() {
    init_tracing!();
    
    let mut call_graph = CallGraph::default();
    let mut global_usage = GlobalUsageAnalysis::default();
    let mut function_usage = FunctionUsageAnalysis::default();
    
    // Build mock call graph
    call_graph.calls.insert("main".to_string(), 
        vec!["helper1".to_string(), "helper2".to_string()].into_iter().collect());
    call_graph.callers.insert("helper1".to_string(), 
        vec!["main".to_string()].into_iter().collect());
    call_graph.callers.insert("helper2".to_string(), 
        vec!["main".to_string()].into_iter().collect());
    
    // Build mock global usage
    global_usage.read_only_variables.insert("const_config".to_string());
    global_usage.constant_variables.insert("version".to_string(), "1.0".to_string());
    global_usage.mergeable_variables.push(vec!["var1".to_string(), "var2".to_string()]);
    
    // Build mock function usage
    function_usage.unreachable_functions.insert("unused_function".to_string());
    function_usage.single_use_functions.insert("helper1".to_string());
    function_usage.hot_functions.insert("main".to_string());
    function_usage.cold_functions.insert("debug_helper".to_string());
    
    let analysis = CrossModuleAnalysis {
        call_graph,
        global_usage,
        function_usage,
        constant_propagation_opportunities: Vec::new(),
        dead_code_candidates: Vec::new(),
        inlining_opportunities: Vec::new(),
    };
    
    // Verify analysis structure
    assert!(analysis.call_graph.calls.contains_key("main"));
    assert!(analysis.global_usage.read_only_variables.contains("const_config"));
    assert!(analysis.function_usage.unreachable_functions.contains("unused_function"));
}

/// Test constant propagation opportunity detection
#[test]
fn test_constant_propagation_opportunities() {
    init_tracing!();
    
    let opportunity = ConstantPropagationOpportunity {
        function: "global".to_string(),
        variable: "VERSION".to_string(),
        constant_value: "2.1.0".to_string(),
        usage_count: 15,
        estimated_benefit: 150,
    };
    
    assert_eq!(opportunity.function, "global");
    assert_eq!(opportunity.variable, "VERSION");
    assert_eq!(opportunity.constant_value, "2.1.0");
    assert_eq!(opportunity.usage_count, 15);
    assert_eq!(opportunity.estimated_benefit, 150);
}

/// Test dead code elimination candidate detection
#[test]
fn test_dead_code_elimination_candidates() {
    init_tracing!();
    
    let candidate = DeadCodeCandidate {
        module: "module_a".to_string(),
        function: Some("unused_helper".to_string()),
        location: "function::unused_helper".to_string(),
        reason: "Function is never called across all modules".to_string(),
        estimated_size_reduction: 250,
    };
    
    assert_eq!(candidate.module, "module_a");
    assert_eq!(candidate.function, Some("unused_helper".to_string()));
    assert_eq!(candidate.estimated_size_reduction, 250);
    assert!(candidate.reason.contains("never called"));
}

/// Test function inlining opportunity detection
#[test]
fn test_inlining_opportunities() {
    init_tracing!();
    
    let opportunity = InliningOpportunity {
        caller: "main".to_string(),
        callee: "small_helper".to_string(),
        call_count: 3,
        callee_size: 25,
        estimated_benefit: 40,
    };
    
    assert_eq!(opportunity.caller, "main");
    assert_eq!(opportunity.callee, "small_helper");
    assert_eq!(opportunity.call_count, 3);
    assert_eq!(opportunity.callee_size, 25);
    assert!(opportunity.estimated_benefit > 0);
}

/// Test LTO optimization with disabled level
#[test]
fn test_lto_optimization_disabled() {
    init_tracing!();
    
    let config = LtoConfig {
        level: LtoLevel::None,
        ..Default::default()
    };
    let mut optimizer = LtoOptimizer::new(config).unwrap();
    
    // Add a compilation unit
    let unit = LtoCompilationUnit::new(
        "test_module".to_string(),
        PathBuf::from("test_module.bc")
    );
    optimizer.add_compilation_unit(unit);
    
    // Run optimization (should skip)
    let result = optimizer.optimize();
    assert!(result.is_ok());
    
    let result = result.unwrap();
    let stats = result.statistics;
    assert_eq!(stats.modules_processed, 0); // Should be skipped
    assert_eq!(stats.functions_inlined, 0);
    assert_eq!(stats.dead_functions_eliminated, 0);
}

/// Test Thin LTO optimization workflow
#[test]
fn test_thin_lto_optimization() {
    init_tracing!();
    
    let config = LtoConfig {
        level: LtoLevel::Thin,
        enable_cross_module_inlining: true,
        enable_whole_program_dce: true,
        max_worker_threads: 2,
        thin_lto_partition_threshold: 500,
        ..Default::default()
    };
    let mut optimizer = LtoOptimizer::new(config).unwrap();
    
    // Add multiple compilation units
    for i in 0..3 {
        let mut unit = LtoCompilationUnit::new(
            format!("module_{}", i),
            PathBuf::from(format!("module_{}.bc", i))
        );
        
        // Add functions with patterns for analysis
        unit.exported_functions.insert(format!("module_{}_main", i));
        unit.exported_functions.insert(format!("module_{}_helper", i));
        if i == 2 {
            unit.exported_functions.insert("module_2_unused".to_string());
        }
        
        // Add globals with patterns
        unit.exported_globals.insert(format!("module_{}_const_config", i));
        
        unit.size_estimate = 300 + i * 100;
        optimizer.add_compilation_unit(unit);
    }
    
    // Run Thin LTO optimization
    let result = optimizer.optimize();
    assert!(result.is_ok());
    
    let result = result.unwrap();
    let stats = result.statistics;
    
    // Verify optimization was performed
    assert_eq!(stats.modules_processed, 3);
    assert!(stats.total_time > Duration::from_secs(0));
    assert!(stats.analysis_time > Duration::from_secs(0));
    assert!(stats.optimization_time > Duration::from_secs(0));
    
    // Check that some optimizations were applied
    assert!(stats.functions_inlined > 0 || stats.dead_functions_eliminated > 0 || stats.globals_optimized > 0);
}

/// Test Full LTO optimization workflow
#[test]
fn test_full_lto_optimization() {
    init_tracing!();
    
    let config = LtoConfig {
        level: LtoLevel::Full,
        enable_cross_module_inlining: true,
        enable_whole_program_dce: true,
        enable_global_variable_optimization: true,
        enable_cross_module_constant_propagation: true,
        enable_devirtualization: true,
        ..Default::default()
    };
    let mut optimizer = LtoOptimizer::new(config).unwrap();
    
    // Add compilation units with more complex patterns
    for i in 0..4 {
        let mut unit = LtoCompilationUnit::new(
            format!("complex_module_{}", i),
            PathBuf::from(format!("complex_module_{}.bc", i))
        );
        
        // Add functions with realistic patterns
        unit.exported_functions.insert(format!("complex_module_{}_main", i));
        unit.exported_functions.insert(format!("complex_module_{}_hot_function", i));
        unit.exported_functions.insert(format!("complex_module_{}_cold_function", i));
        
        // Add some functions that might be inlined
        if i < 2 {
            unit.exported_functions.insert(format!("complex_module_{}_small_helper", i));
        }
        
        // Add globals with various patterns
        unit.exported_globals.insert(format!("complex_module_{}_const_version", i));
        unit.exported_globals.insert(format!("complex_module_{}_config", i));
        
        unit.size_estimate = 800 + i * 200;
        optimizer.add_compilation_unit(unit);
    }
    
    // Run Full LTO optimization
    let result = optimizer.optimize();
    assert!(result.is_ok());
    
    let result = result.unwrap();
    let stats = result.statistics;
    
    // Verify comprehensive optimization was performed
    assert_eq!(stats.modules_processed, 4);
    assert!(stats.total_time > Duration::from_secs(0));
    
    // Full LTO should perform more optimizations
    let total_optimizations = stats.functions_inlined + stats.dead_functions_eliminated + 
                            stats.globals_optimized + stats.constants_propagated;
    assert!(total_optimizations > 0);
    
    // Check optimization effectiveness
    assert!(stats.optimization_effectiveness() > 0.0);
}

/// Test LTO statistics calculations
#[test]
fn test_lto_statistics_calculations() {
    init_tracing!();
    
    let mut stats = LtoStatistics {
        code_size_before: 10000,
        code_size_after: 8000,
        modules_processed: 5,
        functions_inlined: 8,
        dead_functions_eliminated: 3,
        globals_optimized: 5,
        constants_propagated: 12,
        ..Default::default()
    };
    
    // Test code size reduction calculation
    assert_eq!(stats.code_size_reduction_percent(), 20.0);
    
    // Test optimization effectiveness
    let expected_effectiveness = (8 + 3 + 5 + 12) as f64 / 5.0;
    assert_eq!(stats.optimization_effectiveness(), expected_effectiveness);
    
    // Test edge cases
    stats.code_size_before = 0;
    assert_eq!(stats.code_size_reduction_percent(), 0.0);
    
    stats.modules_processed = 0;
    assert!(stats.optimization_effectiveness() >= 0.0);
}

/// Test LTO report generation
#[test]
fn test_lto_report_generation() {
    init_tracing!();
    
    let config = LtoConfig {
        level: LtoLevel::Thin,
        enable_caching: true,
        ..Default::default()
    };
    let mut optimizer = LtoOptimizer::new(config).unwrap();
    
    // Add a compilation unit
    let unit = LtoCompilationUnit::new(
        "report_test_module".to_string(),
        PathBuf::from("report_test_module.bc")
    );
    optimizer.add_compilation_unit(unit);
    
    // Generate report before optimization
    let report = optimizer.generate_report();
    assert!(report.is_ok());
    
    let report_content = report.unwrap();
    assert!(report_content.contains("CURSED Link-Time Optimization Report"));
    assert!(report_content.contains("LTO Level"));
    assert!(report_content.contains("thin"));
    assert!(report_content.contains("Performance Breakdown"));
    assert!(report_content.contains("Optimization Results"));
}

/// Test LTO configuration validation
#[test]
fn test_lto_configuration_validation() {
    init_tracing!();
    
    // Test default configuration
    let default_config = LtoConfig::default();
    assert_eq!(default_config.level, LtoLevel::None);
    assert!(default_config.enable_cross_module_inlining);
    assert!(default_config.enable_whole_program_dce);
    assert!(default_config.enable_global_variable_optimization);
    assert!(default_config.enable_cross_module_constant_propagation);
    assert!(default_config.enable_devirtualization);
    assert!(default_config.enable_caching);
    
    // Test custom configuration
    let custom_config = LtoConfig {
        level: LtoLevel::Full,
        enable_cross_module_inlining: false,
        enable_whole_program_dce: true,
        max_worker_threads: 8,
        thin_lto_partition_threshold: 2000,
        enable_caching: false,
        enable_profiling: true,
        ..Default::default()
    };
    
    assert_eq!(custom_config.level, LtoLevel::Full);
    assert!(!custom_config.enable_cross_module_inlining);
    assert_eq!(custom_config.max_worker_threads, 8);
    assert_eq!(custom_config.thin_lto_partition_threshold, 2000);
    assert!(!custom_config.enable_caching);
    assert!(custom_config.enable_profiling);
}

/// Test error handling in LTO optimization
#[test]
fn test_lto_error_handling() {
    init_tracing!();
    
    // Test LTO level parsing errors
    let invalid_level = LtoLevel::from_str("invalid_level");
    assert!(invalid_level.is_err());
    if let Err(Error::Other(msg)) = invalid_level {
        assert!(msg.contains("Invalid LTO level"));
    }
    
    // Test optimizer creation with invalid cache directory
    let config = LtoConfig {
        enable_caching: true,
        cache_directory: Some(PathBuf::from("/invalid/path/that/cannot/be/created")),
        ..Default::default()
    };
    
    // This should still succeed as we create cache on demand
    let optimizer = LtoOptimizer::new(config);
    assert!(optimizer.is_ok());
}

/// Test LTO with empty compilation units
#[test]
fn test_lto_with_empty_units() {
    init_tracing!();
    
    let config = LtoConfig {
        level: LtoLevel::Thin,
        ..Default::default()
    };
    let mut optimizer = LtoOptimizer::new(config).unwrap();
    
    // Don't add any compilation units
    let result = optimizer.optimize();
    assert!(result.is_ok());
    
    let result = result.unwrap();
    assert_eq!(result.statistics.modules_processed, 0);
    assert_eq!(result.statistics.total_time, Duration::from_secs(0));
}

/// Test LTO cache functionality
#[test]
fn test_lto_cache_functionality() {
    init_tracing!();
    
    use cursed::optimization::lto::{LtoCache, CacheEntry};
    use std::collections::HashMap;
    
    // Create cache with default directory
    let mut cache = LtoCache::new(None);
    assert!(cache.is_ok());
    
    let mut cache = cache.unwrap();
    
    // Test cache operations
    let entry = CacheEntry {
        timestamp: std::time::SystemTime::now(),
        content_hash: "abcdef123456".to_string(),
        artifact_path: PathBuf::from("cached_artifact.o"),
        metadata: HashMap::new(),
    };
    
    // Put and get cache entry
    cache.put("test_key".to_string(), entry.clone());
    let retrieved = cache.get("test_key");
    assert!(retrieved.is_some());
    
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.content_hash, "abcdef123456");
    assert_eq!(retrieved.artifact_path, PathBuf::from("cached_artifact.o"));
    
    // Test non-existent key
    let missing = cache.get("non_existent_key");
    assert!(missing.is_none());
}

/// Test complex optimization scenarios
#[test]
fn test_complex_optimization_scenarios() {
    init_tracing!();
    
    let config = LtoConfig {
        level: LtoLevel::Full,
        enable_cross_module_inlining: true,
        enable_whole_program_dce: true,
        enable_global_variable_optimization: true,
        enable_cross_module_constant_propagation: true,
        enable_devirtualization: true,
        enable_profiling: true,
        ..Default::default()
    };
    let mut optimizer = LtoOptimizer::new(config).unwrap();
    
    // Create a complex scenario with interdependent modules
    let module_configs = vec![
        ("core", vec!["core_init", "core_process", "core_cleanup"], vec!["CORE_VERSION"]),
        ("utils", vec!["utils_helper", "utils_format", "utils_unused"], vec!["UTILS_CONFIG"]),
        ("api", vec!["api_handler", "api_virtual_call"], vec!["API_ENDPOINTS"]),
        ("main", vec!["main", "main_hot_path"], vec!["MAIN_CONFIG"]),
    ];
    
    for (module_name, functions, globals) in module_configs {
        let mut unit = LtoCompilationUnit::new(
            module_name.to_string(),
            PathBuf::from(format!("{}.bc", module_name))
        );
        
        for func in functions {
            unit.exported_functions.insert(func.to_string());
        }
        
        for global in globals {
            unit.exported_globals.insert(global.to_string());
        }
        
        unit.size_estimate = match module_name {
            "main" => 500,
            "core" => 800,
            "api" => 600,
            "utils" => 300,
            _ => 400,
        };
        
        optimizer.add_compilation_unit(unit);
    }
    
    // Run comprehensive optimization
    let result = optimizer.optimize();
    assert!(result.is_ok());
    
    let result = result.unwrap();
    let stats = result.statistics;
    
    // Verify comprehensive optimization results
    assert_eq!(stats.modules_processed, 4);
    assert!(stats.total_time > Duration::from_secs(0));
    
    // Should have found optimization opportunities
    let total_optimizations = stats.functions_inlined + stats.dead_functions_eliminated + 
                            stats.globals_optimized + stats.constants_propagated;
    assert!(total_optimizations > 5); // Expect significant optimizations
    
    // Check analysis results if available
    if let Some(analysis) = result.analysis {
        // Verify call graph was built
        assert!(!analysis.call_graph.calls.is_empty() || !analysis.call_graph.callers.is_empty());
        
        // Verify optimization opportunities were found
        assert!(analysis.constant_propagation_opportunities.len() > 0 ||
                analysis.dead_code_candidates.len() > 0 ||
                analysis.inlining_opportunities.len() > 0);
    }
    
    // Generate and verify report
    let report = optimizer.generate_report();
    assert!(result.is_ok());
    
    let report_content = report.unwrap();
    assert!(report_content.contains("full"));
    assert!(report_content.contains("4")); // 4 modules
}

/// Performance test for LTO with many modules
#[test]
fn test_lto_performance_many_modules() {
    init_tracing!();
    
    let config = LtoConfig {
        level: LtoLevel::Thin,
        max_worker_threads: 4,
        thin_lto_partition_threshold: 1000,
        enable_profiling: true,
        ..Default::default()
    };
    let mut optimizer = LtoOptimizer::new(config).unwrap();
    
    // Create many small modules
    let module_count = 20;
    for i in 0..module_count {
        let mut unit = LtoCompilationUnit::new(
            format!("perf_module_{:03}", i),
            PathBuf::from(format!("perf_module_{:03}.bc", i))
        );
        
        // Add several functions per module
        for j in 0..5 {
            unit.exported_functions.insert(format!("module_{}_func_{}", i, j));
        }
        
        // Add globals
        unit.exported_globals.insert(format!("module_{}_global", i));
        
        unit.size_estimate = 200 + (i % 5) * 100;
        optimizer.add_compilation_unit(unit);
    }
    
    // Measure optimization time
    let start = std::time::Instant::now();
    let result = optimizer.optimize();
    let duration = start.elapsed();
    
    assert!(result.is_ok());
    
    let result = result.unwrap();
    let stats = result.statistics;
    
    // Verify all modules were processed
    assert_eq!(stats.modules_processed, module_count);
    
    // Performance should be reasonable (less than 5 seconds for 20 modules)
    assert!(duration < Duration::from_secs(5));
    assert!(stats.total_time < Duration::from_secs(5));
    
    info!("Processed {} modules in {:?} (total: {:?})", 
          module_count, duration, stats.total_time);
}
