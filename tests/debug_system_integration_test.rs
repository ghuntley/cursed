/// Comprehensive integration tests for the CURSED debug system
///
/// Tests the complete stack trace and debug information system including:
/// - Stack trace generation and formatting
/// - Debug information capture and resolution
/// - LLVM debug integration
/// - Gen Z themed debug output
/// - Integration with panic and error systems

use cursed::runtime::{
    StackTraceManager, StackTrace, CallFrame, DebugInfo, StackTraceConfig,
    StackWalker, RawStackFrame, ContextualStackWalk, StackWalkConfig,
    DebugFormatter, DebugOutputConfig, GenZMessages, format_panic_trace,
    EnhancedStackTrace, EnhancedStackFrame, VariableInfo
};
use cursed::error::{Error as CursedError, SourceLocation};
use cursed::debug::enhanced_debug::{EnhancedDebugInfo, DebugInfoRegistry};
use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;
use std::time::{SystemTime, Duration};

#[path = "common.rs"]
mod common;

/// Test basic stack trace manager functionality
#[test]
fn test_stack_trace_manager_basic() {
    common::tracing::setup();
    
    let manager = StackTraceManager::new();
    
    // Test function entry/exit
    let params = HashMap::new();
    let location = SourceLocation::new(10, 5).with_file("test.csd");
    
    let result = manager.enter_function(
        "test_slay_function".to_string(),
        Some("test_module".to_string()),
        Some(location),
        params
    );
    assert!(result.is_ok());
    
    assert_eq!(manager.get_call_depth(), 1);
    assert_eq!(manager.get_current_function(), Some("test_slay_function".to_string()));
    
    let exit_result = manager.exit_function(Some("test_slay_function".to_string()));
    assert!(exit_result.is_ok());
    assert_eq!(manager.get_call_depth(), 0);
}

/// Test stack trace capture with multiple frames
#[test]
fn test_stack_trace_capture_multiple_frames() {
    common::tracing::setup();
    
    let manager = StackTraceManager::new();
    
    // Build up a call stack
    let params = HashMap::new();
    
    // Frame 1: main function
    let _ = manager.enter_function(
        "main".to_string(),
        Some("main".to_string()),
        Some(SourceLocation::new(1, 1).with_file("main.csd")),
        params.clone()
    );
    
    // Frame 2: yolo function
    let _ = manager.enter_function(
        "yolo_calculate".to_string(),
        Some("math".to_string()),
        Some(SourceLocation::new(25, 10).with_file("math.csd")),
        params.clone()
    );
    
    // Frame 3: slay function
    let _ = manager.enter_function(
        "slay_dragons".to_string(),
        Some("game".to_string()),
        Some(SourceLocation::new(42, 5).with_file("game.csd")),
        params
    );
    
    // Capture the trace
    let trace = manager.capture_stack_trace(Some(123)).unwrap();
    
    assert_eq!(trace.frames.len(), 3);
    assert_eq!(trace.goroutine_id, Some(123));
    
    // Check frame order (most recent first)
    assert_eq!(trace.frames[0].function_name, "slay_dragons");
    assert_eq!(trace.frames[1].function_name, "yolo_calculate");
    assert_eq!(trace.frames[2].function_name, "main");
    
    // Check frame details
    assert_eq!(trace.frames[0].module_name, Some("game".to_string()));
    assert!(trace.frames[0].source_location.is_some());
}

/// Test stack walker functionality
#[test]
fn test_stack_walker_basic() {
    common::tracing::setup();
    
    let walker = StackWalker::new();
    
    // Walk the current stack
    let frames = walker.walk_stack().unwrap();
    
    // Should have at least some frames from this test
    assert!(!frames.is_empty());
    
    // Check that we can access frame information
    for frame in &frames {
        assert!(frame.instruction_pointer > 0);
        // Some frames should have symbol names
    }
    
    // Get statistics
    let stats = walker.get_statistics().unwrap();
    assert!(stats.total_walks > 0);
    assert!(stats.total_frames > 0);
}

/// Test stack walker with configuration
#[test]
fn test_stack_walker_with_config() {
    common::tracing::setup();
    
    let config = StackWalkConfig {
        max_frames: 5,
        skip_system_frames: true,
        cursed_frames_only: false,
        ..Default::default()
    };
    
    let walker = StackWalker::with_config(config);
    let frames = walker.walk_stack().unwrap();
    
    // Should respect max_frames limit
    assert!(frames.len() <= 5);
}

/// Test contextual stack walk
#[test]
fn test_contextual_stack_walk() {
    common::tracing::setup();
    
    let walker = StackWalker::new();
    
    let contextual = walker.walk_stack_with_context(None, Some(456)).unwrap();
    
    assert_eq!(contextual.goroutine_id, Some(456));
    assert_eq!(contextual.thread_id, thread::current().id());
    assert!(!contextual.frames.is_empty());
    
    // Test frame access methods
    let top_frame = contextual.top_frame();
    assert!(top_frame.is_some());
    
    let cursed_frames = contextual.cursed_frames();
    // Might not have any CURSED frames in this test, but should not panic
    assert!(cursed_frames.len() <= contextual.frames.len());
}

/// Test debug formatter with basic stack trace
#[test]
fn test_debug_formatter_basic() {
    common::tracing::setup();
    
    let mut formatter = DebugFormatter::new();
    
    // Create a test stack trace
    let mut trace = StackTrace::new(10);
    trace.trace_id = 42;
    
    let frame = CallFrame::new("test_periodt_function".to_string(), 0)
        .with_location(SourceLocation::new(15, 8).with_file("test.csd"))
        .with_module("test_module".to_string())
        .with_parameter("param1".to_string(), "\"hello bestie\"".to_string());
    
    trace.add_frame(frame);
    
    let output = formatter.format_stack_trace(&trace).unwrap();
    
    // Check for expected content
    assert!(output.contains("Stack Trace"));
    assert!(output.contains("test_periodt_function"));
    assert!(output.contains("Trace ID: 42"));
    assert!(output.contains("test.csd"));
    assert!(output.contains("param1"));
}

/// Test debug formatter with Gen Z theming
#[test]
fn test_debug_formatter_gen_z_theming() {
    common::tracing::setup();
    
    let config = DebugOutputConfig {
        use_gen_z_slang: true,
        use_colors: false, // Disable colors for easier testing
        ..Default::default()
    };
    
    let mut formatter = DebugFormatter::with_config(config);
    
    let mut trace = StackTrace::new(10);
    let frame = CallFrame::new("slay_function".to_string(), 0);
    trace.add_frame(frame);
    
    let output = formatter.format_stack_trace(&trace).unwrap();
    
    // Should contain Gen Z slang
    assert!(output.contains("💥") || output.contains("ain't it chief") || output.contains("bestie"));
}

/// Test enhanced stack trace formatting
#[test]
fn test_enhanced_stack_trace_formatting() {
    common::tracing::setup();
    
    let mut formatter = DebugFormatter::new();
    
    // Create enhanced debug info
    let debug_info = crate::runtime::debug_info::DebugInfo::new(
        "enhanced.csd",
        30,
        12,
        "enhanced_yolo_function".to_string()
    ).with_module("enhanced_module".to_string());
    
    // Create enhanced frame
    let frame = EnhancedStackFrame::new(debug_info, 0)
        .with_inlined(true)
        .with_optimization_level("O2".to_string());
    
    // Create enhanced trace
    let trace = EnhancedStackTrace::new()
        .with_frames(vec![frame])
        .with_goroutine(789);
    
    let output = formatter.format_enhanced_stack_trace(&trace).unwrap();
    
    assert!(output.contains("Enhanced Stack Trace"));
    assert!(output.contains("enhanced_yolo_function"));
    assert!(output.contains("enhanced_module"));
    assert!(output.contains("[inlined]"));
    assert!(output.contains("O2"));
    assert!(output.contains("goroutine #789"));
}

/// Test variable information formatting
#[test]
fn test_variable_formatting() {
    common::tracing::setup();
    
    let formatter = DebugFormatter::new();
    
    let var = VariableInfo::new("my_sus_variable".to_string(), "sus".to_string())
        .with_value("42".to_string())
        .with_mutability(true)
        .with_scope_depth(2)
        .with_location("test.csd:25".to_string());
    
    let formatted = formatter.format_variable_info(&var);
    
    assert!(formatted.contains("mut"));
    assert!(formatted.contains("my_sus_variable"));
    assert!(formatted.contains("sus"));
    assert!(formatted.contains("42"));
    assert!(formatted.contains("test.csd:25"));
    assert!(formatted.contains("scope: 2"));
}

/// Test Gen Z messages
#[test]
fn test_gen_z_messages() {
    common::tracing::setup();
    
    // Test panic messages
    let null_panic = GenZMessages::panic_message("null_pointer");
    assert!(null_panic.contains("bestie"));
    assert!(null_panic.contains("💀"));
    
    let bounds_panic = GenZMessages::panic_message("index_out_of_bounds");
    assert!(bounds_panic.contains("out of bounds"));
    assert!(bounds_panic.contains("📍"));
    
    // Test success messages
    let compilation_success = GenZMessages::success_message("compilation");
    assert!(compilation_success.contains("✨"));
    assert!(compilation_success.contains("SENDING"));
    
    // Test warning messages
    let performance_warning = GenZMessages::warning_message("performance");
    assert!(performance_warning.contains("⚠️"));
    assert!(performance_warning.contains("slow vibes"));
}

/// Test panic trace formatting
#[test]
fn test_panic_trace_formatting() {
    common::tracing::setup();
    
    let mut trace = StackTrace::new(10);
    
    // Add some frames that look like a panic scenario
    let panic_frame = CallFrame::new("panic_handler".to_string(), 0)
        .with_location(SourceLocation::new(100, 1).with_file("runtime.rs"));
    
    let user_frame = CallFrame::new("user_yolo_function".to_string(), 1)
        .with_location(SourceLocation::new(25, 5).with_file("user.csd"))
        .with_module("user_module".to_string());
    
    trace.add_frame(panic_frame);
    trace.add_frame(user_frame);
    
    let output = format_panic_trace(&trace).unwrap();
    
    assert!(output.contains("PANIC") || output.contains("💥"));
    assert!(output.contains("panic_handler"));
    assert!(output.contains("user_yolo_function"));
}

/// Test debug output with source context (mocked)
#[test]
fn test_debug_output_with_source_context() {
    common::tracing::setup();
    
    // Create a temporary source file for testing
    let temp_file = std::env::temp_dir().join("test_cursed_source.csd");
    let source_content = r#"// Test CURSED source file
slay calculate_vibes(sus x, sus y) -> sus {
    facts result = x + y;
    lowkey (result > 100) {
        periodt; // This is the error line
    }
    bestie result;
}
"#;
    
    std::fs::write(&temp_file, source_content).unwrap();
    
    let mut formatter = DebugFormatter::new();
    
    let location = SourceLocation::new(5, 9).with_file(&temp_file.to_string_lossy());
    let context = formatter.get_source_context(&location).unwrap();
    
    // Should contain source lines with the error highlighted
    assert!(context.contains("Source context"));
    assert!(context.contains("periodt"));
    assert!(context.contains(">"));  // Error line marker
    
    // Clean up
    let _ = std::fs::remove_file(&temp_file);
}

/// Test CURSED function detection
#[test]
fn test_cursed_function_detection() {
    common::tracing::setup();
    
    let formatter = DebugFormatter::new();
    
    // Should detect CURSED keywords
    assert!(formatter.is_cursed_function("slay_dragons"));
    assert!(formatter.is_cursed_function("yolo_adventure"));
    assert!(formatter.is_cursed_function("periodt_check"));
    assert!(formatter.is_cursed_function("sus_behavior"));
    
    // Should not detect regular functions
    assert!(!formatter.is_cursed_function("regular_function"));
    assert!(!formatter.is_cursed_function("std::vec::push"));
}

/// Test integration with existing error system
#[test]
fn test_error_system_integration() {
    common::tracing::setup();
    
    let error = CursedError::Runtime("Something went wrong bestie!".to_string());
    
    // Create a trace for the error
    let mut trace = StackTrace::new(10);
    let frame = CallFrame::new("error_causing_function".to_string(), 0)
        .with_location(SourceLocation::new(50, 15).with_file("error.csd"));
    trace.add_frame(frame);
    
    let output = cursed::runtime::debug_output::format_error_with_context(&error, Some(&trace)).unwrap();
    
    assert!(output.contains("ERROR") || output.contains("❌"));
    assert!(output.contains("Something went wrong bestie!"));
    assert!(output.contains("error_causing_function"));
    assert!(output.contains("Stack trace"));
}

/// Test thread safety of debug systems
#[test]
fn test_thread_safety() {
    common::tracing::setup();
    
    let manager = std::sync::Arc::new(StackTraceManager::new());
    let walker = std::sync::Arc::new(StackWalker::new());
    
    let handles: Vec<_> = (0..5).map(|i| {
        let manager_clone = manager.clone();
        let walker_clone = walker.clone();
        
        thread::spawn(move || {
            let params = HashMap::new();
            let location = SourceLocation::new(i * 10, 5).with_file(&format!("thread_{}.csd", i));
            
            // Test stack trace manager
            let _ = manager_clone.enter_function(
                format!("thread_function_{}", i),
                Some(format!("thread_module_{}", i)),
                Some(location),
                params
            );
            
            let trace = manager_clone.capture_stack_trace(Some(i as u64));
            assert!(trace.is_ok());
            
            // Test stack walker
            let frames = walker_clone.walk_stack();
            assert!(frames.is_ok());
            
            let _ = manager_clone.exit_function(Some(format!("thread_function_{}", i)));
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify statistics
    let stats = manager.get_statistics().unwrap();
    assert!(stats.total_traces >= 5);
    
    let walker_stats = walker.get_statistics().unwrap();
    assert!(walker_stats.total_walks >= 5);
}

/// Test performance of debug systems
#[test]
fn test_debug_system_performance() {
    common::tracing::setup();
    
    let manager = StackTraceManager::new();
    
    let start_time = std::time::Instant::now();
    
    // Perform many stack operations
    for i in 0..1000 {
        let params = HashMap::new();
        let location = SourceLocation::new(i % 100 + 1, 5).with_file("perf_test.csd");
        
        let _ = manager.enter_function(
            format!("perf_function_{}", i % 10),
            Some("perf_module".to_string()),
            Some(location),
            params
        );
        
        if i % 100 == 0 {
            let _ = manager.capture_stack_trace(None);
        }
        
        let _ = manager.exit_function(Some(format!("perf_function_{}", i % 10)));
    }
    
    let elapsed = start_time.elapsed();
    
    // Should complete in reasonable time (less than 1 second for 1000 operations)
    assert!(elapsed < Duration::from_secs(1));
    
    let stats = manager.get_statistics().unwrap();
    assert!(stats.total_traces >= 10); // At least 10 traces captured
}

/// Test debug system memory usage
#[test]
fn test_debug_memory_efficiency() {
    common::tracing::setup();
    
    let manager = StackTraceManager::new();
    
    // Build up a large call stack
    for i in 0..50 {
        let params = HashMap::new();
        let _ = manager.enter_function(
            format!("deep_function_{}", i),
            Some("deep_module".to_string()),
            Some(SourceLocation::new(i + 1, 5).with_file("deep.csd")),
            params
        );
    }
    
    // Capture trace
    let trace = manager.capture_stack_trace(None).unwrap();
    assert_eq!(trace.frames.len(), 50);
    
    // Clear the stack
    manager.clear_stack();
    assert_eq!(manager.get_call_depth(), 0);
}

/// Test configuration and customization
#[test]
fn test_configuration_customization() {
    common::tracing::setup();
    
    // Test custom stack trace config
    let stack_config = StackTraceConfig {
        max_frames: 25,
        capture_variables: false,
        capture_parameters: true,
        capture_rust_backtrace: false,
        ..Default::default()
    };
    
    let manager = StackTraceManager::with_config(stack_config);
    
    // Test custom stack walk config
    let walk_config = StackWalkConfig {
        max_frames: 15,
        skip_system_frames: false,
        cursed_frames_only: true,
        ..Default::default()
    };
    
    let walker = StackWalker::with_config(walk_config);
    
    // Test custom debug output config
    let output_config = DebugOutputConfig {
        use_gen_z_slang: false,
        show_source_context: false,
        compact_mode: true,
        max_display_frames: 10,
        ..Default::default()
    };
    
    let formatter = DebugFormatter::with_config(output_config);
    
    // Verify configurations are applied
    assert!(!formatter.config.use_gen_z_slang);
    assert!(formatter.config.compact_mode);
    assert_eq!(formatter.config.max_display_frames, 10);
}
