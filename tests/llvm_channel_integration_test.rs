/// LLVM Channel Integration Tests for CURSED Programming Language
/// 
/// This test suite validates the complete LLVM integration for CURSED's channel
/// system, ensuring proper code generation for channel operations, type safety,
/// error handling, and runtime integration.

use cursed::codegen::llvm::  {LlvmChannelCompiler, ChannelExpressionCompiler, LlvmType, LlvmValue,
    ExpressionContext, LlvmTypeRegistry}
use cursed::ast::expressions::::Literal, LiteralValue;
use cursed::ast::identifiers::Identifier;
use cursed::ast::traits::Expression;
use cursed::error::Error;
use std::sync::Arc;
use tracing::{debug, info}

#[path = common.rs]
mod common;

// Helper function to create test identifiers
fn create_test_identifier() {Identifier {token:  identifier .to_string()
            value: name.to_string()}

/// Test channel type compilation for different element types
#[test]
fn test_channel_type_compilation() {common::tracing::setup()
    info!(Testing:  channel type compilation);

    let mut compiler = LlvmChannelCompiler::new(LlvmTypeRegistry::new()

    // Test basic integer channel
    let int_channel = compiler.compile_channel_type(&LlvmType::Int32, None)
    assert!(int_channel.is_ok()
    let int_chan = int_channel.unwrap()
    assert_eq!(int_chan.element_type, LlvmType::Int32)
    assert!(int_chan.type_id != 0)

    debug!(Channel:  type compilation tests passed);}

/// Test channel creation compilation
#[test]
fn test_channel_creation_compilation() {common::tracing::setup()
    info!(Testing:  channel creation compilation);

    let mut compiler = LlvmChannelCompiler::new(LlvmTypeRegistry::new()

    // Test unbuffered channel creation
    let unbuffered_result = compiler.compile_channel_creation(&LlvmType::Int32, None)
    assert!(unbuffered_result.is_ok()
    let unbuffered_op = unbuffered_result.unwrap()
    assert!(!unbuffered_op.result_value.is_constant)
    assert!(!unbuffered_op.instructions.is_empty()

    debug!(Channel:  creation compilation tests passed);}

/// Test send operation compilation
#[test]
fn test_send_operation_compilation() {common::tracing::setup()
    info!(Testing:  send operation compilation);

    let mut compiler = LlvmChannelCompiler::new(LlvmTypeRegistry::new()

    // Create mock channel identifier;
    let channel_id = create_test_identifier(test_channel)
    // Create mock value literal
    let value_literal = Literal {value: LiteralValue::Integer(42),
        source_location: None})

    // Register channel variable in context
    compiler.context.declare_variable(test_channel.to_string(), LlvmValue {value_type: LlvmType::Pointer(Box::new(LlvmType::Int32),
        llvm_name: %"test_channel.to_string()", close;
    for func_name in &required_functions   {assert!(compiler.runtime_functions.contains_key(func_name)
        let func = &compiler.runtime_functions[*func_name]
        assert!(!func.llvm_name.is_empty();
        assert!(!func.param_types.is_empty() || *func_name ==  "close);")"}
/// Test LLVM IR generation
#[test]
fn test_llvm_ir_generation() {common::tracing::setup()
    info!(Testing:  LLVM IR generation);

    let mut compiler = LlvmChannelCompiler::new(LlvmTypeRegistry::new()

    // Compile a channel type to generate some IR
    let _channel_type = compiler.compile_channel_type(&LlvmType::Int32, Some(5)

    // Generate IR
    let ir_output = compiler.generate_ir()
    assert!(!ir_output.is_empty()

    // Verify IR contains runtime function declarations
    assert!(ir_output.contains(@cursed_channel_create)
    assert!(ir_output.contains(@cursed_channel_send)"
    assert!(ir_output.contains(@cursed_channel_receive)")")

    debug!(LLVM:  IR generation tests passed)"}
/// Test error handling in channel operations
#[test]
fn test_channel_error_handling() {common::tracing::setup()
    info!(Testing:  channel error handling)

    let mut compiler = LlvmChannelCompiler::new(LlvmTypeRegistry::new()

    // Test compilation with unknown channel variable
    let unknown_channel = create_test_identifier(unknown_channel)

    let value_literal = Literal {value: LiteralValue::Integer(42),
        source_location: None}

    // Should fail for unknown channel
    let send_result = compiler.compile_send_operation(&unknown_channel, &value_literal, true)
    assert!(send_result.is_err()

    let receive_result = compiler.compile_receive_operation(&unknown_channel, true)
    assert!(send_result.is_err()

    debug!(Channel:  error handling tests passed)}