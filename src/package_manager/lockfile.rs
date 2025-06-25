use crate::error::CursedError;
/// Lock file management for reproducible builds
/// 
/// Handles generation, parsing, and validation of CursedPackage.lock files
/// to ensure consistent dependency resolution across environments.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

use crate::package_manager::{PackageManagerError, PackageMetadata};

/// Lock file format version
pub const LOCK_FILE_VERSION: u32 = 1;

/// Lock file representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFile {
    /// Lock file format version
    
    /// Locked packages with resolved versions
    #[serde(rename = "package")]
    
    /// Lock file metadata
/// Individual locked package entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPackage {
    /// Package name
    
    /// Exact version that was resolved
    
    /// Source of the package (registry, git, path)
    
    /// Dependencies of this package with locked versions
    
    /// Checksum for integrity verification
    
    /// Optional build metadata
/// Package source information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PackageSource {
    /// Registry package
    Registry {
        /// Registry URL
    
    /// Git repository
    Git {
        /// Git repository URL
        /// Branch, tag, or commit
    
    /// Local path
    Path {
        /// Relative or absolute path
/// Lock file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileMetadata {
    /// When the lock file was generated
    
    /// Version of the package manager that generated this
    
    /// Platform information
    
    /// Optional workspace root
/// Lock file manager
#[derive(Debug)]
pub struct LockFileManager {
    /// Path to the lock file
    
    /// Current lock file data
/// Lock file specific errors
#[derive(CursedError, Debug)]
pub enum LockFileError {
    #[error("Lock file not found at {path:?}")]
    
    #[error("Lock file format version {found} is not supported (expected {expected})")]
    
    #[error("Checksum mismatch for package {package}: expected {expected}, got {actual}")]
    ChecksumMismatch {
    
    #[error("Lock file is corrupted: {reason}")]
    
    #[error("Lock file validation failed: {reason}")]
    
    #[error("Package {package} version {version} not found in lock file")]
    
    #[error("Lock file is out of date with package dependencies")]
    
    #[error("IO error: {0}")]
    
    #[error("TOML parsing error: {0}")]
    
    #[error("TOML serialization error: {0}")]
impl LockFileManager {
    /// Create a new lock file manager
    pub fn new<P: AsRef<Path>>(lock_file_path: P) -> Self {
        Self {
        }
    }
    
    /// Load existing lock file
    pub fn load(&mut self) -> crate::error::Result<()> {
        if !self.lock_file_path.exists() {
            return Err(LockFileError::NotFound {
            });
        let content = std::fs::read_to_string(&self.lock_file_path)?;
        let lock_file: LockFile = toml::from_str(&content)?;
        
        // Validate version compatibility
        if lock_file.version != LOCK_FILE_VERSION {
            return Err(LockFileError::UnsupportedVersion {
            });
        self.lock_file = Some(lock_file);
        Ok(())
    /// Generate lock file from resolved dependencies
    pub fn generate_from_dependencies(
    ) -> crate::error::Result<()> {
        let mut packages = Vec::new();
        
        for dep in dependencies {
            let locked_package = LockedPackage {
                source: PackageSource::Registry {
                    url: "registry+https://packages.cursed-lang.org/".to_string(),
            packages.push(locked_package);
        let lock_file = LockFile {
            metadata: LockFileMetadata {
        
        self.lock_file = Some(lock_file);
        Ok(())
    /// Save lock file to disk
    pub fn save(&self) -> crate::error::Result<()> {
        let lock_file = self.lock_file.as_ref().ok_or_else(|| {
            LockFileError::Corrupted {
            }
        })?;
        
        let content = toml::to_string_pretty(lock_file)?;
        
        // Ensure parent directory exists
        if let Some(parent) = self.lock_file_path.parent() {
            std::fs::create_dir_all(parent)?;
        std::fs::write(&self.lock_file_path, content)?;
        tracing::info!(path = ?self.lock_file_path, "Lock file saved successfully");
        Ok(())
    /// Validate lock file integrity
    pub fn validate(&self) -> crate::error::Result<()> {
        let lock_file = self.lock_file.as_ref().ok_or_else(|| {
            LockFileError::Corrupted {
            }
        })?;
        
        // Check version compatibility
        if lock_file.version != LOCK_FILE_VERSION {
            return Err(LockFileError::UnsupportedVersion {
            });
        // Validate package entries
        for package in &lock_file.packages {
            if package.name.is_empty() {
                return Err(LockFileError::ValidationFailed {
                });
            if package.version.is_empty() {
                return Err(LockFileError::ValidationFailed {
                });
            if package.checksum.is_empty() {
                return Err(LockFileError::ValidationFailed {
                });
            }
        }
        
        // Check for duplicate packages
        let mut seen_packages = HashMap::new();
        for package in &lock_file.packages {
            let key = (&package.name, &package.version);
            if seen_packages.contains_key(&key) {
                return Err(LockFileError::ValidationFailed {
                });
            }
            seen_packages.insert(key, package);
        Ok(())
    /// Get locked version for a package
    pub fn get_locked_version(&self, package_name: &str) -> Option<&LockedPackage> {
        self.lock_file.as_ref()?.packages.iter()
            .find(|pkg| pkg.name == package_name)
    /// Check if lock file exists
    pub fn exists(&self) -> bool {
        self.lock_file_path.exists()
    /// Update lock file with new dependencies
    pub fn update_dependencies(&mut self, new_dependencies: &[PackageMetadata]) -> crate::error::Result<()> {
        self.generate_from_dependencies(new_dependencies, None)?;
        self.save()
    /// Get all locked packages
    pub fn get_packages(&self) -> Option<&[LockedPackage]> {
        self.lock_file.as_ref().map(|lf| lf.packages.as_slice())
    /// Verify package checksum
    pub fn verify_package_checksum(&self, package: &PackageMetadata) -> crate::error::Result<()> {
        let locked_package = self.get_locked_version(&package.name)
            .ok_or_else(|| LockFileError::PackageNotLocked {
            })?;
        
        let calculated_checksum = self.calculate_checksum(package)?;
        Ok(locked_package.checksum == calculated_checksum)
    /// Calculate checksum for a package
    fn calculate_checksum(&self, package: &PackageMetadata) -> crate::error::Result<()> {
        let mut hasher = Sha256::new();
        
        // Hash package metadata in a deterministic way
        hasher.update(package.name.as_bytes());
        hasher.update(package.version.as_bytes());
        hasher.update(package.description.as_bytes());
        
        // Hash dependencies in sorted order for determinism
        let mut deps: Vec<_> = package.dependencies.iter().collect();
        deps.sort_by_key(|(k, _)| *k);
        for (name, version) in deps {
            hasher.update(name.as_bytes());
            hasher.update(version.to_string().as_bytes());
        let result = hasher.finalize();
        Ok(format!("sha256:{:x}", result))
    /// Convert lock file error to package manager error
    pub fn to_package_manager_error(err: LockFileError) -> PackageManagerError {
        match err {
            LockFileError::TomlParse(_) => PackageManagerError::InvalidMetadata {
            LockFileError::TomlSerialize(e) => PackageManagerError::InvalidMetadata {
            _ => PackageManagerError::InvalidMetadata {
        }
    }
/// Helper functions for lock file operations
impl LockFile {
    /// Create a new empty lock file
    pub fn new() -> Self {
        Self {
            metadata: LockFileMetadata {
        }
    }
    
    /// Add a package to the lock file
    pub fn add_package(&mut self, package: LockedPackage) {
        // Remove existing entry if present
        self.packages.retain(|p| !(p.name == package.name && p.version == package.version));
        self.packages.push(package);
        
        // Sort packages by name for consistency
        self.packages.sort_by(|a, b| a.name.cmp(&b.name));
    /// Remove a package from the lock file
    pub fn remove_package(&mut self, name: &str) {
        self.packages.retain(|p| p.name != name);
    /// Find packages that depend on a given package
    pub fn find_dependents(&self, package_name: &str) -> Vec<&LockedPackage> {
        self.packages.iter()
            .filter(|pkg| pkg.dependencies.iter().any(|dep| {
                // Parse dependency string (format: "name version")
                dep.split_whitespace().next().unwrap_or("") == package_name
            }))
            .collect()
    }
}

impl Default for LockFile {
    fn default() -> Self {
        Self::new()
    }
}

/// Utilities for working with dependency strings in lock files
pub fn parse_dependency_string(dep_str: &str) -> Option<(&str, &str)> {
    let mut parts = dep_str.split_whitespace();
    let name = parts.next()?;
    let version = parts.next()?;
    Some((name, version))
/// Format dependency as lock file string
pub fn format_dependency_string(name: &str, version: &str) -> String {
    format!("{} {}", name, version)
