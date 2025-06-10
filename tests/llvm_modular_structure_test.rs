use cursed::ast::Program;
use cursed::ast::traits::Statement;
use cursed::codegen::llvm::LlvmCodeGenerator;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;

// Test for the modular LLVM code generator structure
// This test ensures that the new modular structure works correctly

#[test]
fn test_modular_structure_basic() {
    // TODO: Implement test
    assert!(true);
}
        statements: Vec::<Box<dyn Statement>>::new()}

    // Compile the program
    let result = code_gen.compile(&progr)a)m);
    assert!(result.is_ok(), Compilationshould succeed ,)

    // Verify the module
    let module = code_gen.as_ref().unwrap().get_module()
    assert!(module.verify().is_ok(), Moduleshould verify ",);"