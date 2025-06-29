// Package Manager System for CURSED
//
// This module provides package management functionality including:
// - Installing packages from registries
// - Managing dependencies and versions
// - Caching downloaded packages
// - Resolving package conflicts

// Re-export sub-modules
pub mod registry;
pub mod resolver;
pub mod downloader;
pub mod cache;
pub mod version;
pub mod installer;

#[cfg(test)]
mod tests;

// Import and re-export main types
pub use registry::{PackageRegistry, PackageInfo, RegistryConfig};
pub use resolver::{PackageResolver, ResolvedPackage, ResolutionResult, ResolutionConfig};
pub use downloader::{PackageDownloader, DownloadedPackage, DownloadConfig};
pub use cache::{PackageCache, CachedPackage, CacheConfig};
pub use version::{Version, VersionReq};
pub use installer::{PackageInstaller, InstallConfig, InstalledPackageInfo, InstallResult, UninstallResult};

use std::collections::HashMap;
use std::str::FromStr;

/// Main package manager
#[derive(Debug)]
pub struct PackageManager {
    registry: PackageRegistry,
    resolver: PackageResolver,
    downloader: PackageDownloader,
    cache: PackageCache,
    installer: PackageInstaller,
    config: PackageManagerConfig,
}

/// Configuration for the package manager
#[derive(Debug, Clone)]
pub struct PackageManagerConfig {
    pub cache_dir: String,
    pub registry_url: String,
    pub offline_mode: bool,
    pub verify_signatures: bool,
}

/// Legacy type alias for backward compatibility
pub type InstalledPackage = InstalledPackageInfo;

impl Default for PackageManagerConfig {
    fn default() -> Self {
        Self {
            cache_dir: "target/packages".to_string(),
            registry_url: "https://packages.cursed-lang.org".to_string(),
            offline_mode: false,
            verify_signatures: true,
        }
    }
}

impl PackageManager {
    /// Create new package manager with configuration
    pub fn new(config: PackageManagerConfig) -> crate::error::Result<Self> {
        // Initialize all components
        let registry_config = RegistryConfig {
            url: config.registry_url.clone(),
            timeout: std::time::Duration::from_secs(30),
            max_retries: 3,
            api_key: None,
        };
        let registry = PackageRegistry::new(registry_config)?;
        
        let resolver = PackageResolver::new(registry.clone());
        
        let download_config = DownloadConfig::default();
        let downloader = PackageDownloader::new(download_config.clone())?;
        
        let cache_config = CacheConfig {
            cache_dir: std::path::PathBuf::from(&config.cache_dir),
            ..Default::default()
        };
        let cache = PackageCache::new(cache_config)?;
        
        let install_config = InstallConfig {
            install_dir: std::path::PathBuf::from(&config.cache_dir).join("installed"),
            temp_dir: std::path::PathBuf::from(&config.cache_dir).join("temp"),
            ..Default::default()
        };
        let installer_downloader = PackageDownloader::new(download_config.clone())?;
        let installer = PackageInstaller::new(install_config, installer_downloader)?;

        Ok(Self {
            registry,
            resolver,
            downloader,
            cache,
            installer,
            config,
        })
    }

    /// Install a package with optional version specification
    pub async fn install_package(&mut self, name: &str, version: Option<&str>) -> crate::error::Result<InstalledPackage> {
        tracing::info!("Installing package: {} (version: {:?})", name, version);

        // Check if already installed
        if let Some(installed) = self.installer.get_installed_package(name) {
            let version_str = version.unwrap_or("latest");
            if version.is_none() || installed.version.to_string() == version_str {
                tracing::info!("Package {} already installed", name);
                return Ok(installed.clone());
            }
        }

        // Parse version requirement
        let version_req = if let Some(v) = version {
            VersionReq::parse(v)?
        } else {
            VersionReq::Any
        };

        // Resolve dependencies
        let root_packages = vec![(name.to_string(), version_req)];
        let resolution = self.resolver.resolve_dependencies(root_packages, ResolutionConfig::default()).await?;

        // Install resolved packages
        let install_result = self.installer.install_packages(resolution).await?;

        // Return the installed package info for the requested package
        install_result.installed_packages.into_iter()
            .find(|p| p.name == name)
            .ok_or_else(|| crate::error::CursedError::General(format!("Failed to install package: {}", name)))
    }

    /// Uninstall a package
    pub async fn uninstall_package(&mut self, name: &str) -> crate::error::Result<()> {
        tracing::info!("Uninstalling package: {}", name);

        let uninstall_result = self.installer.uninstall_package(name, false).await?;
        
        if uninstall_result.removed_packages.contains(&name.to_string()) {
            tracing::info!("Package {} uninstalled", name);
            Ok(())
        } else {
            Err(crate::error::CursedError::General(format!("Failed to uninstall package: {}", name)))
        }
    }

    /// List all installed packages
    pub fn list_installed(&self) -> Vec<&InstalledPackage> {
        self.installer.list_installed_packages()
    }

    /// Search for packages in the registry
    pub async fn search_packages(&self, query: &str) -> crate::error::Result<Vec<PackageInfo>> {
        tracing::info!("Searching packages for: {}", query);
        self.registry.search_packages(query).await
    }

    /// Update a specific package to latest version
    pub async fn update_package(&mut self, name: &str) -> crate::error::Result<InstalledPackage> {
        tracing::info!("Updating package: {}", name);
        
        // Get latest version from registry
        let latest_version = self.registry.get_latest_version(name).await?;
        
        // Install the latest version (this will upgrade if different)
        self.install_package(name, Some(&latest_version.to_string())).await
    }

    /// Update all installed packages
    pub async fn update_all(&mut self) -> crate::error::Result<Vec<InstalledPackage>> {
        let packages: Vec<String> = self.installer.list_installed_packages()
            .into_iter()
            .map(|p| p.name.clone())
            .collect();
        
        let mut updated = Vec::new();

        for package_name in packages {
            match self.update_package(&package_name).await {
                Ok(updated_package) => {
                    updated.push(updated_package);
                }
                Err(e) => {
                    tracing::warn!("Failed to update package {}: {}", package_name, e);
                }
            }
        }

        Ok(updated)
    }

    /// Check if a package is installed
    pub fn is_installed(&self, name: &str) -> bool {
        self.installer.is_package_installed(name)
    }

    /// Get package information for an installed package
    pub fn get_installed_package(&self, name: &str) -> Option<&InstalledPackage> {
        self.installer.get_installed_package(name)
    }

    /// Get latest version of a package from registry
    pub async fn get_latest_version(&self, name: &str) -> crate::error::Result<Version> {
        self.registry.get_latest_version(name).await
    }

    /// Get package information from registry
    pub async fn get_package_info(&self, name: &str, version: Option<&str>) -> crate::error::Result<PackageInfo> {
        let parsed_version = if let Some(v) = version {
            Some(Version::from_str(v)?)
        } else {
            None
        };
        
        self.registry.get_package_info(name, parsed_version.as_ref()).await
    }
}

impl Default for PackageManager {
    fn default() -> Self {
        Self::new(PackageManagerConfig::default())
            .expect("Failed to create default PackageManager")
    }
}
