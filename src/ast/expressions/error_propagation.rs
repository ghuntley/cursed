use crate::error::CursedError;
use std::any::Any;
use crate::ast::traits::{Expression, Node};

#[derive(Debug, Clone)]
pub struct ErrorPropagation {
impl ErrorPropagation {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
impl Node for ErrorPropagation {
    fn string(&self) -> String {
        format!("{}?", self.expression.string())
    fn token_literal(&self) -> String {
        "?".to_string()
    }
}

impl Expression for ErrorPropagation {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
