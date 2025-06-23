/// Core type definitions for the CURSED AST
/// 
/// This module provides essential type definitions that are commonly referenced
/// throughout the AST but may be missing from specific modules.

use crate::ast::traits::{Node, Statement, Expression};
use crate::ast::identifiers::Identifier;
use crate::ast::block::BlockStatement;
use crate::ast::expressions::Parameter;
use crate::ast::declarations::{TypeParameter, GenericConstraint, FieldStatement, MethodDeclaration};
use crate::error::SourceLocation;
use crate::lexer::Token;
use std::any::Any;
use crate::ast::ASTNode;

/// AST root type representing complete program structure
#[derive(Debug, Clone)]
pub struct AST {
    pub program: crate::ast::Program,
}

impl AST {
    pub fn new(program: crate::ast::Program) -> Self {
        Self { program }
    }
}

impl Node for AST {
    fn string(&self) -> String {
        self.program.string()
    }

    fn token_literal(&self) -> String {
        self.program.token_literal()
    }
}

/// Variable declaration statement (sus name = value or facts name = value)
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
    pub type_annotation: Option<Box<dyn Expression>>,
    pub is_mutable: bool,
    pub location: Option<SourceLocation>,
}

impl VariableDeclaration {
    pub fn new(
        token: Token,
        name: Identifier,
        value: Option<Box<dyn Expression>>,
        is_mutable: bool,
    ) -> Self {
        Self {
            token,
            name,
            value,
            type_annotation: None,
            is_mutable,
            location: None,
        }
    }

    pub fn with_type(mut self, type_annotation: Box<dyn Expression>) -> Self {
        self.type_annotation = Some(type_annotation);
        self
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl Node for VariableDeclaration {
    fn string(&self) -> String {
        let keyword = if self.is_mutable { "sus" } else { "facts" };
        let mut result = format!("{} {}", keyword, self.to_string().string());
        
        if let Some(type_ann) = &self.type_annotation {
            result.push_str(&format!(" {}", type_ann.string()));
        }
        
        if let Some(value) = &self.value {
            result.push_str(&format!(" = {}", value.string()));
        }
        
        result
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for VariableDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(VariableDeclaration {
            token: self.token.clone(),
            name: self.to_string().clone(),
            value: self.value.as_ref().map(|v| v.clone_box()),
            type_annotation: self.type_annotation.as_ref().map(|t| t.clone_box()),
            is_mutable: self.is_mutable,
            location: self.location.clone(),
        })
    }
}

/// Constant declaration statement (vibes name = value)
#[derive(Debug, Clone)]
pub struct ConstantDeclaration {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
    pub type_annotation: Option<Box<dyn Expression>>,
    pub location: Option<SourceLocation>,
}

impl ConstantDeclaration {
    pub fn new(
        token: Token,
        name: Identifier,
        value: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            name,
            value,
            type_annotation: None,
            location: None,
        }
    }

    pub fn with_type(mut self, type_annotation: Box<dyn Expression>) -> Self {
        self.type_annotation = Some(type_annotation);
        self
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl Node for ConstantDeclaration {
    fn string(&self) -> String {
        let mut result = format!("vibes {} = {}", self.to_string().string(), self.value.string());
        
        if let Some(type_ann) = &self.type_annotation {
            result = format!("vibes {} {} = {}", self.to_string().string(), type_ann.string(), self.value.string());
        }
        
        result
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ConstantDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ConstantDeclaration {
            token: self.token.clone(),
            name: self.to_string().clone(),
            value: self.value.clone_box(),
            type_annotation: self.type_annotation.as_ref().map(|t| t.clone_box()),
            location: self.location.clone(),
        })
    }
}

/// Import declaration statement
#[derive(Debug, Clone)]
pub struct ImportDeclaration {
    pub token: Token,
    pub path: String,
    pub alias: Option<String>,
    pub location: Option<SourceLocation>,
}

impl ImportDeclaration {
    pub fn new(token: Token, path: String) -> Self {
        Self {
            token,
            path,
            alias: None,
            location: None,
        }
    }

    pub fn with_alias(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl Node for ImportDeclaration {
    fn string(&self) -> String {
        if let Some(alias) = &self.alias {
            format!("yeet {} \"{}\"", alias, self.path)
        } else {
            format!("yeet \"{}\"", self.path)
        }
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ImportDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Package declaration statement
#[derive(Debug, Clone)]
pub struct PackageDeclaration {
    pub token: Token,
    pub name: String,
    pub location: Option<SourceLocation>,
}

impl PackageDeclaration {
    pub fn new(token: Token, name: String) -> Self {
        Self {
            token,
            name,
            location: None,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl Node for PackageDeclaration {
    fn string(&self) -> String {
        format!("vibe {}", self.to_string())
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for PackageDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Interface method definition
#[derive(Debug, Clone)]
pub struct InterfaceMethod {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Box<dyn Expression>>,
    pub location: Option<SourceLocation>,
}

impl InterfaceMethod {
    pub fn new(
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Box<dyn Expression>>,
    ) -> Self {
        Self {
            name,
            parameters,
            return_type,
            location: None,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl Node for InterfaceMethod {
    fn string(&self) -> String {
        let params: Vec<String> = self.parameters.iter()
            .map(|p| p.string())
            .collect();
        
        let mut result = format!("{}({})", self.to_string().string(), params.join(", "));
        
        if let Some(ret_type) = &self.return_type {
            result.push_str(&format!(" {}", ret_type.string()));
        }
        
        result
    }

    fn token_literal(&self) -> String {
        self.to_string().token_literal()
    }
}

/// Struct field definition
#[derive(Debug, Clone)]
pub struct StructField {
    pub name: Identifier,
    pub field_type: Box<dyn Expression>,
    pub visibility: FieldVisibility,
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Clone)]
pub enum FieldVisibility {
    Public,
    Private,
}

impl StructField {
    pub fn new(name: Identifier, field_type: Box<dyn Expression>) -> Self {
        Self {
            name,
            field_type,
            visibility: FieldVisibility::Public,
            location: None,
        }
    }

    pub fn with_visibility(mut self, visibility: FieldVisibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl Node for StructField {
    fn string(&self) -> String {
        format!("{} {}", self.to_string().string(), self.field_type.string())
    }

    fn token_literal(&self) -> String {
        self.to_string().token_literal()
    }
}

/// Module declaration statement
#[derive(Debug, Clone)]
pub struct ModuleDeclaration {
    pub token: Token,
    pub name: String,
    pub body: BlockStatement,
    pub is_public: bool,
    pub location: Option<SourceLocation>,
}

impl ModuleDeclaration {
    pub fn new(token: Token, name: String, body: BlockStatement) -> Self {
        Self {
            token,
            name,
            body,
            is_public: false,
            location: None,
        }
    }

    pub fn with_visibility(mut self, is_public: bool) -> Self {
        self.is_public = is_public;
        self
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl Node for ModuleDeclaration {
    fn string(&self) -> String {
        format!("mod {} {}", self.to_string(), self.body.string())
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ModuleDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// ASTNode type alias for compatibility
pub type ASTNode = crate::ast::AstNode;
