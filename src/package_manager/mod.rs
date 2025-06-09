/// CURSED Package Manager
/// 
/// Provides comprehensive package management functionality including:
/// - Package metadata parsing and validation
/// - Registry operations (search, download, version resolution)
/// - Dependency resolution and installation
/// - Cache management and integrity verification

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod metadata;
pub mod registry;
pub mod cache;
pub mod resolver;
pub mod cli;

// Re-export commonly used types
pub use metadata::{PackageMetadata, VersionSpec};
pub use registry::{PackageRegistry, PackageInfo, PackageData};
pub use cache::{PackageCache, CacheStats};
pub use resolver::{DependencyResolver, ResolvedDependency};
pub use cli::{PackageManagerCli, Commands};





/// Main package manager coordinator
#[derive(Debug)]
pub struct PackageManager {
    registry: PackageRegistry,
    cache: PackageCache,
    resolver: DependencyResolver,
    config: PackageManagerConfig,
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
    
    #[error("Circular dependency detected: {cycle}")]
    CircularDependency { cycle: String },
    
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
}

impl PackageManager {
    /// Create a new package manager with default configuration
    pub fn new(config: PackageManagerConfig) -> Result<Self, PackageManagerError> {
        let registry = PackageRegistry::new(config.registry_url.clone())?;
        let cache = PackageCache::new(config.cache_dir.clone(), config.max_cache_size)?;
        let resolver = DependencyResolver::new();
        
        Ok(Self {
            registry,
            cache,
            resolver,
            config,
        })
    }
    
    /// Install a package and its dependencies
    pub async fn install_package(
        &mut self,
        package_name: &str,
        version: Option<&str>
    ) -> Result<Vec<PackageMetadata>, PackageManagerError> {
        tracing::info!(package = package_name, ?version, "Installing package");
        
        // Search for package in registry
        let package_info = self.registry.search_package(package_name, version).await?;
        
        // Resolve dependencies
        let resolved_deps = self.resolver.resolve_dependencies(&package_info).await?;
        
        // Install packages in dependency order
        let mut installed = Vec::new();
        for dep in resolved_deps {
            let cached_package = self.install_single_package(&dep).await?;
            installed.push(cached_package);
        }
        
        Ok(installed)
    }
    
    /// Search for packages in the registry
    pub async fn search_packages(
        &self,
        query: &str,
        limit: Option<usize>
    ) -> Result<Vec<PackageMetadata>, PackageManagerError> {
        self.registry.search_packages(query, limit).await
    }
    
    /// Remove a package from cache and workspace
    pub fn remove_package(&mut self, package_name: &str) -> Result<(), PackageManagerError> {
        tracing::info!(package = package_name, "Removing package");
        self.cache.remove_package(package_name)
    }
    
    /// List installed packages
    pub fn list_installed(&self) -> Result<Vec<PackageMetadata>, PackageManagerError> {
        self.cache.list_packages()
    }
    
    /// Clean package cache
    pub fn clean_cache(&mut self) -> Result<(), PackageManagerError> {
        self.cache.clean()
    }
    
    /// Update package registry index
    pub async fn update_registry(&mut self) -> Result<(), PackageManagerError> {
        self.registry.update_index().await
    }
    
    /// Install a single package without dependency resolution
    async fn install_single_package(
        &mut self,
        package: &PackageMetadata
    ) -> Result<PackageMetadata, PackageManagerError> {
        // Check if already cached
        if let Some(cached) = self.cache.get_package(&package.name, &package.version)? {
            tracing::debug!(package = package.name, version = package.version, "Package found in cache");
            return Ok(cached);
        }
        
        // Download from registry
        let package_data = self.registry.download_package(&package.name, &package.version).await?;
        
        // Store in cache
        self.cache.store_package(package, &package_data)?;
        
        tracing::info!(package = package.name, version = package.version, "Package installed successfully");
        Ok(package.clone())
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
) -> Result<(), PackageManagerError> {
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
        authors: vec!["Your Name <your.email@example.com>".to_string()],
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
        repository: None,
        license: None,
        keywords: vec![],
        categories: vec![],
    };
    
    let content = toml::to_string(&metadata)?;
    std::fs::write(package_file, content)?;
    
    // Create basic directory structure
    std::fs::create_dir_all("src")?;
    std::fs::write("src/main.csd", "slay main() {\n    capicola(\"Hello, CURSED World!\");\n}\n")?;
    
    tracing::info!(name, "Package initialized successfully");
    Ok(())
}
