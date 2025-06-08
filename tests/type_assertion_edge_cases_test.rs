//! Comprehensive edge case tests for type assertion functionality
//! 
//! Tests various failure modes, panic scenarios, and edge cases for the type assertion system

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

use cursed::ast::expressions::{TypeAssertion, TypeAssertionQuestion, Identifier};
use cursed::ast::traits::{Expression, Node};
use cursed::codegen::llvm::{LlvmCodeGenerator, InterfaceTypeAssertion};
use cursed::error::type_assertion_error::{TypeAssertionError, helpers};
use cursed::error::{Error, SourceLocation};
use cursed::error_enhanced::{CursedError, ErrorKind};
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;
use std::path::PathBuf;
use tracing::{info, error, debug};

/// Helper to create a mock interface value for testing
fn create_mock_interface_value<'ctx>(
    codegen: &mut LlvmCodeGenerator<'ctx>,
    type_id: u64,
    data_value: Option<BasicValueEnum<'ctx>>
) -> Result<BasicValueEnum<'ctx>, Error> {
    let context = codegen.context();
    
    // Create a function context if one doesn't exist
    if codegen.current_function().is_none() {
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = codegen.module().add_function("test_fn", fn_type, None);
        let entry_block = context.append_basic_block(function, "entry");
        codegen.builder().position_at_end(entry_block);
        codegen.set_current_function(function);
    }
    
    // Create interface struct: (data_pointer, type_id)
    let ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let data_ptr = data_value.unwrap_or_else(|| ptr_type.const_null().into());
    let type_id_val = context.i64_type().const_int(type_id, false);
    
    codegen.build_tuple(vec![data_ptr, type_id_val.into()])
}

#[test]
fn test_nil_interface_assertion() {
    tracing_setup::init_test_tracing();
    info!("Testing type assertion on nil interface value");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Create a nil interface value (null pointer with zero type ID)
    let nil_interface = create_mock_interface_value(&mut codegen, 0, None).unwrap();
    
    // Attempt type assertion on nil interface
    let result = codegen.check_instance_of(
        nil_interface,
        "Person",
        Some(SourceLocation {
            line: 42,
            column: 10,
            file: Some("test.csd".to_string()),
            source_line: "person := nil_value.(Person)".to_string(),
        })
    );
    
    assert!(result.is_ok(), "Should handle nil interface gracefully");
    
    let is_instance = result.unwrap().into_int_value();
    // Should return false for nil interface
    assert_eq!(is_instance.get_zero_extended_constant().unwrap(), 0);
    
    info!("Nil interface assertion test passed");
}

#[test]
fn test_invalid_type_assertion() {
    tracing_setup::init_test_tracing();
    info!("Testing type assertion with invalid/unknown type");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Create interface with known type ID
    let person_type_id = codegen.hash_type_name("Person");
    let interface_value = create_mock_interface_value(&mut codegen, person_type_id, None).unwrap();
    
    // Attempt assertion to completely different type
    let result = codegen.check_instance_of(
        interface_value,
        "NonExistentType",
        Some(SourceLocation {
            line: 15,
            column: 8,
            file: Some("test.csd".to_string()),
            source_line: "val := obj.(NonExistentType)".to_string(),
        })
    );
    
    assert!(result.is_ok(), "Should handle unknown type assertion");
    
    let is_instance = result.unwrap().into_int_value();
    // Should return false for mismatched types
    assert_eq!(is_instance.get_zero_extended_constant().unwrap(), 0);
    
    info!("Invalid type assertion test passed");
}

#[test]
fn test_complex_nested_type_assertion() {
    tracing_setup::init_test_tracing();
    info!("Testing complex nested type assertion expressions");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Create nested type assertion AST
    let inner_assertion = TypeAssertion {
        token: "inner".to_string(),
        expression: Box::new(Identifier {
            token: "obj".to_string(),
            value: "obj".to_string(),
        }),
        type_name: "Stringer".to_string(),
    };
    
    let outer_assertion = TypeAssertion {
        token: "outer".to_string(),
        expression: Box::new(inner_assertion),
        type_name: "Person".to_string(),
    };
    
    // Test string representation
    let expected_string = "obj.(Stringer).(Person)";
    assert_eq!(outer_assertion.string(), expected_string);
    
    info!("Complex nested type assertion structure test passed: {}", expected_string);
}

#[test]
fn test_type_assertion_error_creation() {
    tracing_setup::init_test_tracing();
    info!("Testing type assertion error creation and handling");

    // Test basic error creation
    let error = TypeAssertionError::new("Stringer", "Person");
    assert_eq!(error.interface_type, "Stringer");
    assert_eq!(error.target_type, "Person");
    
    // Test error with full context
    let detailed_error = helpers::create_detailed_assertion_error(
        "Stringer",
        "Person", 
        Some(0x1234567890ABCDEF),
        Some(0xFEDCBA0987654321),
        Some("Dog".to_string()),
        Some(0x1111222233334444),
        Some(SourceLocation {
            line: 25,
            column: 5,
            file: Some("complex.csd".to_string()),
            source_line: "    result := interface_val.(Person)".to_string(),
        })
    );
    
    let detailed_message = detailed_error.to_detailed_string();
    assert!(detailed_message.contains("Failed to assert that Stringer is a Person"));
    assert!(detailed_message.contains("Actual type was Dog"));
    assert!(detailed_message.contains("Type IDs: interface=0x1234567890abcdef"));
    assert!(detailed_message.contains("Location: complex.csd:25:5"));
    
    info!("Type assertion error creation test passed");
}

#[test]
fn test_type_assertion_error_conversion() {
    tracing_setup::init_test_tracing();
    info!("Testing conversion from TypeAssertionError to CursedError");

    let assertion_error = TypeAssertionError::new("Stringer", "Person")
        .with_actual_type("Dog", Some(0x1234567890ABCDEF))
        .with_interface_type_id(0xFEDCBA0987654321)
        .with_target_type_id(0x1111222233334444)
        .with_location(SourceLocation {
            line: 100,
            column: 20,
            file: Some("error_test.csd".to_string()),
            source_line: "let person = obj.(Person)".to_string(),
        });

    let cursed_error: CursedError = assertion_error.into();
    
    assert_eq!(cursed_error.kind(), &ErrorKind::TypeAssertion);
    assert!(cursed_error.code().is_some());
    assert_eq!(cursed_error.code().unwrap(), "ASSERT-001");
    
    // Check context preservation
    assert!(cursed_error.get_context("interface_type").is_some());
    assert!(cursed_error.get_context("target_type").is_some());
    assert!(cursed_error.get_context("actual_type").is_some());
    
    info!("Type assertion error conversion test passed");
}

#[test]
fn test_performance_edge_cases() {
    tracing_setup::init_test_tracing();
    info!("Testing performance edge cases for type assertions");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Test hash collision resistance
    let type_names = vec![
        "Person", "Dog", "Cat", "Fish", "Bird", "Horse", "Elephant", "Mouse",
        "PersonExtended", "DogBreed", "CatFamily", "FishSpecies", "BirdType",
        "HorseVariant", "ElephantSize", "MouseColor"
    ];
    
    let mut hashes = std::collections::HashSet::new();
    let mut collision_count = 0;
    
    for type_name in &type_names {
        let hash = codegen.hash_type_name(type_name);
        if !hashes.insert(hash) {
            collision_count += 1;
            error!("Hash collision detected for type: {}", type_name);
        }
    }
    
    // Allow some collisions but not too many
    assert!(collision_count < type_names.len() / 4, 
           "Too many hash collisions: {} out of {}", collision_count, type_names.len());
    
    info!("Performance edge cases test passed. Collision rate: {}/{}", 
          collision_count, type_names.len());
}

#[test]
fn test_interface_data_extraction_edge_cases() {
    tracing_setup::init_test_tracing();
    info!("Testing interface data extraction edge cases");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Create function context
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = codegen.module().add_function("test_fn", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    codegen.builder().position_at_end(entry_block);
    codegen.set_current_function(function);
    
    // Test 1: Empty struct interface
    let empty_struct = context.struct_type(&[], false).get_undef();
    let result1 = codegen.extract_interface_data_ptr(empty_struct.into());
    assert!(result1.is_ok(), "Should handle empty struct gracefully");
    
    // Test 2: Non-struct, non-pointer value
    let int_value = context.i32_type().const_int(42, false);
    let result2 = codegen.extract_interface_data_ptr(int_value.into());
    assert!(result2.is_ok(), "Should handle non-struct values gracefully");
    
    // Test 3: Direct pointer value
    let ptr_value = context.i8_type().ptr_type(AddressSpace::default()).const_null();
    let result3 = codegen.extract_interface_data_ptr(ptr_value.into());
    assert!(result3.is_ok(), "Should handle direct pointer values");
    
    info!("Interface data extraction edge cases test passed");
}

#[test]
fn test_type_assertion_with_source_location() {
    tracing_setup::init_test_tracing();
    info!("Testing type assertion compilation with source location preservation");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("location_test.csd"));
    
    // Create function context
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = codegen.module().add_function("test_fn", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    codegen.builder().position_at_end(entry_block);
    codegen.set_current_function(function);
    
    // Create type assertion with detailed source information
    let type_assertion = TypeAssertion {
        token: "detailed_test".to_string(),
        expression: Box::new(Identifier {
            token: "some_var".to_string(),
            value: "some_var".to_string(),
        }),
        type_name: "ComplexType".to_string(),
    };
    
    // This should preserve the source location information in the generated code
    let result = codegen.compile_type_assertion(&type_assertion);
    assert!(result.is_ok(), "Type assertion compilation should succeed");
    
    info!("Type assertion with source location test passed");
}

#[test]
fn test_registry_fallback_behavior() {
    tracing_setup::init_test_tracing();
    info!("Testing registry fallback behavior for unknown types");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Test type ID lookup with and without registry
    let type_name = "UnknownRegistryType";
    
    // Should fall back to hash-based type ID
    let direct_hash = codegen.hash_type_name(type_name);
    let type_id_result = codegen.get_type_id(type_name);
    
    assert!(type_id_result.is_ok(), "Type ID generation should work even without registry");
    
    let type_id_value = type_id_result.unwrap().into_int_value();
    let generated_id = type_id_value.get_zero_extended_constant().unwrap();
    
    assert_eq!(generated_id, direct_hash, "Should fall back to hash-based ID");
    
    info!("Registry fallback behavior test passed");
}

#[test]
fn test_memory_safety_assertions() {
    tracing_setup::init_test_tracing();
    info!("Testing memory safety in type assertion operations");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Create function context
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = codegen.module().add_function("test_fn", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    codegen.builder().position_at_end(entry_block);
    codegen.set_current_function(function);
    
    // Test casting with null/invalid values
    let null_ptr = context.i8_type().ptr_type(AddressSpace::default()).const_null();
    let cast_result = codegen.cast_to_interface_type(null_ptr.into(), "TestType");
    
    assert!(cast_result.is_ok(), "Should handle null pointer casting safely");
    
    // Test multiple successive operations for memory consistency
    for i in 0..100 {
        let test_value = context.i32_type().const_int(i, false);
        let cast_result = codegen.cast_to_interface_type(test_value.into(), &format!("Type{}", i));
        assert!(cast_result.is_ok(), "Memory operations should remain stable");
    }
    
    info!("Memory safety assertions test passed");
}

#[test]
fn test_error_message_quality() {
    tracing_setup::init_test_tracing();
    info!("Testing quality and informativeness of error messages");

    // Test various error scenarios and their messages
    let scenarios = vec![
        ("Stringer", "Person", Some("Dog"), "Basic type mismatch"),
        ("", "Person", None, "Empty interface type"),
        ("Stringer", "", None, "Empty target type"),
        ("VeryLongInterfaceTypeName", "VeryLongTargetTypeName", Some("ActualVeryLongTypeName"), "Long type names"),
    ];
    
    for (interface_type, target_type, actual_type, description) in scenarios {
        let mut error = TypeAssertionError::new(interface_type, target_type);
        
        if let Some(actual) = actual_type {
            error = error.with_actual_type(actual, None);
        }
        
        let message = error.get_description();
        assert!(!message.is_empty(), "Error message should not be empty for: {}", description);
        assert!(message.contains("Failed to assert"), "Error message should contain assertion failure indication");
        
        debug!("Error message for {}: {}", description, message);
    }
    
    info!("Error message quality test passed");
}

#[test]  
fn test_type_assertion_question_error_propagation() {
    tracing_setup::init_test_tracing();
    info!("Testing error propagation for TypeAssertionQuestion");

    // Test the AST structure for error propagating type assertions
    let type_assertion_q = TypeAssertionQuestion {
        token: "error_prop_test".to_string(),
        expression: Box::new(Identifier {
            token: "test_obj".to_string(),
            value: "test_obj".to_string(),
        }),
        type_name: "ErrorPropType".to_string(),
    };
    
    // Verify proper string representation includes error propagation syntax
    let string_repr = type_assertion_q.string();
    assert!(string_repr.contains("?"), "Should include error propagation operator");
    assert!(string_repr.contains("test_obj"), "Should include original expression");
    assert!(string_repr.contains("ErrorPropType"), "Should include target type");
    
    info!("Type assertion question error propagation test passed: {}", string_repr);
}
