//! AST nodes for field declarations and related structures.

use crate::ast::expressions::identifiers::Identifier;
use crate::ast::{Node, Statement};
use std::any::Any;

/// Represents a field in a struct or interface declaration.
///
/// Fields have a name and a type, and can be used in struct, interface, and other
/// composite type declarations.
pub struct Field {
    pub name: Identifier,
    pub type_name: Identifier,
}

/// Represents a parameter in a function declaration
///
/// Parameters have a name and a type, and are used in function signatures.
pub struct Parameter {
    pub token: String, // Usually the parameter name token
    pub name: Identifier,
    pub param_type: Box<dyn crate::ast::traits::Expression>,
}

impl Node for Parameter {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{} {}", self.name.string(), self.param_type.string())
    }
}

impl Statement for Parameter {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}