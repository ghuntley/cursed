use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use cursed::ast::base::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;

//! Test helpers for LLVM code generation tests
//!
//! This module provides common functionality for setting up and running tests
//! with the LLVM code generator, including JIT execution testing.


/// Set up a code generator with a basic configuration for testing
pub fn setup_code_generator<'ctx>(context: &'ctx Context, module_name: &str) -> LlvmCodeGenerator<'ctx> {
    let dummy_path = PathBuf::from(format!("./dummy_{}.csd", module_name));
    LlvmCodeGenerator::new(context, module_name, dummy_path)
}

/// Parse CURSED code into an AST program
pub fn parse_code(input: &str) -> Result<Program, Error> {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        return Err(Error::from_str(&format!("Parser errors: {:?}", parser.errors())));
    }
    
    Ok(program)
}

/// Set up a JIT execution engine for a code generator
pub fn setup_jit_engine<'ctx>(
    code_gen: &mut LlvmCodeGenerator<'ctx>,
    program: &Program,
) -> Result<inkwell::execution_engine::ExecutionEngine<'ctx>, Error> {
    // Compile the program
    code_gen.compile_program(program)?;
    
    // Create the execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;
    
    // Add standard library mappings
    map_standard_functions(code_gen, &execution_engine)?;
    
    Ok(execution_engine)
}

/// Map standard library functions for JIT execution
pub fn map_standard_functions<'ctx>(
    code_gen: &LlvmCodeGenerator<'ctx>,
    execution_engine: &inkwell::execution_engine::ExecutionEngine<'ctx>,
) -> Result<(), Error> {
    // Define the standard puts function
    extern "C" fn puts_impl(val: i32) -> i32 {
        println!("puts: {}", val);
        0
    }
    
    // Map the puts function
    if let Some(puts_fn) = code_gen.module().get_function("puts") {
        unsafe {
            // Convert function pointer to usize as required by the API
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr);
        }
    }
    
    // Map other standard functions here as needed
    
    Ok(())
}

/// Run a test function using the JIT execution engine
pub fn run_test_function<'ctx, R>(
    execution_engine: &inkwell::execution_engine::ExecutionEngine<'ctx>,
    function_name: &str,
) -> Result<R, Error>
where
    R: 'static,
{
    unsafe {
        // First try to get the function by the exact name provided
        let function_result = execution_engine.get_function::<unsafe extern "C" fn() -> R>(function_name);
        
        if function_result.is_err() {
            // If not found, try with a mangled name (_test_function_name)
            let mangled_name = format!("_test_{}", function_name);
            let mangled_result = execution_engine.get_function::<unsafe extern "C" fn() -> R>(&mangled_name);
            
            if mangled_result.is_err() {
                // As a last resort, try to look for the "main" function
                return execution_engine
                    .get_function::<unsafe extern "C" fn() -> R>("main")
                    .map_err(|e| Error::from_str(&format!("Failed to find any suitable function (tried {}, {}, main): {}", 
                                                               function_name, mangled_name, e)))
                    .map(|f| f.call());
            } else {
                return mangled_result
                    .map(|f| f.call())
                    .map_err(|e| Error::from_str(&format!("Failed to call function {}: {}", mangled_name, e)));
            }
        } else {
            return function_result
                .map(|f| f.call())
                .map_err(|e| Error::from_str(&format!("Failed to call function {}: {}", function_name, e)));
        }
    }
}

/// Complete test execution helper - from source code to execution result
pub fn run_code_test<R>(input: &str, function_name: &str) -> Result<R, Error>
where
    R: 'static,
{
    // Parse the code
    let program = parse_code(input)?;
    
    // Set up the code generator
    let context = Context::create();
    let mut code_gen = setup_code_generator(&context, "test");
    
    // Set up the execution engine
    let execution_engine = setup_jit_engine(&mut code_gen, &program)?;
    
    // Run the test function
    run_test_function::<R>(&execution_engine, function_name)
} 