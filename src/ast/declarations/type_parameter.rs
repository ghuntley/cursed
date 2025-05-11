//! AST nodes for type parameter declarations used in generic types.
//! 
//! Type parameters are used in generic type declarations, function declarations,
//! and method signatures.

use crate::ast::{Node, Statement};
use std::any::Any;
use crate::lexer::token::Token;

/// Represents a type parameter in a generic type or function declaration
/// 
/// TypeParameters have a name and can include optional constraints.
#[derive(Clone, Debug)]
pub struct TypeParameter {
    pub token: Token, // The parameter token
    pub name: String, // The name of the type parameter
    pub value: String, // The value of the type parameter (for compatibility)
    // TODO: Add constraints field for bounded type parameters
}

impl TypeParameter {
    /// Creates a new TypeParameter with the given token and name.
    /// 
    /// Sets both name and value to the same string for compatibility with existing code.
    pub fn new(token: Token, name: String) -> Self {
        Self {
            token,
            name: name.clone(),
            value: name,
        }
    }
}

impl Node for TypeParameter {
    fn token_literal(&self) -> String {
        self.name.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Statement for TypeParameter {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}