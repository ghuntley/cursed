
// Minimal TypeAliasStatement for CURSED minimal build

use std::any::Any;
use crate::ast::traits::{Node, Statement};
use crate::error::{CursedError, SourceLocation};

#[derive(Debug, Clone)]
pub struct TypeAliasStatement {
    pub name: String,
    pub location: SourceLocation,
}

impl TypeAliasStatement {
    pub fn new(name: String) -> Self {
        TypeAliasStatement {
            name,
            location: SourceLocation::default(),
        }
    }
}

impl Node for TypeAliasStatement {
    fn string(&self) -> String {
        format!("type {} = ...", self.name)
    }
    
    fn token_literal(&self) -> String {
        "type".to_string()
    }
}

impl Statement for TypeAliasStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl std::fmt::Display for TypeAliasStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeAliasStatement({})", self.name)
    }
}
