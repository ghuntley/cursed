use crate::error::Error;
//! Import Resolution System
//!
//! Handles resolution of imports including:
//! - Standard library imports
//! - Package imports from installed dependencies
//! - Local file imports
//! - Type imports and re-exports

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use thiserror::Error;

use crate::ast::ImportStatement;
use crate::package_manager::{PackageManager, PackageMetadata, PackageManagerError};

pub mod resolver;
pub mod module_loader;
pub mod package_resolver;

pub use resolver::{ImportResolver, ImportResolverConfig, ResolvedImport, ImportSource};
pub use module_loader::{ModuleLoader, ModuleInfo, LoadedModule};
pub use package_resolver::{PackageImportResolver, PackageResolution};

/// Errors that can occur during import resolution
#[derive(Error, Debug)]
pub enum ImportError {
    #[error("Import not found: {import_path}")]
    NotFound { import_path: String },
    
    #[error("Circular import detected: {cycle:?}")]
    CircularImport { cycle: Vec<String> },
    
    #[error("Package not installed: {package}")]
    PackageNotInstalled { package: String },
    
    #[error("Invalid import path: {path} - {reason}")]
    InvalidPath { path: String, reason: String },
    
    #[error("Module load error: {module} - {error}")]
    ModuleLoadError { module: String, error: String },
    
    #[error("Package manager error: {0}")]
    PackageManager(#[from] PackageManagerError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Main import resolution coordinator
#[derive(Debug)]
pub struct ImportManager {
    resolver: ImportResolver,
    package_resolver: PackageImportResolver,
    module_loader: ModuleLoader,
    package_manager: Arc<Mutex<PackageManager>>,
    cache: ImportCache,
}

/// Cache for resolved imports to avoid repeated resolution
#[derive(Debug, Default)]
pub struct ImportCache {
    resolved_imports: HashMap<String, ResolvedImport>,
    loaded_modules: HashMap<String, Arc<LoadedModule>>,
}

impl ImportManager {
    /// Create new import manager with package manager integration
    pub fn new(
        package_manager: Arc<Mutex<PackageManager>>,
        config: ImportResolverConfig,
    ) -> Result<(), Error> {
        let resolver = ImportResolver::new(config);
        let package_resolver = PackageImportResolver::new(package_manager.clone());
        let module_loader = ModuleLoader::new();
        let cache = ImportCache::default();
        
        Ok(Self {
            resolver,
            package_resolver,
            module_loader,
            package_manager,
            cache,
        })
    }
    
    /// Resolve all imports for a program
    pub async fn resolve_imports(
        &mut self,
        imports: &[ImportStatement],
        context_path: Option<&Path>,
    ) -> Result<(), Error> {
        let mut resolved = Vec::new();
        let mut processing = std::collections::HashSet::new();
        
        for import in imports {
            let resolved_import = self.resolve_single_import(
                import, 
                context_path, 
                &mut processing
            ).await?;
            resolved.push(resolved_import);
        }
        
        Ok(resolved)
    }
    
    /// Resolve a single import statement
    pub async fn resolve_single_import(
        &mut self,
        import: &ImportStatement,
        context_path: Option<&Path>,
        processing: &mut std::collections::HashSet<String>,
    ) -> Result<(), Error> {
        let import_path = &import.path;
        
        // Check cache first
        if let Some(cached) = self.cache.resolved_imports.get(import_path) {
            return Ok(cached.clone());
        }
        
        // Detect circular imports
        if processing.contains(import_path) {
            return Err(ImportError::CircularImport {
                cycle: processing.iter().cloned().collect(),
            });
        }
        processing.insert(import_path.clone());
        
        // Try different resolution strategies
        let resolved = if import_path.starts_with("stdlib::") {
            // Standard library import
            self.resolver.resolve_stdlib_import(import_path)?
        } else if import_path.contains("::") {
            // Package import (e.g., "cursed-http::client")
            self.package_resolver.resolve_package_import(import_path).await?
        } else {
            // Local file import
            self.resolver.resolve_local_import(import_path, context_path)?
        };
        
        processing.remove(import_path);
        
        // Cache the result
        self.cache.resolved_imports.insert(import_path.clone(), resolved.clone());
        
        Ok(resolved)
    }
    
    /// Load module for a resolved import
    pub async fn load_module(
        &mut self,
        resolved: &ResolvedImport,
    ) -> Result<(), Error> {
        let module_key = resolved.get_cache_key();
        
        // Check cache first
        if let Some(cached) = self.cache.loaded_modules.get(&module_key) {
            return Ok(cached.clone());
        }
        
        // Load the module
        let loaded = self.module_loader.load_module(resolved).await?;
        let loaded_arc = Arc::new(loaded);
        
        // Cache the result
        self.cache.loaded_modules.insert(module_key, loaded_arc.clone());
        
        Ok(loaded_arc)
    }
    
    /// Ensure package is installed before resolving its imports
    pub async fn ensure_package_installed(&mut self, package_name: &str) -> Result<(), Error> {
        let mut package_manager = self.package_manager.lock().map_err(|_| {
            ImportError::ModuleLoadError {
                module: package_name.to_string(),
                error: "Failed to lock package manager".to_string(),
            }
        })?;
        
        // Check if already installed
        let installed = package_manager.list_installed()?;
        if let Some(metadata) = installed.iter().find(|p| p.name == package_name) {
            return Ok(metadata.clone());
        }
        
        // Install the package
        let installed_packages = package_manager.install_package(package_name, None).await?;
        
        installed_packages.into_iter()
            .find(|p| p.name == package_name)
            .ok_or_else(|| ImportError::PackageNotInstalled {
                package: package_name.to_string(),
            })
    }
    
    /// Get all available packages for import resolution
    pub fn get_available_packages(&self) -> Result<(), Error> {
        let package_manager = self.package_manager.lock().map_err(|_| {
            ImportError::ModuleLoadError {
                module: "package_manager".to_string(),
                error: "Failed to lock package manager".to_string(),
            }
        })?;
        
        Ok(package_manager.list_installed()?)
    }
    
    /// Clear import cache
    pub fn clear_cache(&mut self) {
        self.cache.resolved_imports.clear();
        self.cache.loaded_modules.clear();
    }
    
    /// Get import statistics
    pub fn get_stats(&self) -> ImportStats {
        ImportStats {
            cached_imports: self.cache.resolved_imports.len(),
            loaded_modules: self.cache.loaded_modules.len(),
        }
    }
}

/// Import resolution statistics
#[derive(Debug, Clone)]
pub struct ImportStats {
    pub cached_imports: usize,
    pub loaded_modules: usize,
}
