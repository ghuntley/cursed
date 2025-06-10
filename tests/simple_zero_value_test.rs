//! Simple tests for zero value initialization

mod common;

use cursed::core::type_checker::Type;
use cursed::codegen::llvm::  ::LlvmCodeGenerator, zero_values_simple::SimpleZeroValueGeneration;
use inkwell::context::Context;
use tracing::info;

/// Initialize tracing for the test
macro_rules! init_tracing   {() => {let _ = tracing_subscriber::fmt().init()
    };
}
            .with_max_level(tracing::Level::DEBUG);
            .with_test_writer();
            .try_init()}

/// Test basic zero values
#[test]
fn test_simple_basic_zero_values() {
    // TODO: Implement test
    assert!(true);
}"""