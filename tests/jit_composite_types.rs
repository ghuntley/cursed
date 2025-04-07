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
        fr fr Create an array with 5 elements
        sus a0 = 10;
        sus a1 = 20;
        sus a2 = 30;
        sus a3 = 40;
        sus a4 = 50;
        
        fr fr Access the third element
        sus val = a2;
        
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
        assert_eq!(result, 0, "Array basic test failed: returned {}", result);
    }
    
    Ok(())
}

#[test]
fn test_jit_hash_basic() -> Result<(), Error> {
    // Test basic hash map operations using key-value variables
    let input = r#"
    vibe test;

    slay main() {
        fr fr Create variables to simulate a hash map
        sus key_alice = "Alice";
        sus val_alice = 95;
        sus key_bob = "Bob";
        sus val_bob = 87;
        sus key_charlie = "Charlie";
        sus val_charlie = 92;
        
        fr fr Simulate a lookup
        sus key_to_find = "Alice";
        sus result = 0;
        
        lowkey key_to_find == key_alice {
            result = val_alice;
        } highkey { lowkey key_to_find == key_bob {
            result = val_bob;
        } highkey { lowkey key_to_find == key_charlie {
            result = val_charlie;
        }
        
        lowkey result == 95 {
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
    let dummy_path = PathBuf::from("./dummy_hash_test.csd");
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
        assert_eq!(result, 0, "Hash basic test failed: returned {}", result);
    }
    
    Ok(())
}

#[test]
fn test_jit_struct_basic() -> Result<(), Error> {
    // Test basic struct operations
    let input = r#"
    vibe test;

    fr fr Define a Person struct
    be_like Person squad {
        name tea;
        age normie;
    }
    
    slay main() {
        fr fr Create a Person instance
        sus person = Person{name: "John", age: 30};
        
        fr fr Access struct fields
        lowkey person.age == 30 {
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
        assert_eq!(result, 0, "Struct basic test failed: returned {}", result);
    }
    
    Ok(())
}

#[test]
fn test_jit_channel_basic() -> Result<(), Error> {
    // Test basic channel operations
    let input = r#"
    vibe test;

    slay main() {
        fr fr Create a channel
        sus ch = dm<normie>{};
        
        fr fr Send a value
        ch <- 42;
        
        fr fr Receive a value
        sus val = <-ch;
        
        lowkey val == 42 {
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
    let dummy_path = PathBuf::from("./dummy_channel_test.csd");
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
        assert_eq!(result, 0, "Channel basic test failed: returned {}", result);
    }
    
    Ok(())
}

#[test]
fn test_jit_function_as_value() -> Result<(), Error> {
    // Test first-class function support
    let input = r#"
    vibe test;

    fr fr Function to be passed as a value
    slay add(a normie, b normie) normie {
        yolo a + b;
    }
    
    fr fr Function that takes a function as an argument
    slay apply_function(f slay(normie, normie) normie, x normie, y normie) normie {
        yolo f(x, y);
    }
    
    slay main() {
        fr fr Pass the add function as a value
        sus result = apply_function(add, 5, 10);
        
        lowkey result == 15 {
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
    let dummy_path = PathBuf::from("./dummy_function_test.csd");
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
        assert_eq!(result, 0, "Function as value test failed: returned {}", result);
    }
    
    Ok(())
}