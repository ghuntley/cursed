//! Tests for LLVM error propagation functionality
//! 
//! This test suite validates the error propagation implementations in the LLVM code generator,
//! ensuring that the `?` operator works correctly with Result and Option types.

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::error_handling::ErrorHandlingCompiler;
use cursed::codegen::llvm::expression_compiler::{LlvmValue, LlvmType};
use cursed::error::{Error, SourceLocation};
use cursed::runtime::panic::{PanicSeverity, PanicCategory};

#[path = "common.rs"]
mod common;

/// Test the compile_error_propagation function for Result types
#[test]
fn test_compile_error_propagation_result() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Create a Result<i32, String> value
    let result_value = LlvmValue {
        value_type: LlvmType::Function {
            return_type: Box::new(LlvmType::String),
            param_types: vec![LlvmType::Int32],
        },
        llvm_name: "%test_result".to_string(),
        is_constant: false,
    };
    
    let result_type = LlvmType::Function {
        return_type: Box::new(LlvmType::String),
        param_types: vec![LlvmType::Int32],
    };
    
    let location = Some(SourceLocation::new(42, 10));
    let function_name = Some("test_function".to_string());
    
    // Test compilation
    let compiled_result = generator.compile_error_propagation(
        result_value,
        result_type,
        location,
        function_name,
    );
    
    assert!(compiled_result.is_ok(), "Error propagation compilation should succeed");
    
    let propagated_value = compiled_result.unwrap();
    assert!(!propagated_value.llvm_name.is_empty(), "Generated value should have a name");
    assert!(!propagated_value.is_constant, "Propagated value should not be constant");
}

/// Test the compile_error_propagation function for Option types
#[test]
fn test_compile_error_propagation_option() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Create an Option<i32> value
    let option_value = LlvmValue {
        value_type: LlvmType::Function {
            return_type: Box::new(LlvmType::Boolean),
            param_types: vec![LlvmType::Int32],
        },
        llvm_name: "%test_option".to_string(),
        is_constant: false,
    };
    
    let option_type = LlvmType::Function {
        return_type: Box::new(LlvmType::Boolean),
        param_types: vec![LlvmType::Int32],
    };
    
    let location = Some(SourceLocation::new(24, 15));
    let function_name = Some("option_test_function".to_string());
    
    // Test compilation
    let compiled_result = generator.compile_error_propagation(
        option_value,
        option_type,
        location,
        function_name,
    );
    
    assert!(compiled_result.is_ok(), "Option error propagation compilation should succeed");
    
    let propagated_value = compiled_result.unwrap();
    assert!(!propagated_value.llvm_name.is_empty(), "Generated value should have a name");
    assert!(!propagated_value.is_constant, "Propagated value should not be constant");
}

/// Test the compile_error_propagation function for generic types
#[test]
fn test_compile_error_propagation_generic() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Create a generic pointer value
    let generic_value = LlvmValue {
        value_type: LlvmType::Pointer(Box::new(LlvmType::Int32)),
        llvm_name: "%test_generic".to_string(),
        is_constant: false,
    };
    
    let generic_type = LlvmType::Pointer(Box::new(LlvmType::Int32));
    let location = Some(SourceLocation::new(100, 20));
    
    // Test compilation
    let compiled_result = generator.compile_error_propagation(
        generic_value,
        generic_type,
        location,
        None,
    );
    
    assert!(compiled_result.is_ok(), "Generic error propagation compilation should succeed");
    
    let propagated_value = compiled_result.unwrap();
    assert!(!propagated_value.llvm_name.is_empty(), "Generated value should have a name");
}

/// Test the generate_error_check function for Result types
#[test]
fn test_generate_error_check_result() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Create a Result value for checking
    let result_value = LlvmValue {
        value_type: LlvmType::Function {
            return_type: Box::new(LlvmType::String),
            param_types: vec![LlvmType::Int32],
        },
        llvm_name: "%check_result".to_string(),
        is_constant: false,
    };
    
    let result_type = LlvmType::Function {
        return_type: Box::new(LlvmType::String),
        param_types: vec![LlvmType::Int32],
    };
    
    // Test error check generation
    let check_result = generator.generate_error_check(result_value, result_type);
    
    assert!(check_result.is_ok(), "Error check generation should succeed");
    
    let check_value = check_result.unwrap();
    assert_eq!(check_value.value_type, LlvmType::Boolean, "Error check should return boolean");
    assert!(!check_value.llvm_name.is_empty(), "Check value should have a name");
    assert!(!check_value.is_constant, "Check result should not be constant");
}

/// Test the generate_error_check function for Option types
#[test]
fn test_generate_error_check_option() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Create an Option value for checking
    let option_value = LlvmValue {
        value_type: LlvmType::Function {
            return_type: Box::new(LlvmType::Boolean),
            param_types: vec![LlvmType::Int32],
        },
        llvm_name: "%check_option".to_string(),
        is_constant: false,
    };
    
    let option_type = LlvmType::Function {
        return_type: Box::new(LlvmType::Boolean),
        param_types: vec![LlvmType::Int32],
    };
    
    // Test error check generation
    let check_result = generator.generate_error_check(option_value, option_type);
    
    assert!(check_result.is_ok(), "Option error check generation should succeed");
    
    let check_value = check_result.unwrap();
    assert_eq!(check_value.value_type, LlvmType::Boolean, "Error check should return boolean");
    assert!(!check_value.llvm_name.is_empty(), "Check value should have a name");
}

/// Test the generate_error_check function for various primitive types
#[test]
fn test_generate_error_check_primitives() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Test different primitive types
    let test_cases = vec![
        (LlvmType::Boolean, "%bool_val"),
        (LlvmType::Int32, "%int32_val"),
        (LlvmType::Int64, "%int64_val"),
        (LlvmType::Float64, "%float_val"),
        (LlvmType::String, "%string_val"),
        (LlvmType::Pointer(Box::new(LlvmType::Int32)), "%ptr_val"),
        (LlvmType::Void, "%void_val"),
    ];
    
    for (value_type, llvm_name) in test_cases {
        let test_value = LlvmValue {
            value_type: value_type.clone(),
            llvm_name: llvm_name.to_string(),
            is_constant: false,
        };
        
        let check_result = generator.generate_error_check(test_value, value_type.clone());
        
        assert!(check_result.is_ok(), "Error check for {:?} should succeed", value_type);
        
        let check_value = check_result.unwrap();
        assert_eq!(check_value.value_type, LlvmType::Boolean, "All error checks should return boolean");
        assert!(!check_value.llvm_name.is_empty(), "Check value should have a name");
    }
}

/// Test the generate_stack_trace_capture function
#[test]
fn test_generate_stack_trace_capture() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Test with default max depth
    let stack_trace_result = generator.generate_stack_trace_capture(None);
    
    assert!(stack_trace_result.is_ok(), "Stack trace capture should succeed");
    
    let stack_trace_value = stack_trace_result.unwrap();
    
    // Check that we get a pointer type
    match stack_trace_value.value_type {
        LlvmType::Pointer(_) => {}, // Expected
        _ => panic!("Stack trace should return a pointer type"),
    }
    
    assert!(!stack_trace_value.llvm_name.is_empty(), "Stack trace value should have a name");
    assert!(!stack_trace_value.is_constant, "Stack trace should not be constant");
}

/// Test the generate_stack_trace_capture function with custom max depth
#[test]
fn test_generate_stack_trace_capture_custom_depth() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Test with custom max depth
    let custom_depth = Some(64);
    let stack_trace_result = generator.generate_stack_trace_capture(custom_depth);
    
    assert!(stack_trace_result.is_ok(), "Stack trace capture with custom depth should succeed");
    
    let stack_trace_value = stack_trace_result.unwrap();
    
    // Check that we get a pointer type
    match stack_trace_value.value_type {
        LlvmType::Pointer(_) => {}, // Expected
        _ => panic!("Stack trace should return a pointer type"),
    }
    
    assert!(!stack_trace_value.llvm_name.is_empty(), "Stack trace value should have a name");
}

/// Test the generate_stack_trace_capture function with debug enabled
#[test]
fn test_generate_stack_trace_capture_with_debug() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new_with_debug(
        cursed::debug::DebugConfig::full()
    ).expect("Failed to create LLVM generator with debug");
    
    // Test stack trace capture with debug enabled
    let stack_trace_result = generator.generate_stack_trace_capture(Some(16));
    
    assert!(stack_trace_result.is_ok(), "Stack trace capture with debug should succeed");
    
    let stack_trace_value = stack_trace_result.unwrap();
    
    // Check that we get a pointer type
    match stack_trace_value.value_type {
        LlvmType::Pointer(_) => {}, // Expected
        _ => panic!("Stack trace should return a pointer type"),
    }
    
    assert!(generator.debug_enabled(), "Debug should be enabled");
    assert!(!stack_trace_value.llvm_name.is_empty(), "Stack trace value should have a name");
}

/// Test error propagation without location information
#[test]
fn test_error_propagation_no_location() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    let test_value = LlvmValue {
        value_type: LlvmType::Int32,
        llvm_name: "%test_val".to_string(),
        is_constant: false,
    };
    
    // Test without location information
    let compiled_result = generator.compile_error_propagation(
        test_value,
        LlvmType::Int32,
        None, // No location
        None, // No function name
    );
    
    assert!(compiled_result.is_ok(), "Error propagation without location should succeed");
}

/// Test helper function for checking Result/Option types
#[test]
fn test_type_checking_helpers() {
    common::tracing::setup();
    
    let generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Test Result type checking
    let result_type = Box::new(LlvmType::String);
    assert!(generator.is_result_type_llvm(&result_type), "Should identify Result type");
    
    // Test Option type checking
    let option_type = Box::new(LlvmType::Boolean);
    assert!(generator.is_option_type_llvm(&option_type), "Should identify Option type");
    
    // Test non-Result/Option type
    let int_type = Box::new(LlvmType::Int32);
    assert!(!generator.is_result_type_llvm(&int_type), "Should not identify int as Result");
    assert!(!generator.is_option_type_llvm(&int_type), "Should not identify int as Option");
}

/// Test error context generation
#[test]
fn test_generate_error_context() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    let error_message = "Test error message";
    let location = Some(SourceLocation::new(42, 15));
    let function_name = Some("test_error_function".to_string());
    
    // Test error context generation
    let context_result = generator.generate_error_context(
        error_message,
        location,
        function_name,
    );
    
    assert!(context_result.is_ok(), "Error context generation should succeed");
    
    let context_value = context_result.unwrap();
    
    // Check that we get a pointer type (error context is a pointer)
    match context_value.value_type {
        LlvmType::Pointer(_) => {}, // Expected
        _ => panic!("Error context should return a pointer type"),
    }
    
    assert!(!context_value.llvm_name.is_empty(), "Error context should have a name");
}

/// Test panic statement compilation
#[test]
fn test_compile_panic_statement() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    let message = "Test panic message";
    let severity = PanicSeverity::Critical;
    let category = PanicCategory::Memory;
    let location = Some(SourceLocation::new(50, 25));
    
    // Test panic statement compilation
    let panic_result = generator.compile_panic_statement(
        message,
        severity,
        category,
        location,
    );
    
    assert!(panic_result.is_ok(), "Panic statement compilation should succeed");
}

/// Test recovery block compilation
#[test]
fn test_compile_recovery_block() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Define a simple protected operation
    let protected_operation = |gen: &mut LlvmCodeGenerator| -> Result<LlvmValue, Error> {
        Ok(LlvmValue {
            value_type: LlvmType::Int32,
            llvm_name: "%protected_result".to_string(),
            is_constant: false,
        })
    };
    
    let location = Some(SourceLocation::new(75, 30));
    
    // Test recovery block compilation
    let recovery_result = generator.compile_recovery_block(
        protected_operation,
        None, // No recovery handler
        location,
    );
    
    assert!(recovery_result.is_ok(), "Recovery block compilation should succeed");
    
    let result_value = recovery_result.unwrap();
    assert_eq!(result_value.value_type, LlvmType::Int32, "Should return the protected operation result");
}

/// Integration test: Complete error propagation workflow
#[test]
fn test_error_propagation_workflow() {
    common::tracing::setup();
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Step 1: Create a Result value
    let result_value = LlvmValue {
        value_type: LlvmType::Function {
            return_type: Box::new(LlvmType::String),
            param_types: vec![LlvmType::Int32],
        },
        llvm_name: "%workflow_result".to_string(),
        is_constant: false,
    };
    
    // Step 2: Generate error check
    let check_result = generator.generate_error_check(
        result_value.clone(),
        result_value.value_type.clone(),
    ).expect("Error check should succeed");
    
    assert_eq!(check_result.value_type, LlvmType::Boolean, "Error check should return boolean");
    
    // Step 3: Compile error propagation
    let propagated_result = generator.compile_error_propagation(
        result_value,
        LlvmType::Function {
            return_type: Box::new(LlvmType::String),
            param_types: vec![LlvmType::Int32],
        },
        Some(SourceLocation::new(100, 50)),
        Some("workflow_function".to_string()),
    ).expect("Error propagation should succeed");
    
    assert!(!propagated_result.llvm_name.is_empty(), "Propagated result should have a name");
    
    // Step 4: Capture stack trace
    let stack_trace_result = generator.generate_stack_trace_capture(Some(32))
        .expect("Stack trace capture should succeed");
    
    match stack_trace_result.value_type {
        LlvmType::Pointer(_) => {}, // Expected
        _ => panic!("Stack trace should return a pointer type"),
    }
}
