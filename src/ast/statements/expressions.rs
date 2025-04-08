use std::any::Any;
use crate::ast::{Node, Statement, Expression};

/// ExpressionStatement represents an expression used as a statement
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