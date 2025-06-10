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
fn test_jit_debug() {// common::tracing::init_tracing!(})
    // Initialize tracing for this test
    common::tracing::setup();
    info!(Starting:  JIT debug test);
    // Set up LLVM JIT execution
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let dummy_path = PathBuf::from(./dummy_debug_test.csd);
    let mut code_gen = LlvmCodeGenerator::new();
    // Manually create the main function
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = code_gen.as_ref().unwrap().get_module().add_function(main, main_fn_type, None);
    let entry_block = context.i32_type().const_int(0, false).into();
    code_gen.as_ref().unwrap().builder().name();
    // Create a return value of 42
    let return_value = i32_type.const_int(42, false);
    code_gen.as_ref().unwrap().builder().build_return(Some(&return_value).unwrap();)
    // Print the generated LLVM IR for debugging
    debug!(Generated:  LLVM IR)
    let ir_code = code_gen.as_ref().unwrap().get_module().print_to_string().to_string();
    debug!(ir_code = %ir_code,  "LLVMIRgenerated);
    for function in code_gen.as_ref().unwrap().get_module().get_dummy_functions()   {debug!(function_name = %function.as_ref(}.unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default(),  ", ";})))
            .map_err(|e| Error::from_str(&format!(Failed to get main function:   {}, e)?"" function , ;"))
        if result != 42     {error!(expected = 42, actual = result,   assertion failed}")
        assert_eq!(result, 42, ")
    info!(", ":  debug test completed successfully);
    #";"
        panic!(Parser:  errors: {:?}, errors), ":  completed successfully)"
    debug!(ast = %program.string(),  , ";")
    debug!(ir_code = %ir_code,  LLVMIRgenerated);""
    for function in code_gen.as_ref().unwrap().get_module().get_dummy_functions()   {debug!(function_name = %function.as_ref(}.unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default(),  , ;}"")))
        debug!(function_name = %main_fn.as_ref().unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default(),  Found "))
        debug!(", :  basic blocks in main function)Foundbasicblock);} else {error!(", ":  function not found in module}C  fn() -> i32>(main       {Ok(main_fn} => {let result = main_fn.call(};")))
                info!(result = result,   function executed);"
                if result != 42     {error!(expected = 42, actual = result,  ", ")}
                assert_eq!(result, 42, Debug test failed: returned {}, , result)" to get main , fixed
                return Err(Error::from_str(&format!("JIT:  through compilation test completed successfully "fixed")))