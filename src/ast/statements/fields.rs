use crate::ast::expressions::Identifier;
use crate::ast::{Node, Statement};
use std::any::Any;

/// FieldStatement represents a field definition in a struct
pub struct FieldStatement {
    pub token: String, // Usually the identifier token
    pub name: Identifier,
    pub type_name: Identifier,
}

impl Node for FieldStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{} {}", self.name.string(), self.type_name.string())
    }
}

impl Statement for FieldStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
