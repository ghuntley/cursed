use crate::error::CursedError;
// Import Resolution System
//
// Handles resolution of imports including:
// - Standard library imports
// - Package imports from installed dependencies
// - Local file imports
// - Type imports and re-exports

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::ast::ImportStatement;
// use crate::package_manager::{PackageManager, PackageMetadata, PackageManagerError};

pub mod resolver;
pub mod module_loader;
pub mod package_resolver;

pub use resolver::{ImportResolver, ImportResolverConfig, ResolvedImport, ImportSource};
pub use module_loader::{ModuleLoader, ModuleInfo, LoadedModule};
pub use package_resolver::{PackageImportResolver, PackageResolution};

/// Errors that can occur during import resolution
#[derive(CursedError, Debug)]
pub enum ImportError {
    #[error("Import not found: {import_path}")]
    
    #[error("Circular import detected: {cycle:?}")]
    
    #[error("Package not installed: {package}")]
    
    #[error("Invalid import path: {path} - {reason}")]
    
    #[error("Module load error: {module} - {error}")]
    
    #[error("Package manager error: {0}")]
    
    #[error("IO error: {0}")]
/// Main import resolution coordinator
#[derive(Debug)]
pub struct ImportManager {
    // package_manager: Arc<Mutex<PackageManager>>, // Disabled for minimal build
/// Cache for resolved imports to avoid repeated resolution
#[derive(Debug, Default)]
pub struct ImportCache {
impl ImportManager {
    /// Create new import manager with package manager integration
    pub fn new(
        // package_manager: Arc<Mutex<PackageManager>>, // Disabled for minimal build
    ) -> crate::error::Result<Self> {
        let resolver = ImportResolver::new(config);
        let package_resolver = PackageImportResolver::new(/* package_manager.clone() */);
        let module_loader = ModuleLoader::new();
        let cache = ImportCache::default();
        
        Ok(Self {
            // package_manager, // Disabled for minimal build
        })
    /// Resolve all imports for a program
    pub async fn resolve_imports(
    ) -> crate::error::Result<()> {
        let mut resolved = Vec::new();
        let mut processing = std::collections::HashSet::new();
        
        for import in imports {
            self.resolve_single_import(
                &mut processing
            ).await?;
        Ok(())
    /// Resolve a single import statement
    pub async fn resolve_single_import(
    ) -> crate::error::Result<()> {
        let import_path = &import.path;
        
        // Check cache first
        if let Some(_cached) = self.cache.resolved_imports.get(import_path) {
            return Ok(());
        // Detect circular imports
        if processing.contains(import_path) {
                processing.iter().cloned().collect::<Vec<_>>())));
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
        
        processing.remove(import_path);
        
        // Cache the result
        self.cache.resolved_imports.insert(import_path.clone(), resolved.clone());
        
        Ok(resolved)
    /// Load module for a resolved import
    pub async fn load_module(
    ) -> crate::error::Result<()> {
        let module_key = resolved.get_cache_key();
        
        // Check cache first
        if let Some(cached) = self.cache.loaded_modules.get(&module_key) {
            return Ok(cached.clone());
        // Load the module
        let loaded = self.module_loader.load_module(resolved).await?;
        let loaded_arc = Arc::new(loaded);
        
        // Cache the result
        self.cache.loaded_modules.insert(module_key, loaded_arc.clone());
        
        Ok(loaded_arc)
    /// Ensure package is installed before resolving its imports
    pub async fn ensure_package_installed(&mut self, package_name: &str) -> crate::error::Result<()> {
        let mut package_manager = self.package_manager.lock().map_err(|_| {
            ImportError::ModuleLoadError {
            }
        })?;
        
        // Check if already installed
        let installed = package_manager.list_installed()?;
        if let Some(metadata) = installed.iter().find(|p| p.name == package_name) {
            return Ok(metadata.clone());
        // Install the package
        let installed_packages = package_manager.install_package(package_name, None).await?;
        
        installed_packages.into_iter()
            .find(|p| p.name == package_name)
            .ok_or_else(|| ImportError::PackageNotInstalled {
            })
    /// Get all available packages for import resolution
    pub fn get_available_packages(&self) -> crate::error::Result<()> {
        let package_manager = self.package_manager.lock().map_err(|_| {
            ImportError::ModuleLoadError {
            }
        })?;
        
        Ok(package_manager.list_installed()?)
    /// Clear import cache
    pub fn clear_cache(&mut self) {
        self.cache.resolved_imports.clear();
        self.cache.loaded_modules.clear();
    /// Get import statistics
    pub fn get_stats(&self) -> ImportStats {
        ImportStats {
        }
    }
/// Import resolution statistics
#[derive(Debug, Clone)]
pub struct ImportStats {
}
