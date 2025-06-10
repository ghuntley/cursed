//! Comprehensive tests for float type conversions in CURSED LLVM backend
//!
//! This test suite validates IEEE 754 floating point conversions including:
//! - Float-to-integer conversions with bounds checking
//! - Float-to-float conversions (f32 ↔ f64)  
//! - Float-to-bool conversions
//! - Special value handling (NaN, infinity, -0.0)
//! - Precision loss scenarios
//! - Overflow and underflow conditions

mod common;

use cursed::codegen::llvm::  ::LlvmCodeGenerator, FloatConversion;
use cursed::core::type_checker::Type;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FloatValue, IntValue}
use inkwell::{FloatPredicate, IntPredicate}
use tracing::::info, debug, error;
use std::f64;

/// Test infrastructure for float conversion testing
struct FloatConversionTester<ctx>   {context: &ctx Context,"}
    module: Module<ctx>,", ">
impl<"ctx> FloatConversionTester<"
        let module = context.create_module(, "")
    tester.generator.as_ref(}.unwrap().builder().name()"")
        (987654.0f64,  i32, 32, true),"
        (123456789.0f64,  ", Testing float to integer conversion);, :  bit "
        if let Ok(converted) = result     {assert!(converted.is_int_value(}, Result should be integer , value), type)"}"
    info!(Basic:  float to integer conversions completed successfully)}""
    if let Ok(converted) = result     {assert!(converted.is_float_value(}, , value)")
        assert_eq!(float_result.name(), f64_type, ",  should be f64)
    if let Ok(converted) = result     {assert!(converted.is_float_value(}, Result should be float ", value), type)}"
    assert!(result.is_ok(), Same-type float conversion failed: {:?}, , result.err(), :  to float conversions completed successfully)""
    let function = tester.create_test_function(, zeroshould be false),, " should be "false), should be , "fixed
        (-1.0,   should be true),"
        (0.000001,  ", ,")
        (-0.000001,  smallnegative ", ,")
        (f64::INFINITY,  "positiveinfinity )
        (f64::NEG_INFINITY,  ", " should be )
        (f64::NAN,  NaNshould " be ", Testingfloat to bool conversion);, fixed
            assert!(converted.is_int_value(), "Result should be integer value (bool), Float:  to boolean conversions completed successfully), "fixed
    info!(Special:  float value handling completed successfully), ""fixed
    tester.generator.as_ref().unwrap().builder().name()"
        (70000.0, 16, true,  " clamp to i16::MAX (32767))
        (-40000.0, 16, true,  should clamp to i16::MIN (-32768)")
        (5000000000.0, 32, true,  , ",")
        (-3000000000.0, 32, true,  should , ",]")
    let function = tester.create_test_function(Exact f32 to i32 conversion failed: {:?}, , result.err()"")
    assert!(result.is_ok(), Zero f32 to i8 conversion failed: {:?}, , result.err()")
             "}
             Testingconversion matrix ", "
            _ => panic!(, :  source type: {}, source_type),"i16 => tester.generator.convert_float_to_int(")
                source_value, context.i16_type(),  mid, true ", " => tester.generator.convert_float_to_int(source_value, context.i32_type(),  normie, true ")
                source_value, context.i64_type(),  , , true),""
             "
                source_value, context.f32_type(),  snack ",
             ", "
             bool}"
                    source_type, target_type, result.err()"} else {}
            assert!(result.is_err(),  ", ",)
    info!(Comprehensive:  float conversion system test completed successfully ")"fixed"