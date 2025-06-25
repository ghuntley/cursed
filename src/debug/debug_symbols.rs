/// Debug symbol generation and management
// use crate::debug::source_location::SourceLocation;
use std::collections::HashMap;
use tracing::{debug, instrument};

/// Types of debug symbols
#[derive(Debug, Clone, PartialEq)]
pub enum DebugSymbolType {
    Function,
    Variable,
    Parameter,
    Type,
    Namespace,
    Member,
}

/// Debug symbol information
#[derive(Debug, Clone)]
pub struct DebugSymbol {
    pub name: String,
    pub symbol_type: DebugSymbolType,
    pub type_name: String,
    pub location: SourceLocation,
    pub scope_start: Option<SourceLocation>,
    pub scope_end: Option<SourceLocation>,
    pub address: Option<u64>,
    pub size: Option<u64>,
    pub is_artificial: bool,
    pub attributes: HashMap<String, String>,
}

impl DebugSymbol {
    /// Create a new debug symbol
    pub fn new(
        name: String,
        symbol_type: DebugSymbolType,
        type_name: String,
        location: SourceLocation,
    ) -> Self {
        Self {
            name,
            symbol_type,
            type_name,
            location,
            scope_start: None,
            scope_end: None,
            address: None,
            size: None,
            is_artificial: false,
            attributes: HashMap::new(),
        }
    }

    /// Create a function symbol
    pub fn function(name: String, location: SourceLocation) -> Self {
        Self::new(name, DebugSymbolType::Function, "function".to_string(), location)
    }

    /// Create a variable symbol
    pub fn variable(name: String, type_name: String, location: SourceLocation) -> Self {
        Self::new(name, DebugSymbolType::Variable, type_name, location)
    }

    /// Create a parameter symbol
    pub fn parameter(name: String, type_name: String, location: SourceLocation) -> Self {
        Self::new(name, DebugSymbolType::Parameter, type_name, location)
    }

    /// Create a type symbol
    pub fn type_def(name: String, location: SourceLocation) -> Self {
        Self::new(name, DebugSymbolType::Type, "type".to_string(), location)
    }

    /// Set scope information
    pub fn with_scope(mut self, start: SourceLocation, end: SourceLocation) -> Self {
        self.scope_start = Some(start);
        self.scope_end = Some(end);
        self
    }

    /// Set address information
    pub fn with_address(mut self, address: u64) -> Self {
        self.address = Some(address);
        self
    }

    /// Set size information
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// Mark as artificial (compiler-generated)
    pub fn as_artificial(mut self) -> Self {
        self.is_artificial = true;
        self
    }

    /// Add an attribute
    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }

    /// Check if symbol is in scope at a given location
    pub fn is_in_scope(&self, location: &SourceLocation) -> bool {
        if let (Some(start), Some(end)) = (&self.scope_start, &self.scope_end) {
            // Simple line-based scope check
            location.line >= start.line && location.line <= end.line
        } else {
            true // Global scope
        }
    }

    /// Get a human-readable description
    pub fn description(&self) -> String {
        format!(
            "{:?} '{}' of type '{}' at {}",
            self.symbol_type,
            self.name,
            self.type_name,
            self.location
        )
    }
}

/// Debug symbol table for managing symbols during compilation
#[derive(Debug)]
pub struct DebugSymbolTable {
    symbols: HashMap<String, DebugSymbol>,
    scopes: Vec<Vec<String>>, // Stack of symbol names in each scope
    current_scope_id: usize,
}

impl DebugSymbolTable {
    /// Create a new symbol table
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            scopes: Vec::from([Vec::new()]), // Global scope
            current_scope_id: 0,
        }
    }

    /// Enter a new scope
    #[instrument(skip(self))]
    pub fn enter_scope(&mut self) {
        debug!(scope_id = self.current_scope_id + 1, "Entering new scope");
        self.scopes.push(Vec::new());
        self.current_scope_id += 1;
    }

    /// Exit the current scope
    #[instrument(skip(self))]
    pub fn exit_scope(&mut self) {
        if self.current_scope_id > 0 {
            debug!(scope_id = self.current_scope_id, "Exiting scope");
            self.scopes.pop();
            self.current_scope_id -= 1;
        }
    }

    /// Add a symbol to the current scope
    #[instrument(skip(self))]
    pub fn add_symbol(&mut self, symbol: DebugSymbol) -> Result<(), String> {
        let name = symbol.name.clone();
        debug!(symbol = ?symbol, "Adding symbol to table");

        // Check for duplicate in current scope
        if let Some(current_scope) = self.scopes.last() {
            if current_scope.contains(&name) {
                return Err(format!("Symbol '{}' already exists in current scope", name));
            }
        }

        // Add to current scope
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.push(name.clone());
        }

        // Add to symbol table with scoped name
        let scoped_name = self.scoped_name(&name);
        self.symbols.insert(scoped_name, symbol);
        Ok(())
    }

    /// Look up a symbol by name
    pub fn lookup_symbol(&self, name: &str) -> Option<&DebugSymbol> {
        // Try current scope first, then outer scopes
        for scope_level in (0..=self.current_scope_id).rev() {
            let scoped_name = format!("{}::{}", scope_level, name);
            if let Some(symbol) = self.symbols.get(&scoped_name) {
                return Some(symbol);
            }
        }
        None
    }

    /// Get all symbols in the current scope
    pub fn current_scope_symbols(&self) -> Vec<&DebugSymbol> {
        let mut result = Vec::new();
        if let Some(current_scope) = self.scopes.last() {
            for name in current_scope {
                let scoped_name = self.scoped_name(name);
                if let Some(symbol) = self.symbols.get(&scoped_name) {
                    result.push(symbol);
                }
            }
        }
        result
    }

    /// Get all symbols
    pub fn all_symbols(&self) -> Vec<&DebugSymbol> {
        self.symbols.values().collect()
    }

    /// Get symbols by type
    pub fn symbols_by_type(&self, symbol_type: DebugSymbolType) -> Vec<&DebugSymbol> {
        self.symbols
            .values()
            .filter(|s| s.symbol_type == symbol_type)
            .collect()
    }

    /// Get symbols in scope at a location
    pub fn symbols_at_location(&self, location: &SourceLocation) -> Vec<&DebugSymbol> {
        self.symbols
            .values()
            .filter(|s| s.is_in_scope(location))
            .collect()
    }

    /// Get the current scope depth
    pub fn scope_depth(&self) -> usize {
        self.current_scope_id
    }

    /// Clear all symbols
    pub fn clear(&mut self) {
        self.symbols.clear();
        self.scopes = Vec::from([Vec::new()]);
        self.current_scope_id = 0;
    }

    /// Generate a scoped name for internal storage
    fn scoped_name(&self, name: &str) -> String {
        format!("{}::{}", self.current_scope_id, name)
    }

    /// Get symbol count
    pub fn symbol_count(&self) -> usize {
        self.symbols.len()
    }

    /// Export symbols for DWARF generation
    pub fn export_for_dwarf(&self) -> Vec<DebugSymbol> {
        self.symbols.values().cloned().collect()
    }
}

impl Default for DebugSymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Symbol scope guard for RAII-style scope management
pub struct SymbolScopeGuard<'a> {
    table: &'a mut DebugSymbolTable,
}

impl<'a> SymbolScopeGuard<'a> {
    pub fn new(table: &'a mut DebugSymbolTable) -> Self {
        table.enter_scope();
        Self { table }
    }
}

impl<'a> Drop for SymbolScopeGuard<'a> {
    fn drop(&mut self) {
        self.table.exit_scope();
    }
}

