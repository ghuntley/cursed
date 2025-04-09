//! Symbol table for tracking variables and types

use std::collections::HashMap;
use std::fmt;

/// The scope level of a symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolScope {
    /// Global scope - accessible throughout the program
    Global,
    /// Local scope - accessible within a function
    Local,
    /// Built-in scope - predefined symbols
    Builtin,
    /// Free variable captured from an outer scope
    Free,
    /// Function scope
    Function,
}

/// A symbol representing a variable or function
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    /// The name of the symbol
    pub name: String,
    /// The scope of the symbol
    pub scope: SymbolScope,
    /// The index in the symbol table
    pub index: usize,
    /// The type of the symbol (if known)
    pub type_name: Option<String>,
}

/// Symbol table for tracking variables and their scopes
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
    /// The outer (parent) symbol table, if any
    pub outer: Option<Box<SymbolTable>>,
    /// The symbols defined in this scope
    pub symbols: HashMap<String, Symbol>,
    /// The number of definitions in this scope
    pub num_definitions: usize,
}

impl SymbolTable {
    /// Create a new empty symbol table
    pub fn new() -> Self {
        Self {
            outer: None,
            symbols: HashMap::new(),
            num_definitions: 0,
        }
    }
    
    /// Create a new symbol table with an outer scope
    pub fn new_enclosed(outer: SymbolTable) -> Self {
        Self {
            outer: Some(Box::new(outer)),
            symbols: HashMap::new(),
            num_definitions: 0,
        }
    }
    
    /// Define a new symbol in this scope
    pub fn define(&mut self, name: &str, type_name: Option<&str>) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: SymbolScope::Local,
            index: self.num_definitions,
            type_name: type_name.map(|s| s.to_string()),
        };
        
        self.symbols.insert(name.to_string(), symbol.clone());
        self.num_definitions += 1;
        
        symbol
    }
    
    /// Look up a symbol by name
    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        if let Some(symbol) = self.symbols.get(name) {
            return Some(symbol.clone());
        }
        
        if let Some(outer) = &self.outer {
            return outer.resolve(name);
        }
        
        None
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Scope with {} definitions:", self.num_definitions)?;
        
        for (name, symbol) in &self.symbols {
            writeln!(f, "  {}: {:?}", name, symbol)?;
        }
        
        if let Some(outer) = &self.outer {
            writeln!(f, "Outer scope:\n{}", outer)?;
        }
        
        Ok(())
    }
}