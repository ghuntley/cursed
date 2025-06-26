// Import Management System for CURSED
//
// This module provides comprehensive import resolution functionality including:
// - Standard library imports
// - Local module imports  
// - Package manager integration
// - Circular dependency detection
// - Import caching for performance

// Re-export sub-modules - TODO: Enable once modules are implemented
// pub mod import_resolver;
// pub mod package_import_resolver; 
// pub mod module_loader;

// Placeholder types for missing imports
#[derive(Debug, Clone)]
pub struct ImportResolver {
    pub config: ImportResolverConfig,
}

#[derive(Debug, Clone)]
pub struct ImportResolverConfig {
    pub search_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ResolvedImport {
    pub path: PathBuf,
    pub module: LoadedModule,
}

#[derive(Debug, Clone)]
pub struct LoadedModule {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum ImportSource {
    Local(PathBuf),
    Package(String),
    Stdlib(String),
}

#[derive(Debug)]
pub struct ModuleLoader {
    pub cache: HashMap<String, LoadedModule>,
}

#[derive(Debug)]
pub struct PackageImportResolver {
    pub packages: HashMap<String, PathBuf>,
}

impl ImportResolver {
    pub fn new(config: ImportResolverConfig) -> Self {
        Self { config }
    }
}

impl ImportResolverConfig {
    pub fn default() -> Self {
        Self {
            search_paths: vec![PathBuf::from(".")],
        }
    }
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }
}

impl PackageImportResolver {
    pub fn new() -> Self {
        Self { packages: HashMap::new() }
    }
}

use crate::error::{CursedError, Result};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

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

/// Main import resolution coordinator
#[derive(Debug)]
pub struct ImportManager {
    resolver: Option<()>, // ImportResolver, // TODO: Enable once implemented
    package_resolver: Option<()>, // PackageImportResolver, // TODO: Enable once implemented  
    module_loader: Option<()>, // ModuleLoader, // TODO: Enable once implemented
    cache: ImportCache,
}

/// Cache for resolved imports to avoid repeated resolution
#[derive(Debug, Default)]
pub struct ImportCache {
    resolved_imports: HashMap<String, PathBuf>,
    failed_imports: HashMap<String, ImportError>,
}

/// Configuration for import resolution
#[derive(Debug, Clone)]
pub struct ImportConfig {
    pub search_paths: Vec<PathBuf>,
    pub stdlib_path: PathBuf,
    pub enable_package_manager: bool,
    pub cache_enabled: bool,
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            search_paths: vec![PathBuf::from(".")],
            stdlib_path: PathBuf::from("stdlib"),
            enable_package_manager: false, // Disabled for minimal build
            cache_enabled: true,
        }
    }
}

impl ImportManager {
    /// Create new import manager with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(ImportConfig::default())
    }

    /// Create new import manager with custom configuration  
    pub fn with_config(config: ImportConfig) -> Result<Self> {
        // TODO: Initialize components once modules are implemented
        // let resolver = ImportResolver::new(config.clone());
        // let package_resolver = PackageImportResolver::new();
        // let module_loader = ModuleLoader::new();
        let cache = ImportCache::default();
        
        Ok(Self {
            resolver: None, // Some(resolver),
            package_resolver: None, // Some(package_resolver),
            module_loader: None, // Some(module_loader),
            cache,
        })
    }

    /// Resolve all imports for a program - TODO: Implement once modules are ready
    pub async fn resolve_imports(&mut self, _imports: &[ImportStatement]) -> Result<()> {
        // TODO: Implement import resolution
        Ok(())
    }

    /// Resolve a single import statement - TODO: Implement once modules are ready
    pub async fn resolve_single_import(
        &mut self,
        _import: &ImportStatement,
        _processing: &mut HashSet<String>
    ) -> Result<()> {
        // TODO: Implement single import resolution
        Ok(())
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
