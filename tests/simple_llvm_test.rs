use cursed::codegen::llvm::LlvmCodeGenerator;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_simple_module_creation() {
    // Test that we can create a code generator
    let result = LlvmCodeGenerator::new();
    assert!(result.is_ok(), "Should be able to create LlvmCodeGenerator");
}