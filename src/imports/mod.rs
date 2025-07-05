// Import Management System for CURSED
//
// This module provides comprehensive import resolution functionality including:
// - Standard library imports
// - Local module imports  
// - Package manager integration
// - Circular dependency detection
// - Import caching for performance

// Re-export main types and functions
pub mod resolver;
pub mod module_loader;
pub mod package_resolver;

#[cfg(test)]
mod tests;

// Re-export main types for convenience
pub use resolver::{
    ImportResolver, ImportSource, ImportError as ResolverImportError,
    ResolvedImport, CompiledModule, ImportStats as ResolverImportStats, 
    resolve_program_imports, module_exists
};

pub use module_loader::{
    ModuleLoader, ModuleLoaderConfig, LoadedModule, ModuleCache,
    CacheStats, validate_module_file, find_modules_in_directory
};

pub use package_resolver::{
    PackageResolver, PackageResolverConfig, ResolvedPackage, 
    PackageDependency, PackageManifest, is_package_directory
};

use crate::error::{CursedError, Result};
use crate::ast::{Program, ImportStatement as AstImportStatement};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Arc;

/// Errors that can occur during import resolution
#[derive(Debug, Clone)]
pub enum ImportError {
    NotFound { import_path: String },
    CircularImport { cycle: Vec<String> },
    PackageNotInstalled { package: String },
    InvalidPath { path: String, reason: String },
    ModuleLoadError { module: String, error: String },
    PackageManagerError(String),
    IoError(String),
}

/// Legacy import manager for backwards compatibility
#[derive(Debug)]
pub struct ImportManager {
    resolver: ImportResolver,
    cache: ImportCache,
    config: ImportConfig,
}

/// Cache for resolved imports to avoid repeated resolution
#[derive(Debug, Default)]
pub struct ImportCache {
    resolved_imports: HashMap<String, PathBuf>,
    failed_imports: HashMap<String, ImportError>,
}

/// Import configuration type alias for consistency with resolver
pub use resolver::ImportConfig;

/// Alias for backward compatibility in examples
pub type ImportResolverConfig = ImportConfig;

impl ImportManager {
    /// Create new import manager with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(ImportConfig::default())
    }

    /// Create new import manager with custom configuration  
    pub fn with_config(config: ImportConfig) -> Result<Self> {
        let resolver = ImportResolver::with_config(config.clone())?;
        let cache = ImportCache::default();
        
        Ok(Self {
            resolver,
            cache,
            config,
        })
    }

    /// Resolve all imports for a program
    pub async fn resolve_imports(&mut self, imports: &[AstImportStatement]) -> Result<Vec<ResolvedImport>> {
        let mut resolved = Vec::new();
        let mut processing = HashSet::new();
        
        for import in imports {
            let resolved_import = self.resolve_single_import(import, &mut processing).await?;
            resolved.push(resolved_import);
        }
        
        Ok(resolved)
    }

    /// Resolve a single import statement
    pub async fn resolve_single_import(
        &mut self,
        import: &AstImportStatement,
        _processing: &mut HashSet<String>
    ) -> Result<ResolvedImport> {
        // Convert to resolver ImportStatement format  
        let resolver_import = crate::ast::ImportStatement {
            path: import.path.clone(),
            alias: import.alias.clone(),
            items: import.items.clone(),
        };
        
        // Use the resolver
        self.resolver.resolve_single_import(&resolver_import).await
    }

    /// Check if an import is cached
    pub fn is_cached(&self, import_path: &str) -> bool {
        self.cache.resolved_imports.contains_key(import_path)
    }

    /// Clear the import cache
    pub fn clear_cache(&mut self) {
        self.cache.resolved_imports.clear();
        self.cache.failed_imports.clear();
    }
    
    /// Get resolver instance for direct access
    pub fn resolver(&self) -> &ImportResolver {
        &self.resolver
    }

    /// Get resolver instance for direct mutable access
    pub fn resolver_mut(&mut self) -> &mut ImportResolver {
        &mut self.resolver
    }

    /// Get import statistics for examples
    pub fn get_stats(&self) -> ImportStats {
        ImportStats {
            cached_imports: self.cache.resolved_imports.len(),
            loaded_modules: self.cache.resolved_imports.len(),
            failed_imports: self.cache.failed_imports.len(),
        }
    }
}

/// Import statistics for examples
#[derive(Debug, Clone)]
pub struct ImportStats {
    pub cached_imports: usize,
    pub loaded_modules: usize,
    pub failed_imports: usize,
}

/// Represents an import statement
#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub path: String,
    pub alias: Option<String>,
    pub items: Vec<String>, // Specific items to import
    pub is_glob: bool, // Import everything (*)
}

impl Default for ImportManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default ImportManager")
    }
}

// Error conversion implementations
impl From<std::io::Error> for ImportError {
    fn from(err: std::io::Error) -> Self {
        ImportError::IoError(err.to_string())
    }
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::NotFound { import_path } => write!(f, "Import not found: {}", import_path),
            ImportError::CircularImport { cycle } => write!(f, "Circular import detected: {:?}", cycle),
            ImportError::PackageNotInstalled { package } => write!(f, "Package not installed: {}", package),
            ImportError::InvalidPath { path, reason } => write!(f, "Invalid import path: {} - {}", path, reason),
            ImportError::ModuleLoadError { module, error } => write!(f, "Module load error: {} - {}", module, error),
            ImportError::PackageManagerError(msg) => write!(f, "Package manager error: {}", msg),
            ImportError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ImportError {}
