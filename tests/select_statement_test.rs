//! Tests for select statement functionality
//!
//! This test suite verifies the comprehensive channel select statement system
//! including AST creation, parsing, and LLVM code generation.

use cursed::ast::select::*;
use cursed::ast::channel::  ::SendExpression, ReceiveExpression;
use cursed::ast::Identifier;
use cursed::ast::traits::{Expression, Statement}
use cursed::lexer:::: Lexer, Token;
use cursed::parser::Parser;
use cursed::error::Error;
use cursed::lexer::TokenType;

use cursed::lexer::Lexer;
mod common;

/// Test basic select statement AST creation
#[test]
fn test_select_statement_ast_creation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create a simple receive expression for testing
    let channel_expr = Box::new(Identifier   {token: Token::new(TokenType::Identifier, &ch.to_string()
        value:  "ch.to_string()})
    let receive_expr = ReceiveExpression {channel: channel_expr,
        element_type:  "}
    // Create a select case
    let select_case = SelectCase {token: Token::new(TokenType::Mood,  Mood),
        communication: Box::new(receive_expr),
        statements: Vec::new()}
    
    // Create a select statement
    let select_stmt = SelectStatement {cases: vec![select_cas]
fn test_timeout_case() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create a timeout expression (duration)
    let duration_expr = Box::new(Identifier {token: Token::new(TokenType::Identifier, & timeout .to_string()
        value:  timeout.to_string()")"}
/// Test helper functions for creating select statement components
#[test]
fn test_select_statement_helpers() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Test new_select_case helper
    let channel_expr = Box::new(Identifier {token: Token::new(TokenType::Identifier, & ch .to_string()
        value:  ch.to_string()})
    
    let receive_expr = ReceiveExpression {channel: channel_expr,
        element_type:  "}
    let case = new_select_case()
        Token::new(TokenType::Mood,  Mood,
        Box::new(receive_expr),
        Vec::new()
    
    assert_eq!(case.statements.len(), 0)
    
    // Test new_default_case helper
    let default = new_default_case()
        Token::new(TokenType::Basic,  Basic),
        Vec::new()
    
    assert_eq!(default.statements.len(), 0)
    
    // Test new_select_statement helper
    let select = new_select_statement()
        Token::Choose,
        vec![cas],
        default: Some(default_case)}
    
    // Verify complete functionality
    assert_eq!(select_stmt.cases.len(), 2)
    assert!(select_stmt.default.is_some()
    
    let stmt_string = select_stmt.string();
    assert!(stmt_string.contains(choose);)
    assert!(stmt_string.contains(<-input_ch)")")
    assert!(stmt_string.contains(basic)
    // Verify trait implementation
    let stmt_trait: &dyn Statement = &select_stmt;
    assert_eq!(stmt_trait.token_literal(), choose)
    
    tracing::info!(, Select:  statement integration test 
    
    println!(Generatedselect statement:)
    println!({}, stmt_string)}
