/// Declaration AST nodes for the CURSED programming language

use crate::ast::traits::{Node, Statement, Expression};
use crate::ast::identifiers::Identifier;
use crate::ast::block::BlockStatement;
use crate::ast::expressions::Parameter;
use crate::error::SourceLocation;
use std::any::Any;

/// Function declaration (slay name(params) return_type { body })
#[derive(Debug, Clone)]
pub struct FunctionStatement {
    pub token: String,
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Box<dyn Expression>>,
    pub body: BlockStatement,
    pub type_parameters: Vec<TypeParameter>,
    pub generic_constraints: Vec<GenericConstraint>,
    pub location: Option<SourceLocation>,
    pub is_public: bool,
    pub is_async: bool,
}

impl FunctionStatement {
    pub fn new(
        token: String,
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Box<dyn Expression>>,
        body: BlockStatement,
    ) -> Self {
        Self {
            token,
            name,
            parameters,
            return_type,
            body,
            type_parameters: Vec::new(),
            generic_constraints: Vec::new(),
            location: None,
            is_public: false,
            is_async: false,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_visibility(mut self, is_public: bool) -> Self {
        self.is_public = is_public;
        self
    }

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
        
        let mut result = format!("slay {}({})", self.name.string(), params.join(", "));
        
        if let Some(ret_type) = &self.return_type {
            result.push_str(&format!(" {}", ret_type.string()));
        }
        
        result.push(' ');
        result.push_str(&self.body.string());
        
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for FunctionStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(FunctionStatement {
            token: self.token.clone(),
            name: self.name.clone(),
            parameters: self.parameters.clone(),
            return_type: self.return_type.as_ref().map(|t| t.clone_box()),
            body: self.body.clone(),
            type_parameters: self.type_parameters.clone(),
            generic_constraints: self.generic_constraints.clone(),
            location: self.location.clone(),
            is_public: self.is_public,
            is_async: self.is_async,
        })
    }
}

/// Function declaration alias for compatibility
pub type FunctionDeclaration = FunctionStatement;

/// Struct declaration (squad name { fields... })
#[derive(Debug, Clone)]
pub struct SquadStatement {
    pub token: String,
    pub name: Identifier,
    pub fields: Vec<FieldStatement>,
    pub type_parameters: Vec<TypeParameter>,
    pub generic_constraints: Vec<GenericConstraint>,
    pub location: Option<SourceLocation>,
    pub is_public: bool,
}

impl SquadStatement {
    pub fn new(
        token: String,
        name: Identifier,
        fields: Vec<FieldStatement>,
    ) -> Self {
        Self {
            token,
            name,
            fields,
            type_parameters: Vec::new(),
            generic_constraints: Vec::new(),
            location: None,
            is_public: false,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

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
        
        format!("squad {} {{\n{}\n}}", self.name.string(), fields.join("\n"))
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for SquadStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Interface declaration (collab name { methods... })
#[derive(Debug, Clone)]
pub struct CollabStatement {
    pub token: String,
    pub name: Identifier,
    pub methods: Vec<MethodDeclaration>,
    pub type_parameters: Vec<TypeParameter>,
    pub location: Option<SourceLocation>,
    pub is_public: bool,
}

impl CollabStatement {
    pub fn new(
        token: String,
        name: Identifier,
        methods: Vec<MethodDeclaration>,
    ) -> Self {
        Self {
            token,
            name,
            methods,
            type_parameters: Vec::new(),
            location: None,
            is_public: false,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

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
        
        format!("collab {} {{\n{}\n}}", self.name.string(), methods.join("\n"))
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for CollabStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Field within a struct
#[derive(Debug, Clone)]
pub struct FieldStatement {
    pub token: String,
    pub name: Identifier,
    pub type_name: Identifier,
}

impl FieldStatement {
    pub fn new(token: String, name: Identifier, type_name: Identifier) -> Self {
        Self { token, name, type_name }
    }
}

impl Node for FieldStatement {
    fn string(&self) -> String {
        format!("{} {}", self.name.string(), self.type_name.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

/// Method declaration within an interface
#[derive(Debug, Clone)]
pub struct MethodDeclaration {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Box<dyn Expression>>,
}

impl MethodDeclaration {
    pub fn new(
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Box<dyn Expression>>,
    ) -> Self {
        Self {
            name,
            parameters,
            return_type,
        }
    }
}

impl Node for MethodDeclaration {
    fn string(&self) -> String {
        let params: Vec<String> = self.parameters.iter()
            .map(|p| p.string())
            .collect();
        
        let mut result = format!("{}({})", self.name.string(), params.join(", "));
        
        if let Some(ret_type) = &self.return_type {
            result.push_str(&format!(" {}", ret_type.string()));
        }
        
        result
    }

    fn token_literal(&self) -> String {
        self.name.token_literal()
    }
}

/// Type parameter for generics
#[derive(Debug, Clone)]
pub struct TypeParameter {
    pub token: String,
    pub name: String,
    pub constraints: Vec<String>,
}

impl TypeParameter {
    pub fn new(token: crate::lexer::Token, name: String) -> Self {
        Self {
            token: token.literal,
            name,
            constraints: Vec::new(),
        }
    }
    
    pub fn with_constraints(token: crate::lexer::Token, name: String, constraints: Vec<String>) -> Self {
        Self {
            token: token.literal,
            name,
            constraints,
        }
    }
}

impl Node for TypeParameter {
    fn string(&self) -> String {
        if self.constraints.is_empty() {
            self.name.clone()
        } else {
            format!("{}: {}", self.name, self.constraints.join(" + "))
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
    pub constraint_name: String,
    /// Type parameters bound by this constraint
    pub type_parameters: Vec<String>,
    /// Additional constraint bounds
    pub bounds: Vec<String>,
}

impl GenericConstraint {
    /// Create a new generic constraint
    pub fn new(constraint_name: String, type_parameters: Vec<String>) -> Self {
        Self {
            constraint_name,
            type_parameters,
            bounds: Vec::new(),
        }
    }

    /// Create constraint with bounds
    pub fn with_bounds(constraint_name: String, type_parameters: Vec<String>, bounds: Vec<String>) -> Self {
        Self {
            constraint_name,
            type_parameters,
            bounds,
        }
    }
}

impl Node for GenericConstraint {
    fn string(&self) -> String {
        format!("{}: {}", self.type_parameters.join(", "), self.constraint_name)
    }

    fn token_literal(&self) -> String {
        self.constraint_name.clone()
    }
}

/// Parameter statement for function parameters
#[derive(Debug, Clone)]
pub struct ParameterStatement {
    pub name: String,
    pub param_type: Option<String>,
}

impl ParameterStatement {
    pub fn new(name: String, param_type: Option<String>) -> Self {
        Self { name, param_type }
    }
}

impl Node for ParameterStatement {
    fn string(&self) -> String {
        if let Some(param_type) = &self.param_type {
            format!("{} {}", self.name, param_type)
        } else {
            self.name.clone()
        }
    }

    fn token_literal(&self) -> String {
        self.name.clone()
    }
}

/// Field declaration for structs and interfaces
#[derive(Debug, Clone)]
pub struct FieldDeclaration {
    pub name: String,
    pub field_type: String,
    pub visibility: Visibility,
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

impl FieldDeclaration {
    pub fn new(name: String, field_type: String) -> Self {
        Self {
            name,
            field_type,
            visibility: Visibility::Public,
        }
    }
}

impl Node for FieldDeclaration {
    fn string(&self) -> String {
        format!("{} {}", self.name, self.field_type)
    }

    fn token_literal(&self) -> String {
        self.name.clone()
    }
}

/// Struct declaration alias
pub type StructDeclaration = SquadStatement;

/// Interface declaration alias
pub type InterfaceDeclaration = CollabStatement;


