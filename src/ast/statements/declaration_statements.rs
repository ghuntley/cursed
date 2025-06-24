// Declaration statements for CURSED language

use std::any::Any;
use crate::ast::traits::{Node, Statement, Expression};
use crate::error::SourceLocation;

#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub path: String,
    pub alias: Option<String>,
    pub location: SourceLocation,
}

impl ImportStatement {
    pub fn new(path: String) -> Self {
        Self {
            path,
            alias: None,
            location: SourceLocation::default(),
        }
    }
    
    pub fn with_alias(path: String, alias: String) -> Self {
        Self {
            path,
            alias: Some(alias),
            location: SourceLocation::default(),
        }
    }
}

impl Node for ImportStatement {
    fn string(&self) -> String {
        match &self.alias {
            Some(alias) => format!("import \"{}\" as {};", self.path, alias),
            None => format!("import \"{}\";", self.path),
        }
    }
    
    fn token_literal(&self) -> String {
        "import".to_string()
    }
}

impl Statement for ImportStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct PackageStatement {
    pub name: String,
    pub location: SourceLocation,
}

impl PackageStatement {
    pub fn new(name: String) -> Self {
        Self {
            name,
            location: SourceLocation::default(),
        }
    }
}

impl Node for PackageStatement {
    fn string(&self) -> String {
        format!("vibe {};", self.name)
    }
    
    fn token_literal(&self) -> String {
        "vibe".to_string()
    }
}

impl Statement for PackageStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: String,
    pub value: Option<Box<dyn Expression>>,
    pub location: SourceLocation,
}

impl LetStatement {
    pub fn new(name: String, value: Option<Box<dyn Expression>>) -> Self {
        Self {
            name,
            value,
            location: SourceLocation::default(),
        }
    }
}

impl Node for LetStatement {
    fn string(&self) -> String {
        match &self.value {
            Some(expr) => format!("let {} = {};", self.name, expr.string()),
            None => format!("let {};", self.name),
        }
    }
    
    fn token_literal(&self) -> String {
        "let".to_string()
    }
}

impl Statement for LetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct FactsStatement {
    pub name: String,
    pub value: Box<dyn Expression>,
    pub location: SourceLocation,
}

impl FactsStatement {
    pub fn new(name: String, value: Box<dyn Expression>) -> Self {
        Self {
            name,
            value,
            location: SourceLocation::default(),
        }
    }
}

impl Node for FactsStatement {
    fn string(&self) -> String {
        format!("facts {} = {};", self.name, self.value.string())
    }
    
    fn token_literal(&self) -> String {
        "facts".to_string()
    }
}

impl Statement for FactsStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct MutStatement {
    pub name: String,
    pub value: Option<Box<dyn Expression>>,
    pub location: SourceLocation,
}

impl MutStatement {
    pub fn new(name: String, value: Option<Box<dyn Expression>>) -> Self {
        Self {
            name,
            value,
            location: SourceLocation::default(),
        }
    }
}

impl Node for MutStatement {
    fn string(&self) -> String {
        match &self.value {
            Some(expr) => format!("sus {} = {};", self.name, expr.string()),
            None => format!("sus {};", self.name),
        }
    }
    
    fn token_literal(&self) -> String {
        "sus".to_string()
    }
}

impl Statement for MutStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ConstStatement {
    pub name: String,
    pub value: Box<dyn Expression>,
    pub location: SourceLocation,
}

impl ConstStatement {
    pub fn new(name: String, value: Box<dyn Expression>) -> Self {
        Self {
            name,
            value,
            location: SourceLocation::default(),
        }
    }
}

impl Node for ConstStatement {
    fn string(&self) -> String {
        format!("const {} = {};", self.name, self.value.string())
    }
    
    fn token_literal(&self) -> String {
        "const".to_string()
    }
}

impl Statement for ConstStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    pub target: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
    pub location: SourceLocation,
}

impl AssignmentStatement {
    pub fn new(target: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self {
            target,
            value,
            location: SourceLocation::default(),
        }
    }
}

impl Node for AssignmentStatement {
    fn string(&self) -> String {
        format!("{} = {};", self.target.string(), self.value.string())
    }
    
    fn token_literal(&self) -> String {
        "=".to_string()
    }
}

impl Statement for AssignmentStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
