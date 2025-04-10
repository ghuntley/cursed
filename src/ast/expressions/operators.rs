use std::any::Any;
use crate::ast::{Node, Expression};
use crate::lexer::token::Token;

/// PrefixExpression represents a prefix expression
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("{} {}", self.operator, self.right.string())
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}

    fn node_type(&self) -> &str {
        "PrefixExpression"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_prefix_expression(&self) -> bool {
        true
    }

    fn as_prefix_expression(&self) -> Option<(String, &dyn Expression)> {
        Some((self.operator.clone(), self.right.as_ref()))
    }
}

/// InfixExpression represents an infix expression
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("{} {} {}", self.left.string(), self.operator, self.right.string())
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}

    fn node_type(&self) -> &str {
        "InfixExpression"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_infix_expression(&self) -> bool {
        true
    }

    fn as_infix_expression(&self) -> Option<(&dyn Expression, String, &dyn Expression)> {
        Some((self.left.as_ref(), self.operator.clone(), self.right.as_ref()))
    }
}