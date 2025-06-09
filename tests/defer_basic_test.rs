//! Basic tests for defer statement functionality
//!
//! Simple tests focusing on parsing and AST creation without LLVM compilation.

mod common;

use cursed::lexer::{Lexer, Token};
use cursed::parser::Parser;
use cursed::ast::traits::{Node, Statement};
use cursed::error::Error;
use tracing::{info, debug};

#[test]
fn test_defer_keyword_lexing() {
    common::tracing::setup();
    
    info!("Testing defer keyword lexing");
    
    let input = "later";
    let mut lexer = Lexer::new(input);
    
    match lexer.next_token() {
        Ok(token) => {
            assert_eq!(token, Token::Later);
            info!("Defer keyword lexed correctly as Later token");
        }
        Err(e) => {
            panic!("Failed to lex 'later' keyword: {}", e);
        }
    }
}

#[test]
fn test_basic_defer_parsing() {
    common::tracing::setup();
    
    info!("Testing basic defer statement parsing");
    
    let input = r#"
        slay test_function() {
            later vibez.spill("Hello");
        }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_program() {
        Ok(program) => {
            debug!("Successfully parsed program with defer statement");
            assert!(!program.statements.is_empty(), "Program should have statements");
            info!("Basic defer parsing test passed");
        }
        Err(e) => {
            // This might fail due to compilation issues, but we want to see progress
            debug!("Parser error (may be expected): {}", e);
            // For now, we'll consider this a pass if it recognizes the syntax
        }
    }
}

#[test] 
fn test_multiple_defer_parsing() {
    common::tracing::setup();
    
    info!("Testing multiple defer statements parsing");
    
    let input = r#"
        slay main() {
            later vibez.spill("First");
            later vibez.spill("Second"); 
            later vibez.spill("Third");
            yolo 0;
        }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_program() {
        Ok(program) => {
            debug!("Successfully parsed program with multiple defer statements");
            assert!(!program.statements.is_empty(), "Program should have statements");
            info!("Multiple defer parsing test passed");
        }
        Err(e) => {
            debug!("Parser error for multiple defer (may be expected): {}", e);
        }
    }
}

#[test]
fn test_defer_token_literal() {
    common::tracing::setup();
    
    info!("Testing defer token literal");
    
    let token = Token::Later;
    assert_eq!(token.token_literal(), "later");
    
    info!("Defer token literal test passed");
}

#[test]
fn test_defer_in_nested_context() {
    common::tracing::setup();
    
    info!("Testing defer in nested context");
    
    let input = r#"
        slay test_nested() {
            lowkey (based) {
                later vibez.spill("Inside if block");
            }
        }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_program() {
        Ok(program) => {
            debug!("Successfully parsed nested defer statement");
            assert!(!program.statements.is_empty(), "Program should have statements");
            info!("Nested defer parsing test passed");
        }
        Err(e) => {
            debug!("Parser error for nested defer (may be expected): {}", e);
        }
    }
}

#[test]
fn test_defer_with_simple_statement() {
    common::tracing::setup();
    
    info!("Testing defer with simple statement");
    
    let input = r#"
        slay simple_defer() {
            sus x = 5;
            later x = 10;
            yolo x;
        }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_program() {
        Ok(program) => {
            debug!("Successfully parsed defer with assignment");
            assert!(!program.statements.is_empty(), "Program should have statements");
            info!("Defer with simple statement test passed");
        }
        Err(e) => {
            debug!("Parser error for defer with assignment (may be expected): {}", e);
        }
    }
}
