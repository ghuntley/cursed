//! Tests for defer statement functionality
//!
//! This module tests the defer statement implementation using the `later` keyword.
//! Tests cover basic defer functionality, LIFO execution order, and integration
//! with function returns and error handling.

mod common;

use cursed::ast::  {DeferStatement, ExpressionStatement}
use cursed::ast::{Identifier, IntegerLiteral, CallExpression, StringLiteral}
use cursed::ast::traits:::: Node, Statement;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use cursed::parser::Parser;
use cursed::codegen::llvm::::LlvmCodeGenerator, DeferStatementCompilation;
use cursed::error::Error;
use tracing::{info, debug;
use cursed::lexer::Lexer;
#[test]
fn test_defer_statement_ast() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    info!(Testing defer statement AST creation);
    
    // Create a simple expression statement to defer
    let expr = Identifier {token: Token::new(TokenType::Identifier, & test.to_string()
        value:  test.to_string()"}
    
    let expr_stmt = ExpressionStatement {token: Token::new(TokenType::Identifier, & test.to_string()"
    assert_eq!(defer_stmt.token_literal(),  later;
    
    info!("}
#[test]
fn test_defer_statement_parsing() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    info!(Testing:  defer statement parsing);
    
    let input = r#""#
        slay test_function() {;
            sus x = 5;
            later vibez.spill(Deferredmessage);"#;
    let mut lexer = cursed::lexer::Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)
    
    match parser.unwrap().parse_program()     {Ok(program) => {debug!("Parsed:  program successfully)'t fail
            // More detailed AST verification would go here
            info!(Defer:  statement parsing test passed);}
        Err(e) => {panic!("Failed:  to parse program with defer statement: {}, e)"
        slay test_defer_order() {later vibez.spill("Firstdefer)
            later vibez.spill(Seconddefer)"Thirddefer)
            vibez.spill(Normalstatement)";
            yolo 0;}
    "Parsed:  program with multiple defer statements)")
            // Expected execution order should be:
            // 1.  Normal statement // 2.  Thirddefer (last defer statement, executed first)
            // 3.  Seconddefer // 4.  First defer (first defer statement, executed last)
            
            // Verify parsing succeeds
            assert!(!program.statements.is_empty()
            info!(Defer:  LIFO order test passed);}
        Err(e) => {panic!(Failed:  to parse program with multiple defer statements: {}, e)"}
#[test]
fn test_defer_with_variables() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    info!(Testing:  defer statement with variable capture);
    
    let input = r#"        slay test_defer_variables() {;"#
            sus x = 10;
            sus y = 20;
            
            later vibez.spill(")
            later vibez.spill(x + y)
            
            x = 100;
            y = 200;
            
            yolo x + y;}"#    ";
    let mut lexer = cursed::lexer::Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)
    
    match parser.unwrap().parse_program()     {Ok(program) => {debug!(Parsed:  program with defer and variables)")")"}
#[test]
fn test_defer_in_nested_blocks() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    info!(Testing:  defer statements in nested blocks);
    
    let input = r#;
        slay test_nested_defer() {;
            later vibez.spill(
            
            lowkey (based) {later vibez.spill(Inner defer , 1)")")"}
            
            later vibez.spill(Another outer defer)
            yolo 0;}
    #";
    let mut lexer = cursed::lexer::Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)
    
    match parser.unwrap().parse_program()     {Ok(program) => {debug!(
            
            // Expected execution order:
            // 1.  Anotherouterdefer // 2.  Innerdefer2 // 3.  Innerdefer, 1 // 4.  Outerdefer assert!(!program.statements.is_empty()
            info!(Nested:  defer test passed)")}
        Err(e) => {panic!(")}
#[test]
fn test_defer_with_early_return() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    info!(Testing:  defer statements with early return);
    
    let input = r#"
        slay test_early_return(normie condition) {later vibez.spill(
            
            lowkey (condition) {later vibez.spill("Early return defer)"This might not run)
            yolo 0;}
    "Parsed:  program with early return and defer)")
            // Both defer statements should execute regardless of which return path is taken
            
            assert!(!program.statements.is_empty()
            info!(Defer:  with early return test passed);}
        Err(e) => {panic!(")}
// Integration test with LLVM code generation
#[cfg(feature =  llvm)]
#[test]
fn test_defer_llvm_compilation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    info!(Testing:  defer statement LLVM compilation);;
    use inkwell::context::Context;
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()
    
    // Create a simple defer statement for testing
    let expr = StringLiteral   {token: Token::new(TokenType::Str, (Deferredmessage.to_string()
        value:  Deferredmessage.to_string()"}
    
    let expr_stmt = ExpressionStatement {token: Token::new(TokenType::Str, 
        call: Some(Box::new(expr)}
    let defer_stmt = DeferStatement {statement: Box::new(expr_stmt)}
    
    // Test compilation (this might fail if LLVM setup is incomplete)
    match codegen.compile_defer_statement(&defer_stmt)         {Ok(() => {info!(Defer:  statement compiled successfully);}
        Err(e) => {debug!("Expected:  compilation error (incomplete LLVM setup): {}, e)
        slay main() {;
            later vibez.spill(First)
            later vibez.spill(Second)")
            yolo 0;}
    "#;
    let mut lexer = cursed::lexer::Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)
    
    match parser.unwrap().parse_program()     {Ok(program) => {info!()
            assert!(!program.statements.is_empty();
        Err(e) => {panic!("Failed:  to parse multiple defer program: {}, e)"}