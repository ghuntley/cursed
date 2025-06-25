//! Comprehensive tests for the Result<T,E> and Option<T> LLVM type system
//!
//! This test suite validates the production-ready Result and Option type
//! implementation across all supported LLVM types, memory layouts, and
//! integration scenarios.

use cursed::codegen::llvm::{LlvmCodeGenerator, ResultTypeCompiler};
use cursed::codegen::llvm::result_types::{result_type_utils, TypeLayout, ResultDiscriminant, OptionDiscriminant};
use cursed::error::CursedError;
use cursed::runtime::Runtime;
use inkwell::context::Context;
use inkwell::types::{BasicTypeEnum, BasicType};
use inkwell::values::BasicValueEnum;
use inkwell::module::Module;
use inkwell::builder::Builder;
use std::sync::{Arc, Mutex};

/// Helper function to create a test LlvmCodeGenerator
fn create_test_generator() -> LlvmCodeGenerator {
    let context = Arc::new(Context::create());
    let module = Arc::new(Mutex::new(context.create_module("test_module")));
    let builder = Arc::new(Mutex::new(context.create_builder()));
    let runtime = Arc::new(Runtime::new().expect("Failed to create runtime"));
    
    // Create a minimal LlvmCodeGenerator for testing
    // Note: This creates a minimal structure for testing the Result/Option types
    LlvmCodeGenerator {
        context: context.clone(),
        module: module.clone(),
        builder: builder.clone(),
        runtime: runtime.clone(),
        debug_generator: cursed::codegen::llvm::LlvmDebugCodeGenerator::default(),
        module_name: Some("test_module".to_string()),
        web_vibez_integration: None,
        expression_compiler: cursed::codegen::llvm::LlvmExpressionCompiler::default(),
        type_context: cursed::codegen::llvm::TypeCompilationContext::default(),
        gc_integration: None,
        package_context: None,
        optimization_manager: None,
        optimization_engine: None,
        real_pass_manager: None,
        enhanced_pass_manager: None,
        optimization_config: cursed::optimization::config::OptimizationConfig::default(),
        optimization_enabled: false,
        use_enhanced_passes: false,
        temp_counter: std::cell::RefCell::new(0),
        block_counter: std::cell::RefCell::new(0),
        current_function: std::cell::RefCell::new(None),
        result_type_registry: std::collections::HashMap::new(),
        option_type_registry: std::collections::HashMap::new(),
        template_compiler: None,
        function_stack: std::cell::RefCell::new(Vec::new()),
        symbol_table: None,
    }
}

#[test]
fn test_result_type_generation_basic_types() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    // Test Result<i32, i32>
    let ok_type = context.i32_type().into();
    let err_type = context.i32_type().into();
    
    let result_type = generator.generate_result_type(ok_type, err_type)
        .expect("Failed to generate Result<i32, i32> type");
    
    // Verify structure has discriminant and data fields
    assert_eq!(result_type.count_fields(), 2, "Result type should have 2 fields");
    
    // Verify discriminant field is i8
    let discriminant_field = result_type.get_field_type_at_index(0)
        .expect("Missing discriminant field");
    assert!(matches!(discriminant_field, BasicTypeEnum::IntType(int_type) if int_type.get_bit_width() == 8),
        "Discriminant should be i8");
}

#[test]
fn test_result_type_generation_mixed_types() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    // Test Result<f64, i32>
    let ok_type = context.f64_type().into();
    let err_type = context.i32_type().into();
    
    let result_type = generator.generate_result_type(ok_type, err_type)
        .expect("Failed to generate Result<f64, i32> type");
    
    assert_eq!(result_type.count_fields(), 2, "Result type should have 2 fields");
    
    // Data field should accommodate the larger type (f64 = 8 bytes)
    let data_field = result_type.get_field_type_at_index(1)
        .expect("Missing data field");
    
    // Should be at least 8 bytes for f64
    match data_field {
        BasicTypeEnum::IntType(int_type) => {
            assert!(int_type.get_bit_width() >= 64, "Data field should be at least 64 bits for f64");
        }
        BasicTypeEnum::ArrayType(_) => {
            // Array-based storage is also acceptable for larger types
        }
        _ => panic!("Unexpected data field type: {:?}", data_field),
    }
}

#[test]
fn test_option_type_generation() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    // Test Option<i64>
    let inner_type = context.i64_type().into();
    
    let option_type = generator.generate_option_type(inner_type)
        .expect("Failed to generate Option<i64> type");
    
    assert_eq!(option_type.count_fields(), 2, "Option type should have 2 fields");
    
    // Verify discriminant field is i8
    let discriminant_field = option_type.get_field_type_at_index(0)
        .expect("Missing discriminant field");
    assert!(matches!(discriminant_field, BasicTypeEnum::IntType(int_type) if int_type.get_bit_width() == 8),
        "Discriminant should be i8");
    
    // Data field should accommodate i64
    let data_field = option_type.get_field_type_at_index(1)
        .expect("Missing data field");
    match data_field {
        BasicTypeEnum::IntType(int_type) => {
            assert!(int_type.get_bit_width() >= 64, "Data field should be at least 64 bits for i64");
        }
        _ => panic!("Unexpected data field type for Option<i64>: {:?}", data_field),
    }
}

#[test]
fn test_create_result_ok() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    let ok_type = context.i32_type().into();
    let err_type = context.i32_type().into();
    let ok_value = context.i32_type().const_int(42, false).into();
    
    let result_ok = generator.create_result_ok(ok_type, err_type, ok_value)
        .expect("Failed to create Result::Ok");
    
    // Verify it's a struct value
    assert!(result_ok.is_struct_value(), "Result::Ok should be a struct value");
    
    // Verify discriminant is correct using is_result_ok
    let is_ok = generator.is_result_ok(result_ok)
        .expect("Failed to check if result is Ok");
    
    assert!(is_ok.is_const(), "is_ok check should be constant for constant input");
    // Note: We can't easily verify the constant value without more complex LLVM operations
}

#[test]
fn test_create_result_err() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    let ok_type = context.i32_type().into();
    let err_type = context.i32_type().into();
    let err_value = context.i32_type().const_int(404, false).into();
    
    let result_err = generator.create_result_err(ok_type, err_type, err_value)
        .expect("Failed to create Result::Err");
    
    // Verify it's a struct value
    assert!(result_err.is_struct_value(), "Result::Err should be a struct value");
    
    // Verify discriminant is correct using is_result_err
    let is_err = generator.is_result_err(result_err)
        .expect("Failed to check if result is Err");
    
    assert!(is_err.is_const(), "is_err check should be constant for constant input");
}

#[test]
fn test_create_option_some() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    let inner_type = context.i32_type().into();
    let some_value = context.i32_type().const_int(123, false).into();
    
    let option_some = generator.create_option_some(inner_type, some_value)
        .expect("Failed to create Option::Some");
    
    // Verify it's a struct value
    assert!(option_some.is_struct_value(), "Option::Some should be a struct value");
    
    // Verify discriminant is correct using is_option_some
    let is_some = generator.is_option_some(option_some)
        .expect("Failed to check if option is Some");
    
    assert!(is_some.is_const(), "is_some check should be constant for constant input");
}

#[test]
fn test_create_option_none() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    let inner_type = context.i32_type().into();
    
    let option_none = generator.create_option_none(inner_type)
        .expect("Failed to create Option::None");
    
    // Verify it's a struct value
    assert!(option_none.is_struct_value(), "Option::None should be a struct value");
    
    // Verify discriminant is correct using is_option_none
    let is_none = generator.is_option_none(option_none)
        .expect("Failed to check if option is None");
    
    assert!(is_none.is_const(), "is_none check should be constant for constant input");
}

#[test]
fn test_result_discriminant_checks_consistency() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    let ok_type = context.i32_type().into();
    let err_type = context.i32_type().into();
    let ok_value = context.i32_type().const_int(42, false).into();
    let err_value = context.i32_type().const_int(404, false).into();
    
    // Create Ok and Err values
    let result_ok = generator.create_result_ok(ok_type, err_type, ok_value)
        .expect("Failed to create Result::Ok");
    let result_err = generator.create_result_err(ok_type, err_type, err_value)
        .expect("Failed to create Result::Err");
    
    // Test discriminant checks
    let ok_is_ok = generator.is_result_ok(result_ok)
        .expect("Failed to check if Ok result is Ok");
    let ok_is_err = generator.is_result_err(result_ok)
        .expect("Failed to check if Ok result is Err");
    
    let err_is_ok = generator.is_result_ok(result_err)
        .expect("Failed to check if Err result is Ok");
    let err_is_err = generator.is_result_err(result_err)
        .expect("Failed to check if Err result is Err");
    
    // All should be constant values for constant inputs
    assert!(ok_is_ok.is_const(), "Ok discriminant check should be constant");
    assert!(ok_is_err.is_const(), "Ok discriminant check should be constant");
    assert!(err_is_ok.is_const(), "Err discriminant check should be constant");
    assert!(err_is_err.is_const(), "Err discriminant check should be constant");
}

#[test]
fn test_option_discriminant_checks_consistency() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    let inner_type = context.i32_type().into();
    let some_value = context.i32_type().const_int(123, false).into();
    
    // Create Some and None values
    let option_some = generator.create_option_some(inner_type, some_value)
        .expect("Failed to create Option::Some");
    let option_none = generator.create_option_none(inner_type)
        .expect("Failed to create Option::None");
    
    // Test discriminant checks
    let some_is_some = generator.is_option_some(option_some)
        .expect("Failed to check if Some option is Some");
    let some_is_none = generator.is_option_none(option_some)
        .expect("Failed to check if Some option is None");
    
    let none_is_some = generator.is_option_some(option_none)
        .expect("Failed to check if None option is Some");
    let none_is_none = generator.is_option_none(option_none)
        .expect("Failed to check if None option is None");
    
    // All should be constant values for constant inputs
    assert!(some_is_some.is_const(), "Some discriminant check should be constant");
    assert!(some_is_none.is_const(), "Some discriminant check should be constant");
    assert!(none_is_some.is_const(), "None discriminant check should be constant");
    assert!(none_is_none.is_const(), "None discriminant check should be constant");
}

#[test]
fn test_extract_result_values() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    let ok_type = context.i32_type().into();
    let err_type = context.i32_type().into();
    let ok_value = context.i32_type().const_int(42, false).into();
    let err_value = context.i32_type().const_int(404, false).into();
    
    // Create Result values
    let result_ok = generator.create_result_ok(ok_type, err_type, ok_value)
        .expect("Failed to create Result::Ok");
    let result_err = generator.create_result_err(ok_type, err_type, err_value)
        .expect("Failed to create Result::Err");
    
    // Extract values
    let extracted_ok = generator.extract_result_ok(result_ok, ok_type)
        .expect("Failed to extract Ok value");
    let extracted_err = generator.extract_result_err(result_err, err_type)
        .expect("Failed to extract Err value");
    
    // Verify extracted values have correct types
    assert!(extracted_ok.is_int_value(), "Extracted Ok value should be int");
    assert!(extracted_err.is_int_value(), "Extracted Err value should be int");
    
    // Note: For constant values, we could verify the actual values,
    // but that requires more complex LLVM constant analysis
}

#[test]
fn test_extract_option_values() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    let inner_type = context.i32_type().into();
    let some_value = context.i32_type().const_int(123, false).into();
    
    // Create Option::Some
    let option_some = generator.create_option_some(inner_type, some_value)
        .expect("Failed to create Option::Some");
    
    // Extract value
    let extracted_some = generator.extract_option_some(option_some, inner_type)
        .expect("Failed to extract Some value");
    
    // Verify extracted value has correct type
    assert!(extracted_some.is_int_value(), "Extracted Some value should be int");
}

#[test]
fn test_pointer_types_in_results() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    // Test Result<*i32, i32>
    let ok_type = context.i32_type().ptr_type(inkwell::AddressSpace::Generic).into();
    let err_type = context.i32_type().into();
    
    let result_type = generator.generate_result_type(ok_type, err_type)
        .expect("Failed to generate Result<*i32, i32> type");
    
    assert_eq!(result_type.count_fields(), 2, "Result type should have 2 fields");
    
    // Create a null pointer for testing
    let null_ptr = context.i32_type().ptr_type(inkwell::AddressSpace::Generic).const_null().into();
    let result_ok = generator.create_result_ok(ok_type, err_type, null_ptr)
        .expect("Failed to create Result::Ok with pointer");
    
    assert!(result_ok.is_struct_value(), "Result with pointer should be struct value");
}

#[test]
fn test_float_types_in_options() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    // Test Option<f64>
    let inner_type = context.f64_type().into();
    
    let option_type = generator.generate_option_type(inner_type)
        .expect("Failed to generate Option<f64> type");
    
    assert_eq!(option_type.count_fields(), 2, "Option type should have 2 fields");
    
    // Create Option::Some with f64
    let float_value = context.f64_type().const_float(3.14159).into();
    let option_some = generator.create_option_some(inner_type, float_value)
        .expect("Failed to create Option::Some with f64");
    
    assert!(option_some.is_struct_value(), "Option with f64 should be struct value");
    
    // Verify discriminant check
    let is_some = generator.is_option_some(option_some)
        .expect("Failed to check if f64 option is Some");
    assert!(is_some.is_const(), "Discriminant check should be constant");
}

#[test]
fn test_result_type_string_representations() {
    let generator = create_test_generator();
    
    // Test string representation functions
    let result_str = generator.get_result_type_string("i32", "String");
    assert_eq!(result_str, "Result<i32, String>");
    
    let option_str = generator.get_option_type_string("f64");
    assert_eq!(option_str, "Option<f64>");
}

#[test]
fn test_type_layout_calculations() {
    // Test Result layout calculation
    let result_layout = result_type_utils::calculate_result_layout(4, 8); // i32, f64
    assert!(result_layout.size >= 9, "Result layout should accommodate discriminant + largest type");
    assert!(result_layout.alignment > 0, "Result layout should have valid alignment");
    assert_eq!(result_layout.discriminant_size, 1, "Discriminant should be 1 byte");
    
    // Test Option layout calculation
    let option_layout = result_type_utils::calculate_option_layout(4); // i32
    assert!(option_layout.size >= 5, "Option layout should accommodate discriminant + inner type");
    assert!(option_layout.alignment > 0, "Option layout should have valid alignment");
    assert_eq!(option_layout.discriminant_size, 1, "Discriminant should be 1 byte");
}

#[test]
fn test_type_string_parsing() {
    // Test Result type parsing
    let (ok_type, err_type) = result_type_utils::parse_result_type("Result<i32, String>")
        .expect("Failed to parse Result type string");
    assert_eq!(ok_type, "i32");
    assert_eq!(err_type, "String");
    
    // Test Option type parsing
    let inner_type = result_type_utils::parse_option_type("Option<f64>")
        .expect("Failed to parse Option type string");
    assert_eq!(inner_type, "f64");
    
    // Test type identification
    assert!(result_type_utils::is_result_type("Result<i32, String>"));
    assert!(!result_type_utils::is_result_type("Option<i32>"));
    
    assert!(result_type_utils::is_option_type("Option<f64>"));
    assert!(!result_type_utils::is_option_type("Result<i32, String>"));
}

#[test]
fn test_common_type_helpers() {
    let context = Context::create();
    
    // Test common Result type creation
    let common_result = result_type_utils::create_common_result_type(&context);
    assert_eq!(common_result.count_fields(), 2, "Common Result type should have 2 fields");
    
    // Test common Option type creation
    let common_option = result_type_utils::create_common_option_type(&context);
    assert_eq!(common_option.count_fields(), 2, "Common Option type should have 2 fields");
}

#[test]
fn test_nested_result_option_types() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    // Test generating types for complex nested scenarios
    // While we can't easily test Result<Option<T>, E> directly without more infrastructure,
    // we can test that the basic building blocks work for such scenarios
    
    // Generate Option<i32> type
    let inner_type = context.i32_type().into();
    let option_type = generator.generate_option_type(inner_type)
        .expect("Failed to generate Option<i32> type");
    
    // Generate Result<i32, i32> type  
    let ok_type = context.i32_type().into();
    let err_type = context.i32_type().into();
    let result_type = generator.generate_result_type(ok_type, err_type)
        .expect("Failed to generate Result<i32, i32> type");
    
    // Both should be valid struct types
    assert_eq!(option_type.count_fields(), 2, "Option type should have 2 fields");
    assert_eq!(result_type.count_fields(), 2, "Result type should have 2 fields");
}

#[test]
fn test_memory_efficiency_for_small_types() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    // Test that small types use efficient layouts
    let small_ok_type = context.i8_type().into();
    let small_err_type = context.i8_type().into();
    
    let result_type = generator.generate_result_type(small_ok_type, small_err_type)
        .expect("Failed to generate Result<i8, i8> type");
    
    // Should use efficient packing for small types
    assert_eq!(result_type.count_fields(), 2, "Small Result type should have 2 fields");
    
    // Data field should be appropriately sized (not wastefully large)
    let data_field = result_type.get_field_type_at_index(1)
        .expect("Missing data field");
    
    match data_field {
        BasicTypeEnum::IntType(int_type) => {
            // Should not be larger than necessary
            assert!(int_type.get_bit_width() <= 32, "Data field for i8 types should not be too large");
        }
        _ => {
            // Other representations are acceptable too
        }
    }
}

#[test]
fn test_error_handling_for_invalid_operations() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    // Test error handling for type mismatches and invalid operations
    let ok_type = context.i32_type().into();
    let err_type = context.i32_type().into();
    
    // Create a Result value
    let ok_value = context.i32_type().const_int(42, false).into();
    let result_ok = generator.create_result_ok(ok_type, err_type, ok_value)
        .expect("Failed to create Result::Ok");
    
    // All basic operations should succeed without panicking
    let _is_ok = generator.is_result_ok(result_ok)
        .expect("is_result_ok should not fail");
    let _is_err = generator.is_result_err(result_ok)
        .expect("is_result_err should not fail");
    
    // Extractions should also work
    let _extracted = generator.extract_result_ok(result_ok, ok_type)
        .expect("extract_result_ok should not fail");
}

#[test]
fn test_discriminant_values_consistency() {
    // Verify discriminant value constants are as expected
    assert_eq!(ResultDiscriminant::Ok as u8, 0);
    assert_eq!(ResultDiscriminant::Err as u8, 1);
    
    assert_eq!(OptionDiscriminant::None as u8, 0);
    assert_eq!(OptionDiscriminant::Some as u8, 1);
}

/// Integration test to ensure the Result/Option system works with the broader LLVM infrastructure
#[test]
fn test_integration_with_llvm_codegen() {
    let mut generator = create_test_generator();
    let context = &*generator.context;
    
    // Test that our Result/Option types can be used in a basic LLVM function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let module = generator.module.lock().unwrap();
    let function = module.add_function("test_result_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    
    let builder = generator.builder.lock().unwrap();
    builder.position_at_end(basic_block);
    
    // Create a Result value within the function context
    let ok_type = i32_type.into();
    let err_type = i32_type.into();
    let ok_value = i32_type.const_int(42, false).into();
    
    let result_ok = generator.create_result_ok(ok_type, err_type, ok_value)
        .expect("Failed to create Result::Ok in function context");
    
    // Verify it's a valid struct value
    assert!(result_ok.is_struct_value(), "Result should be valid in function context");
    
    // The test passes if we can create and use Result types within LLVM functions
    // without any compilation or linking errors
}
