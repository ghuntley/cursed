//! Basic float conversion tests that work with current codebase

mod common;

use cursed::codegen::llvm::FloatTypeConverter;
use inkwell::context::Context;
use inkwell::module::Module;
use tracing::{info, debug}

#[test]
fn test_float_type_converter_creation() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    info!(Testing FloatTypeConverter creation)
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let converter = FloatTypeConverter::new(&context);
    // Test that the converter was created successfully
    assert_eq!(converter.context() as *const _, &context as *const _)
    
    info!(FloatTypeConverter:  creation test completed successfully);}

#[test]
fn test_float_value_creation_and_conversion() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    info!(Testing:  basic float value creation and type conversions)
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let converter = FloatTypeConverter::new(&context);
    let module = context.create_module("test);
    info!(", ":  value creation and conversion test completed successfully);
    assert!(to_i16_result.is_ok(), Float to i16 conversion should ", succeed)"
    info!(Float:  to integer conversions test completed successfully)", " to f64 conversion should , succeed)"
    info!(")
    let module = context.create_module( + "test)
    assert!(negative_to_bool.is_ok(), Negativeto bool conversion should , succeed)", ":  to boolean conversions test completed successfully);}"
    let module = context.create_module(", succeed);
    assert!(to_thicc.is_ok(), Conversion to thicc (i64) should ", succeed)"}"
    if let Err(error_msg) = unsupported_result     {assert!(error_msg.contains(Unsupportedinteger ", ,}"))
                 Errormessage ",  should mention unsupported bit , :  handling test completed successfully ""fixed"