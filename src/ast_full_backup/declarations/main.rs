/// Declaration AST nodes for the CURSED programming language

use crate::ast::traits::{Node, Statement, Expression};
use crate::ast::identifiers::Identifier;
use crate::ast::block::BlockStatement;
use crate::ast::expressions::Parameter;
use crate::ast::TypeExpression;
use crate::error::SourceLocation;
use std::any::Any;

/// Function declaration (slay name(params) return_type { body })
#[derive(Debug, Clone)]
pub struct FunctionStatement {
impl FunctionStatement {
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    pub fn with_visibility(mut self, is_public: bool) -> Self {
        self.is_public = is_public;
        self
    pub fn with_async(mut self, is_async: bool) -> Self {
        self.is_async = is_async;
        self
    }
}

impl Node for FunctionStatement {
    fn string(&self) -> String {
        let params: Vec<String> = self.parameters.iter()
            .map(|p| p.string())
            .collect();
        
        let mut result = format!("slay {}({})", self.to_string().string(), params.join(", "));
        
        if let Some(ret_type) = &self.return_type {
            result.push_str(&format!(" {}", ret_type.string()));
        result.push(' ');
        result.push_str(&self.body.string());
        
        result
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for FunctionStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(FunctionStatement {
        })
    }
}

/// Function declaration alias for compatibility
pub type FunctionDeclaration = FunctionStatement;

/// Struct declaration (squad name { fields... })
#[derive(Debug, Clone)]
pub struct SquadStatement {
impl SquadStatement {
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    pub fn with_visibility(mut self, is_public: bool) -> Self {
        self.is_public = is_public;
        self
    }
}

impl Node for SquadStatement {
    fn string(&self) -> String {
        let fields: Vec<String> = self.fields.iter()
            .map(|f| format!("  {}", f.string()))
            .collect();
        
        format!("squad {} {{\n{}\n}}", self.to_string().string(), fields.join("\n"))
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for SquadStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Interface declaration (collab name { methods... })
#[derive(Debug, Clone)]
pub struct CollabStatement {
impl CollabStatement {
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    pub fn with_visibility(mut self, is_public: bool) -> Self {
        self.is_public = is_public;
        self
    }
}

impl Node for CollabStatement {
    fn string(&self) -> String {
        let methods: Vec<String> = self.methods.iter()
            .map(|m| format!("  {}", m.string()))
            .collect();
        
        format!("collab {} {{\n{}\n}}", self.to_string().string(), methods.join("\n"))
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for CollabStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Field within a struct
#[derive(Debug, Clone)]
pub struct FieldStatement {
impl FieldStatement {
    pub fn new(token: String, name: Identifier, type_name: Identifier) -> Self {
        Self { token, name, type_name }
    }
impl Node for FieldStatement {
    fn string(&self) -> String {
        format!("{} {}", self.to_string().string(), self.type_name.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

/// Method declaration within an interface
#[derive(Debug, Clone)]
pub struct MethodDeclaration {
impl MethodDeclaration {
    pub fn new(
    ) -> Self {
        Self {
        }
    }
impl Node for MethodDeclaration {
    fn string(&self) -> String {
        let params: Vec<String> = self.parameters.iter()
            .map(|p| p.string())
            .collect();
        
        let mut result = format!("{}({})", self.to_string().string(), params.join(", "));
        
        if let Some(ret_type) = &self.return_type {
            result.push_str(&format!(" {}", ret_type.string()));
        result
    fn token_literal(&self) -> String {
        self.to_string().token_literal()
    }
}

/// Type parameter for generics
#[derive(Debug, Clone)]
pub struct TypeParameter {
impl TypeParameter {
    pub fn new(token: crate::lexer::Token, name: String) -> Self {
        Self {
        }
    }
    
    pub fn with_constraints(token: crate::lexer::Token, name: String, constraints: Vec<String>) -> Self {
        Self {
        }
    }
impl Node for TypeParameter {
    fn string(&self) -> String {
        if self.constraints.is_empty() {
            self.to_string().clone()
        } else {
            format!("{}: {}", self.to_string(), self.constraints.join(" + "))
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

/// Generic constraint specification
#[derive(Debug, Clone, PartialEq)]
pub struct GenericConstraint {
    /// Name of the constraint (e.g., "Clone", "Debug", "Comparable")
    /// Type parameters bound by this constraint
    /// Additional constraint bounds
impl GenericConstraint {
    /// Create a new generic constraint
    pub fn new(constraint_name: String, type_parameters: Vec<String>) -> Self {
        Self {
        }
    }

    /// Create constraint with bounds
    pub fn with_bounds(constraint_name: String, type_parameters: Vec<String>, bounds: Vec<String>) -> Self {
        Self {
        }
    }
impl Node for GenericConstraint {
    fn string(&self) -> String {
        format!("{}: {}", self.type_parameters.join(", "), self.constraint_name)
    fn token_literal(&self) -> String {
        self.constraint_name.clone()
    }
}

/// Parameter statement for function parameters
#[derive(Debug, Clone)]
pub struct ParameterStatement {
impl ParameterStatement {
    pub fn new(name: String, param_type: Option<String>) -> Self {
        Self { name, param_type }
    }
impl Node for ParameterStatement {
    fn string(&self) -> String {
        if let Some(param_type) = &self.param_type {
            format!("{} {}", self.to_string(), param_type)
        } else {
            self.to_string().clone()
        }
    }

    fn token_literal(&self) -> String {
        self.to_string().clone()
    }
}

/// Field declaration for structs and interfaces
#[derive(Debug, Clone)]
pub struct FieldDeclaration {
#[derive(Debug, Clone)]
pub enum Visibility {
impl FieldDeclaration {
    pub fn new(name: String, field_type: String) -> Self {
        Self {
        }
    }
impl Node for FieldDeclaration {
    fn string(&self) -> String {
        format!("{} {}", self.to_string(), self.field_type)
    fn token_literal(&self) -> String {
        self.to_string().clone()
    }
}

/// Struct declaration alias
pub type StructDeclaration = SquadStatement;

/// Interface declaration alias
pub type InterfaceDeclaration = CollabStatement;

/// Interface method declaration
#[derive(Debug, Clone)]
pub struct InterfaceMethod {
impl InterfaceMethod {
    pub fn new(name: String, parameters: Vec<Parameter>, return_type: Option<Box<dyn Expression>>) -> Self {
        Self {
        }
    }
impl Node for InterfaceMethod {
    fn string(&self) -> String {
        let params: Vec<String> = self.parameters.iter().map(|p| p.string()).collect();
        let return_str = match &self.return_type {
        format!("{}({}){}", self.to_string(), params.join(", "), return_str)
    fn token_literal(&self) -> String {
        self.to_string().clone()
    }
}

/// Struct field declaration
#[derive(Debug, Clone)]
pub struct StructField {
impl StructField {
    pub fn new(name: String, field_type: Box<dyn Expression>) -> Self {
        Self {
        }
    }
impl Node for StructField {
    fn string(&self) -> String {
        format!("{}: {}", self.to_string(), self.field_type.string())
    fn token_literal(&self) -> String {
        self.to_string().clone()
    }
}

/// Variable declaration
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
impl VariableDeclaration {
    pub fn new(name: String) -> Self {
        Self {
        }
    }
impl Node for VariableDeclaration {
    fn string(&self) -> String {
        let mut result = self.to_string().clone();
        if let Some(t) = &self.var_type {
            result.push_str(&format!(": {}", t.string()));
        }
        if let Some(v) = &self.initial_value {
            result.push_str(&format!(" = {}", v.string()));
        }
        result
    fn token_literal(&self) -> String {
        self.to_string().clone()
    }
}

/// Constant declaration
#[derive(Debug, Clone)]
pub struct ConstantDeclaration {
impl ConstantDeclaration {
    pub fn new(name: String, const_type: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self {
        }
    }
impl Node for ConstantDeclaration {
    fn string(&self) -> String {
        format!("const {}: {} = {}", self.to_string(), self.const_type.string(), self.value.string())
    fn token_literal(&self) -> String {
        self.to_string().clone()
    }
}

/// Enum declaration
#[derive(Debug, Clone)]
pub struct EnumDeclaration {
/// Enum variant
#[derive(Debug, Clone)]
pub struct EnumVariant {
impl EnumDeclaration {
    pub fn new(name: String, variants: Vec<EnumVariant>) -> Self {
        Self {
        }
    }
impl Node for EnumDeclaration {
    fn string(&self) -> String {
        let variants: Vec<String> = self.variants.iter().map(|v| v.to_string().clone()).collect();
        format!("enum {} {{ {} }}", self.to_string(), variants.join(", "))
    fn token_literal(&self) -> String {
        self.to_string().clone()
    }
}

/// Package declaration
#[derive(Debug, Clone)]
pub struct PackageDeclaration {
impl PackageDeclaration {
    pub fn new(name: String) -> Self {
        Self {
        }
    }
impl Node for PackageDeclaration {
    fn string(&self) -> String {
        format!("package {}", self.to_string())
    fn token_literal(&self) -> String {
        self.to_string().clone()
    }
}


