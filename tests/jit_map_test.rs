use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast::Program;
use cursed::prelude::*;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use cursed::codegen::llvm::LlvmCodeGenerator;

#[test]
fn test_jit_map_basic() -> Result<(), Error> {
    // Test basic map operations
    let input = r#"
    vibe test;

    slay main() {
        sus scores = {"Alice": 95, "Bob": 87, "Charlie": 92};
        sus alice_score = scores["Alice"];
        
        lowkey alice_score == 95 {
            puts(1);
        } highkey {
            puts(0);
        }
        
        yolo 0;
    }
    "#;
    
    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    // Ensure no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors());
    }
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_map_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "main", dummy_path);
    
    // Compile the program
    code_gen.compile_program(&program)?;
    
    // Create JIT execution engine
    let execution_engine = code_gen.module().create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;
    
    // Execute the main function
    unsafe {
        let main_fn = execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;
            
        let result = main_fn.call();
        
        // Test should return 1 for success
        assert_eq!(result, 1, "Map basic test failed: returned {}", result);
    }
    
    Ok(())
}

#[test]
fn test_jit_map_mutation() -> Result<(), Error> {
    // Test map mutation
    let input = r#"
    vibe test;

    slay main() {
        sus scores = {"Alice": 95, "Bob": 87, "Charlie": 92};
        scores["Bob"] = 90;
        sus bob_score = scores["Bob"];
        
        lowkey bob_score == 90 {
            puts(1);
        } highkey {
            puts(0);
        }
        
        yolo 0;
    }
    "#;
    
    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_map_mutation.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "main", dummy_path);
    
    // Compile the program
    code_gen.compile_program(&program)?;
    
    // Create JIT execution engine
    let execution_engine = code_gen.module().create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;
    
    // Execute the main function
    unsafe {
        let main_fn = execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;
            
        let result = main_fn.call();
        
        // Test should return 1 for success
        assert_eq!(result, 1, "Map mutation test failed: returned {}", result);
    }
    
    Ok(())
}

#[test]
fn test_jit_map_missing_key() -> Result<(), Error> {
    // Test map with missing key access
    let input = r#"
    vibe test;

    slay main() {
        sus scores = {"Alice": 95, "Bob": 87, "Charlie": 92};
        sus dave_score = scores["Dave"]; // Missing key
        
        // Missing key should return 0 in our simplified implementation
        lowkey dave_score == 0 {
            puts(1);
        } highkey {
            puts(0);
        }
        
        yolo 0;
    }
    "#;
    
    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_map_missing.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "main", dummy_path);
    
    // Compile the program
    code_gen.compile_program(&program)?;
    
    // Create JIT execution engine
    let execution_engine = code_gen.module().create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;
    
    // Execute the main function
    unsafe {
        let main_fn = execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;
            
        let result = main_fn.call();
        
        // Test should return 1 for success
        assert_eq!(result, 1, "Map missing key test failed: returned {}", result);
    }
    
    Ok(())
}