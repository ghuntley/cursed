//! AST nodes for where clauses in generic type declarations.
//!
//! Where clauses provide a flexible way to specify constraints on generic
//! type parameters, allowing complex constraints that go beyond simple interface bounds.

use crate::ast::{Node, Statement};
use crate::ast::declarations::GenericConstraint;
use crate::lexer::token::Token;
use std::any::Any;

/// Represents a where clause that specifies constraints on type parameters
/// 
/// Where clauses allow separating the type parameter declaration from the constraints,
/// providing better readability for complex generic functions.
/// 
/// Example: `where T: Display + Clone, U: Into<String>`
#[derive(Clone, Debug)]
pub struct WhereClause {
    pub token: Token,                        // The 'where' token
    pub constraints: Vec<GenericConstraint>, // List of constraints
}

impl WhereClause {
    /// Creates a new WhereClause with the given token and constraints
    pub fn new(token: Token, constraints: Vec<GenericConstraint>) -> Self {
        Self {
            token,
            constraints,
        }
    }

    /// Checks if the where clause is empty (no constraints)
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    /// Gets the number of constraints in this where clause
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }
}

impl Node for WhereClause {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        if self.constraints.is_empty() {
            return String::new();
        }

        let constraint_strings: Vec<String> = self.constraints
            .iter()
            .map(|constraint| constraint.string())
            .collect();

        format!("where {}", constraint_strings.join(", "))
    }
}

impl Statement for WhereClause {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
