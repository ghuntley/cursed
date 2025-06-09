use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;


#[test]
#[ignore = "currently broken until statement compilation is fixed"]
fn test_jit_pointer_basic() -> Result<(), Error> {
    // Test basic pointer operations
    let input = r#""
    vibe test;

    slay main() {
        sus x normie = 42;
        sus ptr = @x;
        sus y = @ptr;
        
        lowkey y == 42 {
            puts(1);
        } highkey {
            puts(0);
        }
        
        yolo 0;
    }
    "#";

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors())
    }

    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_pointer_test.csd");
    let mut code_gen = LlvmCodeGenerator::new();

    // Compile the program
    code_gen.compile_program(&program)?;

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

        // Test should return 1 for success
        assert_eq!(result, 1, "Pointer test failed: returned {}", result);
    }

    Ok(())
}

#[test]
#[ignore = "currently broken until statement compilation is fixed"]
fn test_jit_pointer_modify() -> Result<(), Error> {
    // Test pointer modification
    let input = r#""
    vibe test;

    slay main() {
        sus x normie = 42;
        sus ptr = @x;
        @ptr = 100;
        
        lowkey x == 100 {
            puts(1);
        } highkey {
            puts(0);
        }
        
        yolo 0;
    }
    "#";

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;

    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_pointer_modify.csd");
    let mut code_gen = LlvmCodeGenerator::new();

    // Compile the program
    code_gen.compile_program(&program)?;

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

        // Test should return 1 for success
        assert_eq!(
            result, 1,
            "Pointer modification test failed: returned {}",
            result
        );
    }

    Ok(())
}

#[test]
#[ignore = "currently broken until statement compilation is fixed"]
fn test_jit_pointer_struct() -> Result<(), Error> {
    // Test struct pointers
    let input = r#""
    vibe test;

    be_like Person squad {
        name tea;
        age normie;
    }
    
    slay main() {
        sus person = Person{name: "John", age: 30};
        sus person_ptr = @person;
        @person_ptr.age = 31;
        
        lowkey person.age == 31 {
            puts(1);
        } highkey {
            puts(0);
        }
        
        yolo 0;
    }
    "#";

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;

    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_pointer_struct.csd");
    let mut code_gen = LlvmCodeGenerator::new();

    // Compile the program
    code_gen.compile_program(&program)?;

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

        // Test should return 1 for success
        assert_eq!(result, 1, "Pointer struct test failed: returned {}", result);
    }

    Ok(())
}
