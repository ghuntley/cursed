// Package management system for CURSED
use crate::error_types::CursedError;
use std::collections::HashMap;
use std::path::PathBuf;

pub mod config;
pub mod resolver;
pub mod downloader;
pub mod registry;
pub mod cache;
pub mod dependency_graph;
pub mod version;

// Re-export main types
pub use config::PackageManagerConfig;
pub use resolver::PackageResolver;
pub use downloader::PackageDownloader;
pub use registry::PackageRegistry;
pub use cache::PackageCache;
pub use version::Version;

/// Main package manager
#[derive(Debug)]
pub struct PackageManager {
impl PackageManager {
    pub fn new(config: PackageManagerConfig) -> crate::error_types::Result<Self> {
        let resolver = PackageResolver::new(config.clone())?;
        let downloader = PackageDownloader::new(config.clone())?;
        let registry = PackageRegistry::new(config.clone())?;
        let cache = PackageCache::new(config.clone())?;

        Ok(Self {
        })
    pub fn install_package(&mut self, name: &str, version: Option<&str>) -> crate::error_types::Result<InstalledPackage> {
        tracing::info!("Installing package: {} (version: {:?})", name, version);

        // Check if already installed
        if let Some(installed) = self.installed_packages.get(name) {
            if version.is_none() || installed.version.to_string() == version.unwrap() {
                tracing::info!("Package {} already installed", name);
                return Ok(installed.clone());
            }
        }

        // Resolve dependencies
        let resolved = self.resolver.resolve(name, version)?;
        
        // Download package
        let downloaded = self.downloader.download(&resolved)?;
        
        // Install to cache
        let installed = self.cache.install(downloaded)?;
        
        // Update registry
        self.registry.register_installed(&installed)?;
        
        // Track installation
        self.installed_packages.insert(name.to_string(), installed.clone());
        
        Ok(installed)
    pub fn uninstall_package(&mut self, name: &str) -> crate::error_types::Result<()> {
        tracing::info!("Uninstalling package: {}", name);

        if let Some(installed) = self.installed_packages.remove(name) {
            self.cache.remove(&installed)?;
            self.registry.unregister(&installed.name)?;
        Ok(())
    pub fn list_installed(&self) -> Vec<&InstalledPackage> {
        self.installed_packages.values().collect()
    pub fn search_packages(&self, query: &str) -> crate::error_types::Result<Vec<PackageInfo>> {
        self.registry.search(query)
    pub fn update_package(&mut self, name: &str) -> crate::error_types::Result<InstalledPackage> {
        // Find latest version
        let latest = self.registry.get_latest_version(name)?;
        
        // Install latest version (this will replace the old one)
        self.install_package(name, Some(&latest.to_string()))
    pub fn update_all(&mut self) -> crate::error_types::Result<Vec<InstalledPackage>> {
        let packages: Vec<String> = self.installed_packages.keys().cloned().collect();
        let mut updated = Vec::new();

        for package_name in packages {
            match self.update_package(&package_name) {
            }
        }

        Ok(updated)
    }
}

/// Installed package information
#[derive(Debug, Clone)]
pub struct InstalledPackage {
/// Package information from registry
#[derive(Debug, Clone)]
pub struct PackageInfo {
/// Package metadata
#[derive(Debug, Clone)]
pub struct PackageMetadata {
impl Default for PackageMetadata {
    fn default() -> Self {
        Self {
        }
    }
/// Package resolution result
#[derive(Debug, Clone)]
pub struct ResolvedPackage {
/// Package source types
#[derive(Debug, Clone)]
pub enum PackageSource {
/// Downloaded package
#[derive(Debug, Clone)]
pub struct DownloadedPackage {
}
