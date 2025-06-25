/// Symbol table system for LLVM code generation
/// 
/// This module provides symbol table management for variables, functions,
/// and other identifiers during LLVM compilation.

use crate::error::CursedError;
use crate::codegen::llvm::expression_compiler::{LlvmValue, LlvmType};
use inkwell::values::PointerValue;
use std::collections::HashMap;
use std::fmt;

/// Symbol information stored in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: LlvmType,
    pub llvm_name: String,
    pub is_constant: bool,
    pub is_global: bool,
    pub scope_level: usize,
    pub alloca_pointer: Option<PointerValue<'static>>,
}

impl Symbol {
    pub fn new(
        name: String,
        symbol_type: LlvmType,
        llvm_name: String,
        is_constant: bool,
        is_global: bool,
        scope_level: usize,
    ) -> Self {
        Self {
            name,
            symbol_type,
            llvm_name,
            is_constant,
            is_global,
            scope_level,
            alloca_pointer: None,
        }
    }
    
    pub fn local_variable(name: String, symbol_type: LlvmType, llvm_name: String, scope_level: usize) -> Self {
        Self::new(name, symbol_type, llvm_name, false, false, scope_level)
    }
    
    pub fn global_variable(name: String, symbol_type: LlvmType, llvm_name: String) -> Self {
        Self::new(name, symbol_type, llvm_name, false, true, 0)
    }
    
    pub fn constant(name: String, symbol_type: LlvmType, llvm_name: String, scope_level: usize) -> Self {
        Self::new(name, symbol_type, llvm_name, true, false, scope_level)
    }
    
    pub fn function(name: String, return_type: LlvmType, param_types: Vec<LlvmType>, llvm_name: String) -> Self {
        let function_type = LlvmType::Function {
            return_type: Box::new(return_type),
            param_types,
        };
        Self::new(name, function_type, llvm_name, true, true, 0)
    }
}

/// Scope represents a single scope level in the symbol table
#[derive(Debug, Clone)]
pub struct Scope {
    pub level: usize,
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<usize>,
}

impl Scope {
    pub fn new(level: usize, parent: Option<usize>) -> Self {
        Self {
            level,
            symbols: HashMap::new(),
            parent,
        }
    }
    
    pub fn insert(&mut self, name: String, symbol: Symbol) {
        self.symbols.insert(name, symbol);
    }
    
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
    
    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }
}

/// Symbol table with support for nested scopes
#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub scopes: Vec<Scope>,
    pub current_scope_level: usize,
    pub global_scope: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut scopes = Vec::new();
        scopes.push(Scope::new(0, None)); // Global scope
        
        Self {
            scopes,
            current_scope_level: 0,
            global_scope: 0,
        }
    }
    
    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        let new_level = self.current_scope_level + 1;
        let parent = Some(self.current_scope_level);
        self.scopes.push(Scope::new(new_level, parent));
        self.current_scope_level = new_level;
    }
    
    /// Exit the current scope
    pub fn exit_scope(&mut self) -> crate::error::Result<()> {
        if self.current_scope_level == 0 {
            return Err(CursedError::CompilationError("Cannot exit global scope".to_string()));
        }
        
        // Find parent scope level
        if let Some(current_scope) = self.scopes.get(self.current_scope_level) {
            if let Some(parent_level) = current_scope.parent {
                self.current_scope_level = parent_level;
            }
        }
        
        Ok(())
    }
    
    /// Declare a symbol in the current scope
    pub fn declare(&mut self, name: String, symbol: Symbol) -> crate::error::Result<()> {
        // Check if symbol already exists in current scope
        if let Some(current_scope) = self.scopes.get_mut(self.current_scope_level) {
            if current_scope.contains(&name) {
                return Err(CursedError::CompilationError(format!(
                    "Symbol '{}' already declared in current scope",
                    name
                )));
            }
            current_scope.insert(name, symbol);
        }
        
        Ok(())
    }
    
    /// Look up a symbol, searching up the scope chain
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut current_level = self.current_scope_level;
        
        loop {
            if let Some(scope) = self.scopes.get(current_level) {
                if let Some(symbol) = scope.get(name) {
                    return Some(symbol);
                }
                
                // Move to parent scope
                if let Some(parent_level) = scope.parent {
                    current_level = parent_level;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        None
    }
    
    /// Look up a symbol for mutable access, searching up the scope chain
    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        let mut current_level = self.current_scope_level;
        
        loop {
            if let Some(scope) = self.scopes.get_mut(current_level) {
                if scope.symbols.contains_key(name) {
                    return scope.symbols.get_mut(name);
                }
                
                // Move to parent scope
                let parent_level = scope.parent;
                if let Some(parent_level) = parent_level {
                    current_level = parent_level;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        None
    }
    
    /// Check if a symbol exists in the current scope only
    pub fn exists_in_current_scope(&self, name: &str) -> bool {
        if let Some(current_scope) = self.scopes.get(self.current_scope_level) {
            current_scope.contains(name)
        } else {
            false
        }
    }
    
    /// Get all symbols in the current scope
    pub fn current_scope_symbols(&self) -> Option<&HashMap<String, Symbol>> {
        self.scopes.get(self.current_scope_level).map(|scope| &scope.symbols)
    }
    
    /// Declare a local variable
    pub fn declare_variable(&mut self, name: String, var_type: LlvmType, llvm_name: String) -> crate::error::Result<()> {
        let symbol = Symbol::local_variable(name.clone(), var_type, llvm_name, self.current_scope_level);
        self.declare(name, symbol)
    }
    
    /// Declare a global variable
    pub fn declare_global_variable(&mut self, name: String, var_type: LlvmType, llvm_name: String) -> crate::error::Result<()> {
        let symbol = Symbol::global_variable(name.clone(), var_type, llvm_name);
        if let Some(global_scope) = self.scopes.get_mut(self.global_scope) {
            if global_scope.contains(&name) {
                return Err(CursedError::CompilationError(format!(
                    "Global variable '{}' already declared",
                    name
                )));
            }
            global_scope.insert(name, symbol);
        }
        Ok(())
    }
    
    /// Declare a constant
    pub fn declare_constant(&mut self, name: String, const_type: LlvmType, llvm_name: String) -> crate::error::Result<()> {
        let symbol = Symbol::constant(name.clone(), const_type, llvm_name, self.current_scope_level);
        self.declare(name, symbol)
    }
    
    /// Declare a function
    pub fn declare_function(
        &mut self,
        name: String,
        return_type: LlvmType,
        param_types: Vec<LlvmType>,
        llvm_name: String,
    ) -> crate::error::Result<()> {
        let symbol = Symbol::function(name.clone(), return_type, param_types, llvm_name);
        if let Some(global_scope) = self.scopes.get_mut(self.global_scope) {
            if global_scope.contains(&name) {
                return Err(CursedError::CompilationError(format!(
                    "Function '{}' already declared",
                    name
                )));
            }
            global_scope.insert(name, symbol);
        }
        Ok(())
    }
    
    /// Get the LLVM name for a symbol
    pub fn get_llvm_name(&self, name: &str) -> Option<String> {
        self.lookup(name).map(|symbol| symbol.llvm_name.clone())
    }
    
    /// Get the type of a symbol
    pub fn get_symbol_type(&self, name: &str) -> Option<LlvmType> {
        self.lookup(name).map(|symbol| symbol.symbol_type.clone())
    }
    
    /// Check if a symbol is constant
    pub fn is_constant(&self, name: &str) -> bool {
        self.lookup(name).map(|symbol| symbol.is_constant).unwrap_or(false)
    }
    
    /// Check if a symbol is global
    pub fn is_global(&self, name: &str) -> bool {
        self.lookup(name).map(|symbol| symbol.is_global).unwrap_or(false)
    }
    
    /// Get current scope level
    pub fn current_scope_level(&self) -> usize {
        self.current_scope_level
    }
    
    /// Debug: Print symbol table structure
    pub fn debug_print(&self) {
        println!("Symbol Table (current scope: {}):", self.current_scope_level);
        for (level, scope) in self.scopes.iter().enumerate() {
            println!("  Scope {}: {} symbols", level, scope.symbols.len());
            for (name, symbol) in &scope.symbols {
                println!("    {}: {:?} -> {}", name, symbol.symbol_type, symbol.llvm_name);
            }
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SymbolTable(current_scope: {})", self.current_scope_level)?;
        for (level, scope) in self.scopes.iter().enumerate() {
            writeln!(f, "  Scope {}: {} symbols", level, scope.symbols.len())?;
            for (name, symbol) in &scope.symbols {
                writeln!(f, "    {} -> {}", name, symbol.llvm_name)?;
            }
        }
        Ok(())
    }
}

