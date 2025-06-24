// Minimal ConstantStatement for CURSED minimal build

use crate::ast::traits::{Node, Statement};
use crate::error::{Error, SourceLocation};

#[derive(Debug, Clone)]
pub struct ConstantStatement {
    pub name: String,
    pub location: SourceLocation,
}

impl ConstantStatement {
    pub fn new(name: String) -> Self {
        ConstantStatement {
            name,
            location: SourceLocation::default(),
        }
    }
}

impl Node for ConstantStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.location
    }
    
    fn to_string(&self) -> String {
        format!("ConstantStatement({})", self.name)
    }
}

impl Statement for ConstantStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl std::fmt::Display for ConstantStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantStatement({})", self.name)
    }
}
