use cursed::ast::*;
use cursed::lexer::Token;

// AST factory for creating test AST nodes
//
// This module provides utilities for creating AST nodes for testing purposes.


/// Factory for creating test AST nodes
pub struct AstFactory;

impl AstFactory {
    /// Create a new integer literal expression
    pub fn int_literal(value: i64) -> Box<dyn Expression> {
        Box::new(IntegerLiteral {
            token: value.to_string(),
            value,
        })
    }
    
    /// Create a new string literal expression  
    pub fn string_literal(value: String) -> Box<dyn Expression> {
        Box::new(StringLiteral {
            token: value.clone(),
            value,
        })
    }
    
    /// Create a new boolean literal expression
    pub fn bool_literal(value: bool) -> Box<dyn Expression> {
        Box::new(BooleanLiteral {
            token: if value { "based".to_string() } else { "cap".to_string() },
            value,
        })
    }
    
    /// Create a new identifier expression
    pub fn identifier(name: String) -> Box<dyn Expression> {
        Box::new(Identifier {
            token: name.clone(),
            value: name,
        })
    }
}
