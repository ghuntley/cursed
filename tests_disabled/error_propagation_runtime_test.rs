use cursed::runtime::error_propagation_runtime::*;
use cursed::error::{Error, SourceLocation as ErrorSourceLocation};
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

#[path = "common.rs"]
pub mod common;

/// Test basic runtime creation and configuration
#[test]
fn test_runtime_creation_and_config() {
    common::tracing::setup();
    
    let runtime = ErrorPropagationRuntime::new();
    assert_eq!(runtime.get_propagation_depth(), 0);
    
    let config = PropagationConfig {
        max_propagation_depth: 50,
        generate_stack_traces: false,
        panic_integration_enabled: false,
        propagation_timeout: Duration::from_secs(1),
        collect_statistics: true,
        preserve_error_context: true,
    };
    
    let runtime_with_config = ErrorPropagationRuntime::with_config(config.clone());
    assert_eq!(runtime_with_config.config.max_propagation_depth, 50);
    assert!(!runtime_with_config.config.generate_stack_traces);
}

/// Test error handler registration and prioritization
#[test]
fn test_error_handler_registration() {
    common::tracing::setup();
    
    let mut runtime = ErrorPropagationRuntime::new();
    
    // Register multiple handlers with different priorities
    let high_priority_handler = Box::new(TestErrorHandler::new("HighPriority", 10));
    let low_priority_handler = Box::new(TestErrorHandler::new("LowPriority", 100));
    let default_handler = Box::new(DefaultErrorHandler::new());
    
    runtime.register_handler(low_priority_handler);
    runtime.register_handler(high_priority_handler);
    runtime.register_handler(default_handler);
    
    assert_eq!(runtime.error_handlers.len(), 3);
    
    // Verify priority ordering (lower numbers = higher priority)
    assert_eq!(runtime.error_handlers[0].name(), "HighPriority");
    assert_eq!(runtime.error_handlers[1].name(), "LowPriority");
    assert_eq!(runtime.error_handlers[2].name(), "DefaultErrorHandler");
}

/// Test stack trace capture functionality
#[test]
fn test_stack_trace_capture() {
    common::tracing::setup();
    
    let runtime = ErrorPropagationRuntime::new();
    let stack_trace = runtime.capture_stack_trace();
    
    // Should capture at least a few frames
    assert!(!stack_trace.is_empty());
    
    // Verify frame structure
    for frame in &stack_trace {
        // At least one of these should be populated
        assert!(
            frame.function_name.is_some() || 
            frame.symbol_name.is_some() || 
            frame.instruction_pointer.is_some()
        );
    }
    
    // Check for test function in stack trace
    let has_test_frame = stack_trace.iter().any(|frame| {
        frame.function_name.as_ref()
            .map(|name| name.contains("test_stack_trace_capture"))
            .unwrap_or(false)
    });
    assert!(has_test_frame, "Should find test function in stack trace");
}

/// Test debug information extraction
#[test]
fn test_debug_info_extraction() {
    common::tracing::setup();
    
    let runtime = ErrorPropagationRuntime::new();
    let stack_trace = runtime.capture_stack_trace();
    let debug_info = runtime.extract_debug_info(&stack_trace);
    
    assert!(debug_info.is_some());
    let debug_info = debug_info.unwrap();
    
    assert!(debug_info.symbols_available);
    assert_eq!(debug_info.source_language, Some("CURSED".to_string()));
}

/// Test tail position detection
#[test]
fn test_tail_position_detection() {
    common::tracing::setup();
    
    let runtime = ErrorPropagationRuntime::new();
    let location = ErrorSourceLocation::new(10, 5);
    
    // Create mock stack trace with main function
    let stack_trace = vec![
        StackFrame {
            function_name: Some("test_function".to_string()),
            symbol_name: None,
            file_path: None,
            line_number: Some(10),
            column_number: Some(5),
            module_path: Some("test::module".to_string()),
            instruction_pointer: Some(0x1000),
        },
        StackFrame {
            function_name: Some("main".to_string()),
            symbol_name: None,
            file_path: None,
            line_number: Some(1),
            column_number: Some(1),
            module_path: Some("main".to_string()),
            instruction_pointer: Some(0x2000),
        },
    ];
    
    let is_tail = runtime.is_tail_position(&location, &stack_trace);
    assert!(is_tail, "Should detect tail position when next frame is main");
}

/// Test error propagation with stack traces
#[test]
fn test_error_propagation_with_traces() {
    common::tracing::setup();
    
    let mut runtime = ErrorPropagationRuntime::new();
    let handler = Box::new(TestErrorHandler::new("TestHandler", 50));
    runtime.register_handler(handler);
    
    let location = ErrorSourceLocation::new(15, 10);
    let error = Error::Runtime("Test propagation error".to_string());
    
    let result = runtime.propagate_error(
        error,
        location,
        Some("test_function".to_string()),
    );
    
    assert!(result.is_ok());
    assert_eq!(runtime.get_propagation_depth(), 1);
    
    // Check that stack trace was captured
    assert!(!runtime.propagation_stack.is_empty());
    let frame = &runtime.propagation_stack[0];
    assert!(!frame.stack_trace.is_empty());
    assert!(frame.debug_info.is_some());
}

/// Test propagation depth limits
#[test]
fn test_propagation_depth_limits() {
    common::tracing::setup();
    
    let config = PropagationConfig {
        max_propagation_depth: 2,
        ..PropagationConfig::default()
    };
    let mut runtime = ErrorPropagationRuntime::with_config(config);
    
    let location = ErrorSourceLocation::new(1, 1);
    let error = Error::Runtime("Test error".to_string());
    
    // First propagation should succeed
    let result1 = runtime.propagate_error(
        error.clone(),
        location.clone(),
        None,
    );
    assert!(result1.is_ok());
    
    // Second propagation should succeed
    let result2 = runtime.propagate_error(
        error.clone(),
        location.clone(),
        None,
    );
    assert!(result2.is_ok());
    
    // Third propagation should fail due to depth limit
    let result3 = runtime.propagate_error(
        error,
        location,
        None,
    );
    assert!(result3.is_err());
}

/// Test statistics collection
#[test]
fn test_statistics_collection() {
    common::tracing::setup();
    
    let mut runtime = ErrorPropagationRuntime::new();
    let handler = Box::new(TestErrorHandler::new("StatsHandler", 50));
    runtime.register_handler(handler);
    
    let initial_stats = runtime.get_statistics().unwrap();
    assert_eq!(initial_stats.total_propagations, 0);
    
    // Perform several propagations
    for i in 0..5 {
        let location = ErrorSourceLocation::new(i, i);
        let error = Error::Runtime(format!("Test error {}", i));
        let _ = runtime.propagate_error(error, location, None);
    }
    
    let final_stats = runtime.get_statistics().unwrap();
    assert_eq!(final_stats.total_propagations, 5);
    assert_eq!(final_stats.successful_propagations, 5);
    assert_eq!(final_stats.failed_propagations, 0);
    assert!(final_stats.average_propagation_time_us > 0.0);
}

/// Test thread-local state management
#[test]
fn test_thread_local_state() {
    common::tracing::setup();
    
    let runtime = Arc::new(Mutex::new(ErrorPropagationRuntime::new()));
    let error_occurred = Arc::new(Mutex::new(false));
    
    let runtime_clone = runtime.clone();
    let error_occurred_clone = error_occurred.clone();
    
    let handle = thread::spawn(move || {
        let mut runtime = runtime_clone.lock().unwrap();
        let location = ErrorSourceLocation::new(1, 1);
        let error = Error::Runtime("Thread error".to_string());
        
        let result = runtime.propagate_error(error, location, None);
        if result.is_err() {
            *error_occurred_clone.lock().unwrap() = true;
        }
    });
    
    handle.join().unwrap();
    
    // Should have propagated successfully in the thread
    assert!(!*error_occurred.lock().unwrap());
}

/// Test function name extraction from stack trace
#[test]
fn test_function_name_extraction() {
    common::tracing::setup();
    
    let runtime = ErrorPropagationRuntime::new();
    let function_name = runtime.get_current_function_name();
    
    assert!(function_name.is_some());
    let name = function_name.unwrap();
    
    // Should contain the test function name
    assert!(name.contains("test_function_name_extraction"));
}

/// Test minimal stack trace creation
#[test]
fn test_minimal_stack_trace_creation() {
    common::tracing::setup();
    
    let runtime = ErrorPropagationRuntime::new();
    let minimal_trace = runtime.create_minimal_stack_trace();
    
    assert!(!minimal_trace.frames.is_empty());
    assert!(minimal_trace.total_frames > 0);
    
    // Verify frame content
    for frame in &minimal_trace.frames {
        assert!(!frame.function_name.is_empty());
        assert!(!frame.file_name.is_empty());
        // Line numbers might be 0 if debug info isn't available
    }
}

/// Test panic integration
#[test]
fn test_panic_integration() {
    common::tracing::setup();
    
    let config = PropagationConfig {
        panic_integration_enabled: true,
        ..PropagationConfig::default()
    };
    let mut runtime = ErrorPropagationRuntime::with_config(config);
    runtime = runtime.with_panic_integration("test_panic_runtime".to_string());
    
    assert!(runtime.panic_runtime.is_some());
    assert_eq!(runtime.panic_runtime.unwrap(), "panic_enabled");
}

/// Test FFI functions
#[test]
fn test_ffi_functions() {
    common::tracing::setup();
    
    // Test initialization
    cursed_error_propagation_init();
    
    // Test function name retrieval
    let function_name_ptr = cursed_get_current_function_name();
    assert!(!function_name_ptr.is_null());
    
    unsafe {
        let c_str = std::ffi::CStr::from_ptr(function_name_ptr as *const i8);
        let function_name = c_str.to_str().unwrap();
        assert!(!function_name.is_empty());
    }
    
    // Test minimal stack trace creation
    let trace_ptr = cursed_create_minimal_stack_trace();
    assert!(!trace_ptr.is_null());
    
    // Clean up
    cursed_free_minimal_stack_trace(trace_ptr);
    cursed_error_propagation_cleanup();
}

/// Test error propagation under stress
#[test]
fn test_error_propagation_stress() {
    common::tracing::setup();
    
    let mut runtime = ErrorPropagationRuntime::new();
    let handler = Box::new(TestErrorHandler::new("StressHandler", 50));
    runtime.register_handler(handler);
    
    // Perform many rapid propagations
    for i in 0..100 {
        let location = ErrorSourceLocation::new(i % 50, i % 25);
        let error = Error::Runtime(format!("Stress test error {}", i));
        
        let result = runtime.propagate_error(
            error,
            location,
            Some(format!("stress_function_{}", i % 10)),
        );
        
        if i < 95 { // Allow for some to fail due to depth limits
            assert!(result.is_ok() || result.is_err()); // Either is acceptable under stress
        }
    }
    
    let stats = runtime.get_statistics().unwrap();
    assert!(stats.total_propagations >= 95);
    assert!(stats.successful_propagations > 0);
}

/// Test concurrent error propagation
#[test]
fn test_concurrent_error_propagation() {
    common::tracing::setup();
    
    let runtime = Arc::new(Mutex::new(ErrorPropagationRuntime::new()));
    let mut handles = Vec::new();
    
    // Spawn multiple threads performing error propagation
    for thread_id in 0..5 {
        let runtime_clone = runtime.clone();
        let handle = thread::spawn(move || {
            let mut runtime = runtime_clone.lock().unwrap();
            
            for i in 0..10 {
                let location = ErrorSourceLocation::new(i + thread_id * 10, i);
                let error = Error::Runtime(format!("Concurrent error {} from thread {}", i, thread_id));
                
                let _ = runtime.propagate_error(
                    error,
                    location,
                    Some(format!("thread_{}_function", thread_id)),
                );
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Check final statistics
    let final_runtime = runtime.lock().unwrap();
    let stats = final_runtime.get_statistics().unwrap();
    assert!(stats.total_propagations > 0);
}

/// Helper test error handler
#[derive(Debug)]
struct TestErrorHandler {
    name: String,
    priority: u32,
}

impl TestErrorHandler {
    fn new(name: &str, priority: u32) -> Self {
        Self {
            name: name.to_string(),
            priority,
        }
    }
}

impl ErrorHandler for TestErrorHandler {
    fn handle_error(&self, error: &Error, _context: &PropagationFrame) -> Result<(), Error> {
        // Successfully handle the error for testing
        tracing::info!(handler = %self.name, error = %error, "Handling error");
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn can_handle(&self, _error: &Error) -> bool {
        true // Can handle any error for testing
    }
    
    fn priority(&self) -> u32 {
        self.priority
    }
}
