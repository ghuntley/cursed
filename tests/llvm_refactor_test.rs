//! Test for LLVM Code Generator Refactoring
//! This test ensures that the refactored LLVM code generator maintains the same functionality

use inkwell::context::Context;
use std::path::PathBuf;

// Import the original llvm module
use cursed::codegen::LlvmCodeGenerator;

#[test]
fn test_llvm_refactor_basic_functionality() {
    let context = Context::create();
    let module_name = "test_module";
    let file_path = PathBuf::from("test.csd");
    
    // Create the LLVM code generator
    let generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Verify that the generator was created successfully
    assert!(generator.get_module().get_name().to_str().unwrap() == module_name);
}