// Panic and recovery statements for CURSED language

use std::any::Any;
use crate::ast::traits::{Node, Statement, Expression};
use crate::error::SourceLocation;

#[derive(Debug, Clone)]
pub struct PanicStatement {
impl PanicStatement {
    pub fn new(message: Option<Box<dyn Expression>>) -> Self {
        Self {
        }
    }
impl Node for PanicStatement {
    fn string(&self) -> String {
        match &self.message {
        }
    }
    
    fn token_literal(&self) -> String {
        "panic".to_string()
    }
}

impl Statement for PanicStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct RecoveryStatement {
impl RecoveryStatement {
    pub fn new(body: Vec<Box<dyn Statement>>) -> Self {
        Self {
        }
    }
    
    pub fn with_handler(body: Vec<Box<dyn Statement>>, handler: Vec<Box<dyn Statement>>) -> Self {
        Self {
        }
    }
impl Node for RecoveryStatement {
    fn string(&self) -> String {
        let mut result = "recover {\n".to_string();
        for stmt in &self.body {
            result.push_str(&format!("  {}\n", stmt.string()));
        }
        result.push('}');
        
        if let Some(handler) = &self.handler {
            result.push_str(" handle {\n");
            for stmt in handler {
                result.push_str(&format!("  {}\n", stmt.string()));
            }
            result.push('}');
        result
    fn token_literal(&self) -> String {
        "recover".to_string()
    }
}

impl Statement for RecoveryStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
