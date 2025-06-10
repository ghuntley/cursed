use cursed::codegen::llvm::LlvmCodeGenerator;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_simple_module_creation() {
    // Create a context and code generator
    let _context = Context::create();
    let _module_name = "test_module";
    let _file_path = PathBuf::from("test.csd");

    let generator = LlvmCodeGenerator::new().expect("Should create code generator");

    // Verify the module exists (using the dummy module for now)
    let module = generator.get_module();
    
    // Basic test that we can create the module
    assert!(module.verify().is_ok(), "Module should verify");
}

#[test]
fn test_generator_creation() {
    // Test that we can create a code generator
    let result = LlvmCodeGenerator::new();
    assert!(result.is_ok(), "Should be able to create LlvmCodeGenerator");
}
