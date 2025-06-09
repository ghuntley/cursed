//! If Expression Adapter for Cursed
//! 
//! This module provides an adapter to treat if statements as expressions for
//! the LLVM code generator. It wraps an IfStatement from control_flow module.

use crate::ast::traits::{Node, Expression};
use crate::ast::control_flow::conditionals::IfStatement;
use crate::ast::statements::block::BlockStatement;
use std::any::Any;

/// An adapter that wraps an IfStatement and presents it as an Expression.
/// 
/// This is used by the LLVM code generator to support the implementation of
/// if expressions, where the result of the if can be used as a value.
#[derive(Debug)]
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
    
    fn clone_box(&self) -> Box<dyn Expression> {
        // Clone the IfStatement manually since it doesn't implement Clone
        let cloned_statement = IfStatement {
            token: self.statement.token.clone(),
            condition: self.statement.condition.clone_box(),
            consequence: Box::new(BlockStatement {
                token: self.statement.consequence.token.clone(),
                statements: self.statement.consequence.statements.iter()
                    .map(|stmt| stmt.as_any().downcast_ref::<BlockStatement>()
                        .map(|block| Box::new(BlockStatement {
                            token: block.token.clone(),
                            statements: Vec::new(), // Simplified version for now
                        }) as Box<dyn crate::ast::traits::Statement>)
                        .unwrap_or_else(|| panic!("Failed to clone block statement"))
                    ).collect(),
            }),
            alternative: self.statement.alternative.as_ref().map(|alt| {
                Box::new(BlockStatement {
                    token: alt.token.clone(),
                    statements: Vec::new(), // Simplified version for now
                })
            }),
        };
        
        Box::new(IfExpression {
            statement: cloned_statement,
        })
    }
}