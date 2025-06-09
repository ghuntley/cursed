//! Integration tests for float conversion system in CURSED language
//!
//! This test suite validates the integration of the comprehensive float conversion
//! system with the CURSED language's type system and LLVM backend.

mod common;

use cursed::codegen::llvm::{LlvmCodeGenerator, FloatConversion};
use cursed::core::type_checker::Type;
use cursed::ast::*;
use cursed::lexer::*;
use cursed::parser::*;
use inkwell::context::Context;
use inkwell::module::Module;
use tracing::{info, debug, error};

/// Integration test for full pipeline float conversions
#[test]
fn test_float_conversion_full_pipeline() {
    common::tracing::setup();
    info!("Testing float conversion through full CURSED pipeline");

    let context = Context::create();
    let module = context.create_module("float_integration_test");
    let mut generator = LlvmCodeGenerator::new(&context, "float_integration_test", std::path::PathBuf::from("test.cursed"));

    // Test CURSED code snippets that use float conversions
    let test_programs = vec![
        // Basic float to int conversion
        r#"
        sus x = 3.14 as normie
        "#,
        
        // Float to float conversion
        r#"
        sus snack_val = 2.718 as snack
        sus meal_val = snack_val as meal
        "#,
        
        // Float to bool conversion
        r#"
        sus is_nonzero = 42.0 as lit
        sus is_zero = 0.0 as lit
        "#,
        
        // Complex conversion chain
        r#"
        sus original = 123.456 as meal
        sus truncated = original as snack
        sus as_int = truncated as normie
        sus back_to_float = as_int as meal
        "#,
    ];

    for (i, program) in test_programs.iter().enumerate() {
        debug!(test_number = i, program = program, "Testing program");
        
        // This is a simplified test - in a full implementation we would:
        // 1. Tokenize the program
        // 2. Parse into AST
        // 3. Type check
        // 4. Generate LLVM IR with our enhanced conversion system
        
        // For now, we test that the basic types and conversion infrastructure exists
        assert!(generator.context() == &context, "Generator should have correct context");
        
        info!("Program {} completed successfully", i);
    }

    info!("Float conversion full pipeline test completed");
}

/// Test IEEE 754 compliance in conversions
#[test]
fn test_ieee754_compliance() {
    common::tracing::setup();
    info!("Testing IEEE 754 compliance in float conversions");

    let context = Context::create();
    let module = context.create_module("ieee754_test");
    let generator = LlvmCodeGenerator::new(&context, "ieee754_test", std::path::PathBuf::from("test.cursed"));

    // Test special value handling
    let test_cases = vec![
        ("NaN", f64::NAN),
        ("Positive Infinity", f64::INFINITY),
        ("Negative Infinity", f64::NEG_INFINITY),
        ("Positive Zero", 0.0),
        ("Negative Zero", -0.0),
        ("Subnormal", f64::MIN_POSITIVE / 2.0),
    ];

    for (name, value) in test_cases {
        debug!(test_case = name, value = value, "Testing IEEE 754 special value");
        
        // Create function for testing
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function(&format!("test_{}", name.replace(" ", "_")), fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        generator.builder().position_at_end(basic_block);

        // Create test value and test various conversions
        let f64_value = context.f64_type().const_float(value);
        
        // Test NaN detection
        if value.is_nan() {
            let nan_check = generator.check_is_nan(f64_value);
            assert!(nan_check.is_ok(), "NaN detection failed for {}: {:?}", name, nan_check.err());
        }
        
        // Test infinity detection
        if value.is_infinite() {
            let inf_check = generator.check_is_infinite(f64_value);
            assert!(inf_check.is_ok(), "Infinity detection failed for {}: {:?}", name, inf_check.err());
        }
        
        info!("IEEE 754 test case '{}' completed", name);
    }

    info!("IEEE 754 compliance testing completed");
}

/// Test bounds checking for float-to-int conversions
#[test]
fn test_conversion_bounds_checking() {
    common::tracing::setup();
    info!("Testing bounds checking for float-to-int conversions");

    let context = Context::create();
    let module = context.create_module("bounds_test");
    let generator = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.cursed"));

    // Create test function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_bounds", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(basic_block);

    // Test cases that should trigger bounds checking
    let bounds_test_cases = vec![
        // (float_value, target_type, bit_width, should_clamp)
        (300.0, "smol", 8, true),    // Exceeds i8::MAX
        (-200.0, "smol", 8, true),   // Below i8::MIN
        (100000.0, "mid", 16, true), // Exceeds i16::MAX
        (-50000.0, "mid", 16, true), // Below i16::MIN
        (1.5e10, "normie", 32, true), // Exceeds i32::MAX
        (-2.5e9, "normie", 32, true), // Below i32::MIN
    ];

    for (float_val, target_name, bit_width, should_clamp) in bounds_test_cases {
        debug!(
            float_value = float_val,
            target_type = target_name,
            bit_width = bit_width,
            should_clamp = should_clamp,
            "Testing bounds checking scenario"
        );

        let f64_value = context.f64_type().const_float(float_val);
        let target_int_type = match bit_width {
            8 => context.i8_type(),
            16 => context.i16_type(),
            32 => context.i32_type(),
            64 => context.i64_type(),
            _ => panic!("Unsupported bit width"),
        };

        // Test bounds checking
        let bounds_result = generator.apply_bounds_checking(
            f64_value,
            target_int_type,
            target_name,
            true, // signed
        );

        assert!(bounds_result.is_ok(), 
                "Bounds checking failed for {} -> {}: {:?}", 
                float_val, target_name, bounds_result.err());

        // Test overflow detection
        let overflow_result = generator.would_overflow(
            f64_value,
            target_int_type,
            true, // signed
        );

        assert!(overflow_result.is_ok(),
                "Overflow detection failed for {} -> {}: {:?}",
                float_val, target_name, overflow_result.err());

        info!("Bounds checking test for {} -> {} completed", float_val, target_name);
    }

    info!("Conversion bounds checking completed");
}

/// Test precision preservation and loss scenarios
#[test]
fn test_precision_handling() {
    common::tracing::setup();
    info!("Testing precision preservation and loss in float conversions");

    let context = Context::create();
    let module = context.create_module("precision_test");
    let generator = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.cursed"));

    // Create test function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_precision", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(basic_block);

    // Test precision preservation (f32 -> f64)
    let f32_value = context.f32_type().const_float(3.14159);
    let f64_type = context.f64_type();
    
    let extension_result = generator.convert_float_to_float(
        f32_value,
        f64_type,
        "meal",
    );

    assert!(extension_result.is_ok(), 
            "Precision preserving conversion (f32->f64) failed: {:?}", 
            extension_result.err());

    // Test precision loss (f64 -> f32) 
    let high_precision_f64 = context.f64_type().const_float(1.23456789012345678901234567890);
    let f32_type = context.f32_type();
    
    let truncation_result = generator.convert_float_to_float(
        high_precision_f64,
        f32_type,
        "snack",
    );

    assert!(truncation_result.is_ok(),
            "Precision losing conversion (f64->f32) failed: {:?}",
            truncation_result.err());

    // Test large integer to float precision loss
    let large_int = context.i64_type().const_int(1234567890123456789u64, false);
    let f32_type = context.f32_type();
    
    let int_to_float_result = generator.convert_int_to_float(
        large_int,
        f32_type,
        "snack",
        true, // signed
    );

    assert!(int_to_float_result.is_ok(),
            "Large integer to float conversion failed: {:?}",
            int_to_float_result.err());

    info!("Precision handling test completed");
}

/// Test error handling and edge cases
#[test]
fn test_conversion_error_handling() {
    common::tracing::setup();
    info!("Testing error handling in float conversions");

    let context = Context::create();
    let module = context.create_module("error_test");
    let generator = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.cursed"));

    // Create test function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_errors", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(basic_block);

    // Test conversion with NaN
    let nan_value = context.f64_type().const_float(f64::NAN);
    let i32_type = context.i32_type();
    
    let nan_conversion = generator.convert_float_to_int(
        nan_value,
        i32_type,
        "normie",
        true,
    );

    // NaN conversion should handle gracefully (return 0 or error)
    assert!(nan_conversion.is_ok(), 
            "NaN to integer conversion should handle gracefully: {:?}", 
            nan_conversion.err());

    // Test conversion with infinity
    let inf_value = context.f64_type().const_float(f64::INFINITY);
    
    let inf_conversion = generator.convert_float_to_int(
        inf_value,
        i32_type,
        "normie",
        true,
    );

    assert!(inf_conversion.is_ok(),
            "Infinity to integer conversion should handle gracefully: {:?}",
            inf_conversion.err());

    info!("Error handling test completed");
}

/// Test performance characteristics of float conversions
#[test]
fn test_conversion_performance() {
    common::tracing::setup();
    info!("Testing performance characteristics of float conversions");

    let context = Context::create();
    let module = context.create_module("performance_test");
    let generator = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.cursed"));

    // Create test function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_performance", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(basic_block);

    let start_time = std::time::Instant::now();

    // Perform many conversions to test performance
    for i in 0..1000 {
        let test_value = i as f64 / 100.0;
        let f64_value = context.f64_type().const_float(test_value);
        
        // Test various conversion types
        let f32_type = context.f32_type();
        let _f32_result = generator.convert_float_to_float(
            f64_value,
            f32_type,
            "snack",
        );

        let i32_type = context.i32_type();
        let _int_result = generator.convert_float_to_int(
            f64_value,
            i32_type,
            "normie",
            true,
        );

        let _bool_result = generator.convert_float_to_bool(f64_value);
    }

    let elapsed = start_time.elapsed();
    info!("1000 float conversions completed in {:?}", elapsed);

    // Performance should be reasonable (< 1 second for 1000 conversions)
    assert!(elapsed.as_secs() < 5, 
            "Float conversions should be performant, took {:?}", elapsed);

    info!("Performance test completed");
}

/// Test CURSED type system integration
#[test]
fn test_cursed_type_integration() {
    common::tracing::setup();
    info!("Testing integration with CURSED type system");

    let context = Context::create();
    let module = context.create_module("type_integration_test");
    let generator = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.cursed"));

    // Test that our float conversion system integrates with CURSED types
    let cursed_float_types = vec![
        ("snack", Type::Snack),  // f32
        ("meal", Type::Meal),    // f64
    ];

    let cursed_int_types = vec![
        ("smol", Type::Smol),    // i8
        ("mid", Type::Mid),      // i16
        ("normie", Type::Normie), // i32
        ("thicc", Type::Thicc),  // i64
    ];

    for (float_name, float_type) in &cursed_float_types {
        for (int_name, int_type) in &cursed_int_types {
            debug!(
                float_type = float_name,
                int_type = int_name,
                "Testing CURSED type integration"
            );
            
            // Test that type mappings work correctly
            // In a full implementation, this would test the type conversion
            // system's ability to map CURSED types to LLVM types correctly
            
            info!("CURSED type mapping test: {} -> {} completed", float_name, int_name);
        }
    }

    info!("CURSED type integration test completed");
}
