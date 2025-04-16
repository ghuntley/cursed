//! If Expression Adapter for Cursed
//! 
//! This module provides an adapter to treat if statements as expressions for
//! the LLVM code generator. It wraps an IfStatement from control_flow module.

use crate::ast::traits::{Node, Expression};
use crate::ast::control_flow::conditionals::IfStatement;
use std::any::Any;

/// An adapter that wraps an IfStatement and presents it as an Expression.
/// 
/// This is used by the LLVM code generator to support the implementation of
/// if expressions, where the result of the if can be used as a value.
pub struct IfExpression {
    /// The wrapped if statement
    pub statement: IfStatement,
}

impl IfExpression {
    /// Create a new if expression adapter from an if statement
    pub fn new(statement: IfStatement) -> Self {
        IfExpression { statement }
    }
    
    /// Get a reference to the wrapped statement
    pub fn statement(&self) -> &IfStatement {
        &self.statement
    }
    
    /// Get the condition expression
    pub fn condition(&self) -> &Box<dyn Expression> {
        &self.statement.condition
    }
    
    /// Get the consequence block
    pub fn consequence(&self) -> &Vec<Box<dyn crate::ast::traits::Statement>> {
        &self.statement.consequence.statements
    }
    
    /// Get the alternative block, if any
    pub fn alternative(&self) -> Option<&Vec<Box<dyn crate::ast::traits::Statement>>> {
        self.statement.alternative.as_ref().map(|alt| &alt.statements)
    }
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.statement.token_literal()
    }
    
    fn string(&self) -> String {
        self.statement.string()
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}