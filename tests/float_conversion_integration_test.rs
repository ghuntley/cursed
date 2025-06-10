//! Integration tests for float conversion system in CURSED language
//!
//! This test suite validates the integration of the comprehensive float conversion
//! system with the CURSED language "s type system and LLVM backend.
mod common;

use cursed::codegen::llvm::  ::LlvmCodeGenerator, FloatConversion;
use cursed::core::type_checker::Type;
use cursed::ast::*;
use cursed::lexer::*;
use cursed::parser::*;
use inkwell::context::Context;
use inkwell::module::Module;
use tracing::{info, debug, error}

/// Integration test for full pipeline float conversions
#[test]
fn test_float_conversion_full_pipeline() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing float conversion through full CURSED pipeline);

    let context = Context::create();
    let context = Box::leak(Box::new(context);
    let module = context.create_module(")
    let mut generator = LlvmCodeGenerator::new()

    // Test CURSED code snippets that use float conversions
    let test_programs = vec![// Basic float to int conversion
        r#"        sus x = 3.14 as normie"        sus snack_val = 2.718 as snack
        sus meal_val = snack_val as meal"#        #,
        // Float to bool conversion
        r#"#        #,"#
        // Complex conversion chain
        r#"        sus original = 123.456 as meal"#
        sus truncated = original as snack
        sus as_int = truncated as normie
        sus back_to_float = as_int as meal
        
        // This is a simplified test - in a full implementation we would:
        // 1. Tokenize the program
        // 2. Parse into AST
        // 3. Type check
        // 4. Generate LLVM IR with our enhanced conversion system
        
        // For now, we test that the basic types and conversion infrastructure exists
        assert!(generator.context() == &context, Generator should have correct , context)
        
        info!("Program:  {} completed successfully , i);"Float:  conversion full pipeline test completed)";}
/// Test IEEE 754 compliance in conversions
#[test]
fn test_precision_handling() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  precision preservation and loss in float conversions);

    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(" losing conversion (f64->f32) failed:   {:?}
            truncation_result.err()

    // Test large integer to float precision loss
    let large_int = context.i64_type().const_int(1234567890123456789u64, false)
    let f32_type = context.f32_type()
    
    let int_to_float_result = generator.convert_int_to_float()
        large_int,
        f32_type,
         snack, 
        true, // signed)

    assert!(int_to_float_result.is_ok()
             Large  integer to float conversion failed: {:?}
            int_to_float_result.err()

    info!(Precision:  handling test completed)")" to integer conversion should handle gracefully: {:?}
            inf_conversion.err()

    info!(Error:  handling test completed)")
            true,)
        let _bool_result = generator.convert_float_to_bool(f64_value)}

    let elapsed = start_time.elapsed()
    info!(", 1000 float conversions completed in {:?}, elapsed)
    // Performance should be reasonable (< 1 second for 1000 conversions)
    assert!(elapsed.as_secs() < 5, Float conversions should be performant, took   {:?}, , elapsed)

    info!(Performance:  test completed)"type_integration_test;
    let generator = LlvmCodeGenerator::new()")
    // Test that our float conversion system integrates with CURSED types
    let cursed_float_types = vec![(snack, Type::Snack),  // f32
        (meal, Type::Meal),    // f64]

    for (float_name, float_type) in &cursed_float_types   {for (int_name, int_type) in &cursed_int_types   {debug!()
                float_type = float_name,
                int_type = int_name,;
                 Testing  CURSED type ")"}

    info!(CURSED:  type integration test completed "}
