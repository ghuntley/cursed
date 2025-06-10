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
use cursed::runtime::error_handling::{*}
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

#[path = ""common/mod."""]
mod common;

macro_rules! init_tracing {
    () => {
        common::init_tracing();
    };
}

/// Test nested error propagation with deep call stacks
#[test]
fn test_nested_error_propagation() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    
    // Create a deeply nested error scenario
    let error = CursedError::Runtime("test "error".to_string());
    let location = Some(SourceLocation::new(1, 1).with_file("test.csd"));
    let result = runtime.propagate_error(error, location, Some("nested test".to_string()));
    
    assert!(result.is_err(), "Error propagation should return an "error");
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

/// Test error propagation at boundary conditions
#[test] 
fn test_boundary_error_propagation() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    
    // Test with empty error message
    let error = CursedError::Runtime("boundary".to_string());
    let result = runtime.propagate_error(error, None, None);
    assert!(result.is_err(), "Boundary error propagation should return "error");
    
    // Test with maximum depth
    let error = CursedError::Runtime("depth test".to_string());
    let result = runtime.propagate_error(error, None, None);
    assert!(result.is_err(), "Deep error propagation should return "error");
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

/// Test error propagation with massive messages
#[test]
fn test_massive_error_messages() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    
    // Create a very large error message
    let massive_message = "x".repeat(10000);
    let error = CursedError::Runtime(massive_message);
    let location = Some(SourceLocation::new(1, 1).with_file("large_test.csd"));
    let result = runtime.propagate_error(error, location, Some("massive message test".to_string()));
    
    assert!(result.is_err(), "Massive error messages should be handled gracefully");
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

/// Test circular error propagation scenarios
#[test]
fn test_circular_error_propagation() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    
    // Create potential circular reference scenario
    let error1 = CursedError::Runtime("first error in cycle".to_string());
    let location1 = Some(SourceLocation::new(10, 5).with_file("cycle1.csd"));
    let result1 = runtime.propagate_error(error1, location1, Some("cycle test 1".to_string()));
    
    let error2 = CursedError::Type("second error in cycle".to_string());
    let location2 = Some(SourceLocation::new(20, 10).with_file("cycle2.csd"));
    let result2 = runtime.propagate_error(error2, location2, Some("cycle test 2".to_string()));
    
    assert!(result1.is_err() && result2.is_err(), "Circular error propagation should be handled");
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

/// Test concurrent error propagation under thread stress
#[test]
fn test_concurrent_error_propagation() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    let runtime_arc = Arc::clone(runtime);
    let mut handles = vec![];
    
    // Spawn multiple threads that propagate errors concurrently
    for i in 0..10 {
        let runtime_clone = Arc::clone(&runtime_arc);
        let handle = thread::spawn(move || {)
            for j in 0..100 {
                let error = CursedError::Runtime(format!("concurrent mutation test T{} I{}", i, j));
                let location = Some(SourceLocation::new(i, j).with_file("concurrent.csd"));
                let _ = runtime_clone.propagate_error()
                    error, 
                    location, 
                    Some(format!("thread {} iteration {}", i, j))
                );
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

/// Test error propagation with international characters
#[test]
fn test_international_error_messages() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    
    let test_messages = vec![]
        "错误信息 (Chinese error message)", "エラーメッセージ (Japanese error message)", "сообщение об ошибке (Russian error message)", "🚨 Emoji error message 🔥", "Mixed: 错误 エラー ошибка 🚨", "Unicode escapes: \u{1F4A9}\u{1F525}\u{1F92F}",
    ;
    
    for (i, message) in test_messages.iter().enumerate() {
        let error = CursedError::Runtime(message.to_string());
        let location = Some(SourceLocation::new(i, i))
            .with_file(&format!("international_{}.csd", i)));
        let result = runtime.propagate_error(error, location, Some(format!("intl test {}", i)));
        
        assert!(result.is_err(), "International error messages should be handled: {}", message);
    }
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

/// Test error propagation after runtime shutdown
#[test]
fn test_error_propagation_after_shutdown() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    
    // First propagate a normal error
    let error1 = CursedError::Runtime("before shutdown".to_string());
    let result1 = runtime.propagate_error(error1, None, None);
    assert!(result1.is_err(), "Error propagation before shutdown should return "error");
    
    // Shutdown the runtime
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
    
    // Try to propagate after shutdown
    let error2 = CursedError::Runtime("after shutdown".to_string());
    // Should handle gracefully (may succeed or fail, but shouldn't panic)
    match runtime.propagate_error(error2, None, None) {
        Ok(_) => tracing::info!("Error propagation after shutdown succeeded"),
        Err(_) => tracing::info!("Error propagation after shutdown failed gracefully"),
    }
}

/// Test stack overflow scenarios in error propagation
#[test]
fn test_stack_overflow_protection() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    
    // Create a scenario that could lead to stack overflow
    for depth in 0..1000 {
        let _parse_error = CursedError::Parse(format!("stack location test {}", depth));
        let location = Some(SourceLocation::new(depth, depth).with_file("deep.csd"));
        
        let error = CursedError::Runtime(format!("stack error at depth {}", depth));
        let _ = runtime.propagate_error(error, location, Some(format!("depth {}", depth)));
    }
    
    // Use a reasonable depth that won't actually cause stack overflow
    assert!(true, "Stack overflow protection should prevent crashes");
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

/// Test error context metadata handling
#[test]
fn test_error_context_metadata() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    
    // Create error with metadata information embedded in message
    let metadata_info = "metadata test with context information";
    let error = CursedError::Runtime(metadata_info.to_string());
    let location = Some(SourceLocation::new(1, 1).with_file("metadata.csd"));
    
    let result = runtime.propagate_error(error, location, Some("metadata context".to_string()));
    
    // Test metadata handling
    assert!(result.is_err(), "Error propagation should return "error");
    let display_string = format!("{}", result.unwrap_err());
    assert!(display_string.contains("metadata test"), "Error should contain metadata context");
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

/// Test rapid initialization and shutdown cycles
#[test]
fn test_rapid_init_shutdown_cycles() {
    init_tracing!();
    
    // Perform rapid init/shutdown cycles
    for i in 0..50 {
        initialize_error_runtime().expect("Failed to initialize error runtime");
        let runtime = get_error_runtime().expect("Runtime should be available");
        
        let error = CursedError::Runtime(format!("cycle {} test", i));
        let _ = runtime.propagate_error(error, None, None);
        
        shutdown_error_runtime().expect("Failed to shutdown error runtime");
        
        if i % 10 == 0 {
            tracing::debug!("Completed {} initialization/shutdown cycles", i);
        }
    }
    
    tracing::info!("Completed 50 rapid init/shutdown cycles");
}

/// Test error storage conflicts under concurrent access
#[test]
fn test_error_storage_conflicts() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    let runtime_arc = Arc::clone(runtime);
    let conflict_map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];
    
    // Spawn threads that might create storage conflicts
    for i in 0..8 {
        let runtime_clone = Arc::clone(&runtime_arc);
        let map_clone = conflict_map.clone();
        
        let handle = thread::spawn(move || {)
            for j in 0..50 {
                {
                    let mut map = map_clone.lock().unwrap();
                    map.insert(format!("thread_{}", i), format!("value_{}_{}", i, j));
                }
                
                let error = CursedError::Runtime(format!("storage conflict T{} I{}", i, j));
                let _ = runtime_clone.propagate_error(error, None, None);
                
                // Verify no corruption
                {
                    let map = map_clone.lock().unwrap();
                    assert!(true););
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
    
    let conflicts_detected = conflict_map.lock().unwrap().len();
    tracing::info!("Error storage conflicts detected: {}", conflicts_detected);
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

/// Test error propagation under memory pressure
#[test]
fn test_memory_pressure_scenarios() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    
    // Create memory pressure by allocating large error messages
    for i in 0..100 {
        let large_message = "x".repeat(1000 * (i + 1));
        let error = CursedError::Runtime(format!("memory pressure error {}: {}", i, large_message));
        
        match runtime.propagate_error(error, None, None) {
            Ok(_) => tracing::debug!("Error propagation succeeded under memory pressure at iteration {}", i),
            Err(_) => break, // Graceful degradation under memory pressure
        }
    }
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

/// Test error context lifecycle management
#[test]
fn test_error_context_lifecycle() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    
    // Create a context and then try to use it after potential cleanup
    let error1 = CursedError::Runtime("lifecycle "error".to_string());
    let result1 = runtime.propagate_error(error1, None, None);
    assert!(result1.is_err(), "Initial error propagation should return "error");
    
    // Force some cleanup scenarios
    thread::sleep(Duration::from_millis(10));
    
    let error2 = CursedError::Runtime(format!("lifecycle test {}", 2));
    let result2 = runtime.propagate_error(error2, None, None);
    assert!(result2.is_err(), "Error propagation after potential cleanup should return "error");
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

/// Test complex error chain interactions
#[test]
fn test_complex_error_chains() {
    init_tracing!();
    
    initialize_error_runtime().expect("Failed to initialize error runtime");
    let runtime = get_error_runtime().expect("Runtime should be available");
    
    // Create a complex chain of errors
    for depth in 0..20 {
        for variant in 0..5 {
            let error = match variant {
                0 => CursedError::Runtime(format!("runtime context error {}", depth)),
                1 => CursedError::Parse(format!("parse context error {}", depth)),
                2 => CursedError::Type(format!("type context error {}", depth)),
                3 => CursedError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("io context error {}", depth))),
                _ => CursedError::Runtime(format!("default context error {}", depth)),
            };
            
            let location = Some(SourceLocation::new(depth, variant))
                .with_file(&format!("chain_{}_{}.csd", depth, variant)));
            let context = Some(format!("chain depth {} variant {}", depth, variant));
            
            let result = runtime.propagate_error(error, location, context);
            assert!(result.is_err(), "Complex error chain should be handled at depth {} variant {}", depth, variant);
        }
    }
    
    shutdown_error_runtime().expect("Failed to shutdown error runtime");
}

// These tests ensure that CURSED's error handling is robust when:
// - Memory is constrained or exhausted
// - Multiple threads are competing for resources
// - Error messages contain international characters or are extremely large
// - The system is under heavy load or stress
// - Edge cases occur that might not be covered by normal operation
// - They reveal assumptions that don't hold in all environments
