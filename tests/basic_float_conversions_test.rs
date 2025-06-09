//! Basic float conversion tests that work with current codebase

mod common;

use cursed::codegen::llvm::FloatTypeConverter;
use inkwell::context::Context;
use inkwell::module::Module;
use tracing::{info, debug};

#[test]
fn test_float_type_converter_creation() {
    // init_tracing!();
    common::tracing::setup();
    info!("Testing FloatTypeConverter creation");

    let context = Context::create();
    let converter = FloatTypeConverter::new(&context);
    
    // Test that the converter was created successfully
    assert_eq!(converter.context() as *const _, &context as *const _);
    
    info!("FloatTypeConverter creation test completed successfully");
}

#[test]
fn test_float_value_creation_and_conversion() {
    // init_tracing!();
    common::tracing::setup();
    info!("Testing basic float value creation and type conversions");

    let context = Context::create();
    let converter = FloatTypeConverter::new(&context);
    let module = context.create_module("test");
    
    // Create a test function to hold our conversions
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_conversions", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    converter.builder().position_at_end(basic_block);

    // Test creating float constants
    let f32_value = context.f32_type().const_float(3.14159);
    let f64_value = context.f64_type().const_float(2.71828);
    
    debug!("Created float values: f32={:?}, f64={:?}", f32_value, f64_value);
    
    // Test float-to-float conversions
    let f32_to_f64_result = converter.convert_float_to_float(f32_value, true);
    assert!(f32_to_f64_result.is_ok(), "f32 to f64 conversion should succeed");
    
    let f64_to_f32_result = converter.convert_float_to_float(f64_value, false);
    assert!(f64_to_f32_result.is_ok(), "f64 to f32 conversion should succeed");
    
    info!("Float value creation and conversion test completed successfully");
}

#[test]
fn test_float_to_integer_conversions() {
    // init_tracing!();
    common::tracing::setup();
    info!("Testing float to integer conversions");

    let context = Context::create();
    let converter = FloatTypeConverter::new(&context);
    let module = context.create_module("test");
    
    // Create a test function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_float_to_int", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    converter.builder().position_at_end(basic_block);

    // Test various float to integer conversions
    let test_value = context.f64_type().const_float(42.7);
    
    // Test conversion to different integer sizes
    let to_i8_result = converter.convert_float_to_int(test_value, 8, true);
    assert!(to_i8_result.is_ok(), "Float to i8 conversion should succeed");
    
    let to_i16_result = converter.convert_float_to_int(test_value, 16, true);
    assert!(to_i16_result.is_ok(), "Float to i16 conversion should succeed");
    
    let to_i32_result = converter.convert_float_to_int(test_value, 32, true);
    assert!(to_i32_result.is_ok(), "Float to i32 conversion should succeed");
    
    let to_i64_result = converter.convert_float_to_int(test_value, 64, true);
    assert!(to_i64_result.is_ok(), "Float to i64 conversion should succeed");
    
    info!("Float to integer conversions test completed successfully");
}

#[test]
fn test_integer_to_float_conversions() {
    // init_tracing!();
    common::tracing::setup();
    info!("Testing integer to float conversions");

    let context = Context::create();
    let converter = FloatTypeConverter::new(&context);
    let module = context.create_module("test");
    
    // Create a test function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_int_to_float", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    converter.builder().position_at_end(basic_block);

    // Test integer to float conversions
    let test_value = context.i32_type().const_int(42, false);
    
    let to_f32_result = converter.convert_int_to_float(test_value, false, true);
    assert!(to_f32_result.is_ok(), "Integer to f32 conversion should succeed");
    
    let to_f64_result = converter.convert_int_to_float(test_value, true, true);
    assert!(to_f64_result.is_ok(), "Integer to f64 conversion should succeed");
    
    info!("Integer to float conversions test completed successfully");
}

#[test]
fn test_float_to_boolean_conversion() {
    // init_tracing!();
    common::tracing::setup();
    info!("Testing float to boolean conversions");

    let context = Context::create();
    let converter = FloatTypeConverter::new(&context);
    let module = context.create_module("test");
    
    // Create a test function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_float_to_bool", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    converter.builder().position_at_end(basic_block);

    // Test various float values to boolean
    let zero_value = context.f64_type().const_float(0.0);
    let non_zero_value = context.f64_type().const_float(42.0);
    let negative_value = context.f64_type().const_float(-1.0);
    
    let zero_to_bool = converter.convert_float_to_bool(zero_value);
    assert!(zero_to_bool.is_ok(), "Zero to bool conversion should succeed");
    
    let non_zero_to_bool = converter.convert_float_to_bool(non_zero_value);
    assert!(non_zero_to_bool.is_ok(), "Non-zero to bool conversion should succeed");
    
    let negative_to_bool = converter.convert_float_to_bool(negative_value);
    assert!(negative_to_bool.is_ok(), "Negative to bool conversion should succeed");
    
    info!("Float to boolean conversions test completed successfully");
}

#[test]
fn test_bounds_checking() {
    // init_tracing!();
    common::tracing::setup();
    info!("Testing bounds checking for float-to-int conversions");

    let context = Context::create();
    let converter = FloatTypeConverter::new(&context);
    
    // Test that type limits are calculated correctly
    assert_eq!(converter.get_int_type_limits(8, true), (-128.0, 127.0));
    assert_eq!(converter.get_int_type_limits(8, false), (0.0, 255.0));
    assert_eq!(converter.get_int_type_limits(16, true), (-32768.0, 32767.0));
    assert_eq!(converter.get_int_type_limits(32, true), (-2147483648.0, 2147483647.0));
    
    info!("Bounds checking test completed successfully");
}

#[test]
fn test_cursed_type_conversions() {
    // init_tracing!();
    common::tracing::setup();
    info!("Testing CURSED-specific type conversions");

    let context = Context::create();
    let converter = FloatTypeConverter::new(&context);
    let module = context.create_module("test");
    
    // Create a test function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_cursed_types", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    converter.builder().position_at_end(basic_block);

    // Test CURSED type-specific conversions
    let test_float = context.f64_type().const_float(42.7);
    
    // Test conversions to CURSED integer types
    let to_smol = converter.to_smol(test_float);
    assert!(to_smol.is_ok(), "Conversion to smol (i8) should succeed");
    
    let to_mid = converter.to_mid(test_float);
    assert!(to_mid.is_ok(), "Conversion to mid (i16) should succeed");
    
    let to_normie = converter.to_normie(test_float);
    assert!(to_normie.is_ok(), "Conversion to normie (i32) should succeed");
    
    let to_thicc = converter.to_thicc(test_float);
    assert!(to_thicc.is_ok(), "Conversion to thicc (i64) should succeed");
    
    info!("CURSED type conversions test completed successfully");
}

#[test]
fn test_special_values_compilation() {
    // init_tracing!();
    common::tracing::setup();
    info!("Testing that special value handling compiles correctly");

    let context = Context::create();
    let converter = FloatTypeConverter::new(&context);
    let module = context.create_module("test");
    
    // Create a test function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_special_values", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    converter.builder().position_at_end(basic_block);

    // Test special value constants
    let nan_value = context.f64_type().const_float(f64::NAN);
    let inf_value = context.f64_type().const_float(f64::INFINITY);
    let neg_inf_value = context.f64_type().const_float(f64::NEG_INFINITY);
    let neg_zero_value = context.f64_type().const_float(-0.0);
    
    debug!("Created special values: NaN={:?}, +∞={:?}, -∞={:?}, -0={:?}", 
           nan_value, inf_value, neg_inf_value, neg_zero_value);
    
    // Test that conversion of special values compiles (may return errors, but should compile)
    let _nan_to_int = converter.convert_float_to_int(nan_value, 32, true);
    let _inf_to_int = converter.convert_float_to_int(inf_value, 32, true);
    let _neg_inf_to_int = converter.convert_float_to_int(neg_inf_value, 32, true);
    let _neg_zero_to_bool = converter.convert_float_to_bool(neg_zero_value);
    
    info!("Special values compilation test completed successfully");
}

#[test]
fn test_error_handling() {
    // init_tracing!();
    common::tracing::setup();
    info!("Testing error handling in float conversions");

    let context = Context::create();
    let converter = FloatTypeConverter::new(&context);
    
    // Test unsupported bit widths
    let test_float = context.f64_type().const_float(42.0);
    let unsupported_result = converter.convert_float_to_int(test_float, 128, true);
    assert!(unsupported_result.is_err(), "Unsupported bit width should return error");
    
    if let Err(error_msg) = unsupported_result {
        assert!(error_msg.contains("Unsupported integer bit width"), 
                "Error message should mention unsupported bit width");
    }
    
    info!("Error handling test completed successfully");
}
