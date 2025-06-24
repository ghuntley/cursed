// Minimal EnumStatement for CURSED minimal build

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
    fn source_location(&self) -> &SourceLocation {
        &self.location
    }
    
    fn to_string(&self) -> String {
        format!("EnumStatement({})", self.name)
    }
}

impl Statement for EnumStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl std::fmt::Display for EnumStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EnumStatement({})", self.name)
    }
}
