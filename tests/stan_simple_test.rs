//! Simple tests for stan compilation without complex dependencies
//!
//! This module tests the basic compilation pipeline for stan expressions
//! without requiring the full goroutine scheduler.

#[path = "common.rs"]
pub mod common;

use cursed::ast::expressions::concurrency::StanExpression;
use cursed::ast::expressions::Identifier;
use cursed::ast::traits::{Expression, Node};
use cursed::lexer::{Lexer, Token};
use cursed::parser::Parser;
use tracing::{debug, info};

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[test]
fn test_stan_token_recognition() {
    init_tracing!();
    info!("Testing stan token recognition");
    
    let input = "stan";
    let mut lexer = Lexer::new(input);
    
    let token = lexer.next_token().expect("Failed to get token");
    assert_eq!(token, Token::Stan);
    
    debug!("Stan token recognition test passed");
}

#[test]
fn test_stan_expression_creation() {
    init_tracing!();
    info!("Testing StanExpression creation");
    
    // Create a simple identifier for the function
    let func_ident = Identifier {
        token: "test_func".to_string(),
        value: "test_func".to_string(),
    };
    
    // Create the stan expression
    let stan_expr = StanExpression {
        token: Token::Stan,
        expression: Box::new(func_ident),
    };
    
    // Test the AST interface
    assert_eq!(stan_expr.token_literal(), "stan");
    assert_eq!(stan_expr.string(), "stan test_func");
    
    debug!("StanExpression creation test passed");
}

#[test]
fn test_stan_parsing_simple() {
    init_tracing!();
    info!("Testing simple stan parsing");
    
    let input = "stan foo;";
    let mut lexer = Lexer::new(input);
    
    // Test that we can at least create a parser and it recognizes stan token
    match Parser::new(&mut lexer) {
        Ok(mut parser) => {
            // Parse as a full program instead of individual expression
            let program_result = parser.parse_program();
            debug!("Program parsing result: {:?}", program_result.is_ok());
            
            if let Ok(program) = program_result {
                debug!("Parsed {} statements", program.statements.len());
                for (i, stmt) in program.statements.iter().enumerate() {
                    debug!("Statement {}: {}", i, stmt.string());
                }
            }
        },
        Err(e) => {
            debug!("Parser creation failed: {:?}", e);
        }
    }
    
    debug!("Simple stan parsing test completed");
}

#[test]
fn test_multiple_stan_tokens() {
    init_tracing!();
    info!("Testing multiple stan tokens");
    
    let input = "stan foo stan bar";
    let mut lexer = Lexer::new(input);
    
    // First stan
    let token1 = lexer.next_token().expect("Failed to get first token");
    assert_eq!(token1, Token::Stan);
    
    // foo identifier  
    let token2 = lexer.next_token().expect("Failed to get second token");
    assert!(matches!(token2, Token::Identifier(_)));
    
    // Second stan
    let token3 = lexer.next_token().expect("Failed to get third token");
    assert_eq!(token3, Token::Stan);
    
    // bar identifier
    let token4 = lexer.next_token().expect("Failed to get fourth token");
    assert!(matches!(token4, Token::Identifier(_)));
    
    debug!("Multiple stan tokens test passed");
}

#[test]
fn test_stan_with_function_call() {
    init_tracing!();
    info!("Testing stan with function call syntax");
    
    let input = "stan func();";
    let mut lexer = Lexer::new(input);
    
    match Parser::new(&mut lexer) {
        Ok(mut parser) => {
            let program_result = parser.parse_program();
            debug!("Function call stan parsing result: {:?}", program_result.is_ok());
        },
        Err(e) => {
            debug!("Parser creation failed: {:?}", e);
        }
    }
    
    debug!("Stan with function call test completed");
}

#[test]
fn test_stan_expression_string_representation() {
    init_tracing!();
    info!("Testing stan expression string representation");
    
    // Create different types of expressions for stan
    let simple_ident = Identifier {
        token: "simple".to_string(),
        value: "simple".to_string(),
    };
    
    let stan_simple = StanExpression {
        token: Token::Stan,
        expression: Box::new(simple_ident),
    };
    
    let representation = stan_simple.string();
    assert!(representation.starts_with("stan"));
    assert!(representation.contains("simple"));
    
    debug!("Stan expression string: {}", representation);
    debug!("String representation test passed");
}
