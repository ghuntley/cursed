/// Function call expressions for the CURSED programming language

use crate::ast::traits::{Node, Expression};
use std::any::Any;

/// Function call expression (function(args...))
#[derive(Debug, Clone)]
pub struct CallExpression {
impl CallExpression {
    pub fn new(
    ) -> Self {
        Self {
        }
    }
impl Node for CallExpression {
    fn string(&self) -> String {
        let args: Vec<String> = self.arguments.iter()
            .map(|arg| arg.string())
            .collect();
        format!("{}({})", self.function.string(), args.join(", "))
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for CallExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(CallExpression {
        })
    }
}
