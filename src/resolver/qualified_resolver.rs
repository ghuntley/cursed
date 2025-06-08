//! Qualified name resolution for package.Symbol access
//!
//! This module handles the resolution of qualified names (e.g., package.Symbol)
//! including handling of import aliases and cross-package symbol lookup.

use crate::error::Error;
use crate::resolver::{ResolvedSymbol, symbol_table::GlobalSymbolTable};
use std::collections::HashMap;

/// Maps qualified references to actual package names within different contexts
#[derive(Debug)]
pub struct QualifiedNameResolver {
    /// Map from (current_package, qualifier) to actual_package
    /// Used to resolve import aliases and package references
    package_mappings: HashMap<(String, String), String>,
}

impl QualifiedNameResolver {
    pub fn new() -> Self {
        Self {
            package_mappings: HashMap::new(),
        }
    }

    /// Register a package mapping for qualified name resolution
    /// 
    /// This maps a qualifier (which could be a package name or alias) within
    /// a specific package context to the actual package name.
    #[tracing::instrument(skip(self), fields(current_package = %current_package, qualifier = %qualifier, actual_package = %actual_package), level = "debug")]
    pub fn register_package_mapping(&mut self, current_package: &str, qualifier: &str, actual_package: &str) {
        let key = (current_package.to_string(), qualifier.to_string());
        self.package_mappings.insert(key, actual_package.to_string());
        
        tracing::debug!(
            current_package = %current_package,
            qualifier = %qualifier,
            actual_package = %actual_package,
            "Package mapping registered"
        );
    }

    /// Resolve a qualified name (package.symbol) within a given package context
    #[tracing::instrument(skip(self, global_table), fields(current_package = %current_package, qualified_name = %qualified_name), level = "debug")]
    pub fn resolve(&self, current_package: &str, qualified_name: &str, global_table: &GlobalSymbolTable) -> Result<ResolvedSymbol, Error> {
        // Parse the qualified name
        let parts = self.parse_qualified_name(qualified_name)?;
        let (qualifier, symbol_name) = parts;

        // Resolve the qualifier to the actual package name
        let actual_package = self.resolve_qualifier(current_package, &qualifier)?;

        // Build the canonical qualified name
        let canonical_qualified = format!("{}.{}", actual_package, symbol_name);

        // Resolve the symbol in the global table
        match global_table.resolve_qualified(&canonical_qualified) {
            Some(resolved) => {
                tracing::debug!(
                    qualified_name = %qualified_name,
                    resolved_package = %resolved.package,
                    symbol = %resolved.name,
                    "Qualified name resolved"
                );
                Ok(resolved)
            }
            None => {
                Err(Error::from_str(&format!(
                    "Symbol '{}' not found in package '{}' (qualified as '{}')",
                    symbol_name, actual_package, qualified_name
                )))
            }
        }
    }

    /// Parse a qualified name into (qualifier, symbol_name)
    fn parse_qualified_name(&self, qualified_name: &str) -> Result<(String, String), Error> {
        let parts: Vec<&str> = qualified_name.split('.').collect();
        
        if parts.len() != 2 {
            return Err(Error::from_str(&format!(
                "Invalid qualified name '{}'. Expected format: package.Symbol",
                qualified_name
            )));
        }

        if parts[0].is_empty() || parts[1].is_empty() {
            return Err(Error::from_str(&format!(
                "Invalid qualified name '{}'. Package and symbol names cannot be empty",
                qualified_name
            )));
        }

        Ok((parts[0].to_string(), parts[1].to_string()))
    }

    /// Resolve a qualifier (package name or alias) to the actual package name
    fn resolve_qualifier(&self, current_package: &str, qualifier: &str) -> Result<String, Error> {
        let key = (current_package.to_string(), qualifier.to_string());
        
        match self.package_mappings.get(&key) {
            Some(actual_package) => Ok(actual_package.clone()),
            None => {
                // If no mapping exists, assume the qualifier is the actual package name
                // This handles cases where packages are referenced by their real names
                // without explicit imports (which might be valid in some contexts)
                Ok(qualifier.to_string())
            }
        }
    }

    /// Check if a qualifier is mapped in the given package context
    pub fn has_qualifier_mapping(&self, current_package: &str, qualifier: &str) -> bool {
        let key = (current_package.to_string(), qualifier.to_string());
        self.package_mappings.contains_key(&key)
    }

    /// Get all qualifiers available in a package context
    pub fn get_available_qualifiers(&self, current_package: &str) -> Vec<String> {
        self.package_mappings.keys()
            .filter_map(|(pkg, qualifier)| {
                if pkg == current_package {
                    Some(qualifier.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Remove package mappings for a specific package (useful for cleanup)
    pub fn clear_package_mappings(&mut self, current_package: &str) {
        self.package_mappings.retain(|(pkg, _), _| pkg != current_package);
    }

    /// Get the actual package name for a qualifier in a given context
    pub fn get_actual_package(&self, current_package: &str, qualifier: &str) -> Option<String> {
        let key = (current_package.to_string(), qualifier.to_string());
        self.package_mappings.get(&key).cloned()
    }

    /// Validate a qualified name format without resolving it
    pub fn validate_qualified_name_format(&self, qualified_name: &str) -> Result<(), Error> {
        self.parse_qualified_name(qualified_name).map(|_| ())
    }

    /// Split a qualified name into its components (for external use)
    pub fn split_qualified_name(&self, qualified_name: &str) -> Result<(String, String), Error> {
        self.parse_qualified_name(qualified_name)
    }

    /// Get statistics about registered mappings
    pub fn get_mapping_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        for (pkg, _) in self.package_mappings.keys() {
            *stats.entry(pkg.clone()).or_insert(0) += 1;
        }
        
        stats
    }
}

impl Default for QualifiedNameResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::{SymbolType, SymbolVisibility, symbol_table::{GlobalSymbolTable, PackageSymbolTable}};

    fn create_test_global_table() -> GlobalSymbolTable {
        let mut global = GlobalSymbolTable::new();
        
        // Register packages
        global.register_package("vibez").unwrap();
        global.register_package("cryptz").unwrap();
        
        // Add some symbols to the global qualified namespace
        global.register_symbol("vibez", "spill", SymbolType::Function).unwrap();
        global.register_symbol("cryptz", "md5sum", SymbolType::Function).unwrap();
        
        // Create package tables with actual symbols
        if let Some(vibez_table) = global.get_package_mut("vibez") {
            vibez_table.define_symbol("spill", SymbolType::Function, SymbolVisibility::Public).unwrap();
        }
        
        if let Some(cryptz_table) = global.get_package_mut("cryptz") {
            cryptz_table.define_symbol("md5sum", SymbolType::Function, SymbolVisibility::Public).unwrap();
        }
        
        global
    }

    #[test]
    fn test_qualified_name_parsing() {
        let resolver = QualifiedNameResolver::new();
        
        // Valid qualified names
        let (pkg, sym) = resolver.parse_qualified_name("vibez.spill").unwrap();
        assert_eq!(pkg, "vibez");
        assert_eq!(sym, "spill");
        
        // Invalid qualified names
        assert!(resolver.parse_qualified_name("invalid").is_err());
        assert!(resolver.parse_qualified_name("too.many.parts").is_err());
        assert!(resolver.parse_qualified_name(".empty").is_err());
        assert!(resolver.parse_qualified_name("empty.").is_err());
    }

    #[test]
    fn test_package_mapping_resolution() {
        let mut resolver = QualifiedNameResolver::new();
        let global_table = create_test_global_table();
        
        // Register package mapping (import alias)
        resolver.register_package_mapping("main", "crypto", "cryptz");
        
        // Resolve qualified name with alias
        let resolved = resolver.resolve("main", "crypto.md5sum", &global_table).unwrap();
        assert_eq!(resolved.name, "md5sum");
        assert_eq!(resolved.package, "cryptz");
        
        // Resolve qualified name without alias (direct package name)
        resolver.register_package_mapping("main", "vibez", "vibez");
        let resolved = resolver.resolve("main", "vibez.spill", &global_table).unwrap();
        assert_eq!(resolved.name, "spill");
        assert_eq!(resolved.package, "vibez");
    }

    #[test]
    fn test_qualifier_mapping_queries() {
        let mut resolver = QualifiedNameResolver::new();
        
        resolver.register_package_mapping("main", "crypto", "cryptz");
        resolver.register_package_mapping("main", "output", "vibez");
        resolver.register_package_mapping("other", "crypto", "differentcrypto");
        
        // Test qualifier existence
        assert!(resolver.has_qualifier_mapping("main", "crypto"));
        assert!(!resolver.has_qualifier_mapping("main", "nonexistent"));
        
        // Test available qualifiers
        let qualifiers = resolver.get_available_qualifiers("main");
        assert_eq!(qualifiers.len(), 2);
        assert!(qualifiers.contains(&"crypto".to_string()));
        assert!(qualifiers.contains(&"output".to_string()));
        
        // Test actual package resolution
        assert_eq!(resolver.get_actual_package("main", "crypto"), Some("cryptz".to_string()));
        assert_eq!(resolver.get_actual_package("other", "crypto"), Some("differentcrypto".to_string()));
    }

    #[test]
    fn test_unresolved_symbols() {
        let resolver = QualifiedNameResolver::new();
        let global_table = create_test_global_table();
        
        // Try to resolve non-existent symbol
        let result = resolver.resolve("main", "vibez.nonexistent", &global_table);
        assert!(result.is_err());
        
        // Try to resolve symbol in non-existent package
        let result = resolver.resolve("main", "nonexistent.symbol", &global_table);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation() {
        let resolver = QualifiedNameResolver::new();
        
        assert!(resolver.validate_qualified_name_format("valid.name").is_ok());
        assert!(resolver.validate_qualified_name_format("invalid").is_err());
        assert!(resolver.validate_qualified_name_format("too.many.parts").is_err());
    }

    #[test]
    fn test_mapping_statistics() {
        let mut resolver = QualifiedNameResolver::new();
        
        resolver.register_package_mapping("main", "crypto", "cryptz");
        resolver.register_package_mapping("main", "output", "vibez");
        resolver.register_package_mapping("other", "utils", "utility_package");
        
        let stats = resolver.get_mapping_stats();
        assert_eq!(stats.get("main"), Some(&2));
        assert_eq!(stats.get("other"), Some(&1));
    }
}
