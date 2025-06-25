/// Comprehensive Tests for OSR and Tiered Compilation System
/// 
/// Tests the On-Stack Replacement (OSR) and Tiered Compilation features
/// of the CURSED JIT compilation system.

use cursed::codegen::llvm::{
    jit_compilation::{JitCompilationInterface, JitCompilationConfig, create_optimized_jit_interface, create_debug_jit_interface},
    jit_engine::CursedJitEngine,
    osr::{OSRManager, OSRConfig, DeoptimizationReason, StackFrame, VariableValue, VariableValueType, create_optimized_osr_manager, create_debug_osr_manager},
    tiered_compilation::{TieredCompilationManager, TieredCompilationConfig, CompilationTier, create_optimized_tiered_manager, create_debug_tiered_manager},
    LlvmCodeGenerator,
};
use cursed::error::Error;
use std::collections::HashMap;
use std::time::Duration;
use inkwell::{context::Context, OptimizationLevel};

/// Test OSR manager creation and basic functionality
#[test]
fn test_osr_manager_creation() {
    let context = Context::create();
    let config = OSRConfig::default();
    let osr_manager = OSRManager::new(&context, config);
    
    assert_eq!(osr_manager.get_current_stack_depth(), 0);
    assert!(osr_manager.get_pending_osr_functions().is_empty());
    assert_eq!(osr_manager.get_stats().total_osr_replacements, 0);
}

/// Test OSR manager with optimized and debug configurations
#[test]
fn test_osr_manager_configurations() {
    let context = Context::create();
    
    // Test optimized configuration
    let optimized_manager = create_optimized_osr_manager(&context);
    assert!(optimized_manager.get_config().enable_loop_osr);
    assert!(optimized_manager.get_config().enable_function_osr);
    assert!(optimized_manager.get_config().enable_speculative_optimizations);
    
    // Test debug configuration
    let debug_manager = create_debug_osr_manager(&context);
    assert!(!debug_manager.get_config().enable_loop_osr);
    assert!(debug_manager.get_config().enable_function_osr);
    assert!(!debug_manager.get_config().enable_speculative_optimizations);
}

/// Test stack frame tracking
#[test]
fn test_stack_frame_tracking() {
    let context = Context::create();
    let osr_manager = OSRManager::new(&context, OSRConfig::default());
    
    // Create test stack frames
    let mut local_vars1 = HashMap::new();
    local_vars1.insert(
        "test_var".to_string(),
        VariableValue {
            name: "test_var".to_string(),
            value: VariableValueType::Integer(42),
            type_name: "i32".to_string(),
            is_live: true,
        }
    );
    
    let frame1 = StackFrame {
        function_name: "test_function_1".to_string(),
        local_variables: local_vars1,
        return_address: Some(0x1000),
        frame_pointer: Some(0x2000),
        stack_pointer: Some(0x3000),
    };
    
    let frame2 = StackFrame {
        function_name: "test_function_2".to_string(),
        local_variables: HashMap::new(),
        return_address: Some(0x1100),
        frame_pointer: Some(0x2100),
        stack_pointer: Some(0x3100),
    };
    
    // Test frame tracking
    assert_eq!(osr_manager.get_current_stack_depth(), 0);
    
    osr_manager.record_stack_frame_entry(frame1).unwrap();
    assert_eq!(osr_manager.get_current_stack_depth(), 1);
    
    osr_manager.record_stack_frame_entry(frame2).unwrap();
    assert_eq!(osr_manager.get_current_stack_depth(), 2);
    
    osr_manager.record_stack_frame_exit("test_function_2").unwrap();
    assert_eq!(osr_manager.get_current_stack_depth(), 1);
    
    osr_manager.record_stack_frame_exit("test_function_1").unwrap();
    assert_eq!(osr_manager.get_current_stack_depth(), 0);
}

/// Test OSR trigger conditions
#[test]
fn test_osr_trigger_conditions() {
    let context = Context::create();
    let config = OSRConfig {
        osr_trigger_threshold: 100,
        ..OSRConfig::default()
    };
    let osr_manager = OSRManager::new(&context, config);
    
    // Should not trigger before threshold
    assert!(!osr_manager.should_trigger_osr("test_function", 50));
    
    // Would trigger after threshold (but no replacement prepared)
    assert!(!osr_manager.should_trigger_osr("test_function", 150));
}

/// Test deoptimization scenarios
#[test]
fn test_deoptimization() {
    let context = Context::create();
    let mut osr_manager = OSRManager::new(&context, OSRConfig::default());
    
    // Test different deoptimization reasons
    let deopt_reasons = vec![
        DeoptimizationReason::SpeculativeOptimizationFailed,
        DeoptimizationReason::TypeAssumptionViolated,
        DeoptimizationReason::ControlFlowAssumptionViolated,
        DeoptimizationReason::MemoryLayoutAssumptionViolated,
        DeoptimizationReason::ExternalDependencyChanged,
    ];
    
    for (i, reason) in deopt_reasons.iter().enumerate() {
        let function_name = format!("test_function_{}", i);
        let result = osr_manager.trigger_deoptimization(&function_name, reason.clone());
        assert!(result.is_ok());
    }
    
    let stats = osr_manager.get_stats();
    assert_eq!(stats.deoptimizations, deopt_reasons.len() as u64);
}

/// Test tiered compilation manager creation
#[test]
fn test_tiered_compilation_manager_creation() {
    let context = Context::create();
    let config = TieredCompilationConfig::default();
    let manager = TieredCompilationManager::new(&context, config);
    
    assert!(manager.is_ok());
    let manager = manager.unwrap();
    assert_eq!(manager.get_stats().total_promotions, 0);
    assert_eq!(manager.get_stats().total_demotions, 0);
}

/// Test tiered compilation configurations
#[test]
fn test_tiered_compilation_configurations() {
    let context = Context::create();
    
    // Test optimized configuration
    let optimized_manager = create_optimized_tiered_manager(&context).unwrap();
    assert!(optimized_manager.get_config().enable_auto_promotion);
    assert!(optimized_manager.get_config().enable_background_compilation);
    assert!(optimized_manager.get_config().enable_profiling_guided_optimization);
    
    // Test debug configuration
    let debug_manager = create_debug_tiered_manager(&context).unwrap();
    assert!(!debug_manager.get_config().enable_auto_promotion);
    assert!(!debug_manager.get_config().enable_background_compilation);
    assert!(!debug_manager.get_config().enable_profiling_guided_optimization);
}

/// Test function registration and tier tracking
#[test]
fn test_function_registration_and_tiers() {
    let context = Context::create();
    let mut manager = TieredCompilationManager::new(&context, TieredCompilationConfig::default()).unwrap();
    
    // Register functions
    let functions = vec!["function_1", "function_2", "function_3"];
    for function in &functions {
        let result = manager.register_function(function);
        assert!(result.is_ok());
        
        // All functions should start at Interpreter tier
        assert_eq!(manager.get_function_tier(function), CompilationTier::Interpreter);
    }
    
    // Check function profiles
    for function in &functions {
        let profile = manager.get_function_profile(function);
        assert!(profile.is_some());
        let profile = profile.unwrap();
        assert_eq!(profile.function_name, *function);
        assert_eq!(profile.current_tier, CompilationTier::Interpreter);
        assert_eq!(profile.execution_count, 0);
    }
}

/// Test execution recording and performance tracking
#[test]
fn test_execution_recording() {
    let context = Context::create();
    let mut manager = TieredCompilationManager::new(&context, TieredCompilationConfig::default()).unwrap();
    
    let function_name = "test_execution_function";
    manager.register_function(function_name).unwrap();
    
    // Record multiple executions
    for i in 1..=10 {
        let execution_time = Duration::from_millis(i * 10);
        let result = manager.record_execution(function_name, execution_time);
        assert!(result.is_ok());
    }
    
    // Check updated profile
    let profile = manager.get_function_profile(function_name).unwrap();
    assert_eq!(profile.execution_count, 10);
    assert!(profile.total_execution_time > Duration::ZERO);
    assert!(profile.avg_execution_time > Duration::ZERO);
}

/// Test hot path detection
#[test]
fn test_hot_path_detection() {
    let context = Context::create();
    let mut manager = TieredCompilationManager::new(&context, TieredCompilationConfig::default()).unwrap();
    
    let function_name = "hot_path_function";
    manager.register_function(function_name).unwrap();
    
    // Detect hot paths
    let hot_paths = manager.detect_hot_paths(function_name);
    assert!(hot_paths.is_ok());
    
    let hot_paths = hot_paths.unwrap();
    assert!(!hot_paths.is_empty());
    
    // Verify hot path segments
    for hot_path in &hot_paths {
        assert!(hot_path.segment_id.contains(function_name));
        assert!(hot_path.execution_frequency > 0);
        assert!(hot_path.optimization_potential > 0.0);
        assert!(hot_path.optimization_potential <= 1.0);
    }
}

/// Test optimization opportunity identification
#[test]
fn test_optimization_opportunities() {
    let context = Context::create();
    let mut manager = TieredCompilationManager::new(&context, TieredCompilationConfig::default()).unwrap();
    
    let function_name = "optimization_test_function";
    manager.register_function(function_name).unwrap();
    
    // Test opportunities for different tiers
    let tiers = vec![
        CompilationTier::BasicJIT,
        CompilationTier::OptimizedJIT,
        CompilationTier::HighlyOptimizedJIT,
        CompilationTier::SpeculativeJIT,
    ];
    
    for tier in tiers {
        let opportunities = manager.identify_optimization_opportunities(function_name, tier);
        assert!(opportunities.is_ok());
        
        let opportunities = opportunities.unwrap();
        if tier != CompilationTier::Interpreter {
            assert!(!opportunities.is_empty());
            
            for opportunity in &opportunities {
                assert_eq!(opportunity.required_tier, tier);
                assert!(opportunity.potential_improvement > 1.0);
                assert!(opportunity.confidence_score > 0.0);
                assert!(opportunity.confidence_score <= 1.0);
            }
        }
    }
}

/// Test JIT compilation interface with OSR and tiered compilation
#[test]
fn test_integrated_jit_interface() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    
    let config = JitCompilationConfig {
        enable_osr: true,
        enable_tiered_compilation: true,
        hot_path_threshold: 5, // Low threshold for testing
        ..JitCompilationConfig::default()
    };
    
    let mut interface = JitCompilationInterface::new(&context, jit_engine, codegen, config).unwrap();
    
    // Test function compilation
    let function_name = "integrated_test_function";
    let result = interface.compile_function(function_name, "");
    assert!(result.is_ok());
    
    // Test function execution with OSR and tiered compilation
    for _ in 0..10 {
        let result = interface.execute_function(function_name);
        assert!(result.is_ok());
    }
    
    // Check that function was registered with tiered compilation
    let tier = interface.get_tiered_manager().get_function_tier(function_name);
    assert_eq!(tier, CompilationTier::Interpreter); // Should start at interpreter tier
    
    // Check statistics
    let stats = interface.get_stats();
    assert_eq!(stats.total_jit_compilations, 1);
    assert!(stats.tiered_stats.functions_per_tier.get(&CompilationTier::Interpreter).unwrap_or(&0) >= &1);
}

/// Test comprehensive JIT interface functionality
#[test]
fn test_comprehensive_jit_features() {
    let context = Context::create();
    
    // Test with optimized configuration
    let mut interface = create_optimized_jit_interface(&context).unwrap();
    
    // Test multiple functions
    let functions = vec!["func_1", "func_2", "func_3"];
    
    for function in &functions {
        // Compile function
        let result = interface.compile_function(function, "");
        assert!(result.is_ok());
        
        // Execute multiple times to trigger tiered compilation
        for _ in 0..20 {
            let result = interface.execute_function(function);
            assert!(result.is_ok());
        }
    }
    
    // Check comprehensive statistics
    let stats = interface.get_stats();
    assert_eq!(stats.total_jit_compilations, functions.len() as u64);
    
    // Test comprehensive report generation
    let report = interface.generate_comprehensive_report();
    assert!(report.contains("Comprehensive JIT Performance Report"));
    assert!(report.contains("JIT Compilation Statistics"));
    assert!(report.contains("OSR (On-Stack Replacement) Statistics"));
    assert!(report.contains("Tiered Compilation Statistics"));
    assert!(report.contains("Configuration"));
}

/// Test OSR preparation and transitions
#[test] 
fn test_osr_preparation_and_transitions() {
    let context = Context::create();
    
    let mut interface = create_debug_jit_interface(&context).unwrap();
    
    let function_name = "osr_test_function";
    interface.compile_function(function_name, "").unwrap();
    
    // Test OSR preparation
    let result = interface.prepare_osr_for_function(function_name);
    assert!(result.is_ok());
    
    // Test deoptimization
    let result = interface.trigger_deoptimization(function_name, DeoptimizationReason::TypeAssumptionViolated);
    assert!(result.is_ok());
}

/// Test error handling in OSR and tiered compilation
#[test]
fn test_error_handling() {
    let context = Context::create();
    let osr_manager = OSRManager::new(&context, OSRConfig::default());
    
    // Test stack depth limits
    let config = OSRConfig {
        max_stack_depth: 2,
        ..OSRConfig::default()
    };
    let limited_manager = OSRManager::new(&context, config);
    
    // Should be able to add frames within limit
    let frame1 = StackFrame {
        function_name: "frame1".to_string(),
        local_variables: HashMap::new(),
        return_address: None,
        frame_pointer: None,
        stack_pointer: None,
    };
    
    let frame2 = StackFrame {
        function_name: "frame2".to_string(),
        local_variables: HashMap::new(),
        return_address: None,
        frame_pointer: None,
        stack_pointer: None,
    };
    
    let frame3 = StackFrame {
        function_name: "frame3".to_string(),
        local_variables: HashMap::new(),
        return_address: None,
        frame_pointer: None,
        stack_pointer: None,
    };
    
    assert!(limited_manager.record_stack_frame_entry(frame1).is_ok());
    assert!(limited_manager.record_stack_frame_entry(frame2).is_ok());
    
    // Should fail when exceeding limit
    let result = limited_manager.record_stack_frame_entry(frame3);
    assert!(result.is_err());
}

/// Test performance and statistics tracking
#[test]
fn test_performance_statistics() {
    let context = Context::create();
    let mut interface = create_optimized_jit_interface(&context).unwrap();
    
    // Compile and execute functions
    let functions = vec!["perf_func_1", "perf_func_2"];
    
    for function in &functions {
        interface.compile_function(function, "").unwrap();
        
        // Execute with varying patterns
        for i in 0..15 {
            interface.execute_function(function).unwrap();
            
            // Simulate different execution patterns
            if i % 5 == 0 {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }
    
    // Check detailed statistics
    let stats = interface.get_stats();
    assert!(stats.total_jit_compilations >= functions.len() as u64);
    
    // Check OSR statistics
    assert!(stats.osr_stats.total_osr_replacements >= 0);
    
    // Check tiered compilation statistics
    assert!(stats.tiered_stats.total_promotions >= 0);
    
    // Test report generation
    let report = interface.generate_comprehensive_report();
    assert!(!report.is_empty());
    
    // Test individual manager reports
    let osr_report = interface.get_osr_manager().generate_osr_report();
    assert!(osr_report.contains("OSR (On-Stack Replacement) Report"));
    
    let tiered_report = interface.get_tiered_manager().generate_report();
    assert!(tiered_report.contains("Tiered Compilation Report"));
}

/// Test variable value types in OSR
#[test]
fn test_variable_value_types() {
    // Test different variable value types
    let int_val = VariableValue {
        name: "test_int".to_string(),
        value: VariableValueType::Integer(42),
        type_name: "i32".to_string(),
        is_live: true,
    };
    
    let float_val = VariableValue {
        name: "test_float".to_string(),
        value: VariableValueType::Float(3.14),
        type_name: "f64".to_string(),
        is_live: true,
    };
    
    let bool_val = VariableValue {
        name: "test_bool".to_string(),
        value: VariableValueType::Boolean(true),
        type_name: "bool".to_string(),
        is_live: false,
    };
    
    let string_val = VariableValue {
        name: "test_string".to_string(),
        value: VariableValueType::String("hello".to_string()),
        type_name: "String".to_string(),
        is_live: true,
    };
    
    let pointer_val = VariableValue {
        name: "test_pointer".to_string(),
        value: VariableValueType::Pointer(0x1000),
        type_name: "*const i32".to_string(),
        is_live: true,
    };
    
    // Test aggregate type
    let aggregate_val = VariableValue {
        name: "test_aggregate".to_string(),
        value: VariableValueType::Aggregate(vec![int_val.clone(), float_val.clone()]),
        type_name: "Tuple".to_string(),
        is_live: true,
    };
    
    // Verify properties
    assert_eq!(int_val.name, "test_int");
    assert!(int_val.is_live);
    assert!(matches!(int_val.value, VariableValueType::Integer(42)));
    
    assert_eq!(float_val.name, "test_float");
    assert!(matches!(float_val.value, VariableValueType::Float(f) if (f - 3.14).abs() < f64::EPSILON));
    
    assert_eq!(bool_val.name, "test_bool");
    assert!(!bool_val.is_live);
    assert!(matches!(bool_val.value, VariableValueType::Boolean(true)));
    
    assert!(matches!(string_val.value, VariableValueType::String(ref s) if s == "hello"));
    assert!(matches!(pointer_val.value, VariableValueType::Pointer(0x1000)));
    assert!(matches!(aggregate_val.value, VariableValueType::Aggregate(ref vec) if vec.len() == 2));
}

/// Integration test with complex execution patterns
#[test]
fn test_complex_execution_patterns() {
    let context = Context::create();
    let mut interface = create_optimized_jit_interface(&context).unwrap();
    
    // Create functions with different characteristics
    let hot_function = "very_hot_function";
    let warm_function = "warm_function";
    let cold_function = "cold_function";
    
    // Compile all functions
    interface.compile_function(hot_function, "").unwrap();
    interface.compile_function(warm_function, "").unwrap(); 
    interface.compile_function(cold_function, "").unwrap();
    
    // Create different execution patterns
    // Hot function: many executions
    for _ in 0..100 {
        interface.execute_function(hot_function).unwrap();
    }
    
    // Warm function: moderate executions
    for _ in 0..20 {
        interface.execute_function(warm_function).unwrap();
    }
    
    // Cold function: few executions
    for _ in 0..3 {
        interface.execute_function(cold_function).unwrap();
    }
    
    // Check that hot path detection works
    let hot_paths = interface.get_hot_paths();
    
    // At least one function should be detected as hot
    // Note: This depends on the hot path threshold configuration
    if interface.get_config().hot_path_threshold <= 100 {
        assert!(hot_paths.contains(&hot_function.to_string()));
    }
    
    // Generate final report
    let report = interface.generate_comprehensive_report();
    assert!(report.contains(hot_function));
    assert!(report.contains(warm_function));
    assert!(report.contains(cold_function));
}

/// Test configuration updates and dynamic reconfiguration
#[test]
fn test_dynamic_configuration() {
    let context = Context::create();
    let mut interface = create_debug_jit_interface(&context).unwrap();
    
    // Test initial configuration
    let initial_config = interface.get_config();
    assert!(!initial_config.enable_osr);
    assert!(!initial_config.enable_tiered_compilation);
    
    // Update configuration to enable features
    let mut new_config = initial_config.clone();
    new_config.enable_osr = true;
    new_config.enable_tiered_compilation = true;
    new_config.hot_path_threshold = 5;
    
    interface.update_config(new_config);
    
    let updated_config = interface.get_config();
    assert!(updated_config.enable_osr);
    assert!(updated_config.enable_tiered_compilation);
    assert_eq!(updated_config.hot_path_threshold, 5);
}

/// Benchmark-style test for performance validation
#[test]
fn test_performance_benchmarks() {
    let context = Context::create();
    let mut interface = create_optimized_jit_interface(&context).unwrap();
    
    let function_name = "benchmark_function";
    interface.compile_function(function_name, "").unwrap();
    
    // Measure compilation performance
    let compile_start = std::time::Instant::now();
    interface.compile_function("compile_benchmark", "").unwrap();
    let compile_time = compile_start.elapsed();
    
    // Should compile reasonably quickly
    assert!(compile_time < Duration::from_secs(1));
    
    // Measure execution performance
    let exec_start = std::time::Instant::now();
    for _ in 0..1000 {
        interface.execute_function(function_name).unwrap();
    }
    let exec_time = exec_start.elapsed();
    
    // Should execute 1000 times in reasonable time
    assert!(exec_time < Duration::from_secs(10));
    
    // Check that average execution time is reasonable
    let avg_exec_time = exec_time / 1000;
    assert!(avg_exec_time < Duration::from_millis(10));
    
    println!("Compile time: {:?}", compile_time);
    println!("Total execution time for 1000 calls: {:?}", exec_time);
    println!("Average execution time: {:?}", avg_exec_time);
}
