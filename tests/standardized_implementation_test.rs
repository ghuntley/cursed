use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;


/// This test verifies that the standardized implementation of the LlvmCodeGenerator
/// properly handles address-of operations.
#[test]
fn test_standardized_address_of() -> Result<(), Error> {
    // Simple test program with address-of operations
    let input = r#"
    vibe test;

    slay main() {
        // Basic variable declaration and address-of
        sus x normie = 42;
        sus ptr = @x;  // Take address of x
        
        // Verify we can dereference the pointer to get the original value
        sus y = @ptr;
        
        // Add debug output
        puts(y);  // Should print 42
        
        // Return 0 if the test passed
        yolo 0;
    }
    "#;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors())
    }

    // Set up LLVM JIT execution with the standardized implementation
    let context = Context::create();
    let dummy_path = PathBuf::from("./standardized_implementation_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", dummy_path);

    // Print debug information about the LlvmCodeGenerator implementation
    println!("Using standardized LlvmCodeGenerator implementation");
    
    // Compile the program
    code_gen.compile_program(&program)?;

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Define and map the 'puts' function for test output
    extern "C" fn puts_impl(val: i32) -> i32 {
        println!("Test output: {}", val);
        0 // Return 0 for success
    }

    // Map the puts function
    if let Some(puts_fn) = code_gen.module().get_function("puts") {
        unsafe {
            // Convert function pointer to usize for LLVM mapping
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr);
        }
    }

    // Execute the main function
    unsafe {
        let main_fn = execution_engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;

        let result = main_fn.call();

        // Test should return 0 for success
        assert_eq!(result, 0, "Standardized implementation test failed: returned {}", result);
    }

    Ok(())
}

/// Test address-of and pointer modification
#[test]
fn test_standardized_pointer_modification() -> Result<(), Error> {
    let input = r#"
    vibe test;

    slay main() {
        // Declare a variable
        sus x normie = 10;
        
        // Get a pointer to it
        sus ptr = @x;
        
        // Change the value through the pointer
        @ptr = 50;
        
        // Output the new value
        puts(x);  // Should print 50
        
        // Return 0 if successful
        yolo 0;
    }
    "#;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./standardized_pointer_mod_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", dummy_path);
    
    // Compile the program
    code_gen.compile_program(&program)?;

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Define and map the 'puts' function for test output
    extern "C" fn puts_impl(val: i32) -> i32 {
        println!("Test output: {}", val);
        assert_eq!(val, 50, "Pointer modification test failed: variable value is {}, expected 50", val);
        0
    }

    // Map the puts function
    if let Some(puts_fn) = code_gen.module().get_function("puts") {
        unsafe {
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr);
        }
    }

    // Execute the main function
    unsafe {
        let main_fn = execution_engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;

        let result = main_fn.call();

        // Test should return 0 for success
        assert_eq!(result, 0, "Standardized implementation pointer modification test failed: returned {}", result);
    }

    Ok(())
}

/// Test pointer to struct types
#[test]
fn test_standardized_struct_pointer() -> Result<(), Error> {
    let input = r#"
    vibe test;

    be_like Point squad {
        x normie;
        y normie;
    }
    
    slay main() {
        // Create a struct
        sus point = Point{x: 10, y: 20};
        
        // Take the address of the struct
        sus ptr = @point;
        
        // Modify the struct through the pointer
        @ptr.x = 30;
        @ptr.y = 40;
        
        // Output the modified values
        puts(point.x);  // Should print 30
        puts(point.y);  // Should print 40
        
        // Return 0 for success
        yolo 0;
    }
    "#;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./standardized_struct_ptr_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", dummy_path);
    
    // Compile the program
    code_gen.compile_program(&program)?;

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Define and map the 'puts' function for test output
    let mut counter = 0;
    extern "C" fn puts_impl(val: i32) -> i32 {
        unsafe {
            static mut COUNTER: i32 = 0;
            COUNTER += 1;
            
            println!("Test output {}: {}", COUNTER, val);
            
            match COUNTER {
                1 => assert_eq!(val, 30, "Struct pointer test failed: x field is {}, expected 30", val),
                2 => assert_eq!(val, 40, "Struct pointer test failed: y field is {}, expected 40", val),
                _ => {},
            }
            0
        }
    }

    // Map the puts function
    if let Some(puts_fn) = code_gen.module().get_function("puts") {
        unsafe {
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr);
        }
    }

    // Execute the main function
    unsafe {
        let main_fn = execution_engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;

        let result = main_fn.call();

        // Test should return 0 for success
        assert_eq!(result, 0, "Standardized implementation struct pointer test failed: returned {}", result);
    }

    Ok(())
} 