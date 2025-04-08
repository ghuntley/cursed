use std::fmt;
use crate::ast::{Node, Statement};

/// Program represents a CURSED program
#[derive(Default)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }

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