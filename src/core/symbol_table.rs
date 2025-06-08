//! Symbol Table
//!
//! This module provides a basic symbol table implementation for the CURSED language.
//! The symbol table tracks variable names, types, and scopes during compilation.
//!
//! ## Scoping
//!
//! The symbol table supports lexical scoping through a linked list of symbol table
//! rules, with each table potentially having an outer (parent) scope.

use std::collections::HashMap;
use std::fmt;

/// The scope level of a symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolScope {
    /// Global scope (module level)
    Global,
    /// Local scope (within a function or block)
    Local,
    /// Built-in scope (language built-ins)
    Builtin,
    /// Free variable (captured from outer scope)
    Free,
    /// Function parameter
    Function,
}

impl fmt::Display for SymbolScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolScope::Global => write!(f, "GLOBAL"),
            SymbolScope::Local => write!(f, "LOCAL"),
            SymbolScope::Builtin => write!(f, "BUILTIN"),
            SymbolScope::Free => write!(f, "FREE"),
            SymbolScope::Function => write!(f, "FUNCTION"),
        }
    }
}

/// A symbol in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbol name
    pub name: String,
    /// Symbol scope
    pub scope: SymbolScope,
    /// Symbol type (simplified as string for now)
    pub symbol_type: String,
    /// Whether the symbol is mutable
    pub is_mutable: bool,
}

/// The main symbol table structure
///
/// This symbol table now integrates with package resolution for qualified name resolution.
pub struct SymbolTable {
    /// The outer (parent) scope
    pub outer: Option<Box<SymbolTable>>,
    /// The symbols defined in this scope
    pub symbols: HashMap<String, Symbol>,
    /// The number of definitions in this scope
    pub num_definitions: usize,
    /// Current package context for this symbol table
    pub current_package: Option<String>,
}

impl SymbolTable {
    /// Create a new empty symbol table
    pub fn new() -> Self {
        Self {
            outer: None,
            symbols: HashMap::new(),
            num_definitions: 0,
            current_package: None,
        }
    }

    /// Create a new symbol table with an outer scope
    pub fn new_enclosed(outer: SymbolTable) -> Self {
        Self {
            outer: Some(Box::new(outer)),
            symbols: HashMap::new(),
            num_definitions: 0,
            current_package: None,
        }
    }

    /// Create a new enclosed symbol table with package context
    pub fn new_enclosed_with_context(outer: SymbolTable) -> Self {
        let current_package = outer.current_package.clone();
        Self {
            outer: Some(Box::new(outer)),
            symbols: HashMap::new(),
            num_definitions: 0,
            current_package,
        }
    }

    /// Define a new symbol in the current scope
    #[tracing::instrument(skip(self), fields(name = ?name), level = "debug")]
    pub fn define(&mut self, name: String) -> Symbol {
        let symbol = Symbol {
            name: name.clone(),
            scope: SymbolScope::Local,
            symbol_type: "unknown".to_string(),
            is_mutable: false,
        };
        self.symbols.insert(name, symbol.clone());
        self.num_definitions += 1;
        symbol
    }

    /// Define a symbol with specific type and mutability
    #[tracing::instrument(skip(self), fields(name = ?name, symbol_type = ?symbol_type), level = "debug")]
    pub fn define_typed(&mut self, name: String, symbol_type: String, is_mutable: bool, scope: SymbolScope) -> Symbol {
        let symbol = Symbol {
            name: name.clone(),
            scope,
            symbol_type,
            is_mutable,
        };
        self.symbols.insert(name, symbol.clone());
        self.num_definitions += 1;
        symbol
    }

    /// Resolve a symbol by name, searching up the scope chain
    #[tracing::instrument(skip(self), fields(name = ?name), level = "debug")]
    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        // First check current scope
        if let Some(symbol) = self.symbols.get(name) {
            return Some(symbol.clone());
        }

        // Check if we have an outer scope
        if let Some(outer) = &self.outer {
            // Try to resolve in outer scope
            if let Some(mut symbol) = outer.resolve(name) {
                // Adjust scope if it's local in outer scope
                if symbol.scope == SymbolScope::Local {
                    symbol.scope = SymbolScope::Free;
                }
                return Some(symbol);
            }
        }

        None
    }

    /// Register an import in the symbol table
    #[tracing::instrument(skip(self), fields(imported_package = package, alias = ?alias), level = "debug")]
    pub fn register_import(&mut self, package: &str, alias: Option<&str>) -> Result<(), crate::error::Error> {
        // For now, just store the import as a symbol
        let import_name = alias.unwrap_or(package);
        let symbol = Symbol {
            name: import_name.to_string(),
            scope: SymbolScope::Global,
            symbol_type: "import".to_string(),
            is_mutable: false,
        };
        self.symbols.insert(import_name.to_string(), symbol);
        Ok(())
    }

    /// Define a symbol in the current package
    #[tracing::instrument(skip(self), fields(symbol_name = name, symbol_type = ?symbol_type), level = "debug")]
    pub fn define_package_symbol(&mut self, name: &str, symbol_type: &str) -> Result<(), crate::error::Error> {
        let symbol = Symbol {
            name: name.to_string(),
            scope: SymbolScope::Global,
            symbol_type: symbol_type.to_string(),
            is_mutable: false,
        };
        self.symbols.insert(name.to_string(), symbol);
        Ok(())
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SymbolTable {{")?;
        for (name, symbol) in &self.symbols {
            write!(f, " {}:{}", name, symbol.scope)?;
        }
        write!(f, " }}")
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
