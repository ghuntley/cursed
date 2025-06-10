/// Simple test for the debug system to check basic compilation
use cursed::runtime::{
    StackTraceManager, StackTrace, CallFrame,
    StackWalker, DebugFormatter, DebugOutputConfig
};
use cursed::error::SourceLocation;
use std::collections::HashMap;

#[path = "common.rs"]
mod common;

#[test]
fn test_basic_debug_components() {
    common::tracing::setup();
    
    // Test stack trace manager creation
    let manager = StackTraceManager::new();
    assert_eq!(manager.get_call_depth(), 0);
    
    // Test stack walker creation
    let walker = StackWalker::new();
    assert_eq!(walker.config.max_frames, 100);
    
    // Test debug formatter creation
    let formatter = DebugFormatter::new();
    assert!(formatter.config.use_colors);
    
    // Test basic call frame creation
    let frame = CallFrame::new("test_function".to_string(), 0)
        .with_location(SourceLocation::new(10, 5))
        .with_module("test_module".to_string());
    
    assert_eq!(frame.function_name, "test_function");
    assert_eq!(frame.depth, 0);
    assert!(frame.source_location.is_some());
    
    // Test basic trace creation
    let mut trace = StackTrace::new(10);
    trace.add_frame(frame);
    assert_eq!(trace.frames.len(), 1);
}

#[test]
fn test_stack_walker_basic() {
    common::tracing::setup();
    
    let walker = StackWalker::new();
    
    // This should work with the mock implementation
    let result = walker.walk_stack();
    assert!(result.is_ok());
    
    let frames = result.unwrap();
    assert!(!frames.is_empty());
}

#[test]
fn test_debug_formatter_basic() {
    common::tracing::setup();
    
    let mut formatter = DebugFormatter::new();
    
    let mut trace = StackTrace::new(10);
    trace.trace_id = 42;
    
    let frame = CallFrame::new("test_function".to_string(), 0);
    trace.add_frame(frame);
    
    let result = formatter.format_stack_trace(&trace);
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert!(output.contains("test_function"));
    assert!(output.contains("42"));
}
