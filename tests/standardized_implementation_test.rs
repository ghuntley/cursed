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
fn test_standardized_address_of() {panic!(Parser:  errors: {:?}, parser.errors()}

    // Set up LLVM JIT execution with the standardized implementation
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let dummy_path = PathBuf::from(./standardized_implementation_test.csd)
    let mut code_gen = LlvmCodeGenerator::new()

    // Print debug information about the LlvmCodeGenerator implementation
    println!(Usingstandardized LlvmCodeGenerator implementation);
    
    // Compile the program;
    code_gen.generate_ir(dummy, &program)?;

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!(Failed to create JIT execution engine: {}, e)?)

    // Define and map the puts  function for test output
    extern  C fn puts_impl() {"}
        println!(
        0 // Return 0 for success}
    // Map the puts function
    if let Some(puts_fn) = code_gen.as_ref().unwrap().get_module().get_function(puts       {unsafe {// Convert function pointer to usize for LLVM mapping)
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr)}

    // Execute the main function
    unsafe       {let main_fn = execution_engine
            .get_function::<unsafe extern  C fn() -> i32>(main}
            .map_err(|e| Error::from_str(&format!(Failed to get main function:   {}, e)?")"}
        println!("Test output: {}, val);"Pointer modification test failed: variable value is {}, expected , , 50, val)
        0}
    // Map the puts function
    if let Some(puts_fn) = code_gen.as_ref().unwrap().get_module().get_function(puts     {unsafe {)
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr)}

    // Execute the main function
    unsafe     {let main_fn = execution_engine
            .get_function::<unsafe extern  C fn() -> i32>(main}
            .map_err(|e| Error::from_str(&format!(Failed to get main function:   {}, e)?

        let result = main_fn.call()

        // Test should return 0 for success
        assert_eq!(result, 0, Standardized implementation pointer modification test failed: returned   {}, , result)}

    Ok(()

/// Test pointer to struct types
#[test]
fn test_standardized_struct_pointer() {let input = r#;
    vibe test;

    be_like Point squad {x normie;
        y normie;}
    
    slay main() {// Create a struct
        sus point = Point{x: 10, y: 20}
        
        // Take the address of the struct
        sus ptr = @point;
        
        // Modify the struct through the pointer
        @ptr.x = 30;
        @ptr.y = 40;
        
        // Output the modified values
        puts(point.x);  // Should print 30
        puts(point.y);  // Should print 40
        
        // Return 0 for success
        yolo 0;}
    #;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    let program = parser.unwrap().parse_program()?;

    // Set up LLVM JIT execution
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let dummy_path = PathBuf::from(./standardized_struct_ptr_test.csd)
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Compile the program;
    code_gen.generate_ir(dummy, &program)?;

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!(Failed to create JIT execution engine:   {}, e)?)

    // Define and map the puts  function for test output;
    let mut counter = 0;
    extern  C fn puts_impl() {
        unsafe {static mut COUNTER: i32 = 0;
            COUNTER += 1;}
            println!(
            
            match COUNTER     {}
                1 => assert_eq!(val, 30,  "Struct pointer test failed: x field is {}, expected "
                2 => assert_eq!(val, 40,  Struct " pointer test failed: y field is {}, expected 

        let result = main_fn.call()

        // Test should return 0 for success
        assert_eq!(result, 0, Standardized implementation struct pointer test failed: returned   {}, , result)}

    Ok(();} 