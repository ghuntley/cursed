//! Type parameter declarations for the CURSED language AST.
//!
//! This module defines the AST representation for generic type parameters used in
//! function, struct, and interface declarations that support generics.

use crate::ast::Node;
use crate::lexer::token::Token;

/// TypeParameter represents a generic type parameter in a type declaration.
///
/// Type parameters allow for generic programming in CURSED, similar to
/// generics in other languages. They are used in function, struct, and interface
/// declarations as placeholders for concrete types that will be provided later.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// be_like Stack[T] squad {
///     items tea[T]
/// }
///
/// slay push[T](stack *Stack[T], item T) {
///     // implementation
/// }
/// ```
///
/// The AST would have `TypeParameter` nodes with value "T".
pub struct TypeParameter {
    pub token: Token,
    pub value: String,
}

impl Node for TypeParameter {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

/// GenericConstraint represents a constraint on a generic type parameter.
///
/// Generic constraints specify that a type parameter must implement certain interfaces,
/// allowing for bounded polymorphism.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// slay toString[T: Stringer](value T) normie {
///     return value.toString()
/// }
/// ```
///
/// The AST would have a `GenericConstraint` where type_parameter is "T" and trait_name is "Stringer".
pub struct GenericConstraint {
    pub token: String,
    pub type_parameter: crate::ast::expressions::identifiers::Identifier,
    pub trait_name: crate::ast::expressions::identifiers::Identifier,
}