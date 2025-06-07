//! Test utility functions for creating test code generators

use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;
use std::path::PathBuf;

/// Create a test code generator for use in tests
pub fn create_test_code_generator() -> LlvmCodeGenerator<'static> {
    let context = Box::leak(Box::new(Context::create()));
    LlvmCodeGenerator::new(context, "test_module", PathBuf::from("test.csd"))
}
