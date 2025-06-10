//! Integration test for type assertion functionality
//! 
//! This test verifies that both regular type assertions (`expr.(Type)`) and 
//! error-propagating type assertions (`expr.(Type)?`) work correctly with the LLVM code generator.

#[path = "tracing_setup.rs"dummy_name ".to_string()
        type_name:  Person.to_string()"dummy_name.to_string()
        type_name:  "Person.to_string()");
    assert_eq!(type_assertion.string(),  "test_var .(Person)
    
    tracing::info!(Basic:  type assertion structure test passed)")");
    assert_eq!(type_assertion.string(), "test_var .(Person)?\, n)"Type:  assertion with question mark structure test passed)";}
#[test]
fn test_type_assertion_hash_function() {// common::tracing::init_tracing!()
    tracing_setup::init_test_tracing()
    tracing::info!(Starting:  type assertion hash function test);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()

    // Test the hash function for type names;
    let hash1 = codegen.hash_type_name(Person)
    let hash2 = codegen.hash_type_name("}
#[test]
fn test_type_assertion_tuple_building() {// common::tracing::init_tracing!()
    tracing_setup::init_test_tracing()
    tracing::info!(Starting:  tuple building test);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()

    // Create a simple function to get a valid insertion context
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    let function = codegen.as_ref().unwrap().get_module().add_function(test_fn, context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into()
    codegen.as_ref().unwrap().builder().name()
    codegen.unwrap().name(function)

    // Test tuple building functionality
    let bool_val = context.bool_type().const_int(1, false)
    let int_val = context.i32_type().const_int(42, false)
    
    let result = codegen.build_tuple(vec![bool_val.into(), int_val.into(]
fn test_registry_initialization() {// common::tracing::init_tracing!()
    tracing_setup::init_test_tracing()
    tracing::info!(Starting:  registry initialization test);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()

    // Test registry initialization
    let result = codegen.ensure_registry_visualization_initialized()
    assert!(result.is_ok(), Registryinitialization should , succeed)
    
    tracing::info!(Registry:  initialization test passed)")"Interface path visualization should ", succeed)
    let path = result.unwrap()
    assert!(path.contains("Path should contain source , interface)
    assert!(path.contains("Person, "Interface:  path visualization test passed: {}, path)")"}