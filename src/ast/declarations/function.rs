//! Function declarations for the CURSED language AST.
//!
//! This module defines the AST representation for function declarations,
//! which define reusable blocks of code with parameters and return types.

use crate::ast::declarations::Parameter;
use crate::ast::declarations::type_parameter::{GenericConstraint, TypeParameter};
use crate::ast::expressions::identifiers::Identifier;
use crate::ast::statements::block::BlockStatement;
use crate::ast::{Expression, Node, Statement};
use std::any::Any;

/// FunctionStatement represents a function definition
///
/// In CURSED, functions are declared using the `slay` keyword, followed by
/// an optional set of generic type parameters, a name, parameters, an optional
/// return type, and a block body.
///
/// # Examples
///
/// ```
/// slay fibonacci(n normie) normie {
///     lowkey (n <= 1) {
///         yolo n
///     }
///     yolo fibonacci(n - 1) + fibonacci(n - 2)
/// }
/// ```
pub struct FunctionStatement {
    pub token: String, // Token::Function
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub body: BlockStatement,
    pub return_type: Option<Box<dyn Expression>>,
    pub type_parameters: Vec<TypeParameter>, // Generic type parameters for function [T], [A, B], etc.
    pub generic_constraints: Vec<GenericConstraint>, // Constraints on generic types
}

impl Node for FunctionStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        // Format the function name with optional type parameters
        let type_params_str = if !self.type_parameters.is_empty() {
            let params: Vec<String> = self
                .type_parameters
                .iter()
                .map(|param| param.value.clone())
                .collect();
            format!("[{}] ", params.join(", "))
        } else {
            String::new()
        };

        out.push_str(&format!(
            "{} {}{}",
            self.token_literal(),
            self.name.string(),
            type_params_str
        ));

        // Format parameters
        out.push_str("(");
        let params: Vec<String> = self.parameters.iter().map(|param| param.string()).collect();
        out.push_str(&params.join(", "));
        out.push_str(")");

        // Format return type if any
        if let Some(ret_type) = &self.return_type {
            out.push_str(&format!(": {}", ret_type.string()));
        }

        // Format body
        out.push_str(" ");
        out.push_str(&self.body.string());

        out
    }
}

impl Statement for FunctionStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}