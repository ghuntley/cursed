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