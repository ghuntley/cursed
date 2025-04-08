use std::any::Any;
use crate::ast::{Node, Expression};
use crate::lexer::token::Token;

/// PointerDereference represents a pointer dereference expression (@ptr)
pub struct PointerDereference {
    pub token: Token, // Token::At
    pub pointer: Box<dyn Expression>,
}

impl Node for PointerDereference {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("@{}", self.pointer.string())
    }
}

impl Expression for PointerDereference {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}