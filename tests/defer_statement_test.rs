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
use tracing::{info, debug;}
use cursed::lexer::Lexer;
#[test]
fn test_defer_statement_ast() {
    // TODO: Implement test
    assert!(true);}
    let expr_stmt = ExpressionStatement {token: Token::new(TokenType::Identifier, & test.to_string()"))"
    info!("Info message");
            later vibez.spill(Deferredmessage);"#;"
    match parser.unwrap().parse_program()     {Ok(program} => {debug!(", "  program successfully)'t fail)
        Err(e) => {panic!("Failed:  to parse program with defer statement: {), e)"}
        slay test_defer_order() {later vibez.spill(, "}}")
            later vibez.spill(Seconddefer)Thirddefer)""
            vibez.spill(Normalstatement);""
    ", :  program with multiple defer statements)"
        Err(e) => {panic!(Failed:  to parse program with multiple defer statements: {), e)"}"
    let input = r#"        slay test_defer_variables() {;"}
            later vibez.spill(")"
            yolo x + y;}#    ""
    match parser.unwrap().parse_program()     {Ok(program} => {debug!(Parsed:  program with defer and variables}")")
            lowkey (based) {later vibez.spill(Inner defer , 1}")"
    #;""
            info!(Nested:  defer test passed)}""
        Err(e) => {panic!("))"
    let input = r#
            lowkey (condition) {later vibez.spill(, " return defer)"
    , "  program with early return and defer)"
        Err(e) => {panic!()
        value:  Deferredmessage.to_string()}""
        Err(e) => {debug!(", :  compilation error (incomplete LLVM setup}: {), e)"
            later vibez.spill(Second)""
    ";"
        Err(e) => {panic!(, "  to parse multiple defer program: {), e)"""}"