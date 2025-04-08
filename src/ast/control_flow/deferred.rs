use std::any::Any;
use crate::ast::{Node, Statement};

/// LaterStatement represents a scheduled execution (defer, later, etc.)
pub struct LaterStatement {
    pub token: String,
    pub body: Box<dyn Statement>,
}

impl Node for LaterStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        format!("later {}", self.body.string())
    }
}

impl Statement for LaterStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}