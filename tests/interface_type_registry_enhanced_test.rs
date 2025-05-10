//! Tests for the enhanced interface type registry functionality

use std::sync::Arc;
use std::collections::HashMap;
use std::sync::RwLock;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::AddressSpace;

use cursed::error::Error;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_type_registry_enhanced::{EnhancedTypeRegistry, RuntimeTypeInfo};

// Import test utilities
mod common;
use common::tracing;

#[test]
fn test_type_registry_basic_operations() {
    // Initialize tracing for this test
    tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create empty maps for a new code generator
    let interface_registry = Arc::new(RwLock::new(HashMap::new()));
    
    // Create a code generator with the registry
    let mut code_gen = LlvmCodeGenerator::new(
        &context,
        module,
        builder,
        OptimizationLevel::None,
    );
    
    code_gen.interface_type_registry = Some(interface_registry);
    
    // Create a main function to hold our test code
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = code_gen.module().add_function("main", main_fn_type, None);
    let entry_bb = context.append_basic_block(main_fn, "entry");
    code_gen.builder().position_at_end(entry_bb);
    
    // Register some test types
    let type1_id = code_gen.register_type("TestType1").expect("Failed to register type 1");
    let type2_id = code_gen.register_type("TestType2").expect("Failed to register type 2");
    let type3_id = code_gen.register_type("TestType3").expect("Failed to register type 3");
    
    // Verify the IDs are unique
    assert_ne!(type1_id, type2_id);
    assert_ne!(type1_id, type3_id);
    assert_ne!(type2_id, type3_id);
    
    // Test lookup by name
    let type1_lookup = code_gen.get_type_id("TestType1").expect("Failed to look up type 1");
    let type2_lookup = code_gen.get_type_id("TestType2").expect("Failed to look up type 2");
    
    assert_eq!(type1_id, type1_lookup);
    assert_eq!(type2_id, type2_lookup);
    
    // Test lookup by ID
    let name1 = code_gen.lookup_type_name(type1_id).expect("Failed to look up name 1");
    let name2 = code_gen.lookup_type_name(type2_id).expect("Failed to look up name 2");
    
    assert_eq!(name1, "TestType1");
    assert_eq!(name2, "TestType2");
    
    // Test re-registration of the same type returns the same ID
    let type1_again = code_gen.register_type("TestType1").expect("Failed to re-register type 1");
    assert_eq!(type1_id, type1_again);
    
    // Test that invalid lookups return appropriate errors
    let invalid_lookup = code_gen.get_type_id("NonExistentType");
    assert!(invalid_lookup.is_err());
    
    let invalid_name = code_gen.lookup_type_name(999999);
    assert!(invalid_name.is_none());
    
    // Complete the function
    let return_val = context.i32_type().const_int(0, false);
    code_gen.builder().build_return(Some(&return_val)).expect("Failed to build return");
    
    // Verify the module
    code_gen.module().verify().expect("Module verification failed");
}

#[test]
fn test_global_type_registry_initialization() {
    // Initialize tracing for this test
    tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create a code generator
    let mut code_gen = LlvmCodeGenerator::new(
        &context,
        module,
        builder,
        OptimizationLevel::None,
    );
    
    // Create a main function to hold our test code
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = code_gen.module().add_function("main", main_fn_type, None);
    let entry_bb = context.append_basic_block(main_fn, "entry");
    code_gen.builder().position_at_end(entry_bb);
    
    // Register some types
    code_gen.register_type("TestStruct1").expect("Failed to register TestStruct1");
    code_gen.register_type("TestStruct2").expect("Failed to register TestStruct2");
    code_gen.register_type("TestInterface").expect("Failed to register TestInterface");
    
    // Initialize the global registry
    code_gen.initialize_global_type_registry().expect("Failed to initialize global registry");
    
    // Verify the global arrays were created
    assert!(code_gen.global_type_names.is_some());
    assert!(code_gen.global_type_count.is_some());
    
    // Complete the function
    let return_val = context.i32_type().const_int(0, false);
    code_gen.builder().build_return(Some(&return_val)).expect("Failed to build return");
    
    // Verify the module
    code_gen.module().verify().expect("Module verification failed");
}

#[test]
fn test_runtime_type_name_lookup() {
    // Initialize tracing for this test
    tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create a code generator
    let mut code_gen = LlvmCodeGenerator::new(
        &context,
        module,
        builder,
        OptimizationLevel::None,
    );
    
    // Create a function that performs runtime type name lookup
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let lookup_fn_type = i8_ptr_type.fn_type(&[context.i64_type().into()], false);
    let lookup_fn = code_gen.module().add_function("test_lookup", lookup_fn_type, None);
    let entry_bb = context.append_basic_block(lookup_fn, "entry");
    code_gen.builder().position_at_end(entry_bb);
    
    // Get the type ID parameter
    let type_id_param = lookup_fn.get_nth_param(0).expect("Failed to get type ID parameter");
    
    // Register some test types
    let type1_id = code_gen.register_type("TestType1").expect("Failed to register type 1");
    let type2_id = code_gen.register_type("TestType2").expect("Failed to register type 2");
    
    // Initialize the global registry
    code_gen.initialize_global_type_registry().expect("Failed to initialize global registry");
    
    // Add the test types to the global array manually (usually done by initialize_global_type_registry)
    code_gen.add_type_to_global_registry(type1_id, "TestType1").expect("Failed to add type1 to global registry");
    code_gen.add_type_to_global_registry(type2_id, "TestType2").expect("Failed to add type2 to global registry");
    
    // Get the runtime type name for the parameter
    let type_name_ptr = code_gen.get_runtime_type_name(type_id_param.into()).expect("Failed to get runtime type name");
    
    // Return the type name pointer
    code_gen.builder().build_return(Some(&type_name_ptr)).expect("Failed to build return");
    
    // Verify the function
    code_gen.module().verify().expect("Function verification failed");
    
    // Create a JIT execution engine
    let execution_engine = code_gen.module().create_jit_execution_engine(OptimizationLevel::None)
        .expect("Failed to create execution engine");
    
    // Define the function signature for the JIT
    type LookupFunc = unsafe extern "C" fn(u64) -> *const i8;
    
    // Get a pointer to the JIT-compiled function
    let jit_lookup: JitFunction<LookupFunc> = unsafe {
        execution_engine.get_function("test_lookup").expect("Failed to get JIT function")
    };
    
    // Test the function with valid type IDs
    unsafe {
        let result1_ptr = jit_lookup.call(type1_id);
        let result2_ptr = jit_lookup.call(type2_id);
        
        // Convert the returned C strings to Rust strings
        let result1 = std::ffi::CStr::from_ptr(result1_ptr).to_str().unwrap();
        let result2 = std::ffi::CStr::from_ptr(result2_ptr).to_str().unwrap();
        
        assert_eq!(result1, "TestType1");
        assert_eq!(result2, "TestType2");
        
        // Test with an invalid type ID (should return "<invalid>" or similar)
        let invalid_result_ptr = jit_lookup.call(999999);
        let invalid_result = std::ffi::CStr::from_ptr(invalid_result_ptr).to_str().unwrap();
        
        assert!(invalid_result == "<invalid>" || invalid_result == "<unknown>", 
                "Expected invalid type name, got: {}", invalid_result);
    }
}

#[test]
fn test_type_assertion_error_formatting() {
    // Initialize tracing for this test
    tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create a code generator
    let mut code_gen = LlvmCodeGenerator::new(
        &context,
        module,
        builder,
        OptimizationLevel::None,
    );
    
    // Register some test types
    let type1_id = code_gen.register_type("String").expect("Failed to register String");
    let type2_id = code_gen.register_type("Person").expect("Failed to register Person");
    
    // Create test code that formats error messages
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let error_fn_type = i8_ptr_type.fn_type(&[context.i64_type().into()], false);
    let error_fn = code_gen.module().add_function("test_error", error_fn_type, None);
    let entry_bb = context.append_basic_block(error_fn, "entry");
    code_gen.builder().position_at_end(entry_bb);
    
    // Get the actual type ID parameter
    let actual_type_id = error_fn.get_nth_param(0).expect("Failed to get type ID parameter");
    
    // Initialize the global registry
    code_gen.initialize_global_type_registry().expect("Failed to initialize global registry");
    
    // Add the test types to the global registry
    code_gen.add_type_to_global_registry(type1_id, "String").expect("Failed to add String to global registry");
    code_gen.add_type_to_global_registry(type2_id, "Person").expect("Failed to add Person to global registry");
    
    // Generate an error message for a type assertion failure (String to Person)
    let error_message = code_gen.create_type_assertion_error(
        actual_type_id.into(),
        "Person"
    ).expect("Failed to create type assertion error");
    
    // Return the error message
    code_gen.builder().build_return(Some(&error_message)).expect("Failed to build return");
    
    // Verify the function
    code_gen.module().verify().expect("Function verification failed");
    
    // Create a JIT execution engine
    let execution_engine = code_gen.module().create_jit_execution_engine(OptimizationLevel::None)
        .expect("Failed to create execution engine");
    
    // Define the function signature for the JIT
    type ErrorFunc = unsafe extern "C" fn(u64) -> *const i8;
    
    // Get a pointer to the JIT-compiled function
    let jit_error: JitFunction<ErrorFunc> = unsafe {
        execution_engine.get_function("test_error").expect("Failed to get JIT function")
    };
    
    // Test the error message generation with String -> Person assertion
    unsafe {
        let error_msg_ptr = jit_error.call(type1_id);
        let error_msg = std::ffi::CStr::from_ptr(error_msg_ptr).to_str().unwrap();
        
        // The message should contain both the actual and expected type names
        assert!(error_msg.contains("String"), "Error message doesn't contain actual type: {}", error_msg);
        assert!(error_msg.contains("Person"), "Error message doesn't contain expected type: {}", error_msg);
        assert!(error_msg.contains("cannot convert"), "Error message doesn't contain 'cannot convert': {}", error_msg);
    }
}

#[test]
fn test_runtime_type_info_integration() {
    // Initialize tracing for this test
    tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create a code generator
    let mut code_gen = LlvmCodeGenerator::new(
        &context,
        module,
        builder,
        OptimizationLevel::None,
    );
    
    // Register some test types
    code_gen.register_type("Person").expect("Failed to register Person");
    code_gen.register_type("Customer").expect("Failed to register Customer");
    code_gen.register_type("Employee").expect("Failed to register Employee");
    
    // Create a simple interface struct type
    let interface_struct_type = context.struct_type(&[
        context.i8_type().ptr_type(AddressSpace::default()).into(), // data pointer
        context.i8_type().ptr_type(AddressSpace::default()).into()  // vtable pointer
    ], false);
    
    // Create a vtable struct type for test purposes
    let vtable_struct_type = context.struct_type(&[
        context.i64_type().into(), // type ID
        context.i8_type().ptr_type(AddressSpace::default()).into() // method table
    ], false);
    
    // Create a main function to test within
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = code_gen.module().add_function("main", main_fn_type, None);
    let entry_bb = context.append_basic_block(main_fn, "entry");
    code_gen.builder().position_at_end(entry_bb);
    
    // Generate code for a test interface value
    let data_ptr = code_gen.builder().build_alloca(context.i8_type(), "dummy_data")
        .expect("Failed to allocate dummy data");
    
    // Create a global vtable
    let vtable_type_id = context.i64_type().const_int(code_gen.get_type_id("Person").expect("Failed to get type ID"), false);
    let method_table_ptr = context.i8_type().ptr_type(AddressSpace::default()).const_null();
    let vtable_struct = vtable_struct_type.const_named_struct(&[
        vtable_type_id.into(),
        method_table_ptr.into()
    ]);
    
    let vtable_global = module.add_global(vtable_struct_type, None, "person_vtable");
    vtable_global.set_initializer(&vtable_struct);
    let vtable_ptr = vtable_global.as_pointer_value();
    
    // Create an interface value
    let interface_value = code_gen.builder().build_alloca(interface_struct_type, "interface_value")
        .expect("Failed to allocate interface value");
    
    // Set data pointer
    let data_ptr_gep = code_gen.builder().build_struct_gep(interface_struct_type, interface_value, 0, "data_ptr_gep")
        .expect("Failed to build data pointer GEP");
    code_gen.builder().build_store(data_ptr_gep, data_ptr)
        .expect("Failed to store data pointer");
    
    // Set vtable pointer
    let vtable_ptr_gep = code_gen.builder().build_struct_gep(interface_struct_type, interface_value, 1, "vtable_ptr_gep")
        .expect("Failed to build vtable pointer GEP");
    code_gen.builder().build_store(vtable_ptr_gep, vtable_ptr)
        .expect("Failed to store vtable pointer");
    
    // Load the interface value
    let loaded_interface = code_gen.builder().build_load(interface_struct_type, interface_value, "loaded_interface")
        .expect("Failed to load interface value");
    
    // Initialize the global registry
    code_gen.initialize_global_type_registry().expect("Failed to initialize global registry");
    
    // Test format_type_assertion_error
    let error_msg = code_gen.format_type_assertion_error(
        vtable_type_id.into(),
        "Customer"
    ).expect("Failed to format error message");
    
    assert!(error_msg.contains("Person"), "Error message should contain actual type name");
    assert!(error_msg.contains("Customer"), "Error message should contain expected type name");
    
    // Complete the function
    let return_val = context.i32_type().const_int(0, false);
    code_gen.builder().build_return(Some(&return_val)).expect("Failed to build return");
    
    // Verify the module
    code_gen.module().verify().expect("Module verification failed");
}