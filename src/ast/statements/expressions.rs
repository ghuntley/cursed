//! AST node for expression statements in the CURSED language.
//!
//! This module defines the AST representation for expression statements, which
//! are expressions that are used as standalone statements. In many programming
//! languages, including CURSED, expressions can be used as statements when their
//! side effects are desired but their value is not used.
//!
//! Common examples include function calls, assignments, and increment/decrement
//! operations when used as standalone lines of code.

use std::any::Any;
use crate::ast::{Node, Statement, Expression};

/// Represents an expression used as a statement in the AST.
///
/// An expression statement is an expression that appears as a standalone statement
/// in the code. The expression is evaluated for its side effects, but its result value
/// is not used or stored.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// vibez.println("Hello, world!")  // Function call as statement
/// x = 5                          // Assignment as statement
/// counter++                      // Increment as statement
/// ```
///
/// The AST would have an `ExpressionStatement` wrapping each expression.
pub struct ExpressionStatement {
    pub token: String,
    pub expression: Option<Box<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        if let Some(expr) = &self.expression {
            expr.string()
        } else {
            String::new()
        }
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}