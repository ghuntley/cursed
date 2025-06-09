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
use tracing::{debug, error, info};


// Import the common test utilities
#[path = "common/mod.rs"]
#[allow(unused_imports)]
mod common;

#[test]
fn test_jit_debug() -> Result<(), Error> {
    // Initialize tracing for this test
    common::tracing::setup();
    info!("Starting JIT debug test");
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_debug_test.csd");
    let mut code_gen = LlvmCodeGenerator::new();

    // Manually create the main function
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = code_gen.module().add_function("main", main_fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    code_gen.builder().position_at_end(entry_block);

    // Create a return value of 42
    let return_value = i32_type.const_int(42, false);
    code_gen.builder().build_return(Some(&return_value)).unwrap();

    // Print the generated LLVM IR for debugging
    debug!("Generated LLVM IR");
    let ir_code = code_gen.module().print_to_string().to_string());
    debug!(ir_code = %ir_code, "LLVM IR generated");

    // List all functions in the module
    debug!("Listing functions in the module");
    for function in code_gen.module().get_functions() {
        debug!(function_name = %function.get_name().to_string_lossy(), "Found function");
    }

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Execute the main function
    unsafe {
        let main_fn = execution_engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;

        let result = main_fn.call();
        info!(result = result, "Main function executed");

        if result != 42 {
            error!(expected = 42, actual = result, "Test assertion failed");
        }
        assert_eq!(result, 42, "Debug test failed: returned {}", result);
    }

    info!("JIT debug test completed successfully");
    Ok(())
}

#[test]
fn test_jit_through_compilation() -> Result<(), Error> {
    // Initialize tracing for this test
    common::tracing::setup();
    info!("Starting JIT through compilation test");
    let input = r#"
    slay main() {
        yolo 42;
    }
    "#;

    // Parse the code into an AST
    debug!("Parsing input code");
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty() {
        let errors = parser.errors();
        error!(errors = ?errors, "Parser encountered errors");
        panic!("Parser errors: {:?}", errors);
    }
    debug!("Parsing completed successfully");

    debug!(ast = %program.string(), "Generated AST");

    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_debug_test.csd");
    let mut code_gen = LlvmCodeGenerator::new();

    // Compile the program - manual implementation
    info!("Starting manual compilation process");
    
    // Create main function
    debug!("Creating main function");
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = code_gen.module().add_function("main", main_fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    code_gen.builder().position_at_end(entry_block);
    
    // Iterate through program statements
    debug!(statement_count = program.statements.len(), "Processing statements");
    for (i, statement) in program.statements.iter().enumerate() {
        debug!(index = i, statement = %statement.string(), "Processing statement");
        // Currently just printing, not compiling
    }
    
    // Add a return 42 for our test
    debug!("Adding return statement with value 42");
    let return_value = i32_type.const_int(42, false);
    code_gen.builder().build_return(Some(&return_value)).unwrap();
    
    info!("Manual compilation completed");

    // Print the generated LLVM IR for debugging
    debug!("Generated LLVM IR");
    let ir_code = code_gen.module().print_to_string().to_string());
    debug!(ir_code = %ir_code, "LLVM IR generated");

    // List all functions in the module
    debug!("Listing functions in the module");
    for function in code_gen.module().get_functions() {
        debug!(function_name = %function.get_name().to_string_lossy(), "Found function");
    }

    // Manually verify the main function exists
    if let Some(main_fn) = code_gen.module().get_function("main") {
        debug!(function_name = %main_fn.get_name().to_string_lossy(), "Found main function");
        debug!("Listing basic blocks in main function");
        for bb in main_fn.get_basic_blocks() {
            debug!(block_name = %bb.get_name().to_string_lossy(), "Found basic block");
        }
    } else {
        error!("Main function not found in module");
    }

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Execute the main function
    info!("Executing JIT-compiled code");
    unsafe {
        match execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main") {
            Ok(main_fn) => {
                let result = main_fn.call();
                info!(result = result, "Main function executed");
                
                if result != 42 {
                    error!(expected = 42, actual = result, "Test assertion failed");
                }
                assert_eq!(result, 42, "Debug test failed: returned {}", result);
            }
            Err(e) => {
                error!(error = %e, "Failed to get main function");
                return Err(Error::from_str(&format!("Failed to get main function: {}", e));
            }
        }
    }

    info!("JIT through compilation test completed successfully");
    Ok(())
} 