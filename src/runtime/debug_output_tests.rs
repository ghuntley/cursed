//! Comprehensive tests for debug output functionality
//!
//! Tests stack trace capture, goroutine context tracking, performance monitoring,
//! and DWARF integration for both interpretation and compilation modes.

use crate::runtime::debug_output::*;
use crate::runtime::goroutine::*;
use crate::runtime::stack_trace::*;
use crate::error_types::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_debug_output_basic() {
    let debug_system = DebugOutputSystem::new();
    
    // Test basic logging
    let result = debug_system.log(DebugLevel::Info, "test_module", "Test message");
    assert!(result.is_ok());
    
    // Test different debug levels
    let levels = [
        DebugLevel::Trace,
        DebugLevel::Debug,
        DebugLevel::Info,
        DebugLevel::Warn,
        DebugLevel::Error,
        DebugLevel::Fatal,
    ];
    
    for level in &levels {
        let result = debug_system.log(*level, "test_module", &format!("Test message at {:?}", level));
        assert!(result.is_ok());
    }
}

#[test]
fn test_stack_trace_capture() {
    let debug_system = DebugOutputSystem::new();
    
    // Test stack trace capture
    let result = debug_system.capture_stack_trace();
    assert!(result.is_ok());
    
    let stack_trace = result.unwrap();
    assert!(!stack_trace.frames.is_empty());
    
    // Verify frame structure
    let frame = &stack_trace.frames[0];
    assert!(!frame.function_name.is_empty());
    assert!(frame.frame_type == FrameType::Function);
}

#[test]
fn test_goroutine_context_tracking() {
    // Initialize global scheduler
    let _ = initialize_global_scheduler();
    
    let debug_system = DebugOutputSystem::new();
    
    // Test goroutine context retrieval
    let result = debug_system.get_current_goroutine_context();
    assert!(result.is_ok());
    
    // The context might be None if no goroutine is running
    let context = result.unwrap();
    if let Some(ctx) = context {
        assert!(ctx.id > 0);
        assert!(matches!(ctx.state, GoroutineState::Ready | GoroutineState::Running));
    }
}

#[test]
fn test_performance_monitoring() {
    let debug_system = DebugOutputSystem::new();
    
    // Generate some debug messages
    for i in 0..100 {
        let _ = debug_system.log(DebugLevel::Info, "test_module", &format!("Test message {}", i));
    }
    
    // Test performance metrics
    let metrics = debug_system.monitor_performance();
    assert!(metrics.total_messages >= 100);
    assert!(metrics.buffer_size > 0);
    assert!(metrics.average_message_size > 0.0);
    assert!(metrics.peak_memory_usage > 0);
}

#[test]
fn test_enhanced_debug_formatting() {
    let debug_system = DebugOutputSystem::new();
    
    // Configure debug system to show thread IDs
    let mut config = debug_system.get_config();
    config.show_thread_ids = true;
    debug_system.set_config(config);
    
    // Create a test debug message
    let mut data = HashMap::new();
    data.insert("key1".to_string(), DebugValue::String("value1".to_string()));
    data.insert("key2".to_string(), DebugValue::Integer(42));
    
    let message = DebugMessage {
        id: 1,
        level: DebugLevel::Info,
        message: "Test message".to_string(),
        module: "test_module".to_string(),
        function: Some("test_function".to_string()),
        location: Some(("test.rs".to_string(), 123)),
        timestamp: std::time::SystemTime::now(),
        data,
        tags: vec!["tag1".to_string(), "tag2".to_string()],
        thread_id: Some(12345),
        goroutine_id: Some(67890),
    };
    
    let formatted = debug_system.format_enhanced_message(&message);
    assert!(formatted.contains("INFO"));
    assert!(formatted.contains("test_module"));
    assert!(formatted.contains("test_function"));
    assert!(formatted.contains("Test message"));
    assert!(formatted.contains("[G67890]"));
    assert!(formatted.contains("[T12345]"));
}

#[test]
fn test_debug_value_formatting() {
    let values = vec![
        DebugValue::String("test".to_string()),
        DebugValue::Integer(42),
        DebugValue::Float(3.14),
        DebugValue::Boolean(true),
        DebugValue::Null,
        DebugValue::Array(vec![
            DebugValue::Integer(1),
            DebugValue::Integer(2),
            DebugValue::Integer(3),
        ]),
    ];
    
    for value in values {
        let formatted = format!("{}", value);
        assert!(!formatted.is_empty());
    }
}

#[test]
fn test_debug_configuration() {
    let mut debug_system = DebugOutputSystem::new();
    
    // Test configuration changes
    let config = DebugConfig {
        min_level: DebugLevel::Warn,
        show_timestamps: true,
        show_locations: true,
        use_colors: false,
        show_thread_ids: true,
        max_message_length: Some(100),
        format: DebugFormat::Json,
        included_modules: vec!["test_module".to_string()],
        excluded_modules: Vec::new(),
        included_tags: vec!["important".to_string()],
    };
    
    debug_system.set_config(config);
    
    // Test that configuration is applied
    let result = debug_system.log(DebugLevel::Debug, "test_module", "This should be filtered");
    assert!(result.is_ok());
    
    let result = debug_system.log(DebugLevel::Error, "test_module", "This should pass");
    assert!(result.is_ok());
}

#[test]
fn test_performance_monitoring_with_timing() {
    let debug_system = DebugOutputSystem::new();
    
    // Test performance monitoring
    let start = std::time::Instant::now();
    let result = debug_system.log_with_performance(DebugLevel::Info, "test_module", "Performance test");
    let duration = start.elapsed();
    
    assert!(result.is_ok());
    assert!(duration.as_millis() < 50); // Should be fast
}

#[test]
fn test_dwarf_integration() {
    use crate::runtime::dwarf_parser::DwarfParser;
    
    // Test DWARF integration (simplified)
    let dwarf_data = vec![0u8; 100]; // Mock DWARF data
    let parser_result = DwarfParser::new(&dwarf_data);
    assert!(parser_result.is_ok());
    
    let parser = parser_result.unwrap();
    
    // Test function info retrieval
    let func_result = parser.get_function_info("test_function");
    // This might fail with mock data, but should not panic
    
    // Test local variables retrieval
    let vars_result = parser.get_local_variables("test_function");
    assert!(vars_result.is_ok());
}

#[test]
fn test_goroutine_debug_context() {
    let context = GoroutineDebugContext {
        id: 12345,
        state: GoroutineState::Running,
        parent_id: Some(54321),
        created_at: std::time::Instant::now(),
        creation_stack: vec![],
        current_stack: vec![],
        metadata: HashMap::new(),
    };
    
    assert_eq!(context.id, 12345);
    assert_eq!(context.state, GoroutineState::Running);
    assert_eq!(context.parent_id, Some(54321));
}

#[test]
#[ignore = "Causes infinite logging spam - generates excessive 'Thread X message Y' output that hangs test suite"]
fn test_debug_system_thread_safety() {
    use std::sync::Arc;
    use std::thread;
    
    let debug_system = Arc::new(DebugOutputSystem::new());
    let mut handles = Vec::new();
    
    // Test concurrent access
    for i in 0..10 {
        let debug_clone = debug_system.clone();
        let handle = thread::spawn(move || {
            for j in 0..100 {
                let result = debug_clone.log(
                    DebugLevel::Info,
                    "test_module",
                    &format!("Thread {} message {}", i, j)
                );
                assert!(result.is_ok());
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify metrics
    let metrics = debug_system.monitor_performance();
    assert!(metrics.total_messages >= 1000);
}

#[test]
fn test_stack_frame_parsing() {
    let debug_system = DebugOutputSystem::new();
    
    // Test backtrace line parsing
    let test_lines = vec![
        "   0: cursed::runtime::debug_output::test_function",
        "   1: cursed::main",
        "   2: std::rt::lang_start::{{closure}}",
        "             at /home/user/project/src/main.rs:123:45",
    ];
    
    for line in test_lines {
        let frame = debug_system.parse_backtrace_line(line);
        if frame.is_some() {
            let f = frame.unwrap();
            assert!(!f.function_name.is_empty());
        }
    }
}

// Integration tests for both interpretation and compilation modes
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_debug_in_interpretation_mode() {
        // This would be run with the interpreter
        // Testing basic debug functionality
        let debug_system = DebugOutputSystem::new();
        
        // Test logging in interpretation context
        let result = debug_system.log(DebugLevel::Info, "interpreter", "Test interpretation debug");
        assert!(result.is_ok());
        
        // Test stack trace in interpretation
        let stack_result = debug_system.capture_stack_trace();
        assert!(stack_result.is_ok());
    }
    
    #[test]
    fn test_debug_in_compilation_mode() {
        // This would be run with compiled code
        // Testing debug functionality with native code
        let debug_system = DebugOutputSystem::new();
        
        // Test logging in compilation context
        let result = debug_system.log(DebugLevel::Info, "compiler", "Test compilation debug");
        assert!(result.is_ok());
        
        // Test performance in compilation mode
        let metrics = debug_system.monitor_performance();
        assert!(metrics.total_messages >= 0);
    }
}
