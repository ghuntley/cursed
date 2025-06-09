//! Simple focused test for float conversion functionality

#[path = "common.rs"]
mod common;

use cursed::codegen::llvm::FloatConversion;
use inkwell::context::Context;
use tracing::info;

#[test]
fn test_float_conversion_trait_exists() {
    common::tracing::setup();
    info!("Testing that FloatConversion trait exists and compiles");

    let context = Context::create();
    
    // Test that the type limits function works correctly
    assert_eq!(cursed::codegen::llvm::float_conversions::FloatConversion::get_int_type_limits(8, true), (-128.0, 127.0));
    assert_eq!(cursed::codegen::llvm::float_conversions::FloatConversion::get_int_type_limits(8, false), (0.0, 255.0));
    assert_eq!(cursed::codegen::llvm::float_conversions::FloatConversion::get_int_type_limits(32, true), (-2147483648.0, 2147483647.0));
    
    info!("Float conversion trait validation completed");
}

#[test]
fn test_ieee754_special_values() {
    common::tracing::setup();
    info!("Testing IEEE 754 special value constants");

    // Test that special values work as expected
    assert!(f64::NAN.is_nan());
    assert!(f64::INFINITY.is_infinite());
    assert!(f64::NEG_INFINITY.is_infinite());
    assert!(!f64::INFINITY.is_nan());
    assert!(!f64::NEG_INFINITY.is_nan());
    
    // Test zero values
    assert_eq!(0.0f64.to_bits(), 0x0000000000000000u64);
    assert_eq!((-0.0f64).to_bits(), 0x8000000000000000u64);
    
    info!("IEEE 754 special values test completed");
}

#[test]
fn test_float_conversion_module_exists() {
    common::tracing::setup();
    info!("Testing that float conversion module compiles and links");

    // Test that we can access the module and its components
    // This validates that the module structure is correct
    let _trait_exists = std::any::type_name::<dyn cursed::codegen::llvm::FloatConversion>();
    
    info!("Float conversion module validation completed");
}
