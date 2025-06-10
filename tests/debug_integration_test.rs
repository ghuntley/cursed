/// Integration tests for enhanced debug information system
///
/// Tests the complete debug information pipeline from LLVM code generation
/// through runtime error handling with rich stack traces and source context.

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::runtime::debug_info::*;
use cursed::runtime::debug_manager::*;
use cursed::runtime::panic::*;
use cursed::error::debug_context::*;
use cursed::codegen::llvm::debug_info::*;
use inkwell::context::Context;
use std::sync::Arc;
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs::File;
use std::io::Write;

/// Test complete debug information pipeline
#[test]
fn test_complete_debug_pipeline() {
    // Set up temporary source file
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("test_program.csd");
    
    let mut file = File::create(&source_file).unwrap();
    writeln!(file, "slay main() {{").unwrap();
    writeln!(file, "    sus x = 42").unwrap();
    writeln!(file, "    facts flag = true").unwrap();
    writeln!(file, "    lowkey (x > 0) {{").unwrap();
    writeln!(file, "        yeet \"Error occurred!\"").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();

    // Initialize debug manager
    let debug_manager = Arc::new(DebugManager::new());
    debug_manager.register_source_file(&source_file).unwrap();
    
    // Register function debug information
    let main_func = FunctionDebugInfo::new("main".to_string(), source_file.clone(), 1)
        .with_end_line(7)
        .with_local_variable(
            VariableInfo::new("x".to_string(), "sus".to_string())
                .with_value("42".to_string())
                .with_mutability(true)
        )
        .with_local_variable(
            VariableInfo::new("flag".to_string(), "facts".to_string())
                .with_value("true".to_string())
                .with_mutability(false)
        )
        .with_ip_range(0x1000, 0x2000);
    
    debug_manager.register_function(main_func).unwrap();
    
    // Initialize panic runtime with debug manager
    let panic_runtime = PanicRuntime::new();
    panic_runtime.initialize().unwrap();
    panic_runtime.set_debug_manager(Arc::clone(&debug_manager)).unwrap();
    
    // Simulate a panic with enhanced stack trace
    let panic_info = CursedPanicInfo::new(
        "Division by zero in main function".to_string(),
        PanicSeverity::Critical,
        PanicCategory::Arithmetic,
    )
    .with_location(SourceLocation::new(5, 15).with_file("test_program.csd"))
    .with_goroutine(123);

    // The panic runtime should capture enhanced stack trace
    let enhanced_trace = panic_runtime.capture_enhanced_stack_trace(Some(123));
    assert!(enhanced_trace.is_some());
    
    let trace = enhanced_trace.unwrap();
    assert_eq!(trace.goroutine_id, Some(123));
    assert!(!trace.frames.is_empty());
    
    // Create debug context with all information
    let debug_context = DebugContextBuilder::new(
        CursedError::panic_error("Division by zero in main function".to_string())
    )
    .stack_trace(trace)
    .annotation("variable_state", "x=42, flag=true")
    .annotation("operation", "division")
    .goroutine(123)
    .debug_manager(Arc::clone(&debug_manager))
    .build();

    // Test comprehensive error report
    let mut context = debug_context;
    let report = context.generate_error_report();
    
    assert!(report.contains("Error:"));
    assert!(report.contains("Stack trace"));
    assert!(report.contains("Goroutine: #123"));
    assert!(report.contains("variable_state: x=42, flag=true"));
    
    panic_runtime.shutdown().unwrap();
}

/// Test LLVM debug information generation
#[test]
fn test_llvm_debug_generation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let source_file = PathBuf::from("test.csd");
    
    // Test debug manager creation
    let debug_manager = LlvmDebugManager::new(&context, &module, &source_file, true);
    assert!(debug_manager.is_ok());
    
    let mut manager = debug_manager.unwrap();
    
    // Test function debug setup
    let function_type = context.i32_type().fn_type(&[], false);
    let function = module.add_function("test_function", function_type, None);
    
    let result = manager.setup_function_debug(function, "test_function", &source_file, 10);
    assert!(result.is_ok());
    
    // Test debug location creation
    let location = manager.create_expression_debug(42, 15, Some(&source_file));
    assert!(location.is_ok());
    
    let debug_location = location.unwrap();
    assert_eq!(debug_location.get_line(), 42);
    assert_eq!(debug_location.get_column(), 15);
    
    // Test variable debug information
    let i32_type = context.i32_type();
    let storage = i32_type.const_int(0, false).as_global_value().as_pointer_value();
    
    let var_result = manager.add_variable_debug(
        "test_var",
        "sus",
        storage,
        42,
        &source_file,
    );
    assert!(var_result.is_ok());
    
    manager.finalize();
}

/// Test debug information extraction and source snippets
#[test]
fn test_source_snippet_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("complex_program.csd");
    
    let mut file = File::create(&source_file).unwrap();
    writeln!(file, "// Complex CURSED program").unwrap();           // Line 1
    writeln!(file, "").unwrap();                                    // Line 2
    writeln!(file, "slay fibonacci(sus n) -> sus {{").unwrap();     // Line 3
    writeln!(file, "    lowkey (n <= 1) {{").unwrap();             // Line 4
    writeln!(file, "        periodt n").unwrap();                   // Line 5
    writeln!(file, "    }}").unwrap();                              // Line 6
    writeln!(file, "    periodt fibonacci(n-1) + fibonacci(n-2)").unwrap(); // Line 7
    writeln!(file, "}}").unwrap();                                  // Line 8
    writeln!(file, "").unwrap();                                    // Line 9
    writeln!(file, "slay main() {{").unwrap();                      // Line 10
    writeln!(file, "    sus result = fibonacci(10)").unwrap();      // Line 11
    writeln!(file, "    println(\"Result: {{}}\", result)").unwrap(); // Line 12
    writeln!(file, "}}").unwrap();                                  // Line 13

    let debug_manager = DebugManager::new();
    debug_manager.register_source_file(&source_file).unwrap();
    
    // Test snippet extraction with context
    let snippet = debug_manager.get_source_snippet(&source_file, 7, 2).unwrap();
    
    assert!(snippet.contains("Line 5"));
    assert!(snippet.contains("Line 7"));
    assert!(snippet.contains("Line 9"));
    assert!(snippet.contains("> 7"));  // Should mark the target line
    assert!(snippet.contains("fibonacci(n-1) + fibonacci(n-2)"));
    
    // Test snippet for different line
    let snippet2 = debug_manager.get_source_snippet(&source_file, 11, 1).unwrap();
    assert!(snippet2.contains("sus result = fibonacci(10)"));
    assert!(snippet2.contains("> 11"));
}

/// Test enhanced error context with real-world scenario
#[test]
fn test_enhanced_error_context_scenario() {
    // Simulate a complex error scenario with multiple stack frames
    let temp_dir = TempDir::new().unwrap();
    let main_file = temp_dir.path().join("main.csd");
    let lib_file = temp_dir.path().join("lib.csd");
    
    // Create main file
    let mut file = File::create(&main_file).unwrap();
    writeln!(file, "import \"lib\"").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "slay main() {{").unwrap();
    writeln!(file, "    sus data = [1, 2, 3, 4, 5]").unwrap();
    writeln!(file, "    sus result = process_data(data)").unwrap();
    writeln!(file, "    println(\"Result: {{}}\", result)").unwrap();
    writeln!(file, "}}").unwrap();
    
    // Create library file
    let mut file = File::create(&lib_file).unwrap();
    writeln!(file, "slay process_data(sus[] data) -> sus {{").unwrap();
    writeln!(file, "    sus sum = 0").unwrap();
    writeln!(file, "    periodt calculate_sum(data)").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "slay calculate_sum(sus[] data) -> sus {{").unwrap();
    writeln!(file, "    sus total = 0").unwrap();
    writeln!(file, "    fr fr(sus i = 0; i < len(data); i++) {{").unwrap();
    writeln!(file, "        // Bug: index out of bounds access").unwrap();
    writeln!(file, "        total += data[i + 1]").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "    periodt total").unwrap();
    writeln!(file, "}}").unwrap();

    let debug_manager = Arc::new(DebugManager::new());
    debug_manager.register_source_file(&main_file).unwrap();
    debug_manager.register_source_file(&lib_file).unwrap();
    
    // Register function debug information
    let main_func = FunctionDebugInfo::new("main".to_string(), main_file, 3)
        .with_end_line(7)
        .with_local_variable(
            VariableInfo::new("data".to_string(), "sus[]".to_string())
                .with_value("[1, 2, 3, 4, 5]".to_string())
        )
        .with_ip_range(0x1000, 0x1500);
    
    let process_data_func = FunctionDebugInfo::new("process_data".to_string(), lib_file.clone(), 1)
        .with_end_line(4)
        .with_parameter(
            VariableInfo::new("data".to_string(), "sus[]".to_string())
        )
        .with_ip_range(0x2000, 0x2500);
    
    let calculate_sum_func = FunctionDebugInfo::new("calculate_sum".to_string(), lib_file.clone(), 6)
        .with_end_line(13)
        .with_local_variable(
            VariableInfo::new("total".to_string(), "sus".to_string())
                .with_value("10".to_string()) // Value at time of error
        )
        .with_local_variable(
            VariableInfo::new("i".to_string(), "sus".to_string())
                .with_value("4".to_string()) // Loop index when error occurred
        )
        .with_ip_range(0x3000, 0x3500);
    
    debug_manager.register_function(main_func).unwrap();
    debug_manager.register_function(process_data_func).unwrap();
    debug_manager.register_function(calculate_sum_func).unwrap();
    
    // Create stack trace simulating the call chain
    let main_debug = DebugInfo::new(&main_file, 5, 20, "main".to_string())
        .with_instruction_pointer(0x1300);
    let process_debug = DebugInfo::new(&lib_file, 3, 15, "process_data".to_string())
        .with_instruction_pointer(0x2200);
    let calculate_debug = DebugInfo::new(&lib_file, 10, 25, "calculate_sum".to_string())
        .with_instruction_pointer(0x3200);
    
    let main_frame = EnhancedStackFrame::new(main_debug, 2)
        .with_variable(
            VariableInfo::new("data".to_string(), "sus[]".to_string())
                .with_value("[1, 2, 3, 4, 5]".to_string())
        );
    
    let process_frame = EnhancedStackFrame::new(process_debug, 1)
        .with_variable(
            VariableInfo::new("data".to_string(), "sus[]".to_string())
                .with_value("[1, 2, 3, 4, 5]".to_string())
        );
    
    let calculate_frame = EnhancedStackFrame::new(calculate_debug, 0)
        .with_variable(
            VariableInfo::new("total".to_string(), "sus".to_string())
                .with_value("10".to_string())
        )
        .with_variable(
            VariableInfo::new("i".to_string(), "sus".to_string())
                .with_value("4".to_string())
        );
    
    let enhanced_trace = EnhancedStackTrace::new()
        .with_frames(vec![calculate_frame, process_frame, main_frame])
        .with_goroutine(456);
    
    // Create bounds check error
    let bounds_error = CursedError::Runtime(
        "Index out of bounds: attempted to access index 5 of array with length 5".to_string()
    );
    
    let mut debug_context = DebugContextBuilder::new(bounds_error)
        .stack_trace(enhanced_trace)
        .annotation("array_length", "5")
        .annotation("attempted_index", "5")
        .annotation("loop_variable", "i=4")
        .annotation("bug_location", "data[i + 1] should be data[i]")
        .goroutine(456)
        .debug_manager(debug_manager)
        .build();

    // Test comprehensive error report
    let report = debug_context.generate_error_report();
    
    // Verify the report contains all expected information
    assert!(report.contains("Index out of bounds"));
    assert!(report.contains("Stack trace"));
    assert!(report.contains("calculate_sum"));
    assert!(report.contains("process_data"));
    assert!(report.contains("main"));
    assert!(report.contains("array_length: 5"));
    assert!(report.contains("attempted_index: 5"));
    assert!(report.contains("loop_variable: i=4"));
    assert!(report.contains("Goroutine: #456"));
    
    // Test user frames filtering
    let user_frames = debug_context.user_frames();
    assert_eq!(user_frames.len(), 3); // All frames should be user frames in this scenario
    
    // Test helpful message generation
    let helpful_message = debug_context.create_helpful_message();
    assert!(helpful_message.contains("Index out of bounds"));
    assert!(helpful_message.contains("Suggestion")); // Should have suggestions for runtime errors
}

/// Test performance with complex debug information
#[test]
fn test_performance_with_complex_debug_info() {
    use std::time::Instant;
    
    let start = Instant::now();
    
    let debug_manager = Arc::new(DebugManager::new());
    
    // Create many source files and functions
    for i in 0..100 {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&format!("module_{}.csd", i));
        
        let mut file = File::create(&file_path).unwrap();
        for j in 0..10 {
            writeln!(file, "slay function_{}_{j}() {{ /* code */ }}", i).unwrap();
        }
        
        debug_manager.register_source_file(&file_path).unwrap();
        
        // Register multiple functions per file
        for j in 0..10 {
            let func_info = FunctionDebugInfo::new(
                format!("function_{}_{}", i, j),
                file_path.clone(),
                j + 1,
            )
            .with_end_line(j + 2)
            .with_ip_range(
                0x10000 + (i * 1000) + (j * 100),
                0x10000 + (i * 1000) + (j * 100) + 99
            );
            
            debug_manager.register_function(func_info).unwrap();
        }
    }
    
    let setup_time = start.elapsed();
    
    // Test lookups performance
    let lookup_start = Instant::now();
    
    for i in 0..100 {
        let function_name = format!("function_{}_{}", i % 100, i % 10);
        let _func = debug_manager.get_function(&function_name).unwrap();
        
        let ip = 0x10000 + ((i % 100) * 1000) + ((i % 10) * 100) + 50;
        let _func_by_ip = debug_manager.get_function_by_ip(ip).unwrap();
    }
    
    let lookup_time = lookup_start.elapsed();
    
    // Test stack trace creation performance
    let trace_start = Instant::now();
    
    let mut frames = Vec::new();
    for i in 0..50 {
        let debug_info = DebugInfo::new(
            &format!("module_{}.csd", i % 10),
            (i % 100) + 1,
            (i % 50) + 1,
            format!("function_{}_{}", i % 10, i % 5),
        );
        
        let mut frame = EnhancedStackFrame::new(debug_info, i);
        
        // Add some variables
        for j in 0..5 {
            let var = VariableInfo::new(
                format!("var_{}", j),
                "sus".to_string(),
            ).with_value(format!("{}", j * i));
            
            frame = frame.with_variable(var);
        }
        
        frames.push(frame);
    }
    
    let trace = EnhancedStackTrace::new().with_frames(frames);
    let _user_frames = trace.user_frames();
    
    let trace_time = trace_start.elapsed();
    
    // Performance should be reasonable
    assert!(setup_time.as_millis() < 1000, "Setup took too long: {}ms", setup_time.as_millis());
    assert!(lookup_time.as_millis() < 100, "Lookups took too long: {}ms", lookup_time.as_millis());
    assert!(trace_time.as_millis() < 50, "Trace creation took too long: {}ms", trace_time.as_millis());
    
    let total_time = start.elapsed();
    println!("Total performance test time: {}ms", total_time.as_millis());
}

/// Test integration with question mark operator
#[test]
fn test_question_mark_integration() {
    let debug_manager = Arc::new(DebugManager::new());
    
    // Simulate a chain of errors using the question mark operator
    let original_error = CursedError::Type("Type mismatch in assignment".to_string());
    
    // First level: function that propagates the error
    let debug_info1 = DebugInfo::new("helper.csd", 15, 10, "validate_type".to_string());
    let frame1 = EnhancedStackFrame::new(debug_info1, 0);
    
    // Second level: function that calls the first
    let debug_info2 = DebugInfo::new("main.csd", 25, 5, "process_input".to_string());
    let frame2 = EnhancedStackFrame::new(debug_info2, 1);
    
    // Third level: main function
    let debug_info3 = DebugInfo::new("main.csd", 5, 1, "main".to_string());
    let frame3 = EnhancedStackFrame::new(debug_info3, 2);
    
    let stack_trace = EnhancedStackTrace::new()
        .with_frames(vec![frame1, frame2, frame3]);
    
    // Create error propagation chain
    let propagated_error = CursedError::ErrorPropagation {
        message: "Error propagated through question mark operator".to_string(),
        line: Some(25),
        column: Some(5),
        original_error: Some(Box::new(original_error)),
    };
    
    let debug_context = DebugContextBuilder::new(propagated_error)
        .stack_trace(stack_trace)
        .annotation("propagation_method", "question_mark_operator")
        .annotation("error_chain_depth", "3")
        .debug_manager(debug_manager)
        .build();

    // Test that error chain is properly represented
    let report = debug_context.generate_error_report();
    assert!(report.contains("Error propagated through question mark"));
    assert!(report.contains("validate_type"));
    assert!(report.contains("process_input"));
    assert!(report.contains("main"));
    assert!(report.contains("propagation_method: question_mark_operator"));
}

/// Test memory safety and cleanup
#[test]
fn test_memory_safety() {
    let debug_manager = Arc::new(DebugManager::new());
    
    // Create many temporary objects
    for i in 0..1000 {
        let debug_info = DebugInfo::new(
            &format!("file_{}.csd", i),
            i as u32,
            (i % 80) as u32,
            format!("function_{}", i),
        );
        
        let frame = EnhancedStackFrame::new(debug_info, i);
        let trace = EnhancedStackTrace::new().with_frames(vec![frame]);
        
        let error = CursedError::Runtime(format!("Error {}", i));
        let _context = DebugContext::new(error).with_stack_trace(trace);
        
        // Objects should be properly cleaned up when they go out of scope
    }
    
    // Clear caches to test cleanup
    debug_manager.clear_caches().unwrap();
    
    let stats = debug_manager.get_statistics().unwrap();
    assert_eq!(stats.cache_hits, 0);
    assert_eq!(stats.cache_misses, 0);
}

/// Test thread safety
#[test]
fn test_thread_safety() {
    use std::thread;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let debug_manager = Arc::new(DebugManager::new());
    let error_count = Arc::new(AtomicUsize::new(0));
    let success_count = Arc::new(AtomicUsize::new(0));
    
    let mut handles = Vec::new();
    
    // Spawn multiple threads that create debug contexts concurrently
    for thread_id in 0..8 {
        let manager_clone = Arc::clone(&debug_manager);
        let error_count_clone = Arc::clone(&error_count);
        let success_count_clone = Arc::clone(&success_count);
        
        let handle = thread::spawn(move || {
            for i in 0..100 {
                let debug_info = DebugInfo::new(
                    &format!("thread_{}_file_{}.csd", thread_id, i),
                    (i + 1) as u32,
                    10,
                    format!("thread_{}_function_{}", thread_id, i),
                );
                
                let frame = EnhancedStackFrame::new(debug_info, 0);
                let trace = EnhancedStackTrace::new().with_frames(vec![frame]);
                
                let error = CursedError::Runtime(format!("Thread {} error {}", thread_id, i));
                let context = DebugContextBuilder::new(error)
                    .stack_trace(trace)
                    .annotation("thread_id", &thread_id.to_string())
                    .annotation("iteration", &i.to_string())
                    .debug_manager(Arc::clone(&manager_clone))
                    .build();

                // Test report generation in concurrent environment
                match std::panic::catch_unwind(|| {
                    let _report = context.generate_error_report();
                }) {
                    Ok(_) => {
                        success_count_clone.fetch_add(1, Ordering::SeqCst);
                    }
                    Err(_) => {
                        error_count_clone.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_success = success_count.load(Ordering::SeqCst);
    let total_errors = error_count.load(Ordering::SeqCst);
    
    // All operations should succeed
    assert_eq!(total_success, 800); // 8 threads * 100 iterations
    assert_eq!(total_errors, 0);
}
