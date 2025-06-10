/// Identifier expressions for the CURSED programming language
/// 
/// This module contains AST nodes for identifiers and qualified names.

use crate::ast::traits::{Node, Expression};
use crate::lexer::Token;
use std::any::Any;

/// Identifier expression (variable names, function names, etc.)
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: String,
    pub value: String,
}

impl Identifier {
    pub fn new(token: String, value: String) -> Self {
        Self { token, value }
    }
    
    pub fn from_name(name: &str) -> Self {
        Self {
            token: name.to_string(),
            value: name.to_string(),
        }
    }
}

impl Node for Identifier {
    fn string(&self) -> String {
        self.value.clone()
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for Identifier {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Qualified name for accessing members across packages/modules
/// Example: package.Type or module.function
#[derive(Debug, Clone)]
pub struct QualifiedName {
    pub package: String,
    pub name: String,
    pub kind: QualifiedSymbolKind,
}

#[derive(Debug, Clone)]
pub enum QualifiedSymbolKind {
    Type,
    Function,
    Variable,
    Constant,
    Module,
}

impl QualifiedName {
    pub fn new(package: String, name: String, kind: QualifiedSymbolKind) -> Self {
        Self { package, name, kind }
    }
    
    pub fn type_ref(package: &str, name: &str) -> Self {
        Self::new(package.to_string(), name.to_string(), QualifiedSymbolKind::Type)
    }
    
    pub fn function_ref(package: &str, name: &str) -> Self {
        Self::new(package.to_string(), name.to_string(), QualifiedSymbolKind::Function)
    }
}

impl Node for QualifiedName {
    fn string(&self) -> String {
        format!("{}.{}", self.package, self.name)
    }

    fn token_literal(&self) -> String {
        self.name.clone()
    }
}

impl Expression for QualifiedName {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Type identifier specifically for type references
#[derive(Debug, Clone)]
pub struct TypeIdentifier {
    pub name: String,
    pub token: String,
    pub type_args: Vec<Box<dyn Expression>>,
}

impl TypeIdentifier {
    pub fn new(name: String, token: String) -> Self {
        Self {
            name,
            token,
            type_args: Vec::new(),
        }
    }
    
    pub fn with_type_args(name: String, token: String, type_args: Vec<Box<dyn Expression>>) -> Self {
        Self {
            name,
            token,
            type_args,
        }
    }
    
    pub fn is_generic(&self) -> bool {
        !self.type_args.is_empty()
    }
}

impl Node for TypeIdentifier {
    fn string(&self) -> String {
        if self.type_args.is_empty() {
            self.name.clone()
        } else {
            let args: Vec<String> = self.type_args.iter()
                .map(|arg| arg.string())
                .collect();
            format!("{}<{}>", self.name, args.join(", "))
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for TypeIdentifier {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeIdentifier {
            name: self.name.clone(),
            token: self.token.clone(),
            type_args: self.type_args.iter().map(|arg| arg.clone_box()).collect(),
        })
    }
}

/// Package identifier for import statements
#[derive(Debug, Clone)]
pub struct PackageIdentifier {
    pub name: String,
    pub path: String,
}

impl PackageIdentifier {
    pub fn new(name: String, path: String) -> Self {
        Self { name, path }
    }
}

impl Node for PackageIdentifier {
    fn string(&self) -> String {
        if self.name != self.path {
            format!("{} \"{}\"", self.name, self.path)
        } else {
            format!("\"{}\"", self.path)
        }
    }

    fn token_literal(&self) -> String {
        self.name.clone()
    }
}

/// Self identifier for referring to current instance in methods
#[derive(Debug, Clone)]
pub struct SelfIdentifier {
    pub token: String,
}

impl SelfIdentifier {
    pub fn new() -> Self {
        Self {
            token: "self".to_string(),
        }
    }
}

impl Node for SelfIdentifier {
    fn string(&self) -> String {
        "self".to_string()
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for SelfIdentifier {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Default for SelfIdentifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for creating common identifiers
pub fn ident(name: &str) -> Identifier {
    Identifier::from_name(name)
}
    
    pub fn type_ident(name: &str) -> TypeIdentifier {
    TypeIdentifier::new(name.to_string(), name.to_string())
}
    
    pub fn qualified(package: &str, name: &str) -> QualifiedName {
    QualifiedName::new(package.to_string(), name.to_string(), QualifiedSymbolKind::Type)
}
