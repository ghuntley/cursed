//! Error Handling Tests for Type Conversion System
//!
//! This test suite focuses on comprehensive error handling and edge cases
//! in the type conversion system, ensuring robust error reporting and recovery.

use std::collections::HashMap;
use tracing::{info, debug, warn, error};

// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init()
            .ok();
    };
}

use cursed::codegen::llvm::{LlvmCodeGenerator, TypeConversionSystem, ConversionConfig, ConversionType};
use cursed::ast::{TypeConversionExpression, TypeAssertion, Literal, LiteralValue, Identifier};
use cursed::ast::traits::{Expression, Node};
use cursed::lexer::token::{Token, TokenType};
use cursed::core::type_checker::Type;
use cursed::error::Error;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;

/// Helper to create test LLVM context and generator
fn create_test_generator() -> (Context, LlvmCodeGenerator<'static>) {
    let context = Context::create();
    let module = context.create_module("error_test_module");
    let builder = context.create_builder();
    
    // We need to leak the context to satisfy lifetime requirements
    let leaked_context = Box::leak(Box::new(context));
    let leaked_module = Box::leak(Box::new(module));
    let leaked_builder = Box::leak(Box::new(builder));
    
    let generator = LlvmCodeGenerator::new(leaked_context, leaked_module, leaked_builder);
    
    // Return the leaked context reference and the generator
    unsafe { (std::ptr::read(leaked_context), generator) }
}

/// Helper to create type conversion expressions
fn create_conversion_expression(value: LiteralValue, target_type: &str) -> TypeConversionExpression {
    let token = Token::new(TokenType::NORMIE, "test", 1, 1);
    let literal = Box::new(Literal {
        value,
        token: token.clone(),
    });

    TypeConversionExpression {
        token,
        expression: literal,
        type_name: target_type.to_string(),
    }
}

/// Helper to create invalid expressions for error testing
fn create_invalid_expression() -> Box<dyn Expression> {
    let token = Token::new(TokenType::IDENTIFIER, "undefined_var", 1, 1);
    Box::new(Identifier {
        token,
        value: "undefined_var".to_string(),
    })
}

/// Test unknown target type errors
#[test]
fn test_unknown_target_type_errors() {
    init_tracing!();
    info!("Testing unknown target type errors");

    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    let unknown_types = vec![
        "unknown",
        "invalid_type",
        "nonexistent",
        "foo",
        "bar123",
        "",  // Empty type name
        "NORMIE", // Wrong case
        "Tea", // Wrong case
    ];

    for unknown_type in unknown_types {
        info!("Testing unknown type: '{}'", unknown_type);
        
        let conversion = create_conversion_expression(LiteralValue::Integer(42), unknown_type);
        let result = generator.compile_explicit_conversion(&conversion, &config);
        
        assert!(result.is_err(), "Conversion to unknown type '{}' should fail", unknown_type);
        
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains(unknown_type) || error_msg.contains("Unknown") || error_msg.contains("type"),
                "Error message should mention the unknown type: {}", error_msg);
    }

    info!("Unknown target type error tests completed");
}

/// Test invalid source expression errors
#[test]
fn test_invalid_source_expression_errors() {
    init_tracing!();
    info!("Testing invalid source expression errors");

    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    // Create conversion with invalid source expression
    let token = Token::new(TokenType::NORMIE, "test", 1, 1);
    let invalid_conversion = TypeConversionExpression {
        token,
        expression: create_invalid_expression(),
        type_name: "normie".to_string(),
    };

    let result = generator.compile_explicit_conversion(&invalid_conversion, &config);
    assert!(result.is_err(), "Conversion with invalid source should fail");

    let error_msg = result.unwrap_err().to_string();
    info!("Invalid source expression error: {}", error_msg);

    info!("Invalid source expression error tests completed");
}

/// Test lossy conversion restriction errors
#[test]
fn test_lossy_conversion_restriction_errors() {
    init_tracing!();
    info!("Testing lossy conversion restriction errors");

    let (context, mut generator) = create_test_generator();
    let mut config = ConversionConfig::default();
    config.allow_lossy_conversions = false; // Explicitly disable

    let lossy_conversions = vec![
        (LiteralValue::Integer(1000), "smol"),     // Large int to small int
        (LiteralValue::Integer(70000), "mid"),     // Large int to medium int
        (LiteralValue::Float(3.14159), "normie"), // Float to int
        (LiteralValue::Float(2.718), "smol"),     // Float to small int
        (LiteralValue::Float(1.23456789012345), "snack"), // Double to float
    ];

    for (value, target_type) in lossy_conversions {
        info!("Testing lossy conversion restriction: {:?} to {}", value, target_type);
        
        let conversion = create_conversion_expression(value, target_type);
        let result = generator.compile_explicit_conversion(&conversion, &config);
        
        assert!(result.is_err(), "Lossy conversion to '{}' should fail when disabled", target_type);
        
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("lossy") || error_msg.contains("Lossy") || 
                error_msg.contains("Narrowing") || error_msg.contains("not allowed"),
                "Error message should mention lossy/narrowing restriction: {}", error_msg);
    }

    info!("Lossy conversion restriction error tests completed");
}

/// Test implicit conversion disabled errors
#[test]
fn test_implicit_conversion_disabled_errors() {
    init_tracing!();
    info!("Testing implicit conversion disabled errors");

    let (context, mut generator) = create_test_generator();
    let mut config = ConversionConfig::default();
    config.allow_implicit_conversions = false;

    // Create test value
    let int_value = generator.context.i8_type().const_int(42, false).into();

    // Try implicit conversion (should fail)
    let result = generator.compile_implicit_conversion(
        int_value,
        &Type::Normie // Was Smol,
        &Type::Normie,
        &config,
    );

    assert!(result.is_err(), "Implicit conversion should fail when disabled");
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Implicit") || error_msg.contains("disabled"),
            "Error message should mention implicit conversions are disabled: {}", error_msg);

    info!("Implicit conversion disabled error tests completed");
}

/// Test conversion chain depth limit errors
#[test]
fn test_conversion_chain_depth_limit_errors() {
    init_tracing!();
    info!("Testing conversion chain depth limit errors");

    let (context, mut generator) = create_test_generator();
    let mut config = ConversionConfig::default();
    config.max_conversion_depth = 2; // Very low limit

    // Create test value
    let int_value = generator.context.i8_type().const_int(42, false).into();

    // Create chain that exceeds the limit
    let long_chain = vec![
        (Type::Normie // Was Smol, Type::Normie // Was Mid),
        (Type::Normie // Was Mid, Type::Normie),
        (Type::Normie, Type::Thicc), // This should exceed the limit
    ];

    let result = generator.apply_conversion_chain(int_value, &long_chain, &config);
    assert!(result.is_err(), "Conversion chain should fail when exceeding depth limit");

    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("deep") || error_msg.contains("depth") || error_msg.contains("chain"),
            "Error message should mention depth limit: {}", error_msg);

    info!("Conversion chain depth limit error tests completed");
}

/// Test type assertion errors
#[test]
fn test_type_assertion_errors() {
    init_tracing!();
    info!("Testing type assertion errors");

    let (context, mut generator) = create_test_generator();
    let mut config = ConversionConfig::default();
    config.enable_runtime_type_checking = false; // Disable type checking

    // Create a mock type assertion (this would normally be parsed)
    let token = Token::new(TokenType::IDENTIFIER, "test", 1, 1);
    let interface_expr = Box::new(Literal {
        value: LiteralValue::Integer(42),
        token: token.clone(),
    });
    
    let type_assertion = TypeAssertion {
        token,
        interface_expr,
        target_type: Box::new(Identifier {
            token: Token::new(TokenType::IDENTIFIER, "SomeType", 1, 1),
            value: "SomeType".to_string(),
        }),
    };

    let result = generator.compile_type_assertion(&type_assertion, &config);
    assert!(result.is_err(), "Type assertion should fail when runtime checking disabled");

    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Runtime") || error_msg.contains("type checking") || error_msg.contains("disabled"),
            "Error message should mention runtime type checking is disabled: {}", error_msg);

    info!("Type assertion error tests completed");
}

/// Test malformed configuration errors
#[test]
fn test_malformed_configuration_errors() {
    init_tracing!();
    info!("Testing malformed configuration errors");

    let (context, mut generator) = create_test_generator();

    // Test with zero max conversion depth
    let mut bad_config = ConversionConfig::default();
    bad_config.max_conversion_depth = 0;

    let int_value = generator.context.i8_type().const_int(42, false).into();
    let chain = vec![(Type::Normie // Was Smol, Type::Normie)];

    let result = generator.apply_conversion_chain(int_value, &chain, &bad_config);
    assert!(result.is_err(), "Chain application should fail with zero max depth");

    // Test with contradictory configuration
    let mut contradictory_config = ConversionConfig::default();
    contradictory_config.allow_implicit_conversions = false;
    contradictory_config.allow_lossy_conversions = true; // This combination might cause issues

    let conversion = create_conversion_expression(LiteralValue::Integer(42), "thicc");
    let result = generator.compile_explicit_conversion(&conversion, &contradictory_config);
    // This should still work for explicit conversions

    info!("Malformed configuration error tests completed");
}

/// Test memory limit and resource exhaustion scenarios
#[test]
fn test_resource_exhaustion_errors() {
    init_tracing!();
    info!("Testing resource exhaustion scenarios");

    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    // Test with extremely long conversion chains (stress test)
    let int_value = generator.context.i8_type().const_int(42, false).into();
    
    // Create a very long chain that cycles through types
    let types = vec![Type::Normie // Was Smol, Type::Normie // Was Mid, Type::Normie, Type::Thicc];
    let mut long_chain = Vec::new();
    
    for i in 0..1000 {
        let from_idx = i % types.len();
        let to_idx = (i + 1) % types.len();
        long_chain.push((types[from_idx].clone(), types[to_idx].clone()));
    }

    let result = generator.apply_conversion_chain(int_value, &long_chain, &config);
    assert!(result.is_err(), "Extremely long conversion chain should fail");

    info!("Resource exhaustion error tests completed");
}

/// Test concurrent error scenarios
#[test]
fn test_concurrent_error_scenarios() {
    init_tracing!();
    info!("Testing concurrent error scenarios");

    use std::sync::Arc;
    use std::thread;

    let num_threads = 4;
    let errors_per_thread = 100;

    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        thread::spawn(move || {
            let (context, mut generator) = create_test_generator();
            let config = ConversionConfig::default();
            
            let mut error_count = 0;
            
            for i in 0..errors_per_thread {
                // Intentionally create invalid conversions
                let invalid_type = format!("invalid_type_{}_{}", thread_id, i);
                let conversion = create_conversion_expression(LiteralValue::Integer(i as i64), &invalid_type);
                
                let result = generator.compile_explicit_conversion(&conversion, &config);
                if result.is_err() {
                    error_count += 1;
                }
            }
            
            info!("Thread {} generated {} errors", thread_id, error_count);
            error_count
        })
    }).collect();

    let mut total_errors = 0;
    for handle in handles {
        total_errors += handle.join().unwrap();
    }

    let expected_errors = num_threads * errors_per_thread;
    assert_eq!(total_errors, expected_errors, 
               "Should have generated {} errors, got {}", expected_errors, total_errors);

    info!("Concurrent error scenarios completed");
}

/// Test error message quality and informativeness
#[test]
fn test_error_message_quality() {
    init_tracing!();
    info!("Testing error message quality");

    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    let error_scenarios = vec![
        (create_conversion_expression(LiteralValue::Integer(42), "unknown"), 
         vec!["unknown", "type", "Unknown"]),
        (create_conversion_expression(LiteralValue::Integer(1000), "smol"), 
         vec!["lossy", "Lossy", "narrowing", "Narrowing", "not allowed"]),
    ];

    for (conversion, expected_keywords) in error_scenarios {
        let result = generator.compile_explicit_conversion(&conversion, &config);
        assert!(result.is_err(), "Conversion should fail");

        let error_msg = result.unwrap_err().to_string();
        info!("Error message: {}", error_msg);

        // Check that the error message contains at least one expected keyword
        let contains_keyword = expected_keywords.iter().any(|keyword| error_msg.contains(keyword));
        assert!(contains_keyword, 
                "Error message '{}' should contain one of: {:?}", 
                error_msg, expected_keywords);

        // Check that error message is not empty and reasonably informative
        assert!(!error_msg.is_empty(), "Error message should not be empty");
        assert!(error_msg.len() > 10, "Error message should be reasonably detailed");
        assert!(!error_msg.contains("TODO"), "Error message should not contain TODO placeholders");
    }

    info!("Error message quality tests completed");
}

/// Test error recovery scenarios
#[test]
fn test_error_recovery() {
    init_tracing!();
    info!("Testing error recovery scenarios");

    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    // Test that generator continues working after errors
    let failing_conversion = create_conversion_expression(LiteralValue::Integer(42), "unknown");
    let result1 = generator.compile_explicit_conversion(&failing_conversion, &config);
    assert!(result1.is_err(), "First conversion should fail");

    // Test that subsequent valid conversions still work
    let working_conversion = create_conversion_expression(LiteralValue::Integer(42), "thicc");
    let result2 = generator.compile_explicit_conversion(&working_conversion, &config);
    assert!(result2.is_ok(), "Second conversion should succeed after first failure");

    // Test multiple failures don't break the generator
    for i in 0..10 {
        let failing_conversion = create_conversion_expression(
            LiteralValue::Integer(i), 
            &format!("invalid_type_{}", i)
        );
        let result = generator.compile_explicit_conversion(&failing_conversion, &config);
        assert!(result.is_err(), "Conversion {} should fail", i);
    }

    // Test that generator still works after multiple failures
    let final_conversion = create_conversion_expression(LiteralValue::Integer(100), "thicc");
    let final_result = generator.compile_explicit_conversion(&final_conversion, &config);
    assert!(final_result.is_ok(), "Final conversion should succeed after multiple failures");

    info!("Error recovery tests completed");
}

/// Test edge cases in error handling
#[test]
fn test_error_handling_edge_cases() {
    init_tracing!();
    info!("Testing error handling edge cases");

    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    // Test with maximum integer values
    let max_int_conversion = create_conversion_expression(LiteralValue::Integer(i64::MAX), "smol");
    let result = generator.compile_explicit_conversion(&max_int_conversion, &config);
    assert!(result.is_err(), "Max int to small int should fail");

    // Test with minimum integer values
    let min_int_conversion = create_conversion_expression(LiteralValue::Integer(i64::MIN), "smol");
    let result = generator.compile_explicit_conversion(&min_int_conversion, &config);
    assert!(result.is_err(), "Min int to small int should fail");

    // Test with special float values
    let special_floats = vec![
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        0.0,
        -0.0,
    ];

    for special_float in special_floats {
        info!("Testing special float value: {}", special_float);
        let float_conversion = create_conversion_expression(LiteralValue::Float(special_float), "normie");
        // These might succeed or fail depending on implementation, but shouldn't crash
        let _ = generator.compile_explicit_conversion(&float_conversion, &config);
    }

    info!("Error handling edge cases completed");
}

/// Test error propagation through complex conversion chains
#[test]
fn test_error_propagation_in_chains() {
    init_tracing!();
    info!("Testing error propagation in conversion chains");

    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    let int_value = generator.context.i8_type().const_int(42, false).into();

    // Test chain with invalid type in the middle
    let invalid_chain = vec![
        (Type::Normie // Was Smol, Type::Normie),  // Valid
        // If we could include an invalid type here, it would test error propagation
        (Type::Normie, Type::Thicc), // Valid
    ];

    // For now, test that the chain succeeds when all types are valid
    let result = generator.apply_conversion_chain(int_value, &invalid_chain, &config);
    assert!(result.is_ok(), "Valid conversion chain should succeed");

    // Test chain where later conversions might fail due to configuration
    let mut restrictive_config = config.clone();
    restrictive_config.max_conversion_depth = 1;

    let long_chain = vec![
        (Type::Normie // Was Smol, Type::Normie),
        (Type::Normie, Type::Thicc),
    ];

    let result = generator.apply_conversion_chain(int_value, &long_chain, &restrictive_config);
    assert!(result.is_err(), "Chain should fail when exceeding depth limit");

    info!("Error propagation tests completed");
}
