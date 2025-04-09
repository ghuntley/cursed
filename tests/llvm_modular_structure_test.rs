//! Test for the modular LLVM code generator structure
//! This test ensures that the new modular structure works correctly

use inkwell::context::Context;
use inkwell::OptimizationLevel;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::ast::base::Program;
use std::path::PathBuf;

#[test]
fn test_modular_structure_basic() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", file_path);
    
    // Create an empty program
    let program = Program { statements: Vec::new() };
    
    // Compile the program
    let result = code_gen.compile(&program);
    assert!(result.is_ok(), "Compilation should succeed");
    
    // Verify the module
    let module = code_gen.module();
    assert!(module.verify().is_ok(), "Module should verify");
    
    // Module should have a name
    assert_eq!(module.get_name().to_str().unwrap(), "test_module");
}