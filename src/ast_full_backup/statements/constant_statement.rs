
// Minimal ConstantStatement for CURSED minimal build

use std::any::Any;
use crate::ast::traits::{Node, Statement};
use crate::error::{CursedError, SourceLocation};

#[derive(Debug, Clone)]
pub struct ConstantStatement {
impl ConstantStatement {
    pub fn new(name: String) -> Self {
        ConstantStatement {
        }
    }
impl Node for ConstantStatement {
    fn string(&self) -> String {
        format!("facts {} = ...", self.name)
    fn token_literal(&self) -> String {
        "facts".to_string()
    }
}

impl Statement for ConstantStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl std::fmt::Display for ConstantStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantStatement({})", self.name)
    }
}
