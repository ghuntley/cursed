// Minimal ModuleStatement for CURSED minimal build

use crate::ast::traits::{Node, Statement};
use crate::error::{Error, SourceLocation};

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
    fn source_location(&self) -> &SourceLocation {
        &self.location
    }
    
    fn to_string(&self) -> String {
        format!("ModuleStatement({})", self.name)
    }
}

impl Statement for ModuleStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl std::fmt::Display for ModuleStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModuleStatement({})", self.name)
    }
}
