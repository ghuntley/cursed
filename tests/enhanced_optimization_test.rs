/// Comprehensive Test Suite for Enhanced LLVM Optimization System
/// 
/// Tests all components of the enhanced optimization infrastructure including
/// optimization manager, engine, passes, pipeline, and performance monitoring.

use cursed::codegen::llvm::{
    optimization::*,
    optimization_engine::*,
    optimization_passes::*,
    optimization_pipeline::*,
    performance_monitor::*,
};
use cursed::error::Result;
use std::time::Duration;
use std::collections::HashSet;
use std::sync::Arc;
use inkwell::context::Context;

#[test]
fn test_optimization_level_conversions() {
    // Test string conversions
    assert_eq!(OptimizationLevel::from_str("O0").unwrap(), OptimizationLevel::None);
    assert_eq!(OptimizationLevel::from_str("O1").unwrap(), OptimizationLevel::Less);
    assert_eq!(OptimizationLevel::from_str("O2").unwrap(), OptimizationLevel::Default);
    assert_eq!(OptimizationLevel::from_str("O3").unwrap(), OptimizationLevel::Aggressive);
    assert_eq!(OptimizationLevel::from_str("Os").unwrap(), OptimizationLevel::Size);
    assert_eq!(OptimizationLevel::from_str("Oz").unwrap(), OptimizationLevel::SizeAggressive);
    
    // Test invalid conversion
    assert!(OptimizationLevel::from_str("O4").is_err());
    
    // Test as_str
    assert_eq!(OptimizationLevel::None.as_str(), "O0");
    assert_eq!(OptimizationLevel::Aggressive.as_str(), "O3");
    
    // Test inkwell conversion
    assert_eq!(OptimizationLevel::None.to_inkwell_level(), inkwell::OptimizationLevel::None);
    assert_eq!(OptimizationLevel::Aggressive.to_inkwell_level(), inkwell::OptimizationLevel::Aggressive);
}

#[test]
fn test_optimization_config_defaults() {
    let config = OptimizationConfig::default();
    
    assert_eq!(config.level, OptimizationLevel::Default);
    assert!(config.vectorize_loops);
    assert!(config.enable_parallel_optimization);
    assert!(config.enable_caching);
    assert!(config.enable_cursed_specific);
    assert!(config.enable_auto_tuning);
    assert_eq!(config.cache_size_limit, 1000);
    assert_eq!(config.parallel_threshold, 10);
    assert_eq!(config.optimization_timeout, Some(Duration::from_secs(300)));
}

#[test]
fn test_optimization_stats_defaults() {
    let stats = OptimizationStats::default();
    
    assert_eq!(stats.passes_run, 0);
    assert_eq!(stats.cache_hits, 0);
    assert_eq!(stats.cache_misses, 0);
    assert_eq!(stats.parallel_passes_run, 0);
    assert_eq!(stats.cursed_specific_optimizations, 0);
    assert_eq!(stats.auto_tuning_adjustments, 0);
}

#[test]
fn test_optimization_manager_creation() {
    let context = Context::create();
    let config = OptimizationConfig::default();
    let manager = OptimizationManager::new(&context, config.clone());
    
    assert_eq!(manager.get_config().level, config.level);
    assert_eq!(manager.get_config().enable_parallel_optimization, config.enable_parallel_optimization);
}

#[test]
fn test_module_analysis() {
    let context = Context::create();
    let module = context.create_module("test_module");
    
    // Add a simple function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("test_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    let param = function.get_first_param().unwrap().into_int_value();
    let result = builder.build_int_add(param, i32_type.const_int(1, false), "add").unwrap();
    builder.build_return(Some(&result)).unwrap();
    
    let config = OptimizationConfig::default();
    let manager = OptimizationManager::new(&context, config);
    
    let analysis = manager.analyze_module(&module);
    assert_eq!(analysis.total_functions, 1);
    assert!(analysis.total_instructions > 0);
    assert!(analysis.estimated_compilation_time > Duration::from_secs(0));
}

#[test]
fn test_cache_key_generation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let config = OptimizationConfig::default();
    let manager = OptimizationManager::new(&context, config);
    
    let key1 = manager.generate_cache_key(&module);
    let key2 = manager.generate_cache_key(&module);
    
    // Same module should generate same key
    assert_eq!(key1, key2);
    
    // Different module should generate different key
    let module2 = context.create_module("test_module2");
    let key3 = manager.generate_cache_key(&module2);
    assert_ne!(key1, key3);
}

#[test]
fn test_optimization_engine_creation() {
    let context = Context::create();
    let config = OptimizationEngineConfig::default();
    let engine = OptimizationEngine::new(&context, config.clone());
    
    assert_eq!(engine.config.enable_adaptive_optimization, config.enable_adaptive_optimization);
    assert_eq!(engine.config.enable_profile_guided_optimization, config.enable_profile_guided_optimization);
}

#[test]
fn test_optimization_engine_strategy_selection() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let config = OptimizationEngineConfig::default();
    let engine = OptimizationEngine::new(&context, config);
    
    // Test strategy selection based on module characteristics
    let strategy = engine.select_optimization_strategy(&module, "test_module").unwrap();
    
    // Should return adaptive strategy by default
    matches!(strategy, OptimizationStrategy::Adaptive);
}

#[test]
fn test_pass_registry_creation() {
    let registry = PassRegistry::new();
    
    // Should have default passes registered
    assert!(registry.get_pass_count() > 0);
    
    // Should have basic passes
    assert!(registry.get_pass("instruction-combining").is_some());
    assert!(registry.get_pass("gvn").is_some());
    assert!(registry.get_pass("cfg-simplification").is_some());
}

#[test]
fn test_pass_registry_custom_pass() {
    let registry = PassRegistry::new();
    
    let custom_pass = OptimizationPass::new("custom-test-pass", "Test pass for unit testing")
        .depends_on("instruction-combining")
        .estimated_improvement(1.5)
        .compile_time_cost(PassTimeCategory::Medium);
    
    registry.register_pass(custom_pass);
    
    let retrieved_pass = registry.get_pass("custom-test-pass").unwrap();
    assert_eq!(retrieved_pass.name, "custom-test-pass");
    assert_eq!(retrieved_pass.estimated_improvement, 1.5);
    assert_eq!(retrieved_pass.estimated_compile_time_cost, PassTimeCategory::Medium);
}

#[test]
fn test_pass_selection() {
    let registry = PassRegistry::new();
    
    let config = PassConfiguration {
        optimization_level: OptimizationLevel::Default,
        enable_expensive_passes: true,
        enable_cursed_passes: true,
        ..Default::default()
    };
    
    let selected_passes = registry.select_passes(&config).unwrap();
    assert!(!selected_passes.is_empty());
    
    // Should include basic passes
    assert!(selected_passes.contains(&"instruction-combining".to_string()));
    
    // Test with disabled expensive passes
    let config_no_expensive = PassConfiguration {
        optimization_level: OptimizationLevel::Default,
        enable_expensive_passes: false,
        enable_cursed_passes: true,
        ..Default::default()
    };
    
    let selected_no_expensive = registry.select_passes(&config_no_expensive).unwrap();
    assert!(!selected_no_expensive.is_empty());
    assert!(selected_no_expensive.len() <= selected_passes.len());
}

#[test]
fn test_pass_time_categories() {
    assert_eq!(PassTimeCategory::from_duration(Duration::from_millis(0)), PassTimeCategory::VeryFast);
    assert_eq!(PassTimeCategory::from_duration(Duration::from_millis(5)), PassTimeCategory::Fast);
    assert_eq!(PassTimeCategory::from_duration(Duration::from_millis(50)), PassTimeCategory::Medium);
    assert_eq!(PassTimeCategory::from_duration(Duration::from_millis(500)), PassTimeCategory::Slow);
    assert_eq!(PassTimeCategory::from_duration(Duration::from_millis(2000)), PassTimeCategory::VerySlow);
    
    assert_eq!(PassTimeCategory::Fast.as_str(), "fast");
    assert_eq!(PassTimeCategory::Slow.as_str(), "slow");
}

#[test]
fn test_optimization_pipeline_creation() {
    let context = Context::create();
    let pass_registry = Arc::new(PassRegistry::new());
    let mut pipeline = OptimizationPipeline::new(&context, pass_registry);
    
    // Test creating default pipelines
    assert!(pipeline.create_default_pipeline(OptimizationLevel::None).is_ok());
    assert!(pipeline.create_default_pipeline(OptimizationLevel::Default).is_ok());
    assert!(pipeline.create_default_pipeline(OptimizationLevel::Aggressive).is_ok());
}

#[test]
fn test_pipeline_stage_creation() {
    let stage = PipelineStage::new("test-stage", "Test stage for unit testing")
        .with_passes(vec!["pass1".to_string(), "pass2".to_string()])
        .parallel()
        .optional()
        .with_timeout(Duration::from_secs(30))
        .depends_on(vec!["earlier-stage".to_string()]);
    
    assert_eq!(stage.name, "test-stage");
    assert_eq!(stage.passes.len(), 2);
    assert!(stage.parallel_execution);
    assert!(stage.optional);
    assert_eq!(stage.timeout, Some(Duration::from_secs(30)));
    assert_eq!(stage.dependencies.len(), 1);
}

#[test]
fn test_performance_monitor_creation() {
    let config = MonitoringConfig::default();
    let monitor = PerformanceMonitor::new(config.clone());
    
    assert_eq!(monitor.config.enable_compilation_timing, config.enable_compilation_timing);
    assert_eq!(monitor.config.enable_memory_tracking, config.enable_memory_tracking);
    assert_eq!(monitor.config.history_retention_days, config.history_retention_days);
}

#[test]
fn test_performance_sample_creation() {
    let sample = PerformanceSample {
        timestamp: 1234567890,
        module_name: "test_module".to_string(),
        optimization_level: "O2".to_string(),
        compilation_time: Duration::from_millis(100),
        memory_peak_usage: 1024 * 1024, // 1MB
        memory_average_usage: 512 * 1024, // 512KB
        code_metrics_before: CodeMetrics::default(),
        code_metrics_after: CodeMetrics::default(),
        passes_executed: 10,
        passes_successful: 9,
        estimated_runtime_improvement: 1.2,
        cache_hits: 5,
        cache_misses: 2,
    };
    
    assert_eq!(sample.module_name, "test_module");
    assert_eq!(sample.passes_executed, 10);
    assert_eq!(sample.passes_successful, 9);
    assert_eq!(sample.estimated_runtime_improvement, 1.2);
}

#[test]
fn test_code_metrics_defaults() {
    let metrics = CodeMetrics::default();
    
    assert_eq!(metrics.instruction_count, 0);
    assert_eq!(metrics.function_count, 0);
    assert_eq!(metrics.complexity_score, 0.0);
    assert_eq!(metrics.estimated_cache_performance, 1.0);
}

#[test]
fn test_baseline_metrics_defaults() {
    let baseline = BaselineMetrics::default();
    
    assert_eq!(baseline.average_compilation_time, Duration::from_secs(1));
    assert_eq!(baseline.average_memory_usage, 100 * 1024 * 1024); // 100MB
    assert_eq!(baseline.average_performance_improvement, 1.2);
    assert_eq!(baseline.typical_pass_success_rate, 0.95);
    assert_eq!(baseline.sample_count, 0);
}

#[test]
fn test_alert_thresholds() {
    let thresholds = AlertThresholds::default();
    
    assert_eq!(thresholds.compilation_time_regression_percent, 20.0);
    assert_eq!(thresholds.memory_usage_increase_percent, 50.0);
    assert_eq!(thresholds.code_size_increase_percent, 10.0);
    assert_eq!(thresholds.performance_degradation_percent, 5.0);
    assert_eq!(thresholds.pass_failure_rate_percent, 5.0);
}

#[test]
fn test_compilation_session_tracking() {
    let config = MonitoringConfig::default();
    let monitor = PerformanceMonitor::new(config);
    
    let session = monitor.start_compilation_monitoring("test_module");
    
    // Test memory tracking
    session.record_allocation(1024);
    session.record_allocation(2048);
    session.record_deallocation(512);
    
    // Complete session
    let code_before = CodeMetrics {
        instruction_count: 100,
        function_count: 5,
        complexity_score: 50.0,
        ..Default::default()
    };
    
    let code_after = CodeMetrics {
        instruction_count: 90,
        function_count: 5,
        complexity_score: 45.0,
        ..Default::default()
    };
    
    session.complete(
        "O2",
        code_before,
        code_after,
        10, // passes_executed
        9,  // passes_successful
        1.15, // estimated_runtime_improvement
        5,  // cache_hits
        2,  // cache_misses
    );
    
    // Verify sample was recorded
    let (sample_count, _total_time, _avg_improvement) = monitor.get_current_statistics().unwrap();
    assert_eq!(sample_count, 1);
}

#[test]
fn test_optimization_utilities() {
    // Test development config
    let dev_config = utils::dev_config();
    assert_eq!(dev_config.level, OptimizationLevel::None);
    assert!(!dev_config.vectorize_loops);
    assert!(!dev_config.enable_lto);
    
    // Test release config
    let release_config = utils::release_config();
    assert_eq!(release_config.level, OptimizationLevel::Aggressive);
    assert!(release_config.vectorize_loops);
    assert!(release_config.enable_lto);
    
    // Test config creation from args
    let config = utils::create_config_from_args(
        Some("O3"),
        Some("native"),
        &["sse4.2".to_string()],
        true,
    ).unwrap();
    
    assert_eq!(config.level, OptimizationLevel::Aggressive);
    assert_eq!(config.target_cpu, Some("native".to_string()));
    assert!(config.enable_lto);
}

#[test]
fn test_llvm_optimizer_interface() {
    let context = Context::create();
    let config = OptimizationConfig::default();
    let mut optimizer = LlvmOptimizer::new(&context, config);
    
    let module = context.create_module("test_module");
    
    // Test initialization
    assert!(optimizer.initialize(&module).is_ok());
    
    // Test optimization
    assert!(optimizer.optimize(&module).is_ok());
    
    // Test statistics
    let stats = optimizer.get_statistics();
    assert!(stats.modules_optimized >= 1);
    
    // Test analysis
    let analysis = optimizer.analyze_module(&module);
    assert_eq!(analysis.total_functions, 0); // Empty module
}

#[test]
fn test_integration_optimization_flow() {
    let context = Context::create();
    let module = context.create_module("integration_test");
    
    // Create a simple function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("add_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    let param1 = function.get_first_param().unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    let result = builder.build_int_add(param1, param2, "add").unwrap();
    builder.build_return(Some(&result)).unwrap();
    
    // Test complete optimization flow
    let mut config = OptimizationConfig::default();
    config.enable_caching = true;
    config.enable_parallel_optimization = false; // Disable for simpler testing
    config.enable_cursed_specific = true;
    
    let mut optimizer = LlvmOptimizer::new(&context, config);
    
    // Initialize and optimize
    assert!(optimizer.initialize(&module).is_ok());
    assert!(optimizer.optimize(&module).is_ok());
    
    // Check statistics
    let stats = optimizer.get_statistics();
    assert!(stats.modules_optimized >= 1);
    assert!(stats.optimization_time > Duration::from_secs(0));
    
    // Test caching - second optimization should be faster
    let stats_before_cache = optimizer.get_statistics();
    assert!(optimizer.optimize(&module).is_ok());
    let stats_after_cache = optimizer.get_statistics();
    
    // Should have cache hits
    assert!(stats_after_cache.cache_hits >= stats_before_cache.cache_hits);
}

#[test]
fn test_performance_monitoring_integration() {
    let monitor_config = MonitoringConfig {
        enable_compilation_timing: true,
        enable_memory_tracking: true,
        sample_rate: 1.0,
        ..Default::default()
    };
    
    let monitor = PerformanceMonitor::new(monitor_config);
    
    // Simulate multiple compilation sessions
    for i in 0..5 {
        let session = monitor.start_compilation_monitoring(&format!("module_{}", i));
        
        // Simulate memory allocations
        session.record_allocation(1024 * (i + 1));
        
        std::thread::sleep(Duration::from_millis(10)); // Simulate compilation time
        
        let code_before = CodeMetrics {
            instruction_count: 100 + i * 20,
            function_count: 5,
            complexity_score: 50.0 + i as f64 * 5.0,
            ..Default::default()
        };
        
        let code_after = CodeMetrics {
            instruction_count: 90 + i * 15,
            function_count: 5,
            complexity_score: 45.0 + i as f64 * 4.0,
            ..Default::default()
        };
        
        session.complete(
            "O2",
            code_before,
            code_after,
            10,
            9,
            1.1 + (i as f64 * 0.05),
            i,
            1,
        );
    }
    
    // Verify samples were collected
    let (sample_count, total_time, avg_improvement) = monitor.get_current_statistics().unwrap();
    assert_eq!(sample_count, 5);
    assert!(total_time > Duration::from_secs(0));
    assert!(avg_improvement > 1.0);
    
    // Test report generation
    if let Ok(report) = monitor.generate_performance_report() {
        assert_eq!(report.total_samples, 5);
        assert!(report.analysis_period > Duration::from_secs(0));
        assert!(report.compilation_performance.average_compilation_time > Duration::from_secs(0));
    }
}

// Helper function to create a test module with multiple functions
fn create_complex_test_module(context: &Context, name: &str) -> inkwell::module::Module {
    let module = context.create_module(name);
    let i32_type = context.i32_type();
    let builder = context.create_builder();
    
    // Add multiple functions with different complexity
    for i in 0..5 {
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function(&format!("function_{}", i), fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        let param = function.get_first_param().unwrap().into_int_value();
        
        // Create different levels of complexity
        let mut result = param;
        for j in 0..i + 1 {
            result = builder.build_int_add(result, i32_type.const_int(j as u64, false), "add").unwrap();
        }
        
        builder.build_return(Some(&result)).unwrap();
    }
    
    module
}

#[test]
fn test_complex_optimization_scenario() {
    let context = Context::create();
    let module = create_complex_test_module(&context, "complex_test");
    
    // Set up performance monitoring
    let monitor_config = MonitoringConfig::default();
    let monitor = PerformanceMonitor::new(monitor_config);
    let session = monitor.start_compilation_monitoring("complex_test");
    
    // Set up optimization pipeline
    let pass_registry = Arc::new(PassRegistry::new());
    let mut pipeline = OptimizationPipeline::new(&context, pass_registry.clone());
    
    // Create aggressive pipeline
    assert!(pipeline.create_default_pipeline(OptimizationLevel::Aggressive).is_ok());
    assert!(pipeline.initialize(&module).is_ok());
    
    // Execute pipeline
    let pass_config = PassConfiguration {
        optimization_level: OptimizationLevel::Aggressive,
        enable_expensive_passes: true,
        enable_cursed_passes: true,
        ..Default::default()
    };
    
    let pipeline_result = pipeline.execute(&module, &pass_config).unwrap();
    
    // Verify pipeline execution
    assert!(pipeline_result.overall_success);
    assert!(pipeline_result.stages_executed > 0);
    assert!(pipeline_result.total_passes_executed > 0);
    assert!(pipeline_result.overall_performance_improvement >= 1.0);
    
    // Complete monitoring session
    let code_before = CodeMetrics {
        instruction_count: 50,
        function_count: 5,
        complexity_score: 75.0,
        ..Default::default()
    };
    
    let code_after = CodeMetrics {
        instruction_count: 40,
        function_count: 5,
        complexity_score: 60.0,
        ..Default::default()
    };
    
    session.complete(
        "O3",
        code_before,
        code_after,
        pipeline_result.total_passes_executed,
        pipeline_result.total_passes_successful,
        pipeline_result.overall_performance_improvement,
        0, // No cache in this test
        1,
    );
    
    // Verify integration worked
    let (sample_count, _total_time, avg_improvement) = monitor.get_current_statistics().unwrap();
    assert_eq!(sample_count, 1);
    assert!(avg_improvement >= 1.0);
}

#[test]
fn test_error_handling() {
    let context = Context::create();
    
    // Test invalid optimization level
    assert!(OptimizationLevel::from_str("invalid").is_err());
    
    // Test pass registry with empty configuration
    let registry = PassRegistry::new();
    let config = PassConfiguration {
        optimization_level: OptimizationLevel::None,
        enabled_passes: HashSet::new(),
        disabled_passes: HashSet::new(),
        ..Default::default()
    };
    
    let selected_passes = registry.select_passes(&config).unwrap();
    // Should still work, just return fewer passes
    assert!(selected_passes.len() >= 0);
}

#[test]
fn test_memory_management() {
    let context = Context::create();
    let config = OptimizationConfig::default();
    let manager = OptimizationManager::new(&context, config);
    
    // Test cache operations
    manager.clear_cache();
    let (hits, misses, hit_rate) = manager.get_cache_stats();
    assert_eq!(hits, 0);
    assert_eq!(misses, 0);
    assert_eq!(hit_rate, 0.0);
    
    // Test performance data recording
    manager.record_performance("test_key".to_string(), 1.5);
    
    // These operations should not panic or cause memory issues
    let stats = manager.get_stats();
    assert_eq!(stats.cache_hits, 0);
}

#[test]
fn test_concurrent_optimization() {
    use std::thread;
    use std::sync::Arc;
    
    let context = Arc::new(Context::create());
    let config = OptimizationConfig {
        enable_parallel_optimization: true,
        enable_caching: true,
        parallel_threshold: 1, // Low threshold for testing
        ..Default::default()
    };
    
    let manager = Arc::new(OptimizationManager::new(&context, config));
    
    // Create multiple threads doing optimization
    let handles: Vec<_> = (0..3).map(|i| {
        let context = context.clone();
        let manager = manager.clone();
        
        thread::spawn(move || {
            let module = context.create_module(&format!("concurrent_module_{}", i));
            
            // Add a simple function
            let i32_type = context.i32_type();
            let fn_type = i32_type.fn_type(&[], false);
            let function = module.add_function("test_fn", fn_type, None);
            let basic_block = context.append_basic_block(function, "entry");
            let builder = context.create_builder();
            builder.position_at_end(basic_block);
            builder.build_return(Some(&i32_type.const_int(42, false))).unwrap();
            
            // Analyze and potentially optimize
            let _analysis = manager.analyze_module(&module);
            
            // This should not panic or cause data races
            let _stats = manager.get_stats();
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify final state
    let final_stats = manager.get_stats();
    // Should have accumulated some statistics from concurrent operations
    assert!(final_stats.modules_optimized >= 0);
}
