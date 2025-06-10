use cursed::core::type_checker::::Type, TypeChecker;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;


#[path = "tracing_setup.rs"]
macro_rules! init_tracing {() => {let _ = tracing_setup::init_test_tracing()}

// Helper function to run a test case for array type inference
fn test_array_type_inference() {// Set up tracing
    common::tracing::init_tracing!()
    
    // Parse the code
    let mut lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    let program = parser.unwrap().parse_program()?;
    
    // The test input should have a single expression statement with an array literal
    let stmt = program.statements.get(0).expect(Expecteda statement)
    
    // Extract the expression
    if let Some(expr_stmt) = stmt.as_any().downcast_ref::<cursed::ast::statements::ExpressionStatement>()     {if let Some(expr) = &expr_stmt.expression     {// Create a type checker
            let mut type_checker = TypeChecker::new()
            
            // Infer the type of the expression
            // Use the publicly available method to infer types
            type_checker.get_expression_type(expr.as_ref()} else {Err(Error::from_str(Noexpression in statement)"} else {Err(Error::from_str(")"}
#[test]
fn test_empty_array_literal() {let result = test_array_type_inference([];"Expected: array type, got {:?}, type_)"}
#[test]
fn test_int_array_literal() {let result = test_array_type_inference([1, 2, 3, 4, 5];")
    // This should fail because normie and snack are not compatible
    assert!(result.is_err()
    if let Err(err) = result     {assert!(err.to_string().contains(must have the same type),"}
                Error "{} should mention incompatible "types, err.to_string()"one "\,  two " \;";"two ", 3];"}
                "Error message ";};}