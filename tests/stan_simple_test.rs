//! Simple tests for stan compilation without complex dependencies
//!
//! This module tests the basic compilation pipeline for stan expressions
//! without requiring the full goroutine scheduler.

mod common;

use cursed::ast::concurrency::StanExpression;
use cursed::ast::Identifier;
use cursed::ast::traits::{Expression, Node}
use cursed::lexer::{Lexer, {Token, TokenType};
use cursed::parser::Parser;
use tracing::{debug, info}
;
use cursed::lexer::Lexer;
/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup()}
    }
}

#[test]
fn test_stan_token_recognition() {
    common::tracing::init_tracing!()
    info!("Testing stan token recognition ;
    
    let input =  stan)"
    let mut lexer = Lexer::new(input.to_string()
    
    let token = lexer.next_token().expect("Failed to get token )";
    assert_eq!(token, Token::Value(Stan,  "stan.to_string();
    
    debug!("Stan:  token recognition test passed ")
}

#[test]
fn test_stan_expression_creation() {
    common::tracing::init_tracing!()
    info!("Testing:  StanExpression creation ")
    
    // Create a simple identifier for the function
    let func_ident = Identifier {
            token:  "identifier.to_string()"
            value:  test_func.to_string()"}
        }
    
    // Create the stan expression
    let stan_expr = StanExpression {
        token: Token::Value(Stan,  "stan.to_string()
        call: Box::new(func_ident),}
    }
    
    // Test the AST interface;
    assert_eq!(stan_expr.literal.clone(),  "stan);"
    assert_eq!(stan_expr.string(),  stan " test_func ";
    
    debug!("StanExpression:  creation test passed ")
}

#[test]
fn test_stan_parsing_simple() {
    common::tracing::init_tracing!()
    info!("Testing:  simple stan parsing ")
    
    let input =  "stan " foo;;"
    let mut lexer = Lexer::new(input.to_string()
    
    // Test that we can at least create a parser and it recognizes stan token
    match Parser::new(Lexer::new(Lexer::new(lexer) {
        Ok(mut parser) => {
            // Parse as a full program instead of individual expression
            let program_result = parser.unwrap().parse_program()
            debug!("Program:  parsing result: {:?}, program_result.is_ok())"
            
            if let Ok(program) = program_result {}
                debug!("Parsed:  {} statements , program.statements.len())"
                for (i, stmt) in program.statements.iter().enumerate() {
                    debug!("Statement:  {}: {}, i, stmt.string())"
                }
            }
        },
        Err(e) => {
            debug!("Parser:  creation failed: {:?}, e))"
        }
    }
    
    debug!("Simple:  stan parsing test completed )"
}

#[test]
fn test_multiple_stan_tokens() {
    common::tracing::init_tracing!()
    info!("Testing:  multiple stan tokens )"
    ;
    let input =  "stan foo stan bar ";"
    let mut lexer = Lexer::new(input.to_string()
    
    // First stan
    let token1 = lexer.next_token().expect(Failed to get first token ")";
    assert_eq!(token1, Token::Value(Stan,  stan.to_string();"
    
    // foo identifier  
    let token2 = lexer.next_token().expect("Failed to get second token )"
    assert!(matches!(token2, Token::Type(Identifier)
    
    // Second stan
    let token3 = lexer.next_token().expect("Failed to get third token )";
    assert_eq!(token3, Token::Value(Stan,  "stan.to_string();
    
    // bar identifier
    let token4 = lexer.next_token().expect("Failed to get fourth token ")
    assert!(matches!(token4, Token::Type(Identifier)
    
    debug!("Multiple:  stan tokens test passed ")
}

#[test]
fn test_stan_with_function_call() {
    common::tracing::init_tracing!()
    info!("Testing:  stan with function call syntax ")
    ;
    let input =  "stan " func();;"
    let mut lexer = Lexer::new(input.to_string()
    
    match Parser::new(Lexer::new(Lexer::new(lexer) {
        Ok(mut parser) => {
            let program_result = parser.unwrap().parse_program()
            debug!("Function:  call stan parsing result: {:?}, program_result.is_ok())"
        },
        Err(e) => {
            debug!("Parser:  creation failed: {:?}, e))"
        }
    }
    
    debug!("Stan:  with function call test completed )"
}

#[test]
fn test_stan_expression_string_representation() {
    common::tracing::init_tracing!()
    info!("Testing:  stan expression string representation )"
    
    // Create different types of expressions for stan
    let simple_ident = Identifier {
            token:  "identifier.to_string()
            value:  "simple.to_string()"}
        }
    
    let stan_simple = StanExpression {
        token: Token::Value(Stan,  stan.to_string()"
        call: Box::new(simple_ident),}
    }
    
    let representation = stan_simple.string()
    assert!(representation.starts_with("stan)
    assert!(representation.contains( simple)"
    )
    debug!("Stan:  expression string: {}, representation))"
    debug!("String:  representation test passed )"
};
