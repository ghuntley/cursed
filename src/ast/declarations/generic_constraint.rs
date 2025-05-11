//! AST nodes for generic type constraints used in generic types.
//! 
//! Generic constraints define bounds on type parameters, specifying
//! what interfaces or requirements a type parameter must satisfy.

use crate::ast::{Node, Statement};
use std::any::Any;
use crate::lexer::token::Token;

/// Represents a constraint on a type parameter in a generic type or function declaration
/// 
/// GenericConstraints specify requirements that a type parameter must meet,
/// typically by implementing specific interfaces.
#[derive(Clone, Debug)]
pub struct GenericConstraint {
    pub token: Token,                // The constraint token
    pub parameter_name: String,      // The name of the type parameter being constrained
    pub interface_name: String,      // The name of the interface that must be implemented
}

impl GenericConstraint {
    /// Creates a new GenericConstraint with the given token, parameter name, and interface name.
    pub fn new(token: Token, parameter_name: String, interface_name: String) -> Self {
        Self {
            token,
            parameter_name,
            interface_name,
        }
    }
}

impl Node for GenericConstraint {
    fn token_literal(&self) -> String {
        format!("{}:{}", self.parameter_name, self.interface_name)
    }

    fn string(&self) -> String {
        format!("{}:{}", self.parameter_name, self.interface_name)
    }
}

impl Statement for GenericConstraint {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}