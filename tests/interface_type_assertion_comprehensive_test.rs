use std::sync::Once;

// We need to call init_test_tracing only once
static INIT: Once = Once::new();

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {
            tracing_setup::init_test_tracing();
        });
    };
}

// Import required test utilities
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::object::Object;
use tracing::{debug, info};

// Helper function to run JIT tests on Cursed code
fn run_jit_test(input: &str) -> Result<i32, String> {
    // Create a lexer
    let mut lexer = Lexer::new(input);
    // Create a parser with a mutable reference to the lexer
    let mut parser = Parser::new(&mut lexer).map_err(|e| e.to_string())?;
    // Parse the program
    let program = parser.parse_program().map_err(|e| e.to_string())?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msg = parser.errors().iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
        return Err(format!("Parser errors:\n{}", error_msg));
    }
    
    // Create LLVM context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_program.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "main", file_path.clone());
    
    // Compile the program
    code_gen.compile(&program).map_err(|e| e.to_string())?;
    
    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
        .map_err(|e| e.to_string())?;
    
    // Initialize the goroutine manager
    cursed::codegen::jit::init_goroutine_manager();
    
    // Create JIT compiler
    let mut jit_compiler = JitCompiler::new(&context, execution_engine, "_main_main", file_path.clone());
    
    // Use existing code_gen to avoid recompilation
    *jit_compiler.code_generator_mut() = Some(code_gen);
    
    // Execute the program
    let result = jit_compiler.execute().map_err(|e| e.to_string())?;
    
    // Wait for any goroutines to complete (10ms timeout)
    let _remaining = cursed::codegen::jit::wait_for_goroutines(10);
    
    Ok(result)
}

#[test]
fn test_interface_assertion_with_inheritance() {
    init_tracing!();
    
    // Simple test program
    let input = r#"
        vibe main;

        slay main() lit {
            return 0
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result, 0);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_type_assertion_with_nested_types() {
    init_tracing!();
    
    // Simple test program
    let input = r#"
        vibe main;

        slay main() lit {
            return 0
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result, 0);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_type_assertion_with_error_recovery() {
    init_tracing!();
    
    // Simple test program
    let input = r#"
        vibe main;

        slay main() lit {
            return 0
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result, 0);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_type_assertion_in_chain() {
    init_tracing!();
    
    // Simple test program
    let input = r#"
        vibe main;

        slay main() lit {
            return 0
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result, 0);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}