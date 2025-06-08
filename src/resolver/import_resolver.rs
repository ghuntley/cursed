//! Import resolution for CURSED packages
//!
//! This module handles the resolution of import statements and manages
//! the mapping between import aliases and actual package names.

use crate::error::Error;
use std::collections::HashMap;

/// Information about an imported package
#[derive(Debug, Clone)]
pub struct ImportInfo {
    /// The actual package being imported
    pub package_name: String,
    /// Optional alias for the package
    pub alias: Option<String>,
    /// Import path as specified in the source code
    pub import_path: String,
}

impl ImportInfo {
    pub fn new(package_name: String, alias: Option<String>, import_path: String) -> Self {
        Self {
            package_name,
            alias,
            import_path,
        }
    }

    /// Get the name to use when referencing this import (alias or package name)
    pub fn reference_name(&self) -> &str {
        self.alias.as_ref().unwrap_or(&self.package_name)
    }
}

/// Manages import statements and package mappings
#[derive(Debug)]
pub struct ImportResolver {
    /// Map from importing package to list of imported packages
    /// package_name -> Vec<ImportInfo>
    imports: HashMap<String, Vec<ImportInfo>>,
    
    /// Map from (importing_package, reference_name) to actual package name
    /// Used for quick lookup of what package a reference resolves to
    reference_map: HashMap<(String, String), String>,
    
    /// Map from package name to import path (for standard library packages)
    stdlib_packages: HashMap<String, String>,
}

impl ImportResolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            imports: HashMap::new(),
            reference_map: HashMap::new(),
            stdlib_packages: HashMap::new(),
        };
        
        resolver.init_stdlib_packages();
        resolver
    }

    /// Initialize standard library package mappings
    fn init_stdlib_packages(&mut self) {
        let stdlib_packages = vec![
            ("vibez", "std/vibez"),
            ("htmlrizzler", "std/htmlrizzler"),
            ("mathz", "std/mathz"),
            ("timez", "std/timez"),
            ("cryptz", "std/cryptz"),
            ("reflectz", "std/reflectz"),
            ("stringz", "std/stringz"),
            ("dropz", "std/dropz"),
            ("concurrenz", "std/concurrenz"),
            ("web_vibez", "std/web_vibez"),
            ("rizztemplate", "std/rizztemplate"),
            ("core", "std/core"),
        ];

        for (package, path) in stdlib_packages {
            self.stdlib_packages.insert(package.to_string(), path.to_string());
        }
    }

    /// Register an import statement
    #[tracing::instrument(skip(self), fields(current_package = %current_package, imported_package = %imported_package, alias = ?alias), level = "debug")]
    pub fn register_import(
        &mut self,
        current_package: &str,
        imported_package: &str,
        alias: Option<&str>,
    ) -> Result<(), Error> {
        // Resolve import path
        let import_path = self.resolve_import_path(imported_package)?;
        
        let import_info = ImportInfo::new(
            imported_package.to_string(),
            alias.map(|s| s.to_string()),
            import_path,
        );

        // Check for duplicate imports
        if let Some(existing_imports) = self.imports.get(current_package) {
            let reference_name = import_info.reference_name();
            
            for existing in existing_imports {
                if existing.reference_name() == reference_name {
                    return Err(Error::from_str(&format!(
                        "Import name '{}' conflicts with existing import in package '{}'",
                        reference_name, current_package
                    )));
                }
            }
        }

        // Add to imports list
        let imports_list = self.imports.entry(current_package.to_string())
            .or_insert_with(Vec::new);
        imports_list.push(import_info.clone());

        // Add to reference map
        let reference_name = import_info.reference_name().to_string();
        self.reference_map.insert(
            (current_package.to_string(), reference_name),
            imported_package.to_string(),
        );

        tracing::debug!(
            current_package = %current_package,
            imported_package = %imported_package,
            reference_name = %import_info.reference_name(),
            "Import registered"
        );

        Ok(())
    }

    /// Resolve what package a reference name points to
    #[tracing::instrument(skip(self), fields(current_package = %current_package, reference_name = %reference_name), level = "debug")]
    pub fn resolve_reference(&self, current_package: &str, reference_name: &str) -> Option<String> {
        self.reference_map.get(&(current_package.to_string(), reference_name.to_string()))
            .cloned()
    }

    /// Check if a package is imported by another package
    pub fn is_package_imported(&self, current_package: &str, target_package: &str) -> bool {
        self.imports.get(current_package)
            .map(|imports| {
                imports.iter().any(|import| import.package_name == target_package)
            })
            .unwrap_or(false)
    }

    /// Get all imports for a package
    pub fn get_imports(&self, package_name: &str) -> Vec<(String, Option<String>)> {
        self.imports.get(package_name)
            .map(|imports| {
                imports.iter()
                    .map(|import| (import.package_name.clone(), import.alias.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get import info by reference name
    pub fn get_import_info(&self, current_package: &str, reference_name: &str) -> Option<&ImportInfo> {
        self.imports.get(current_package)?
            .iter()
            .find(|import| import.reference_name() == reference_name)
    }

    /// Resolve import path (handles standard library packages)
    fn resolve_import_path(&self, package_name: &str) -> Result<String, Error> {
        // Check if it's a standard library package
        if let Some(stdlib_path) = self.stdlib_packages.get(package_name) {
            return Ok(stdlib_path.clone());
        }

        // For user packages, use the package name as path for now
        // In a real implementation, this would involve path resolution logic
        Ok(package_name.to_string())
    }

    /// Check if a package is a standard library package
    pub fn is_stdlib_package(&self, package_name: &str) -> bool {
        self.stdlib_packages.contains_key(package_name)
    }

    /// Get all standard library package names
    pub fn get_stdlib_packages(&self) -> Vec<String> {
        self.stdlib_packages.keys().cloned().collect()
    }

    /// Clear all imports for a package (useful for testing or recompilation)
    pub fn clear_imports(&mut self, package_name: &str) {
        if let Some(imports) = self.imports.remove(package_name) {
            // Remove from reference map
            for import in imports {
                let reference_name = import.reference_name().to_string();
                self.reference_map.remove(&(package_name.to_string(), reference_name));
            }
        }
    }

    /// Get import statistics
    pub fn get_import_stats(&self) -> (usize, usize) {
        let total_packages = self.imports.len();
        let total_imports = self.imports.values()
            .map(|imports| imports.len())
            .sum();
        (total_packages, total_imports)
    }
}

impl Default for ImportResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_registration() {
        let mut resolver = ImportResolver::new();
        
        // Register an import without alias
        resolver.register_import("main", "vibez", None).unwrap();
        
        // Check resolution
        assert_eq!(resolver.resolve_reference("main", "vibez"), Some("vibez".to_string()));
        assert!(resolver.is_package_imported("main", "vibez"));
        
        // Register an import with alias
        resolver.register_import("main", "htmlrizzler", Some("html")).unwrap();
        assert_eq!(resolver.resolve_reference("main", "html"), Some("htmlrizzler".to_string()));
    }

    #[test]
    fn test_import_conflicts() {
        let mut resolver = ImportResolver::new();
        
        resolver.register_import("main", "vibez", None).unwrap();
        
        // Try to import another package with same reference name
        let result = resolver.register_import("main", "othervibez", Some("vibez"));
        assert!(result.is_err());
    }

    #[test]
    fn test_stdlib_packages() {
        let resolver = ImportResolver::new();
        
        assert!(resolver.is_stdlib_package("vibez"));
        assert!(resolver.is_stdlib_package("cryptz"));
        assert!(!resolver.is_stdlib_package("user_package"));
        
        let stdlib_packages = resolver.get_stdlib_packages();
        assert!(stdlib_packages.contains(&"vibez".to_string()));
        assert!(stdlib_packages.contains(&"cryptz".to_string()));
    }

    #[test]
    fn test_import_info() {
        let mut resolver = ImportResolver::new();
        
        resolver.register_import("main", "cryptz", Some("crypto")).unwrap();
        
        let import_info = resolver.get_import_info("main", "crypto").unwrap();
        assert_eq!(import_info.package_name, "cryptz");
        assert_eq!(import_info.alias, Some("crypto".to_string()));
        assert_eq!(import_info.reference_name(), "crypto");
    }

    #[test]
    fn test_get_imports() {
        let mut resolver = ImportResolver::new();
        
        resolver.register_import("main", "vibez", None).unwrap();
        resolver.register_import("main", "cryptz", Some("crypto")).unwrap();
        
        let imports = resolver.get_imports("main");
        assert_eq!(imports.len(), 2);
        
        assert!(imports.contains(&("vibez".to_string(), None)));
        assert!(imports.contains(&("cryptz".to_string(), Some("crypto".to_string()))));
    }
}
