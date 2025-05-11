use cursed::ast::Program;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::codegen::jit::JitCompiler;
use inkwell::context::Context;
use std::path::PathBuf;

// Test for JIT integration with improved print support
#[test]
fn test_jit_print_support() -> Result<(), Error> {
    // Initialize tracing for better diagnostics (if common test module is used)
    if let Ok(()) = setup_test_tracing() {
        tracing::info!("Test tracing initialized");
    }
    
    // Create a simple CURSED program that prints different types
    let input = r#"
    vibe test;

    slay main() {
        // Test integer printing
        puts(42);
        
        // Test string printing
        println("Hello from JIT!");
        
        // Test bool printing (if print_bool is available)
        sus is_true lit = based;
        lowkey is_true {
            puts(1);
        } highkey {
            puts(0);
        }
        
        yolo 0;
    }
    "#;

    // Parse the program
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    // Verify there are no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors());
    }

    // Set up the JIT compiler
    let context = Context::create();
    let module = context.create_module("test");
    let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create execution engine: {:?}", e)))?;
    
    let mut jit = JitCompiler::new(
        &context,
        execution_engine,
        "test",
        PathBuf::from("test.csd")
    );
    
    // Generate code
    let code_gen = cursed::codegen::llvm::LlvmCodeGenerator::new(&context, module, "test");
    *(jit.code_generator_mut()) = Some(code_gen);
    
    if let Some(ref mut code_gen) = *(jit.code_generator_mut()) {
        code_gen.compile_program(&program)?;
        
        // Execute the compiled program
        match jit.execute() {
            Ok(exit_code) => {
                assert_eq!(exit_code, 0, "Program returned non-zero exit code: {}", exit_code);
                tracing::info!("JIT execution completed successfully with exit code 0");
            },
            Err(e) => {
                panic!("JIT execution failed: {:?}", e);
            }
        }
    } else {
        panic!("Failed to get code generator");
    }
    
    Ok(())
}

// Helper function to set up tracing in tests
fn setup_test_tracing() -> Result<(), ()> {
    // Try to use the common test module if available
    #[cfg(test)]
    {
        match std::path::Path::new("tests/common.rs").exists() {
            true => {
                #[path = "common.rs"]
                mod common;
                common::tracing::setup();
                return Ok(());
            },
            false => {}
        }
    }
    
    // Fallback to a simple tracing setup if common module not found
    #[allow(unused_imports)]
    match tracing_subscriber::fmt().with_env_filter("info").try_init() {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}