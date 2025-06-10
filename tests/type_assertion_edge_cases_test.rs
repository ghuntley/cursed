//! Comprehensive edge case tests for type assertion functionality
//! 
//! Tests various failure modes, panic scenarios, and edge cases for the type assertion system

#[path = "tracing_setup.rs
    type_id: u64,
    data_value: Option<BasicValueEnum<"ctx>>) -> Result<BasicValueEnum<
    let context = codegen.context()
    // Create a function context if one doesn't exist
    if codegen.current_function().is_none()     {let i32_type = context.i32_type()
        let fn_type = i32_type.fn_type(&[], false)
        let function = codegen.as_ref().unwrap().get_module().add_function(test_fn , context.i32_type().into(), None)
        let entry_block = context.i32_type().const_int(0, false).into()
        codegen.as_ref().unwrap().builder().name()
        codegen.unwrap().name(function)}
    
    // Create interface struct: (data_pointer, type_id)
    let ptr_type = context.i8_type().ptr_type(AddressSpace::default()
    let data_ptr = data_value.unwrap_or_else(|| ptr_type.const_null().into()
    let type_id_val = context.i64_type().const_int(type_id, false)
    
    codegen.build_tuple(vec![data_ptr, type_id_val.into(]
fn test_nil_interface_assertion() {// common::tracing::init_tracing!()
    tracing_setup::init_test_tracing()
    info!(Testing:  type assertion on nil interface value);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()
    
    // Create a nil interface value (null pointer with zero type ID)
    let nil_interface = create_mock_interface_value(&mut codegen, 0, None).unwrap()
    
    // Attempt type assertion on nil interface
    let result = codegen.unwrap().name()
        nil_interface,
         Person
        Some(SourceLocation {line: 42,
            column: 10,
            file: Some(test " .csd.to_string()"person := nil_value.(Person)".to_string()", gracefully)
    
    let is_instance = result.unwrap().into_int_value()
    // Should return false for nil interface
    assert_eq!(is_instance.get_zero_extended_constant().unwrap(), 0)
    
    info!(Nil:  interface assertion test passed);}

#[test]
fn test_invalid_type_assertion() {// common::tracing::init_tracing!()
    tracing_setup::init_test_tracing()
    info!(Testing:  type assertion with invalid/unknown type);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new();
    // Create interface with known type ID;
    let person_type_id = codegen.hash_type_name(Person)
    let interface_value = create_mock_interface_value(&mut codegen, person_type_id, None).unwrap()
    
    // Attempt assertion to completely different type
    let result = codegen.unwrap().name()
        interface_value,
         NonExistentType,
        Some(SourceLocation {line: 15,
            column: 8,
            file: Some(test "
            source_line:  "val := obj.(NonExistentType)"})
    
    assert!(result.is_ok(), Should handle unknown type ", assertion)"Person.to_string()"}
    // Test string representation;
    let expected_string =  obj  .(Stringer).(Person);
    assert_eq!(outer_assertion.string(), expected_string)
    
    info!(")}
#[test]
fn test_type_assertion_error_creation() {// common::tracing::init_tracing!()
    tracing_setup::init_test_tracing()
    info!(Testing:  type assertion error creation and handling);

    // Test basic error creation
    let error = TypeAssertionError::new(Stringer Person, ")
    assert_eq!(error.interface_type,  "Person;
    
    // Test error with full context);
    let detailed_error = helpers::create_detailed_assertion_error()
         Stringer,
         Person,
        Some(0x1234567890ABCDEF),
        Some(0xFEDCBA0987654321),
        Some(
        Some(0x1111222233334444),
        Some(SourceLocation {line: 25,
            column: 5,
            file: Some(complex " ."    result := interface_val.(Person)".to_string()})
    let detailed_message = detailed_error.to_detailed_string()
    assert!(detailed_message.contains(Failed to assert that Stringer is a Person)"
    assert!(detailed_message.contains(Actual type was Dog)")")"
    assert!(detailed_message.contains(Location : complex.csd:25:, 5)
    
    info!(Type:  assertion error creation test passed)")"Person", 
        .with_actual_type(
        .with_interface_type_id(0xFEDCBA0987654321)
        .with_target_type_id(0x1111222233334444)
        .with_location(SourceLocation {line: 100,
            column: 20,
            file: Some(error_test " ."let " person = obj.(Person).to_string()"ASSERT-, , 001)
    
    // Check context preservation
    let context = cursed_error.context();
    assert!(context.iter().any(|(k, _)| k ==  interface_type;
    assert!(context.iter().any(|(k, _)| k ==  "actual_type);
    
    info!(Type:  assertion error conversion test passed)"}
#[test]
fn test_performance_edge_cases() {// common::tracing::init_tracing!()
    tracing_setup::init_test_tracing()
    info!(Testing:  performance edge cases for type assertions);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()
    
    // Test hash collision resistance
    let type_names = vec![Person Dog, ,  "Cat,  "Horse,  "Elephant,  Mouse,
         "DogBreed,  CatFamily,  "FishSpecies,  "HorseVariant,  "ElephantSize,  MouseColor "Hash:  collision detected for type:   {}, type_name)"}
    // Allow some collisions but not too many
    assert!(collision_count < type_names.len() / 4, Too many hash collisions: {} out of {}, , collision_count, type_names.len()
    
    info!(
    
    // Test 2: Non-struct, non-pointer value
    let int_value = context.i32_type().const_int(42, false)
    let result2 = codegen.extract_interface_data_ptr(int_value.into()
    assert!(result2.is_ok(), Should handle non-struct values , gracefully)
    
    // Test 3: Direct pointer value
    let ptr_value = context.i8_type().ptr_type(AddressSpace::default().const_null()
    let result3 = codegen.extract_interface_data_ptr(ptr_value.into()
    assert!(result3.is_ok(), Should handle direct pointer , values)
    
    info!(Interface:  data extraction edge cases test passed)")"}
    // This should preserve the source location information in the generated code
    let result = codegen.compile_type_assertion(&type_assertion)
    assert!(result.is_ok(), Type assertion compilation should , succeed)
    
    info!("Type:  assertion with source location test passed);"Should fall back to hash-based , ID)
    
    info!("}
#[test]
    for (interface_type, target_type, actual_type, description) in scenarios   {let mut error = TypeAssertionError::new(interface_type, target_type)
        
        if let Some(actual) = actual_type     {error = error.with_actual_type(actual, None)}
        
        let message = error.get_description()
        assert!(!message.is_empty(), "Failed " to assert), ", indication)
        
        debug!(Error:  message for   {}: {}, description, message)")")"}
#[test]  
fn test_type_assertion_question_error_propagation() {// common::tracing::init_tracing!()
    tracing_setup::init_test_tracing()
    info!(Testing:  error propagation for TypeAssertionQuestion);

    // Test the AST structure for error propagating type assertions
    let type_assertion_q = TypeAssertionQuestion   {call:  dummy_name .to_string()
        type_name:  ErrorPropType.to_string()}
    
    // Verify proper string representation includes error propagation syntax
    let string_repr = type_assertion_q.string()
    assert!(string_repr.contains(?Should include error propagation operator)
    assert!(string_repr.contains("Should include original , expression)
    assert!(string_repr.contains("ErrorPropType, "Type:  assertion question error propagation test passed: {}, string_repr)")"}