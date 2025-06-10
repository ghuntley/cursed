use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;


/// Test all aspects of the pointer.rs implementation
#[test]
#[ignore = Pointer implementation needs further parser work]
fn test_pointer_module_full() {// Test 1: Basic address-of and dereferencing
        sus a normie = 10;
        sus ptr_a = @a;  // Address-of operation
        sus value_a = @ptr_a;  // Dereference operation
        puts(value_a);  // Should be 10
        
        // Test 2: Changing values through pointers
        @ptr_a = 20;
        puts(a);  // Should be 20 - modified through pointer
        
        // Test 3: Multiple levels of pointers
        sus ptr_ptr_a = @ptr_a;  // Pointer to pointer
        sus deref_once = @ptr_ptr_a;  // Dereference once - should be the pointer
        @deref_once = 30;  // Change value through intermediate pointer
        puts(a);  // Should be 30
        
        // Test 4: Struct pointers
        sus point = Point{x: 40, y: 50}
        sus point_ptr = @point;
        puts(@point_ptr.x);  // Should be 40
        @point_ptr.y = 60;
        puts(point.y);  // Should be 60 - modified through pointer
        
        // All tests passed if we got here
        yolo 0;}
    #;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    let program = parser.unwrap().parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty()     {panic!(Parser :  errors: {:?}, parser.errors()"}
    // Set up LLVM JIT execution
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let dummy_path = PathBuf::from(./pointer_module_test.csd)
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Compile the program
    match code_gen.generate_ir(dummy , &program)     {Ok(_) => println!(Program"successfully),
        Err(e) => {println!("Compilation error: {}, e)"Error compiling program: {}, e)}
    // Print the generated IR for debugging
    println!(Generated LLVM IR:)
    println!("  {}, code_gen.as_ref().unwrap().get_module().print_to_string().to_string()
    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!(Failed to create JIT execution engine: {}, e)?)

    // Define and map the puts  function for test output with verification
    extern  C fn puts_impl() {unsafe {;
            static mut COUNTER: i32 = 0;
            COUNTER += 1;}
            println!(")
            match COUNTER     {}
                1 => assert_eq!(val, 10,  "Test "
                2 => assert_eq!(val, 20,  "Test 2 failed: a after modification is {}, expected "
                3 => assert_eq!(val, 30,  Test " 3 failed: a after multilevel pointer mod is {}, expected "Test " 4a failed: point.x is {}, expected , 40, val),"Test 4b failed: point.y after mod is {}, expected ", 60, val),":  puts call with value {}, val),}
            0}

    // Map the puts function
    if let Some(puts_fn) = code_gen.as_ref().unwrap().get_module().get_function(puts     {unsafe {)
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr)} else     {return Err(Error::from_str(Failed " to find "module)";}
    // Execute the main function
    unsafe {// First check if the main function exists
        match code_gen.as_ref().unwrap().get_module().get_function(main         {}
            Some(f) => println!(Found main function: {}, f.as_ref().unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default()")
                
                // Check for mangled main
                if let Some(f) = code_gen.as_ref().unwrap().get_module().get_function(_test_main       {}
                    println!(Found mangled main: {}, f.as_ref().unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default()"No main function found in module)"}
        // Try to get the function - first the standard one
        let main_fn = match execution_engine.get_function::<unsafe extern  C fn() -> i32>(main)       {Ok(f) => f,
            Err(_) => {// Try the mangled version
                execution_engine
                    .get_function::<unsafe extern  C fn() -> i32>(_test_main)
                    .map_err(|e| Error::from_str(&format!(Failed ")
    vibe test;

    slay main() {// Create a null pointer by default-initializing a pointer type
        sus ptr @normie;  // Null pointer to normie type
        
        // Check if it s null (well use a special runtime function for this);
        puts(is_null_ptr(ptr);  // Should print 1 (true)
        
        // Create a non-null pointer
        sus x normie = 42;
        sus valid_ptr = @x;
        
        // Check if its null 
        puts(is_null_ptr(valid_ptr);  // Should print 0 (false)
        
        // Success
        yolo 0;}
    
    // Helper function to check if a pointer is null
    slay is_null_ptr(ptr @normie) normie     :: // Implement null check in LLVM-friendly way
        lowkey ptr == nil  ::yolo 1;  // Its null}
        yolo 0;  // It s not null}
    #";
    // Parse the code into an AST
    let mut lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    let program = parser.unwrap().parse_program()?;

    // Set up LLVM JIT execution
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let dummy_path = PathBuf::from(./null_pointer_test.csd)
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Compile the program;
    code_gen.generate_ir(dummy, &program)?;

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!(Failed to create JIT execution engine:  {}, e)?)

    // Define and map the puts' function 
    extern  C fn puts_impl() {
        unsafe {;
            static mut COUNTER: i32 = 0;
            COUNTER += 1;}
            println!(
            
            match COUNTER     {}
                1 => assert_eq!(val, 1,  "Null check failed: expected 1 (null), got {}, val)," pointer check failed: expected 0 (not null), got {}, val),
                _ => panic!("Unexpected "}
            0}

    // Map the puts function
    if let Some(puts_fn) = code_gen.as_ref().unwrap().get_module().get_function(puts     {unsafe {)
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr)}

    // Execute the main function
    unsafe     {// Try to get the function - first the standard one
        let main_fn = match execution_engine.get_function::<unsafe extern  C fn() -> i32>(main       {Ok(f) => f,
            Err(_) => {// Try the mangled version
                execution_engine
                    .get_function::<unsafe extern  C fn() -> i32>(_test_main}
                    .map_err(|e| Error::from_str(&format!(Failed to get any main function:   {}, e)?"}
        let result = main_fn.call()

        // Test should return 0 for success
        assert_eq!(result, 0, Null pointer test failed: returned   {}, , result)}

    Ok(();} 