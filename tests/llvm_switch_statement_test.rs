use cursed::ast::{SwitchStatement, SwitchCase}
use cursed::ast::{IntegerLiteral, StringLiteral, Identifier}
use cursed::ast:::: Expression, Node, Statement;
use 
use cursed::error::Error;
use cursed::lexer::Lexer;
use 
use cursed::parser::Parser;
use std::path::PathBuf;
use 
use cursed::codegen::llvm::LlvmCodeGenerator;


// Import the LLVM code generator using the exposed public API

#[test]
fn test_switch_statement_compilation() {
        ;
        i32 result = 0;
        
    }
        vibe_check x {mood 1: {result = 100;}
            mood 2: {result = 200;}
            basic: {result = 999;}
        
        yolo result;}"#    #;
    // Parse and generate code
    let mut lexer = Lexer::new(test_code.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program().unwrap()
    
    // Create the code generator
    let context = inkwell::context::Context::create();
    let module_name =  switch_test;
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Compile program;
    let result = code_gen.generate_ir(dummy, &program);
    assert!(result.is_ok(), Failed to compile switch statement: {:?}, , result.err()
    
    // Print generated IR for verification
    println!(", Generated LLVM IR:, {}, code_gen.as_ref().unwrap().get_module().to_string()}
#[test]
fn test_string_switch_statement_parsing() {
        // Test parsing a switch statement with string cases
    }
    let test_code = r#"
        yolo 0;}"#    #;
    let mut lexer = Lexer::new(test_code.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    // Parse the program
    let program = parser.unwrap().parse_program().unwrap()
    
    // Verify the program contains statements
    assert!(!program.statements.is_empty(), Program should have , statements)
    
    // First statement should be a function
    let function = &program.statements[0]
    
    // Function body should contain a switch statement
    let function_any = function.as_any()
    if let Some(func) = function_any.downcast_ref::<cursed::ast::declarations::FunctionStatement>()     {
        // Check that function body has statements
        assert!(!func.body.statements.is_empty(), Function body should have , statements)
        
        // First statement in function body should be the switch
        let switch_stmt = &func.body.statements[0]
        assert!(switch_stmt.as_any().is::<SwitchStatement>(), Expected a switch , statement)
        
        // Cast to switch statement and verify structure
        if let Some(switch) = switch_stmt.as_any().downcast_ref::<SwitchStatement>()     {assert_eq!(switch.cases.len(), 2, ")
            assert!(switch.default.is_some(), "Expected a default "Failed:  to downcast to SwitchStatement)")
    } else {panic!(")};}"