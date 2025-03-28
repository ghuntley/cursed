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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Valid identifiers for our tests
    fn identifier_strategy() -> impl Strategy<Value = String> {
        // Generate valid variable names (start with letter, can contain alphanumeric and underscore)
        "[a-zA-Z][a-zA-Z0-9_]{0,20}".prop_map(|s| s.to_string())
    }

    proptest! {
        /// Property: After defining a symbol, it should be resolvable with the same name
        #[test]
        fn define_and_resolve(name in identifier_strategy()) {
            let mut symbol_table = SymbolTable::new();
            let symbol = symbol_table.define(&name);
            
            let resolved = symbol_table.resolve(&name);
            assert!(resolved.is_some(), "Should resolve symbol after defining it");
            
            let resolved_symbol = resolved.unwrap();
            assert_eq!(resolved_symbol.name, name, "Resolved symbol should have the same name");
            assert_eq!(resolved_symbol.scope, SymbolScope::Global, "Symbol in root table should have Global scope");
            assert_eq!(resolved_symbol.index, 0, "First symbol should have index 0");
        }

        /// Property: Multiple symbols increment the index properly
        #[test]
        fn multiple_defines_increment_index(
            names in proptest::collection::vec(identifier_strategy(), 1..10)
                .prop_map(|v| v.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect::<Vec<_>>())
        ) {
            let mut symbol_table = SymbolTable::new();
            
            for (i, name) in names.iter().enumerate() {
                let symbol = symbol_table.define(name);
                assert_eq!(symbol.index, i, "Symbol index should match the order of definition");
            }
            
            assert_eq!(symbol_table.num_definitions, names.len(), "num_definitions should match the number of symbols defined");
        }

        /// Property: Symbols defined in outer scope are accessible from inner scope
        #[test]
        fn outer_scope_symbols_accessible(
            outer_name in identifier_strategy(),
            inner_name in identifier_strategy()
        ) {
            // Skip test if the names are the same to avoid collisions
            prop_assume!(outer_name != inner_name);
            
            let mut outer_table = SymbolTable::new();
            let outer_symbol = outer_table.define(&outer_name);
            
            let mut inner_table = SymbolTable::new_enclosed(outer_table);
            let inner_symbol = inner_table.define(&inner_name);
            
            // The inner symbol should be resolvable in the inner scope
            let resolved_inner = inner_table.resolve(&inner_name);
            assert!(resolved_inner.is_some(), "Inner symbol should be resolvable in inner scope");
            assert_eq!(resolved_inner.unwrap().scope, SymbolScope::Local, "Inner symbol should have Local scope");
            
            // The outer symbol should be resolvable in the inner scope too
            let resolved_outer = inner_table.resolve(&outer_name);
            assert!(resolved_outer.is_some(), "Outer symbol should be resolvable in inner scope");
            
            // The outer symbol should be marked as a free variable when accessed from inner scope
            let free_symbol = resolved_outer.unwrap();
            assert_eq!(free_symbol.scope, SymbolScope::Free, "Outer local symbol should be marked as Free when accessed from inner scope");
            assert!(!inner_table.free_symbols.is_empty(), "Free symbols list should not be empty");
        }

        /// Property: Builtin symbols have the correct scope
        #[test]
        fn builtin_symbols_have_correct_scope(
            name in identifier_strategy(),
            index in 0..100usize
        ) {
            let mut symbol_table = SymbolTable::new();
            let symbol = symbol_table.define_builtin(index, &name);
            
            assert_eq!(symbol.name, name, "Builtin symbol should have the correct name");
            assert_eq!(symbol.scope, SymbolScope::Builtin, "Builtin symbol should have Builtin scope");
            assert_eq!(symbol.index, index, "Builtin symbol should have the provided index");
            
            let resolved = symbol_table.resolve(&name);
            assert!(resolved.is_some(), "Builtin symbol should be resolvable");
            assert_eq!(resolved.unwrap().scope, SymbolScope::Builtin, "Resolved builtin should have Builtin scope");
        }

        /// Property: Function symbols have the correct scope and index
        #[test]
        fn function_symbols_have_correct_scope(name in identifier_strategy()) {
            let mut symbol_table = SymbolTable::new();
            let symbol = symbol_table.define_function_name(&name);
            
            assert_eq!(symbol.name, name, "Function symbol should have the correct name");
            assert_eq!(symbol.scope, SymbolScope::Function, "Function symbol should have Function scope");
            assert_eq!(symbol.index, 0, "Function symbol should have index 0");
            
            let resolved = symbol_table.resolve(&name);
            assert!(resolved.is_some(), "Function symbol should be resolvable");
            assert_eq!(resolved.unwrap().scope, SymbolScope::Function, "Resolved function should have Function scope");
        }

        /// Property: Non-existent symbols should not be resolvable
        #[test]
        fn nonexistent_symbols_not_resolvable(
            defined_name in identifier_strategy(),
            undefined_name in identifier_strategy()
        ) {
            // Skip test if the names are the same
            prop_assume!(defined_name != undefined_name);
            
            let mut symbol_table = SymbolTable::new();
            symbol_table.define(&defined_name);
            
            let resolved = symbol_table.resolve(&undefined_name);
            assert!(resolved.is_none(), "Undefined symbol should not be resolvable");
        }

        /// Property: Taking the outer scope works correctly
        #[test]
        fn take_outer_works_correctly(name in identifier_strategy()) {
            let mut outer_table = SymbolTable::new();
            outer_table.define(&name);
            
            let mut inner_table = SymbolTable::new_enclosed(outer_table.clone());
            
            // Inner table should have an outer table
            assert!(inner_table.outer.is_some(), "Inner table should have an outer table");
            
            // Take the outer table
            let taken_outer = inner_table.take_outer();
            assert!(taken_outer.is_some(), "Should be able to take the outer table");
            assert!(inner_table.outer.is_none(), "Inner table should no longer have an outer table after taking it");
            
            // The taken outer table should have the original symbol
            let mut taken_outer = *taken_outer.unwrap();
            let resolved = taken_outer.resolve(&name);
            assert!(resolved.is_some(), "Taken outer table should contain the original symbol");
        }
    }

    // Regular unit tests to complement the property-based tests
    #[test]
    fn test_basic_symbol_table_operations() {
        let mut symbol_table = SymbolTable::new();
        
        // Define and resolve a symbol
        let symbol = symbol_table.define("x");
        assert_eq!(symbol.name, "x");
        assert_eq!(symbol.scope, SymbolScope::Global);
        assert_eq!(symbol.index, 0);
        
        let resolved = symbol_table.resolve("x").unwrap();
        assert_eq!(resolved.name, "x");
        
        // Define a second symbol
        let symbol2 = symbol_table.define("y");
        assert_eq!(symbol2.index, 1);
        assert_eq!(symbol_table.num_definitions, 2);
    }

    #[test]
    fn test_nested_scopes() {
        let mut global = SymbolTable::new();
        global.define("global_var");
        
        let mut local = SymbolTable::new_enclosed(global);
        local.define("local_var");
        
        // Local var has Local scope
        let local_sym = local.resolve("local_var").unwrap();
        assert_eq!(local_sym.scope, SymbolScope::Local);
        
        // Global var is accessible from local scope and marked as free
        let global_sym = local.resolve("global_var").unwrap();
        assert_eq!(global_sym.scope, SymbolScope::Free);
        assert_eq!(local.free_symbols.len(), 1);
        
        // Create a deeper nesting
        let mut inner = SymbolTable::new_enclosed(local);
        inner.define("inner_var");
        
        // All variables should be accessible
        assert!(inner.resolve("inner_var").is_some());
        assert!(inner.resolve("local_var").is_some());
        assert!(inner.resolve("global_var").is_some());
        
        // Check free symbols tracking
        assert_eq!(inner.free_symbols.len(), 2);
    }
} 