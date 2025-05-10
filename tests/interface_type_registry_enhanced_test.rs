//! Tests for the enhanced interface type registry implementation

use std::sync::Arc;
use cursed::codegen::llvm::interface_type_registry_enhanced::*;
use cursed::error::Error;

#[path = "common.rs"]
mod common;

#[test]
fn test_interface_registry_initialization() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    // Create a new LLVM context and code generator
    let context = inkwell::context::Context::create();
    let module_name = "test_interface_registry";
    let code_gen = cursed::codegen::llvm::LlvmCodeGenerator::new(
        &context, 
        module_name, 
        std::path::PathBuf::from("test_file.csd")
    );
    
    // Verify the unique ID counter is initialized
    assert_ne!(code_gen.unique_id_counter.load(std::sync::atomic::Ordering::SeqCst), 0);
    
    // Verify the interface registry is initialized
    assert!(code_gen.interface_type_registry.is_some());
}

#[test]
fn test_type_registration() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    // Create a new LLVM context and code generator
    let context = inkwell::context::Context::create();
    let module_name = "test_type_registration";
    let mut code_gen = cursed::codegen::llvm::LlvmCodeGenerator::new(
        &context, 
        module_name, 
        std::path::PathBuf::from("test_file.csd")
    );
    
    // Register a few types
    let type1_id = code_gen.register_type("Vector2D").expect("Failed to register type");
    let type2_id = code_gen.register_type("Person").expect("Failed to register type");
    let type3_id = code_gen.register_type("Logger").expect("Failed to register type");
    
    // Verify IDs are different
    assert_ne!(type1_id, type2_id);
    assert_ne!(type1_id, type3_id);
    assert_ne!(type2_id, type3_id);
    
    // Look up types by ID
    let type1_name = code_gen.lookup_type_name(type1_id).expect("Type not found");
    let type2_name = code_gen.lookup_type_name(type2_id).expect("Type not found");
    let type3_name = code_gen.lookup_type_name(type3_id).expect("Type not found");
    
    // Verify names match
    assert_eq!(type1_name, "Vector2D");
    assert_eq!(type2_name, "Person");
    assert_eq!(type3_name, "Logger");
    
    // Verify type ID lookup works
    let id1 = code_gen.get_type_id("Vector2D").expect("Type not found");
    let id2 = code_gen.get_type_id("Person").expect("Type not found");
    let id3 = code_gen.get_type_id("Logger").expect("Type not found");
    
    assert_eq!(id1, type1_id);
    assert_eq!(id2, type2_id);
    assert_eq!(id3, type3_id);
}

#[test]
fn test_global_type_registry_initialization() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    // Create a new LLVM context and code generator
    let context = inkwell::context::Context::create();
    let module_name = "test_global_registry";
    let mut code_gen = cursed::codegen::llvm::LlvmCodeGenerator::new(
        &context, 
        module_name, 
        std::path::PathBuf::from("test_file.csd")
    );
    
    // Initialize global registry
    assert!(code_gen.initialize_global_type_registry().is_ok());
    
    // Verify globals are initialized
    assert!(code_gen.global_type_names.is_some());
    assert!(code_gen.global_type_count.is_some());
    
    // Register a type and add it to the global registry
    let type_id = code_gen.register_type("TestType").expect("Failed to register type");
    let result = code_gen.add_type_to_global_registry(type_id, "TestType");
    assert!(result.is_ok());
}

#[test]
fn test_runtime_type_name_lookup() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    // Create a new LLVM context and code generator
    let context = inkwell::context::Context::create();
    let module_name = "test_type_lookup";
    let mut code_gen = cursed::codegen::llvm::LlvmCodeGenerator::new(
        &context, 
        module_name, 
        std::path::PathBuf::from("test_file.csd")
    );
    
    // Register a type
    let type_id = code_gen.register_type("RuntimeType").expect("Failed to register type");
    
    // Initialize global registry and add the type
    code_gen.initialize_global_type_registry().expect("Failed to initialize registry");
    code_gen.add_type_to_global_registry(type_id, "RuntimeType").expect("Failed to add type");
    
    // Create a function where we can test the runtime lookup
    let function_type = context.void_type().fn_type(&[], false);
    let function = code_gen.module.add_function("test_type_lookup", function_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    code_gen.builder.position_at_end(basic_block);
    
    // Current function must be set for get_runtime_type_name to work
    code_gen.current_function = Some(function);
    
    // Create a type ID constant and try to look up the type name
    let type_id_val = context.i64_type().const_int(type_id, false);
    let type_name_ptr = code_gen.get_runtime_type_name(type_id_val.into()).expect("Failed to get type name");
    
    // We can't directly check the string value here since it's a pointer to runtime data,
    // but we can check that we got a valid pointer
    assert!(!type_name_ptr.is_null());
    
    // Add a return instruction to complete the function
    code_gen.builder.build_return(None).expect("Failed to build return");
    
    // Verify the module to make sure everything is valid
    assert!(code_gen.module.verify().is_ok());
}