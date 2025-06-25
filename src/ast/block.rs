/// Block statements for the CURSED programming language
/// 
/// This module contains the block statement which groups multiple statements together.

use crate::ast::traits::{Node, Statement};
use std::any::Any;

/// Block statement ({ statement1; statement2; ... })
#[derive(Debug, Clone)]
pub struct BlockStatement {
impl BlockStatement {
    pub fn new(token: String, statements: Vec<Box<dyn Statement>>) -> Self {
        Self { token, statements }
    }
    
    pub fn empty() -> Self {
        Self {
        }
    }
    
    pub fn with_statements(statements: Vec<Box<dyn Statement>>) -> Self {
        Self {
        }
    }
    
    pub fn add_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    pub fn len(&self) -> usize {
        self.statements.len()
    }
}

impl Node for BlockStatement {
    fn string(&self) -> String {
        let mut result = String::from("{\n");
        
        for statement in &self.statements {
            result.push_str(&format!("  {}\n", statement.string()));
        result.push('}');
        result
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for BlockStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(BlockStatement {
        })
    }
}

/// Block expression that can return a value
#[derive(Debug, Clone)]
pub struct BlockExpression {
impl BlockExpression {
    pub fn new(
    ) -> Self {
        Self {
        }
    }
    
    pub fn empty() -> Self {
        Self {
        }
    }
    
    pub fn with_return(
    ) -> Self {
        Self {
        }
    }
impl Node for BlockExpression {
    fn string(&self) -> String {
        let mut result = String::from("{\n");
        
        for statement in &self.statements {
            result.push_str(&format!("  {}\n", statement.string()));
        if let Some(return_val) = &self.return_value {
            result.push_str(&format!("  {}\n", return_val.string()));
        result.push('}');
        result
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl crate::ast::traits::Expression for BlockExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn crate::ast::traits::Expression> {
        Box::new(BlockExpression {
        })
    }
}

/// Alias for Block (commonly used in tests)
pub type Block = BlockStatement;

/// Helper functions for creating blocks
pub fn block(statements: Vec<Box<dyn Statement>>) -> BlockStatement {
    BlockStatement::with_statements(statements)
    pub fn empty_block() -> BlockStatement {
    BlockStatement::empty()
}
