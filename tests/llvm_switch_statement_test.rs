use cursed::ast::control_flow::{SwitchStatement, SwitchCase};
use cursed::ast::expressions::{IntegerLiteral, StringLiteral, Identifier};
use cursed::ast::{Expression, Node, Statement};
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use cursed::codegen::llvm::LlvmCodeGenerator;


// Import the LLVM code generator using the exposed public API

#[test]
fn test_switch_statement_compilation() {
    // Test compiling a switch statement
    let test_code = r#"
    slay test_switch(i32 x) i32 {
        i32 result = 0;
        
        vibe_check x {
            mood 1: {
                result = 100;
            }
            mood 2: {
                result = 200;
            }
            basic: {
                result = 999;
            }
        }
        
        yolo result;
    }
    "#;
    
    // Parse and generate code
    let mut lexer = Lexer::new(test_code);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    // Create the code generator
    let context = inkwell::context::Context::create();
    let module_name = "switch_test";
    let mut code_gen = LlvmCodeGenerator::new(&context, module_name, PathBuf::from("switch_test.csd"));
    
    // Compile program
    let result = code_gen.compile_program(&program);
    assert!(result.is_ok(), "Failed to compile switch statement: {:?}", result.err());
    
    // Print generated IR for verification
    println!("Generated LLVM IR:\n{}", code_gen.module().to_string());
}

#[test]
fn test_string_switch_statement_parsing() {
    // Test parsing a switch statement with string cases
    let test_code = r#"
    slay test_switch(txt day) i32 {
        vibe_check day {
            mood "Monday": {
                result = 1;
            }
            mood "Tuesday": {
                result = 2;
            }
            basic: {
                result = 7;
            }
        }
        yolo 0;
    }
    "#;
    
    let mut lexer = Lexer::new(test_code);
    let mut parser = Parser::new(&mut lexer).unwrap();
    
    // Parse the program
    let program = parser.parse_program().unwrap();
    
    // Verify the program contains statements
    assert!(!program.statements.is_empty(), "Program should have statements");
    
    // First statement should be a function
    let function = &program.statements[0];
    
    // Function body should contain a switch statement
    let function_any = function.as_any();
    if let Some(func) = function_any.downcast_ref::<cursed::ast::declarations::FunctionStatement>() {
        // Check that function body has statements
        assert!(!func.body.statements.is_empty(), "Function body should have statements");
        
        // First statement in function body should be the switch
        let switch_stmt = &func.body.statements[0];
        assert!(switch_stmt.as_any().is::<SwitchStatement>(), "Expected a switch statement");
        
        // Cast to switch statement and verify structure
        if let Some(switch) = switch_stmt.as_any().downcast_ref::<SwitchStatement>() {
            assert_eq!(switch.cases.len(), 2, "Expected 2 cases");
            assert!(switch.default.is_some(), "Expected a default case");
        } else {
            panic!("Failed to downcast to SwitchStatement");
        }
    } else {
        panic!("First statement should be a function");
    }
}