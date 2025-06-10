/// Comprehensive tests for enhanced debug information system
///
/// Tests debug information capture, stack trace generation, source location mapping,
/// symbol resolution, and integration with panic system and error handling.

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::runtime::debug_info::*;
use cursed::runtime::debug_manager::*;
use cursed::runtime::panic::*;
use cursed::error::debug_context::*;
use std::sync::Arc;
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs::File;
use std::io::Write;

/// Test enhanced debug information creation and manipulation
#[test]
fn test_debug_info_creation() {
    let debug_info = DebugInfo::new("test.csd", 42, 10, "test_function".to_string())
        .with_module("test_module".to_string())
        .with_variable("x".to_string(), "sus".to_string())
        .with_instruction_pointer(0x1000);

    assert_eq!(debug_info.line, 42);
    assert_eq!(debug_info.column, 10);
    assert_eq!(debug_info.function_name, "test_function");
    assert_eq!(debug_info.module_name, Some("test_module".to_string()));
    assert!(debug_info.variables.contains_key("x"));
    assert_eq!(debug_info.instruction_pointer, Some(0x1000));
}

/// Test variable information handling
#[test]
fn test_variable_info() {
    let var = VariableInfo::new("test_var".to_string(), "String".to_string())
        .with_value("\"hello world\"".to_string())
        .with_location("stack+8".to_string())
        .with_mutability(true)
        .with_scope_depth(2);

    assert_eq!(var.name, "test_var");
    assert_eq!(var.type_name, "String");
    assert_eq!(var.value, Some("\"hello world\"".to_string()));
    assert_eq!(var.location, Some("stack+8".to_string()));
    assert!(var.is_mutable);
    assert_eq!(var.scope_depth, 2);
}

/// Test enhanced stack frame creation
#[test]
fn test_enhanced_stack_frame() {
    let debug_info = DebugInfo::new("main.csd", 100, 5, "main".to_string());
    let call_site = DebugInfo::new("lib.csd", 50, 10, "helper".to_string());
    let var = VariableInfo::new("count".to_string(), "sus".to_string())
        .with_value("42".to_string());

    let frame = EnhancedStackFrame::new(debug_info, 0)
        .with_call_site(call_site)
        .with_variable(var)
        .with_inlined(false)
        .with_optimization_level("O2".to_string());

    assert_eq!(frame.frame_index, 0);
    assert!(!frame.is_inlined);
    assert!(frame.call_site.is_some());
    assert!(frame.local_variables.contains_key("count"));
    assert_eq!(frame.optimization_level, Some("O2".to_string()));
}

/// Test enhanced stack trace creation and manipulation
#[test]
fn test_enhanced_stack_trace() {
    let debug_info1 = DebugInfo::new("main.csd", 100, 5, "main".to_string());
    let debug_info2 = DebugInfo::new("lib.csd", 50, 10, "helper".to_string());
    
    let frame1 = EnhancedStackFrame::new(debug_info1, 0);
    let frame2 = EnhancedStackFrame::new(debug_info2, 1);
    
    let trace = EnhancedStackTrace::new()
        .with_frames(vec![frame1, frame2])
        .with_goroutine(42)
        .with_truncation(10);

    assert_eq!(trace.frames.len(), 2);
    assert_eq!(trace.goroutine_id, Some(42));
    assert_eq!(trace.total_depth, 10);
    assert!(trace.is_truncated);
    
    let top_frame = trace.top_frame();
    assert!(top_frame.is_some());
    assert_eq!(top_frame.unwrap().debug_info.function_name, "main");
    
    let user_frames = trace.user_frames();
    assert_eq!(user_frames.len(), 2); // Both are user frames in this test
}

/// Test stack trace capture configuration
#[test]
fn test_stack_trace_config() {
    let config = StackTraceConfig {
        max_frames: 25,
        capture_variables: false,
        capture_call_sites: true,
        capture_rust_backtrace: false,
        resolve_symbols: true,
        max_variable_depth: 5,
        include_inlined_frames: false,
        exclude_patterns: vec!["test_pattern".to_string()],
    };

    assert_eq!(config.max_frames, 25);
    assert!(!config.capture_variables);
    assert!(config.capture_call_sites);
    assert!(!config.capture_rust_backtrace);
    assert!(config.resolve_symbols);
    assert_eq!(config.max_variable_depth, 5);
    assert!(!config.include_inlined_frames);
    assert!(config.exclude_patterns.contains(&"test_pattern".to_string()));
}

/// Test stack trace capture functionality
#[test]
fn test_stack_trace_capture() {
    let capture = StackTraceCapture::new();
    
    // Test basic capture
    let trace = capture.capture();
    assert!(trace.is_ok());
    
    let trace = trace.unwrap();
    assert!(!trace.frames.is_empty());
    assert_eq!(trace.thread_id, std::thread::current().id());
    
    // Test capture with goroutine context
    let trace_with_goroutine = capture.capture_with_context(Some(123));
    assert!(trace_with_goroutine.is_ok());
    
    let trace = trace_with_goroutine.unwrap();
    assert_eq!(trace.goroutine_id, Some(123));
}

/// Test mock symbol resolver
#[test]
fn test_mock_symbol_resolver() {
    let mut resolver = MockSymbolResolver::new();
    
    let symbol = SymbolInfo {
        name: "test_function".to_string(),
        file: Some(PathBuf::from("test.csd")),
        line: Some(42),
        column: Some(10),
        offset: Some(0x100),
    };
    
    resolver.add_symbol(0x1000, symbol);
    
    // Test single symbol resolution
    let resolved = resolver.resolve_symbol(0x1000);
    assert!(resolved.is_some());
    let resolved = resolved.unwrap();
    assert_eq!(resolved.name, "test_function");
    assert_eq!(resolved.line, Some(42));
    
    // Test non-existent symbol
    let not_found = resolver.resolve_symbol(0x2000);
    assert!(not_found.is_none());
    
    // Test batch resolution
    let symbols = resolver.resolve_symbols(&[0x1000, 0x2000, 0x1000]);
    assert_eq!(symbols.len(), 3);
    assert!(symbols[0].is_some());
    assert!(symbols[1].is_none());
    assert!(symbols[2].is_some());
}

/// Test source file handling in debug manager
#[test]
fn test_source_file_handling() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.csd");
    
    // Create test file
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "facts hello = \"world\"").unwrap();
    writeln!(file, "sus count = 42").unwrap();
    writeln!(file, "// Comment line").unwrap();
    writeln!(file, "yolo println(\"Hello\")").unwrap();

    // Test source file creation and loading
    let mut source_file = SourceFile::new(&file_path);
    assert!(!source_file.is_cached);
    
    source_file.load_content().unwrap();
    assert!(source_file.is_cached);
    assert!(source_file.content.is_some());
    
    // Test line access
    assert_eq!(source_file.get_line(1), Some("facts hello = \"world\"".to_string()));
    assert_eq!(source_file.get_line(2), Some("sus count = 42".to_string()));
    assert_eq!(source_file.get_line(5), None);
    
    // Test context lines
    let context = source_file.get_lines_with_context(2, 1);
    assert!(context.is_some());
    let context = context.unwrap();
    assert_eq!(context.len(), 3); // Lines 1, 2, 3
    assert_eq!(context[1].0, 2); // Middle line should be line 2
    assert_eq!(context[1].1, "sus count = 42");
}

/// Test function debug information
#[test]
fn test_function_debug_info() {
    let file_path = PathBuf::from("test.csd");
    let param = VariableInfo::new("param1".to_string(), "sus".to_string())
        .with_value("42".to_string());
    let local_var = VariableInfo::new("local1".to_string(), "tea".to_string())
        .with_value("\"hello\"".to_string());
    
    let func_info = FunctionDebugInfo::new("test_function".to_string(), file_path, 10)
        .with_end_line(20)
        .with_parameter(param)
        .with_local_variable(local_var)
        .with_ip_range(0x1000, 0x2000)
        .with_module("test_module".to_string());

    assert_eq!(func_info.name, "test_function");
    assert_eq!(func_info.start_line, 10);
    assert_eq!(func_info.end_line, Some(20));
    assert_eq!(func_info.parameters.len(), 1);
    assert_eq!(func_info.local_variables.len(), 1);
    assert_eq!(func_info.module_name, Some("test_module".to_string()));
    
    // Test IP range checking
    assert!(func_info.contains_ip(0x1500));
    assert!(!func_info.contains_ip(0x3000));
    assert!(!func_info.contains_ip(0x500));
}

/// Test debug manager functionality
#[test]
fn test_debug_manager() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.csd");
    
    // Create test file
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "slay main() {{").unwrap();
    writeln!(file, "    facts x = 42").unwrap();
    writeln!(file, "    periodt").unwrap();
    writeln!(file, "}}").unwrap();

    let manager = DebugManager::new();
    
    // Test source file registration
    assert!(manager.register_source_file(&file_path).is_ok());
    
    let retrieved = manager.get_source_file(&file_path).unwrap();
    assert!(retrieved.is_some());
    let source_file = retrieved.unwrap();
    assert!(source_file.is_cached);
    
    // Test function registration
    let func_info = FunctionDebugInfo::new("main".to_string(), file_path.clone(), 1)
        .with_end_line(4)
        .with_ip_range(0x1000, 0x2000);
    
    assert!(manager.register_function(func_info).is_ok());
    
    // Test function retrieval by name
    let retrieved_func = manager.get_function("main").unwrap();
    assert!(retrieved_func.is_some());
    assert_eq!(retrieved_func.unwrap().name, "main");
    
    // Test function retrieval by IP
    let func_by_ip = manager.get_function_by_ip(0x1500).unwrap();
    assert!(func_by_ip.is_some());
    assert_eq!(func_by_ip.unwrap().name, "main");
    
    // Test source snippet extraction
    let snippet = manager.get_source_snippet(&file_path, 2, 1).unwrap();
    assert!(snippet.contains("facts x = 42"));
    assert!(snippet.contains("> 2")); // Should mark line 2
}

/// Test debug manager with symbol resolver
#[test]
fn test_debug_manager_with_symbol_resolver() {
    let manager = DebugManager::new();
    let mut resolver = MockSymbolResolver::new();
    
    let symbol = SymbolInfo {
        name: "test_function".to_string(),
        file: Some(PathBuf::from("test.csd")),
        line: Some(42),
        column: Some(10),
        offset: Some(0x100),
    };
    
    resolver.add_symbol(0x1000, symbol);
    assert!(manager.set_symbol_resolver(resolver).is_ok());
    
    // Test symbol resolution
    let resolved = manager.resolve_symbol(0x1000).unwrap();
    assert!(resolved.is_some());
    let symbol_info = resolved.unwrap();
    assert_eq!(symbol_info.name, "test_function");
    assert_eq!(symbol_info.line, Some(42));
}

/// Test debug context creation and error reporting
#[test]
fn test_debug_context() {
    let error = CursedError::Runtime("Test runtime error".to_string());
    let debug_info = DebugInfo::new("test.csd", 42, 10, "test_function".to_string());
    let frame = EnhancedStackFrame::new(debug_info, 0);
    let stack_trace = EnhancedStackTrace::new().with_frames(vec![frame]);
    
    let mut context = DebugContext::new(error)
        .with_stack_trace(stack_trace)
        .with_annotation("context".to_string(), "unit test".to_string())
        .with_goroutine(123);

    assert_eq!(context.goroutine_id, Some(123));
    assert!(context.annotations.contains_key("context"));
    assert!(context.stack_trace.is_some());
    
    // Test error report generation
    let report = context.generate_error_report();
    assert!(report.contains("Error:"));
    assert!(report.contains("Stack trace"));
    assert!(report.contains("Additional information"));
    assert!(report.contains("unit test"));
    assert!(report.contains("Goroutine: #123"));
    
    // Test severity detection
    assert_eq!(context.severity(), ErrorSeverity::Error);
    
    // Test helpful message generation
    let helpful = context.create_helpful_message();
    assert!(helpful.contains("Test runtime error"));
}

/// Test debug context builder pattern
#[test]
fn test_debug_context_builder() {
    let error = CursedError::Type("Type mismatch error".to_string());
    let stack_trace = EnhancedStackTrace::new();
    
    let context = DebugContextBuilder::new(error)
        .stack_trace(stack_trace)
        .annotation("hint", "Check variable types")
        .annotation("location", "main function")
        .goroutine(456)
        .build();

    assert_eq!(context.goroutine_id, Some(456));
    assert_eq!(context.annotations.get("hint"), Some(&"Check variable types".to_string()));
    assert_eq!(context.annotations.get("location"), Some(&"main function".to_string()));
    assert!(context.stack_trace.is_some());
}

/// Test error severity classification
#[test]
fn test_error_severity() {
    let runtime_error = CursedError::Runtime("test".to_string());
    let runtime_context = DebugContext::new(runtime_error);
    assert_eq!(runtime_context.severity(), ErrorSeverity::Error);
    
    let parse_error = CursedError::ParseError {
        message: "syntax error".to_string(),
        source_location: None,
    };
    let parse_context = DebugContext::new(parse_error);
    assert_eq!(parse_context.severity(), ErrorSeverity::Error);
    
    let panic_error = CursedError::panic_error("test panic".to_string());
    let panic_context = DebugContext::new(panic_error);
    assert_eq!(panic_context.severity(), ErrorSeverity::Critical);
    
    let recoverable_panic = CursedError::recoverable_panic("test recoverable".to_string());
    let recoverable_context = DebugContext::new(recoverable_panic);
    assert_eq!(recoverable_context.severity(), ErrorSeverity::Critical);
}

/// Test integration with panic system
#[test]
fn test_panic_integration() {
    let runtime = PanicRuntime::new();
    runtime.initialize().unwrap();
    
    // Test setting debug manager
    let debug_manager = Arc::new(DebugManager::new());
    assert!(runtime.set_debug_manager(Arc::clone(&debug_manager)).is_ok());
    
    // Test enhanced stack trace capture
    let enhanced_trace = runtime.capture_enhanced_stack_trace(Some(789));
    assert!(enhanced_trace.is_some());
    
    let trace = enhanced_trace.unwrap();
    assert_eq!(trace.goroutine_id, Some(789));
    assert!(!trace.frames.is_empty());
    
    runtime.shutdown().unwrap();
}

/// Test enhanced panic info with stack traces
#[test]
fn test_enhanced_panic_info() {
    let location = SourceLocation::new(42, 10).with_file("test.csd");
    let debug_info = DebugInfo::new("test.csd", 42, 10, "test_function".to_string());
    let frame = EnhancedStackFrame::new(debug_info, 0);
    let enhanced_trace = EnhancedStackTrace::new().with_frames(vec![frame]);
    
    let panic_info = CursedPanicInfo::new(
        "Test panic with enhanced debug info".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User,
    )
    .with_location(location)
    .with_goroutine(999)
    .with_enhanced_stack_trace(enhanced_trace)
    .with_metadata("test_key".to_string(), "test_value".to_string());

    assert_eq!(panic_info.message, "Test panic with enhanced debug info");
    assert_eq!(panic_info.severity, PanicSeverity::Critical);
    assert_eq!(panic_info.category, PanicCategory::User);
    assert_eq!(panic_info.goroutine_id, Some(999));
    assert!(panic_info.enhanced_stack_trace.is_some());
    assert!(panic_info.metadata.contains_key("test_key"));
    
    // Test display formatting includes enhanced trace
    let formatted = format!("{}", panic_info);
    assert!(formatted.contains("Enhanced stack trace"));
}

/// Test user frame filtering
#[test]
fn test_user_frame_filtering() {
    let user_frame = EnhancedStackFrame::new(
        DebugInfo::new("src/main.csd", 10, 5, "main".to_string()),
        0
    );
    let runtime_frame = EnhancedStackFrame::new(
        DebugInfo::new("runtime/panic.rs", 100, 10, "panic_handler".to_string()),
        1
    );
    let std_frame = EnhancedStackFrame::new(
        DebugInfo::new("std/backtrace.rs", 200, 15, "capture".to_string()),
        2
    );
    
    let trace = EnhancedStackTrace::new().with_frames(vec![user_frame, runtime_frame, std_frame]);
    let context = DebugContext::new(CursedError::Runtime("test".to_string()))
        .with_stack_trace(trace);
    
    let user_frames = context.user_frames();
    assert_eq!(user_frames.len(), 1);
    assert_eq!(user_frames[0].debug_info.function_name, "main");
    assert!(user_frames[0].debug_info.file_path.to_string_lossy().contains("main.csd"));
}

/// Test debug manager statistics
#[test]
fn test_debug_manager_statistics() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.csd");
    
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "test content").unwrap();
    
    let manager = DebugManager::new();
    
    // Initial stats
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.files_tracked, 0);
    assert_eq!(stats.functions_tracked, 0);
    assert_eq!(stats.cache_hits, 0);
    assert_eq!(stats.cache_misses, 0);
    
    // Register file and function
    manager.register_source_file(&file_path).unwrap();
    
    let func_info = FunctionDebugInfo::new("test".to_string(), file_path.clone(), 1);
    manager.register_function(func_info).unwrap();
    
    // Check updated stats
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.files_tracked, 1);
    assert_eq!(stats.functions_tracked, 1);
    
    // Access file to generate cache hit
    let _retrieved = manager.get_source_file(&file_path).unwrap();
    let stats = manager.get_statistics().unwrap();
    assert!(stats.cache_hits > 0);
    
    // Access non-existent file to generate cache miss
    let non_existent = temp_dir.path().join("non_existent.csd");
    let _not_found = manager.get_source_file(&non_existent).unwrap();
    let stats = manager.get_statistics().unwrap();
    assert!(stats.cache_misses > 0);
}

/// Test performance with large stack traces
#[test]
fn test_large_stack_trace_performance() {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Create a large stack trace
    let mut frames = Vec::new();
    for i in 0..1000 {
        let debug_info = DebugInfo::new(
            &format!("file{}.csd", i % 10),
            (i % 100) + 1,
            (i % 20) + 1,
            format!("function_{}", i),
        );
        
        let frame = EnhancedStackFrame::new(debug_info, i);
        frames.push(frame);
    }
    
    let trace = EnhancedStackTrace::new().with_frames(frames);
    
    // Test filtering operations
    let user_frames = trace.user_frames();
    let top_frame = trace.top_frame();
    
    let elapsed = start.elapsed();
    
    // Should complete quickly even with large traces
    assert!(elapsed.as_millis() < 100);
    assert!(!user_frames.is_empty());
    assert!(top_frame.is_some());
    assert_eq!(trace.frames.len(), 1000);
}

/// Test memory usage and cleanup
#[test]
fn test_memory_cleanup() {
    let manager = DebugManager::new();
    
    // Register multiple files and functions
    for i in 0..50 {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&format!("test{}.csd", i));
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content {}", i).unwrap();
        
        manager.register_source_file(&file_path).unwrap();
        
        let func_info = FunctionDebugInfo::new(
            format!("function_{}", i),
            file_path,
            1,
        );
        manager.register_function(func_info).unwrap();
    }
    
    let stats_before = manager.get_statistics().unwrap();
    assert_eq!(stats_before.files_tracked, 50);
    assert_eq!(stats_before.functions_tracked, 50);
    
    // Clear caches
    assert!(manager.clear_caches().is_ok());
    
    let stats_after = manager.get_statistics().unwrap();
    assert_eq!(stats_after.cache_hits, 0);
    assert_eq!(stats_after.cache_misses, 0);
    assert_eq!(stats_after.files_cached, 0);
}
