//! AST nodes for switch statements in the CURSED language.

use std::any::Any;
use crate::ast::traits::{Node, Statement, Expression};

/// Represents a switch statement in the AST.
///
/// A switch statement branches based on the value of an expression, matching it
/// against multiple possible cases.
pub struct SwitchStatement {
    pub token: String,
    pub value: Box<dyn Expression>,
    pub cases: Vec<SwitchCase>,
    pub default: Option<SwitchCase>,
}

impl Node for SwitchStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        let mut result = format!("vibe_check {} {{", self.value.string());
        
        for case in &self.cases {
            result.push_str(&format!("\n  {}", case.string()));
        }
        
        if let Some(default) = &self.default {
            result.push_str(&format!("\n  {}", default.string()));
        }
        
        result.push_str("\n}");
        result
    }
}

impl Statement for SwitchStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

/// Represents a case in a switch statement.
pub struct SwitchCase {
    pub value: Box<dyn Expression>,
    pub statements: Vec<Box<dyn Statement>>,
}

impl SwitchCase {
    pub fn string(&self) -> String {
        let mut result = if self.value.string() == "default" {
            "basic: ".to_string()
        } else {
            format!("mood {}: ", self.value.string())
        };
        
        for stmt in &self.statements {
            result.push_str(&stmt.string());
        }
        
        result
    }
}