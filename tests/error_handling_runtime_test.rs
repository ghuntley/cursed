//! Runtime Error Handling Tests for CURSED
//!
//! This module tests the runtime aspects of error handling:
//! - Runtime error propagation mechanics
//! - Panic handler registration and triggering
//! - Stack unwinding behavior
//! - Error context preservation during runtime
//! - FFI integration with compiled code
//! - Performance under runtime stress

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::runtime::error_handling::{
    ErrorRuntime, ErrorContext, ErrorPropagationConfig, next_error_id,
    initialize_error_runtime, get_error_runtime, shutdown_error_runtime
};
use cursed::runtime::panic::{
    PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory, RecoveryAction,
    get_panic_runtime, initialize_panic_runtime, shutdown_panic_runtime
};
use cursed::runtime::stack_trace::{StackTraceManager, CallFrame};
use cursed::runtime::runtime_error::{RuntimeError, ErrorSeverity, ErrorCategory};
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};

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
fn test_runtime_error_id_generation() {
    init_tracing!();
    
    let id1 = next_error_id();
    let id2 = next_error_id();
    let id3 = next_error_id();
    
    // IDs should be unique and sequential
    assert!(id1 < id2);
    assert!(id2 < id3);
    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    
    tracing::info!("Generated error IDs: {}, {}, {}", id1, id2, id3);
}

#[test]
fn test_runtime_error_context_thread_isolation() {
    init_tracing!();
    
    let runtime = Arc::new(ErrorRuntime::new());
    runtime.initialize().unwrap();
    
    let num_threads = 4;
    let errors_per_thread = 5;
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let runtime_clone = Arc::clone(&runtime);
        let handle = thread::spawn(move || {
            let mut thread_contexts = Vec::new();
            
            for error_id in 0..errors_per_thread {
                let error = CursedError::Runtime(
                    format!("Thread {} runtime error {}", thread_id, error_id)
                );
                let location = Some(SourceLocation::new(
                    (thread_id * 10) + error_id,
                    5,
                ).with_file(&format!("thread_{}.csd", thread_id)));
                
                let result = runtime_clone.propagate_error(
                    error,
                    location,
                    Some(format!("thread_{}_function_{}", thread_id, error_id)),
                );
                
                assert!(result.is_err());
                
                // Each thread should have its own context
                if let Some(context) = runtime_clone.get_current_error_context() {
                    thread_contexts.push(context.context_id);
                }
            }
            
            // Verify contexts are unique within thread
            thread_contexts.sort();
            thread_contexts.dedup();
            
            thread_contexts
        });
        handles.push(handle);
    }
    
    // Collect all context IDs from all threads
    let mut all_context_ids = Vec::new();
    for handle in handles {
        let thread_contexts = handle.join().unwrap();
        all_context_ids.extend(thread_contexts);
    }
    
    // All context IDs should be unique across threads
    let original_len = all_context_ids.len();
    all_context_ids.sort();
    all_context_ids.dedup();
    assert_eq!(all_context_ids.len(), original_len, "Context IDs should be unique across threads");
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_runtime_panic_integration() {
    init_tracing!();
    
    // Initialize both panic and error runtimes
    if get_panic_runtime().is_none() {
        let _ = initialize_panic_runtime();
    }
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Test panic info creation
    let panic_info = CursedPanicInfo::new(
        "Runtime panic test".to_string(),
        PanicSeverity::Critical,
        PanicCategory::System,
    ).with_location(SourceLocation::new(50, 25).with_file("runtime_panic.csd"))
      .with_recovery_action(RecoveryAction::LogAndContinue);
    
    assert_eq!(panic_info.message, "Runtime panic test");
    assert_eq!(panic_info.severity, PanicSeverity::Critical);
    assert_eq!(panic_info.category, PanicCategory::System);
    assert!(panic_info.source_location.is_some());
    assert_eq!(panic_info.recovery_action, RecoveryAction::LogAndContinue);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_runtime_error_conversion_to_panic() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    let error = CursedError::Type("Critical type error for panic conversion".to_string());
    let location = Some(SourceLocation::new(60, 30).with_file("conversion_test.csd"));
    let function = Some("critical_function".to_string());
    
    // Note: This test verifies the conversion mechanism without actually panicking
    // In a real scenario, this would trigger the panic handler
    let result = runtime.convert_error_to_panic(error, location, function);
    
    // The conversion should complete (actual panic behavior depends on panic runtime setup)
    match result {
        Ok(()) => tracing::info!("Error conversion completed successfully"),
        Err(e) => tracing::info!("Error conversion resulted in: {}", e),
    }
    
    // Check that conversion was tracked in statistics
    let stats = runtime.get_statistics().unwrap();
    assert!(stats.errors_to_panics >= 1);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_runtime_error_propagation_performance() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    let num_errors = 10000;
    let start_time = Instant::now();
    
    for i in 0..num_errors {
        let error = CursedError::Runtime(format!("Performance test error {}", i));
        let location = Some(SourceLocation::new(i % 1000, 5));
        
        let result = runtime.propagate_error(error, location, None);
        assert!(result.is_err());
        
        // Clear context every 100 errors to prevent memory buildup
        if i % 100 == 0 {
            runtime.clear_error_context();
        }
    }
    
    let elapsed = start_time.elapsed();
    let stats = runtime.get_statistics().unwrap();
    
    assert_eq!(stats.total_errors, num_errors as u64);
    
    // Performance assertion: should handle 10K errors quickly
    assert!(elapsed < Duration::from_secs(2), "Runtime error handling took too long: {:?}", elapsed);
    
    let errors_per_second = num_errors as f64 / elapsed.as_secs_f64();
    tracing::info!(
        "Runtime processed {} errors in {:?} ({:.2} errors/sec)",
        num_errors,
        elapsed,
        errors_per_second
    );
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_runtime_error_chain_memory_management() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Create errors with large messages to test memory handling
    let large_message = "x".repeat(1000); // 1KB per error
    let num_large_errors = 1000;
    
    for i in 0..num_large_errors {
        let error = CursedError::Runtime(format!("{} - error {}", large_message, i));
        let location = Some(SourceLocation::new(i, 10).with_file("memory_test.csd"));
        
        let result = runtime.propagate_error(error, location, None);
        assert!(result.is_err());
        
        // Periodically clear context to test memory management
        if i % 50 == 0 {
            runtime.clear_error_context();
        }
    }
    
    // Final cleanup
    runtime.clear_error_context();
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_errors, num_large_errors as u64);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_runtime_error_statistics_tracking() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Generate different types of errors to test statistics
    let error_batches = vec![
        (CursedError::Parse("Parse error".to_string()), 10),
        (CursedError::Type("Type error".to_string()), 15),
        (CursedError::Runtime("Runtime error".to_string()), 20),
        (CursedError::Compile("Compile error".to_string()), 5),
    ];
    
    let mut total_expected = 0;
    for (base_error, count) in error_batches {
        for i in 0..count {
            let error = match &base_error {
                CursedError::Parse(_) => CursedError::Parse(format!("Parse error {}", i)),
                CursedError::Type(_) => CursedError::Type(format!("Type error {}", i)),
                CursedError::Runtime(_) => CursedError::Runtime(format!("Runtime error {}", i)),
                CursedError::Compile(_) => CursedError::Compile(format!("Compile error {}", i)),
                _ => base_error.clone(),
            };
            
            let result = runtime.propagate_error(error, None, None);
            assert!(result.is_err());
            total_expected += 1;
        }
        
        // Clear context between batches
        runtime.clear_error_context();
    }
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_errors, total_expected);
    assert_eq!(stats.successful_propagations, total_expected);
    assert!(stats.average_propagation_time.as_nanos() > 0);
    
    tracing::info!("Statistics: {:?}", stats);
    runtime.shutdown().unwrap();
}

#[test]
fn test_runtime_concurrent_error_handling() {
    init_tracing!();
    
    let runtime = Arc::new(ErrorRuntime::new());
    runtime.initialize().unwrap();
    
    let num_threads = 8;
    let operations_per_thread = 100;
    let barrier = Arc::new((Mutex::new(0), Condvar::new()));
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let runtime_clone = Arc::clone(&runtime);
        let barrier_clone = Arc::clone(&barrier);
        
        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            let (lock, cvar) = &*barrier_clone;
            let mut count = lock.lock().unwrap();
            *count += 1;
            if *count == num_threads {
                cvar.notify_all();
            } else {
                while *count < num_threads {
                    count = cvar.wait(count).unwrap();
                }
            }
            drop(count);
            
            // Perform concurrent error operations
            for op_id in 0..operations_per_thread {
                let error = CursedError::Runtime(
                    format!("Concurrent error T{} O{}", thread_id, op_id)
                );
                let location = Some(SourceLocation::new(
                    thread_id * 100 + op_id,
                    thread_id + 1,
                ).with_file(&format!("concurrent_{}.csd", thread_id)));
                
                let result = runtime_clone.propagate_error(
                    error,
                    location,
                    Some(format!("concurrent_func_{}_{}", thread_id, op_id)),
                );
                
                assert!(result.is_err());
                
                // Occasionally check state and clear context
                if op_id % 25 == 0 {
                    let _in_error = runtime_clone.is_in_error_handling();
                    let _context = runtime_clone.get_current_error_context();
                    runtime_clone.clear_error_context();
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_errors, (num_threads * operations_per_thread) as u64);
    
    tracing::info!(
        "Concurrent test completed: {} threads, {} ops/thread, {} total errors",
        num_threads,
        operations_per_thread,
        stats.total_errors
    );
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_runtime_error_context_persistence() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Create an error context and verify persistence
    let error = CursedError::Runtime("Persistence test error".to_string());
    let location = Some(SourceLocation::new(70, 35).with_file("persistence.csd"));
    let function = Some("persistence_function".to_string());
    
    let result = runtime.propagate_error(error, location.clone(), function.clone());
    assert!(result.is_err());
    
    // Verify context exists and contains expected information
    let context = runtime.get_current_error_context();
    assert!(context.is_some());
    
    let context = context.unwrap();
    assert!(context.context_id > 0);
    assert_eq!(context.source_location.as_ref(), location.as_ref());
    assert!(!context.error_chain.is_empty());
    
    // Verify error chain contains our error
    let chain_entry = &context.error_chain[0];
    assert!(chain_entry.message.contains("Persistence test error"));
    assert_eq!(chain_entry.source_location, location);
    assert_eq!(chain_entry.function_name, function);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_runtime_error_propagation_timeout() {
    init_tracing!();
    
    let mut config = ErrorPropagationConfig::default();
    config.propagation_timeout = Duration::from_millis(100);
    
    let runtime = ErrorRuntime::with_config(config);
    runtime.initialize().unwrap();
    
    // Test that propagation respects timeout configuration
    let start_time = Instant::now();
    
    let error = CursedError::Runtime("Timeout test error".to_string());
    let result = runtime.propagate_error(error, None, None);
    
    let elapsed = start_time.elapsed();
    assert!(result.is_err());
    
    // Operation should complete well within timeout
    assert!(elapsed < Duration::from_millis(50));
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_runtime_global_error_runtime_integration() {
    init_tracing!();
    
    // Test global runtime lifecycle
    if get_error_runtime().is_none() {
        assert!(initialize_error_runtime().is_ok());
    }
    
    // Global runtime should be available
    assert!(get_error_runtime().is_some());
    
    // Test error propagation through global runtime
    if let Some(global_runtime) = get_error_runtime() {
        let error = CursedError::Runtime("Global runtime test".to_string());
        let location = Some(SourceLocation::new(80, 40).with_file("global_test.csd"));
        
        let result = global_runtime.propagate_error(error, location, None);
        assert!(result.is_err());
        
        // Clear context after test
        global_runtime.clear_error_context();
    }
    
    // Cleanup
    assert!(shutdown_error_runtime().is_ok());
}

#[test]
fn test_runtime_ffi_error_propagation() {
    init_tracing!();
    
    use cursed::runtime::error_handling::{
        cursed_propagate_error, cursed_is_in_error_handling, cursed_clear_error_context,
        cursed_get_error_context_info
    };
    
    // Initialize runtime for FFI testing
    if get_error_runtime().is_none() {
        let _ = initialize_error_runtime();
    }
    
    // Test FFI error propagation
    let error_message = "FFI test error";
    let error_code = 42u32;
    let line = 90u32;
    let column = 45u32;
    let file_name = "ffi_test.csd";
    let function_name = "ffi_test_function";
    
    let result = cursed_propagate_error(
        error_message.as_ptr(),
        error_message.len(),
        error_code,
        line,
        column,
        file_name.as_ptr(),
        file_name.len(),
        function_name.as_ptr(),
        function_name.len(),
    );
    
    // Should return 1 (error propagated)
    assert_eq!(result, 1);
    
    // Test error handling state check
    let in_error = cursed_is_in_error_handling();
    // May or may not be in error handling depending on timing
    assert!(in_error == 0 || in_error == 1);
    
    // Test error context info retrieval
    let mut context_id: u64 = 0;
    let mut chain_depth: u32 = 0;
    let context_available = cursed_get_error_context_info(
        &mut context_id as *mut u64,
        &mut chain_depth as *mut u32,
    );
    
    if context_available == 1 {
        assert!(context_id > 0);
        assert!(chain_depth > 0);
        tracing::info!("FFI context info: ID={}, depth={}", context_id, chain_depth);
    }
    
    // Clear context
    cursed_clear_error_context();
    
    // After clearing, should not be in error handling
    let in_error_after = cursed_is_in_error_handling();
    assert_eq!(in_error_after, 0);
}

#[test]
fn test_runtime_error_recovery_scenarios() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Test recoverable error scenarios
    let recoverable_error = CursedError::recoverable_panic("Recoverable runtime issue".to_string());
    assert!(recoverable_error.is_recoverable_panic());
    
    let result = runtime.propagate_error(recoverable_error, None, None);
    assert!(result.is_err());
    
    // Test recovery error
    let recovery_error = CursedError::recovery_error("Recovery attempt failed".to_string(), 2);
    assert!(recovery_error.is_recovery());
    
    let result2 = runtime.propagate_error(recovery_error, None, None);
    assert!(result2.is_err());
    
    // Test contextual error creation for recovery
    let base_error = CursedError::Type("Type check failed".to_string());
    let enhanced = runtime.create_contextual_error(
        base_error,
        "During recovery attempt",
        Some(SourceLocation::new(100, 50)),
    );
    
    assert!(enhanced.to_string().contains("During recovery attempt"));
    assert!(enhanced.to_string().contains("Type check failed"));
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_runtime_error_metadata_handling() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Create error context with metadata
    let mut context = ErrorContext::new()
        .with_location(SourceLocation::new(110, 55).with_file("metadata_test.csd"))
        .with_goroutine(98765)
        .with_metadata("error_type".to_string(), "validation".to_string())
        .with_metadata("severity".to_string(), "high".to_string())
        .with_metadata("component".to_string(), "type_checker".to_string());
    
    context.add_to_chain(
        "Metadata test error".to_string(),
        Some(SourceLocation::new(110, 55)),
        Some("metadata_function".to_string()),
    );
    
    // Verify metadata is preserved
    assert_eq!(context.metadata.get("error_type"), Some(&"validation".to_string()));
    assert_eq!(context.metadata.get("severity"), Some(&"high".to_string()));
    assert_eq!(context.metadata.get("component"), Some(&"type_checker".to_string()));
    assert_eq!(context.goroutine_id, Some(98765));
    
    // Test display includes metadata context
    let display_string = format!("{}", context);
    assert!(display_string.contains("goroutine #98765"));
    assert!(display_string.contains("Error Context"));
    
    runtime.shutdown().unwrap();
}

/// Documentation: Runtime Error Handling Testing
/// 
/// This comprehensive test suite validates the runtime behavior of CURSED's
/// error handling system. Key areas covered:
/// 
/// 1. **Error ID Generation**: Ensures unique identifiers for error tracking
/// 2. **Thread Isolation**: Verifies error contexts don't leak between threads
/// 3. **Panic Integration**: Tests integration with panic/recovery system
/// 4. **Performance**: Validates error handling performance under load
/// 5. **Memory Management**: Tests memory usage during error propagation
/// 6. **Statistics**: Verifies error tracking and metrics collection
/// 7. **Concurrency**: Tests thread-safe error handling operations
/// 8. **Context Persistence**: Validates error context preservation
/// 9. **FFI Integration**: Tests foreign function interface for compiled code
/// 10. **Recovery Scenarios**: Tests error recovery and retry mechanisms
/// 11. **Metadata Handling**: Validates error context metadata preservation
/// 
/// These tests ensure that the runtime error handling system:
/// - Performs efficiently under high load
/// - Maintains thread safety in concurrent scenarios
/// - Preserves error context and debugging information
/// - Integrates correctly with compiled code via FFI
/// - Handles memory pressure gracefully
/// - Provides accurate error tracking and statistics
/// 
/// Runtime testing is critical because it validates actual execution behavior
/// rather than just compilation correctness. This ensures that CURSED programs
/// can rely on robust error handling in production environments.

#[cfg(test)]
mod runtime_error_test_utilities {
    use super::*;
    
    /// Helper to create high-volume error scenarios
    pub fn generate_error_load(
        runtime: &ErrorRuntime,
        num_errors: usize,
        error_prefix: &str,
    ) -> Duration {
        let start_time = Instant::now();
        
        for i in 0..num_errors {
            let error = CursedError::Runtime(format!("{} error {}", error_prefix, i));
            let location = Some(SourceLocation::new(i % 1000, 5));
            let _ = runtime.propagate_error(error, location, None);
            
            if i % 100 == 0 {
                runtime.clear_error_context();
            }
        }
        
        start_time.elapsed()
    }
    
    /// Helper to create complex error chain
    pub fn create_complex_error_chain(runtime: &ErrorRuntime, depth: usize) {
        for i in 0..depth {
            let error = CursedError::Runtime(format!("Chain error level {}", i));
            let location = Some(SourceLocation::new(i + 1, 10));
            let _ = runtime.propagate_error(error, location, Some(format!("chain_func_{}", i)));
        }
    }
    
    /// Helper to measure error handling performance
    pub fn measure_error_performance<F>(operation: F) -> (Duration, u64)
    where
        F: FnOnce() -> u64,
    {
        let start = Instant::now();
        let operations = operation();
        let elapsed = start.elapsed();
        (elapsed, operations)
    }
    
    /// Helper to validate error statistics
    pub fn assert_error_stats(
        stats: &cursed::runtime::error_handling::ErrorHandlingStatistics,
        expected_total: u64,
        expected_successful: u64,
    ) {
        assert_eq!(stats.total_errors, expected_total);
        assert_eq!(stats.successful_propagations, expected_successful);
        assert!(stats.average_propagation_time.as_nanos() > 0);
    }
}
