// Symbol table implementation
use std::collections::HashMap;

/// Represents a symbol in the CURSED language 
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize,
}

/// The scope of a symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolScope {
    Global,
    Local,
    Builtin,
    Free,
    Function,
}

/// Symbol table for storing and looking up symbols
pub struct SymbolTable {
    store: HashMap<String, Symbol>,
    outer: Option<Box<SymbolTable>>,
    num_definitions: usize,
}

impl SymbolTable {
    /// Create a new empty symbol table
    pub fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            outer: None,
            num_definitions: 0,
        }
    }

    /// Create a new symbol table with an outer scope
    pub fn new_enclosed(outer: SymbolTable) -> Self {
        SymbolTable {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
            num_definitions: 0,
        }
    }

    /// Define a new symbol in the current scope
    pub fn define(&mut self, name: &str) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: SymbolScope::Global,  // For simplicity, default to Global
            index: self.num_definitions,
        };
        
        self.store.insert(name.to_string(), symbol.clone());
        self.num_definitions += 1;
        
        symbol
    }

    /// Look up a symbol by name
    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        self.store.get(name).cloned().or_else(|| {
            self.outer.as_ref().and_then(|outer| outer.resolve(name))
        })
    }
}

impl Clone for Symbol {
    fn clone(&self) -> Self {
        Symbol {
            name: self.name.clone(),
            scope: self.scope,
            index: self.index,
        }
    }
} 