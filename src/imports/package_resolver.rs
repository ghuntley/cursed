//! Package Resolution System for CURSED
//! 
//! This module handles resolution of package imports, including:
//! - Finding installed packages
//! - Version resolution and compatibility checking
//! - Package namespace resolution
//! - Integration with package manager

use crate::error::{CursedError, Result};
use crate::package_manager::{PackageManager, PackageManagerConfig, Version, VersionReq};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

/// Information about a resolved package
#[derive(Debug, Clone)]
pub struct ResolvedPackage {
    pub name: String,
    pub version: Version,
    pub path: PathBuf,
    pub main_module: PathBuf,
    pub dependencies: Vec<PackageDependency>,
    pub namespace: Option<String>,
}

/// Package dependency information
#[derive(Debug, Clone)]
pub struct PackageDependency {
    pub name: String,
    pub version_req: VersionReq,
    pub resolved_version: Option<Version>,
    pub path: Option<PathBuf>,
}

/// Package resolution configuration
#[derive(Debug, Clone)]
pub struct PackageResolverConfig {
    pub package_dirs: Vec<PathBuf>,
    pub cache_dir: PathBuf,
    pub registry_cache_dir: PathBuf,
    pub allow_local_packages: bool,
    pub allow_dev_dependencies: bool,
    pub strict_version_matching: bool,
}

impl Default for PackageResolverConfig {
    fn default() -> Self {
        Self {
            package_dirs: vec![
                PathBuf::from(".cursed/packages"),
                PathBuf::from("./packages"),
                PathBuf::from("../packages"),
            ],
            cache_dir: PathBuf::from(".cursed/cache"),
            registry_cache_dir: PathBuf::from(".cursed/registry"),
            allow_local_packages: true,
            allow_dev_dependencies: false,
            strict_version_matching: false,
        }
    }
}

/// Package metadata loaded from package manifest
#[derive(Debug, Clone)]
pub struct PackageManifest {
    pub name: String,
    pub version: Version,
    pub main: Option<String>,
    pub dependencies: HashMap<String, VersionReq>,
    pub dev_dependencies: HashMap<String, VersionReq>,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
}

/// Main package resolver
#[derive(Debug)]
pub struct PackageResolver {
    config: PackageResolverConfig,
    package_manager: Option<PackageManager>,
    manifest_cache: HashMap<PathBuf, PackageManifest>,
    resolution_cache: HashMap<String, ResolvedPackage>,
}

impl PackageResolver {
    /// Create a new package resolver with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(PackageResolverConfig::default())
    }

    /// Create a new package resolver with custom configuration
    pub fn with_config(config: PackageResolverConfig) -> Result<Self> {
        let package_manager = match PackageManager::new(PackageManagerConfig::default()) {
            Ok(pm) => Some(pm),
            Err(_) => {
                eprintln!("Warning: Package manager initialization failed");
                None
            }
        };

        Ok(Self {
            config,
            package_manager,
            manifest_cache: HashMap::new(),
            resolution_cache: HashMap::new(),
        })
    }

    /// Resolve a package by name and optional version requirement
    pub fn resolve_package<'a>(&'a mut self, name: &'a str, version_req: Option<&'a str>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ResolvedPackage>> + Send + 'a>> {
        Box::pin(async move {
        let cache_key = format!("{}@{}", name, version_req.unwrap_or("*"));
        
        // Check resolution cache
        if let Some(cached) = self.resolution_cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        // Try to find locally installed package first
        if let Ok(resolved) = self.resolve_local_package(name, version_req).await {
            self.resolution_cache.insert(cache_key, resolved.clone());
            return Ok(resolved);
        }

        // Package manager requires mutable reference for installation
        // For now, skip automatic installation to resolve compilation issues

        Err(CursedError::ImportError(format!(
            "Package '{}' not found", name
        )))
        })
    }

    /// Try to resolve a package from local installation
    fn resolve_local_package<'a>(&'a mut self, name: &'a str, version_req: Option<&'a str>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ResolvedPackage>> + Send + 'a>> {
        Box::pin(async move {
        // Search in package directories
        let package_dirs = self.config.package_dirs.clone();
        for package_dir in &package_dirs {
            let package_path = package_dir.join(name);
            
            if package_path.exists() && package_path.is_dir() {
                // Try to load package manifest
                if let Ok(manifest) = self.load_package_manifest(&package_path) {
                    // Check version compatibility
                    if let Some(version_req_str) = version_req {
                        let req = VersionReq::parse(version_req_str)
                            .map_err(|e| CursedError::ImportError(format!("Invalid version requirement: {}", e)))?;
                        
                        if !req.matches(&manifest.version) {
                            continue; // Version doesn't match, try next directory
                        }
                    }

                    // Find main module
                    let main_module = self.find_main_module(&package_path, &manifest)?;

                    // Resolve dependencies
                    let dependencies = self.resolve_package_dependencies(&manifest).await?;

                    let resolved = ResolvedPackage {
                        name: manifest.name.clone(),
                        version: manifest.version.clone(),
                        path: package_path,
                        main_module,
                        dependencies,
                        namespace: Some(name.to_string()),
                    };

                    return Ok(resolved);
                }
            }
        }

        Err(CursedError::ImportError(format!(
            "Package '{}' not found in local directories", name
        )))
        })
    }

    /// Load package manifest (package.toml or cursed.toml)
    fn load_package_manifest(&mut self, package_path: &Path) -> Result<PackageManifest> {
        // Check cache first
        if let Some(cached) = self.manifest_cache.get(package_path) {
            return Ok(cached.clone());
        }

        // Try different manifest file names
        let manifest_files = vec![
            package_path.join("package.toml"),
            package_path.join("cursed.toml"),
            package_path.join("Cargo.toml"), // For compatibility
        ];

        for manifest_file in manifest_files {
            if manifest_file.exists() {
                let manifest = self.parse_manifest_file(&manifest_file)?;
                self.manifest_cache.insert(package_path.to_path_buf(), manifest.clone());
                return Ok(manifest);
            }
        }

        // No manifest found, create a minimal one
        let package_name = package_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let manifest = PackageManifest {
            name: package_name,
            version: Version::new(0, 1, 0),
            main: None,
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            description: None,
            authors: Vec::new(),
            license: None,
            repository: None,
            keywords: Vec::new(),
        };

        Ok(manifest)
    }

    /// Parse a manifest file
    fn parse_manifest_file(&self, manifest_path: &Path) -> Result<PackageManifest> {
        let content = fs::read_to_string(manifest_path)
            .map_err(|e| CursedError::ImportError(format!("Failed to read manifest: {}", e)))?;

        // Simple TOML-like parsing (would use proper TOML parser in production)
        let mut name = String::new();
        let mut version = Version::new(0, 1, 0);
        let mut main = None;
        let mut dependencies = HashMap::new();
        let mut dev_dependencies = HashMap::new();
        let mut description = None;
        let mut authors = Vec::new();
        let mut license = None;
        let mut repository = None;
        let mut keywords = Vec::new();

        let mut current_section = "";
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Section headers
            if line.starts_with('[') && line.ends_with(']') {
                current_section = &line[1..line.len()-1];
                continue;
            }

            // Key-value pairs
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');

                match current_section {
                    "" | "package" => {
                        match key {
                            "name" => name = value.to_string(),
                            "version" => {
                                version = Version::parse(value)
                                    .map_err(|e| CursedError::ImportError(format!("Invalid version: {}", e)))?;
                            }
                            "main" => main = Some(value.to_string()),
                            "description" => description = Some(value.to_string()),
                            "license" => license = Some(value.to_string()),
                            "repository" => repository = Some(value.to_string()),
                            "authors" => {
                                authors = value.split(',').map(|s| s.trim().to_string()).collect();
                            }
                            "keywords" => {
                                keywords = value.split(',').map(|s| s.trim().to_string()).collect();
                            }
                            _ => {}
                        }
                    }
                    "dependencies" => {
                        let version_req = VersionReq::parse(value)
                            .map_err(|e| CursedError::ImportError(format!("Invalid version requirement: {}", e)))?;
                        dependencies.insert(key.to_string(), version_req);
                    }
                    "dev-dependencies" | "dev_dependencies" => {
                        if self.config.allow_dev_dependencies {
                            let version_req = VersionReq::parse(value)
                                .map_err(|e| CursedError::ImportError(format!("Invalid version requirement: {}", e)))?;
                            dev_dependencies.insert(key.to_string(), version_req);
                        }
                    }
                    _ => {} // Unknown section, ignore
                }
            }
        }

        Ok(PackageManifest {
            name,
            version,
            main,
            dependencies,
            dev_dependencies,
            description,
            authors,
            license,
            repository,
            keywords,
        })
    }

    /// Find the main module file for a package
    fn find_main_module(&self, package_path: &Path, manifest: &PackageManifest) -> Result<PathBuf> {
        // Check if main is specified in manifest
        if let Some(main_file) = &manifest.main {
            let main_path = package_path.join(main_file);
            if main_path.exists() {
                return Ok(main_path);
            }
        }

        // Try common main module names
        let format_name = format!("{}.csd", manifest.name);
        let candidates = vec![
            "lib.csd",
            "main.csd",
            "mod.csd",
            "index.csd",
            &format_name,
        ];

        for candidate in &candidates {
            let candidate_path = package_path.join(candidate);
            if candidate_path.exists() {
                return Ok(candidate_path);
            }
        }

        // Try src directory
        let src_dir = package_path.join("src");
        if src_dir.exists() {
            for candidate in &candidates {
                let candidate_path = src_dir.join(candidate);
                if candidate_path.exists() {
                    return Ok(candidate_path);
                }
            }
        }

        Err(CursedError::ImportError(format!(
            "No main module found for package '{}'", manifest.name
        )))
    }

    /// Resolve package dependencies
    fn resolve_package_dependencies<'a>(&'a mut self, manifest: &'a PackageManifest) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<PackageDependency>>> + Send + 'a>> {
        Box::pin(async move {
        let mut dependencies = Vec::new();

        for (dep_name, version_req) in &manifest.dependencies {
            let resolved_version = match self.resolve_package(dep_name, Some(&version_req.to_string())).await {
                Ok(resolved_pkg) => {
                    dependencies.push(PackageDependency {
                        name: dep_name.clone(),
                        version_req: version_req.clone(),
                        resolved_version: Some(resolved_pkg.version),
                        path: Some(resolved_pkg.path),
                    });
                    continue;
                }
                Err(_) => None,
            };

            dependencies.push(PackageDependency {
                name: dep_name.clone(),
                version_req: version_req.clone(),
                resolved_version,
                path: None,
            });
        }

        Ok(dependencies)
        })
    }

    /// Clear resolution cache
    pub fn clear_cache(&mut self) {
        self.resolution_cache.clear();
        self.manifest_cache.clear();
    }

    /// Get list of installed packages
    pub fn get_installed_packages(&self) -> Result<Vec<String>> {
        let mut packages = Vec::new();

        for package_dir in &self.config.package_dirs {
            if package_dir.exists() && package_dir.is_dir() {
                for entry in fs::read_dir(package_dir)? {
                    let entry = entry?;
                    if entry.path().is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            packages.push(name.to_string());
                        }
                    }
                }
            }
        }

        packages.sort();
        packages.dedup();
        Ok(packages)
    }

    /// Check if a package is installed
    pub fn is_package_installed(&self, name: &str) -> bool {
        for package_dir in &self.config.package_dirs {
            let package_path = package_dir.join(name);
            if package_path.exists() && package_path.is_dir() {
                return true;
            }
        }
        false
    }

    /// Get package information without full resolution
    pub fn get_package_info(&mut self, name: &str) -> Result<Option<PackageManifest>> {
        let package_dirs = self.config.package_dirs.clone();
        for package_dir in &package_dirs {
            let package_path = package_dir.join(name);
            if package_path.exists() && package_path.is_dir() {
                match self.load_package_manifest(&package_path) {
                    Ok(manifest) => return Ok(Some(manifest)),
                    Err(_) => continue,
                }
            }
        }
        Ok(None)
    }

    /// Validate package dependencies
    pub async fn validate_dependencies(&mut self, manifest: &PackageManifest) -> Result<Vec<String>> {
        let mut missing_deps = Vec::new();

        for (dep_name, _version_req) in &manifest.dependencies {
            if !self.is_package_installed(dep_name) {
                missing_deps.push(dep_name.clone());
            }
        }

        Ok(missing_deps)
    }
}

impl Default for PackageResolver {
    fn default() -> Self {
        Self::new().expect("Failed to create default PackageResolver")
    }
}

/// Utility function to check if a path looks like a package directory
pub fn is_package_directory(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }

    // Check for package manifest files
    let manifest_files = vec![
        "package.toml",
        "cursed.toml",
        "Cargo.toml",
    ];

    for manifest_file in manifest_files {
        if path.join(manifest_file).exists() {
            return true;
        }
    }

    // Check for common package structure
    let common_files = vec![
        "lib.csd",
        "main.csd",
        "mod.csd",
        "src/lib.csd",
        "src/main.csd",
    ];

    for file in common_files {
        if path.join(file).exists() {
            return true;
        }
    }

    false
}
