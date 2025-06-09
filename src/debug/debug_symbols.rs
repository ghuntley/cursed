/// Debug symbol generation and management
use crate::debug::source_location::SourceLocation;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::source_location::SourceLocation;
    use std::path::PathBuf;

    #[test]
    fn test_debug_symbol_creation() {
        let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 5);
        let symbol = DebugSymbol::function("test_func".to_string(), location.clone());
        
        assert_eq!(symbol.name, "test_func");
        assert_eq!(symbol.symbol_type, DebugSymbolType::Function);
        assert_eq!(symbol.location, location);
    }

    #[test]
    fn test_symbol_with_scope() {
        let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 5);
        let start = SourceLocation::new(PathBuf::from("test.csd"), 10, 1);
        let end = SourceLocation::new(PathBuf::from("test.csd"), 20, 1);
        
        let symbol = DebugSymbol::variable("x".to_string(), "int".to_string(), location.clone())
            .with_scope(start.clone(), end.clone());
        
        assert_eq!(symbol.scope_start, Some(start));
        assert_eq!(symbol.scope_end, Some(end));
        
        // Test scope checking
        let in_scope = SourceLocation::new(PathBuf::from("test.csd"), 15, 10);
        let out_of_scope = SourceLocation::new(PathBuf::from("test.csd"), 25, 10);
        
        assert!(symbol.is_in_scope(&in_scope));
        assert!(!symbol.is_in_scope(&out_of_scope));
    }

    #[test]
    fn test_symbol_table_operations() {
        let mut table = DebugSymbolTable::new();
        let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 5);
        
        // Add a symbol
        let symbol = DebugSymbol::variable("x".to_string(), "int".to_string(), location);
        table.add_symbol(symbol).unwrap();
        
        // Look it up
        let found = table.lookup_symbol("x");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "x");
        
        // Test scope management
        assert_eq!(table.scope_depth(), 0);
        table.enter_scope();
        assert_eq!(table.scope_depth(), 1);
        table.exit_scope();
        assert_eq!(table.scope_depth(), 0);
    }

    #[test]
    fn test_scoped_symbols() {
        let mut table = DebugSymbolTable::new();
        let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 5);
        
        // Add symbol in global scope
        let global_symbol = DebugSymbol::variable("x".to_string(), "int".to_string(), location.clone());
        table.add_symbol(global_symbol).unwrap();
        
        // Enter new scope and add symbol with same name
        table.enter_scope();
        let local_symbol = DebugSymbol::variable("x".to_string(), "string".to_string(), location);
        table.add_symbol(local_symbol).unwrap();
        
        // Should find the local symbol first
        let found = table.lookup_symbol("x");
        assert!(found.is_some());
        assert_eq!(found.unwrap().type_name, "string");
        
        // Exit scope - should find global symbol again
        table.exit_scope();
        let found = table.lookup_symbol("x");
        assert!(found.is_some());
        assert_eq!(found.unwrap().type_name, "int");
    }

    #[test]
    fn test_scope_guard() {
        let mut table = DebugSymbolTable::new();
        assert_eq!(table.scope_depth(), 0);
        
        {
            let _guard = SymbolScopeGuard::new(&mut table);
            // Can't check depth while guard is active due to borrow checker
            // The depth will be checked after guard is dropped
        }
        
        // Verify scope was properly managed
        assert_eq!(table.scope_depth(), 0);
    }

    #[test]
    fn test_symbols_by_type() {
        let mut table = DebugSymbolTable::new();
        let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 5);
        
        table.add_symbol(DebugSymbol::function("func".to_string(), location.clone())).unwrap();
        table.add_symbol(DebugSymbol::variable("var".to_string(), "int".to_string(), location)).unwrap();
        
        let functions = table.symbols_by_type(DebugSymbolType::Function);
        let variables = table.symbols_by_type(DebugSymbolType::Variable);
        
        assert_eq!(functions.len(), 1);
        assert_eq!(variables.len(), 1);
        assert_eq!(functions[0].name, "func");
        assert_eq!(variables[0].name, "var");
    }
}
