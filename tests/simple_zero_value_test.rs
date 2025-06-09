//! Simple tests for zero value initialization

mod common;

use cursed::core::type_checker::Type;
use cursed::codegen::llvm::{LlvmCodeGenerator, zero_values_simple::SimpleZeroValueGeneration};
use inkwell::context::Context;
use tracing::info;

/// Initialize tracing for the test
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init();
    };
}

/// Test basic zero values
#[test]
fn test_simple_basic_zero_values() {
    init_tracing!();
    info!("Testing simple basic zero values");
    
    let context = Context::create();
    let module = context.create_module("test_simple_zero");
    let builder = context.create_builder();
    let codegen = LlvmCodeGenerator::new(&context, module, builder);
    
    // Test basic types
    assert!(codegen.create_simple_zero_value(&Type::Lit).is_ok());
    assert!(codegen.create_simple_zero_value(&Type::Normie).is_ok());
    assert!(codegen.create_simple_zero_value(&Type::Snack).is_ok());
    assert!(codegen.create_simple_zero_value(&Type::Tea).is_ok());
    
    info!("Simple basic zero values test passed");
}

/// Test Type zero value helpers
#[test]
fn test_type_zero_value_helpers() {
    init_tracing!();
    info!("Testing Type zero value helper methods");
    
    // Test has_zero_value
    assert!(Type::Lit.has_zero_value());
    assert!(Type::Normie.has_zero_value());
    assert!(Type::Tea.has_zero_value());
    assert!(!Type::Unknown.has_zero_value());
    
    // Test zero_value_description
    assert_eq!(Type::Lit.zero_value_description(), "false");
    assert_eq!(Type::Normie.zero_value_description(), "0");
    assert_eq!(Type::Snack.zero_value_description(), "0.0");
    assert_eq!(Type::Tea.zero_value_description(), "\"\"");
    
    let slice_type = Type::Slice(Box::new(Type::Normie));
    assert_eq!(slice_type.zero_value_description(), "nil");
    
    info!("Type zero value helpers test passed");
}

/// Test LLVM type zero values
#[test]
fn test_llvm_type_simple_zero_values() {
    init_tracing!();
    info!("Testing LLVM type simple zero values");
    
    let context = Context::create();
    let module = context.create_module("test_llvm_simple_zero");
    let builder = context.create_builder();
    let codegen = LlvmCodeGenerator::new(&context, module, builder);
    
    // Test basic LLVM types
    let i32_zero = codegen.create_simple_zero_value_for_llvm_type(context.i32_type().into());
    assert!(i32_zero.is_int_value());
    
    let f64_zero = codegen.create_simple_zero_value_for_llvm_type(context.f64_type().into());
    assert!(f64_zero.is_float_value());
    
    let ptr_zero = codegen.create_simple_zero_value_for_llvm_type(
        context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()
    );
    assert!(ptr_zero.is_pointer_value());
    
    info!("LLVM type simple zero values test passed");
}
