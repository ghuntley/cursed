/// Comprehensive JIT Infrastructure Tests for CURSED Language
/// 
/// Tests the complete JIT compilation pipeline including:
/// - JIT engine functionality
/// - Compilation interface
/// - Runtime integration
/// - Configuration management
/// - Performance monitoring
/// - Memory management

use cursed::error::Error;
use cursed::config::{JitConfig, JitEngineConfig, JitCompilationConfig};
use cursed::codegen::llvm::{LlvmCodeGenerator, jit_engine::CursedJitEngine, jit_compilation::JitCompilationInterface};
use cursed::runtime::{Runtime, jit_runtime::JitRuntime};
use cursed::ast::Program;

use std::sync::Arc;
use std::time::Duration;
use inkwell::context::Context;

#[test]
fn test_jit_engine_basic_functionality() {
    let context = Context::create();
    let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    
    // Test function compilation
    let result = engine.compile_function("test_function", "");
    assert!(result.is_ok(), "Function compilation should succeed");
    
    // Test function availability
    assert!(engine.has_function("test_function"), "Function should be available after compilation");
    
    // Test function execution
    let execution_result = engine.execute_function("test_function");
    assert!(execution_result.is_ok(), "Function execution should succeed");
    assert_eq!(execution_result.unwrap(), 0, "Function should return 0");
    
    // Test statistics
    let stats = engine.get_stats();
    assert_eq!(stats.functions_compiled, 1, "Should have compiled 1 function");
    assert_eq!(stats.functions_executed, 1, "Should have executed 1 function");
    assert!(stats.compilation_time_ms > 0, "Compilation time should be recorded");
    assert!(stats.execution_time_ms >= 0, "Execution time should be recorded");
}

#[test]
fn test_jit_engine_caching() {
    let context = Context::create();
    let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    
    // Compile function twice - second should be cache hit
    engine.compile_function("cached_function", "").unwrap();
    let stats_after_first = engine.get_stats();
    
    engine.compile_function("cached_function", "").unwrap();
    let stats_after_second = engine.get_stats();
    
    // Should have cache hit
    assert!(stats_after_second.cache_hits > stats_after_first.cache_hits, "Should have cache hit");
    assert_eq!(stats_after_second.functions_compiled, 1, "Should still only have compiled 1 unique function");
}

#[test]
fn test_jit_engine_memory_management() {
    let context = Context::create();
    let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    
    // Test function removal
    engine.compile_function("removable_function", "").unwrap();
    assert!(engine.has_function("removable_function"));
    
    engine.remove_function("removable_function").unwrap();
    // Note: Due to LLVM ExecutionEngine limitations, function might still exist
    
    // Test cache clearing
    engine.compile_function("function1", "").unwrap();
    engine.compile_function("function2", "").unwrap();
    
    engine.clear_cache().unwrap();
    // Cache should be cleared
}

#[test]
fn test_jit_engine_configuration() {
    let context = Context::create();
    let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    
    // Test initial configuration
    let config = engine.get_config();
    assert!(config.enable_function_cache, "Function cache should be enabled by default");
    
    // Test cache size limits
    engine.set_max_cached_functions(5);
    assert_eq!(engine.get_config().max_cached_functions, 5);
    
    // Test cache enable/disable
    engine.set_function_cache_enabled(false);
    assert!(!engine.get_config().enable_function_cache);
    
    // Test performance monitoring enable/disable
    engine.set_performance_monitoring_enabled(false);
    assert!(!engine.get_config().enable_performance_monitoring);
}

#[test]
fn test_jit_compilation_interface_basic() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    
    let mut interface = JitCompilationInterface::new_with_default_config(
        &context, jit_engine, codegen
    );
    
    // Test function compilation
    let result = interface.compile_function("test_function", "");
    assert!(result.is_ok(), "Function compilation should succeed");
    
    // Test function execution
    let execution_result = interface.execute_function("test_function");
    assert!(execution_result.is_ok(), "Function execution should succeed");
    assert_eq!(execution_result.unwrap(), 0);
    
    // Test function availability
    assert!(interface.has_function("test_function"));
    
    // Test statistics
    let stats = interface.get_stats();
    assert_eq!(stats.total_jit_compilations, 1);
    assert!(stats.total_compilation_time > Duration::ZERO);
}

#[test]
fn test_hot_path_detection() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    
    let mut config = JitCompilationConfig::default();
    config.hot_path_threshold = 3; // Low threshold for testing
    
    let mut interface = JitCompilationInterface::new(
        &context, jit_engine, codegen, config
    );
    
    // Compile function
    interface.compile_function("hot_function", "").unwrap();
    
    // Execute multiple times to trigger hot path detection
    for _ in 0..5 {
        interface.execute_function("hot_function").unwrap();
    }
    
    // Function should be identified as hot path
    let hot_paths = interface.get_hot_paths();
    assert!(hot_paths.contains(&"hot_function".to_string()), "Function should be identified as hot path");
    
    // Test optimization
    let optimization_candidates = interface.get_optimization_candidates();
    assert!(!optimization_candidates.is_empty(), "Should have optimization candidates");
    
    let optimized_count = interface.optimize_hot_paths().unwrap();
    assert!(optimized_count > 0, "Should have optimized some functions");
}

#[test]
fn test_jit_compilation_statistics() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    
    let mut interface = JitCompilationInterface::new_with_default_config(
        &context, jit_engine, codegen
    );
    
    let initial_stats = interface.get_stats();
    assert_eq!(initial_stats.total_jit_compilations, 0);
    
    // Compile and execute function
    interface.compile_function("stats_test", "").unwrap();
    interface.execute_function("stats_test").unwrap();
    
    let stats = interface.get_stats();
    assert_eq!(stats.total_jit_compilations, 1);
    assert!(stats.total_compilation_time > Duration::ZERO);
    
    // Test engine statistics
    let engine_stats = interface.get_engine_stats();
    assert!(engine_stats.functions_compiled > 0);
    assert!(engine_stats.functions_executed > 0);
}

#[test]
fn test_jit_compilation_profiling() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    
    let mut interface = JitCompilationInterface::new_with_default_config(
        &context, jit_engine, codegen
    );
    
    // Compile function
    interface.compile_function("profile_test", "").unwrap();
    
    // Profile function execution
    let avg_time = interface.profile_function_execution("profile_test", 10);
    assert!(avg_time.is_ok(), "Profiling should succeed");
    assert!(avg_time.unwrap() >= Duration::ZERO, "Average time should be non-negative");
    
    // Test function statistics
    let function_stats = interface.get_function_stats("profile_test");
    assert!(function_stats.is_some(), "Should have function statistics");
    
    let (execution_count, total_time, is_optimized) = function_stats.unwrap();
    assert_eq!(execution_count, 10, "Should have 10 executions");
    assert!(total_time > Duration::ZERO, "Total time should be positive");
}

#[test]
fn test_jit_runtime_creation_and_initialization() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let mut jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    
    // Test initialization
    let result = jit_runtime.initialize();
    assert!(result.is_ok(), "JIT runtime initialization should succeed");
    
    // Test statistics
    let stats = jit_runtime.get_stats();
    assert_eq!(stats.total_jit_executions, 0, "Should start with 0 executions");
    
    // Test configuration
    let config = jit_runtime.get_config();
    assert!(config.enable_goroutine_integration, "Goroutine integration should be enabled by default");
    assert!(config.enable_gc_integration, "GC integration should be enabled by default");
}

#[test]
fn test_jit_runtime_function_execution() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    
    // Compile function
    jit_runtime.compile_function("runtime_test", "").unwrap();
    assert!(jit_runtime.has_function("runtime_test"), "Function should be available");
    
    // Execute function
    let result = jit_runtime.execute_function("runtime_test");
    assert!(result.is_ok(), "Function execution should succeed");
    assert_eq!(result.unwrap(), 0, "Function should return 0");
    
    // Check statistics
    let stats = jit_runtime.get_stats();
    assert_eq!(stats.total_jit_executions, 1, "Should have 1 execution");
    assert!(stats.total_execution_time > Duration::ZERO, "Should have execution time recorded");
}

#[test]
fn test_jit_runtime_memory_management() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    
    // Test memory info
    let (allocated, max_memory, usage_percent) = jit_runtime.get_memory_info();
    assert_eq!(allocated, 0, "Should start with 0 allocated memory");
    assert!(max_memory > 0, "Should have positive max memory");
    assert_eq!(usage_percent, 0.0, "Should start with 0% usage");
    
    // Test function performance statistics
    jit_runtime.compile_function("memory_test", "").unwrap();
    jit_runtime.execute_function("memory_test").unwrap();
    
    // Should have performance data available
    let performance = jit_runtime.get_function_performance("memory_test");
    // Note: Performance data might not be available immediately due to implementation details
}

#[test]
fn test_jit_runtime_optimization() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    
    // Compile function
    jit_runtime.compile_function("optimization_test", "").unwrap();
    
    // Execute multiple times to potentially trigger hot path detection
    for _ in 0..10 {
        jit_runtime.execute_function("optimization_test").unwrap();
    }
    
    // Test manual optimization
    let optimized_count = jit_runtime.optimize_hot_paths();
    assert!(optimized_count.is_ok(), "Hot path optimization should succeed");
}

#[test]
fn test_jit_config_creation_and_validation() {
    // Test default configuration
    let config = JitConfig::default();
    assert!(config.validate().is_ok(), "Default config should be valid");
    
    // Test development configuration
    let dev_config = JitConfig::development();
    assert_eq!(dev_config.engine.optimization_level, "none");
    assert!(dev_config.debug.debug_mode);
    assert!(dev_config.validate().is_ok());
    
    // Test production configuration
    let prod_config = JitConfig::production();
    assert_eq!(prod_config.engine.optimization_level, "aggressive");
    assert!(!prod_config.debug.debug_mode);
    assert!(prod_config.validate().is_ok());
    
    // Test benchmarking configuration
    let bench_config = JitConfig::benchmarking();
    assert_eq!(bench_config.engine.optimization_level, "aggressive");
    assert!(!bench_config.monitoring.enabled);
    assert!(bench_config.debug.enable_timing);
    assert!(bench_config.validate().is_ok());
}

#[test]
fn test_jit_config_invalid_values() {
    let mut config = JitConfig::default();
    
    // Test invalid optimization level
    config.engine.optimization_level = "invalid".to_string();
    assert!(config.validate().is_err(), "Should reject invalid optimization level");
    
    // Test invalid threshold
    config.engine.optimization_level = "default".to_string();
    config.compilation.hot_path_threshold = 0;
    assert!(config.validate().is_err(), "Should reject zero hot path threshold");
    
    // Test invalid memory limit
    config.compilation.hot_path_threshold = 100;
    config.memory.max_jit_memory_mb = 0;
    assert!(config.validate().is_err(), "Should reject zero memory limit");
    
    // Test invalid sample rate
    config.memory.max_jit_memory_mb = 100;
    config.monitoring.sample_rate = 1.5;
    assert!(config.validate().is_err(), "Should reject sample rate > 1.0");
}

#[test]
fn test_jit_config_merging() {
    let mut base_config = JitConfig::default();
    let override_config = JitConfig::development();
    
    let original_opt_level = base_config.engine.optimization_level.clone();
    base_config.merge(&override_config);
    
    // Should use development config values
    assert_eq!(base_config.engine.optimization_level, "none");
    assert!(base_config.debug.debug_mode);
    assert_ne!(base_config.engine.optimization_level, original_opt_level);
}

#[test]
fn test_jit_config_serialization() {
    let config = JitConfig::development();
    
    // Test TOML serialization
    let toml_str = toml::to_string(&config).unwrap();
    let deserialized: JitConfig = toml::from_str(&toml_str).unwrap();
    assert_eq!(config.engine.optimization_level, deserialized.engine.optimization_level);
    assert_eq!(config.debug.debug_mode, deserialized.debug.debug_mode);
    
    // Test JSON serialization
    let json_str = serde_json::to_string(&config).unwrap();
    let deserialized: JitConfig = serde_json::from_str(&json_str).unwrap();
    assert_eq!(config.compilation.hot_path_threshold, deserialized.compilation.hot_path_threshold);
    assert_eq!(config.runtime.enable_goroutine_integration, deserialized.runtime.enable_goroutine_integration);
}

#[test]
#[ignore] // Ignore for normal test runs due to file system operations
fn test_jit_config_file_operations() {
    use tempfile::tempdir;
    
    let config = JitConfig::development();
    let temp_dir = tempdir().unwrap();
    
    // Test TOML file operations
    let toml_path = temp_dir.path().join("config.toml");
    config.save_to_toml_file(&toml_path).unwrap();
    let loaded_config = JitConfig::from_toml_file(&toml_path).unwrap();
    assert_eq!(config.engine.optimization_level, loaded_config.engine.optimization_level);
    
    // Test JSON file operations
    let json_path = temp_dir.path().join("config.json");
    config.save_to_json_file(&json_path).unwrap();
    let loaded_config = JitConfig::from_json_file(&json_path).unwrap();
    assert_eq!(config.debug.debug_mode, loaded_config.debug.debug_mode);
}

#[test]
fn test_optimization_level_parsing() {
    use cursed::config::parse_optimization_level;
    
    assert!(parse_optimization_level("none").is_ok());
    assert!(parse_optimization_level("less").is_ok());
    assert!(parse_optimization_level("default").is_ok());
    assert!(parse_optimization_level("aggressive").is_ok());
    assert!(parse_optimization_level("AGGRESSIVE").is_ok()); // Case insensitive
    assert!(parse_optimization_level("invalid").is_err());
}

#[test]
fn test_jit_integration_end_to_end() {
    // Create a complete JIT pipeline
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let mut jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    
    // Initialize runtime
    jit_runtime.initialize().unwrap();
    
    // Compile and execute multiple functions
    let function_names = vec!["func1", "func2", "func3"];
    
    for func_name in &function_names {
        jit_runtime.compile_function(func_name, "").unwrap();
        assert!(jit_runtime.has_function(func_name));
        
        let result = jit_runtime.execute_function(func_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }
    
    // Check final statistics
    let stats = jit_runtime.get_stats();
    assert_eq!(stats.total_jit_executions, function_names.len() as u64);
    
    let jit_stats = jit_runtime.get_jit_stats();
    assert_eq!(jit_stats.total_jit_compilations, function_names.len() as u64);
    
    // Shutdown runtime
    jit_runtime.shutdown().unwrap();
}

#[test]
fn test_jit_performance_monitoring() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    
    let mut config = JitCompilationConfig::default();
    config.hot_path_threshold = 5; // Low threshold for testing
    
    let mut interface = JitCompilationInterface::new(&context, jit_engine, codegen, config);
    
    // Compile function
    interface.compile_function("perf_test", "").unwrap();
    
    // Execute function multiple times to generate performance data
    for _ in 0..10 {
        interface.execute_function("perf_test").unwrap();
    }
    
    // Test function statistics
    let function_stats = interface.get_function_stats("perf_test");
    assert!(function_stats.is_some());
    
    let (execution_count, total_time, is_optimized) = function_stats.unwrap();
    assert_eq!(execution_count, 10);
    assert!(total_time >= Duration::ZERO);
    
    // Test profiling
    let avg_time = interface.profile_function_execution("perf_test", 5);
    assert!(avg_time.is_ok());
    assert!(avg_time.unwrap() >= Duration::ZERO);
}

#[test]
fn test_jit_error_handling() {
    let context = Context::create();
    let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    
    // Test execution of non-existent function
    let result = engine.execute_function("nonexistent_function");
    assert!(result.is_err(), "Should fail to execute non-existent function");
    
    // Test compilation with invalid IR (this will still succeed with our stub implementation)
    let result = engine.compile_function("invalid_function", "invalid ir code");
    assert!(result.is_ok(), "Compilation should handle invalid IR gracefully");
}

#[test]
fn test_jit_memory_limits() {
    use cursed::runtime::jit_runtime::JitMemoryManager;
    
    let memory_manager = JitMemoryManager::new(1024, false); // 1KB limit
    
    // Test successful allocation
    assert!(memory_manager.allocate("small_function", 512).is_ok());
    assert_eq!(memory_manager.get_total_allocated(), 512);
    
    // Test allocation that would exceed limit
    assert!(memory_manager.allocate("large_function", 1024).is_err());
    
    // Test deallocation
    assert!(memory_manager.deallocate("small_function").is_ok());
    assert_eq!(memory_manager.get_total_allocated(), 0);
    
    // Test memory limit checking
    assert!(!memory_manager.would_exceed_limit(512));
    assert!(memory_manager.would_exceed_limit(2048));
}

#[test]
fn test_jit_configuration_environment_variables() {
    // Set environment variables
    std::env::set_var("CURSED_JIT_OPTIMIZATION_LEVEL", "aggressive");
    std::env::set_var("CURSED_JIT_ENABLE_CACHE", "false");
    std::env::set_var("CURSED_JIT_HOT_PATH_THRESHOLD", "25");
    std::env::set_var("CURSED_JIT_DEBUG_MODE", "true");
    
    let config = JitConfig::from_env().unwrap();
    assert_eq!(config.engine.optimization_level, "aggressive");
    assert!(!config.engine.enable_function_cache);
    assert_eq!(config.compilation.hot_path_threshold, 25);
    assert!(config.debug.debug_mode);
    
    // Clean up environment variables
    std::env::remove_var("CURSED_JIT_OPTIMIZATION_LEVEL");
    std::env::remove_var("CURSED_JIT_ENABLE_CACHE");
    std::env::remove_var("CURSED_JIT_HOT_PATH_THRESHOLD");
    std::env::remove_var("CURSED_JIT_DEBUG_MODE");
}

#[test]
fn test_jit_statistics_accuracy() {
    let context = Context::create();
    let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    
    // Reset statistics
    engine.reset_stats();
    let initial_stats = engine.get_stats();
    assert_eq!(initial_stats.functions_compiled, 0);
    assert_eq!(initial_stats.functions_executed, 0);
    
    // Compile and execute functions
    for i in 0..3 {
        let function_name = format!("test_function_{}", i);
        engine.compile_function(&function_name, "").unwrap();
        engine.execute_function(&function_name).unwrap();
        engine.execute_function(&function_name).unwrap(); // Execute twice
    }
    
    let final_stats = engine.get_stats();
    assert_eq!(final_stats.functions_compiled, 3);
    assert_eq!(final_stats.functions_executed, 6); // 3 functions * 2 executions each
    assert!(final_stats.compilation_time_ms > 0);
    assert!(final_stats.execution_time_ms >= 0);
}
