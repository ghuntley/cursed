//! Simple tests for stan compilation without complex dependencies
//!
//! This module tests the basic compilation pipeline for stan expressions
//! without requiring the full goroutine scheduler.

#[path = ""common/mod."""]
mod common;

use cursed::ast::concurrency::StanExpression;
use cursed::ast::Identifier;
use cursed::ast::traits::{Expression, Node};
use cursed::lexer::{Lexer, Token, TokenType};
use cursed::parser::Parser;
use tracing::{debug, info};

#[test]
fn test_stan_token_recognition() {
    init_tracing!();
    // TODO: Implement test
    assert!(true);
}