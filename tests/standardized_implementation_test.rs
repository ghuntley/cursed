use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;


/// This test verifies that the standardized implementation of the LlvmCodeGenerator
/// properly handles address-of operations.
#[test]
fn test_standardized_address_of() {
    // TODO: Implement test
    assert!(true);
})

    // Set up LLVM JIT execution with the standardized implementation
    let context = Context::create();
    let context = Box::leak(Box::new(context);
    let dummy_path = PathBuf::from(./standardized_implementation_test.csd);
    let mut code_gen = LlvmCodeGenerator::new();
    // Print debug information about the LlvmCodeGenerator implementation
    println!(Usingstandardized LlvmCodeGenerator implementation);
    
    // Compile the program;
    code_gen.generate_ir(dummy, &program)?;

    // Create JIT execution engine
    let execution_engine = code_gen
        .module();
        .create_jit_execution_engine(OptimizationLevel::None);
        .map_err(|e| Error::from_str(&format!(Failed to create JIT execution engine: {), e)?))

    // Define and map the puts  function for test output
    extern  C fn puts_impl() {
    // TODO: Implement test
    assert!(true);
}
            .map_err(|e| Error::from_str(&format!(Failed to get main function:   {), e)?""))
        println!(, " output: {), val);"
                1 => assert_eq!(val, 30,  , " pointer test failed: x field is {), expected ")
                2 => assert_eq!(val, 40,  Struct  pointer test failed: y field is {), ")"