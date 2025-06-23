/// Lock file management for reproducible builds
/// 
/// Handles generation, parsing, and validation of CursedPackage.lock files
/// to ensure consistent dependency resolution across environments.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use thiserror::Error;

use crate::package_manager::{PackageManagerError, PackageMetadata};

/// Lock file format version
pub const LOCK_FILE_VERSION: u32 = 1;

/// Lock file representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFile {
    /// Lock file format version
    pub version: u32,
    
    /// Locked packages with resolved versions
    #[serde(rename = "package")]
    pub packages: Vec<LockedPackage>,
    
    /// Lock file metadata
    pub metadata: LockFileMetadata,
}

/// Individual locked package entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPackage {
    /// Package name
    pub name: String,
    
    /// Exact version that was resolved
    pub version: String,
    
    /// Source of the package (registry, git, path)
    pub source: PackageSource,
    
    /// Dependencies of this package with locked versions
    pub dependencies: Vec<String>,
    
    /// Checksum for integrity verification
    pub checksum: String,
    
    /// Optional build metadata
    pub build_metadata: Option<HashMap<String, String>>,
}

/// Package source information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PackageSource {
    /// Registry package
    Registry {
        /// Registry URL
        url: String,
    },
    
    /// Git repository
    Git {
        /// Git repository URL
        url: String,
        /// Branch, tag, or commit
        reference: String,
    },
    
    /// Local path
    Path {
        /// Relative or absolute path
        path: String,
    },
}

/// Lock file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileMetadata {
    /// When the lock file was generated
    pub generated_at: String,
    
    /// Version of the package manager that generated this
    pub cursed_version: String,
    
    /// Platform information
    pub platform: String,
    
    /// Optional workspace root
    pub workspace_root: Option<String>,
}

/// Lock file manager
#[derive(Debug)]
pub struct LockFileManager {
    /// Path to the lock file
    pub lock_file_path: PathBuf,
    
    /// Current lock file data
    pub lock_file: Option<LockFile>,
}

/// Lock file specific errors
#[derive(Error, Debug)]
pub enum LockFileError {
    #[error("Lock file not found at {path:?}")]
    NotFound { path: PathBuf },
    
    #[error("Lock file format version {found} is not supported (expected {expected})")]
    UnsupportedVersion { found: u32, expected: u32 },
    
    #[error("Checksum mismatch for package {package}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        package: String,
        expected: String,
        actual: String,
    },
    
    #[error("Lock file is corrupted: {reason}")]
    Corrupted { reason: String },
    
    #[error("Lock file validation failed: {reason}")]
    ValidationFailed { reason: String },
    
    #[error("Package {package} version {version} not found in lock file")]
    PackageNotLocked { package: String, version: String },
    
    #[error("Lock file is out of date with package dependencies")]
    OutOfDate,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),
    
    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
}

impl LockFileManager {
    /// Create a new lock file manager
    pub fn new<P: AsRef<Path>>(lock_file_path: P) -> Self {
        Self {
            lock_file_path: lock_file_path.as_ref().to_path_buf(),
            lock_file: None,
        }
    }
    
    /// Load existing lock file
    pub fn load(&mut self) -> Result<(), Error> {
        if !self.lock_file_path.exists() {
            return Err(LockFileError::NotFound {
                path: self.lock_file_path.clone(),
            });
        }
        
        let content = std::fs::read_to_string(&self.lock_file_path)?;
        let lock_file: LockFile = toml::from_str(&content)?;
        
        // Validate version compatibility
        if lock_file.version != LOCK_FILE_VERSION {
            return Err(LockFileError::UnsupportedVersion {
                found: lock_file.version,
                expected: LOCK_FILE_VERSION,
            });
        }
        
        self.lock_file = Some(lock_file);
        Ok(())
    }
    
    /// Generate lock file from resolved dependencies
    pub fn generate_from_dependencies(
        &mut self,
        dependencies: &[PackageMetadata],
        workspace_root: Option<String>,
    ) -> Result<(), Error> {
        let mut packages = Vec::new();
        
        for dep in dependencies {
            let locked_package = LockedPackage {
                name: dep.name.clone(),
                version: dep.version.clone(),
                source: PackageSource::Registry {
                    url: "registry+https://packages.cursed-lang.org/".to_string(),
                },
                dependencies: dep.dependencies.keys().cloned().collect(),
                checksum: self.calculate_checksum(dep)?,
                build_metadata: None,
            };
            packages.push(locked_package);
        }
        
        let lock_file = LockFile {
            version: LOCK_FILE_VERSION,
            packages,
            metadata: LockFileMetadata {
                generated_at: chrono::Utc::now().to_rfc3339(),
                cursed_version: env!("CARGO_PKG_VERSION").to_string(),
                platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
                workspace_root,
            },
        };
        
        self.lock_file = Some(lock_file);
        Ok(())
    }
    
    /// Save lock file to disk
    pub fn save(&self) -> Result<(), Error> {
        let lock_file = self.lock_file.as_ref().ok_or_else(|| {
            LockFileError::Corrupted {
                reason: "No lock file data to save".to_string(),
            }
        })?;
        
        let content = toml::to_string_pretty(lock_file)?;
        
        // Ensure parent directory exists
        if let Some(parent) = self.lock_file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(&self.lock_file_path, content)?;
        tracing::info!(path = ?self.lock_file_path, "Lock file saved successfully");
        Ok(())
    }
    
    /// Validate lock file integrity
    pub fn validate(&self) -> Result<(), Error> {
        let lock_file = self.lock_file.as_ref().ok_or_else(|| {
            LockFileError::Corrupted {
                reason: "No lock file loaded".to_string(),
            }
        })?;
        
        // Check version compatibility
        if lock_file.version != LOCK_FILE_VERSION {
            return Err(LockFileError::UnsupportedVersion {
                found: lock_file.version,
                expected: LOCK_FILE_VERSION,
            });
        }
        
        // Validate package entries
        for package in &lock_file.packages {
            if package.name.is_empty() {
                return Err(LockFileError::ValidationFailed {
                    reason: "Package name cannot be empty".to_string(),
                });
            }
            
            if package.version.is_empty() {
                return Err(LockFileError::ValidationFailed {
                    reason: format!("Package {} has empty version", package.name),
                });
            }
            
            if package.checksum.is_empty() {
                return Err(LockFileError::ValidationFailed {
                    reason: format!("Package {} has empty checksum", package.name),
                });
            }
        }
        
        // Check for duplicate packages
        let mut seen_packages = HashMap::new();
        for package in &lock_file.packages {
            let key = (&package.name, &package.version);
            if seen_packages.contains_key(&key) {
                return Err(LockFileError::ValidationFailed {
                    reason: format!("Duplicate package entry: {} {}", package.name, package.version),
                });
            }
            seen_packages.insert(key, package);
        }
        
        Ok(())
    }
    
    /// Get locked version for a package
    pub fn get_locked_version(&self, package_name: &str) -> Option<&LockedPackage> {
        self.lock_file.as_ref()?.packages.iter()
            .find(|pkg| pkg.name == package_name)
    }
    
    /// Check if lock file exists
    pub fn exists(&self) -> bool {
        self.lock_file_path.exists()
    }
    
    /// Update lock file with new dependencies
    pub fn update_dependencies(&mut self, new_dependencies: &[PackageMetadata]) -> Result<(), Error> {
        self.generate_from_dependencies(new_dependencies, None)?;
        self.save()
    }
    
    /// Get all locked packages
    pub fn get_packages(&self) -> Option<&[LockedPackage]> {
        self.lock_file.as_ref().map(|lf| lf.packages.as_slice())
    }
    
    /// Verify package checksum
    pub fn verify_package_checksum(&self, package: &PackageMetadata) -> Result<(), Error> {
        let locked_package = self.get_locked_version(&package.name)
            .ok_or_else(|| LockFileError::PackageNotLocked {
                package: package.name.clone(),
                version: package.version.clone(),
            })?;
        
        let calculated_checksum = self.calculate_checksum(package)?;
        Ok(locked_package.checksum == calculated_checksum)
    }
    
    /// Calculate checksum for a package
    fn calculate_checksum(&self, package: &PackageMetadata) -> Result<(), Error> {
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
        }
        
        let result = hasher.finalize();
        Ok(format!("sha256:{:x}", result))
    }
    
    /// Convert lock file error to package manager error
    pub fn to_package_manager_error(err: LockFileError) -> PackageManagerError {
        match err {
            LockFileError::Io(io_err) => PackageManagerError::Io(io_err),
            LockFileError::TomlParse(_) => PackageManagerError::InvalidMetadata {
                reason: "Invalid lock file format".to_string(),
            },
            LockFileError::TomlSerialize(e) => PackageManagerError::InvalidMetadata {
                reason: format!("Lock file serialization failed: {}", e),
            },
            _ => PackageManagerError::InvalidMetadata {
                reason: err.to_string(),
            },
        }
    }
}

/// Helper functions for lock file operations
impl LockFile {
    /// Create a new empty lock file
    pub fn new() -> Self {
        Self {
            version: LOCK_FILE_VERSION,
            packages: Vec::new(),
            metadata: LockFileMetadata {
                generated_at: chrono::Utc::now().to_rfc3339(),
                cursed_version: env!("CARGO_PKG_VERSION").to_string(),
                platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
                workspace_root: None,
            },
        }
    }
    
    /// Add a package to the lock file
    pub fn add_package(&mut self, package: LockedPackage) {
        // Remove existing entry if present
        self.packages.retain(|p| !(p.name == package.name && p.version == package.version));
        self.packages.push(package);
        
        // Sort packages by name for consistency
        self.packages.sort_by(|a, b| a.name.cmp(&b.name));
    }
    
    /// Remove a package from the lock file
    pub fn remove_package(&mut self, name: &str) {
        self.packages.retain(|p| p.name != name);
    }
    
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
}

/// Format dependency as lock file string
pub fn format_dependency_string(name: &str, version: &str) -> String {
    format!("{} {}", name, version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_lock_file_creation() {
        let lock_file = LockFile::new();
        assert_eq!(lock_file.version, LOCK_FILE_VERSION);
        assert!(lock_file.packages.is_empty());
    }
    
    #[test]
    fn test_dependency_string_parsing() {
        assert_eq!(parse_dependency_string("serde 1.0.0"), Some(("serde", "1.0.0")));
        assert_eq!(parse_dependency_string("invalid"), None);
        assert_eq!(parse_dependency_string(""), None);
    }
    
    #[test]
    fn test_dependency_string_formatting() {
        assert_eq!(format_dependency_string("serde", "1.0.0"), "serde 1.0.0");
    }
    
    #[test]
    fn test_lock_file_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let lock_path = temp_dir.path().join("CursedPackage.lock");
        
        let manager = LockFileManager::new(&lock_path);
        assert_eq!(manager.lock_file_path, lock_path);
        assert!(!manager.exists());
    }
    
    #[test]
    fn test_package_checksum_calculation() {
        let temp_dir = TempDir::new().unwrap();
        let lock_path = temp_dir.path().join("CursedPackage.lock");
        let manager = LockFileManager::new(&lock_path);
        
        let package = PackageMetadata {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package".to_string(),
            authors: vec!["Test Author".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: Vec::new(),
            categories: Vec::new(),
        };
        
        let checksum1 = manager.calculate_checksum(&package).unwrap();
        let checksum2 = manager.calculate_checksum(&package).unwrap();
        
        // Checksums should be deterministic
        assert_eq!(checksum1, checksum2);
        assert!(checksum1.starts_with("sha256:"));
    }
    
    #[test]
    fn test_lock_file_validation() {
        let mut lock_file = LockFile::new();
        
        // Valid lock file should pass validation
        let temp_dir = TempDir::new().unwrap();
        let lock_path = temp_dir.path().join("CursedPackage.lock");
        let mut manager = LockFileManager::new(&lock_path);
        manager.lock_file = Some(lock_file.clone());
        
        assert!(manager.validate().is_ok());
        
        // Add invalid package (empty name)
        lock_file.packages.push(LockedPackage {
            name: "".to_string(),
            version: "1.0.0".to_string(),
            source: PackageSource::Registry { url: "test".to_string() },
            dependencies: Vec::new(),
            checksum: "sha256:test".to_string(),
            build_metadata: None,
        });
        
        manager.lock_file = Some(lock_file);
        assert!(manager.validate().is_err());
    }
}
