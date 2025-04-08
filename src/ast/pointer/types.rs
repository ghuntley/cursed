use std::any::Any;
use crate::ast::{Node, Expression};
use crate::lexer::token::Token;

/// PointerType represents a pointer type (@T)
pub struct PointerType {
    pub token: Token, // Token::At
    pub target_type: Box<dyn Expression>,
}

impl Node for PointerType {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("@{}", self.target_type.string())
    }
}

impl Expression for PointerType {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}