//! Execution Context for CURSED runtime

use crate::error::CursedError;
use crate::execution::CursedValue;
use crate::ast::{StructStatement, InterfaceStatement};
use std::collections::HashMap;

/// Location information for error reporting
#[derive(Debug, Clone)]
pub struct LocationInfo {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

/// Execution context for CURSED programs
pub struct ExecutionContext {
    variables: HashMap<String, CursedValue>,
    functions: HashMap<String, crate::ast::FunctionStatement>,
    defer_stack: Vec<crate::ast::Expression>,
    /// Stack of defer scopes for proper cleanup ordering
    defer_scopes: Vec<Vec<crate::ast::Expression>>,
    /// Error context for stack traces and propagation
    error_contexts: HashMap<String, Vec<String>>,
    /// Error propagation chain
    error_propagation: Vec<(String, Vec<String>)>,
    /// Current function stack for debugging
    call_stack: Vec<String>,
    /// Current location information
    current_location: Option<LocationInfo>,
    /// Struct definitions storage
    struct_definitions: HashMap<String, StructStatement>,
    /// Interface definitions storage
    interface_definitions: HashMap<String, InterfaceStatement>,
    /// Fam context tracking
    fam_context_stack: Vec<bool>,
    /// Loaded modules registry
    loaded_modules: HashMap<String, crate::ast::Program>,
    /// Module search paths
    module_search_paths: Vec<std::path::PathBuf>,
    /// Type aliases registry for runtime type resolution
    type_aliases: HashMap<String, crate::ast::Type>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        let mut search_paths = vec![
            std::path::PathBuf::from("stdlib"),
            std::path::PathBuf::from(".")
        ];
        
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            defer_stack: Vec::new(),
            defer_scopes: Vec::new(),
            error_contexts: HashMap::new(),
            error_propagation: Vec::new(),
            call_stack: Vec::new(),
            current_location: None,
            struct_definitions: HashMap::new(),
            interface_definitions: HashMap::new(),
            fam_context_stack: Vec::new(),
            loaded_modules: HashMap::new(),
            module_search_paths: search_paths,
            type_aliases: HashMap::new(),
        }
    }
    
    /// Create a child context that inherits functions and global constants from parent
    pub fn new_child(&self) -> Self {
        Self {
            variables: self.variables.clone(), // Inherit global constants
            functions: self.functions.clone(),
            defer_stack: Vec::new(),
            defer_scopes: Vec::new(),
            error_contexts: HashMap::new(),
            error_propagation: Vec::new(),
            call_stack: self.call_stack.clone(),
            current_location: self.current_location.clone(),
            struct_definitions: self.struct_definitions.clone(),
            interface_definitions: self.interface_definitions.clone(),
            fam_context_stack: Vec::new(),
            loaded_modules: self.loaded_modules.clone(),
            module_search_paths: self.module_search_paths.clone(),
            type_aliases: self.type_aliases.clone(),
        }
    }
    
    pub fn set_variable(&mut self, name: String, value: CursedValue) {
        self.variables.insert(name, value);
    }
    
    pub fn set_constant(&mut self, name: String, value: CursedValue) {
        // Constants are stored the same way as variables for now
        // In a full implementation, we'd mark them as immutable
        self.variables.insert(name, value);
    }
    
    /// Set error context for debugging
    pub fn set_error_context(&mut self, name: String, stack_trace: Vec<String>) {
        self.error_contexts.insert(name, stack_trace);
    }
    
    /// Add error propagation entry
    pub fn add_error_propagation(&mut self, message: String, stack_trace: Vec<String>) {
        self.error_propagation.push((message, stack_trace));
    }
    
    /// Store struct definition
    pub fn store_struct_definition(&mut self, name: String, struct_def: crate::ast::StructStatement) {
        self.struct_definitions.insert(name, struct_def);
    }
    
    /// Store interface definition
    pub fn store_interface_definition(&mut self, name: String, interface_def: InterfaceStatement) {
        self.interface_definitions.insert(name, interface_def);
    }
    
    /// Get interface definition
    pub fn get_interface_definition(&self, name: &str) -> Option<InterfaceStatement> {
        self.interface_definitions.get(name).cloned()
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
    

    
    /// Get current call stack
    pub fn get_call_stack(&self) -> &[String] {
        &self.call_stack
    }
    
    /// Push function to call stack
    pub fn push_call_stack(&mut self, function_name: String) {
        self.call_stack.push(function_name);
    }
    
    /// Pop function from call stack
    pub fn pop_call_stack(&mut self) -> Option<String> {
        self.call_stack.pop()
    }
    
    /// Set current location
    pub fn set_current_location(&mut self, location: LocationInfo) {
        self.current_location = Some(location);
    }
    
    /// Get current location
    pub fn get_current_location(&self) -> Option<&LocationInfo> {
        self.current_location.as_ref()
    }
    

    
    /// Get error context for debugging
    pub fn get_error_context(&self, error_name: &str) -> Option<&Vec<String>> {
        self.error_contexts.get(error_name)
    }
    
    /// Get error propagation chain
    pub fn get_error_propagation(&self) -> &[(String, Vec<String>)] {
        &self.error_propagation
    }
    
    /// Get current function name
    pub fn current_function(&self) -> Option<&String> {
        self.call_stack.last()
    }
    
    /// Enter fam context
    pub fn enter_fam_context(&mut self) {
        self.fam_context_stack.push(true);
    }
    
    /// Exit fam context
    pub fn exit_fam_context(&mut self) {
        self.fam_context_stack.pop();
    }
    
    /// Check if we're in a fam context
    pub fn is_in_fam_context(&self) -> bool {
        self.fam_context_stack.last().unwrap_or(&false).clone()
    }
    
    /// Load a module by path
    pub fn load_module(&mut self, module_path: &str) -> Result<(), CursedError> {
        // Check if module is already loaded
        if self.loaded_modules.contains_key(module_path) {
            return Ok(());
        }
        
        // Try to find the module file
        let module_file = self.find_module_file(module_path)?;
        
        // Parse the module
        let module_source = std::fs::read_to_string(&module_file)
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read module {}: {}", module_path, e)))?;
        
        let mut lexer = crate::lexer::Lexer::new(module_source);
        let mut parser = crate::parser::Parser::new(lexer)
            .map_err(|e| CursedError::RuntimeError(format!("Failed to create parser for module {}: {}", module_path, e)))?;
        
        let module_program = parser.parse_program()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to parse module {}: {}", module_path, e)))?;
        
        // Store the loaded module
        self.loaded_modules.insert(module_path.to_string(), module_program.clone());
        
        // Import functions from the module into our context
        self.import_module_functions(&module_program)?;
        
        Ok(())
    }
    
    /// Find module file in search paths
    fn find_module_file(&self, module_path: &str) -> Result<std::path::PathBuf, CursedError> {
        for search_path in &self.module_search_paths {
            let candidate = search_path.join(module_path).join("mod.csd");
            if candidate.exists() {
                return Ok(candidate);
            }
            
            // Also try direct .csd file
            let candidate = search_path.join(format!("{}.csd", module_path));
            if candidate.exists() {
                return Ok(candidate);
            }
        }
        
        Err(CursedError::RuntimeError(format!("Module not found: {}", module_path)))
    }
    
    /// Import functions from a loaded module
    fn import_module_functions(&mut self, program: &crate::ast::Program) -> Result<(), CursedError> {
        // First, process any imports this module depends on
        for import in &program.imports {
            if !self.is_module_loaded(&import.path) {
                self.load_module(&import.path)?;
            }
        }
        
        // Then import the functions from this module
        for statement in &program.statements {
            if let crate::ast::Statement::Function(func) = statement {
                self.functions.insert(func.name.clone(), func.clone());
            }
        }
        Ok(())
    }
    
    /// Check if module is loaded
    pub fn is_module_loaded(&self, module_path: &str) -> bool {
        self.loaded_modules.contains_key(module_path)
    }
    
    /// Get loaded module
    pub fn get_module(&self, module_path: &str) -> Option<&crate::ast::Program> {
        self.loaded_modules.get(module_path)
    }

    /// Register a type alias for runtime type resolution
    pub fn set_type_alias(&mut self, name: String, target_type: crate::ast::Type) {
        self.type_aliases.insert(name, target_type);
    }

    /// Get the resolved type for a type alias
    pub fn get_type_alias(&self, name: &str) -> Option<&crate::ast::Type> {
        self.type_aliases.get(name)
    }

    /// Resolve a type, following type aliases if necessary
    pub fn resolve_type(&self, ty: &crate::ast::Type) -> crate::ast::Type {
        match ty {
            crate::ast::Type::Custom(name) => {
                if let Some(resolved_type) = self.get_type_alias(name) {
                    // Recursively resolve in case of nested aliases
                    self.resolve_type(resolved_type)
                } else {
                    ty.clone()
                }
            }
            _ => ty.clone()
        }
    }

}
