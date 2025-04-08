//! Facts statement type (constants)

use std::any::Any;
use crate::ast::traits::{Node, Statement, Expression};

/// FactsStatement represents a constant declaration
pub struct FactsStatement {
    pub token: String,
    pub name: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl Node for FactsStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        format!("facts {} = {}", self.name.string(), self.value.string())
    }
}

impl Statement for FactsStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}