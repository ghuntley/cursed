use std::any::Any;
use crate::ast::{Node, Expression};
use crate::lexer::Token;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Identifier;
    
    #[test]
    fn test_pointer_type() {
        let target_type = Box::new(Identifier {
            token: "normie".to_string(),
            value: "normie".to_string(),
        }) as Box<dyn Expression>;
        
        let pointer_type = PointerType {
            token: Token::At,
            target_type,
        };
        
        assert_eq!(pointer_type.string(), "@normie");
    }
    
    #[test]
    fn test_pointer_dereference() {
        let pointer = Box::new(Identifier {
            token: "ptr".to_string(),
            value: "ptr".to_string(),
        }) as Box<dyn Expression>;
        
        let dereference = PointerDereference {
            token: Token::At,
            pointer,
        };
        
        assert_eq!(dereference.string(), "@ptr");
    }
}