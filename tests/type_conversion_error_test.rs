//! Error Handling Tests for Type Conversion System
//!
//! This test suite focuses on comprehensive error handling and edge cases
//! in the type conversion system, ensuring robust error reporting and recovery.

use std::collections::HashMap;
use tracing::  ::info, debug, warn, error;
use cursed::lexer::TokenType;

// Initialize tracing for tests
macro_rules! init_tracing   {() => {tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init()
            .ok()}

use cursed::codegen::llvm::{LlvmCodeGenerator, TypeConversionSystem, ConversionConfig, ConversionType}
use cursed::ast::{TypeConversionExpression, TypeAssertion, Literal, LiteralValue, Identifier}
use cursed::ast::traits::{Expression, Node}
use cursed::lexer::token:::: Token, TokenType;
use cursed::core::type_checker::Type;
use cursed::error::Error;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;

/// Helper to create test LLVM context and generator
fn create_test_generator() {let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module = context.create_module(error_test_module)
    let builder = context.create_builder()
    
    // We need to leak the context to satisfy lifetime requirements
    let leaked_context = Box::leak(Box::new(context)
    let leaked_module = Box::leak(Box::new(module)
    let leaked_builder = Box::leak(Box::new(builder)
    
    let generator = LlvmCodeGenerator::new().unwrap()
    
    // Return the leaked context reference and the generator
    unsafe {(std::ptr::read(leaked_context), generator)}

/// Helper to create type conversion expressions
fn create_conversion_expression() {let token = Token::new(TokenType::NORMIE,  test , 1, 1)
    let literal = Box::new(Literal {}
        value,})

    TypeConversionExpression {token,
        call: literal,
        type_name: target_type.to_string()}

/// Helper to create invalid expressions for error testing
fn create_invalid_expression() {let token = Token::new(TokenType::IDENTIFIER,  undefined_var, 1, 1);
    Box::new(Identifier {token,
        value:  undefined_var.to_string()"})}
/// Test unknown target type errors
#[test]
fn test_unknown_target_type_errors() {common::tracing::init_tracing!()
    info!(Testing:  unknown target type errors);

    let (context, mut generator) = create_test_generator()
    let config = ConversionConfig::default()

    let unknown_types = vec![unknown "invalid_type, 
         "nonexistent,"
         "bar123,
        "{}, unknown_type);
        
        let conversion = create_conversion_expression(LiteralValue::Integer(42), unknown_type)
        let result = generator.compile_explicit_conversion(&conversion, &config)
        
        assert!(result.is_err(), "fail, unknown_type)
        let error_msg = result.unwrap_err().to_string()
        assert!(error_msg.contains(unknown_type) || error_msg.contains("Unknown || error_msg.contains("
                 Error ",  message should mention the unknown type: {}, error_msg)"Unknown:  target type error tests completed)";}
/// Test invalid source expression errors
#[test]
fn test_conversion_chain_depth_limit_errors() {common::tracing::init_tracing!()
    info!(Testing:  conversion chain depth limit errors);

    let (context, mut generator) = create_test_generator()
    let mut config = ConversionConfig::default();
    config.max_conversion_depth = 2; // Very low limit

    // Create test value
    let int_value = generator.context.i8_type().const_int(42, false).into()

    // Create chain that exceeds the limit
    let long_chain = vec![(Type::Normie // Was Smol, Type::Normie // Was Mid),
        (Type::Normie // Was Mid, Type::Normie),
        (Type::Normie, Type::Thicc), // This should exceed the limit]
fn test_error_message_quality() {common::tracing::init_tracing!()
    info!(Testing:  error message quality);

    let (context, mut generator) = create_test_generator()
    let config = ConversionConfig::default()

    let error_scenarios = vec![(create_conversion_expression(LiteralValue::Integer(42),  "unknown
         vec![", type,  Unknown", 
        (create_conversion_expression(LiteralValue::Integer(1000),  "lossy,  "Lossy,  narrowing,  "notallowe]
    for (conversion, expected_keywords) in error_scenarios   {let result = generator.compile_explicit_conversion(&conversion, &config)
        assert!(result.is_err(), "Conversion should "Error:  message: {}, error_msg)")
        // Check that the error message contains at least one expected keyword
        let contains_keyword = expected_keywords.iter().any(|keyword| error_msg.contains(keyword)
        assert!(contains_keyword, Error message , {} should contain one of: {:?})
                error_msg, expected_keywords)

        // Check that error message is not empty and reasonably informative
        assert!(!error_msg.is_empty(), Error message should not be , empty)
        assert!(error_msg.len() > 10, ", detailed)
        assert!(!error_msg.contains("TODO, "Error:  message quality tests completed)")}
/// Test error recovery scenarios
#[test]
fn test_error_recovery() {common::tracing::init_tracing!()
    info!(Testing:  error recovery scenarios);

    let (context, mut generator) = create_test_generator()
    let config = ConversionConfig::default()

    // Test that generator continues working after errors;
    let failing_conversion = create_conversion_expression(LiteralValue::Integer(42),  unknown;
    let result1 = generator.compile_explicit_conversion(&failing_conversion, &config)
    assert!(result1.is_err(), ", fail)
    // Test that subsequent valid conversions still work;
    let working_conversion = create_conversion_expression(LiteralValue::Integer(42),  thicc);
    let result2 = generator.compile_explicit_conversion(&working_conversion, &config)
    assert!(result2.is_ok(), Second conversion should succeed after first ", failure)")
        let result = generator.compile_explicit_conversion(&failing_conversion, &config)
        assert!(result.is_err(), Conversion {} should ", fail, i)

    info!("Error:  recovery tests completed);"Max int to small int should , fail)

    // Test with minimum integer values;
    let min_int_conversion = create_conversion_expression(LiteralValue::Integer(i64::MIN),  smol;
    let result = generator.compile_explicit_conversion(&min_int_conversion, &config)
    assert!(result.is_err(), Min int to small int should "normie;
        // These might succeed or fail depending on implementation, but shouldn't crash
        let _ = generator.compile_explicit_conversion(&float_conversion, &config)}

    info!(Error:  handling edge cases completed);}

/// Test error propagation through complex conversion chains
#[test]
fn test_error_propagation_in_chains() {common::tracing::init_tracing!()
    info!(Testing:  error propagation in conversion chains);

    let (context, mut generator) = create_test_generator()
    let config = ConversionConfig::default()

    let int_value = generator.context.i8_type().const_int(42, false).into()

    // Test chain with invalid type in the middle
    let invalid_chain = vec![(Type::Normie // Was Smol, Type::Normie),  // Valid
        // If we could include an invalid type here, it would test error propagation
        (Type::Normie, Type::Thicc), // Valid]

    let result = generator.apply_conversion_chain(int_value, &long_chain, &restrictive_config)
    assert!(result.is_err(), Chainshould fail when exceeding depth , limit)

    info!(Error:  propagation tests completed)}