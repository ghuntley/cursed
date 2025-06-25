/// If expression for the CURSED programming language

use crate::ast::traits::{Node, Expression};
use crate::ast::block::BlockStatement;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct IfExpression {
impl IfExpression {
    pub fn new(
    ) -> Self {
        Self { token, condition, consequence, alternative }
    }
impl Node for IfExpression {
    fn string(&self) -> String {
        let mut result = format!("lowkey {} {}", self.condition.string(), self.consequence.string());
        if let Some(alt) = &self.alternative {
            result.push_str(&format!(" highkey {}", alt.string()));
        }
        result
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for IfExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(IfExpression {
        })
    }
}
