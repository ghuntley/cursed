//! AST nodes for generic expressions in the CURSED language.
//!
//! This module defines the AST representations for expressions that involve
//! generic types and operations, such as generic function calls and struct
//! instantiations with type parameters.

use std::any::Any;
use crate::ast::traits::{Node, Expression};
use crate::ast::expressions::identifiers::Identifier;
use crate::lexer::token::Token;

/// Represents a type reference with type arguments
///
/// This is used for types with generic parameters like Box[T] or Result[T, E].
#[derive(Debug)]
pub struct TypeReference {
    pub token: String,
    pub name: Identifier,
    pub type_arguments: Vec<Box<dyn Expression>>,
}

impl Node for TypeReference {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        if self.type_arguments.is_empty() {
            self.name.string()
        } else {
            let type_args = self.type_arguments
                .iter()
                .map(|arg| arg.string())
                .collect::<Vec<String>>()
                .join(", ");
            format!("{name}[{args}]", 
                name = self.name.string(), 
                args = type_args)
        }
    }
}

impl Expression for TypeReference {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeReference {
            token: self.token.clone(),
            name: self.name.clone(),
            type_arguments: self.type_arguments.iter().map(|arg| arg.clone_box()).collect(),
        })
    }
}

/// Represents a generic function call expression (identity[T](x))
#[derive(Debug)]
pub struct GenericCallExpression {
    pub token: String,
    pub function: Box<dyn Expression>,
    pub type_arguments: Vec<Box<dyn Expression>>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Node for GenericCallExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        let args = self.arguments
            .iter()
            .map(|arg| arg.string())
            .collect::<Vec<String>>()
            .join(", ");
            
        let type_args = self.type_arguments
            .iter()
            .map(|arg| arg.string())
            .collect::<Vec<String>>()
            .join(", ");
            
        format!("{function}[{type_args}]({args})",
            function = self.function.string(),
            type_args = type_args,
            args = args)
    }
}

impl Expression for GenericCallExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(GenericCallExpression {
            token: self.token.clone(),
            function: self.function.clone_box(),
            type_arguments: self.type_arguments.iter().map(|arg| arg.clone_box()).collect(),
            arguments: self.arguments.iter().map(|arg| arg.clone_box()).collect(),
        })
    }
}

/// Represents a struct instantiation with fields
#[derive(Debug)]
pub struct BeLikeExpression {
    pub token: String,
    pub struct_name: Identifier,
    pub type_arguments: Vec<Box<dyn Expression>>,
    pub fields: Vec<(String, Box<dyn Expression>)>,
}

impl Node for BeLikeExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        let fields = self.fields
            .iter()
            .map(|(name, value)| format!("{}: {}", name, value.string()))
            .collect::<Vec<String>>()
            .join(", ");
            
        if self.type_arguments.is_empty() {
            format!("{name}{{{fields}}}",
                name = self.struct_name.string(),
                fields = fields)
        } else {
            let type_args = self.type_arguments
                .iter()
                .map(|arg| arg.string())
                .collect::<Vec<String>>()
                .join(", ");
                
            format!("{name}[{type_args}]{{{fields}}}",
                name = self.struct_name.string(),
                type_args = type_args,
                fields = fields)
        }
    }
}

impl Expression for BeLikeExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(BeLikeExpression {
            token: self.token.clone(),
            struct_name: self.struct_name.clone(),
            type_arguments: self.type_arguments.iter().map(|arg| arg.clone_box()).collect(),
            fields: self.fields.iter().map(|(name, expr)| (name.clone(), expr.clone_box())).collect(),
        })
    }
}