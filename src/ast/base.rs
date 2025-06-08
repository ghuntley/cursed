//! Core AST structures for CURSED programs
//!
//! This module defines the base structures of the Abstract Syntax Tree,
//! particularly the Program struct that serves as the root of the AST.

use crate::ast::{Node, Statement};
use std::fmt;

/// Represents a complete CURSED program
///
/// A Program is the root node of the Abstract Syntax Tree and contains
/// a sequence of statements that make up the program. This is the
/// top-level structure produced by the parser and consumed by later
/// compilation stages.
#[derive(Default)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    /// Create a new empty program
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    #[tracing::instrument(skip(self), fields(statements_count = self.statements.len()), level = "trace")]
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }

    #[tracing::instrument(skip(self), fields(statements_count = self.statements.len()), level = "trace")]
    fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }
        out
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program {{")?;
        for (i, stmt) in self.statements.iter().enumerate() {
            writeln!(f, "  Statement {}: {}", i, stmt.string())?;
        }
        writeln!(f, "}}")
    }
}
