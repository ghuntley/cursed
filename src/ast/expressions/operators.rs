//! AST nodes for operator expressions in the CURSED language.
//!
//! This module defines the AST representations for operator expressions:
//! - Prefix operators: unary operators that appear before their operand (e.g., `-x`, `!y`)
//! - Infix operators: binary operators that appear between two operands (e.g., `x + y`, `a == b`)
//!
//! These expressions form the basis for arithmetic, logical, and comparison operations
//! in the language.

use std::any::Any;
use crate::ast::{Node, Expression};
use crate::lexer::token::Token;

/// Represents a prefix (unary) operator expression in the AST.
///
/// A prefix expression consists of an operator followed by an expression.
/// The operator is applied to the result of evaluating the expression.
///
/// # Supported operators
///
/// - `-`: Numeric negation
/// - `!`: Logical negation (boolean NOT)
/// - `*`: Dereference operator (for pointers)
/// - `&`: Address-of operator (creates references)
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// -5
/// !isValid
/// *pointer
/// ```
///
/// The AST would have a `PrefixExpression` with:
/// - operator: "-", "!", or "*"
/// - right: the operand expression
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("{} {}", self.operator, self.right.string())
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}

    fn node_type(&self) -> &str {
        "PrefixExpression"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_prefix_expression(&self) -> bool {
        true
    }

    fn as_prefix_expression(&self) -> Option<(String, &dyn Expression)> {
        Some((self.operator.clone(), self.right.as_ref()))
    }
}

/// Represents an infix (binary) operator expression in the AST.
///
/// An infix expression consists of a left expression, an operator, and a right expression.
/// The operator is applied to the results of evaluating both expressions.
///
/// # Supported operators
///
/// Arithmetic operators:
/// - `+`: Addition
/// - `-`: Subtraction
/// - `*`: Multiplication
/// - `/`: Division
/// - `%`: Modulo
///
/// Comparison operators:
/// - `==`: Equal to
/// - `!=`: Not equal to
/// - `<`: Less than
/// - `>`: Greater than
/// - `<=`: Less than or equal to
/// - `>=`: Greater than or equal to
///
/// Logical operators:
/// - `&&`: Logical AND
/// - `||`: Logical OR
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// x + y
/// age >= 18
/// isAdmin && hasPermission
/// ```
///
/// The AST would have an `InfixExpression` with:
/// - left: the left operand expression
/// - operator: "+", ">=", or "&&"
/// - right: the right operand expression
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("{} {} {}", self.left.string(), self.operator, self.right.string())
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}

    fn node_type(&self) -> &str {
        "InfixExpression"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_infix_expression(&self) -> bool {
        true
    }

    fn as_infix_expression(&self) -> Option<(&dyn Expression, String, &dyn Expression)> {
        Some((self.left.as_ref(), self.operator.clone(), self.right.as_ref()))
    }
}