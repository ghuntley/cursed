//! Comprehensive JIT Engine Tests for CURSED
//!
//! Tests the complete JIT compilation pipeline including:
//! - Basic function compilation and execution
//! - Hot path detection and optimization
//! - Memory management and performance monitoring
//! - REPL integration
//! - Error handling and edge cases

use cursed::codegen::llvm::{
    CursedJitEngine, JitEngineConfig, JitCompilationInterface, 
    create_optimized_jit_engine, create_debug_jit_engine, create_production_jit_engine,
    create_optimized_jit_interface, create_debug_jit_interface
};
use cursed::codegen::LlvmCodeGenerator;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jit_engine_creation_and_configuration() {
        let context = Context::create();
        
        // Test default engine creation
        let engine = CursedJitEngine::new_with_default_config(&context);
        assert!(engine.is_ok());
        
        let engine = engine.unwrap();
        let config = engine.get_config();
        assert_eq!(config.optimization_level, OptimizationLevel::Default);
        assert!(config.enable_function_cache);
        assert!(config.enable_performance_monitoring);
        assert_eq!(config.max_cached_functions, 1000);
    }

    #[test]
    fn test_jit_engine_factory_functions() {
        let context = Context::create();
        
        // Test optimized engine
        let optimized_engine = create_optimized_jit_engine(&context);
        assert!(optimized_engine.is_ok());
        
        // Test debug engine
        let debug_engine = create_debug_jit_engine(&context);
        assert!(debug_engine.is_ok());
        
        // Test production engine
        let production_engine = create_production_jit_engine(&context);
        assert!(production_engine.is_ok());
        
        let production_engine = production_engine.unwrap();
        let config = production_engine.get_config();
        assert_eq!(config.optimization_level, OptimizationLevel::Aggressive);
        assert!(!config.enable_performance_monitoring); // Disabled for performance
        assert_eq!(config.max_cached_functions, 10000);
    }

    #[test]
    fn test_basic_function_compilation_and_execution() {
        let context = Context::create();
        let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
        
        // Compile a simple function
        let result = engine.compile_function("test_function", "");
        assert!(result.is_ok());
        
        // Verify function exists
        assert!(engine.has_function("test_function"));
        
        // Execute the function
        let execution_result = engine.execute_function("test_function");
        assert!(execution_result.is_ok());
        assert_eq!(execution_result.unwrap(), 42); // Expected test value
        
        // Check statistics
        let stats = engine.get_stats();
        assert_eq!(stats.functions_compiled, 1);
        assert_eq!(stats.functions_executed, 1);
        assert!(stats.compilation_time_ms > 0);
    }

    #[test]
    fn test_function_caching() {
        let context = Context::create();
        let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
        
        // Compile function twice
        engine.compile_function("cached_function", "").unwrap();
        engine.compile_function("cached_function", "").unwrap();
        
        let stats = engine.get_stats();
        assert_eq!(stats.functions_compiled, 1); // Only compiled once
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.cache_misses, 1);
    }

    #[test]
    fn test_llvm_ir_parsing() {
        let context = Context::create();
        let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
        
        // Test with simple LLVM IR
        let llvm_ir = "define i32 @test_func() { ret i32 42 }";
        let result = engine.compile_function("ir_test", llvm_ir);
        assert!(result.is_ok());
        
        // Test execution
        let execution_result = engine.execute_function("ir_test");
        assert!(execution_result.is_ok());
        
        // Test with different return value
        let llvm_ir2 = "define i32 @test_func2() { ret i32 100 }";
        engine.compile_function("ir_test2", llvm_ir2).unwrap();
        let result2 = engine.execute_function("ir_test2");
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), 100);
    }

    #[test]
    fn test_memory_management() {
        let context = Context::create();
        let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
        
        // Compile multiple functions
        for i in 0..10 {
            let function_name = format!("func_{}", i);
            engine.compile_function(&function_name, "").unwrap();
        }
        
        let initial_memory = engine.get_memory_usage();
        assert!(initial_memory > 0);
        
        // Check function count
        assert!(engine.get_cached_function_count() > 0);
        
        // Clear cache
        engine.clear_cache().unwrap();
        
        // Memory should be managed properly (though functions may remain in execution engine)
        let stats = engine.get_stats();
        assert!(stats.memory_usage_bytes >= initial_memory);
    }

    #[test]
    fn test_configuration_updates() {
        let context = Context::create();
        let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
        
        // Test configuration updates
        engine.set_function_cache_enabled(false);
        assert!(!engine.get_config().enable_function_cache);
        
        engine.set_performance_monitoring_enabled(false);
        assert!(!engine.get_config().enable_performance_monitoring);
        
        engine.set_max_cached_functions(500);
        assert_eq!(engine.get_config().max_cached_functions, 500);
        
        // Test custom configuration
        let custom_config = JitEngineConfig {
            optimization_level: OptimizationLevel::Aggressive,
            enable_function_cache: true,
            enable_performance_monitoring: true,
            max_cached_functions: 2000,
            enable_debug_info: true,
            target_cpu: Some("native".to_string()),
            target_features: vec!["avx2".to_string()],
        };
        
        engine.update_config(custom_config);
        let config = engine.get_config();
        assert_eq!(config.optimization_level, OptimizationLevel::Aggressive);
        assert_eq!(config.max_cached_functions, 2000);
        assert!(config.enable_debug_info);
    }

    #[test]
    fn test_optimization_functionality() {
        let context = Context::create();
        let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
        
        // Compile and execute functions to build stats
        for i in 0..5 {
            let function_name = format!("opt_func_{}", i);
            engine.compile_function(&function_name, "").unwrap();
            engine.execute_function(&function_name).unwrap();
        }
        
        // Test optimization
        let result = engine.optimize_cached_functions();
        assert!(result.is_ok());
        
        let stats = engine.get_stats();
        assert!(stats.optimization_passes > 0);
    }

    #[test]
    fn test_jit_compilation_interface() {
        let context = Context::create();
        
        // Test interface creation
        let jit_interface = create_optimized_jit_interface(&context);
        assert!(jit_interface.is_ok());
        
        let mut interface = jit_interface.unwrap();
        
        // Test function compilation
        let result = interface.compile_function("interface_test", "slay test_func() { facts x = 42; }");
        assert!(interface.has_function("interface_test"));
        
        // Test function execution
        let execution_result = interface.execute_function("interface_test");
        assert!(execution_result.is_ok());
        
        // Test statistics
        let stats = interface.get_stats();
        assert!(stats.total_jit_compilations > 0);
        
        let engine_stats = interface.get_engine_stats();
        assert!(engine_stats.functions_compiled > 0);
    }

    #[test]
    fn test_hot_path_detection() {
        let context = Context::create();
        let jit_engine = create_debug_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        
        let mut config = cursed::codegen::llvm::JitCompilationConfig::default();
        config.hot_path_threshold = 3; // Low threshold for testing
        
        let mut interface = cursed::codegen::llvm::JitCompilationInterface::new(
            &context, jit_engine, codegen, config
        );
        
        // Compile and execute function multiple times
        interface.compile_function("hot_function", "").unwrap();
        
        for _ in 0..5 {
            interface.execute_function("hot_function").unwrap();
        }
        
        // Function should be identified as hot path
        let hot_paths = interface.get_hot_paths();
        assert!(hot_paths.contains(&"hot_function".to_string()));
        
        // Test optimization
        let optimized_count = interface.optimize_hot_paths().unwrap();
        assert!(optimized_count > 0);
        
        let stats = interface.get_stats();
        assert!(stats.hot_path_optimizations > 0);
    }

    #[test]
    fn test_repl_integration() {
        let context = Context::create();
        let jit_engine = create_optimized_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        let mut interface = cursed::codegen::llvm::JitCompilationInterface::new_with_default_config(
            &context, jit_engine, codegen
        );
        
        // Test REPL code execution
        let result = interface.execute_repl_code("facts x = 42;");
        assert!(result.is_ok());
        
        // Test function caching
        interface.compile_and_cache_function("repl_func", "slay test() { facts result = 100; }").unwrap();
        assert!(interface.has_function("repl_func"));
        
        // Test function listing
        let functions = interface.list_functions();
        assert!(functions.len() > 0);
        
        // Test function information
        let func_info = interface.get_function_info("repl_func");
        assert!(func_info.is_some());
    }

    #[test]
    fn test_performance_monitoring() {
        let context = Context::create();
        let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
        
        // Enable performance monitoring
        engine.set_performance_monitoring_enabled(true);
        
        // Compile and execute multiple functions
        for i in 0..10 {
            let function_name = format!("perf_test_{}", i);
            engine.compile_function(&function_name, "").unwrap();
            engine.execute_function(&function_name).unwrap();
        }
        
        let stats = engine.get_stats();
        assert_eq!(stats.functions_compiled, 10);
        assert_eq!(stats.functions_executed, 10);
        assert!(stats.compilation_time_ms > 0);
        assert!(stats.execution_time_ms > 0);
        assert!(stats.memory_usage_bytes > 0);
    }

    #[test]
    fn test_performance_report_generation() {
        let context = Context::create();
        let jit_engine = create_optimized_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        let mut interface = cursed::codegen::llvm::JitCompilationInterface::new_with_default_config(
            &context, jit_engine, codegen
        );
        
        // Generate some activity
        interface.compile_function("report_test1", "").unwrap();
        interface.compile_function("report_test2", "").unwrap();
        interface.execute_function("report_test1").unwrap();
        interface.execute_function("report_test2").unwrap();
        
        // Generate performance report
        let report = interface.generate_performance_report();
        assert!(report.contains("JIT Performance Report"));
        assert!(report.contains("Total compilations"));
        assert!(report.contains("Function Details"));
        assert!(report.contains("report_test1"));
        assert!(report.contains("report_test2"));
    }

    #[test]
    fn test_function_profiling() {
        let context = Context::create();
        let jit_engine = create_debug_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        let mut interface = cursed::codegen::llvm::JitCompilationInterface::new_with_default_config(
            &context, jit_engine, codegen
        );
        
        // Compile function for profiling
        interface.compile_function("profile_test", "").unwrap();
        
        // Profile function execution
        let avg_time = interface.profile_function_execution("profile_test", 5);
        assert!(avg_time.is_ok());
        assert!(avg_time.unwrap() > Duration::ZERO);
    }

    #[test]
    fn test_error_handling() {
        let context = Context::create();
        let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
        
        // Test execution of non-existent function
        let result = engine.execute_function("nonexistent_function");
        assert!(result.is_err());
        
        // Test getting non-existent function
        let func_result = engine.get_function("nonexistent_function");
        assert!(func_result.is_err());
        
        // Test removing non-existent function (should succeed gracefully)
        let remove_result = engine.remove_function("nonexistent_function");
        assert!(remove_result.is_ok());
    }

    #[test]
    fn test_memory_limits_and_cleanup() {
        let context = Context::create();
        let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
        
        // Set a low function limit for testing
        engine.set_max_cached_functions(5);
        
        // Compile more functions than the limit
        for i in 0..10 {
            let function_name = format!("limit_test_{}", i);
            engine.compile_function(&function_name, "").unwrap();
        }
        
        // Cache should be limited
        let cached_count = engine.get_cached_function_count();
        assert!(cached_count <= 5);
        
        // Test cleanup
        engine.clear_cache().unwrap();
        
        // Reset statistics
        engine.reset_stats();
        let stats = engine.get_stats();
        assert_eq!(stats.functions_compiled, 0);
        assert_eq!(stats.functions_executed, 0);
    }

    #[test]
    fn test_concurrent_compilation() {
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        let context = Context::create();
        let engine = Arc::new(Mutex::new(
            CursedJitEngine::new_with_default_config(&context).unwrap()
        ));
        
        let mut handles = vec![];
        
        // Spawn multiple threads to test concurrent access
        for i in 0..5 {
            let engine_clone = Arc::clone(&engine);
            let handle = thread::spawn(move || {
                let mut engine = engine_clone.lock().unwrap();
                let function_name = format!("concurrent_test_{}", i);
                engine.compile_function(&function_name, "").unwrap();
                engine.execute_function(&function_name).unwrap()
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            let result = handle.join().unwrap();
            assert_eq!(result, 42); // Expected test value
        }
        
        // Verify all functions were compiled
        let engine = engine.lock().unwrap();
        let stats = engine.get_stats();
        assert_eq!(stats.functions_compiled, 5);
        assert_eq!(stats.functions_executed, 5);
    }

    #[test]
    fn test_debug_vs_production_configurations() {
        let context = Context::create();
        
        // Test debug configuration
        let debug_engine = create_debug_jit_engine(&context).unwrap();
        let debug_config = debug_engine.get_config();
        assert_eq!(debug_config.optimization_level, OptimizationLevel::None);
        assert!(debug_config.enable_debug_info);
        assert_eq!(debug_config.max_cached_functions, 100);
        
        // Test production configuration
        let production_engine = create_production_jit_engine(&context).unwrap();
        let production_config = production_engine.get_config();
        assert_eq!(production_config.optimization_level, OptimizationLevel::Aggressive);
        assert!(!production_config.enable_debug_info);
        assert!(!production_config.enable_performance_monitoring);
        assert_eq!(production_config.max_cached_functions, 10000);
        assert_eq!(production_config.target_cpu, Some("native".to_string()));
        assert!(production_config.target_features.contains(&"sse4.2".to_string()));
    }
}
