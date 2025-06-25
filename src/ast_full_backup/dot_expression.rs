/// Dot expression for member access

use crate::ast::traits::{Node, Expression};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct DotExpression {
impl DotExpression {
    pub fn new(token: String, left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self { token, left, right }
    }
impl Node for DotExpression {
    fn string(&self) -> String {
        format!("{}.{}", self.left.string(), self.right.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for DotExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(DotExpression {
        })
    }
}
