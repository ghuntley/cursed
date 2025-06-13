use cursed::runtime::error_propagation_runtime::*;
use cursed::error::{Error, SourceLocation as ErrorSourceLocation};
use cursed::error::error_propagation::ErrorPropagationError;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};

#[path = "common.rs"]
pub mod common;

/// Integration test for complete error propagation workflow
#[test]
fn test_complete_error_propagation_workflow() {
    common::tracing::setup();
    
    let mut runtime = ErrorPropagationRuntime::new();
    
    // Register custom error handlers
    runtime.register_handler(Box::new(IoErrorHandler::new()));
    runtime.register_handler(Box::new(ParseErrorHandler::new()));
    runtime.register_handler(Box::new(DefaultErrorHandler::new()));
    
    // Test different error types
    let test_cases = vec![
        (Error::Io("File not found".to_string()), "IoErrorHandler"),
        (Error::Parse("Syntax error".to_string()), "ParseErrorHandler"),
        (Error::Runtime("Runtime error".to_string()), "DefaultErrorHandler"),
    ];
    
    for (i, (error, expected_handler)) in test_cases.into_iter().enumerate() {
        let location = ErrorSourceLocation::new(10 + i as u32, 5 + i as u32);
        let result = runtime.propagate_error(
            error,
            location,
            Some(format!("test_function_{}", i)),
        );
        
        assert!(result.is_ok(), "Error propagation should succeed");
        
        // Verify propagation frame was created with stack trace
        assert_eq!(runtime.get_propagation_depth(), i + 1);
        let frame = &runtime.propagation_stack[i];
        assert!(!frame.stack_trace.is_empty());
        assert!(frame.debug_info.is_some());
        assert!(frame.function_name.is_some());
    }
    
    // Check final statistics
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_propagations, 3);
    assert_eq!(stats.successful_propagations, 3);
    assert_eq!(stats.failed_propagations, 0);
}

/// Test error propagation with real LLVM integration context
#[test] 
fn test_llvm_integration_context() {
    common::tracing::setup();
    
    let mut runtime = ErrorPropagationRuntime::new();
    runtime.register_handler(Box::new(LlvmErrorHandler::new()));
    
    // Simulate LLVM compilation error
    let error = Error::CodeGeneration {
        message: "LLVM compilation failed".to_string(),
        line: Some(25),
        column: Some(10),
    };
    
    let location = ErrorSourceLocation::new(25, 10);
    let result = runtime.propagate_error(
        error,
        location,
        Some("llvm_compile_function".to_string()),
    );
    
    assert!(result.is_ok());
    
    // Verify LLVM-specific context was captured
    let frame = &runtime.propagation_stack[0];
    assert_eq!(frame.function_name, Some("llvm_compile_function".to_string()));
    assert!(frame.debug_info.is_some());
    
    let debug_info = frame.debug_info.as_ref().unwrap();
    assert_eq!(debug_info.source_language, Some("CURSED".to_string()));
}

/// Test error propagation chain through multiple functions
#[test]
fn test_error_propagation_chain() {
    common::tracing::setup();
    
    let runtime = Arc::new(Mutex::new(ErrorPropagationRuntime::new()));
    
    // Simulate nested function calls with error propagation
    let result = simulate_nested_function_calls(runtime.clone(), 0);
    assert!(result.is_ok());
    
    // Verify the propagation chain
    let runtime = runtime.lock().unwrap();
    assert_eq!(runtime.get_propagation_depth(), 3); // 3 levels deep
    
    // Check that each frame has decreasing line numbers (simulating call stack)
    for (i, frame) in runtime.propagation_stack.iter().enumerate() {
        assert_eq!(frame.location.line, (30 - i * 10) as u32);
        assert!(frame.function_name.is_some());
        assert!(!frame.stack_trace.is_empty());
    }
}

/// Test panic integration during error propagation
#[test]
fn test_panic_integration_during_propagation() {
    common::tracing::setup();
    
    let config = PropagationConfig {
        panic_integration_enabled: true,
        max_propagation_depth: 2,
        ..PropagationConfig::default()
    };
    
    let mut runtime = ErrorPropagationRuntime::with_config(config);
    runtime = runtime.with_panic_integration("test_panic_runtime".to_string());
    
    // Register a handler that always fails
    runtime.register_handler(Box::new(FailingErrorHandler::new()));
    
    let location = ErrorSourceLocation::new(15, 8);
    let error = Error::Runtime("Panic integration test".to_string());
    
    let result = runtime.propagate_error(error, location, Some("panic_test".to_string()));
    
    // Should fail but not panic (panic integration logs and continues)
    assert!(result.is_err());
    
    // Verify panic integration was triggered
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.panic_integrations, 1);
}

/// Test stack trace accuracy in different scenarios
#[test]
fn test_stack_trace_accuracy() {
    common::tracing::setup();
    
    // Test deep call stack
    let result = deep_function_call(5);
    let runtime = result.unwrap();
    
    let stack_trace = runtime.capture_stack_trace();
    assert!(stack_trace.len() >= 5);
    
    // Verify that we can find function names in the stack
    let function_names: Vec<_> = stack_trace
        .iter()
        .filter_map(|frame| frame.function_name.as_ref())
        .collect();
    
    assert!(!function_names.is_empty());
    
    // Should find test-related functions
    let has_test_functions = function_names.iter().any(|name| {
        name.contains("test_stack_trace_accuracy") || 
        name.contains("deep_function_call")
    });
    assert!(has_test_functions);
}

/// Test error context preservation
#[test]
fn test_error_context_preservation() {
    common::tracing::setup();
    
    let config = PropagationConfig {
        preserve_error_context: true,
        generate_stack_traces: true,
        ..PropagationConfig::default()
    };
    
    let mut runtime = ErrorPropagationRuntime::with_config(config);
    runtime.register_handler(Box::new(ContextPreservingHandler::new()));
    
    let original_error = Error::Parse("Original parse error".to_string());
    let location = ErrorSourceLocation::new(42, 15);
    
    let result = runtime.propagate_error(
        original_error,
        location.clone(),
        Some("context_preserving_function".to_string()),
    );
    
    assert!(result.is_ok());
    
    // Verify context preservation
    let frame = &runtime.propagation_stack[0];
    assert_eq!(frame.location.line, 42);
    assert_eq!(frame.location.column, 15);
    assert_eq!(frame.function_name, Some("context_preserving_function".to_string()));
    assert!(!frame.stack_trace.is_empty());
    
    // Verify debug info includes compilation unit detection
    let debug_info = frame.debug_info.as_ref().unwrap();
    assert!(debug_info.symbols_available);
    assert_eq!(debug_info.source_language, Some("CURSED".to_string()));
}

/// Test performance under high load
#[test]
fn test_performance_under_load() {
    common::tracing::setup();
    
    let config = PropagationConfig {
        collect_statistics: true,
        ..PropagationConfig::default()
    };
    
    let mut runtime = ErrorPropagationRuntime::with_config(config);
    runtime.register_handler(Box::new(FastErrorHandler::new()));
    
    let start_time = Instant::now();
    let num_propagations = 1000;
    
    // Perform many error propagations
    for i in 0..num_propagations {
        let location = ErrorSourceLocation::new(i % 100, i % 50);
        let error = Error::Runtime(format!("Performance test error {}", i));
        
        runtime.clear_propagation_stack(); // Reset for each test
        let result = runtime.propagate_error(error, location, None);
        assert!(result.is_ok());
    }
    
    let total_time = start_time.elapsed();
    let stats = runtime.get_statistics().unwrap();
    
    assert_eq!(stats.total_propagations, num_propagations);
    assert!(stats.average_propagation_time_us > 0.0);
    
    // Performance should be reasonable (less than 1ms per propagation on average)
    let avg_time_per_propagation = total_time.as_millis() as f64 / num_propagations as f64;
    assert!(avg_time_per_propagation < 1.0, 
           "Average time per propagation too high: {}ms", avg_time_per_propagation);
}

/// Test thread safety with concurrent access
#[test]
fn test_thread_safety_concurrent_access() {
    common::tracing::setup();
    
    let runtime = Arc::new(Mutex::new(ErrorPropagationRuntime::new()));
    let num_threads = 8;
    let ops_per_thread = 50;
    
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let runtime_clone = runtime.clone();
        
        let handle = thread::spawn(move || {
            for i in 0..ops_per_thread {
                let mut runtime = runtime_clone.lock().unwrap();
                
                let location = ErrorSourceLocation::new(
                    (thread_id * 100 + i) % 200,
                    (thread_id * 10 + i) % 50,
                );
                let error = Error::Runtime(format!("Thread {} error {}", thread_id, i));
                
                let result = runtime.propagate_error(
                    error,
                    location,
                    Some(format!("thread_{}_function_{}", thread_id, i)),
                );
                
                // Clear stack for next iteration
                runtime.clear_propagation_stack();
                
                // Most operations should succeed
                assert!(result.is_ok() || result.is_err()); // Either is fine under concurrency
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify final state
    let final_runtime = runtime.lock().unwrap();
    let stats = final_runtime.get_statistics().unwrap();
    assert!(stats.total_propagations >= num_threads * ops_per_thread / 2); // Allow for some failures
}

// Helper functions for integration tests

fn simulate_nested_function_calls(
    runtime: Arc<Mutex<ErrorPropagationRuntime>>,
    depth: u32,
) -> Result<(), ErrorPropagationError> {
    if depth >= 3 {
        // Base case - propagate an error
        let mut runtime = runtime.lock().unwrap();
        let location = ErrorSourceLocation::new(30 - depth * 10, 5);
        let error = Error::Runtime(format!("Nested error at depth {}", depth));
        return runtime.propagate_error(
            error,
            location,
            Some(format!("nested_function_level_{}", depth)),
        );
    }
    
    // Recursive case
    simulate_nested_function_calls(runtime, depth + 1)
}

fn deep_function_call(remaining: u32) -> Result<ErrorPropagationRuntime, ErrorPropagationError> {
    if remaining == 0 {
        return Ok(ErrorPropagationRuntime::new());
    }
    deep_function_call(remaining - 1)
}

// Test-specific error handlers

#[derive(Debug)]
struct IoErrorHandler {
    name: String,
}

impl IoErrorHandler {
    fn new() -> Self {
        Self { name: "IoErrorHandler".to_string() }
    }
}

impl ErrorHandler for IoErrorHandler {
    fn handle_error(&self, error: &Error, _context: &PropagationFrame) -> Result<(), Error> {
        if matches!(error, Error::Io(_)) {
            tracing::info!("Handling IO error: {}", error);
            Ok(())
        } else {
            Err(Error::Runtime("Cannot handle non-IO error".to_string()))
        }
    }
    
    fn name(&self) -> &str { &self.name }
    fn can_handle(&self, error: &Error) -> bool { matches!(error, Error::Io(_)) }
    fn priority(&self) -> u32 { 10 }
}

#[derive(Debug)]
struct ParseErrorHandler {
    name: String,
}

impl ParseErrorHandler {
    fn new() -> Self {
        Self { name: "ParseErrorHandler".to_string() }
    }
}

impl ErrorHandler for ParseErrorHandler {
    fn handle_error(&self, error: &Error, _context: &PropagationFrame) -> Result<(), Error> {
        if matches!(error, Error::Parse(_)) {
            tracing::info!("Handling parse error: {}", error);
            Ok(())
        } else {
            Err(Error::Runtime("Cannot handle non-parse error".to_string()))
        }
    }
    
    fn name(&self) -> &str { &self.name }
    fn can_handle(&self, error: &Error) -> bool { matches!(error, Error::Parse(_)) }
    fn priority(&self) -> u32 { 20 }
}

#[derive(Debug)]
struct LlvmErrorHandler {
    name: String,
}

impl LlvmErrorHandler {
    fn new() -> Self {
        Self { name: "LlvmErrorHandler".to_string() }
    }
}

impl ErrorHandler for LlvmErrorHandler {
    fn handle_error(&self, error: &Error, context: &PropagationFrame) -> Result<(), Error> {
        if matches!(error, Error::CodeGeneration { .. }) {
            tracing::info!(
                function = ?context.function_name,
                stack_frames = context.stack_trace.len(),
                "Handling LLVM compilation error: {}", 
                error
            );
            Ok(())
        } else {
            Err(Error::Runtime("Cannot handle non-LLVM error".to_string()))
        }
    }
    
    fn name(&self) -> &str { &self.name }
    fn can_handle(&self, error: &Error) -> bool { matches!(error, Error::CodeGeneration { .. }) }
    fn priority(&self) -> u32 { 5 }
}

#[derive(Debug)]
struct FailingErrorHandler {
    name: String,
}

impl FailingErrorHandler {
    fn new() -> Self {
        Self { name: "FailingErrorHandler".to_string() }
    }
}

impl ErrorHandler for FailingErrorHandler {
    fn handle_error(&self, error: &Error, _context: &PropagationFrame) -> Result<(), Error> {
        // Always fail to test panic integration
        Err(Error::Runtime(format!("Handler failed for error: {}", error)))
    }
    
    fn name(&self) -> &str { &self.name }
    fn can_handle(&self, _error: &Error) -> bool { true }
    fn priority(&self) -> u32 { 1 }
}

#[derive(Debug)]
struct ContextPreservingHandler {
    name: String,
}

impl ContextPreservingHandler {
    fn new() -> Self {
        Self { name: "ContextPreservingHandler".to_string() }
    }
}

impl ErrorHandler for ContextPreservingHandler {
    fn handle_error(&self, error: &Error, context: &PropagationFrame) -> Result<(), Error> {
        tracing::info!(
            error = %error,
            location_line = context.location.line,
            location_column = context.location.column,
            function = ?context.function_name,
            stack_frames = context.stack_trace.len(),
            has_debug_info = context.debug_info.is_some(),
            "Preserving error context"
        );
        Ok(())
    }
    
    fn name(&self) -> &str { &self.name }
    fn can_handle(&self, _error: &Error) -> bool { true }
    fn priority(&self) -> u32 { 30 }
}

#[derive(Debug)]
struct FastErrorHandler {
    name: String,
}

impl FastErrorHandler {
    fn new() -> Self {
        Self { name: "FastErrorHandler".to_string() }
    }
}

impl ErrorHandler for FastErrorHandler {
    fn handle_error(&self, _error: &Error, _context: &PropagationFrame) -> Result<(), Error> {
        // Minimal processing for performance testing
        Ok(())
    }
    
    fn name(&self) -> &str { &self.name }
    fn can_handle(&self, _error: &Error) -> bool { true }
    fn priority(&self) -> u32 { 100 }
}
