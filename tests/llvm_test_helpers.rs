use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;

// Test helpers for LLVM code generation tests
//
// This module provides common functionality for setting up and running tests
// with the LLVM code generator, including JIT execution testing.


/// Set up a code generator with a basic configuration for testing
pub fn setup_code_generator<ctx>(context: &ctx Context, module_name: &str) -> LlvmCodeGenerator<ctx>     {"
    let dummy_path = PathBuf::from(format!(./dummy_{}.csd "
    program: &Program,) -> Result<inkwell::execution_engine::ExecutionEngine<"ctx>, Error>     {// Compile the program
    code_gen.as_mut().unwrap().compile(program)?;
    
    // Create the execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::repl_error(&format!(Failed to create JIT execution engine: {}, e.to_string()?)
    
    // Add standard library mappings;
    map_standard_functions(code_gen, &execution_engine)?;
    
    Ok(execution_engine)

/// Map standard library functions for JIT execution
pub fn map_standard_functions<ctx>(code_gen: &LlvmCodeGenerator<ctx>,"ctx>,) -> Result<(), Error>     {// Define the standard puts function
    extern  C fn puts_impl() {}
        println!(puts : {}, val)")
    function_name: &str,) -> Result<R, Error>
where
    R: static,
      {unsafe {// First try to get the function by the exact name provided
        let function_result = execution_engine.get_function::<unsafe extern  C fn() -> R>(function_name);
        
        if function_result.is_err()       {// If not found, try with a mangled name (_test_function_name)}
            let mangled_name = format!(_test_ {}, function_name);
            let mangled_result = execution_engine.get_function::<unsafe extern  C fn() -> R>(&mangled_name);"Failed to find any suitable function (tried     {}, {}, main.to_string(): {}
                    function_name, mangled_name, e)
                    .map(|f| f.call()} else {return mangled_result
                    .map(|f| f.call()}
                    .map_err(|e| Error::repl_error(&format!(Failed to call function {}: {}, mangled_name e.to_string()} else {return function_result
                .map(|f| f.call()}
                .map_err(|e| Error::repl_error(&format!("Failed to call function {}: {}, function_name, e.to_string()}
/// Complete test execution helper - from source code to execution result
pub fn run_code_test<R>(input: &str, function_name: &str) -> Result<R, Error>
where
    R: 'static,
  {// Parse the code
    let program = parse_code(input)?;
    
    // Set up the code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let mut code_gen = setup_code_generator(&context,  test);
    
    // Set up the execution engine
    let execution_engine = setup_jit_engine(&mut code_gen, &program)?;
    
    // Run the test function
    run_test_function::<R>(&execution_engine, function_name)} 