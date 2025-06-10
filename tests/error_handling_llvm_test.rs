//! LLVM Code Generation Tests for CURSED Error Handling
//!
//! This module tests the LLVM integration aspects of error handling:
//! - Compilation of error handling constructs to LLVM IR
//! - Code generation for `?` operator
//! - Panic statement compilation
//! - Stack trace capture integration
//! - FFI function declarations and usage

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::codegen::llvm::error_handling::{
    ErrorHandlingCompiler, ErrorHandlingFunctions, ErrorHandlingPatterns,
    ErrorHandlingIntegration, ErrorHandlingFunction
};
use cursed::runtime::panic::{PanicSeverity, PanicCategory};
use cursed::codegen::llvm::{LlvmCodeGenerator, LlvmValue, LlvmType};
use std::collections::HashMap;

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
fn test_error_handling_function_registry() {
    init_tracing!();
    
    let functions = ErrorHandlingFunctions::new();
    
    // Test that all required functions are registered
    assert!(functions.get_function("cursed_panic").is_some());
    assert!(functions.get_function("cursed_propagate_error").is_some());
    assert!(functions.get_function("cursed_stack_capture").is_some());
    assert!(functions.get_function("cursed_create_error_context").is_some());
    assert!(functions.get_function("cursed_is_in_error_handling").is_some());
    assert!(functions.get_function("cursed_clear_error_context").is_some());
    
    // Test function that doesn't exist
    assert!(functions.get_function("nonexistent_function").is_none());
}

#[test]
fn test_panic_function_descriptor() {
    init_tracing!();
    
    let functions = ErrorHandlingFunctions::new();
    let panic_func = functions.get_function("cursed_panic").unwrap();
    
    assert_eq!(panic_func.llvm_name, "cursed_panic");
    assert_eq!(panic_func.return_type, LlvmType::Void);
    assert!(!panic_func.can_error); // Panics instead of returning errors
    assert_eq!(panic_func.parameters.len(), 6); // message, severity, category, line, column, file
    
    // Check parameter types
    assert!(matches!(panic_func.parameters[0], LlvmType::String)); // message
    assert!(matches!(panic_func.parameters[1], LlvmType::Integer(8))); // severity
    assert!(matches!(panic_func.parameters[2], LlvmType::Integer(8))); // category
    assert!(matches!(panic_func.parameters[3], LlvmType::Integer(32))); // line
    assert!(matches!(panic_func.parameters[4], LlvmType::Integer(32))); // column
    assert!(matches!(panic_func.parameters[5], LlvmType::String)); // file
}

#[test]
fn test_error_propagation_function_descriptor() {
    init_tracing!();
    
    let functions = ErrorHandlingFunctions::new();
    let propagate_func = functions.get_function("cursed_propagate_error").unwrap();
    
    assert_eq!(propagate_func.llvm_name, "cursed_propagate_error");
    assert_eq!(propagate_func.return_type, LlvmType::Integer(8)); // 0 = success, 1 = error
    assert!(propagate_func.can_error);
    assert_eq!(propagate_func.parameters.len(), 6); // message, error_code, line, column, file, function
    
    // Check parameter types
    assert!(matches!(propagate_func.parameters[0], LlvmType::String)); // error_message
    assert!(matches!(propagate_func.parameters[1], LlvmType::Integer(32))); // error_code
    assert!(matches!(propagate_func.parameters[2], LlvmType::Integer(32))); // line
    assert!(matches!(propagate_func.parameters[3], LlvmType::Integer(32))); // column
    assert!(matches!(propagate_func.parameters[4], LlvmType::String)); // file
    assert!(matches!(propagate_func.parameters[5], LlvmType::String)); // function
}

#[test]
fn test_stack_capture_function_descriptor() {
    init_tracing!();
    
    let functions = ErrorHandlingFunctions::new();
    let capture_func = functions.get_function("cursed_stack_capture").unwrap();
    
    assert_eq!(capture_func.llvm_name, "cursed_stack_capture");
    assert_eq!(capture_func.return_type, LlvmType::Pointer); // Pointer to stack trace
    assert!(capture_func.can_error);
    assert_eq!(capture_func.parameters.len(), 1); // max_depth
    
    assert!(matches!(capture_func.parameters[0], LlvmType::Integer(32))); // max_depth
}

#[test]
fn test_function_declarations_generation() {
    init_tracing!();
    
    let functions = ErrorHandlingFunctions::new();
    let declarations = functions.generate_declarations();
    
    // Should contain header comment
    assert!(declarations.contains("CURSED Error Handling Runtime Functions"));
    
    // Should contain all function declarations
    assert!(declarations.contains("declare void @cursed_panic"));
    assert!(declarations.contains("declare i8 @cursed_propagate_error"));
    assert!(declarations.contains("declare i8* @cursed_stack_capture"));
    assert!(declarations.contains("declare i8* @cursed_create_error_context"));
    assert!(declarations.contains("declare i8 @cursed_is_in_error_handling"));
    assert!(declarations.contains("declare void @cursed_clear_error_context"));
    
    // Should contain parameter information
    assert!(declarations.contains("i8* %message"));
    assert!(declarations.contains("i64 %message_len"));
    assert!(declarations.contains("i8 %severity"));
    assert!(declarations.contains("i32 %line"));
    
    tracing::info!("Generated declarations:\n{}", declarations);
}

#[test]
fn test_panic_ir_generation() {
    init_tracing!();
    
    let mut temp_counter = 0;
    let location = Some(SourceLocation::new(10, 5).with_file("test_panic.csd"));
    
    let ir = ErrorHandlingPatterns::generate_panic_ir(
        "Test panic message",
        PanicSeverity::Critical,
        PanicCategory::User,
        location,
        &mut temp_counter,
    );
    
    // Should contain message allocation and storage
    assert!(ir.contains("alloca"));
    assert!(ir.contains("Test panic message"));
    
    // Should contain function call
    assert!(ir.contains("call void @cursed_panic"));
    
    // Should contain severity and category parameters
    assert!(ir.contains("i8 1")); // Critical severity
    assert!(ir.contains("i8 6")); // User category
    
    // Should contain location information
    assert!(ir.contains("i32 10")); // line
    assert!(ir.contains("i32 5"));  // column
    assert!(ir.contains("test_panic.csd"));
    
    // Should end with unreachable
    assert!(ir.contains("unreachable"));
    
    // Temp counter should have advanced
    assert!(temp_counter > 0);
    
    tracing::info!("Generated panic IR:\n{}", ir);
}

#[test]
fn test_panic_ir_with_different_severities() {
    init_tracing!();
    
    let severities = vec![
        (PanicSeverity::Recoverable, "i8 0"),
        (PanicSeverity::Critical, "i8 1"),
        (PanicSeverity::Fatal, "i8 2"),
    ];
    
    for (severity, expected_code) in severities {
        let mut temp_counter = 0;
        let ir = ErrorHandlingPatterns::generate_panic_ir(
            "Severity test",
            severity,
            PanicCategory::Generic,
            None,
            &mut temp_counter,
        );
        
        assert!(ir.contains(expected_code));
        tracing::debug!("Severity {:?} generates code: {}", severity, expected_code);
    }
}

#[test]
fn test_panic_ir_with_different_categories() {
    init_tracing!();
    
    let categories = vec![
        (PanicCategory::Memory, "i8 0"),
        (PanicCategory::TypeAssertion, "i8 1"),
        (PanicCategory::BoundsCheck, "i8 2"),
        (PanicCategory::Arithmetic, "i8 3"),
        (PanicCategory::Channel, "i8 4"),
        (PanicCategory::Goroutine, "i8 5"),
        (PanicCategory::User, "i8 6"),
        (PanicCategory::System, "i8 7"),
        (PanicCategory::Generic, "i8 8"),
    ];
    
    for (category, expected_code) in categories {
        let mut temp_counter = 0;
        let ir = ErrorHandlingPatterns::generate_panic_ir(
            "Category test",
            PanicSeverity::Critical,
            category,
            None,
            &mut temp_counter,
        );
        
        assert!(ir.contains(expected_code));
        tracing::debug!("Category {:?} generates code: {}", category, expected_code);
    }
}

#[test]
fn test_error_propagation_ir_generation() {
    init_tracing!();
    
    let mut temp_counter = 0;
    let location = Some(SourceLocation::new(15, 10).with_file("propagation_test.csd"));
    let function_name = Some("test_function".to_string());
    
    let ir = ErrorHandlingPatterns::generate_error_propagation_ir(
        "Test error message",
        42, // error code
        location,
        function_name,
        &mut temp_counter,
    );
    
    // Should contain message handling
    assert!(ir.contains("Test error message"));
    assert!(ir.contains("alloca"));
    
    // Should contain function call
    assert!(ir.contains("call i8 @cursed_propagate_error"));
    
    // Should contain error code
    assert!(ir.contains("i32 42"));
    
    // Should contain location information
    assert!(ir.contains("i32 15")); // line
    assert!(ir.contains("i32 10")); // column
    assert!(ir.contains("propagation_test.csd"));
    
    // Should contain function name
    assert!(ir.contains("test_function"));
    
    // Should generate pointer variables
    assert!(ir.contains("getelementptr"));
    
    tracing::info!("Generated error propagation IR:\n{}", ir);
}

#[test]
fn test_error_propagation_ir_without_location() {
    init_tracing!();
    
    let mut temp_counter = 0;
    let ir = ErrorHandlingPatterns::generate_error_propagation_ir(
        "No location error",
        1,
        None, // No location
        None, // No function
        &mut temp_counter,
    );
    
    // Should handle missing location gracefully
    assert!(ir.contains("No location error"));
    assert!(ir.contains("call i8 @cursed_propagate_error"));
    assert!(ir.contains("unknown")); // Default file name
    assert!(ir.contains("i32 0")); // Default line/column
    
    tracing::info!("Generated IR without location:\n{}", ir);
}

#[test]
fn test_stack_trace_capture_ir() {
    init_tracing!();
    
    let mut temp_counter = 0;
    
    // Test with specific depth
    let ir_with_depth = ErrorHandlingPatterns::generate_stack_trace_capture_ir(
        Some(50),
        &mut temp_counter,
    );
    
    assert!(ir_with_depth.contains("call i8* @cursed_stack_capture"));
    assert!(ir_with_depth.contains("i32 50"));
    
    // Test with default depth
    let ir_default = ErrorHandlingPatterns::generate_stack_trace_capture_ir(
        None,
        &mut temp_counter,
    );
    
    assert!(ir_default.contains("call i8* @cursed_stack_capture"));
    assert!(ir_default.contains("i32 100")); // Default depth
    
    tracing::info!("Stack trace capture IR: {}", ir_with_depth);
}

#[test]
fn test_error_context_creation_ir() {
    init_tracing!();
    
    let mut temp_counter = 0;
    let location = Some(SourceLocation::new(20, 15).with_file("context_test.csd"));
    let function_name = Some("context_function".to_string());
    
    let ir = ErrorHandlingPatterns::generate_error_context_ir(
        "Context error message",
        location,
        function_name,
        &mut temp_counter,
    );
    
    // Should contain message handling
    assert!(ir.contains("Context error message"));
    assert!(ir.contains("alloca"));
    
    // Should contain function call
    assert!(ir.contains("call i8* @cursed_create_error_context"));
    
    // Should contain location information
    assert!(ir.contains("i32 20")); // line
    assert!(ir.contains("i32 15")); // column
    assert!(ir.contains("context_test.csd"));
    
    // Should contain function name
    assert!(ir.contains("context_function"));
    
    tracing::info!("Generated error context IR:\n{}", ir);
}

#[test]
fn test_error_check_ir_generation() {
    init_tracing!();
    
    let mut temp_counter = 0;
    
    let ir = ErrorHandlingPatterns::generate_error_check_ir(
        "%result_value",
        "error_handler",
        "success_path",
        &mut temp_counter,
    );
    
    // Should contain comparison
    assert!(ir.contains("icmp eq i8 %result_value, 0"));
    
    // Should contain conditional branch
    assert!(ir.contains("br i1"));
    assert!(ir.contains("error_handler"));
    assert!(ir.contains("success_path"));
    
    // Should generate check variable
    assert!(ir.contains("%error_check_"));
    
    tracing::info!("Generated error check IR: {}", ir);
}

#[test]
fn test_error_handling_integration() {
    init_tracing!();
    
    let mut integration = ErrorHandlingIntegration::new();
    
    // Test function declarations
    let declarations = integration.generate_function_declarations();
    assert!(declarations.contains("CURSED Error Handling Runtime Functions"));
    assert!(declarations.contains("@cursed_panic"));
    
    // Test panic generation
    let panic_ir = integration.generate_panic(
        "Integration panic test",
        PanicSeverity::Fatal,
        PanicCategory::System,
        Some(SourceLocation::new(25, 20).with_file("integration.csd")),
    );
    assert!(panic_ir.contains("Integration panic test"));
    assert!(panic_ir.contains("call void @cursed_panic"));
    
    // Test error propagation generation
    let propagation_ir = integration.generate_error_propagation(
        "Integration error test",
        100,
        Some(SourceLocation::new(30, 25).with_file("integration.csd")),
        Some("integration_function".to_string()),
    );
    assert!(propagation_ir.contains("Integration error test"));
    assert!(propagation_ir.contains("call i8 @cursed_propagate_error"));
    assert!(propagation_ir.contains("i32 100"));
    
    // Test stack trace capture
    let capture_ir = integration.generate_stack_trace_capture(Some(25));
    assert!(capture_ir.contains("call i8* @cursed_stack_capture"));
    assert!(capture_ir.contains("i32 25"));
    
    // Test error context generation
    let context_ir = integration.generate_error_context(
        "Integration context test",
        Some(SourceLocation::new(35, 30).with_file("integration.csd")),
        Some("integration_context_func".to_string()),
    );
    assert!(context_ir.contains("Integration context test"));
    assert!(context_ir.contains("call i8* @cursed_create_error_context"));
    
    // Test error checking
    let check_ir = integration.generate_error_check(
        "%integration_result",
        "integration_error",
        "integration_success",
    );
    assert!(check_ir.contains("icmp eq i8 %integration_result, 0"));
    assert!(check_ir.contains("integration_error"));
    assert!(check_ir.contains("integration_success"));
    
    // Verify temp counter is advancing
    assert!(integration.temp_counter > 0);
    
    tracing::info!("Integration test completed with {} temp variables", integration.temp_counter);
}

#[test]
fn test_complex_error_propagation_scenario() {
    init_tracing!();
    
    let mut integration = ErrorHandlingIntegration::new();
    
    // Simulate a complex function with multiple error checks
    let mut full_ir = Vec::new();
    
    // Function prologue
    full_ir.push("define i8 @complex_function() {".to_string());
    full_ir.push("entry:".to_string());
    
    // First operation that might error
    full_ir.push("  ; First operation".to_string());
    let propagation1 = integration.generate_error_propagation(
        "First operation failed",
        1,
        Some(SourceLocation::new(10, 5).with_file("complex.csd")),
        Some("complex_function".to_string()),
    );
    full_ir.push(format!("  {}", propagation1));
    
    // Error check for first operation
    let check1 = integration.generate_error_check(
        "%error_result_1",
        "error_exit",
        "continue_1",
    );
    full_ir.push(format!("  {}", check1));
    
    // Second operation
    full_ir.push("continue_1:".to_string());
    full_ir.push("  ; Second operation".to_string());
    let propagation2 = integration.generate_error_propagation(
        "Second operation failed",
        2,
        Some(SourceLocation::new(15, 10).with_file("complex.csd")),
        Some("complex_function".to_string()),
    );
    full_ir.push(format!("  {}", propagation2));
    
    // Error check for second operation
    let check2 = integration.generate_error_check(
        "%error_result_2",
        "error_exit",
        "success_exit",
    );
    full_ir.push(format!("  {}", check2));
    
    // Success path
    full_ir.push("success_exit:".to_string());
    full_ir.push("  ret i8 0".to_string());
    
    // Error path
    full_ir.push("error_exit:".to_string());
    full_ir.push("  ret i8 1".to_string());
    
    full_ir.push("}".to_string());
    
    let complete_ir = full_ir.join("\n");
    
    // Verify the complete IR contains all expected elements
    assert!(complete_ir.contains("@complex_function"));
    assert!(complete_ir.contains("First operation failed"));
    assert!(complete_ir.contains("Second operation failed"));
    assert!(complete_ir.contains("call i8 @cursed_propagate_error"));
    assert!(complete_ir.contains("icmp eq i8"));
    assert!(complete_ir.contains("br i1"));
    assert!(complete_ir.contains("error_exit"));
    assert!(complete_ir.contains("success_exit"));
    
    tracing::info!("Generated complex function IR:\n{}", complete_ir);
}

#[test]
fn test_panic_with_stack_trace_scenario() {
    init_tracing!();
    
    let mut integration = ErrorHandlingIntegration::new();
    
    // Generate a panic with stack trace capture
    let mut ir = Vec::new();
    
    ir.push("define void @panic_with_trace() {".to_string());
    ir.push("entry:".to_string());
    
    // Capture stack trace first
    let trace_ir = integration.generate_stack_trace_capture(Some(20));
    ir.push(format!("  {}", trace_ir));
    
    // Then trigger panic
    let panic_ir = integration.generate_panic(
        "Critical error with stack trace",
        PanicSeverity::Fatal,
        PanicCategory::System,
        Some(SourceLocation::new(50, 25).with_file("panic_trace.csd")),
    );
    ir.push(format!("  {}", panic_ir));
    
    ir.push("}".to_string());
    
    let complete_ir = ir.join("\n");
    
    assert!(complete_ir.contains("call i8* @cursed_stack_capture"));
    assert!(complete_ir.contains("i32 20")); // stack depth
    assert!(complete_ir.contains("call void @cursed_panic"));
    assert!(complete_ir.contains("Critical error with stack trace"));
    assert!(complete_ir.contains("unreachable"));
    
    tracing::info!("Generated panic with trace IR:\n{}", complete_ir);
}

#[test]
fn test_error_handling_with_debug_info() {
    init_tracing!();
    
    let mut integration = ErrorHandlingIntegration::new();
    
    // Test that location information is properly encoded
    let locations = vec![
        SourceLocation::new(1, 1).with_file("debug1.csd"),
        SourceLocation::new(100, 50).with_file("debug2.csd"),
        SourceLocation::new(999, 123).with_file("very_long_filename_debug.csd"),
    ];
    
    for location in locations {
        let ir = integration.generate_error_context(
            "Debug info test",
            Some(location.clone()),
            Some("debug_function".to_string()),
        );
        
        assert!(ir.contains(&format!("i32 {}", location.line)));
        assert!(ir.contains(&format!("i32 {}", location.column)));
        if let Some(file) = &location.file {
            assert!(ir.contains(file));
        }
        
        tracing::debug!("Generated debug IR for location {}: contains line and column", location);
    }
}

#[test]
fn test_error_handling_memory_layout() {
    init_tracing!();
    
    let functions = ErrorHandlingFunctions::new();
    
    // Test that function signatures match expected memory layout
    let propagate_func = functions.get_function("cursed_propagate_error").unwrap();
    
    // Error propagation should return i8 (1 byte) for efficient checking
    assert_eq!(propagate_func.return_type, LlvmType::Integer(8));
    
    // Stack capture should return pointer for efficient passing
    let capture_func = functions.get_function("cursed_stack_capture").unwrap();
    assert_eq!(capture_func.return_type, LlvmType::Pointer);
    
    // Context creation should return pointer for efficient passing
    let context_func = functions.get_function("cursed_create_error_context").unwrap();
    assert_eq!(context_func.return_type, LlvmType::Pointer);
    
    // State checking should return i8 for efficient boolean
    let state_func = functions.get_function("cursed_is_in_error_handling").unwrap();
    assert_eq!(state_func.return_type, LlvmType::Integer(8));
    
    tracing::info!("All function memory layouts verified");
}

/// Documentation: LLVM Error Handling Integration
/// 
/// This test suite validates the LLVM code generation aspects of CURSED's
/// error handling system. Key areas tested:
/// 
/// 1. **Function Registry**: All error handling functions are properly registered
///    with correct signatures and metadata
/// 
/// 2. **IR Generation**: Error handling constructs generate correct LLVM IR
///    including proper memory allocation, string handling, and function calls
/// 
/// 3. **Panic Compilation**: Panic statements compile to proper LLVM IR with
///    severity and category encoding
/// 
/// 4. **Error Propagation**: The `?` operator generates efficient error checking
///    and propagation code
/// 
/// 5. **Stack Traces**: Stack trace capture integrates properly with LLVM
/// 
/// 6. **Debug Information**: Location information is properly encoded in the IR
/// 
/// 7. **Memory Layout**: Function signatures use efficient data types
/// 
/// 8. **Integration**: All components work together to generate complete
///    error handling workflows
/// 
/// This comprehensive testing ensures that CURSED's error handling compiles
/// efficiently to LLVM IR and maintains compatibility with the runtime system.

#[cfg(test)]
mod llvm_error_test_utilities {
    use super::*;
    
    /// Helper to create test location with standardized format
    pub fn create_test_location(line: usize, column: usize, file: &str) -> SourceLocation {
        SourceLocation::new(line, column).with_file(file)
    }
    
    /// Helper to validate IR contains expected patterns
    pub fn assert_ir_contains_patterns(ir: &str, patterns: &[&str]) {
        for pattern in patterns {
            assert!(ir.contains(pattern), "IR missing pattern: {}\nIR:\n{}", pattern, ir);
        }
    }
    
    /// Helper to count temporary variables in generated IR
    pub fn count_temp_variables(ir: &str) -> usize {
        ir.matches('%').count()
    }
    
    /// Helper to validate function call format
    pub fn assert_valid_function_call(ir: &str, function_name: &str) {
        assert!(ir.contains(&format!("call")));
        assert!(ir.contains(&format!("@{}", function_name)));
    }
}
