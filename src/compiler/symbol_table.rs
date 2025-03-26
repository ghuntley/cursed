use std::collections::HashMap;
use std::fmt;

/// Symbol definition for variables
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    /// Name of the symbol
    pub name: String,
    /// Scope of the symbol (global/local)
    pub scope: SymbolScope,
    /// Index for the symbol
    pub index: usize,
}

/// Scope types for symbols
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolScope {
    /// Global scope
    Global,
    /// Local scope
    Local,
    /// Free variable (for closures)
    Free,
    /// Built-in function
    Builtin,
    /// Function scope
    Function,
}

/// Symbol table for tracking variables in the compiler
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// Outer scope (parent symbol table)
    pub outer: Option<Box<SymbolTable>>,
    /// Symbols in this scope
    pub store: HashMap<String, Symbol>,
    /// Number of definitions in this scope
    pub num_definitions: usize,
    /// Free symbols (for closures)
    pub free_symbols: Vec<Symbol>,
}

impl SymbolTable {
    /// Create a new symbol table
    pub fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            outer: None,
            num_definitions: 0,
            free_symbols: Vec::new(),
        }
    }

    /// Create a new enclosed symbol table with an outer scope
    pub fn new_enclosed(outer: SymbolTable) -> Self {
        SymbolTable {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
            num_definitions: 0,
            free_symbols: Vec::new(),
        }
    }

    /// Define a new symbol
    pub fn define(&mut self, name: &str) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: if self.outer.is_none() {
                SymbolScope::Global
            } else {
                SymbolScope::Local
            },
            index: self.num_definitions,
        };

        self.store.insert(name.to_string(), symbol.clone());
        self.num_definitions += 1;

        symbol
    }

    /// Define a builtin symbol
    pub fn define_builtin(&mut self, index: usize, name: &str) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: SymbolScope::Builtin,
            index,
        };

        self.store.insert(name.to_string(), symbol.clone());
        
        symbol
    }

    /// Define a free symbol
    pub fn define_free(&mut self, original: Symbol) -> Symbol {
        self.free_symbols.push(original.clone());
        
        let symbol = Symbol {
            name: original.name,
            scope: SymbolScope::Free,
            index: self.free_symbols.len() - 1,
        };

        self.store.insert(symbol.name.clone(), symbol.clone());
        
        symbol
    }

    /// Define a function name symbol
    pub fn define_function_name(&mut self, name: &str) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: SymbolScope::Function,
            index: 0,
        };

        self.store.insert(name.to_string(), symbol.clone());
        
        symbol
    }

    /// Resolve a symbol by name
    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        // Check local scope first
        if let Some(symbol) = self.store.get(name) {
            return Some(symbol.clone());
        }

        // Check outer scope if it exists
        if let Some(ref mut outer) = self.outer {
            if let Some(obj) = outer.resolve(name) {
                if obj.scope == SymbolScope::Global || obj.scope == SymbolScope::Builtin {
                    return Some(obj);
                }

                // Define as a free variable if it's from an outer scope
                return Some(self.define_free(obj));
            }
        }

        None
    }

    /// Get free symbols
    pub fn free_symbols(&self) -> &[Symbol] {
        &self.free_symbols
    }

    /// Take the outer symbol table
    pub fn take_outer(&mut self) -> Option<Box<SymbolTable>> {
        self.outer.take()
    }
} 