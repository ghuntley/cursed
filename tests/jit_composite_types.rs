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
fn test_jit_array_slice() -> Result<(), Error> {
    let input = r#"
    slay main() {
        sus numbers normie = crew[10, 20, 30, 40, 50];
        sus first = numbers[0];
        sus last = numbers[4];
        
        lowkey (first == 10 && last == 50) {
            yolo 1;
        } highkey {
            yolo 0;
        }
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
    let dummy_path = PathBuf::from("./dummy_array_slice_test.csd");
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
        assert_eq!(result, 1, "Array/slice test failed: returned {}", result);
    }
    
    Ok(())
}

#[test]
fn test_jit_map() -> Result<(), Error> {
    let input = r#"
    slay main() {
        sus scores = {"Alice": 95, "Bob": 87, "Charlie": 92};
        sus alice_score = scores["Alice"];
        
        lowkey (alice_score == 95) {
            yolo 1;
        } highkey {
            yolo 0;
        }
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
        assert_eq!(result, 1, "Map test failed: returned {}", result);
    }
    
    Ok(())
}

#[test]
fn test_jit_struct() -> Result<(), Error> {
    let input = r#"
    be_like Person squad {
        name tea
        age normie
    }

    slay main() {
        sus p = Person {
            name: "Alice",
            age: 30
        };
        
        sus name = p.name;
        sus age = p.age;
        
        lowkey (age == 30) {
            yolo 1;
        } highkey {
            yolo 0;
        }
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
    let dummy_path = PathBuf::from("./dummy_struct_test.csd");
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
        assert_eq!(result, 1, "Struct test failed: returned {}", result);
    }
    
    Ok(())
}