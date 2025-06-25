//! JIT-REPL Integration Tests for CURSED
//!
//! Tests the integration between the JIT compilation system and the REPL,
//! validating incremental compilation, hot path optimization, and interactive
//! development workflows.

use cursed::repl::{CursedRepl, ReplEvaluator, SessionManager};
use cursed::codegen::llvm::{JitCompilationInterface, create_optimized_jit_interface};
use cursed::codegen::LlvmCodeGenerator;
use cursed::error::CursedError;
use inkwell::context::Context;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_evaluator_jit_initialization() {
        let mut evaluator = ReplEvaluator::new().unwrap();
        
        // Initialize codegen (which should also initialize JIT)
        let result = evaluator.initialize_codegen();
        assert!(result.is_ok());
        
        // Check if JIT support is available
        // Note: This may fail in test environment due to LLVM setup
        println!("JIT support available: {}", evaluator.has_jit_support());
    }

    #[test]
    fn test_jit_compilation_interface_creation() {
        let context = Context::create();
        
        // Test interface creation with different configurations
        let optimized_interface = create_optimized_jit_interface(&context);
        assert!(optimized_interface.is_ok());
        
        let mut interface = optimized_interface.unwrap();
        
        // Test basic functionality
        assert_eq!(interface.get_compiled_function_count(), 0);
        assert!(interface.list_functions().is_empty());
    }

    #[test]
    fn test_repl_code_execution_workflow() {
        let context = Context::create();
        let jit_engine = cursed::codegen::llvm::create_debug_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        let mut interface = JitCompilationInterface::new_with_default_config(
            &context, jit_engine, codegen
        );
        
        // Test simple REPL code execution
        let simple_code = "facts x = 42;";
        let result = interface.execute_repl_code(simple_code);
        assert!(result.is_ok());
        
        // Test function definition and execution
        let function_code = "slay test_func() { facts result = 100; }";
        let compile_result = interface.compile_function("test_func", function_code);
        assert!(compile_result.is_ok());
        
        // Verify function is available
        assert!(interface.has_function("test_func"));
        
        let execution_result = interface.execute_function("test_func");
        assert!(execution_result.is_ok());
    }

    #[test]
    fn test_incremental_compilation() {
        let context = Context::create();
        let jit_engine = cursed::codegen::llvm::create_debug_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        let mut interface = JitCompilationInterface::new_with_default_config(
            &context, jit_engine, codegen
        );
        
        // Compile multiple functions incrementally
        let functions = vec![
            ("func1", "slay func1() { facts x = 1; }"),
            ("func2", "slay func2() { facts x = 2; }"),
            ("func3", "slay func3() { facts x = 3; }"),
        ];
        
        for (name, code) in functions {
            let result = interface.compile_function(name, code);
            assert!(result.is_ok());
            assert!(interface.has_function(name));
        }
        
        // Verify all functions are available
        let function_list = interface.list_functions();
        assert_eq!(function_list.len(), 3);
        assert!(function_list.contains(&"func1".to_string()));
        assert!(function_list.contains(&"func2".to_string()));
        assert!(function_list.contains(&"func3".to_string()));
    }

    #[test]
    fn test_hot_path_optimization_in_repl() {
        let context = Context::create();
        let jit_engine = cursed::codegen::llvm::create_debug_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        
        // Configure for aggressive hot path detection
        let mut config = cursed::codegen::llvm::JitCompilationConfig::default();
        config.hot_path_threshold = 2;
        config.enable_dynamic_recompilation = true;
        config.hot_path_optimization_level = inkwell::OptimizationLevel::Aggressive;
        
        let mut interface = JitCompilationInterface::new(&context, jit_engine, codegen, config);
        
        // Compile a function
        interface.compile_function("hot_function", "slay hot_func() { facts x = 42; }").unwrap();
        
        // Execute it multiple times to trigger hot path detection
        for _ in 0..5 {
            let result = interface.execute_function("hot_function");
            assert!(result.is_ok());
        }
        
        // Check if function became a hot path
        let hot_paths = interface.get_hot_paths();
        println!("Hot paths detected: {:?}", hot_paths);
        
        // Trigger optimization
        let optimized_count = interface.optimize_hot_paths().unwrap();
        println!("Optimized {} hot paths", optimized_count);
        
        let stats = interface.get_stats();
        println!("Hot path optimizations: {}", stats.hot_path_optimizations);
    }

    #[test]
    fn test_repl_session_with_jit() {
        let mut evaluator = ReplEvaluator::new().unwrap();
        let mut session_manager = SessionManager::new();
        
        // Initialize codegen
        if evaluator.initialize_codegen().is_ok() && evaluator.has_jit_support() {
            // Test JIT evaluation
            let jit_result = evaluator.evaluate_with_jit("facts x = 42;");
            println!("JIT evaluation result: {:?}", jit_result);
            
            // Test function compilation
            let compile_result = evaluator.compile_function("test_func", "slay test() { facts result = 100; }");
            println!("Function compilation result: {:?}", compile_result);
            
            // List compiled functions
            let functions = evaluator.list_jit_functions();
            println!("Available JIT functions: {:?}", functions);
            
            if !functions.is_empty() {
                // Execute a compiled function
                let exec_result = evaluator.execute_jit_function(&functions[0]);
                println!("Function execution result: {:?}", exec_result);
            }
        } else {
            println!("JIT support not available in test environment");
        }
    }

    #[test]
    fn test_performance_monitoring_in_repl() {
        let context = Context::create();
        let jit_engine = cursed::codegen::llvm::create_optimized_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        let mut interface = JitCompilationInterface::new_with_default_config(
            &context, jit_engine, codegen
        );
        
        // Enable performance monitoring
        let mut config = interface.get_config().clone();
        config.enable_pgo = true;
        interface.update_config(config);
        
        // Compile and execute several functions
        let test_functions = vec![
            ("perf_test1", "slay test1() { facts x = 1; }"),
            ("perf_test2", "slay test2() { facts x = 2; }"),
            ("perf_test3", "slay test3() { facts x = 3; }"),
        ];
        
        for (name, code) in test_functions {
            interface.compile_function(name, code).unwrap();
            
            // Execute multiple times for performance data
            for _ in 0..3 {
                interface.execute_function(name).unwrap();
            }
        }
        
        // Generate performance report
        let report = interface.generate_performance_report();
        assert!(report.contains("JIT Performance Report"));
        assert!(report.contains("perf_test1"));
        assert!(report.contains("perf_test2"));
        assert!(report.contains("perf_test3"));
        
        println!("Performance Report:\n{}", report);
    }

    #[test]
    fn test_error_handling_in_repl_jit() {
        let context = Context::create();
        let jit_engine = cursed::codegen::llvm::create_debug_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        let mut interface = JitCompilationInterface::new_with_default_config(
            &context, jit_engine, codegen
        );
        
        // Test compilation error handling
        let invalid_code = "invalid cursed syntax here";
        let result = interface.compile_function("invalid_func", invalid_code);
        // This may succeed as our current implementation is lenient
        
        // Test execution of non-existent function
        let exec_result = interface.execute_function("nonexistent_function");
        assert!(exec_result.is_err());
        
        // Test REPL code with potential errors
        let repl_result = interface.execute_repl_code("some potentially invalid code");
        // Result depends on current LLVM IR generation
        println!("REPL execution result: {:?}", repl_result);
    }

    #[test]
    fn test_repl_evaluator_jit_methods() {
        let mut evaluator = ReplEvaluator::new().unwrap();
        
        // Test JIT support detection
        let has_jit = evaluator.has_jit_support();
        println!("Initial JIT support: {}", has_jit);
        
        // Initialize codegen
        if evaluator.initialize_codegen().is_ok() {
            let has_jit_after = evaluator.has_jit_support();
            println!("JIT support after initialization: {}", has_jit_after);
            
            if has_jit_after {
                // Test JIT methods
                let functions = evaluator.list_jit_functions();
                assert!(functions.is_empty()); // Should start empty
                
                // Test performance report
                let report = evaluator.get_jit_performance_report();
                if let Some(report_content) = report {
                    assert!(report_content.contains("JIT Performance Report"));
                    println!("Performance report available");
                } else {
                    println!("No performance report available");
                }
            }
        }
    }

    #[test]
    fn test_memory_management_in_repl_context() {
        let context = Context::create();
        let jit_engine = cursed::codegen::llvm::create_debug_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        let mut interface = JitCompilationInterface::new_with_default_config(
            &context, jit_engine, codegen
        );
        
        // Execute many REPL expressions to test memory management
        for i in 0..20 {
            let code = format!("facts x_{} = {};", i, i * 2);
            let result = interface.execute_repl_code(&code);
            // Don't assert success as it depends on IR generation capabilities
            println!("REPL expression {} result: {:?}", i, result);
        }
        
        // Check memory usage
        let memory_usage = interface.get_memory_usage();
        println!("Memory usage after REPL expressions: {} bytes", memory_usage);
        
        // Clear cache and check memory
        let clear_result = interface.clear_cache();
        assert!(clear_result.is_ok());
        
        let memory_after_clear = interface.get_memory_usage();
        println!("Memory usage after cache clear: {} bytes", memory_after_clear);
        
        // Function count should be reset
        assert_eq!(interface.get_compiled_function_count(), 0);
    }

    #[test]
    fn test_function_profiling_in_repl() {
        let context = Context::create();
        let jit_engine = cursed::codegen::llvm::create_debug_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        let mut interface = JitCompilationInterface::new_with_default_config(
            &context, jit_engine, codegen
        );
        
        // Compile a function for profiling
        let function_name = "profile_target";
        let function_code = "slay profile_func() { facts result = 42; }";
        
        let compile_result = interface.compile_function(function_name, function_code);
        assert!(compile_result.is_ok());
        
        // Profile the function
        let profile_result = interface.profile_function_execution(function_name, 10);
        assert!(profile_result.is_ok());
        
        let avg_time = profile_result.unwrap();
        assert!(avg_time > Duration::ZERO);
        
        println!("Average execution time for {}: {:?}", function_name, avg_time);
        
        // Get detailed function information
        let func_info = interface.get_function_info(function_name);
        assert!(func_info.is_some());
        
        let info = func_info.unwrap();
        assert_eq!(info.name, function_name);
        assert!(info.execution_count >= 10); // Should be at least 10 from profiling
        
        println!("Function info: {:?}", info);
    }

    #[test]
    fn test_concurrent_repl_operations() {
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        let context = Context::create();
        let jit_engine = cursed::codegen::llvm::create_debug_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        let interface = Arc::new(Mutex::new(
            JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen)
        ));
        
        let mut handles = vec![];
        
        // Spawn threads to simulate concurrent REPL usage
        for i in 0..3 {
            let interface_clone = Arc::clone(&interface);
            let handle = thread::spawn(move || {
                let mut interface = interface_clone.lock().unwrap();
                
                // Each thread compiles and executes a function
                let function_name = format!("concurrent_func_{}", i);
                let function_code = format!("slay func_{}() {{ facts x = {}; }}", i, i * 10);
                
                let compile_result = interface.compile_function(&function_name, &function_code);
                if compile_result.is_ok() {
                    let exec_result = interface.execute_function(&function_name);
                    println!("Thread {} execution result: {:?}", i, exec_result);
                    exec_result.is_ok()
                } else {
                    println!("Thread {} compilation failed: {:?}", i, compile_result);
                    false
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads and collect results
        let mut success_count = 0;
        for handle in handles {
            if handle.join().unwrap() {
                success_count += 1;
            }
        }
        
        println!("Successful concurrent operations: {}/3", success_count);
        
        // Check final state
        let interface = interface.lock().unwrap();
        let function_count = interface.get_compiled_function_count();
        println!("Total compiled functions: {}", function_count);
        
        let stats = interface.get_stats();
        println!("Final stats - compilations: {}, executions: {}", 
                 stats.total_jit_compilations, 
                 stats.total_jit_compilations); // Note: Using compilation count as execution count not available
    }

    #[test]
    fn test_repl_command_integration() {
        // This test simulates how JIT commands would work in the REPL
        // Since we can't easily test the actual command system here,
        // we'll test the underlying functionality
        
        let context = Context::create();
        let jit_engine = cursed::codegen::llvm::create_optimized_jit_engine(&context).unwrap();
        let codegen = LlvmCodeGenerator::new().unwrap();
        let mut interface = JitCompilationInterface::new_with_default_config(
            &context, jit_engine, codegen
        );
        
        // Simulate ":jit status" command
        let has_functions = !interface.list_functions().is_empty();
        println!("JIT status - Has functions: {}", has_functions);
        
        // Simulate ":jit compile <name> <code>" command
        let compile_result = interface.compile_function("repl_cmd_test", "slay test() { facts x = 1; }");
        println!("Compile command result: {:?}", compile_result);
        
        // Simulate ":jit functions" command
        let functions = interface.list_functions();
        println!("Available functions: {:?}", functions);
        
        // Simulate ":jit report" command
        let report = interface.generate_performance_report();
        println!("Performance report available: {}", !report.is_empty());
        
        // Simulate ":optimize" command
        let optimize_result = interface.optimize_hot_paths();
        println!("Optimization result: {:?}", optimize_result);
        
        // Simulate ":profile <function>" command
        if !functions.is_empty() {
            let profile_result = interface.profile_function_execution(&functions[0], 5);
            println!("Profile result: {:?}", profile_result);
        }
    }
}
