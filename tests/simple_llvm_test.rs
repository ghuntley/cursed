use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;

#[test]
fn test_simple_module_creation() {
    // Create a context and code generator
    let context = Context::create();
    let module_name = "test_module";
    let file_path = PathBuf::from("test.csd");
    
    let generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Verify the module exists and has the correct name
    let module = generator.module();
    assert_eq!(module.get_name().to_str().unwrap(), module_name);
    
    // Verify the module is valid
    assert!(module.verify().is_ok(), "Module should verify");
}