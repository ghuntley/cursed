/// CURSED Package Manager
/// 
/// Provides comprehensive package management functionality including:
/// - Package metadata parsing and validation
/// - Registry operations (search, download, version resolution)
/// - Dependency resolution and installation
/// - Cache management and integrity verification

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod metadata;
pub mod registry;
pub mod cache;
pub mod resolver;
pub mod cli;
pub mod lockfile;
pub mod lock_file;
pub mod workspace;
pub mod downloader;
pub mod installer;
pub mod database;
pub mod scripts;

// Re-export commonly used types
pub use metadata::{PackageMetadata, VersionSpec};
pub use registry::{PackageRegistry, PackageInfo, PackageData};
pub use cache::{PackageCache, CacheStats};
pub use resolver::{DependencyResolver, ResolvedDependency};
pub use cli::{PackageManagerCli, Commands};
pub use lockfile::{LockFile, LockFileManager, LockedPackage, PackageSource};
pub use lock_file::{LockFileManager as NewLockFileManager, ValidationResult, LockFileExportFormat};
pub use workspace::{WorkspaceManager, WorkspaceConfig, WorkspaceMember};
pub use downloader::{PackageDownloader, DownloadConfig, DownloadedPackage, DownloadStats};
pub use installer::{PackageInstaller, InstallerConfig, InstallationContext, FileOperation};
pub use database::{PackageDatabase, SharedPackageDatabase, InstalledPackage, DatabaseStatistics};
pub use scripts::{ScriptExecutor, InstallScript, ScriptContext, ScriptInterpreter, ScriptResult};





/// Main package manager coordinator
#[derive(Debug)]
pub struct PackageManager {
    registry: Arc<Mutex<PackageRegistry>>,
    cache: PackageCache,
    resolver: DependencyResolver,
    downloader: PackageDownloader,
    config: PackageManagerConfig,
    lock_file_manager: Option<LockFileManager>,
    workspace_manager: Option<WorkspaceManager>,
}

/// Configuration for the package manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManagerConfig {
    pub registry_url: String,
    pub cache_dir: PathBuf,
    pub workspace_dir: PathBuf,
    pub max_cache_size: usize,
    pub timeout_seconds: u64,
    pub parallel_downloads: usize,
}

/// Package manager errors
#[derive(Error, Debug)]
pub enum PackageManagerError {
    #[error("Package not found: {package}")]
    PackageNotFound { package: String },
    
    #[error("Version conflict: {package} requires {required} but {conflicting} is installed")]
    VersionConflict {
        package: String,
        required: String,
        conflicting: String,
    },
    
    #[error("Circular dependency detected: {cycle:?}")]
    CircularDependency { cycle: Vec<String> },
    
    #[error("Dependency resolution error: {reason}")]
    DependencyError { reason: String },
    
    #[error("Invalid version: {version} - {reason}")]
    InvalidVersion { version: String, reason: String },
    
    #[error("Dependency not found: {name} with constraint {constraint}")]
    DependencyNotFound { name: String, constraint: String },
    
    #[error("Dependency version conflict: package {package} has constraints {constraints:?} but available versions are {available:?}")]
    DependencyVersionConflict { package: String, constraints: Vec<String>, available: Vec<String> },
    
    #[error("File system error at {path:?}: {error}")]
    FileSystemError { path: std::path::PathBuf, error: String },
    
    #[error("Lock timeout for package {package} after {timeout_seconds} seconds")]
    LockTimeout { package: String, timeout_seconds: u64 },
    
    #[error("Package too large: {size} bytes exceeds maximum {max_size} bytes")]
    PackageTooLarge { size: usize, max_size: usize },
    
    #[error("Cache corruption: {details}")]
    CacheCorruption { details: String },
    
    #[error("Registry error: {message}")]
    RegistryError { message: String },
    
    #[error("Invalid package metadata: {reason}")]
    InvalidMetadata { reason: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("TOML error: {0}")]
    Toml(#[from] toml::ser::Error),
    
    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),
    
    #[error("Lock file error: {0}")]
    LockFile(#[from] lockfile::LockFileError),
    
    #[error("Workspace error: {0}")]
    Workspace(#[from] workspace::WorkspaceError),
    
    #[error("Unsupported version: {version}")]
    UnsupportedVersion { version: String },
}

impl PackageManager {
    /// Create a new package manager with default configuration
    pub fn new(config: PackageManagerConfig) -> Result<(), Error> {
        let registry = Arc::new(Mutex::new(PackageRegistry::new(config.registry_url.clone())?));
        let cache = PackageCache::new(config.cache_dir.clone(), config.max_cache_size)?;
        let resolver = DependencyResolver::new();
        
        // Initialize downloader with temp directory in cache
        let download_config = DownloadConfig {
            temp_dir: config.cache_dir.join("downloads"),
            max_concurrent: config.parallel_downloads,
            timeout: Duration::from_secs(config.timeout_seconds),
            ..Default::default()
        };
        let downloader = PackageDownloader::with_config(download_config)?;
        
        // Try to discover workspace
        let workspace_manager = WorkspaceManager::discover(&config.workspace_dir)
            .map_err(|e| tracing::debug!("No workspace found: {}", e))
            .ok();
        
        // Initialize lock file manager
        let lock_file_path = if let Some(ref workspace) = workspace_manager {
            workspace.root().join("CursedPackage.lock")
        } else {
            config.workspace_dir.join("CursedPackage.lock")
        };
        let lock_file_manager = Some(LockFileManager::new(lock_file_path));
        
        Ok(Self {
            registry,
            cache,
            resolver,
            downloader,
            config,
            lock_file_manager,
            workspace_manager,
        })
    }
    
    /// Install a package and its dependencies
    pub async fn install_package(
        &mut self,
        package_name: &str,
        version: Option<&str>
    ) -> Result<(), Error> {
        tracing::info!(package = package_name, ?version, "Installing package");
        
        // Check lock file for locked version
        let should_use_locked = if let Some(ref mut lock_manager) = self.lock_file_manager {
            if lock_manager.exists() {
                lock_manager.load()?;
                if let Some(locked_pkg) = lock_manager.get_locked_version(package_name) {
                    tracing::info!(
                        package = package_name,
                        locked_version = locked_pkg.version,
                        "Using locked version"
                    );
                    // Use locked version if no specific version requested
                    let version_to_use = version.unwrap_or(&locked_pkg.version);
                    version_to_use == locked_pkg.version
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };
        
        if should_use_locked {
            return self.install_locked_dependencies().await;
        }
        
        // Search for package in registry
        let package_info = {
            let mut registry = self.registry.lock().map_err(|_| PackageManagerError::RegistryError {
                message: "Failed to lock registry".to_string(),
            })?;
            registry.search_package(package_name, version).await?
        };
        
        // Set registry on resolver before using it
        self.resolver.set_registry(self.registry.clone());
        
        // Resolve dependencies
        let resolved_deps = self.resolver.resolve_dependencies(&package_info).await?;
        
        // Install packages in dependency order
        let mut installed = Vec::new();
        for dep in resolved_deps {
            let cached_package = self.install_single_package(&dep.package).await?;
            installed.push(cached_package);
        }
        
        // Update lock file
        self.update_lock_file(&installed)?;
        
        Ok(installed)
    }
    
    /// Search for packages in the registry
    pub async fn search_packages(
        &mut self,
        query: &str,
        limit: Option<usize>
    ) -> Result<(), Error> {
        let mut registry = self.registry.lock().map_err(|_| PackageManagerError::RegistryError {
            message: "Failed to lock registry".to_string(),
        })?;
        registry.search_packages(query, limit).await
    }
    
    /// Remove a package from cache and workspace
    pub fn remove_package(&mut self, package_name: &str) -> Result<(), Error> {
        tracing::info!(package = package_name, "Removing package");
        self.cache.remove_package(package_name)
    }
    
    /// List installed packages
    pub fn list_installed(&self) -> Result<(), Error> {
        self.cache.list_packages()
    }
    
    /// Clean package cache
    pub fn clean_cache(&mut self) -> Result<(), Error> {
        self.cache.clean()
    }
    
    /// Update package registry index
    pub async fn update_registry(&mut self) -> Result<(), Error> {
        let mut registry = self.registry.lock().map_err(|_| PackageManagerError::RegistryError {
            message: "Failed to lock registry".to_string(),
        })?;
        registry.update_index().await
    }
    
    /// Install a single package without dependency resolution
    async fn install_single_package(
        &mut self,
        package: &PackageMetadata
    ) -> Result<(), Error> {
        // Check if already cached
        if let Some(cached) = self.cache.get_package(&package.name, &package.version)? {
            tracing::debug!(package = package.name, version = package.version, "Package found in cache");
            return Ok(cached);
        }
        
        // Download package using the downloader with progress tracking
        let downloaded_package = {
            let mut registry = self.registry.lock().map_err(|_| PackageManagerError::RegistryError {
                message: "Failed to lock registry".to_string(),
            })?;
            
            // Create progress callback for console output
            let progress_callback = Box::new(|progress: &crate::package_manager::downloader::DownloadProgress| {
                if progress.downloaded_bytes % (1024 * 1024) == 0 || progress.downloaded_bytes == progress.total_bytes {
                    tracing::info!(
                        downloaded = progress.downloaded_bytes,
                        total = progress.total_bytes,
                        rate = format!("{:.1} KB/s", progress.transfer_rate / 1024.0),
                        "Download progress"
                    );
                }
            });
            
            self.downloader.download_package(
                &mut *registry,
                &package.name,
                &package.version,
                None, // Don't extract yet, just download
                Some(progress_callback),
            ).await?
        };
        
        // Convert downloaded package to PackageData for cache storage
        let package_data = PackageData {
            content: std::fs::read(&downloaded_package.archive_path)
                .map_err(|e| PackageManagerError::FileSystemError {
                    path: downloaded_package.archive_path.clone(),
                    error: format!("Failed to read downloaded package: {}", e),
                })?,
            checksum: downloaded_package.checksum,
            size: downloaded_package.size,
            verified: true,
        };
        
        // Store in cache
        self.cache.store_package(package, &package_data)?;
        
        tracing::info!(
            package = package.name, 
            version = package.version,
            size = package_data.size,
            duration_ms = downloaded_package.download_time.as_millis(),
            "Package installed successfully"
        );
        Ok(package.clone())
    }
    
    /// Install dependencies from lock file
    async fn install_locked_dependencies(
        &mut self,
    ) -> Result<(), Error> {
        // Extract packages first to avoid borrowing conflicts
        let packages_to_install = if let Some(ref lock_manager) = self.lock_file_manager {
            let packages = lock_manager.get_packages()
                .ok_or_else(|| PackageManagerError::InvalidMetadata {
                    reason: "No packages in lock file".to_string(),
                })?;
            
            // Convert to PackageMetadata to avoid holding references
            packages.iter().map(|locked_pkg| PackageMetadata {
                name: locked_pkg.name.clone(),
                version: locked_pkg.version.clone(),
                description: "Locked dependency".to_string(),
                authors: Vec::new(),
                dependencies: HashMap::new(),
                dev_dependencies: HashMap::new(),
                repository: None,
                license: None,
                keywords: Vec::new(),
                categories: Vec::new(),
            }).collect::<Vec<_>>()
        } else {
            return Err(PackageManagerError::InvalidMetadata {
                reason: "No lock file manager available".to_string(),
            });
        };
        
        let mut installed = Vec::new();
        
        for metadata in packages_to_install {
            let cached_package = self.install_single_package(&metadata).await?;
            installed.push(cached_package);
        }
        
        Ok(installed)
    }
    
    /// Update lock file with installed packages
    fn update_lock_file(&mut self, packages: &[PackageMetadata]) -> Result<(), Error> {
        if let Some(ref mut lock_manager) = self.lock_file_manager {
            lock_manager.update_dependencies(packages)?;
            tracing::info!("Lock file updated with {} packages", packages.len());
        }
        Ok(())
    }
    
    /// Generate lock file
    pub fn generate_lock_file(&mut self) -> Result<(), Error> {
        // Get installed packages first
        let installed = self.list_installed()?;
        let workspace_root = self.workspace_manager.as_ref()
            .map(|w| w.root().to_string_lossy().to_string());
        
        // Then update lock file
        if let Some(ref mut lock_manager) = self.lock_file_manager {
            lock_manager.generate_from_dependencies(&installed, workspace_root)?;
            lock_manager.save()?;
            tracing::info!("Lock file generated successfully");
        }
        Ok(())
    }
    
    /// Workspace operations
    pub fn workspace(&self) -> Option<&WorkspaceManager> {
        self.workspace_manager.as_ref()
    }
    
    pub fn workspace_mut(&mut self) -> Option<&mut WorkspaceManager> {
        self.workspace_manager.as_mut()
    }
    
    /// Initialize a new workspace
    pub fn init_workspace<P: AsRef<Path>>(
        &mut self,
        root: P,
        members: Vec<String>,
    ) -> Result<(), Error> {
        let workspace = WorkspaceManager::init_workspace(root, members)?;
        self.workspace_manager = Some(workspace);
        Ok(())
    }
    
    /// Install all workspace dependencies
    pub async fn install_workspace(&mut self) -> Result<(), Error> {
        // Extract dependencies to install first to avoid borrowing conflicts
        let dependencies_to_install = if let Some(ref mut workspace) = self.workspace_manager {
            workspace.generate_lock_file()?;
            
            let mut deps = Vec::new();
            for member in workspace.members() {
                tracing::info!(member = member.name, "Installing workspace member dependencies");
                
                for (dep_name, dep_version) in &member.metadata.dependencies {
                    // Skip local workspace dependencies
                    if !member.local_dependencies.contains(dep_name) {
                        let version_str = dep_version.to_string();
                        deps.push((dep_name.clone(), version_str));
                    }
                }
            }
            deps
        } else {
            Vec::new()
        };
        
        // Now install the dependencies
        for (dep_name, version_str) in dependencies_to_install {
            self.install_package(&dep_name, Some(&version_str)).await?;
        }
        
        Ok(())
    }
    
    /// Build workspace in dependency order
    pub async fn build_workspace(&mut self) -> Result<(), Error> {
        if let Some(workspace) = self.workspace_manager.as_ref() {
            let build_order = workspace.get_build_order()
                .map_err(|e| PackageManagerError::Workspace(e))?;
            
            for member in build_order {
                tracing::info!(member = member.name, path = ?member.path, "Building workspace member");
                
                // Here you would integrate with the actual build system
                // For now, we just log the build order
                println!("Building package: {} at {:?}", member.name, member.path);
            }
        }
        Ok(())
    }
    
    /// Clean workspace
    pub fn clean_workspace(&mut self) -> Result<(), Error> {
        if let Some(workspace) = self.workspace_manager.as_ref() {
            for member in workspace.members() {
                tracing::info!(member = member.name, "Cleaning workspace member");
                
                // Clean member-specific cache/build artifacts
                let member_cache = member.path.join("target");
                if member_cache.exists() {
                    std::fs::remove_dir_all(&member_cache)
                        .map_err(|e| PackageManagerError::FileSystemError {
                            path: member_cache,
                            error: e.to_string(),
                        })?;
                }
            }
        }
        Ok(())
    }
    
    /// Validate lock file integrity
    pub fn validate_lock_file(&mut self) -> Result<(), Error> {
        if let Some(ref mut lock_manager) = self.lock_file_manager {
            if lock_manager.exists() {
                lock_manager.load()?;
                lock_manager.validate()?;
                tracing::info!("Lock file validation passed");
            }
        }
        Ok(())
    }
    
    /// Get lock file status
    pub fn lock_file_status(&self) -> Option<&LockFileManager> {
        self.lock_file_manager.as_ref()
    }
    
    /// Get cache statistics  
    pub fn get_cache_stats(&self) -> Result<(), Error> {
        self.cache.stats()
    }
    
    /// Get configuration
    pub fn get_config(&self) -> &PackageManagerConfig {
        &self.config
    }
    
    /// Install locked dependencies from lock file
    pub async fn install_locked_dependencies(&mut self) -> Result<(), Error> {
        // Stub implementation
        tracing::info!("Installing locked dependencies");
        Ok(())
    }
}

impl Default for PackageManagerConfig {
    fn default() -> Self {
        Self {
            registry_url: "https://packages.cursed-lang.org".to_string(),
            cache_dir: dirs::cache_dir().unwrap_or_default().join("cursed"),
            workspace_dir: std::env::current_dir().unwrap_or_default(),
            max_cache_size: 1024 * 1024 * 1024, // 1GB
            timeout_seconds: 30,
            parallel_downloads: 4,
        }
    }
}

/// Initialize a new CURSED package in the current directory
pub fn init_package(
    name: &str,
    version: Option<&str>,
    description: Option<&str>
) -> Result<(), Error> {
    let package_file = PathBuf::from("CursedPackage.toml");
    
    if package_file.exists() {
        return Err(PackageManagerError::InvalidMetadata {
            reason: "CursedPackage.toml already exists".to_string(),
        });
    }
    
    let metadata = PackageMetadata {
        name: name.to_string(),
        version: version.unwrap_or("0.1.0").to_string(),
        description: description.unwrap_or("A CURSED package").to_string(),
        authors: Vec::from(["Your Name <your.email@example.com>".to_string()]),
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
        repository: None,
        license: None,
        keywords: Vec::from([]),
        categories: Vec::from([]),
    };
    
    let content = toml::to_string(&metadata)?;
    std::fs::write(package_file, content)?;
    
    // Create basic directory structure
    std::fs::create_dir_all("src")?;
    std::fs::write("src/main.csd", "slay main() {\n    capicola(\"Hello, CURSED World!\");\n}\n")?;
    
    tracing::info!(name, "Package initialized successfully");
    Ok(())
}
