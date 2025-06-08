//! Package loading and compilation

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use tracing::instrument;
use crate::ast::base::Program;
use crate::lexer::{Lexer, Token};
use crate::parser::Parser;
use crate::ast::FunctionStatement;
use crate::ast::traits::Statement;
use super::symbol_table::PackageSymbolTable;
use super::errors::{ResolverError, ResolverResult};

/// Loads and compiles packages from the filesystem
#[derive(Debug)]
pub struct PackageLoader {
    /// Cache of loaded packages to avoid recompilation
    loaded_packages: HashMap<PathBuf, PackageSymbolTable>,
    /// Package search paths
    search_paths: Vec<PathBuf>,
}

impl PackageLoader {
    /// Create a new package loader
    pub fn new() -> Self {
        Self {
            loaded_packages: HashMap::new(),
            search_paths: Vec::new(),
        }
    }
    
    /// Add a search path for packages
    pub fn add_search_path<P: AsRef<Path>>(&mut self, path: P) {
        self.search_paths.push(path.as_ref().to_path_buf());
    }
    
    /// Load a package from a file path
    #[instrument(skip(self), level = "debug")]
    pub fn load_package_from_path<P: AsRef<Path> + std::fmt::Debug>(&mut self, path: P) -> ResolverResult<PackageSymbolTable> {
        let path = path.as_ref().to_path_buf();
        
        // Check cache first
        if let Some(cached) = self.loaded_packages.get(&path) {
            tracing::debug!("Returning cached package from {}", path.display());
            return Ok(cached.clone());
        }
        
        tracing::debug!("Loading package from {}", path.display());
        
        // Read the file
        let content = fs::read_to_string(&path)
            .map_err(|e| ResolverError::io_error(
                &path.file_stem().unwrap_or_default().to_string_lossy(),
                &e.to_string(),
            ))?;
        
        // Parse the package
        let program = self.parse_package_content(&content, &path)?;
        
        // Extract package name from the first package statement or file name
        let package_name = self.extract_package_name(&program, &path);
        
        // Build symbol table
        let symbol_table = self.build_symbol_table(&package_name, &program)?;
        
        // Cache the result
        self.loaded_packages.insert(path.clone(), symbol_table.clone());
        
        Ok(symbol_table)
    }
    
    /// Parse package content into an AST
    #[instrument(skip(self, content), level = "debug")]
    fn parse_package_content(&self, content: &str, path: &Path) -> ResolverResult<Program> {
        // Parse
        let mut lexer = Lexer::new(content);
        let mut parser = Parser::new(&mut lexer)
            .map_err(|e| ResolverError::compilation_error(
                &path.file_stem().unwrap_or_default().to_string_lossy(),
                &format!("Parser creation error: {}", e),
            ))?;
        let program = parser.parse_program()
            .map_err(|e| ResolverError::compilation_error(
                &path.file_stem().unwrap_or_default().to_string_lossy(),
                &format!("Parser error: {}", e),
            ))?;
        
        Ok(program)
    }
    
    /// Extract package name from AST or file path
    fn extract_package_name(&self, program: &Program, path: &Path) -> String {
        // First, try to find a package statement in the AST
        for stmt in &program.statements {
            if let Some(pkg_stmt) = stmt.as_any().downcast_ref::<crate::ast::statements::PackageStatement>() {
                return pkg_stmt.name.value.clone();
            }
        }
        
        // Fall back to file name without extension
        path.file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string()
    }
    
    /// Build symbol table from AST
    #[instrument(skip(self, program), level = "debug")]
    fn build_symbol_table(&self, package_name: &str, program: &Program) -> ResolverResult<PackageSymbolTable> {
        let mut symbol_table = PackageSymbolTable::new(package_name.to_string());
        
        // Extract functions
        let mut functions = Vec::new();
        for stmt in &program.statements {
            if let Some(func_stmt) = stmt.as_any().downcast_ref::<FunctionStatement>() {
                functions.push(func_stmt.clone());
            }
        }
        
        // Build symbol table from functions
        let table = PackageSymbolTable::from_program_with_name(program, package_name.to_string());
        
        // TODO: Extract other symbol types (types, constants, variables, interfaces)
        // TODO: Extract import dependencies
        // TODO: Extract documentation
        
        Ok(table)
    }
    
    /// Resolve a package by name, searching in configured paths
    #[instrument(skip(self), level = "debug")]
    pub fn resolve_package(&mut self, package_name: &str) -> ResolverResult<PackageSymbolTable> {
        let mut searched_paths = Vec::new();
        
        // Try each search path
        for search_path in &self.search_paths.clone() {
            let candidates = vec![
                search_path.join(format!("{}.csd", package_name)),
                search_path.join(package_name).join("main.csd"),
                search_path.join(package_name).join(format!("{}.csd", package_name)),
            ];
            
            for candidate in candidates {
                searched_paths.push(candidate.clone());
                
                if candidate.exists() {
                    tracing::debug!("Found package {} at {}", package_name, candidate.display());
                    return self.load_package_from_path(candidate);
                }
            }
        }
        
        Err(ResolverError::package_not_found(
            package_name, 
            searched_paths.into_iter().map(|p| p.display().to_string()).collect()
        ))
    }
    
    /// Get all loaded packages
    pub fn loaded_packages(&self) -> &HashMap<PathBuf, PackageSymbolTable> {
        &self.loaded_packages
    }
    
    /// Clear the package cache
    pub fn clear_cache(&mut self) {
        self.loaded_packages.clear();
    }
    
    /// Check if a package is loaded
    pub fn is_package_loaded(&self, path: &Path) -> bool {
        self.loaded_packages.contains_key(path)
    }
    
    /// Get default search paths
    pub fn default_search_paths() -> Vec<PathBuf> {
        vec![
            PathBuf::from("."),           // Current directory
            PathBuf::from("./packages"),  // Local packages directory
            PathBuf::from("./vendor"),    // Vendor directory
            // TODO: Add system-wide package directories
        ]
    }
    
    /// Initialize with default search paths
    pub fn with_default_paths() -> Self {
        let mut loader = Self::new();
        for path in Self::default_search_paths() {
            loader.add_search_path(path);
        }
        loader
    }
    
    /// Validate package structure
    #[instrument(skip(self), level = "debug")]
    pub fn validate_package(&self, symbol_table: &PackageSymbolTable) -> ResolverResult<()> {
        // Check that the package has a valid name
        if symbol_table.package_name().is_empty() {
            return Err(ResolverError::generic("Package name cannot be empty"));
        }
        
        // Check that all symbols have valid names
        for symbol in symbol_table.all_symbols() {
            if symbol.name().is_empty() {
                return Err(ResolverError::generic("Symbol name cannot be empty"));
            }
        }
        
        // TODO: Add more validation rules
        // - Check for duplicate symbols
        // - Validate symbol types
        // - Check export consistency
        
        Ok(())
    }
    
    /// Get package statistics
    pub fn get_package_stats(&self, symbol_table: &PackageSymbolTable) -> PackageStats {
        let functions = symbol_table.functions().len();
        let types = symbol_table.types().len();
        let constants = symbol_table.constants().len();
        let exported_symbols = symbol_table.exported_symbols().len();
        let total_symbols = symbol_table.all_symbols().len();
        
        PackageStats {
            package_name: symbol_table.package_name().to_string(),
            version: "1.0.0".to_string(),
            total_symbols,
            functions,
            types,
            constants,
            exported_symbols,
            dependencies: 0, // Dependencies are now tracked separately
        }
    }
}

impl Default for PackageLoader {
    fn default() -> Self {
        Self::with_default_paths()
    }
}

/// Statistics about a loaded package
#[derive(Debug, Clone)]
pub struct PackageStats {
    pub package_name: String,
    pub version: String,
    pub total_symbols: usize,
    pub functions: usize,
    pub types: usize,
    pub constants: usize,
    pub exported_symbols: usize,
    pub dependencies: usize,
}

impl std::fmt::Display for PackageStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Package {} v{}: {} symbols ({} functions, {} types, {} constants), {} exported, {} dependencies",
            self.package_name,
            self.version,
            self.total_symbols,
            self.functions,
            self.types,
            self.constants,
            self.exported_symbols,
            self.dependencies
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::env;
    
    #[test]
    fn test_package_loader_creation() {
        let loader = PackageLoader::new();
        assert!(loader.loaded_packages.is_empty());
        assert!(loader.search_paths.is_empty());
    }
    
    #[test]
    fn test_default_search_paths() {
        let paths = PackageLoader::default_search_paths();
        assert!(!paths.is_empty());
        assert!(paths.contains(&PathBuf::from(".")));
    }
    
    #[test]
    fn test_package_loader_with_default_paths() {
        let loader = PackageLoader::with_default_paths();
        assert!(!loader.search_paths.is_empty());
    }
    
    #[test]
    fn test_add_search_path() {
        let mut loader = PackageLoader::new();
        loader.add_search_path("/test/path");
        
        assert_eq!(loader.search_paths.len(), 1);
        assert_eq!(loader.search_paths[0], PathBuf::from("/test/path"));
    }
    
    #[test]
    fn test_package_stats_display() {
        let stats = PackageStats {
            package_name: "test".to_string(),
            version: "1.0.0".to_string(),
            total_symbols: 10,
            functions: 5,
            types: 3,
            constants: 2,
            exported_symbols: 8,
            dependencies: 1,
        };
        
        let display = stats.to_string();
        assert!(display.contains("test"));
        assert!(display.contains("1.0.0"));
        assert!(display.contains("10 symbols"));
    }
}
