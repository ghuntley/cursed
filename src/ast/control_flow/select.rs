//! AST nodes for select statements in the CURSED language.
//!
//! Select statements allow non-blocking operations on multiple channels,
//! providing a way to choose between multiple channel operations.

use std::any::Any;
use crate::ast::traits::{Node, Statement, Expression};
use crate::lexer::Token;

/// Represents a select statement in the AST.
///
/// A select statement blocks until one of its cases can proceed, then executes
/// that case. It's similar to a switch statement but for channel operations.
pub struct SelectStatement {
    pub token: Token,
    pub cases: Vec<SelectCase>,
    pub default: Option<DefaultCase>,
}

impl Node for SelectStatement {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        let mut result = "choose {".to_string();
        
        for case in &self.cases {
            result.push_str(&format!("\n  {}", case.string()));
        }
        
        if let Some(default) = &self.default {
            result.push_str(&format!("\n  {}", default.string()));
        }
        
        result.push_str("\n}");
        result
    }
}

impl Statement for SelectStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

/// Represents a case in a select statement.
pub struct SelectCase {
    pub token: Token,
    pub communication: Box<dyn Expression>, // Send or receive expression
    pub statements: Vec<Box<dyn Statement>>,
}

impl SelectCase {
    pub fn string(&self) -> String {
        let mut result = format!("mood {}: ", self.communication.string());
        
        for stmt in &self.statements {
            result.push_str(&format!("\n    {}", stmt.string()));
        }
        
        result
    }
}

/// Represents a default case in a select statement.
pub struct DefaultCase {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl DefaultCase {
    pub fn string(&self) -> String {
        let mut result = "basic: ".to_string();
        
        for stmt in &self.statements {
            result.push_str(&format!("\n    {}", stmt.string()));
        }
        
        result
    }
}

/// Represents a timeout case in a select statement.
pub struct TimeoutCase {
    pub token: Token,
    pub duration: Box<dyn Expression>,
    pub statements: Vec<Box<dyn Statement>>,
}

impl TimeoutCase {
    pub fn string(&self) -> String {
        let mut result = format!("timeout {}: ", self.duration.string());
        
        for stmt in &self.statements {
            result.push_str(&format!("\n    {}", stmt.string()));
        }
        
        result
    }
}

/// Helper function to create a new select statement
pub fn new_select_statement(
    token: Token,
    cases: Vec<SelectCase>,
    default: Option<DefaultCase>,
) -> SelectStatement {
    SelectStatement {
        token,
        cases,
        default,
    }
}

/// Helper function to create a new select case
pub fn new_select_case(
    token: Token,
    communication: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
) -> SelectCase {
    SelectCase {
        token,
        communication,
        statements,
    }
}

/// Helper function to create a new default case
pub fn new_default_case(
    token: Token,
    statements: Vec<Box<dyn Statement>>,
) -> DefaultCase {
    DefaultCase {
        token,
        statements,
    }
}

/// Helper function to create a new timeout case
pub fn new_timeout_case(
    token: Token,
    duration: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
) -> TimeoutCase {
    TimeoutCase {
        token,
        duration,
        statements,
    }
}
