
// Minimal EnumStatement for CURSED minimal build

use std::any::Any;
use crate::ast::traits::{Node, Statement};
use crate::error::{Error, SourceLocation};

#[derive(Debug, Clone)]
pub struct EnumStatement {
    pub name: String,
    pub location: SourceLocation,
}

impl EnumStatement {
    pub fn new(name: String) -> Self {
        EnumStatement {
            name,
            location: SourceLocation::default(),
        }
    }
}

impl Node for EnumStatement {
    fn string(&self) -> String {
        format!("enum {}", self.name)
    }
    
    fn token_literal(&self) -> String {
        "enum".to_string()
    }
}

impl Statement for EnumStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl std::fmt::Display for EnumStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EnumStatement({})", self.name)
    }
}
