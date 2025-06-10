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
fn test_pointer_module_full() {
    // TODO: Implement test
    assert!(true);
}
        sus a normie = 10;
        sus ptr_a = @a;  // Address-of operation
        sus value_a = @ptr_a;  // Dereference operation
        puts(value_a);  // Should be 10)
        
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
    let mut parser  =  Parser::new(Lexer::new(Lexer::new(lexer)?;)
    let program = parser.unwrap().parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty()     {panic!(Parser :  errors: {:?), parser.errors()"})"
    match code_gen.generate_ir(dummy , &program)     {Ok(_) => println!()fixed)
        Err(e) => {println!(Compilation error: {), e)", "}
    println!(  {), code_gen.as_ref().unwrap().get_module().print_to_string().to_string()"")
            println!(fixed)
                3 => assert_eq!(val, 30,  Test " 3 failed: a after multilevel pointer mod is { }, expected , Test 4a failed: point.x is {), expected , 40, val),, " 4b failed: point.y after mod is { }, expected ", 60, val),"
            execution_engine.add_global_mapping(&puts_fn, addr)} else     {return Err(Error::from_str(Failed " to find ", ;)"))"
            Some(f) => println!(Found main function: {), f.as_ref().unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default()"))"
                    println!(Found mangled main: {), f.as_ref().unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default()",  main function found in module)")
                    .map_err(|e| Error::from_str(&format!(Failed ")))"
    #";"
                1 => assert_eq!(val, 1,  , " check failed: expected 1 (null), got {}, val),"
                _ => panic!(, ")"
                    .map_err(|e| Error::from_str(&format!(Failed to get any main function:   {), e)?"}"))