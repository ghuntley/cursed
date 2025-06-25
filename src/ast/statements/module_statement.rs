
// Minimal ModuleStatement for CURSED minimal build

use std::any::Any;
use crate::ast::traits::{Node, Statement};
use crate::error::{CursedError, SourceLocation};

#[derive(Debug, Clone)]
pub struct ModuleStatement {
    pub name: String,
    pub location: SourceLocation,
}

impl ModuleStatement {
    pub fn new(name: String) -> Self {
        ModuleStatement {
            name,
            location: SourceLocation::default(),
        }
    }
}

impl Node for ModuleStatement {
    fn string(&self) -> String {
        format!("mod {}", self.name)
    }
    
    fn token_literal(&self) -> String {
        "mod".to_string()
    }
}

impl Statement for ModuleStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl std::fmt::Display for ModuleStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModuleStatement({})", self.name)
    }
}
