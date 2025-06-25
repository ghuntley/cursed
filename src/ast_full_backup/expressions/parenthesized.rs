/// Parenthesized expression for the CURSED programming language
/// 
/// Represents expressions wrapped in parentheses for explicit precedence.

use crate::ast::traits::{Node, Expression};
use std::any::Any;

/// Parenthesized expression ((expression))
#[derive(Debug, Clone)]
pub struct ParenthesizedExpression {
impl ParenthesizedExpression {
    pub fn new(token: String, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
impl Node for ParenthesizedExpression {
    fn string(&self) -> String {
        format!("({})", self.expression.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for ParenthesizedExpression {
    fn as_any(&self) -> &dyn Any {
        self


    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
