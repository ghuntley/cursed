use crate::error::CursedError;
// Package import resolution

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

// use crate::package_manager::{PackageManager, PackageMetadata};
use super::{ImportError, ResolvedImport, ImportSource};

/// Package-specific import resolution
#[derive(Debug)]
pub struct PackageImportResolver {
/// Information about what a package exports
#[derive(Debug, Clone)]
pub struct PackageExportInfo {
/// Information about what a module within a package exports
#[derive(Debug, Clone)]
pub struct ModuleExportInfo {
/// Result of package resolution
#[derive(Debug, Clone)]
pub struct PackageResolution {
impl PackageImportResolver {
    /// Create new package import resolver
    pub fn new(package_manager: Arc<Mutex<PackageManager>>) -> Self {
        let mut resolver = Self {
        
        resolver.init_known_package_exports();
        resolver
    /// Initialize known package export information
    fn init_known_package_exports(&mut self) {
        // cursed-http package exports
        let mut http_modules = HashMap::new();
        http_modules.insert("client".to_string(), ModuleExportInfo {
            functions: vec![
            types: vec![
        });
        
        http_modules.insert("server".to_string(), ModuleExportInfo {
            functions: vec![
            types: vec![
        });
        
        self.package_exports.insert("cursed-http".to_string(), PackageExportInfo {
        });
        
        // cursed-json package exports
        let mut json_modules = HashMap::new();
        json_modules.insert("parse".to_string(), ModuleExportInfo {
            functions: vec![
            types: vec![
        });
        
        self.package_exports.insert("cursed-json".to_string(), PackageExportInfo {
        });
        
        // cursed-db package exports
        let mut db_modules = HashMap::new();
        db_modules.insert("sql".to_string(), ModuleExportInfo {
            functions: vec![
            types: vec![
        });
        
        self.package_exports.insert("cursed-db".to_string(), PackageExportInfo {
        });
    /// Resolve package import (e.g., "cursed-http::client")
    pub async fn resolve_package_import(&self, import_path: &str) -> crate::error::Result<()> {
        // Parse package import path
        let parts: Vec<&str> = import_path.split("::").collect();
        if parts.len() < 2 {
            return Err(ImportError::InvalidPath {
            });
        let package_name = parts[0];
        let module_path = parts[1..].join("::");
        
        // Ensure package is installed
        let package_metadata = self.ensure_package_available(package_name).await?;
        
        // Get package export information
        let package_exports = self.package_exports.get(package_name)
            .ok_or_else(|| ImportError::NotFound {
            })?;
        
        // Find the specific module
        let module_info = package_exports.modules.get(&module_path)
            .or_else(|| {
                // Try finding by the last part of the path
                let module_name = parts.last().unwrap();
                package_exports.modules.get(*module_name)
            })
            .ok_or_else(|| ImportError::NotFound {
            })?;
        
        // Build resolved path to the package module
        let cache_dir = dirs::cache_dir().unwrap_or_default().join("cursed/packages");
        let package_dir = cache_dir.join(&package_name).join(&package_metadata.version);
        let module_file_path = package_dir.join("src").join(&module_info.module_path).with_extension("csd");
        
        // Combine exports
        let mut all_exports = module_info.functions.clone();
        all_exports.extend(module_info.constants.clone());
        
        Ok(ResolvedImport {
            source: ImportSource::InstalledPackage {
        })
    /// Ensure package is available for import
    async fn ensure_package_available(&self, package_name: &str) -> crate::error::Result<()> {
        let mut package_manager = self.package_manager.lock().map_err(|_| {
            ImportError::ModuleLoadError {
            }
        })?;
        
        // Check if package is already installed
        let installed = package_manager.list_installed()?;
        if let Some(metadata) = installed.iter().find(|p| p.name == package_name) {
            return Ok(metadata.clone());
        // Package not installed, try to install it
        tracing::info!(package = package_name, "Package not found, attempting to install");
        
        let installed_packages = package_manager.install_package(package_name, None).await?;
        
        installed_packages.into_iter()
            .find(|p| p.name == package_name)
            .ok_or_else(|| ImportError::PackageNotInstalled {
            })
    /// Update package export information from installed packages
    pub async fn update_package_exports(&mut self) -> crate::error::Result<()> {
        let installed = {
            let package_manager = self.package_manager.lock().map_err(|_| {
                ImportError::ModuleLoadError {
                }
            })?;
            package_manager.list_installed()?
        
        for package in installed {
            if !self.package_exports.contains_key(&package.name) {
                // Try to discover exports for unknown packages
                self.discover_package_exports(&package)?;
            }
        }
        
        Ok(())
    /// Discover exports for an unknown package
    fn discover_package_exports(&mut self, package: &PackageMetadata) -> crate::error::Result<()> {
        // In a real implementation, this would parse the package files
        // For now, create a basic export info
        let mut modules = HashMap::new();
        modules.insert("main".to_string(), ModuleExportInfo {
        });
        
        let export_info = PackageExportInfo {
        
        self.package_exports.insert(package.name.clone(), export_info);
        
        tracing::info!(package = package.name, "Discovered basic exports for package");
        Ok(())
    /// Get available packages for completion/suggestions
    pub fn get_available_packages(&self) -> Vec<String> {
        self.package_exports.keys().cloned().collect()
    /// Get available modules for a package
    pub fn get_package_modules(&self, package_name: &str) -> Option<Vec<String>> {
        self.package_exports.get(package_name)
            .map(|exports| exports.modules.keys().cloned().collect())
    /// Get exports for a specific package module
    pub fn get_module_exports(&self, package_name: &str, module_name: &str) -> Option<&ModuleExportInfo> {
        self.package_exports.get(package_name)
            .and_then(|exports| exports.modules.get(module_name))
    }
}

