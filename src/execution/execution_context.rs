//! Execution Context for CURSED runtime

use crate::error::CursedError;
use crate::execution::CursedValue;
use std::collections::HashMap;

/// Execution context for CURSED programs
pub struct ExecutionContext {
    variables: HashMap<String, CursedValue>,
    functions: HashMap<String, crate::ast::FunctionStatement>,
    defer_stack: Vec<crate::ast::Expression>,
    /// Stack of defer scopes for proper cleanup ordering
    defer_scopes: Vec<Vec<crate::ast::Expression>>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            defer_stack: Vec::new(),
            defer_scopes: Vec::new(),
        }
    }
    
    /// Create a child context that inherits functions and global constants from parent
    pub fn new_child(&self) -> Self {
        Self {
            variables: self.variables.clone(), // Inherit global constants
            functions: self.functions.clone(),
            defer_stack: Vec::new(),
            defer_scopes: Vec::new(),
        }
    }
    
    pub fn set_variable(&mut self, name: String, value: CursedValue) {
        self.variables.insert(name, value);
    }
    
    pub fn get_variable(&self, name: &str) -> Option<CursedValue> {
        self.variables.get(name).cloned()
    }
    
    pub fn set_function(&mut self, name: String, func: crate::ast::FunctionStatement) {
        self.functions.insert(name, func);
    }
    
    pub fn get_function(&self, name: &str) -> Option<crate::ast::FunctionStatement> {
        self.functions.get(name).cloned()
    }
    
    /// Get all variables for lambda environment capture
    pub fn get_all_variables(&self) -> &HashMap<String, CursedValue> {
        &self.variables
    }
    
    /// Add an expression to the defer stack
    pub fn push_defer(&mut self, expression: crate::ast::Expression) {
        self.defer_stack.push(expression);
    }
    
    /// Execute all deferred expressions in reverse order (LIFO)
    pub fn execute_defers(&mut self) -> Vec<crate::ast::Expression> {
        let mut defers = Vec::new();
        while let Some(expr) = self.defer_stack.pop() {
            defers.push(expr);
        }
        defers
    }
    
    /// Push a new defer scope (for function entry, block entry, etc.)
    pub fn push_defer_scope(&mut self) {
        self.defer_scopes.push(Vec::new());
    }
    
    /// Pop and execute all deferred expressions in the current scope
    pub fn pop_defer_scope(&mut self) -> Vec<crate::ast::Expression> {
        if let Some(scope_defers) = self.defer_scopes.pop() {
            // Return deferred expressions in reverse order (LIFO)
            scope_defers.into_iter().rev().collect()
        } else {
            Vec::new()
        }
    }
    
    /// Add an expression to the current defer scope
    pub fn push_defer_to_scope(&mut self, expression: crate::ast::Expression) {
        if let Some(current_scope) = self.defer_scopes.last_mut() {
            current_scope.push(expression);
        } else {
            // No scope active, use the main defer stack
            self.defer_stack.push(expression);
        }
    }
    
    /// Check if there are any defer scopes active
    pub fn has_defer_scopes(&self) -> bool {
        !self.defer_scopes.is_empty()
    }
}
