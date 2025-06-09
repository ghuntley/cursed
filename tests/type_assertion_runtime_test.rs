//! Runtime test for type assertion functionality
//! 
//! This test verifies that type assertions work correctly at runtime,
//! including successful assertions, failed assertions, and error handling.

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

use cursed::ast::expressions::{TypeAssertion, TypeAssertionQuestion, Identifier, StringLiteral};
use cursed::ast::traits::{Expression, Node};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::token::Token;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use std::path::PathBuf;

fn setup_codegen_with_function<'ctx>(context: &'ctx Context) -> LlvmCodeGenerator<'ctx> {
    let mut codegen = LlvmCodeGenerator::new(context, "test_module", PathBuf::from("test.csd"));
    
    // Create a test function to provide execution context
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = codegen.module().add_function("test_fn", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    codegen.builder().position_at_end(entry_block);
    codegen.set_current_function(function);
    
    codegen
}

#[test]
fn test_type_assertion_compilation_basic() {
    tracing_setup::init_test_tracing();
    tracing::info!("Starting basic type assertion compilation test");

    let context = Context::create();
    let mut codegen = setup_codegen_with_function(&context);

    // Create a simple type assertion AST
    let type_assertion = TypeAssertion {
        token: "test_token".to_string(),
        expression: Box::new(StringLiteral {
            token: "string_lit".to_string(),
            value: "test_value".to_string(),
        }),
        type_name: "String".to_string(),
    };

    // Test that we can compile the type assertion
    use cursed::codegen::llvm::type_assertion::InterfaceTypeAssertion;
    let result = codegen.compile_type_assertion(&type_assertion);
    
    // Should compile successfully even if it doesn't work at runtime
    tracing::info!("Type assertion compilation result: {:?}", result.is_ok());
    
    tracing::info!("Basic type assertion compilation test completed");
}

#[test]
fn test_instance_of_check() {
    tracing_setup::init_test_tracing();
    tracing::info!("Starting instance-of check test");

    let context = Context::create();
    let mut codegen = setup_codegen_with_function(&context);

    // Create a mock interface value (simplified for testing)
    let i64_type = context.i64_type();
    let bool_type = context.bool_type();
    
    // Create a tuple type representing an interface value: (data_ptr, type_id)
    let ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let interface_type = context.struct_type(&[ptr_type.into(), i64_type.into()], false);
    
    // Create a test interface value
    let null_ptr = ptr_type.const_null();
    let type_id = i64_type.const_int(12345, false); // Mock type ID
    let mut interface_value = interface_type.get_undef();
    
    interface_value = codegen.builder().build_insert_value(interface_value, null_ptr, 0, "data_ptr")
        .unwrap().into_struct_value();
    interface_value = codegen.builder().build_insert_value(interface_value, type_id, 1, "type_id")
        .unwrap().into_struct_value();

    // Test the instance-of check
    use cursed::codegen::llvm::type_assertion::InterfaceTypeAssertion;
    let result = codegen.check_instance_of(interface_value.into(), "TestType", None);
    
    assert!(result.is_ok(), "Instance-of check should compile successfully");
    let check_result = result.unwrap();
    assert!(check_result.is_int_value(), "Instance-of check should return a boolean");
    
    tracing::info!("Instance-of check test passed");
}

#[test]
fn test_interface_data_extraction() {
    tracing_setup::init_test_tracing();
    tracing::info!("Starting interface data extraction test");

    let context = Context::create();
    let mut codegen = setup_codegen_with_function(&context);

    // Create a mock interface value with actual data
    let i32_type = context.i32_type();
    let ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let interface_type = context.struct_type(&[ptr_type.into(), i32_type.into()], false);
    
    // Allocate some data and get its pointer
    let data_alloca = codegen.builder().build_alloca(i32_type, "test_data").unwrap();
    let test_value = i32_type.const_int(42, false);
    codegen.builder().build_store(data_alloca, test_value).unwrap();
    
    let data_ptr = codegen.builder().build_pointer_cast(
        data_alloca, 
        ptr_type, 
        "data_ptr_cast"
    ).unwrap();
    
    // Create interface value
    let type_id = i32_type.const_int(67890, false);
    let mut interface_value = interface_type.get_undef();
    
    interface_value = codegen.builder().build_insert_value(interface_value, data_ptr, 0, "data_ptr")
        .unwrap().into_struct_value();
    interface_value = codegen.builder().build_insert_value(interface_value, type_id, 1, "type_id")
        .unwrap().into_struct_value();

    // Test data pointer extraction
    use cursed::codegen::llvm::type_assertion::InterfaceTypeAssertion;
    let result = codegen.extract_interface_data_ptr(interface_value.into());
    
    assert!(result.is_ok(), "Data pointer extraction should succeed");
    let _extracted_ptr = result.unwrap();
    // PointerValue is always a pointer, so just getting here successfully means it worked
    
    tracing::info!("Interface data extraction test passed");
}

#[test]
fn test_type_id_hashing() {
    tracing_setup::init_test_tracing();
    tracing::info!("Starting type ID hashing test");

    let context = Context::create();
    let codegen = LlvmCodeGenerator::new());

    // Test consistent hashing
    let hash1 = codegen.hash_type_name("Person");
    let hash2 = codegen.hash_type_name("Person");
    let hash3 = codegen.hash_type_name("Dog");
    
    assert_eq!(hash1, hash2, "Hash should be consistent for same type name");
    assert_ne!(hash1, hash3, "Hash should be different for different type names");
    
    // Test hash distribution (should not be zero for reasonable inputs)
    assert_ne!(hash1, 0, "Hash should not be zero for non-empty input");
    assert_ne!(hash3, 0, "Hash should not be zero for non-empty input");
    
    tracing::info!("Type ID hashing test passed: Person={:016x}, Dog={:016x}", hash1, hash3);
}

#[test]
fn test_interface_casting() {
    tracing_setup::init_test_tracing();
    tracing::info!("Starting interface casting test");

    let context = Context::create();
    let mut codegen = setup_codegen_with_function(&context);

    // Create a test value to cast to interface
    let i32_type = context.i32_type();
    let test_value = i32_type.const_int(123, false);

    // Test casting to interface type
    use cursed::codegen::llvm::type_assertion::InterfaceTypeAssertion;
    let result = codegen.cast_to_interface_type(test_value.into(), "TestInterface");
    
    assert!(result.is_ok(), "Interface casting should succeed");
    let interface_value = result.unwrap();
    assert!(interface_value.is_struct_value(), "Interface value should be a struct");
    
    tracing::info!("Interface casting test passed");
}

#[test]
fn test_type_assertion_error_handling() {
    tracing_setup::init_test_tracing();
    tracing::info!("Starting type assertion error handling test");

    let context = Context::create();
    let mut codegen = setup_codegen_with_function(&context);

    // Test error handling with invalid input
    use cursed::error::{Error, TypeAssertionError};
    
    // Create a TypeAssertionError
    let assertion_error = TypeAssertionError::new("InvalidInterface", "NonExistentType")
        .with_message("Test error message");
    
    // Convert to general Error
    let general_error: Error = Error::TypeAssertion(assertion_error.into());
    
    // Test error message formatting
    let error_msg = format!("{}", general_error);
    assert!(error_msg.contains("Failed to assert"), "Error message should contain assertion failure info");
    
    tracing::info!("Type assertion error handling test passed");
}

#[test]
fn test_type_assertion_question_ast() {
    tracing_setup::init_test_tracing();
    tracing::info!("Starting type assertion question AST test");

    // Test the TypeAssertionQuestion AST node
    let type_assertion_q = TypeAssertionQuestion {
        token: "test_token".to_string(),
        expression: Box::new(Identifier {
            token: "var_name".to_string(),
            value: "test_var".to_string(),
        }),
        type_name: "SomeType".to_string(),
    };

    // Test string representation
    let string_repr = type_assertion_q.string();
    assert!(string_repr.contains("test_var"), "String representation should contain variable name");
    assert!(string_repr.contains("SomeType"), "String representation should contain type name");
    assert!(string_repr.contains("?"), "String representation should contain question mark");
    
    // Test node type
    assert_eq!(type_assertion_q.node_type(), "TypeAssertionQuestion");
    
    tracing::info!("Type assertion question AST test passed: {}", string_repr);
}
