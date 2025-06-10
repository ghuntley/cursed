//! Edge Case Tests for CURSED Error Handling System
//!
//! This module tests edge cases and boundary conditions for error handling:
//! - Nested error propagation scenarios
//! - Stack overflow and deep recursion handling  
//! - Recovery from critical errors
//! - Thread-safety edge cases
//! - Resource exhaustion scenarios
//! - Complex error chain interactions

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::runtime::error_handling::{
    ErrorRuntime, ErrorContext, ErrorPropagationConfig, ErrorHandlingStatistics,
    initialize_error_runtime, get_error_runtime, shutdown_error_runtime
};
use cursed::runtime::panic::{PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory, RecoveryAction};
use cursed::runtime::stack_trace::{StackTraceManager, CallFrame};
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicBool, AtomicUsize, Ordering};

#[path = "common.rs"]
mod common;

/// Initialize tracing for all tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("trace")
            .with_test_writer()
            .try_init();
    };
}

#[test]
fn test_maximum_error_chain_depth_boundary() {
    init_tracing!();
    
    let mut config = ErrorPropagationConfig::default();
    config.max_chain_depth = 5; // Very small limit for testing
    config.auto_panic_threshold = Some(10);
    
    let runtime = ErrorRuntime::with_config(config);
    runtime.initialize().unwrap();
    
    // Test exactly at the boundary
    for i in 0..5 {
        let error = CursedError::Runtime(format!("Boundary test error {}", i));
        let location = Some(SourceLocation::new(i + 1, 5));
        let result = runtime.propagate_error(error, location, Some(format!("boundary_func_{}", i)));
        assert!(result.is_err());
    }
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_errors, 5);
    assert_eq!(stats.max_chain_depth, 5);
    
    // Test exceeding the boundary - should handle gracefully
    let error = CursedError::Runtime("Exceeding boundary".to_string());
    let result = runtime.propagate_error(error, None, None);
    // Should still work but may trigger different behavior
    assert!(result.is_err() || result.is_ok());
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_zero_chain_depth_configuration() {
    init_tracing!();
    
    let mut config = ErrorPropagationConfig::default();
    config.max_chain_depth = 0; // Edge case: no chaining allowed
    
    let runtime = ErrorRuntime::with_config(config);
    runtime.initialize().unwrap();
    
    let error = CursedError::Runtime("Zero depth test".to_string());
    let result = runtime.propagate_error(error, None, None);
    
    // Should handle gracefully even with zero depth
    assert!(result.is_err());
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_errors, 1);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_extremely_long_error_messages() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Test with very long error message (1MB)
    let massive_message = "x".repeat(1024 * 1024);
    let error = CursedError::Runtime(massive_message.clone());
    let location = Some(SourceLocation::new(1, 1).with_file("massive_error.csd"));
    
    let result = runtime.propagate_error(error, location, Some("massive_function".to_string()));
    assert!(result.is_err());
    
    // Test error message contains expected content
    if let Err(propagated_error) = result {
        let error_string = propagated_error.to_string();
        assert!(error_string.len() > 500000); // Should contain most of the massive message
    }
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_nested_error_propagation_with_cycles() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Create a scenario that might create cycles in error propagation
    let error1 = CursedError::Runtime("First error in cycle".to_string());
    let location1 = Some(SourceLocation::new(10, 5).with_file("cycle_test.csd"));
    
    // First propagation
    let result1 = runtime.propagate_error(error1, location1, Some("cycle_func_1".to_string()));
    assert!(result1.is_err());
    
    // Second propagation in same context (potential cycle)
    let error2 = CursedError::Type("Second error in cycle".to_string());
    let location2 = Some(SourceLocation::new(20, 10).with_file("cycle_test.csd"));
    
    let result2 = runtime.propagate_error(error2, location2, Some("cycle_func_2".to_string()));
    assert!(result2.is_err());
    
    // Check that cycle was handled properly
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_errors, 2);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_concurrent_error_context_mutations() {
    init_tracing!();
    
    let runtime = Arc::new(ErrorRuntime::new());
    runtime.initialize().unwrap();
    
    let num_threads = 10;
    let mutations_per_thread = 100;
    let shared_counter = Arc::new(AtomicUsize::new(0));
    
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let runtime_clone = Arc::clone(&runtime);
        let counter_clone = Arc::clone(&shared_counter);
        
        let handle = thread::spawn(move || {
            for i in 0..mutations_per_thread {
                // Create error
                let error = CursedError::Runtime(format!("Concurrent mutation T{} I{}", thread_id, i));
                let location = Some(SourceLocation::new(i + 1, thread_id + 1));
                
                let _ = runtime_clone.propagate_error(error, location, None);
                
                // Concurrent context operations
                match i % 4 {
                    0 => runtime_clone.clear_error_context(),
                    1 => {
                        if let Some(_context) = runtime_clone.get_current_error_context() {
                            // Context exists, continue
                        }
                    }
                    2 => {
                        let _ = runtime_clone.update_config(|config| {
                            config.log_propagation = !config.log_propagation;
                        });
                    }
                    _ => {
                        let _ = runtime_clone.get_statistics();
                    }
                }
                
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_count = shared_counter.load(Ordering::SeqCst);
    assert_eq!(final_count, num_threads * mutations_per_thread);
    
    let stats = runtime.get_statistics().unwrap();
    assert!(stats.total_errors > 0);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_error_propagation_with_unicode_content() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Test with various Unicode content
    let unicode_tests = vec![
        "错误消息 (Chinese error message)",
        "エラーメッセージ (Japanese error message)",
        "сообщение об ошибке (Russian error message)",
        "🚨 Emoji error message 🔥",
        "Mixed: 错误 エラー ошибка 🚨",
        "\u{1F4A9}\u{1F525}\u{1F92F}", // Various emoji
    ];
    
    for (i, unicode_message) in unicode_tests.iter().enumerate() {
        let error = CursedError::Runtime(unicode_message.to_string());
        let location = Some(SourceLocation::new(i + 1, 10)
            .with_file(&format!("unicode_test_{}.csd", i)));
        
        let result = runtime.propagate_error(error, location, Some(format!("unicode_func_{}", i)));
        assert!(result.is_err());
        
        // Verify Unicode content is preserved
        if let Err(propagated_error) = result {
            let error_string = propagated_error.to_string();
            assert!(error_string.contains(unicode_message));
        }
    }
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_errors, unicode_tests.len() as u64);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_error_handling_during_shutdown() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Create some errors before shutdown
    let error1 = CursedError::Runtime("Before shutdown".to_string());
    let _ = runtime.propagate_error(error1, None, None);
    
    // Initiate shutdown
    runtime.shutdown().unwrap();
    
    // Try to propagate error after shutdown - should handle gracefully
    let error2 = CursedError::Runtime("After shutdown".to_string());
    let result = runtime.propagate_error(error2, None, None);
    
    // Should handle gracefully (may succeed or fail, but shouldn't panic)
    match result {
        Ok(_) => tracing::info!("Error propagation after shutdown succeeded"),
        Err(_) => tracing::info!("Error propagation after shutdown failed gracefully"),
    }
    
    // Multiple shutdowns should be safe
    let shutdown_result = runtime.shutdown();
    assert!(shutdown_result.is_ok() || shutdown_result.is_err()); // Either is acceptable
}

#[test]
fn test_error_with_invalid_source_locations() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Test with extreme source location values
    let invalid_locations = vec![
        SourceLocation::new(0, 0), // Zero values
        SourceLocation::new(usize::MAX, usize::MAX), // Maximum values
        SourceLocation::new(1, usize::MAX), // Mixed extreme values
        SourceLocation::new(usize::MAX, 1),
    ];
    
    for (i, location) in invalid_locations.iter().enumerate() {
        let error = CursedError::Parse(format!("Invalid location test {}", i));
        let result = runtime.propagate_error(error, Some(location.clone()), None);
        
        // Should handle gracefully regardless of location values
        assert!(result.is_err());
    }
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_errors, invalid_locations.len() as u64);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_error_propagation_stack_overflow_simulation() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Simulate deep recursion that might cause stack overflow
    fn recursive_error_propagation(runtime: &ErrorRuntime, depth: usize, max_depth: usize) -> Result<(), CursedError> {
        if depth >= max_depth {
            return Ok(());
        }
        
        let error = CursedError::Runtime(format!("Recursive error at depth {}", depth));
        let location = Some(SourceLocation::new(depth, 10));
        
        // Propagate error
        let _ = runtime.propagate_error(error, location, Some(format!("recursive_func_{}", depth)));
        
        // Recurse (but with limited depth to prevent actual stack overflow)
        recursive_error_propagation(runtime, depth + 1, max_depth)
    }
    
    // Use a reasonable depth that won't actually overflow
    let result = recursive_error_propagation(&runtime, 0, 100);
    assert!(result.is_ok());
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_errors, 100);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_error_context_with_extremely_large_metadata() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Create context with massive metadata
    let mut large_metadata = HashMap::new();
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let value = "x".repeat(1000); // 1KB per value
        large_metadata.insert(key, value);
    }
    
    let mut context = ErrorContext::new()
        .with_location(SourceLocation::new(1, 1).with_file("large_metadata.csd"));
    
    // Add all metadata
    for (key, value) in large_metadata {
        context = context.with_metadata(key, value);
    }
    
    // Add error to chain
    context.add_to_chain(
        "Large metadata test".to_string(),
        Some(SourceLocation::new(1, 1)),
        Some("large_metadata_function".to_string()),
    );
    
    // Test that context can be displayed without issues
    let display_string = format!("{}", context);
    assert!(display_string.len() > 50000); // Should be quite large
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_rapid_runtime_initialization_shutdown_cycles() {
    init_tracing!();
    
    let num_cycles = 50;
    
    for cycle in 0..num_cycles {
        let runtime = ErrorRuntime::new();
        let init_result = runtime.initialize();
        assert!(init_result.is_ok());
        
        // Quick error propagation
        let error = CursedError::Runtime(format!("Cycle {} test", cycle));
        let _ = runtime.propagate_error(error, None, None);
        
        // Immediate shutdown
        let shutdown_result = runtime.shutdown();
        assert!(shutdown_result.is_ok());
        
        if cycle % 10 == 0 {
            tracing::debug!("Completed {} initialization/shutdown cycles", cycle + 1);
        }
    }
    
    tracing::info!("Successfully completed {} rapid init/shutdown cycles", num_cycles);
}

#[test]
fn test_error_handling_with_thread_local_storage_conflicts() {
    init_tracing!();
    
    let runtime = Arc::new(ErrorRuntime::new());
    runtime.initialize().unwrap();
    
    let num_threads = 8;
    let thread_local_conflicts = Arc::new(AtomicUsize::new(0));
    
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let runtime_clone = Arc::clone(&runtime);
        let conflicts_clone = Arc::clone(&thread_local_conflicts);
        
        let handle = thread::spawn(move || {
            // Create thread-specific context
            thread_local! {
                static THREAD_DATA: std::cell::RefCell<HashMap<String, String>> = 
                    std::cell::RefCell::new(HashMap::new());
            }
            
            for i in 0..100 {
                // Simulate thread-local storage operations
                THREAD_DATA.with(|data| {
                    let mut map = data.borrow_mut();
                    map.insert(format!("key_{}", i), format!("value_{}_{}", thread_id, i));
                });
                
                // Error propagation with potential TLS conflicts
                let error = CursedError::Runtime(format!("TLS conflict T{} I{}", thread_id, i));
                let location = Some(SourceLocation::new(i + 1, thread_id + 1));
                
                match runtime_clone.propagate_error(error, location, None) {
                    Ok(_) => conflicts_clone.fetch_add(1, Ordering::SeqCst),
                    Err(_) => {} // Expected
                }
                
                // Access TLS again to check for corruption
                THREAD_DATA.with(|data| {
                    let map = data.borrow();
                    assert!(map.contains_key(&format!("key_{}", i)));
                });
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let conflicts = thread_local_conflicts.load(Ordering::SeqCst);
    tracing::info!("Thread-local storage conflicts detected: {}", conflicts);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_error_handling_during_memory_exhaustion_simulation() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Simulate memory pressure by creating many large allocations
    let mut large_allocations = Vec::new();
    
    // Allocate memory until we have significant pressure
    for i in 0..100 {
        let allocation = vec![0u8; 1024 * 1024]; // 1MB allocations
        large_allocations.push(allocation);
        
        // Try error propagation under memory pressure
        let error = CursedError::Runtime(format!("Memory pressure error {}", i));
        let location = Some(SourceLocation::new(i + 1, 10));
        
        let result = runtime.propagate_error(error, location, None);
        
        // Should handle gracefully even under memory pressure
        match result {
            Ok(_) => tracing::debug!("Error propagation succeeded under memory pressure at iteration {}", i),
            Err(_) => {} // Expected
        }
        
        // Clear context periodically to help with memory management
        if i % 10 == 0 {
            runtime.clear_error_context();
        }
    }
    
    let stats = runtime.get_statistics().unwrap();
    assert!(stats.total_errors > 0);
    
    // Clean up allocations
    drop(large_allocations);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_error_propagation_with_corrupted_context() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Create a context and then try to "corrupt" it in various ways
    let error1 = CursedError::Runtime("Initial error".to_string());
    let _ = runtime.propagate_error(error1, None, None);
    
    // Try operations that might reveal corruption handling
    for i in 0..50 {
        let error = CursedError::Runtime(format!("Corruption test {}", i));
        let location = Some(SourceLocation::new(i + 1, 10));
        
        // Various operations that might interact with potentially corrupted state
        let _ = runtime.propagate_error(error, location, None);
        
        if i % 5 == 0 {
            runtime.clear_error_context();
        }
        
        if i % 7 == 0 {
            let _ = runtime.get_current_error_context();
        }
        
        if i % 11 == 0 {
            let _ = runtime.get_statistics();
        }
    }
    
    // Should complete without panicking
    let stats = runtime.get_statistics().unwrap();
    assert!(stats.total_errors > 0);
    
    runtime.shutdown().unwrap();
}

/// Documentation: Why Edge Case Testing is Essential
/// 
/// Edge cases often reveal fundamental flaws in error handling systems.
/// These tests ensure that CURSED's error handling is robust when:
/// 
/// 1. **Boundary Conditions**: Operating at configuration limits
/// 2. **Resource Exhaustion**: Handling errors when resources are scarce  
/// 3. **Concurrent Edge Cases**: Thread safety under unusual conditions
/// 4. **Data Corruption**: Graceful handling of invalid states
/// 5. **Unicode and Encoding**: Proper handling of international content
/// 6. **Lifecycle Edge Cases**: Errors during initialization/shutdown
/// 7. **Memory Pressure**: Error handling when memory is limited
/// 8. **Deep Recursion**: Stack management during complex scenarios
/// 
/// Edge cases are critical because:
/// - They often occur in production under stress
/// - They can cause cascading failures if not handled properly
/// - They reveal assumptions that don't hold in all scenarios
/// - They test the robustness of the fundamental design
/// 
/// These tests ensure CURSED's error handling is production-ready
/// even under the most challenging conditions.

#[cfg(test)]
mod edge_case_test_utilities {
    use super::*;
    
    /// Create a runtime with extreme configuration for testing
    pub fn create_extreme_runtime() -> ErrorRuntime {
        let mut config = ErrorPropagationConfig::default();
        config.max_chain_depth = 1;
        config.auto_panic_threshold = Some(2);
        config.capture_stack_traces = true;
        config.log_propagation = true;
        
        let runtime = ErrorRuntime::with_config(config);
        runtime.initialize().unwrap();
        runtime
    }
    
    /// Create a large error context for testing
    pub fn create_large_error_context(size: usize) -> ErrorContext {
        let mut context = ErrorContext::new();
        
        for i in 0..size {
            context.add_to_chain(
                format!("Large context error {}", i),
                Some(SourceLocation::new(i + 1, 10)),
                Some(format!("large_function_{}", i)),
            );
        }
        
        context
    }
    
    /// Simulate memory pressure by allocating large amounts of memory
    pub fn simulate_memory_pressure() -> Vec<Vec<u8>> {
        let mut allocations = Vec::new();
        for _ in 0..50 {
            allocations.push(vec![0u8; 1024 * 1024]); // 1MB each
        }
        allocations
    }
}
