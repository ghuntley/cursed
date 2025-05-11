//! Parameter statement for function declarations
//!
//! This module defines the AST representation for parameter statements
//! used in function declarations.

use crate::ast::expressions::identifiers::Identifier;
use crate::ast::{Expression, Node, Statement};
use std::any::Any;

/// ParameterStatement represents a parameter in a function declaration
///
/// Parameters have a name and a type, and are used in function signatures.
pub struct ParameterStatement {
    pub token: String, // Usually the parameter name token
    pub name: Identifier,
    pub type_name: Box<dyn Expression>,
}

impl Node for ParameterStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{} {}", self.name.string(), self.type_name.string())
    }
}

impl Statement for ParameterStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}