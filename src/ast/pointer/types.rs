//! AST node for pointer type expressions in the CURSED language.
//!
//! This module defines the AST representation for pointer types, which allow
//! references to memory addresses. Pointer types are denoted with the @ symbol
//! followed by the target type, like @int or @Person.
//!
//! Pointers enable more efficient handling of large data structures and allow for
//! creating more complex data structures like linked lists and trees.

use crate::ast::{Expression, Node};
use crate::lexer::token::Token;
use std::any::Any;

/// Represents a pointer type expression in the AST.
///
/// A pointer type expression denotes a type that references another type by storing
/// its memory address. It consists of the @ symbol followed by the target type.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// var x @int      // Declare x as a pointer to an integer
/// type Node @Node // Recursive type definition using pointers
/// func process(data @[]byte) { ... }  // Function taking a pointer to a byte slice
/// ```
///
/// The AST would have a `PointerType` with the target type expression representing
/// the pointed-to type (int, Node, []byte).
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
