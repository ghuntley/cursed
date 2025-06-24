// Minimal TypeAliasStatement for CURSED minimal build

use crate::ast::traits::{Node, Statement};
use crate::error::{Error, SourceLocation};

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
    fn source_location(&self) -> &SourceLocation {
        &self.location
    }
    
    fn to_string(&self) -> String {
        format!("TypeAliasStatement({})", self.name)
    }
}

impl Statement for TypeAliasStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl std::fmt::Display for TypeAliasStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeAliasStatement({})", self.name)
    }
}
