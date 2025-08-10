//! Unit tests for goroutine context switching implementation

#[cfg(test)]
mod tests {
    use super::super::goroutine_context::*;
    use crate::error::CursedError;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_context_system_initialization() {
        let result = initialize_goroutine_context_system();
        assert!(result.is_ok(), "Context system should initialize successfully");
    }

    #[test]
    fn test_executable_function_registration() {
        let func = ExecutableFunction {
            func_ptr: test_function as *const () as usize,
            name: "test_function".to_string(),
            arity: 0,
            return_type: "drip".to_string(),
            param_types: vec![],
            is_native: true,
            jit_metadata: None,
        };

        let result = register_executable_function("test_function".to_string(), func);
        assert!(result.is_ok(), "Function registration should succeed");

        let retrieved = get_executable_function("test_function");
        assert!(retrieved.is_some(), "Registered function should be retrievable");
        assert_eq!(retrieved.unwrap().name, "test_function");
    }

    #[test]
    fn test_context_save_and_restore() {
        let goroutine_id = 1;
        
        // Save context
        let save_result = save_goroutine_context(goroutine_id);
        assert!(save_result.is_ok(), "Context save should succeed");

        // Restore context
        let restore_result = restore_goroutine_context(goroutine_id);
        assert!(restore_result.is_ok(), "Context restore should succeed");

        // Cleanup
        let cleanup_result = cleanup_goroutine_context(goroutine_id);
        assert!(cleanup_result.is_ok(), "Context cleanup should succeed");
    }

    #[test]
    fn test_context_system_stats() {
        let stats_result = get_context_system_stats();
        assert!(stats_result.is_ok(), "Stats should be retrievable");

        let stats = stats_result.unwrap();
        assert!(stats.registered_functions >= 0, "Function count should be non-negative");
        assert!(stats.active_contexts >= 0, "Context count should be non-negative");
    }

    #[test]
    fn test_context_switching_between_goroutines() {
        let goroutine_a = 100;
        let goroutine_b = 101;

        // Save context for goroutine A
        let save_a = save_goroutine_context(goroutine_a);
        assert!(save_a.is_ok(), "Should save context for goroutine A");

        // Save context for goroutine B  
        let save_b = save_goroutine_context(goroutine_b);
        assert!(save_b.is_ok(), "Should save context for goroutine B");

        // Switch from A to B
        let switch_result = switch_goroutine_context(goroutine_a, goroutine_b);
        assert!(switch_result.is_ok(), "Context switch should succeed");

        // Cleanup
        let _ = cleanup_goroutine_context(goroutine_a);
        let _ = cleanup_goroutine_context(goroutine_b);
    }

    #[cfg(target_arch = "wasm32")]
    #[test]
    fn test_wasm_specific_context() {
        let init_result = initialize_wasm_context_system();
        assert!(init_result.is_ok(), "WASM context system should initialize");

        let (stack_base, stack_size) = allocate_wasm_stack(4096).unwrap();
        assert!(stack_base > 0, "Stack should be allocated");
        assert_eq!(stack_size, 4096, "Stack size should match request");

        let goroutine_id = 200;
        let set_result = set_current_wasm_goroutine(goroutine_id);
        assert!(set_result.is_ok(), "Should set current WASM goroutine");
    }

    #[test]
    fn test_function_execution() {
        // Register a test function
        let func = ExecutableFunction {
            func_ptr: test_add_function as *const () as usize,
            name: "test_add".to_string(),
            arity: 2,
            return_type: "drip".to_string(),
            param_types: vec!["drip".to_string(), "drip".to_string()],
            is_native: true,
            jit_metadata: None,
        };

        let reg_result = register_executable_function("test_add".to_string(), func);
        assert!(reg_result.is_ok(), "Function registration should succeed");

        // Execute the function
        let exec_result = execute_function_value("test_add", &[5, 3]);
        assert!(exec_result.is_ok(), "Function execution should succeed");
        assert_eq!(exec_result.unwrap(), 8, "Function should return correct result");
    }

    #[test]
    fn test_concurrent_context_operations() {
        let num_threads = 4;
        let num_goroutines_per_thread = 10;
        let handles: Vec<_> = (0..num_threads).map(|thread_id| {
            thread::spawn(move || {
                for i in 0..num_goroutines_per_thread {
                    let goroutine_id = (thread_id * num_goroutines_per_thread + i) as u64;
                    
                    // Save context
                    let save_result = save_goroutine_context(goroutine_id);
                    assert!(save_result.is_ok(), "Concurrent save should succeed");
                    
                    // Small delay to increase contention
                    thread::sleep(Duration::from_millis(1));
                    
                    // Restore context
                    let restore_result = restore_goroutine_context(goroutine_id);
                    assert!(restore_result.is_ok(), "Concurrent restore should succeed");
                    
                    // Cleanup
                    let cleanup_result = cleanup_goroutine_context(goroutine_id);
                    assert!(cleanup_result.is_ok(), "Concurrent cleanup should succeed");
                }
            })
        }).collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }
    }

    #[test]
    fn test_cross_platform_context_abstraction() {
        // This test uses the cross-platform abstraction layer
        use crate::runtime::context_abstraction::*;
        
        let goroutine_id = 300;
        
        // Save using cross-platform API
        let save_result = save_goroutine_context_cross_platform(goroutine_id);
        assert!(save_result.is_ok(), "Cross-platform save should succeed");
        
        // Restore using cross-platform API
        let restore_result = restore_goroutine_context_cross_platform(goroutine_id);
        assert!(restore_result.is_ok(), "Cross-platform restore should succeed");
    }

    // Helper test functions
    extern "C" fn test_function() -> usize {
        42
    }

    extern "C" fn test_add_function(a: usize, b: usize) -> usize {
        a + b
    }
}
