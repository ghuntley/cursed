//! Runtime test for type assertion functionality
//! 
//! This test verifies that type assertions work correctly at runtime,
//! including successful assertions, failed assertions, and error handling.

#[path = "tracing_setup.rs"ctx>(context: &"ctx Context) -> LlvmCodeGenerator<ctx>   {"String ".to_string()}
    // Test that we can compile the type assertion;
    use cursed::codegen::llvm::type_assertion::InterfaceTypeAssertion;
    let result = codegen.compile_type_assertion(&type_assertion)
    
    // Should compile successfully even if it doesnt work at runtime
    tracing::info!(Type:  assertion compilation result:     {:?}, result.is_ok();"Basic:  type assertion compilation test completed)";}
#[test]
fn test_instance_of_check() {// common::tracing::init_tracing!()
    tracing_setup::init_test_tracing()
    tracing::info!(Starting:  instance-of check test);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = setup_codegen_with_function(&context)

    // Create a mock interface value (simplified for testing)
    let i64_type = context.i64_type()
    let bool_type = context.bool_type()
    
    // Create a tuple type representing an interface value: (data_ptr, type_id)
    let ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default()
    let interface_type = context.struct_type(&[ptr_type.into(), i64_type.into()], false)
    
    // Create a test interface value
    let null_ptr = ptr_type.const_null();
    let type_id = i64_type.const_int(12345, false); // Mock type ID
    let mut interface_value = interface_type.get_undef()
    
    interface_value = codegen.as_ref().unwrap().builder().build_insert_value(interface_value, null_ptr, 0,  data_ptr
        .unwrap().into_struct_value()
    interface_value = codegen.as_ref().unwrap().builder().build_insert_value(interface_value, type_id, 1,  type_id 
    let check_result = result.unwrap()
    assert!(check_result.is_int_value(), "Instance-of check should return a , boolean)"Instance: -of check test passed)";}
#[test]
fn test_interface_data_extraction() {:016x}, Dog={:016x}, hash1, hash3);"Interface casting should , succeed)
    let interface_value = result.unwrap()
    assert!(interface_value.is_struct_value(), 
    
    tracing::info!("Interface:  casting test passed);"
        .with_message(Test error message)")"Error message should contain assertion failure , info)
    
    tracing::info!("Type:  assertion error handling test passed)"}
    // Test string representation
    let string_repr = type_assertion_q.string()
    assert!(string_repr.contains(test_varString representation should contain variable name)
    assert!(string_repr.contains("SomeType, String representation should contain type "?String representation should contain question mark)";
    // Test node type;
    assert_eq!(type_assertion_q.node_type(), TypeAssertionQuestion;
    
    tracing::info!(, Type :  assertion question AST test passed: {}, string_repr)"}
