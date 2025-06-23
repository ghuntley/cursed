/// Execution Context for CURSED Programs
/// 
/// This module provides the runtime context for executing CURSED programs,
/// including symbol tables, function registries, and scope management.

use crate::execution::{CursedValue, ValueType};
use std::collections::HashMap;

/// Main execution context for CURSED programs
pub struct ExecutionContext {
    symbol_table: SymbolTable,
    function_registry: FunctionRegistry,
    call_stack: Vec<CallFrame>,
    global_scope: Scope,
    current_scope_id: ScopeId,
    next_scope_id: ScopeId,
}

/// Symbol table for variable and function lookups
pub struct SymbolTable {
    scopes: HashMap<ScopeId, Scope>,
    scope_stack: Vec<ScopeId>,
    global_scope_id: ScopeId,
}

/// Function registry for compiled functions
pub struct FunctionRegistry {
    functions: HashMap<String, FunctionInfo>,
    compiled_functions: HashMap<String, CompiledFunction>,
}

/// Information about a function
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub parameter_types: Vec<ValueType>,
    pub return_type: ValueType,
    pub is_compiled: bool,
    pub source_location: Option<crate::error::SourceLocation>,
}

/// Compiled function information
#[derive(Debug, Clone)]
pub struct CompiledFunction {
    pub name: String,
    pub ir_code: String,
    pub entry_point: String,
    pub compiled_at: std::time::SystemTime,
}

/// A scope containing variable bindings
#[derive(Debug, Clone)]
pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub variables: HashMap<String, VariableInfo>,
    pub is_function_scope: bool,
    pub function_name: Option<String>,
}

/// Information about a variable
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub value: CursedValue,
    pub value_type: ValueType,
    pub is_mutable: bool,
    pub is_parameter: bool,
    pub declared_at: std::time::SystemTime,
}

/// A frame on the call stack
#[derive(Debug, Clone)]
pub struct CallFrame {
    pub function_name: String,
    pub scope_id: ScopeId,
    pub return_address: Option<String>,
    pub parameters: Vec<CursedValue>,
    pub created_at: std::time::SystemTime,
}

/// Unique identifier for scopes
pub type ScopeId = u32;

impl ExecutionContext {
    /// Create a new execution context
    pub fn new() -> Self {
        let global_scope_id = 0;
        let mut scopes = HashMap::new();
        
        let global_scope = Scope {
            id: global_scope_id,
            parent: None,
            variables: HashMap::new(),
            is_function_scope: false,
            function_name: None,
        };
        
        scopes.insert(global_scope_id, global_scope.clone());
        
        let symbol_table = SymbolTable {
            scopes,
            scope_stack: vec![global_scope_id],
            global_scope_id,
        };

        Self {
            symbol_table,
            function_registry: FunctionRegistry::new(),
            call_stack: Vec::new(),
            global_scope,
            current_scope_id: global_scope_id,
            next_scope_id: 1,
        }
    }

    /// Define a variable in the current scope
    pub fn define_variable(
        &mut self,
        name: String,
        value: CursedValue,
        is_mutable: bool,
    ) -> Result<(), Error> {
        let variable_info = VariableInfo {
            name: name.clone(),
            value_type: value.get_type(),
            value,
            is_mutable,
            is_parameter: false,
            declared_at: std::time::SystemTime::now(),
        };

        self.symbol_table.define_variable(self.current_scope_id, name, variable_info)
    }

    /// Get a variable value
    pub fn get_variable(&self, name: &str) -> Option<&CursedValue> {
        self.symbol_table.get_variable(name).map(|info| &info.value)
    }

    /// Set a variable value (if mutable)
    pub fn set_variable(&mut self, name: &str, value: CursedValue) -> Result<(), Error> {
        self.symbol_table.set_variable(name, value)
    }

    /// Create a new scope
    pub fn push_scope(&mut self, is_function_scope: bool, function_name: Option<String>) -> ScopeId {
        let scope_id = self.next_scope_id;
        self.next_scope_id += 1;

        let scope = Scope {
            id: scope_id,
            parent: Some(self.current_scope_id),
            variables: HashMap::new(),
            is_function_scope,
            function_name,
        };

        self.symbol_table.scopes.insert(scope_id, scope);
        self.symbol_table.scope_stack.push(scope_id);
        self.current_scope_id = scope_id;

        scope_id
    }

    /// Remove the current scope
    pub fn pop_scope(&mut self) -> Result<(), Error> {
        if self.current_scope_id == self.symbol_table.global_scope_id {
            return Err(crate::error::Error::RuntimeError("Cannot pop global scope".to_string()));
        }

        self.symbol_table.scope_stack.pop();
        
        if let Some(&parent_scope_id) = self.symbol_table.scope_stack.last() {
            self.current_scope_id = parent_scope_id;
        } else {
            self.current_scope_id = self.symbol_table.global_scope_id;
        }

        Ok(())
    }

    /// Register a function
    pub fn register_function(&mut self, info: FunctionInfo) {
        self.function_registry.register_function(info);
    }

    /// Get function information
    pub fn get_function(&self, name: &str) -> Option<&FunctionInfo> {
        self.function_registry.get_function(name)
    }

    /// Add a compiled function
    pub fn add_compiled_function(&mut self, compiled_function: CompiledFunction) {
        self.function_registry.add_compiled_function(compiled_function);
    }

    /// Get compiled function
    pub fn get_compiled_function(&self, name: &str) -> Option<&CompiledFunction> {
        self.function_registry.get_compiled_function(name)
    }

    /// Push a call frame onto the call stack
    pub fn push_call_frame(&mut self, frame: CallFrame) {
        self.call_stack.push(frame);
    }

    /// Pop a call frame from the call stack
    pub fn pop_call_frame(&mut self) -> Option<CallFrame> {
        self.call_stack.pop()
    }

    /// Get the current call stack depth
    pub fn call_stack_depth(&self) -> usize {
        self.call_stack.len()
    }

    /// Get all variable names in the current scope
    pub fn get_variable_names(&self) -> Vec<String> {
        self.symbol_table.get_all_variable_names()
    }

    /// Get the number of defined variables
    pub fn get_variable_count(&self) -> usize {
        self.symbol_table.get_variable_count()
    }

    /// Get all function names
    pub fn get_function_names(&self) -> Vec<String> {
        self.function_registry.get_function_names()
    }

    /// Clear all variables and functions (for REPL reset)
    pub fn clear(&mut self) {
        self.symbol_table.clear();
        self.function_registry.clear();
        self.call_stack.clear();
        self.current_scope_id = self.symbol_table.global_scope_id;
    }

    /// Get current scope information
    pub fn get_current_scope(&self) -> Option<&Scope> {
        self.symbol_table.scopes.get(&self.current_scope_id)
    }

    /// Check if we're in a function scope
    pub fn in_function_scope(&self) -> bool {
        if let Some(scope) = self.get_current_scope() {
            scope.is_function_scope
        } else {
            false
        }
    }

    /// Get the current function name (if in function scope)
    pub fn get_current_function_name(&self) -> Option<String> {
        if let Some(scope) = self.get_current_scope() {
            scope.function_name.clone()
        } else {
            None
        }
    }
}

impl SymbolTable {
    /// Define a variable in the specified scope
    pub fn define_variable(
        &mut self,
        scope_id: ScopeId,
        name: String,
        variable_info: VariableInfo,
    ) -> Result<(), Error> {
        if let Some(scope) = self.scopes.get_mut(&scope_id) {
            if scope.variables.contains_key(&name) {
                return Err(crate::error::Error::RuntimeError(
                    format!("Variable '{}' is already defined in this scope", name)
                ));
            }
            scope.variables.insert(name, variable_info);
            Ok(())
        } else {
            Err(crate::error::Error::RuntimeError(
                format!("Scope {} not found", scope_id)
            ))
        }
    }

    /// Get a variable by searching up the scope chain
    pub fn get_variable(&self, name: &str) -> Option<&VariableInfo> {
        for &scope_id in self.scope_stack.iter().rev() {
            if let Some(scope) = self.scopes.get(&scope_id) {
                if let Some(variable) = scope.variables.get(name) {
                    return Some(variable);
                }
            }
        }
        None
    }

    /// Set a variable value
    pub fn set_variable(&mut self, name: &str, value: CursedValue) -> Result<(), Error> {
        for &scope_id in self.scope_stack.iter().rev() {
            if let Some(scope) = self.scopes.get_mut(&scope_id) {
                if let Some(variable) = scope.variables.get_mut(name) {
                    if !variable.is_mutable {
                        return Err(crate::error::Error::RuntimeError(
                            format!("Variable '{}' is not mutable", name)
                        ));
                    }
                    variable.value = value;
                    variable.value_type = variable.value.get_type();
                    return Ok(());
                }
            }
        }
        Err(crate::error::Error::RuntimeError(
            format!("Variable '{}' not found", name)
        ))
    }

    /// Get all variable names across all scopes
    pub fn get_all_variable_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for scope in self.scopes.values() {
            for name in scope.variables.keys() {
                if !names.contains(name) {
                    names.push(name.clone());
                }
            }
        }
        names.sort();
        names
    }

    /// Get total variable count
    pub fn get_variable_count(&self) -> usize {
        self.scopes.values()
            .map(|scope| scope.variables.len())
            .sum()
    }

    /// Clear all scopes except global
    pub fn clear(&mut self) {
        self.scopes.retain(|&k, _| k == self.global_scope_id);
        self.scope_stack.clear();
        self.scope_stack.push(self.global_scope_id);
        
        // Clear global scope variables
        if let Some(global_scope) = self.scopes.get_mut(&self.global_scope_id) {
            global_scope.variables.clear();
        }
    }
}

impl FunctionRegistry {
    /// Create a new function registry
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            compiled_functions: HashMap::new(),
        }
    }

    /// Register a function
    pub fn register_function(&mut self, info: FunctionInfo) {
        self.functions.insert(info.name.clone(), info);
    }

    /// Get function information
    pub fn get_function(&self, name: &str) -> Option<&FunctionInfo> {
        self.functions.get(name)
    }

    /// Add a compiled function
    pub fn add_compiled_function(&mut self, compiled_function: CompiledFunction) {
        self.compiled_functions.insert(compiled_function.name.clone(), compiled_function);
    }

    /// Get compiled function
    pub fn get_compiled_function(&self, name: &str) -> Option<&CompiledFunction> {
        self.compiled_functions.get(name)
    }

    /// Get all function names
    pub fn get_function_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.functions.keys().cloned().collect();
        names.sort();
        names
    }

    /// Clear all functions
    pub fn clear(&mut self) {
        self.functions.clear();
        self.compiled_functions.clear();
    }

    /// Check if a function is compiled
    pub fn is_function_compiled(&self, name: &str) -> bool {
        self.compiled_functions.contains_key(name)
    }

    /// Mark a function as compiled
    pub fn mark_function_compiled(&mut self, name: &str) {
        if let Some(function) = self.functions.get_mut(name) {
            function.is_compiled = true;
        }
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_context_creation() {
        let context = ExecutionContext::new();
        assert_eq!(context.call_stack_depth(), 0);
        assert_eq!(context.get_variable_count(), 0);
    }

    #[test]
    fn test_variable_definition() {
        let mut context = ExecutionContext::new();
        
        let value = CursedValue::Integer(42);
        let result = context.define_variable("x".to_string(), value, true);
        assert!(result.is_ok());
        
        let retrieved = context.get_variable("x");
        assert_eq!(retrieved, Some(&CursedValue::Integer(42)));
    }

    #[test]
    fn test_variable_mutation() {
        let mut context = ExecutionContext::new();
        
        // Define mutable variable
        context.define_variable("x".to_string(), CursedValue::Integer(42), true).unwrap();
        
        // Mutate it
        let result = context.set_variable("x", CursedValue::Integer(100));
        assert!(result.is_ok());
        
        let retrieved = context.get_variable("x");
        assert_eq!(retrieved, Some(&CursedValue::Integer(100)));
    }

    #[test]
    fn test_immutable_variable() {
        let mut context = ExecutionContext::new();
        
        // Define immutable variable
        context.define_variable("x".to_string(), CursedValue::Integer(42), false).unwrap();
        
        // Try to mutate it (should fail)
        let result = context.set_variable("x", CursedValue::Integer(100));
        assert!(result.is_err());
    }

    #[test]
    fn test_scope_management() {
        let mut context = ExecutionContext::new();
        
        // Define variable in global scope
        context.define_variable("global_var".to_string(), CursedValue::Integer(1), true).unwrap();
        
        // Create new scope
        let scope_id = context.push_scope(false, None);
        assert!(scope_id > 0);
        
        // Define variable in new scope
        context.define_variable("local_var".to_string(), CursedValue::Integer(2), true).unwrap();
        
        // Should be able to access both variables
        assert_eq!(context.get_variable("global_var"), Some(&CursedValue::Integer(1)));
        assert_eq!(context.get_variable("local_var"), Some(&CursedValue::Integer(2)));
        
        // Pop scope
        context.pop_scope().unwrap();
        
        // Should still access global variable
        assert_eq!(context.get_variable("global_var"), Some(&CursedValue::Integer(1)));
        // Local variable should no longer be accessible
        assert_eq!(context.get_variable("local_var"), None);
    }

    #[test]
    fn test_function_registry() {
        let mut context = ExecutionContext::new();
        
        let function_info = FunctionInfo {
            name: "test_func".to_string(),
            parameter_types: vec![ValueType::Integer],
            return_type: ValueType::Integer,
            is_compiled: false,
            source_location: None,
        };
        
        context.register_function(function_info);
        
        let retrieved = context.get_function("test_func");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "test_func");
    }
}
