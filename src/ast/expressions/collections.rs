//! Expression types for collections (arrays and maps/hashes)

use std::any::Any;
use crate::ast::traits::{Node, Expression};
use crate::lexer::token::Token;

/// ArrayLiteral represents an array literal expression
pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<Box<dyn Expression>>,
}

impl Node for ArrayLiteral {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        let elements = self.elements
            .iter()
            .map(|e| e.string())
            .collect::<Vec<String>>()
            .join(", ");
        
        format!("[{}]", elements)
    }
}

impl Expression for ArrayLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

/// HashLiteral represents a hash literal expression
pub struct HashLiteral {
    pub token: Token,
    pub pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>,
}

impl Node for HashLiteral {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        let pairs = self.pairs
            .iter()
            .map(|(k, v)| format!("{}: {}", k.string(), v.string()))
            .collect::<Vec<String>>()
            .join(", ");
        
        format!("{{{}}}", pairs)
    }
}

impl Expression for HashLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}