//! Go statement for spawning goroutines.
//!
//! This module defines the AST node for a go statement, which is used
//! to spawn a new goroutine (lightweight thread) in the CURSED language.

use crate::ast::traits::{Node, Statement, Expression};
use crate::lexer::Token;
use std::any::Any;

/// A go statement that spawns a goroutine
// No derives for now due to trait object issues
pub struct GoStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Node for GoStatement {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("go {};", self.expression.string())
    }
}

impl Statement for GoStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}