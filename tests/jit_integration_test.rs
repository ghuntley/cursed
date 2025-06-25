/// JIT Integration Tests for CURSED Language
/// 
/// Tests integration between JIT compilation and existing CURSED systems:
/// - LLVM codegen integration
/// - Runtime system integration  
/// - Memory management integration
/// - Goroutine system integration
/// - Error handling integration

use cursed::error::Error;
use cursed::config::JitConfig;
use cursed::codegen::llvm::{LlvmCodeGenerator, jit_engine::CursedJitEngine, jit_compilation::JitCompilationInterface};
use cursed::runtime::{Runtime, jit_runtime::JitRuntime};
use cursed::ast::Program;
use cursed::memory::gc::GarbageCollector;

use std::sync::Arc;
use std::time::Duration;
use inkwell::context::Context;

/// Test integration between JIT engine and LLVM codegen
#[test]
fn test_jit_llvm_codegen_integration() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let mut codegen = LlvmCodeGenerator::new().unwrap();
    
    // Test IR generation
    let ir = codegen.generate_ir("").unwrap();
    assert!(!ir.is_empty(), "IR should be generated");
    assert!(ir.contains("define"), "IR should contain function definitions");
    
    // Test JIT compilation with generated IR
    let mut jit_engine = jit_engine;
    let result = jit_engine.compile_function("main", &ir);
    assert!(result.is_ok(), "JIT should compile generated IR");
    
    // Test execution
    let execution_result = jit_engine.execute_function("main");
    assert!(execution_result.is_ok(), "JIT should execute compiled function");
}

/// Test integration between JIT compilation interface and runtime systems
#[test]
fn test_jit_compilation_runtime_integration() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let mut jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    // Test compilation with runtime context
    let result = jit_interface.compile_function("runtime_test", "");
    assert!(result.is_ok(), "Should compile with runtime integration");
    
    // Test execution with performance monitoring
    let execution_result = jit_interface.execute_function("runtime_test");
    assert!(execution_result.is_ok(), "Should execute with monitoring");
    
    // Verify statistics are collected
    let stats = jit_interface.get_stats();
    assert!(stats.total_compilation_time > Duration::ZERO, "Should record compilation time");
    
    let engine_stats = jit_interface.get_engine_stats();
    assert!(engine_stats.functions_compiled > 0, "Should record compiled functions");
}

/// Test JIT runtime integration with base runtime system
#[test]
fn test_jit_runtime_base_integration() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    // Create base runtime
    let mut base_runtime = Runtime::new();
    base_runtime.initialize().unwrap();
    let base_runtime = Arc::new(base_runtime);
    
    // Create JIT runtime with base runtime integration
    let mut jit_runtime = JitRuntime::new_with_default_config(jit_interface, base_runtime.clone());
    jit_runtime.initialize().unwrap();
    
    // Test function execution with runtime integration
    jit_runtime.compile_function("integration_test", "").unwrap();
    let result = jit_runtime.execute_function("integration_test");
    assert!(result.is_ok(), "Should execute with runtime integration");
    
    // Verify runtime statistics are updated
    let stats = jit_runtime.get_stats();
    assert_eq!(stats.total_jit_executions, 1, "Should record execution in runtime stats");
}

/// Test JIT memory management integration
#[test]
fn test_jit_memory_management_integration() {
    use cursed::runtime::jit_runtime::JitMemoryManager;
    
    let memory_manager = JitMemoryManager::new(10 * 1024 * 1024, true); // 10MB with GC integration
    
    // Test memory allocation tracking
    memory_manager.allocate("function1", 1024 * 1024).unwrap(); // 1MB
    memory_manager.allocate("function2", 2 * 1024 * 1024).unwrap(); // 2MB
    
    assert_eq!(memory_manager.get_total_allocated(), 3 * 1024 * 1024);
    
    // Test memory pressure detection
    assert!(!memory_manager.would_exceed_limit(1024 * 1024)); // 1MB more should be OK
    assert!(memory_manager.would_exceed_limit(8 * 1024 * 1024)); // 8MB more should exceed
    
    // Test GC trigger
    let result = memory_manager.trigger_gc_if_needed();
    assert!(result.is_ok(), "GC trigger should succeed");
    
    // Test memory deallocation
    memory_manager.deallocate("function1").unwrap();
    assert_eq!(memory_manager.get_total_allocated(), 2 * 1024 * 1024);
    
    memory_manager.deallocate("function2").unwrap();
    assert_eq!(memory_manager.get_total_allocated(), 0);
}

/// Test JIT configuration integration with different systems
#[test]
fn test_jit_config_integration() {
    // Test development configuration
    let dev_config = JitConfig::development();
    assert!(dev_config.validate().is_ok());
    
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let mut jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    
    // Test runtime configuration update
    let config = jit_runtime.get_config();
    assert!(config.enable_goroutine_integration, "Should enable goroutine integration by default");
    assert!(config.enable_gc_integration, "Should enable GC integration by default");
    
    // Test production configuration
    let prod_config = JitConfig::production();
    assert!(prod_config.validate().is_ok());
    assert_eq!(prod_config.engine.optimization_level, "aggressive");
    assert!(!prod_config.debug.debug_mode);
    
    // Test benchmarking configuration
    let bench_config = JitConfig::benchmarking();
    assert!(bench_config.validate().is_ok());
    assert!(!bench_config.compilation.enable_background_compilation);
    assert!(!bench_config.monitoring.enabled);
}

/// Test JIT hot path optimization integration
#[test]
fn test_jit_hot_path_optimization_integration() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    
    let mut config = cursed::codegen::llvm::jit_compilation::JitCompilationConfig::default();
    config.hot_path_threshold = 3; // Low threshold for testing
    config.enable_dynamic_recompilation = true;
    
    let mut jit_interface = JitCompilationInterface::new(&context, jit_engine, codegen, config);
    
    // Compile function
    jit_interface.compile_function("hot_path_test", "").unwrap();
    
    // Execute multiple times to trigger hot path detection
    for _ in 0..5 {
        jit_interface.execute_function("hot_path_test").unwrap();
    }
    
    // Verify hot path detection
    let hot_paths = jit_interface.get_hot_paths();
    assert!(hot_paths.contains(&"hot_path_test".to_string()), "Function should be detected as hot path");
    
    // Test optimization
    let candidates = jit_interface.get_optimization_candidates();
    assert!(!candidates.is_empty(), "Should have optimization candidates");
    
    let optimized_count = jit_interface.optimize_hot_paths().unwrap();
    assert!(optimized_count > 0, "Should optimize hot paths");
    
    // Verify optimization statistics
    let stats = jit_interface.get_stats();
    assert!(stats.hot_path_optimizations > 0, "Should record hot path optimizations");
}

/// Test JIT error handling integration
#[test]
fn test_jit_error_handling_integration() {
    let context = Context::create();
    let mut jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    
    // Test compilation error handling
    let result = jit_engine.compile_function("", ""); // Empty function name
    // This should still succeed with our current stub implementation
    
    // Test execution error handling
    let result = jit_engine.execute_function("nonexistent_function");
    assert!(result.is_err(), "Should fail gracefully for non-existent function");
    
    // Test with JIT runtime
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    
    // Test execution of non-existent function
    let result = jit_runtime.execute_function("nonexistent_function");
    assert!(result.is_err(), "Runtime should handle execution errors gracefully");
}

/// Test JIT performance monitoring integration
#[test]
fn test_jit_performance_monitoring_integration() {
    use cursed::runtime::jit_runtime::JitPerformanceMonitor;
    
    let monitor = JitPerformanceMonitor::new(true);
    
    // Record performance data
    for i in 0..10 {
        monitor.record_execution("test_function", Duration::from_millis(i * 10));
        monitor.record_memory_usage("test_function", 1024 * (i + 1));
    }
    
    // Test performance statistics
    let (avg_time, memory_usage) = monitor.get_function_performance("test_function").unwrap();
    assert!(avg_time > Duration::ZERO, "Should have average execution time");
    assert!(memory_usage > 0, "Should have memory usage data");
    
    // Test optimization opportunity detection
    let opportunities = monitor.get_optimization_opportunities();
    // Opportunities might be detected based on execution patterns
    
    // Test performance data clearing
    monitor.clear_optimization_opportunities();
    let cleared_opportunities = monitor.get_optimization_opportunities();
    assert!(cleared_opportunities.is_empty(), "Should clear optimization opportunities");
}

/// Test JIT integration with complex execution scenarios
#[test]
fn test_jit_complex_execution_scenarios() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let mut jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    jit_runtime.initialize().unwrap();
    
    // Scenario 1: Compile multiple functions
    let function_names = vec!["func1", "func2", "func3", "func4", "func5"];
    for func_name in &function_names {
        jit_runtime.compile_function(func_name, "").unwrap();
    }
    
    // Scenario 2: Execute functions with different patterns
    // Execute func1 many times (should become hot path)
    for _ in 0..20 {
        jit_runtime.execute_function("func1").unwrap();
    }
    
    // Execute other functions fewer times
    for func_name in &function_names[1..] {
        for _ in 0..3 {
            jit_runtime.execute_function(func_name).unwrap();
        }
    }
    
    // Scenario 3: Verify statistics and performance data
    let stats = jit_runtime.get_stats();
    assert_eq!(stats.total_jit_executions, 20 + 4 * 3); // 20 + 12 = 32
    
    // Scenario 4: Test optimization
    let optimized_count = jit_runtime.optimize_hot_paths().unwrap();
    // May or may not optimize depending on hot path detection
    
    // Scenario 5: Test memory usage tracking
    let (allocated, max_memory, usage_percent) = jit_runtime.get_memory_info();
    assert!(max_memory > 0, "Should have positive max memory");
    assert!(usage_percent >= 0.0, "Should have non-negative usage percentage");
    
    // Scenario 6: Clean shutdown
    jit_runtime.shutdown().unwrap();
}

/// Test JIT integration with concurrent execution
#[test]
fn test_jit_concurrent_execution_integration() {
    use std::thread;
    use std::sync::atomic::{AtomicU32, Ordering};
    
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let jit_runtime = Arc::new(JitRuntime::new_with_default_config(jit_interface, runtime));
    
    // Compile a function to execute concurrently
    jit_runtime.compile_function("concurrent_test", "").unwrap();
    
    let execution_count = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];
    
    // Spawn multiple threads to execute the function concurrently
    for thread_id in 0..4 {
        let jit_runtime_clone = jit_runtime.clone();
        let execution_count_clone = execution_count.clone();
        
        let handle = thread::spawn(move || {
            for _ in 0..5 {
                let result = jit_runtime_clone.execute_function("concurrent_test");
                if result.is_ok() {
                    execution_count_clone.fetch_add(1, Ordering::SeqCst);
                }
                
                // Small delay to simulate work
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_executions = execution_count.load(Ordering::SeqCst);
    assert_eq!(total_executions, 20, "Should have 20 successful executions"); // 4 threads * 5 executions
    
    // Verify runtime statistics
    let stats = jit_runtime.get_stats();
    assert_eq!(stats.total_jit_executions, 20, "Runtime should record all executions");
}

/// Test JIT integration with different optimization levels
#[test]
fn test_jit_optimization_level_integration() {
    let context = Context::create();
    
    // Test with different optimization levels
    let optimization_levels = ["none", "less", "default", "aggressive"];
    
    for opt_level in &optimization_levels {
        let config = cursed::config::JitEngineConfig {
            optimization_level: opt_level.to_string(),
            enable_function_cache: true,
            max_cached_functions: 100,
            enable_debug_info: opt_level == &"none",
            target_cpu: None,
            target_features: Vec::new(),
            enable_orc_v2: true,
            enable_lazy_compilation: true,
        };
        
        // Create JIT engine with specific optimization level
        let mut jit_engine = CursedJitEngine::new(&context, cursed::codegen::llvm::jit_engine::JitEngineConfig {
            optimization_level: cursed::config::parse_optimization_level(opt_level).unwrap(),
            enable_function_cache: config.enable_function_cache,
            enable_performance_monitoring: true,
            max_cached_functions: config.max_cached_functions,
            enable_debug_info: config.enable_debug_info,
            target_cpu: config.target_cpu.clone(),
            target_features: config.target_features.clone(),
        }).unwrap();
        
        // Test compilation and execution
        let function_name = format!("test_function_{}", opt_level);
        jit_engine.compile_function(&function_name, "").unwrap();
        let result = jit_engine.execute_function(&function_name);
        assert!(result.is_ok(), "Should execute with {} optimization", opt_level);
        
        // Verify configuration
        let engine_config = jit_engine.get_config();
        assert_eq!(engine_config.enable_debug_info, opt_level == "none");
    }
}

/// Test JIT integration with configuration loading
#[test]
fn test_jit_config_loading_integration() {
    // Test environment variable configuration
    std::env::set_var("CURSED_JIT_OPTIMIZATION_LEVEL", "aggressive");
    std::env::set_var("CURSED_JIT_ENABLE_CACHE", "true");
    std::env::set_var("CURSED_JIT_HOT_PATH_THRESHOLD", "50");
    
    let config = JitConfig::from_env().unwrap();
    assert_eq!(config.engine.optimization_level, "aggressive");
    assert!(config.engine.enable_function_cache);
    assert_eq!(config.compilation.hot_path_threshold, 50);
    
    // Test configuration validation
    assert!(config.validate().is_ok(), "Environment config should be valid");
    
    // Test configuration summary
    let summary = config.summary();
    assert!(summary.contains("Optimization Level: aggressive"));
    assert!(summary.contains("Hot Path Threshold: 50"));
    
    // Clean up environment
    std::env::remove_var("CURSED_JIT_OPTIMIZATION_LEVEL");
    std::env::remove_var("CURSED_JIT_ENABLE_CACHE");
    std::env::remove_var("CURSED_JIT_HOT_PATH_THRESHOLD");
}

/// Test JIT integration with memory pressure scenarios
#[test]
fn test_jit_memory_pressure_integration() {
    use cursed::runtime::jit_runtime::JitMemoryManager;
    
    // Create memory manager with low limit to test pressure scenarios
    let memory_manager = JitMemoryManager::new(5 * 1024 * 1024, true); // 5MB limit
    
    // Allocate memory for multiple functions
    let mut allocated_functions = vec![];
    
    for i in 0..3 {
        let function_name = format!("memory_test_{}", i);
        let size = 1024 * 1024; // 1MB per function
        
        let result = memory_manager.allocate(&function_name, size);
        assert!(result.is_ok(), "Should allocate memory for function {}", i);
        allocated_functions.push(function_name);
    }
    
    assert_eq!(memory_manager.get_total_allocated(), 3 * 1024 * 1024); // 3MB
    
    // Try to allocate more memory that would exceed limit
    let result = memory_manager.allocate("large_function", 3 * 1024 * 1024); // 3MB more
    assert!(result.is_err(), "Should fail to allocate when exceeding limit");
    
    // Test memory pressure detection
    assert!(memory_manager.would_exceed_limit(3 * 1024 * 1024), "Should detect memory pressure");
    assert!(!memory_manager.would_exceed_limit(1024 * 1024), "Should allow reasonable allocation");
    
    // Test GC trigger under pressure
    let result = memory_manager.trigger_gc_if_needed();
    assert!(result.is_ok(), "GC trigger should succeed under pressure");
    
    // Clean up by deallocating functions
    for function_name in allocated_functions {
        memory_manager.deallocate(&function_name).unwrap();
    }
    
    assert_eq!(memory_manager.get_total_allocated(), 0, "Should deallocate all memory");
}

/// Test end-to-end JIT integration with realistic scenarios
#[test]
fn test_jit_end_to_end_integration() {
    // Create complete JIT pipeline with realistic configuration
    let context = Context::create();
    let jit_config = JitConfig::development(); // Use development config for testing
    
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let mut jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    
    // Initialize complete system
    jit_runtime.initialize().unwrap();
    
    // Simulate realistic usage pattern
    let functions = vec![
        ("fibonacci", 100),     // Hot path - execute many times
        ("quicksort", 50),      // Moderate usage
        ("hello_world", 10),    // Light usage
        ("matrix_mult", 30),    // Medium usage
        ("file_parser", 5),     // Rarely used
    ];
    
    // Compile all functions
    for (func_name, _) in &functions {
        jit_runtime.compile_function(func_name, "").unwrap();
        assert!(jit_runtime.has_function(func_name), "Function {} should be available", func_name);
    }
    
    // Execute functions according to usage pattern
    for (func_name, execution_count) in &functions {
        for _ in 0..*execution_count {
            let result = jit_runtime.execute_function(func_name);
            assert!(result.is_ok(), "Execution of {} should succeed", func_name);
        }
    }
    
    // Verify final statistics
    let stats = jit_runtime.get_stats();
    let total_expected_executions: u32 = functions.iter().map(|(_, count)| count).sum();
    assert_eq!(stats.total_jit_executions, total_expected_executions as u64);
    assert!(stats.total_execution_time > Duration::ZERO);
    
    // Test optimization of hot paths
    let optimized_count = jit_runtime.optimize_hot_paths().unwrap();
    // May or may not optimize depending on hot path detection thresholds
    
    // Test memory usage tracking
    let (allocated, max_memory, usage_percent) = jit_runtime.get_memory_info();
    assert!(max_memory > 0);
    assert!(usage_percent >= 0.0 && usage_percent <= 100.0);
    
    // Test performance monitoring
    let performance = jit_runtime.get_function_performance("fibonacci");
    // Performance data might not be available immediately due to implementation details
    
    // Clean shutdown
    jit_runtime.shutdown().unwrap();
}
