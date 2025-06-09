//! AST nodes for type-related expressions in the CURSED language.
//!
//! This module defines the AST representations for expressions that involve
//! operations on types, such as type conversions (casts). These expressions
//! allow programmers to explicitly convert values from one type to another
//! when the type system permits such conversions.
//!
//! Type conversion expressions appear in CURSED using `as` syntax, similar to Rust,
//! rather than the function-like syntax of some other languages.

use std::any::Any;
use crate::ast::traits::{Node, Expression};
use crate::lexer::token::Token;

/// Represents a type conversion (cast) expression in the AST.
///
/// A type conversion expression explicitly converts a value from one type to another.
/// It consists of an expression to be converted and a target type name.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// intValue as float    // Convert an integer to a floating-point number
/// byteValue as rune    // Convert a byte to a Unicode code point
/// pointer as uintptr   // Convert a pointer to an unsigned integer
/// ```
///
/// The AST would have a `TypeConversionExpression` with:
/// - expression: the value to be converted
/// - type_name: the target type name
#[derive(Debug)]
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
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeConversionExpression {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
            type_name: self.type_name.clone(),
        })
    }
}

/// Represents a map type expression in the AST.
///
/// A map type expression specifies the key and value types for a map.
/// This is used in type declarations and type annotations.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// tea[tea]normie         // Map of strings to integers
/// tea[normie]tea         // Map of integers to strings  
/// tea[CustomKey]CustomValue  // Map of custom types
/// ```
///
/// The AST would have a `MapTypeExpression` with:
/// - key_type: the type expression for the key
/// - value_type: the type expression for the value
pub struct MapTypeExpression {
    pub token: Token,
    pub key_type: Box<dyn Expression>,
    pub value_type: Box<dyn Expression>,
}

impl std::fmt::Debug for MapTypeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapTypeExpression")
            .field("token", &self.token.token_literal())
            .field("key_type", &self.key_type.string())
            .field("value_type", &self.value_type.string())
            .finish()
    }
}

impl Node for MapTypeExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("tea[{}]{}", 
            self.key_type.string(), 
            self.value_type.string())
    }
}

impl Expression for MapTypeExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(MapTypeExpression {
            token: self.token.clone(),
            key_type: self.key_type.clone_box(),
            value_type: self.value_type.clone_box(),
        })
    }
}

impl MapTypeExpression {
    /// Creates a new map type expression
    ///
    /// # Arguments
    ///
    /// * `token` - The token that starts the map type (usually 'tea')
    /// * `key_type` - An expression representing the key type
    /// * `value_type` - An expression representing the value type
    ///
    /// # Returns
    ///
    /// A new `MapTypeExpression` instance
    pub fn new(
        token: Token,
        key_type: Box<dyn Expression>,
        value_type: Box<dyn Expression>,
    ) -> Self {
        MapTypeExpression {
            token,
            key_type,
            value_type,
        }
    }
    
    /// Returns the key type expression
    pub fn get_key_type(&self) -> &dyn Expression {
        self.key_type.as_ref()
    }
    
    /// Returns the value type expression
    pub fn get_value_type(&self) -> &dyn Expression {
        self.value_type.as_ref()
    }
}