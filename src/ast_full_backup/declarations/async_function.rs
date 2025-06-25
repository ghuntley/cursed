/// Async function declaration AST node for the CURSED programming language
/// 
/// Represents async function declarations with the syntax:
/// slay async function_name(params) -> ReturnType { body }

use crate::ast::traits::{Node, Statement, Expression};
use crate::ast::identifiers::Identifier;
use crate::ast::block::BlockStatement;
use crate::ast::expressions::Parameter;
use crate::ast::declarations::{TypeParameter, GenericConstraint};
use std::any::Any;

/// Async function declaration (slay async name(params) return_type { body })
#[derive(Debug, Clone)]
pub struct AsyncFunctionStatement {
    pub is_async: bool, // Always true for async functions
impl AsyncFunctionStatement {
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    pub fn with_generics(
    ) -> Self {
        Self {
        }
    }

    /// Check if this function returns a Promise-like type
    pub fn returns_promise(&self) -> bool {
        if let Some(return_type) = &self.return_type {
            let type_str = return_type.string();
            type_str.starts_with("Promise<") || type_str.starts_with("Future<") || type_str.starts_with("Task<")
        } else {
            false
        }
    }

    /// Get the inner type of a Promise return type
    pub fn get_inner_return_type(&self) -> Option<String> {
        if let Some(return_type) = &self.return_type {
            let type_str = return_type.string();
            if type_str.starts_with("Promise<") && type_str.ends_with('>') {
                let inner = type_str[8..type_str.len()-1].to_string();
                return Some(inner);
            }
        }
        None
    }
}

impl Node for AsyncFunctionStatement {
    fn string(&self) -> String {
        let params: Vec<String> = self.parameters.iter()
            .map(|p| p.string())
            .collect();
        
        let mut result = format!("slay async {}({})", self.to_string().string(), params.join(", "));
        
        if let Some(ret_type) = &self.return_type {
            result.push_str(&format!(" -> {}", ret_type.string()));
        result.push(' ');
        result.push_str(&self.body.string());
        
        result
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for AsyncFunctionStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(AsyncFunctionStatement {
        })
    }
}

/// Async function declaration alias for compatibility
pub type AsyncFunctionDeclaration = AsyncFunctionStatement;

