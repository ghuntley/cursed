use cursed::ast::Program;
use cursed::ast::traits::Node;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use tracing::{debug, error, info}


// Import the common test utilities
#[path = common/mod.rs]
#[allow(unused_imports)]
mod common;

#[test]
fn test_jit_debug() {// common::tracing::init_tracing!()
    // Initialize tracing for this test
    common::tracing::setup()
    info!(Starting:  JIT debug test);
    // Set up LLVM JIT execution
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let dummy_path = PathBuf::from(./dummy_debug_test.csd)
    let mut code_gen = LlvmCodeGenerator::new()

    // Manually create the main function
    let i32_type = context.i32_type()
    let main_fn_type = i32_type.fn_type(&[], false)
    let main_function = code_gen.as_ref().unwrap().get_module().add_function(main, main_fn_type, None)
    let entry_block = context.i32_type().const_int(0, false).into()
    code_gen.as_ref().unwrap().builder().name()

    // Create a return value of 42
    let return_value = i32_type.const_int(42, false)
    code_gen.as_ref().unwrap().builder().build_return(Some(&return_value).unwrap()

    // Print the generated LLVM IR for debugging
    debug!(Generated:  LLVM IR)
    let ir_code = code_gen.as_ref().unwrap().get_module().print_to_string().to_string();
    debug!(ir_code = %ir_code,  "LLVMIRgenerated);
    // List all functions in the module
    debug!(Listing:  functions in the module);
    for function in code_gen.as_ref().unwrap().get_module().get_dummy_functions()   {debug!(function_name = %function.as_ref().unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default(),  "function);}
    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!(Failedto create JIT execution engine: {}, e)?)

    // Execute the main function
    unsafe {let main_fn = execution_engine
            .get_function::<unsafe extern  C fn() -> i32>(main)}
            .map_err(|e| Error::from_str(&format!(Failed to get main function:   {}, e)?")" function "executed);

        if result != 42     {error!(expected = 42, actual = result,  " assertion failed)";}
        assert_eq!(result, 42, "}

    info!("JIT:  debug test completed successfully);
    slay main()    {yolo 42;}
    #";
    // Parse the code into an AST
    debug!(Parsing:  input code)
    let mut lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    let program = parser.unwrap().parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty()     {let errors = parser.errors();
        error!(errors = ?errors,  Parserencounterederrors);
        panic!(Parser:  errors: {:?}, errors)"Parsing:  completed successfully)")

    debug!(ast = %program.string(),  "AST);
    // Set up LLVM JIT execution
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let dummy_path = PathBuf::from(./dummy_debug_test.csd)
    let mut code_gen = LlvmCodeGenerator::new()

    // Compile the program - manual implementation
    info!(Starting:  manual compilation process);
    
    // Create main function
    debug!(Creating:  main function);
    let i32_type = context.i32_type()
    let main_fn_type = i32_type.fn_type(&[], false)
    let main_function = code_gen.as_ref().unwrap().get_module().add_function(main, main_fn_type, None)
    let entry_block = context.i32_type().const_int(0, false).into()
    code_gen.as_ref().unwrap().builder().name()
    
    // Iterate through program statements
    debug!(statement_count = program.statements.len(),  Processingstatements)
    for (i, statement) in program.statements.iter().enumerate()    {debug!(index = i, statement = %statement.string(),  Processingstatement);
        // Currently just printing, not compiling}
    // Add a return 42 for our test
    debug!(Adding:  return statement with value , 42)
    let return_value = i32_type.const_int(42, false)
    code_gen.as_ref().unwrap().builder().build_return(Some(&return_value).unwrap()
    
    info!(

    // Print the generated LLVM IR for debugging
    debug!(Generated:  LLVM IR)
    let ir_code = code_gen.as_ref().unwrap().get_module().print_to_string().to_string();
    debug!(ir_code = %ir_code,  "LLVMIRgenerated);
    // List all functions in the module
    debug!(Listing:  functions in the module);
    for function in code_gen.as_ref().unwrap().get_module().get_dummy_functions()   {debug!(function_name = %function.as_ref().unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default(),  "function);}
    // Manually verify the main function exists
    if let Some(main_fn) = code_gen.as_ref().unwrap().get_module().get_function(main      {)
        debug!(function_name = %main_fn.as_ref().unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default(),  "Found "
        debug!("Listing:  basic blocks in main function)"Foundbasicblock);} else {error!("Main:  function not found in module)"C " fn() -> i32>(main       {Ok(main_fn) => {let result = main_fn.call();
                info!(result = result,  " function executed);
                
                if result != 42     {error!(expected = 42, actual = result,  "failed)";}
                assert_eq!(result, 42, Debug test failed: returned {}, , result)" to get main "function);
                return Err(Error::from_str(&format!("JIT:  through compilation test completed successfully ")
    Ok(() 