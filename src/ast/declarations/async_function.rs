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
    pub token: String,
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Box<dyn Expression>>,
    pub body: BlockStatement,
    pub type_parameters: Vec<TypeParameter>,
    pub generic_constraints: Vec<GenericConstraint>,
    pub is_async: bool, // Always true for async functions
}

impl AsyncFunctionStatement {
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
            is_async: true,
        }
    }

    pub fn with_generics(
        token: String,
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Box<dyn Expression>>,
        body: BlockStatement,
        type_parameters: Vec<TypeParameter>,
        generic_constraints: Vec<GenericConstraint>,
    ) -> Self {
        Self {
            token,
            name,
            parameters,
            return_type,
            body,
            type_parameters,
            generic_constraints,
            is_async: true,
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
        
        let mut result = format!("slay async {}({})", self.name.string(), params.join(", "));
        
        if let Some(ret_type) = &self.return_type {
            result.push_str(&format!(" -> {}", ret_type.string()));
        }
        
        result.push(' ');
        result.push_str(&self.body.string());
        
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for AsyncFunctionStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(AsyncFunctionStatement {
            token: self.token.clone(),
            name: self.name.clone(),
            parameters: self.parameters.clone(),
            return_type: self.return_type.as_ref().map(|t| t.clone_box()),
            body: self.body.clone(),
            type_parameters: self.type_parameters.clone(),
            generic_constraints: self.generic_constraints.clone(),
            is_async: self.is_async,
        })
    }
}

/// Async function declaration alias for compatibility
pub type AsyncFunctionDeclaration = AsyncFunctionStatement;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::identifiers::Identifier;
    use crate::ast::block::BlockStatement;
    use crate::ast::statements::Statement as StmtTrait;

    #[test]
    fn test_async_function_creation() {
        let name = Identifier::new("test_func".to_string(), "test_func".to_string());
        let body = BlockStatement::new(String::new(), Vec::new());
        
        let async_func = AsyncFunctionStatement::new(
            "slay".to_string(),
            name,
            Vec::new(),
            None,
            body,
        );

        assert!(async_func.is_async);
        assert_eq!(async_func.name.value, "test_func");
        assert!(async_func.parameters.is_empty());
        assert!(async_func.return_type.is_none());
    }

    #[test]
    fn test_async_function_string_representation() {
        let name = Identifier::new("fetch_data".to_string(), "fetch_data".to_string());
        let body = BlockStatement::new("{}".to_string(), Vec::new());
        
        let async_func = AsyncFunctionStatement::new(
            "slay".to_string(),
            name,
            Vec::new(),
            None,
            body,
        );

        let string_repr = async_func.string();
        assert!(string_repr.contains("slay async fetch_data"));
        assert!(string_repr.contains("()"));
    }

    #[test]
    fn test_promise_return_type_detection() {
        let name = Identifier::new("async_func".to_string(), "async_func".to_string());
        let body = BlockStatement::new("{}".to_string(), Vec::new());
        
        let async_func = AsyncFunctionStatement::new(
            "slay".to_string(),
            name,
            Vec::new(),
            None, // No explicit return type
            body,
        );

        assert!(!async_func.returns_promise());
        assert!(async_func.get_inner_return_type().is_none());
    }

    #[test]
    fn test_async_function_cloning() {
        let name = Identifier::new("clone_test".to_string(), "clone_test".to_string());
        let body = BlockStatement::new("{}".to_string(), Vec::new());
        
        let async_func = AsyncFunctionStatement::new(
            "slay".to_string(),
            name,
            Vec::new(),
            None,
            body,
        );

        let cloned = async_func.clone();
        assert_eq!(async_func.name.value, cloned.name.value);
        assert_eq!(async_func.is_async, cloned.is_async);
        assert_eq!(async_func.token, cloned.token);
    }
}
