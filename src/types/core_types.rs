
use crate::crate::types::source_location::SourceLocation;
use crate::ast::crate::types::Type;

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
    pub is_public: bool,
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Clone)]
pub struct InterfaceMethod {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: Option<Type>,
    pub value: Option<Expression>,
    pub is_mutable: bool,
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

// Placeholder for Expression type
#[derive(Debug, Clone)]
pub enum Expression {
    // Add variants as needed
    Literal(String),
}

#[derive(Debug, Clone)]
pub struct ImportDeclaration {
    pub module_path: String,
    pub items: Vec<String>,
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Clone)]
pub struct PackageDeclaration {
    pub name: String,
    pub version: Option<String>, 
    pub location: Option<SourceLocation>,
}
