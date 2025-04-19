//! AST nodes for special expression types in the CURSED language.
//!
//! This module defines the AST representations for expressions that have special
//! semantics in the language:
//! - Assignment expressions: updating variables (e.g., `x = 5`)
//! - Struct instantiation expressions: creating struct instances ("be_like" in CURSED)
//! - Default case expressions: handling the default case in switch statements ("basic" in CURSED)
//! - Type assertion expressions: checking and converting interface values to concrete types ("value.(Type)")
//!
//! These expressions provide important functionality beyond typical operators and literals.

use super::identifiers::Identifier;
use crate::ast::{Expression, Node};
use crate::lexer::token::Token;
use std::any::Any;

// The TypeConversionExpression is actually defined in types.rs, so we should remove this duplicate

/// Represents an assignment expression in the AST.
///
/// An assignment expression updates the value of a variable by assigning a new value to it.
/// It consists of a target identifier (lvalue) and a value expression (rvalue).
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// x = 5
/// name = "zoomer"
/// isValid = checkCondition()
/// ```
///
/// The AST would have an `AssignmentExpression` with:
/// - name: the target identifier (`x`, `name`, `isValid`)
/// - value: the value expression (`5`, `"zoomer"`, `checkCondition()`)
pub struct AssignmentExpression {
    pub token: String, // Token::Assign
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for AssignmentExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!(
            "{} {} {}",
            self.name.string(),
            self.token_literal(),
            self.value.string()
        )
    }
}

impl Expression for AssignmentExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Represents a struct instantiation expression in the AST (using the "be_like" keyword in CURSED).
///
/// A struct instantiation creates a new instance of a struct type with specified field values.
/// It can include optional generic type parameters for generic structs.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// // Regular struct instantiation
/// user := be_like Person {
///     name: "zoomer",
///     age: 21
/// }
///
/// // Generic struct instantiation
/// pair := be_like Pair[int, string] {
///     first: 42,
///     second: "meaning"
/// }
/// ```
///
/// The AST would have a `BeLikeExpression` with:
/// - struct_name: the name of the struct type ("Person", "Pair")
/// - type_arguments: any generic type parameters (empty for regular structs, [int, string] for generic)
/// - fields: the field names and values as key-value pairs
pub struct BeLikeExpression {
    pub token: String,
    pub struct_name: Identifier,
    pub type_arguments: Vec<Box<dyn Expression>>, // Generic type arguments [normie], [tea, normie], etc.
    pub fields: Vec<(String, Box<dyn Expression>)>,
}

impl Node for BeLikeExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = format!("be_like {}", self.struct_name.string());

        // Format type arguments if present
        if !self.type_arguments.is_empty() {
            let type_args: Vec<String> =
                self.type_arguments.iter().map(|arg| arg.string()).collect();
            out.push_str(&format!("[{}]", type_args.join(", ")));
        }

        if !self.fields.is_empty() {
            out.push_str(" {");
            let fields_str: Vec<String> = self
                .fields
                .iter()
                .map(|(name, expr)| format!("{}: {}", name, expr.string()))
                .collect();
            out.push_str(&fields_str.join(", "));
            out.push_str("}");
        }

        out
    }
}

impl Expression for BeLikeExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Represents the default case in a switch statement in the AST (using the "basic" keyword in CURSED).
///
/// The default case provides a fallback for when no other case in a switch statement matches
/// the input value. In CURSED, this is indicated with the "basic" keyword.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// vibe_check status {
///     mood "good": {
///         vibez.println("Nice!")
///     }
///     basic: {
///         vibez.println("Unknown status")
///     }
/// }
/// ```
///
/// The AST would have a `DefaultCase` expression within the switch statement
/// representing the "basic" clause.
pub struct DefaultCase {
    pub token: String,
}

impl Node for DefaultCase {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        "basic".to_string()
    }
}

impl Expression for DefaultCase {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// Type assertion moved to a separate module: src/ast/expressions/type_assertion.rs
