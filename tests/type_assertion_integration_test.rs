//! Integration test for type assertion functionality
//! 
//! This test verifies that both regular type assertions (`expr.(Type)`) and 
//! error-propagating type assertions (`expr.(Type)?`) work correctly with the LLVM code generator.

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

use cursed::ast::{TypeAssertion, TypeAssertionQuestion, Identifier, StringLiteral};
use cursed::ast::traits::{Expression, Node};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::token::Token;
use inkwell::context::Context;
use inkwell::types::AnyType;
use std::path::PathBuf;

fn create_test_type_assertion() -> Box<TypeAssertion> {
    Box::new(TypeAssertion {
        token: "test_token".to_string(),
        expression: Box::new(Identifier {
            token: "test_var".to_string(),
            value: "test_var".to_string(),
        }),
        type_name: "Person".to_string(),
    })
}

fn create_test_type_assertion_question() -> Box<TypeAssertionQuestion> {
    Box::new(TypeAssertionQuestion {
        token: "test_token".to_string(),
        expression: Box::new(Identifier {
            token: "test_var".to_string(),
            value: "test_var".to_string(),
        }),
        type_name: "Person".to_string(),
    })
}

#[test]
fn test_type_assertion_basic_functionality() {
    // init_tracing!();
    tracing_setup::init_test_tracing();
    tracing::info!("Starting basic type assertion functionality test");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new());

    // Test basic type assertion creation
    let type_assertion = create_test_type_assertion();
    
    // Verify the AST structure
    assert_eq!(type_assertion.type_name, "Person");
    assert_eq!(type_assertion.token, "test_token");
    assert_eq!(type_assertion.string(), "test_var.(Person)");
    
    tracing::info!("Basic type assertion structure test passed");
}

#[test]
fn test_type_assertion_question_basic_functionality() {
    // init_tracing!();
    tracing_setup::init_test_tracing();
    tracing::info!("Starting type assertion with question mark functionality test");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new());

    // Test type assertion with error propagation
    let type_assertion = create_test_type_assertion_question();
    
    // Verify the AST structure
    assert_eq!(type_assertion.type_name, "Person");
    assert_eq!(type_assertion.token, "test_token");
    assert_eq!(type_assertion.string(), "test_var.(Person)?\n");
    
    tracing::info!("Type assertion with question mark structure test passed");
}

#[test]
fn test_type_assertion_hash_function() {
    // init_tracing!();
    tracing_setup::init_test_tracing();
    tracing::info!("Starting type assertion hash function test");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new());

    // Test the hash function for type names
    let hash1 = codegen.hash_type_name("Person");
    let hash2 = codegen.hash_type_name("Dog");
    let hash3 = codegen.hash_type_name("Person"); // Should be same as hash1

    assert_eq!(hash1, hash3);
    assert_ne!(hash1, hash2);
    
    tracing::info!("Hash function test passed: Person={:016x}, Dog={:016x}", hash1, hash2);
}

#[test]
fn test_type_assertion_tuple_building() {
    // init_tracing!();
    tracing_setup::init_test_tracing();
    tracing::info!("Starting tuple building test");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new());

    // Create a simple function to get a valid insertion context
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = codegen.module().add_function("test_fn", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    codegen.builder().position_at_end(entry_block);
    codegen.set_current_function(function);

    // Test tuple building functionality
    let bool_val = context.bool_type().const_int(1, false);
    let int_val = context.i32_type().const_int(42, false);
    
    let result = codegen.build_tuple(vec![bool_val.into(), int_val.into()]);
    assert!(result.is_ok(), "Tuple building should succeed");
    
    tracing::info!("Tuple building test passed");
}

#[test]
fn test_type_id_generation() {
    // init_tracing!();
    tracing_setup::init_test_tracing();
    tracing::info!("Starting type ID generation test");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new());

    // Test type ID generation
    let type_id_result = codegen.get_type_id("Person");
    assert!(type_id_result.is_ok(), "Type ID generation should succeed");
    
    let type_id = type_id_result.unwrap();
    assert!(type_id.is_int_value(), "Type ID should be an integer value");
    
    tracing::info!("Type ID generation test passed");
}

#[test] 
fn test_pointer_type_helper() {
    // init_tracing!();
    tracing_setup::init_test_tracing();
    tracing::info!("Starting pointer type helper test");

    let context = Context::create();
    let codegen = LlvmCodeGenerator::new());

    // Test pointer type helper
    let ptr_type = codegen.pointer_type();
    // Pointer types in inkwell don't have is_pointer_type method
    // Just verify we got a pointer type by checking it's a pointer type enum
    assert!(matches!(ptr_type.as_any_type_enum(), inkwell::types::AnyTypeEnum::PointerType(_)), "Should return a pointer type");
    
    tracing::info!("Pointer type helper test passed");
}

#[test]
fn test_registry_initialization() {
    // init_tracing!();
    tracing_setup::init_test_tracing();
    tracing::info!("Starting registry initialization test");

    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new());

    // Test registry initialization
    let result = codegen.ensure_registry_visualization_initialized();
    assert!(result.is_ok(), "Registry initialization should succeed");
    
    tracing::info!("Registry initialization test passed");
}

#[test]
fn test_interface_path_visualization() {
    // init_tracing!();
    tracing_setup::init_test_tracing();
    tracing::info!("Starting interface path visualization test");

    let context = Context::create();
    let codegen = LlvmCodeGenerator::new());

    // Test interface path visualization
    let result = codegen.visualize_interface_path("Stringer", "Person");
    assert!(result.is_ok(), "Interface path visualization should succeed");
    
    let path = result.unwrap();
    assert!(path.contains("Stringer"), "Path should contain source interface");
    assert!(path.contains("Person"), "Path should contain target interface");
    
    tracing::info!("Interface path visualization test passed: {}", path);
}
