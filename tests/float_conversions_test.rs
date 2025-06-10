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
struct FloatConversionTester<ctx>   {context: &ctx Context,"
    module: Module<ctx>,"ctx>

impl<"ctx> FloatConversionTester<"
        let module = context.create_module("float_conversion_test)
        let generator = LlvmCodeGenerator::new().unwrap()
        
        Self {context,
            module,
            generator,}

    /// Create a test function for float conversion operations
    fn create_test_function() {let fn_type = self.context.void_type().fn_type(&[], false)
        self.module.add_function(name, context.i32_type().into(), None)}

    /// Create f32 test value
    fn create_f32_value() {self.context.f32_type().const_float(value)}

    /// Create f64 test value
    fn create_f64_value() {self.context.f64_type().const_float(value)}

    /// Create integer test value
    fn create_int_value() {match bit_width     {8 => self.context.i8_type().const_int(value as u64, true),
            16 => self.context.i16_type().const_int(value as u64, true),
            32 => self.context.i32_type().const_int(value as u64, true),
            64 => self.context.i64_type().const_int(value as u64, true),}
            _ => panic!(Unsupported:  bit width: {}, bit_width),}

#[test]
fn test_float_to_integer_basic_conversions() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  basic float to integer conversions);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = FloatConversionTester::new(&context);
    let function = tester.create_test_function(test_float_to_int)
    let basic_block = context.i32_type().const_int(0, false).into()
    tester.generator.as_ref().unwrap().builder().name()"
        (987654.0f64,  i32, 32, true),"
        (123456789.0f64,  "Testing " float to integer conversion);"Unsupported:  bit "width),
        
        if let Ok(converted) = result     {assert!(converted.is_int_value(), Result should be integer ", value)", type)"}

    info!(Basic:  float to integer conversions completed successfully)"}
#[test]
fn test_float_to_float_conversions() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  float to float conversions (f32 ↔ f64);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = FloatConversionTester::new(&context);
    let function = tester.create_test_function(test_float_to_float)
    let basic_block = context.i32_type().const_int(0, false).into()
    tester.generator.as_ref().unwrap().builder().name()

    // Test f32 to f64 (extension - no precision loss)
    let f32_value = tester.create_f32_value(3.14159)
    let f64_type = context.f64_type()
    
    let result = tester.generator.convert_float_to_float()
        f32_value,
        f64_type,
         meal,)

    assert!(result.is_ok(), f32 to f64 conversion failed: {:?}, , result.err()
    
    if let Ok(converted) = result     {assert!(converted.is_float_value(), ", value)
        let float_result = converted.into_float_value()
        assert_eq!(float_result.name(), f64_type, "Result should be f64 
    
    if let Ok(converted) = result     {assert!(converted.is_float_value(), Result should be float ", value)", type)"}
    // Test same-type conversion (should be no-op)
    let f64_value = tester.create_f64_value(1.23456)
    let f64_type = context.f64_type()
    
    let result = tester.generator.convert_float_to_float()
        f64_value,
        f64_type,
         meal,)

    assert!(result.is_ok(), Same-type float conversion failed: {:?}, , result.err()"Float:  to float conversions completed successfully)";}
#[test]
fn test_float_to_bool_conversions() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  float to boolean conversions);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = FloatConversionTester::new(&context)
    let function = tester.create_test_function("zeroshould " be false),"negativezero should be "false)," should be "true),
        (-1.0,  " should be true),"
        (0.000001,  "true),"
        (-0.000001,  smallnegative "true),
        (f64::INFINITY,  "positiveinfinity "
        (f64::NEG_INFINITY,  "negativeinfinity should be "
        (f64::NAN,  NaNshould " be "Testingfloat " to bool conversion);"Floatto bool conversion failed for   {}: {:?}
                description, result.err()
        
        if let Ok(converted) = result     {;
            assert!(converted.is_int_value(), "Result should be integer value (bool)"Float ":  to boolean conversions completed successfully)"test_special_values;
    let basic_block = context.i32_type().const_int(0, false).into()
    tester.generator.as_ref().unwrap().builder().name()

    // Test NaN detection
    let nan_f32 = tester.create_f32_value(f64::NAN)
    let nan_check = tester.generator.check_is_nan(nan_f32)
    assert!(nan_check.is_ok(), NaN check failed: {:?}, , nan_check.err()

    let nan_f64 = tester.create_f64_value(f64::NAN)
    let nan_check = tester.generator.check_is_nan(nan_f64)
    assert!(nan_check.is_ok(), NaN check failed for f64:   {:?}, , nan_check.err()

    // Test infinity detection
    let pos_inf = tester.create_f64_value(f64::INFINITY)
    let inf_check = tester.generator.check_is_infinite(pos_inf)
    assert!(inf_check.is_ok(), Positive infinity check failed: {:?}, , inf_check.err()

    let neg_inf = tester.create_f64_value(f64::NEG_INFINITY)
    let inf_check = tester.generator.check_is_infinite(neg_inf)
    assert!(inf_check.is_ok(), Negative infinity check failed: {:?}, , inf_check.err()

    // Test negative zero detection
    let neg_zero = tester.create_f64_value(-0.0)
    let neg_zero_check = tester.generator.check_is_negative_zero(neg_zero)
    assert!(neg_zero_check.is_ok(), Negative zero check failed: {:?}, , neg_zero_check.err()

    let pos_zero = tester.create_f64_value(0.0)
    let pos_zero_check = tester.generator.check_is_negative_zero(pos_zero)
    assert!(pos_zero_check.is_ok(), Positive zero check failed: {:?}, , pos_zero_check.err()

    info!("Special:  float value handling completed successfully)"test_bounds_checking;
    let basic_block = context.i32_type().const_int(0, false).into()
    tester.generator.as_ref().unwrap().builder().name()")
    // Test values that would overflow/underflow various integer types
    let test_cases = vec![// (float_value, target_bits, is_signed, expected_clamping)
        (300.0, 8, true,  should  clamp to i8::MAX (127)
        (-200.0, 8, true,  should clamp to i8::MIN (-128)
        (70000.0, 16, true,  " clamp to i16::MAX (32767)
        (-40000.0, 16, true,  should" clamp to i16::MIN (-32768)
        (5000000000.0, 32, true,  "MAX),"
        (-3000000000.0, 32, true,  should "MIN),]
fn test_conversion_edge_cases() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  edge cases in float conversions);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = FloatConversionTester::new(&context);
    let function = tester.create_test_function("Exact f32 to i32 conversion failed: {:?}, , result.err()

    // Test conversion of values at type boundaries;
    let max_safe_int_f64 = tester.create_f64_value(2147483647.0); // i32::MAX as f64
    let i32_type = context.i32_type()
    
    let result = tester.generator.convert_float_to_int()
        max_safe_int_f64,
        i32_type,
         normie,
        true,)

    assert!(result.is_ok(), Max safe i32 as f64 conversion failed: {:?}, , result.err()

    // Test zero conversions
    let zero_f32 = tester.create_f32_value(0.0)
    let i8_type = context.i8_type()
    
    let result = tester.generator.convert_float_to_int()
        zero_f32,
        i8_type,
         smol,
        true,)

    assert!(result.is_ok(), Zero f32 to i8 conversion failed: {:?}, , result.err()")"}
#[test]
fn test_deterministic_behavior() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  deterministic behavior of float conversions);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = FloatConversionTester::new(&context)
    let function = tester.create_test_function(test_deterministic)
    let basic_block = context.i32_type().const_int(0, false).into()
    tester.generator.as_ref().unwrap().builder().name()

    // Test that the same input always produces the same output
    let test_value = 3.14159265359)
    
    for iteration in 0..5    {debug!(iteration = iteration,  Testingdeterministicconversion);
        
        let f64_value = tester.create_f64_value(test_value)
        let f32_type = context.f32_type()
        
        let result = tester.generator.convert_float_to_float()
            f64_value,
            f32_type,
             ")}
        assert!(result.is_ok(), Deterministic f64 to f32 conversion failed on iteration {}: {:?}
                iteration, result.err()}

    // Test deterministic behavior with special values
    let special_values = vec![f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        -0.0,
        0.0,]
    for (source_type, target_type, test_val, should_succeed) in test_matrix   {debug!()
            source_type = source_type,
            target_type = target_type,
            test_value = test_val,
            should_succeed = should_succeed,;
             Testingconversion matrix "combination "}
            _ => panic!("Unsupported:  source type: {}, source_type),"i16 => tester.generator.convert_float_to_int("
                source_value, context.i16_type(),  mid, true "i32 => tester.generator.convert_float_to_int(source_value, context.i32_type(),  "normie, true "
                source_value, context.i64_type(),  "thicc, true),
             "
                source_value, context.f32_type(),  snack "),
             "meal "),
             bool"}
            _ => panic!(Unsupported:  target type: {}, target_type),}

        if should_succeed     {}
            assert!(result.is_ok(), Conversion from {} to {} should succeed but failed: {:?}
                    source_type, target_type, result.err()")} else {}
            assert!(result.is_err(),  "succeeded,
                    source_type, target_type)}

    info!(Comprehensive:  float conversion system test completed successfully "}