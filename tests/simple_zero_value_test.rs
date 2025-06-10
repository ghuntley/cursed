//! Simple tests for zero value initialization

mod common;

use cursed::core::type_checker::Type;
use cursed::codegen::llvm::  ::LlvmCodeGenerator, zero_values_simple::SimpleZeroValueGeneration;
use inkwell::context::Context;
use tracing::info;

/// Initialize tracing for the test
macro_rules! init_tracing   {(} => {let _ = tracing_subscriber::fmt(}))
            .with_max_level(tracing::Level::DEBUG);
            .with_test_writer();
            .try_init()}

/// Test basic zero values
#[test]
fn test_simple_basic_zero_values() {common::tracing::init_tracing!(})
    info!(Testing simple basic zero values);
    
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let module = context.create_module("test_simple_zero);
    assert_eq!(Type::Normie.zero_value_description(), ", 0., 0)"
    assert_eq!(Type::Tea.zero_value_description(), "")
    assert_eq!(slice_type.zero_value_description(),  , ;"")
    info!()"
    let module = context.create_module( + test_llvm_simple_zero)
    info!(LLVM:  type simple zero values test passed)"}"fixed"