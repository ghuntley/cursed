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
fn test_jit_array_basic() -> Result<(), Error> {
    // Test basic array operations
    let input = r#"
    vibe test;

    slay main() {
        sus arr normie[5] = [10, 20, 30, 40, 50];
        sus val = arr[2]; // Should be 30
        
        lowkey val == 30 {
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
    let dummy_path = PathBuf::from("./dummy_array_test.csd");
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
        assert_eq!(result, 1, "Array basic test failed: returned {}", result);
    }
    
    Ok(())
}

#[test]
fn test_jit_array_mutation() -> Result<(), Error> {
    // Test array mutation
    let input = r#"
    vibe test;

    slay main() {
        sus arr normie[5] = [10, 20, 30, 40, 50];
        arr[2] = 99;
        sus val = arr[2]; // Should be 99 now
        
        lowkey val == 99 {
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
    let dummy_path = PathBuf::from("./dummy_array_mutation.csd");
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
        assert_eq!(result, 1, "Array mutation test failed: returned {}", result);
    }
    
    Ok(())
}

#[test]
fn test_jit_array_mixed_types() -> Result<(), Error> {
    // Test array with mixed type elements
    let input = r#"
    vibe test;

    slay main() {
        sus arr normie[5] = [10, 20, 30, 40, 50];
        sus val1 = arr[0]; // Integer: 10
        sus val2 = arr[1]; // Float: 20.5 (but stored as an integer in the array)
        
        lowkey val1 == 10 && val2 == 20 {
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
    let dummy_path = PathBuf::from("./dummy_array_mixed.csd");
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
        assert_eq!(result, 1, "Array mixed types test failed: returned {}", result);
    }
    
    Ok(())
}