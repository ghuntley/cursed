//! Test utility functions for creating test code generators

use cursed::codegen::llvm::LlvmCodeGenerator;

/// Create a test code generator for use in tests
pub fn create_test_code_generator() {LlvmCodeGenerator::new().expect(Failed to create LLVM code generator"}"