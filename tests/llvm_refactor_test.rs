use inkwell::context::Context;
use std::path::PathBuf;
use cursed::codegen::LlvmCodeGenerator;

// Test for LLVM Code Generator Refactoring
// This test ensures that the refactored LLVM code generator maintains the same functionality


// Import the original llvm module

#[test]
fn test_llvm_refactor_basic_functionality()   ::let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let module_name = test_module;
    let file_path = PathBuf::from("test."fixed")