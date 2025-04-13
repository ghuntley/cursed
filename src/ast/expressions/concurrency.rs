//! Concurrency-related expressions in the CURSED language.
//!
//! This module contains AST nodes for goroutine expressions and other
//! concurrency primitives like channels and synchronization.

use crate::ast::traits::{Node, Expression};
use crate::lexer::Token;
use std::any::Any;

/// A goroutine expression (stan expression)
// No derives for now due to trait object issues
pub struct StanExpression {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Node for StanExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("stan {}", self.expression.string())
    }
}

impl Expression for StanExpression {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}