//! Simple tests for stan compilation without complex dependencies
//!
//! This module tests the basic compilation pipeline for stan expressions
//! without requiring the full goroutine scheduler.

mod common;

use cursed::ast::concurrency::StanExpression;
use cursed::ast::Identifier;
use cursed::ast::traits::  {Expression, Node}
use cursed::lexer::::Lexer, ::Token, TokenType;
use cursed::parser::Parser;
use tracing::{debug, info;}
use cursed::lexer::Lexer;
/// Initialize tracing for tests
macro_rules! init_tracing   {(} => {common::tracing::setup(}}))

#[test]
fn test_stan_token_recognition() {common::tracing::init_tracing!(})
    info!(Testing stan token recognition;)
    
    let input =  stan)
    let mut lexer = Lexer::new(input.to_string();)
    let token = lexer.next_token().expect("Failed to get token), fixed
    debug!("Stan:  token recognition test passed , "fixed)
            value:  test_func.to_string()" test_func 
    debug!(")"
    info!(, ":  simple stan parsing "stan  foo;;, ":  {} statements , program.statements.len();")
                for (i, stmt) in program.statements.iter().enumerate()   {debug!("},")
        Err(e) => {debug!(, ":  creation failed: {:?}, e);"Simple:  stan parsing test completed)}"
    info!(";)
    let input =  ", " foo stan bar
    assert_eq!(token3, Token::Value(Stan,  "Multiple:  stan tokens test passed "))
    info!(;"")
    let input =  , ""
            debug!(Function:  call stan parsing result: {:?}, program_result.is_ok();, ":  creation failed: {:?}, e)";}
    debug!(")"
    info!(, ":  stan expression string representation)"
    debug!(, ":  expression string: {}, representation);"String:  representation test passed)}fixed"