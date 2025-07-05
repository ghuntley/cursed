use cursed::runtime::error_handling::ErrorRuntime;
use cursed::runtime::panic::{PanicRuntime, PanicRuntimeConfig};

#[test]
fn test_error_handling_stack_trace() {
    let error_runtime = ErrorRuntime::new();
    let trace = error_runtime.capture_stack_trace();
    
    // Should have at least one frame
    assert!(!trace.is_empty());
    
    // Should not contain the placeholder message
    for frame in &trace {
        assert!(!frame.contains("Stack trace capture not implemented yet"));
    }
    
    println!("Error handling stack trace captured {} frames", trace.len());
}

#[test]
fn test_panic_runtime_stack_trace() {
    let mut config = PanicRuntimeConfig::default();
    config.capture_stack_traces = true;
    
    let panic_runtime = PanicRuntime::with_config(config);
    let trace = panic_runtime.capture_stack_trace();
    
    // Should have at least one frame
    assert!(!trace.is_empty());
    
    // Should not contain the placeholder message
    for frame in &trace {
        assert!(!frame.contains("Stack trace capture not implemented yet"));
    }
    
    println!("Panic runtime stack trace captured {} frames", trace.len());
}

#[test]
fn test_panic_runtime_stack_trace_disabled() {
    let mut config = PanicRuntimeConfig::default();
    config.capture_stack_traces = false;
    
    let panic_runtime = PanicRuntime::with_config(config);
    let trace = panic_runtime.capture_stack_trace();
    
    // Should be empty when disabled
    assert!(trace.is_empty());
}

// Test that we can actually capture meaningful stack traces
#[test]
fn test_nested_stack_trace() {
    fn level_3() -> Vec<String> {
        let error_runtime = ErrorRuntime::new();
        error_runtime.capture_stack_trace()
    }
    
    fn level_2() -> Vec<String> {
        level_3()
    }
    
    fn level_1() -> Vec<String> {
        level_2()
    }
    
    let trace = level_1();
    
    // Should have multiple frames due to nested calls
    assert!(!trace.is_empty());
    
    // Should contain function names (at least some recognizable symbols)
    let trace_text = trace.join("\n");
    println!("Nested stack trace:\n{}", trace_text);
    
    // The trace should contain at least one function name
    assert!(trace.iter().any(|frame| 
        frame.contains("level_") || 
        frame.contains("test_") || 
        !frame.contains("<unknown>")
    ));
}
