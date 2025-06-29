//! Execution Context for CURSED runtime

use crate::error::CursedError;
use crate::execution::CursedValue;
use std::collections::HashMap;

/// Execution context for CURSED programs
pub struct ExecutionContext {
    variables: HashMap<String, CursedValue>,
    functions: HashMap<String, crate::ast::FunctionStatement>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
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
}
