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
fn test_jit_basic_variables() -> Result<(), Error> {
    // Test basic variable operations (simpler than arrays)
    let input = r#"vibe test

slay main() {
    fr fr Create variables
    sus a = 10
    sus b = 20
    sus c = 30
    
    fr fr Access variable
    sus val = c
    
    lowkey val == 30 {
        puts(1)
    }
    
    yolo 0
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
    let dummy_path = PathBuf::from("./dummy_basic_vars.csd");
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
        
        // Test should return 0 for success
        assert_eq!(result, 0, "Basic variable test failed: returned {}", result);
    }
    
    Ok(())
}

#[test]
fn test_jit_struct_basic() -> Result<(), Error> {
    // Test basic struct operations
    let input = r#"vibe test

be_like Person squad {
    name tea
    age normie
}

slay main() {
    fr fr Create a Person instance
    sus person = Person{name: "John", age: 30}
    
    fr fr Access struct field
    sus val = person.age
    
    lowkey val == 30 {
        puts(1)
    }
    
    yolo 0
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
        
        // Test should return 0 for success
        assert_eq!(result, 0, "Struct basic test failed: returned {}", result);
    }
    
    Ok(())
}