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
// Remove import to avoid conflict with type alias below

/// AST root type representing complete program structure
#[derive(Debug, Clone)]
pub struct AST {
impl AST {
    pub fn new(program: crate::ast::Program) -> Self {
        Self { program }
    }
impl Node for AST {
    fn string(&self) -> String {
        self.program.string()
    fn token_literal(&self) -> String {
        self.program.token_literal()
    }
}

/// Variable declaration statement (sus name = value or facts name = value)
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
impl VariableDeclaration {
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    pub fn with_type(mut self, type_annotation: Box<dyn Expression>) -> Self {
        self.type_annotation = Some(type_annotation);
        self
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
        if let Some(value) = &self.value {
            result.push_str(&format!(" = {}", value.string()));
        result
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for VariableDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(VariableDeclaration {
        })
    }
}

/// Constant declaration statement (vibes name = value)
#[derive(Debug, Clone)]
pub struct ConstantDeclaration {
impl ConstantDeclaration {
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    pub fn with_type(mut self, type_annotation: Box<dyn Expression>) -> Self {
        self.type_annotation = Some(type_annotation);
        self
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
        result
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ConstantDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ConstantDeclaration {
        })
    }
}

/// Import declaration statement
#[derive(Debug, Clone)]
pub struct ImportDeclaration {
impl ImportDeclaration {
    pub fn new(token: Token, path: String) -> Self {
        Self {
        }
    }

    pub fn with_alias(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
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
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Package declaration statement
#[derive(Debug, Clone)]
pub struct PackageDeclaration {
impl PackageDeclaration {
    pub fn new(token: Token, name: String) -> Self {
        Self {
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
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for PackageDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Interface method definition
#[derive(Debug, Clone)]
pub struct InterfaceMethod {
impl InterfaceMethod {
    pub fn new(
    ) -> Self {
        Self {
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
        result
    fn token_literal(&self) -> String {
        self.to_string().token_literal()
    }
}

/// Struct field definition
#[derive(Debug, Clone)]
pub struct StructField {
#[derive(Debug, Clone)]
pub enum FieldVisibility {
impl StructField {
    pub fn new(name: Identifier, field_type: Box<dyn Expression>) -> Self {
        Self {
        }
    }

    pub fn with_visibility(mut self, visibility: FieldVisibility) -> Self {
        self.visibility = visibility;
        self
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl Node for StructField {
    fn string(&self) -> String {
        format!("{} {}", self.to_string().string(), self.field_type.string())
    fn token_literal(&self) -> String {
        self.to_string().token_literal()
    }
}

/// Module declaration statement
#[derive(Debug, Clone)]
pub struct ModuleDeclaration {
impl ModuleDeclaration {
    pub fn new(token: Token, name: String, body: BlockStatement) -> Self {
        Self {
        }
    }

    pub fn with_visibility(mut self, is_public: bool) -> Self {
        self.is_public = is_public;
        self
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl Node for ModuleDeclaration {
    fn string(&self) -> String {
        format!("mod {} {}", self.to_string(), self.body.string())
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ModuleDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// ASTNode type alias for compatibility
pub type ASTNode = crate::ast::AstNode;
