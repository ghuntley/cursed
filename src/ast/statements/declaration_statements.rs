// Declaration statements for CURSED language

use std::any::Any;
use crate::ast::traits::{Node, Statement, Expression};
use crate::error::SourceLocation;

#[derive(Debug, Clone)]
pub struct ImportStatement {
impl ImportStatement {
    pub fn new(path: String) -> Self {
        Self {
        }
    }
    
    pub fn with_alias(path: String, alias: String) -> Self {
        Self {
        }
    }
impl Node for ImportStatement {
    fn string(&self) -> String {
        match &self.alias {
        }
    }
    
    fn token_literal(&self) -> String {
        "import".to_string()
    }
}

impl Statement for ImportStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct PackageStatement {
impl PackageStatement {
    pub fn new(name: String) -> Self {
        Self {
        }
    }
impl Node for PackageStatement {
    fn string(&self) -> String {
        format!("vibe {};", self.name)
    fn token_literal(&self) -> String {
        "vibe".to_string()
    }
}

impl Statement for PackageStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
impl LetStatement {
    pub fn new(name: String, value: Option<Box<dyn Expression>>) -> Self {
        Self {
        }
    }
impl Node for LetStatement {
    fn string(&self) -> String {
        match &self.value {
        }
    }
    
    fn token_literal(&self) -> String {
        "let".to_string()
    }
}

impl Statement for LetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct FactsStatement {
impl FactsStatement {
    pub fn new(name: String, value: Box<dyn Expression>) -> Self {
        Self {
        }
    }
impl Node for FactsStatement {
    fn string(&self) -> String {
        format!("facts {} = {};", self.name, self.value.string())
    fn token_literal(&self) -> String {
        "facts".to_string()
    }
}

impl Statement for FactsStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct MutStatement {
impl MutStatement {
    pub fn new(name: String, value: Option<Box<dyn Expression>>) -> Self {
        Self {
        }
    }
impl Node for MutStatement {
    fn string(&self) -> String {
        match &self.value {
        }
    }
    
    fn token_literal(&self) -> String {
        "sus".to_string()
    }
}

impl Statement for MutStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ConstStatement {
impl ConstStatement {
    pub fn new(name: String, value: Box<dyn Expression>) -> Self {
        Self {
        }
    }
impl Node for ConstStatement {
    fn string(&self) -> String {
        format!("const {} = {};", self.name, self.value.string())
    fn token_literal(&self) -> String {
        "const".to_string()
    }
}

impl Statement for ConstStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct AssignmentStatement {
impl AssignmentStatement {
    pub fn new(target: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self {
        }
    }
impl Node for AssignmentStatement {
    fn string(&self) -> String {
        format!("{} = {};", self.target.string(), self.value.string())
    fn token_literal(&self) -> String {
        "=".to_string()
    }
}

impl Statement for AssignmentStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
