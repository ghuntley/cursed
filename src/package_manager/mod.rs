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
pub mod archive;
pub mod config;
pub mod workspace;
pub mod lock_file;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod config_test;

// Import and re-export main types
pub use registry::{PackageRegistry, PackageInfo, RegistryConfig, PackageMetadata};
pub use resolver::{PackageResolver, ResolvedPackage, ResolutionResult, ResolutionConfig};
pub use downloader::{PackageDownloader, DownloadedPackage, DownloadConfig};
pub use cache::{PackageCache, CachedPackage, CacheConfig};
pub use version::{Version, VersionReq};
pub use installer::{PackageInstaller, InstallConfig, InstalledPackageInfo, InstallResult, UninstallResult};
pub use workspace::{WorkspaceManager, WorkspaceConfig, WorkspaceMember, WorkspacePackageMetadata};
pub use lock_file::{LockFileManager, LockedPackage, LockFileStats, LockFileValidation};

use std::collections::HashMap;
use std::str::FromStr;
use std::path::PathBuf;

/// Main package manager
#[derive(Debug)]
pub struct PackageManager {
    registry: PackageRegistry,
    resolver: PackageResolver,
    downloader: PackageDownloader,
    cache: PackageCache,
    installer: PackageInstaller,
    config: PackageManagerConfig,
    workspace: Option<WorkspaceManager>,
    lock_file_manager: Option<LockFileManager>,
}

/// Configuration for the package manager
#[derive(Debug, Clone)]
pub struct PackageManagerConfig {
    pub cache_dir: PathBuf,
    pub registry_url: String,
    pub offline_mode: bool,
    pub verify_signatures: bool,
    pub workspace_dir: PathBuf,
    pub max_cache_size: usize,
    pub timeout_seconds: u64,
    pub parallel_downloads: u32,
}

/// Version specification type
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum VersionSpec {
    Simple(String),
    Range(String),
    Git { url: String, branch: Option<String> },
}

/// Legacy type alias for backward compatibility
pub type InstalledPackage = InstalledPackageInfo;

/// Package manager specific error types
#[derive(Debug, thiserror::Error)]
pub enum PackageManagerError {
    #[error("Package not found: {name}")]
    PackageNotFound { name: String },
    
    #[error("Registry error: {message}")]
    RegistryError { message: String },
    
    #[error("Invalid version: {version}")]
    InvalidVersion { version: String },
    
    #[error("Dependency error: {reason}")]
    DependencyError { reason: String },
    
    #[error("Circular dependency detected: {cycle:?}")]
    CircularDependency { cycle: Vec<String> },
    
    #[error("Package too large: {size} bytes (max: {max_size} bytes)")]
    PackageTooLarge { size: u64, max_size: u64 },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("General error: {0}")]
    General(String),
}

impl Default for PackageManagerConfig {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from("target/packages"),
            registry_url: "https://packages.cursed-lang.org".to_string(),
            offline_mode: false,
            verify_signatures: true,
            workspace_dir: PathBuf::from("."),
            max_cache_size: 1024 * 1024 * 1024, // 1GB
            timeout_seconds: 30,
            parallel_downloads: 4,
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
            cache_dir: config.cache_dir.clone(),
            ..Default::default()
        };
        let cache = PackageCache::new(cache_config)?;
        
        let install_config = InstallConfig {
            install_dir: config.cache_dir.join("installed"),
            temp_dir: config.cache_dir.join("temp"),
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
            workspace: None,
            lock_file_manager: None,
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

    /// Generate lock file for current workspace
    pub fn generate_lock_file(&self) -> crate::error::Result<()> {
        tracing::info!("Generating lock file");
        
        let lock_path = self.config.workspace_dir.join("CursedPackage.lock");
        let mut lock_manager = LockFileManager::new(&lock_path);
        
        // Add installed packages to lock file
        let installed = self.list_installed();
        for package in installed {
            let locked_package = LockedPackage {
                name: package.name.clone(),
                version: package.version.to_string(),
                source: "registry".to_string(),
                checksum: None,
                dependencies: Vec::new(),
            };
            lock_manager.add_package(locked_package);
        }
        
        // If we have a workspace, add workspace members
        if let Some(workspace) = &self.workspace {
            let members: Vec<String> = workspace.members().iter()
                .map(|m| m.name.clone())
                .collect();
            lock_manager.set_workspace_members(members);
        }
        
        lock_manager.save()?;
        Ok(())
    }

    /// Validate existing lock file
    pub fn validate_lock_file(&self) -> crate::error::Result<()> {
        let lock_path = self.config.workspace_dir.join("CursedPackage.lock");
        
        if !lock_path.exists() {
            return Err(crate::error::CursedError::General("No lock file found".to_string()));
        }
        
        let mut lock_manager = LockFileManager::new(&lock_path);
        lock_manager.load()?;
        
        let is_valid = lock_manager.validate()?;
        if !is_valid {
            return Err(crate::error::CursedError::General("Lock file validation failed".to_string()));
        }
        
        Ok(())
    }

    /// Get lock file status
    pub fn lock_file_status(&self) -> Option<&LockFileManager> {
        self.lock_file_manager.as_ref()
    }

    /// Initialize workspace with members
    pub fn init_workspace(&mut self, workspace_root: &std::path::Path, members: Vec<String>) -> crate::error::Result<()> {
        tracing::info!("Initializing workspace in: {:?} with members: {:?}", workspace_root, members);
        
        std::fs::create_dir_all(workspace_root)?;
        
        // Create workspace manager
        let workspace = WorkspaceManager::init_workspace(workspace_root, members)?;
        self.workspace = Some(workspace);
        
        // Initialize lock file manager
        let lock_path = workspace_root.join("CursedPackage.lock");
        self.lock_file_manager = Some(LockFileManager::new(&lock_path));
        
        Ok(())
    }

    /// Get workspace
    pub fn workspace(&self) -> Option<&WorkspaceManager> {
        self.workspace.as_ref()
    }

    /// Install all workspace dependencies
    pub async fn install_workspace(&mut self) -> crate::error::Result<Vec<InstalledPackage>> {
        tracing::info!("Installing workspace dependencies");
        
        // Get dependencies first to avoid borrow checker issues
        let dependencies = if let Some(workspace) = &self.workspace {
            let deps = workspace.list_dependencies();
            let member_names: Vec<String> = workspace.members().iter().map(|m| m.name.clone()).collect();
            Some((deps, member_names))
        } else {
            None
        };
        
        if let Some((dependencies, member_names)) = dependencies {
            let mut installed = Vec::new();
            
            // Install external dependencies for each member
            for (member_name, deps) in dependencies {
                for dep_name in deps {
                    // Skip local dependencies (they are workspace members)
                    if !member_names.contains(&dep_name) {
                        match self.install_package(&dep_name, None).await {
                            Ok(package) => installed.push(package),
                            Err(e) => tracing::warn!("Failed to install dependency {} for {}: {}", dep_name, member_name, e),
                        }
                    }
                }
            }
            
            Ok(installed)
        } else {
            Ok(Vec::new())
        }
    }

    /// Build workspace
    pub async fn build_workspace(&self) -> crate::error::Result<()> {
        tracing::info!("Building workspace");
        
        if let Some(workspace) = &self.workspace {
            // Validate workspace before building
            workspace.validate()?;
            
            // Get build order
            let build_order = workspace.get_build_order()?;
            
            // Build each member in dependency order
            for member in build_order {
                tracing::info!("Building workspace member: {}", member.name);
                // In a real implementation, this would trigger the CURSED compiler
                // For now, just verify the member directory exists
                if !member.path.exists() {
                    return Err(crate::error::CursedError::General(
                        format!("Workspace member {} does not exist at {:?}", member.name, member.path)
                    ));
                }
            }
            
            Ok(())
        } else {
            Err(crate::error::CursedError::General("No workspace initialized".to_string()))
        }
    }

    /// Clean workspace
    pub fn clean_workspace(&self) -> crate::error::Result<()> {
        tracing::info!("Cleaning workspace");
        
        if self.config.cache_dir.exists() {
            std::fs::remove_dir_all(&self.config.cache_dir)?;
        }
        
        Ok(())
    }
}

impl Default for PackageManager {
    fn default() -> Self {
        Self::new(PackageManagerConfig::default())
            .expect("Failed to create default PackageManager")
    }
}
