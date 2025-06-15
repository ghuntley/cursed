/// Comprehensive Optimization Performance Tests
/// 
/// Tests the performance improvements and functionality of the CURSED
/// compiler optimization system, including pass execution, pipeline coordination,
/// and performance monitoring.

use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing_test::traced_test;

use cursed::codegen::llvm::optimization_engine::{
    OptimizationEngine, OptimizationEngineConfig, OptimizationResult
};
use cursed::codegen::llvm::optimization_passes::{
    PassRegistry, PassConfiguration, PassResult
};
use cursed::codegen::llvm::performance_monitor::{
    PerformanceMonitor, MonitoringConfig, CodeMetrics
};
use cursed::codegen::llvm::optimization::OptimizationLevel;
use cursed::build_system::performance_tracker::{
    BuildPerformanceTracker, PerformanceConfig
};

use inkwell::context::Context;
use inkwell::module::Module;

/// Test optimization engine creation and configuration
#[traced_test]
#[test]
fn test_optimization_engine_creation() {
    let context = Context::create();
    let config = OptimizationEngineConfig::default();
    
    let result = OptimizationEngine::new(&context, config);
    assert!(result.is_ok());
    
    let engine = result.unwrap();
    assert_eq!(engine.get_statistics().total_optimizations, 0);
}

/// Test optimization engine with different optimization levels
#[traced_test]
#[test]
fn test_optimization_levels() {
    let context = Context::create();
    
    let levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Less,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
        OptimizationLevel::Size,
        OptimizationLevel::SizeAggressive,
    ];
    
    for level in levels {
        let config = OptimizationEngineConfig {
            optimization_level: level,
            ..Default::default()
        };
        
        let result = OptimizationEngine::new(&context, config);
        assert!(result.is_ok(), "Failed to create engine with level: {:?}", level);
    }
}

/// Test pass registry creation and pass registration
#[traced_test]
#[test]
fn test_pass_registry() {
    let config = PassConfiguration::default();
    let registry = PassRegistry::create_default_registry(config);
    
    let pass_names = registry.get_pass_names();
    assert!(!pass_names.is_empty(), "Registry should have default passes");
    
    // Verify expected passes are present
    let expected_passes = vec![
        "dead_code_elimination",
        "constant_propagation", 
        "function_inlining",
        "loop_optimization",
        "control_flow_optimization",
    ];
    
    for expected in expected_passes {
        assert!(
            pass_names.contains(&expected.to_string()),
            "Expected pass '{}' not found in registry", 
            expected
        );
    }
}

/// Test pass execution with different optimization levels
#[traced_test]
#[test]
fn test_pass_execution_by_level() {
    let config = PassConfiguration::default();
    let registry = PassRegistry::create_default_registry(config);
    
    let levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Less,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
    ];
    
    for level in levels {
        let passes = registry.get_passes_for_level(level);
        
        match level {
            OptimizationLevel::None => {
                assert_eq!(passes.len(), 0, "O0 should have no passes");
            }
            OptimizationLevel::Less => {
                assert!(passes.len() >= 2, "O1 should have basic passes");
            }
            OptimizationLevel::Default => {
                assert!(passes.len() >= 5, "O2 should have standard passes");
            }
            OptimizationLevel::Aggressive => {
                assert!(passes.len() >= 7, "O3 should have all passes");
            }
            _ => {}
        }
    }
}

/// Test performance monitoring functionality
#[traced_test]
#[test] 
fn test_performance_monitoring() {
    let config = MonitoringConfig::default();
    let monitor = PerformanceMonitor::new(config);
    
    // Create mock pass result
    let pass_result = PassResult {
        changed: true,
        instructions_eliminated: 50,
        functions_inlined: 5,
        constants_folded: 20,
        execution_time: Duration::from_millis(100),
        memory_usage: 1024,
        ..Default::default()
    };
    
    let before_metrics = CodeMetrics {
        function_count: 10,
        instruction_count: 1000,
        code_size: 50000,
        ..Default::default()
    };
    
    let after_metrics = CodeMetrics {
        function_count: 10,
        instruction_count: 950,
        code_size: 48000,
        ..Default::default()
    };
    
    // Record pass execution
    let result = monitor.record_pass_execution(
        "test_pass",
        &pass_result,
        &before_metrics,
        &after_metrics,
    );
    
    assert!(result.is_ok());
    
    // Generate report
    let report = monitor.generate_report();
    assert!(report.is_ok());
    
    let report = report.unwrap();
    assert!(report.pass_analyses.contains_key("test_pass"));
}

/// Test optimization effectiveness calculation
#[traced_test]
#[test]
fn test_optimization_effectiveness() {
    let config = MonitoringConfig::default();
    let monitor = PerformanceMonitor::new(config);
    
    // Test high-effectiveness optimization
    let high_effect_result = PassResult {
        changed: true,
        instructions_eliminated: 200,
        functions_inlined: 10,
        constants_folded: 50,
        execution_time: Duration::from_millis(50),
        ..Default::default()
    };
    
    let before_metrics = CodeMetrics {
        code_size: 100000,
        instruction_count: 2000,
        ..Default::default()
    };
    
    let after_metrics = CodeMetrics {
        code_size: 80000,
        instruction_count: 1800,
        ..Default::default()
    };
    
    monitor.record_pass_execution(
        "high_effect_pass",
        &high_effect_result,
        &before_metrics,
        &after_metrics,
    ).unwrap();
    
    // Test low-effectiveness optimization
    let low_effect_result = PassResult {
        changed: true,
        instructions_eliminated: 5,
        execution_time: Duration::from_millis(1000), // High time, low benefit
        ..Default::default()
    };
    
    monitor.record_pass_execution(
        "low_effect_pass", 
        &low_effect_result,
        &before_metrics,
        &after_metrics,
    ).unwrap();
    
    let report = monitor.generate_report().unwrap();
    
    // High effectiveness pass should have better score
    let high_analysis = &report.pass_analyses["high_effect_pass"];
    let low_analysis = &report.pass_analyses["low_effect_pass"];
    
    assert!(
        high_analysis.average_effectiveness > low_analysis.average_effectiveness,
        "High effectiveness pass should score better"
    );
}

/// Test build performance tracking
#[traced_test]
#[test]
fn test_build_performance_tracking() {
    let config = PerformanceConfig::default();
    let tracker = BuildPerformanceTracker::new(config);
    
    // Start build tracking
    tracker.start_build("test_build".to_string(), "debug".to_string()).unwrap();
    
    // Track some phases
    tracker.start_phase("parsing".to_string()).unwrap();
    std::thread::sleep(Duration::from_millis(10));
    tracker.end_phase("parsing".to_string()).unwrap();
    
    tracker.start_phase("type_checking".to_string()).unwrap();
    std::thread::sleep(Duration::from_millis(20));
    tracker.end_phase("type_checking".to_string()).unwrap();
    
    // Record file compilations
    tracker.record_file_compilation(
        "test.csd".into(),
        Duration::from_millis(50),
        100,
        50.0,
        true,
    ).unwrap();
    
    tracker.record_file_compilation(
        "main.csd".into(),
        Duration::from_millis(30),
        200,
        75.0,
        true,
    ).unwrap();
    
    // End build tracking
    let report = tracker.end_build().unwrap();
    
    assert!(report.build_record.success);
    assert_eq!(report.build_record.files_compiled, 2);
    assert_eq!(report.build_record.lines_compiled, 300);
    assert!(report.build_record.total_duration > Duration::from_millis(30));
    
    // Check phase analysis
    assert!(!report.phase_analysis.phase_summary.is_empty());
}

/// Performance benchmark test
#[traced_test]
#[test]
fn test_optimization_performance_benchmark() {
    let context = Context::create();
    
    // Create a simple module for testing
    let module = context.create_module("test_module");
    
    // Add a simple function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("test_function", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Add some instructions
    let param = function.get_first_param().unwrap().into_int_value();
    let const_one = i32_type.const_int(1, false);
    let result = builder.build_int_add(param, const_one, "add_result").unwrap();
    builder.build_return(Some(&result)).unwrap();
    
    // Test different optimization levels
    let levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Less,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
    ];
    
    let mut benchmark_results = HashMap::new();
    
    for level in levels {
        let config = OptimizationEngineConfig {
            optimization_level: level,
            enable_performance_monitoring: true,
            ..Default::default()
        };
        
        let mut engine = OptimizationEngine::new(&context, config).unwrap();
        
        // Benchmark optimization time
        let start = Instant::now();
        let result = engine.optimize_module(&module);
        let duration = start.elapsed();
        
        assert!(result.is_ok(), "Optimization failed for level: {:?}", level);
        
        let optimization_result = result.unwrap();
        benchmark_results.insert(level, (duration, optimization_result));
        
        println!("Optimization level {:?}: {:?}", level, duration);
    }
    
    // Verify that higher optimization levels take more time (generally)
    let o0_time = benchmark_results[&OptimizationLevel::None].0;
    let o3_time = benchmark_results[&OptimizationLevel::Aggressive].0;
    
    // O3 should generally take longer than O0 (unless the module is too simple)
    // This is just a sanity check - in practice O0 might be fastest for trivial cases
    assert!(o0_time <= o3_time + Duration::from_millis(100));
}

/// Test performance regression detection
#[traced_test]
#[test]
fn test_performance_regression_detection() {
    let config = PerformanceConfig::default();
    let tracker = BuildPerformanceTracker::new(config);
    
    // Simulate multiple builds with increasing build times
    for i in 1..=5 {
        tracker.start_build(format!("build_{}", i), "debug".to_string()).unwrap();
        
        // Simulate progressively slower builds
        std::thread::sleep(Duration::from_millis(i * 10));
        
        tracker.record_file_compilation(
            "test.csd".into(),
            Duration::from_millis(i * 5),
            100,
            50.0,
            true,
        ).unwrap();
        
        let report = tracker.end_build().unwrap();
        
        if i >= 3 {
            // Later builds should show some trend
            let comparison = &report.historical_comparison;
            if i == 5 {
                // Fifth build should show degradation
                assert!(comparison.time_change_percentage > 0.0);
            }
        }
    }
    
    // Get final statistics
    let stats = tracker.get_performance_statistics().unwrap();
    assert_eq!(stats.total_builds, 5);
    assert!(stats.average_build_time > Duration::from_millis(20));
}

/// Test optimization suggestions generation
#[traced_test]
#[test]
fn test_optimization_suggestions() {
    let config = PerformanceConfig::default();
    let tracker = BuildPerformanceTracker::new(config);
    
    // Simulate a build with poor cache performance
    tracker.start_build("slow_build".to_string(), "debug".to_string()).unwrap();
    
    // Record many cache misses
    for _ in 0..20 {
        tracker.record_cache_event(false).unwrap(); // Cache miss
    }
    for _ in 0..5 {
        tracker.record_cache_event(true).unwrap(); // Cache hit
    }
    
    // Record many file compilations (suggesting parallelization benefits)
    for i in 0..50 {
        tracker.record_file_compilation(
            format!("file_{}.csd", i).into(),
            Duration::from_millis(100),
            100,
            50.0,
            true,
        ).unwrap();
    }
    
    let report = tracker.end_build().unwrap();
    
    // Should suggest cache and parallelization improvements
    assert!(!report.optimization_suggestions.is_empty());
    
    let suggestions: Vec<_> = report.optimization_suggestions.iter()
        .map(|s| s.category.as_str())
        .collect();
    
    assert!(suggestions.contains(&"Caching") || suggestions.contains(&"Parallelization"));
}

/// Test memory and resource tracking
#[traced_test]
#[test]
fn test_resource_tracking() {
    let config = PerformanceConfig {
        enable_resource_monitoring: true,
        ..Default::default()
    };
    let tracker = BuildPerformanceTracker::new(config);
    
    tracker.start_build("resource_test".to_string(), "debug".to_string()).unwrap();
    
    // Simulate resource-intensive operations
    tracker.start_phase("intensive_phase".to_string()).unwrap();
    
    // Simulate some work
    std::thread::sleep(Duration::from_millis(100));
    
    tracker.end_phase("intensive_phase".to_string()).unwrap();
    
    let report = tracker.end_build().unwrap();
    
    // Resource statistics should be populated
    assert!(report.resource_statistics.peak_memory_mb > 0.0);
    assert!(report.resource_statistics.average_cpu_percent >= 0.0);
}

/// Integration test with real optimization pipeline
#[traced_test]
#[test]
fn test_optimization_pipeline_integration() {
    let context = Context::create();
    let module = create_test_module(&context);
    
    let config = OptimizationEngineConfig {
        optimization_level: OptimizationLevel::Default,
        enable_performance_monitoring: true,
        enable_performance_analysis: true,
        ..Default::default()
    };
    
    let mut engine = OptimizationEngine::new(&context, config).unwrap();
    
    // Set baseline for comparison
    engine.set_baseline_metrics(&module);
    
    // Run optimization
    let result = engine.optimize_module(&module);
    assert!(result.is_ok());
    
    let optimization_result = result.unwrap();
    assert!(optimization_result.success);
    
    // Check that some passes were applied
    assert!(!optimization_result.passes_applied.is_empty());
    
    // Get performance report
    let performance_report = engine.get_performance_report();
    assert!(performance_report.is_ok());
    
    let report = performance_report.unwrap();
    assert!(!report.pass_analyses.is_empty());
}

/// Helper function to create a test module with some complexity
fn create_test_module(context: &Context) -> Module {
    let module = context.create_module("test_optimization_module");
    
    // Create some functions with opportunities for optimization
    let i32_type = context.i32_type();
    let builder = context.create_builder();
    
    // Function 1: Simple arithmetic with constants
    let fn1_type = i32_type.fn_type(&[], false);
    let function1 = module.add_function("arithmetic_function", fn1_type, None);
    let bb1 = context.append_basic_block(function1, "entry");
    builder.position_at_end(bb1);
    
    let const_5 = i32_type.const_int(5, false);
    let const_10 = i32_type.const_int(10, false);
    let add_result = builder.build_int_add(const_5, const_10, "add").unwrap();
    let mul_result = builder.build_int_mul(add_result, const_5, "mul").unwrap();
    builder.build_return(Some(&mul_result)).unwrap();
    
    // Function 2: Function that could be inlined
    let fn2_type = i32_type.fn_type(&[i32_type.into()], false);
    let function2 = module.add_function("small_function", fn2_type, None);
    let bb2 = context.append_basic_block(function2, "entry");
    builder.position_at_end(bb2);
    
    let param = function2.get_first_param().unwrap().into_int_value();
    let const_1 = i32_type.const_int(1, false);
    let result = builder.build_int_add(param, const_1, "increment").unwrap();
    builder.build_return(Some(&result)).unwrap();
    
    // Function 3: Function that calls the small function (inlining opportunity)
    let fn3_type = i32_type.fn_type(&[i32_type.into()], false);
    let function3 = module.add_function("calling_function", fn3_type, None);
    let bb3 = context.append_basic_block(function3, "entry");
    builder.position_at_end(bb3);
    
    let param3 = function3.get_first_param().unwrap();
    let call_result = builder.build_call(function2, &[param3], "call_small").unwrap();
    let return_value = call_result.try_as_basic_value().left().unwrap().into_int_value();
    builder.build_return(Some(&return_value)).unwrap();
    
    module
}

/// Benchmark test comparing optimization levels
#[traced_test]
#[test]
fn test_optimization_level_comparison() {
    let context = Context::create();
    let module = create_complex_test_module(&context);
    
    let levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Less,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
    ];
    
    let mut results = HashMap::new();
    
    for level in levels {
        let config = OptimizationEngineConfig {
            optimization_level: level,
            enable_performance_monitoring: true,
            ..Default::default()
        };
        
        let mut engine = OptimizationEngine::new(&context, config).unwrap();
        
        let start = Instant::now();
        let result = engine.optimize_module(&module).unwrap();
        let duration = start.elapsed();
        
        results.insert(level, (duration, result));
    }
    
    // O0 should have no passes applied
    assert_eq!(results[&OptimizationLevel::None].1.passes_applied.len(), 0);
    
    // Higher optimization levels should apply more passes
    assert!(
        results[&OptimizationLevel::Aggressive].1.passes_applied.len() >= 
        results[&OptimizationLevel::Less].1.passes_applied.len()
    );
    
    // Print benchmark results
    for (level, (duration, result)) in &results {
        println!("Level {:?}: {:?}, {} passes, {:.2}% improvement", 
                level, 
                duration,
                result.passes_applied.len(),
                result.performance_improvement);
    }
}

/// Create a more complex test module for benchmarking
fn create_complex_test_module(context: &Context) -> Module {
    let module = context.create_module("complex_test_module");
    let i32_type = context.i32_type();
    let builder = context.create_builder();
    
    // Create multiple functions with various optimization opportunities
    for i in 0..10 {
        let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
        let function = module.add_function(&format!("function_{}", i), fn_type, None);
        let bb = context.append_basic_block(function, "entry");
        builder.position_at_end(bb);
        
        let param1 = function.get_nth_param(0).unwrap().into_int_value();
        let param2 = function.get_nth_param(1).unwrap().into_int_value();
        
        // Create some computations with constants (constant folding opportunities)
        let const_val = i32_type.const_int(i as u64, false);
        let add1 = builder.build_int_add(param1, const_val, "add1").unwrap();
        let mul1 = builder.build_int_mul(add1, param2, "mul1").unwrap();
        
        // Add some dead code (dead code elimination opportunities)
        let dead_add = builder.build_int_add(const_val, const_val, "dead").unwrap();
        
        // Use only the live computation
        builder.build_return(Some(&mul1)).unwrap();
    }
    
    module
}
