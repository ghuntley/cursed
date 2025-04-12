//! Operator precedence handling for the parser
//!
//! This module defines the precedence levels used in the Pratt parsing
//! algorithm for expression parsing. Precedence determines the order of
//! operations in expressions with multiple operators.

/// Defines precedence levels for expression parsing in the Pratt parser
///
/// The precedence levels are ordered from lowest to highest, with higher
/// precedence operators binding more tightly. This is used to ensure
/// expressions are parsed with the correct operator associativity.
///
/// For example, in the expression `a + b * c`, the multiplication
/// has higher precedence than addition, so `b * c` is parsed first.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Index,       // array[index]
    Dot,         // object.property
}
