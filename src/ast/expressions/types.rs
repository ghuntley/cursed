//! Expression types related to type operations

use std::any::Any;
use crate::ast::traits::{Node, Expression};
use crate::lexer::token::Token;

/// TypeConversionExpression represents a type conversion expression
pub struct TypeConversionExpression {
    pub token: Token,
    pub expression: Box<dyn Expression>,
    pub type_name: String,
}

impl Node for TypeConversionExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("{} as {}", self.expression.string(), self.type_name)
    }
}

impl Expression for TypeConversionExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}