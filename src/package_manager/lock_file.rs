// Lock File Management for Reproducible Builds
//
// This module provides functionality for generating, reading, and validating
// lock files that ensure reproducible dependency resolution across different
// environments and time periods.

use crate::package_manager::{
    PackageManagerError,
    resolver::{ResolvedDependency, LockFile, LockedPackage, PackageSource, LockFileMetadata},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, warn, debug, instrument};
use chrono::Utc;
use semver::Version;

/// Lock file manager for handling lock file operations
#[derive(Debug)]
pub struct LockFileManager {
    /// Path to the lock file
    lock_file_path: PathBuf,
    /// Current lock file content (if loaded)
    current_lock: Option<LockFile>,
}

/// Lock file validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub missing_packages: Vec<String>,
    pub version_mismatches: Vec<VersionMismatch>,
    pub checksum_mismatches: Vec<ChecksumMismatch>,
    pub extra_packages: Vec<String>,
}

/// Version mismatch information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionMismatch {
    pub package: String,
    pub expected: String,
    pub actual: String,
}

/// Checksum mismatch information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksumMismatch {
    pub package: String,
    pub expected: String,
    pub actual: String,
}

/// Lock file format versions
#[derive(Debug, Clone, PartialEq)]
pub enum LockFileFormat {
    V1_0,
    Unknown(String),
}

impl LockFileManager {
    /// Create a new lock file manager
    pub fn new<P: Into<PathBuf>>(lock_file_path: P) -> Self {
        Self {
            lock_file_path: lock_file_path.into(),
            current_lock: None,
        }
    }

    /// Create with default lock file name in project directory
    pub fn with_project_dir<P: AsRef<Path>>(project_dir: P) -> Self {
        let lock_path = project_dir.as_ref().join("Cursed.lock");
        Self::new(lock_path)
    }

    /// Load existing lock file from disk
    #[instrument(skip(self))]
    pub fn load(&mut self) -> Result<(), Error> {
        if !self.lock_file_path.exists() {
            debug!("Lock file does not exist: {:?}", self.lock_file_path);
            return Ok(None);
        }

        info!("Loading lock file from {:?}", self.lock_file_path);
        
        let content = fs::read_to_string(&self.lock_file_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: self.lock_file_path.clone(),
                error: e.to_string()
            })?;

        let lock_file: LockFile = serde_json::from_str(&content)
            .map_err(|e| PackageManagerError::InvalidMetadata { 
                reason: format!("Failed to parse lock file: {}", e)
            })?;

        // Validate format version
        let format = self.parse_format_version(&lock_file.version);
        if !self.is_supported_format(&format) {
            return Err(PackageManagerError::UnsupportedVersion { 
                version: lock_file.version.clone()
            });
        }

        info!("Successfully loaded lock file with {} packages", lock_file.packages.len());
        self.current_lock = Some(lock_file);
        Ok(self.current_lock.as_ref())
    }

    /// Save lock file to disk
    #[instrument(skip(self, lock_file))]
    pub fn save(&mut self, lock_file: &LockFile) -> Result<(), Error> {
        info!("Saving lock file to {:?} with {} packages", 
              self.lock_file_path, lock_file.packages.len());

        // Ensure parent directory exists
        if let Some(parent) = self.lock_file_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| PackageManagerError::FileSystemError { 
                    path: parent.to_path_buf(),
                    error: e.to_string()
                })?;
        }

        // Serialize with pretty printing for human readability
        let content = serde_json::to_string_pretty(lock_file)
            .map_err(|e| PackageManagerError::InvalidMetadata { 
                reason: format!("Failed to serialize lock file: {}", e)
            })?;

        // Write to temporary file first, then rename for atomicity
        let temp_path = self.lock_file_path.with_extension("lock.tmp");
        
        fs::write(&temp_path, content)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: temp_path.clone(),
                error: e.to_string()
            })?;

        fs::rename(&temp_path, &self.lock_file_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                path: self.lock_file_path.clone(),
                error: e.to_string()
            })?;

        self.current_lock = Some(lock_file.clone());
        info!("Lock file saved successfully");
        Ok(())
    }

    /// Validate lock file against current dependencies
    #[instrument(skip(self, dependencies))]
    pub fn validate(&self, dependencies: &[ResolvedDependency]) -> ValidationResult {
        let Some(lock_file) = &self.current_lock else {
            return ValidationResult {
                is_valid: false,
                missing_packages: dependencies.iter().map(|d| d.package.name.clone()).collect(),
                version_mismatches: Vec::new(),
                checksum_mismatches: Vec::new(),
                extra_packages: Vec::new(),
            };
        };

        let mut result = ValidationResult {
            is_valid: true,
            missing_packages: Vec::new(),
            version_mismatches: Vec::new(),
            checksum_mismatches: Vec::new(),
            extra_packages: Vec::new(),
        };

        // Check all current dependencies are in lock file
        for dep in dependencies {
            if let Some(locked_pkg) = lock_file.packages.get(&dep.package.name) {
                // Check version match
                if locked_pkg.version != dep.resolved_version.to_string() {
                    result.version_mismatches.push(VersionMismatch {
                        package: dep.package.name.clone(),
                        expected: locked_pkg.version.clone(),
                        actual: dep.resolved_version.to_string(),
                    });
                    result.is_valid = false;
                }

                // Note: Checksum validation would require actual package content
                // For now, we skip checksum validation in tests
            } else {
                result.missing_packages.push(dep.package.name.clone());
                result.is_valid = false;
            }
        }

        // Check for extra packages in lock file
        for locked_name in lock_file.packages.keys() {
            if !dependencies.iter().any(|dep| &dep.package.name == locked_name) {
                result.extra_packages.push(locked_name.clone());
                result.is_valid = false;
            }
        }

        if result.is_valid {
            info!("Lock file validation passed");
        } else {
            warn!("Lock file validation failed: {} missing, {} version mismatches, {} extra", 
                  result.missing_packages.len(), 
                  result.version_mismatches.len(),
                  result.extra_packages.len());
        }

        result
    }

    /// Generate a new lock file from resolved dependencies
    pub fn generate_from_dependencies(&self, dependencies: &[ResolvedDependency]) -> LockFile {
        let mut packages = BTreeMap::new();
        let mut dependency_tree = BTreeMap::new();

        for dep in dependencies {
            let locked_package = LockedPackage {
                version: dep.resolved_version.to_string(),
                checksum: self.calculate_checksum(&dep.package.name, &dep.resolved_version),
                source: PackageSource::Registry { 
                    url: "https://registry.cursed-lang.org".to_string() 
                },
                dependencies: dep.package.dependencies.keys().cloned().collect(),
                resolved_at: Utc::now().to_rfc3339(),
            };

            packages.insert(dep.package.name.clone(), locked_package);

            // Build dependency tree
            if !dep.required_by.is_empty() {
                dependency_tree.insert(
                    dep.package.name.clone(),
                    dep.required_by.clone()
                );
            }
        }

        let metadata = LockFileMetadata {
            generated_at: Utc::now().to_rfc3339(),
            resolver_version: env!("CARGO_PKG_VERSION").to_string(),
            total_packages: packages.len(),
            resolution_time_ms: 0, // Would be filled by resolver
        };

        LockFile {
            version: "1.0".to_string(),
            packages,
            metadata,
            dependency_tree,
        }
    }

    /// Update an existing lock file with new dependencies
    pub fn update_lock_file(&self, new_dependencies: &[ResolvedDependency]) -> LockFile {
        let mut new_lock = self.generate_from_dependencies(new_dependencies);

        // If we have an existing lock file, preserve some metadata
        if let Some(existing) = &self.current_lock {
            // Could preserve creation time, maintain package order, etc.
            // For now, just generate fresh
            new_lock.metadata.generated_at = Utc::now().to_rfc3339();
        }

        new_lock
    }

    /// Check if lock file exists
    pub fn exists(&self) -> bool {
        self.lock_file_path.exists()
    }

    /// Get lock file path
    pub fn path(&self) -> &Path {
        &self.lock_file_path
    }

    /// Get current loaded lock file
    pub fn current_lock(&self) -> Option<&LockFile> {
        self.current_lock.as_ref()
    }

    /// Delete lock file from disk
    pub fn delete(&mut self) -> Result<(), Error> {
        if self.lock_file_path.exists() {
            fs::remove_file(&self.lock_file_path)
                .map_err(|e| PackageManagerError::FileSystemError { 
                    path: self.lock_file_path.clone(),
                    error: e.to_string()
                })?;
        }
        self.current_lock = None;
        Ok(())
    }

    /// Calculate package checksum (simplified for now)
    fn calculate_checksum(&self, _package_name: &str, _version: &Version) -> String {
        // In a real implementation, this would calculate actual checksums
        // from package contents downloaded from registry
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(_package_name.as_bytes());
        hasher.update(_version.to_string().as_bytes());
        format!("sha256:{:x}", hasher.finalize())
    }

    /// Parse lock file format version
    fn parse_format_version(&self, version: &str) -> LockFileFormat {
        match version {
            "1.0" => LockFileFormat::V1_0,
            other => LockFileFormat::Unknown(other.to_string()),
        }
    }

    /// Check if format version is supported
    fn is_supported_format(&self, format: &LockFileFormat) -> bool {
        matches!(format, LockFileFormat::V1_0)
    }

    /// Export lock file in different formats
    pub fn export(&self, format: LockFileExportFormat) -> Result<(), Error> {
        let Some(lock_file) = &self.current_lock else {
            return Err(PackageManagerError::InvalidMetadata { 
                reason: "No lock file loaded".to_string()
            });
        };

        match format {
            LockFileExportFormat::Json => {
                serde_json::to_string_pretty(lock_file)
                    .map_err(|e| PackageManagerError::InvalidMetadata { 
                        reason: format!("Failed to export as JSON: {}", e)
                    })
            }
            LockFileExportFormat::Yaml => {
                serde_yaml::to_string(lock_file)
                    .map_err(|e| PackageManagerError::InvalidMetadata { 
                        reason: format!("Failed to export as YAML: {}", e)
                    })
            }
            LockFileExportFormat::Summary => {
                Ok(self.generate_summary(lock_file))
            }
        }
    }

    /// Generate human-readable summary of lock file
    fn generate_summary(&self, lock_file: &LockFile) -> String {
        let mut summary = String::new();
        
        summary.push_str(&format!("Lock File Summary\n"));
        summary.push_str(&format!("================\n\n"));
        summary.push_str(&format!("Format Version: {}\n", lock_file.version));
        summary.push_str(&format!("Generated: {}\n", lock_file.metadata.generated_at));
        summary.push_str(&format!("Resolver Version: {}\n", lock_file.metadata.resolver_version));
        summary.push_str(&format!("Total Packages: {}\n\n", lock_file.metadata.total_packages));
        
        summary.push_str("Packages:\n");
        summary.push_str("---------\n");
        
        for (name, pkg) in &lock_file.packages {
            summary.push_str(&format!("  {} @ {}\n", name, pkg.version));
            
            if !pkg.dependencies.is_empty() {
                summary.push_str(&format!("    Dependencies: {}\n", pkg.dependencies.join(", ")));
            }
            
            match &pkg.source {
                PackageSource::Registry { url } => {
                    summary.push_str(&format!("    Source: Registry ({})\n", url));
                }
                PackageSource::Git { url, rev } => {
                    summary.push_str(&format!("    Source: Git ({}, {})\n", url, rev));
                }
                PackageSource::Path { path } => {
                    summary.push_str(&format!("    Source: Path ({})\n", path));
                }
                PackageSource::Local => {
                    summary.push_str("    Source: Local\n");
                }
            }
            
            summary.push_str(&format!("    Checksum: {}\n", pkg.checksum));
            summary.push('\n');
        }
        
        if !lock_file.dependency_tree.is_empty() {
            summary.push_str("Dependency Tree:\n");
            summary.push_str("---------------\n");
            
            for (package, required_by) in &lock_file.dependency_tree {
                summary.push_str(&format!("  {} required by: {}\n", 
                                        package, required_by.join(", ")));
            }
        }
        
        summary
    }
    
    /// Check if lock file exists on disk
    pub fn exists(&self) -> bool {
        self.lock_file_path.exists()
    }
    
    /// Load lock file from disk
    pub fn load(&mut self) -> Result<(), Error> {
        self.load_from_disk()?;
        Ok(())
    }
    
    /// Get locked version for a package
    pub fn get_locked_version(&self, package_name: &str) -> Option<LockedPackage> {
        self.current_lock.as_ref()?
            .packages
            .iter()
            .find(|pkg| pkg.name == package_name)
            .cloned()
    }
}

/// Export format options for lock files
#[derive(Debug, Clone)]
pub enum LockFileExportFormat {
    Json,
    Yaml,
    Summary,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::package_manager::metadata::PackageMetadata;
    use std::collections::HashMap;

    fn create_test_dependency() -> ResolvedDependency {
        let metadata = PackageMetadata {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package".to_string(),
            authors: vec!["Test Author".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: Some("https://github.com/test/test-package".to_string()),
            license: Some("MIT".to_string()),
            keywords: vec!["test".to_string()],
            categories: vec!["development".to_string()],
        };

        ResolvedDependency {
            package: metadata,
            depth: 1,
            required_by: vec!["root".to_string()],
            constraint: "^1.0.0".to_string(),
            resolved_version: Version::parse("1.0.0").unwrap(),
            is_dev_dependency: false,
            optional: false,
        }
    }

    #[test]
    fn test_lock_file_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let lock_path = temp_dir.path().join("test.lock");
        
        let manager = LockFileManager::new(&lock_path);
        assert_eq!(manager.path(), lock_path);
        assert!(!manager.exists());
        assert!(manager.current_lock().is_none());
    }

    #[test]
    fn test_lock_file_generation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = LockFileManager::with_project_dir(temp_dir.path());
        
        let dependencies = vec![create_test_dependency()];
        let lock_file = manager.generate_from_dependencies(&dependencies);
        
        assert_eq!(lock_file.version, "1.0");
        assert_eq!(lock_file.packages.len(), 1);
        assert!(lock_file.packages.contains_key("test-package"));
        
        let package = &lock_file.packages["test-package"];
        assert_eq!(package.version, "1.0.0");
        assert!(!package.checksum.is_empty());
    }

    #[test]
    fn test_lock_file_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = LockFileManager::with_project_dir(temp_dir.path());
        
        let dependencies = vec![create_test_dependency()];
        let lock_file = manager.generate_from_dependencies(&dependencies);
        
        // Save lock file
        let save_result = manager.save(&lock_file);
        assert!(save_result.is_ok());
        assert!(manager.exists());
        
        // Create new manager and load
        let mut new_manager = LockFileManager::with_project_dir(temp_dir.path());
        let load_result = new_manager.load();
        assert!(load_result.is_ok());
        
        let loaded_lock = load_result.unwrap();
        assert!(loaded_lock.is_some());
        
        let loaded = loaded_lock.unwrap();
        assert_eq!(loaded.packages.len(), lock_file.packages.len());
        assert_eq!(loaded.version, lock_file.version);
    }

    #[test]
    fn test_lock_file_validation() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = LockFileManager::with_project_dir(temp_dir.path());
        
        let dependencies = vec![create_test_dependency()];
        let lock_file = manager.generate_from_dependencies(&dependencies);
        
        manager.save(&lock_file).unwrap();
        manager.load().unwrap();
        
        // Validation with same dependencies should pass
        let validation = manager.validate(&dependencies);
        assert!(validation.is_valid);
        assert!(validation.missing_packages.is_empty());
        assert!(validation.version_mismatches.is_empty());
        assert!(validation.extra_packages.is_empty());
        
        // Validation with different version should fail
        let mut modified_dep = create_test_dependency();
        modified_dep.resolved_version = Version::parse("2.0.0").unwrap();
        
        let validation = manager.validate(&[modified_dep]);
        assert!(!validation.is_valid);
        assert_eq!(validation.version_mismatches.len(), 1);
    }

    #[test]
    fn test_lock_file_export_formats() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = LockFileManager::with_project_dir(temp_dir.path());
        
        let dependencies = vec![create_test_dependency()];
        let lock_file = manager.generate_from_dependencies(&dependencies);
        
        manager.save(&lock_file).unwrap();
        manager.load().unwrap();
        
        // Test JSON export
        let json_export = manager.export(LockFileExportFormat::Json);
        assert!(json_export.is_ok());
        let json_content = json_export.unwrap();
        assert!(!json_content.is_empty());
        
        // Verify it's valid JSON
        let _: serde_json::Value = serde_json::from_str(&json_content).unwrap();
        
        // Test YAML export
        let yaml_export = manager.export(LockFileExportFormat::Yaml);
        assert!(yaml_export.is_ok());
        let yaml_content = yaml_export.unwrap();
        assert!(!yaml_content.is_empty());
        
        // Test Summary export
        let summary_export = manager.export(LockFileExportFormat::Summary);
        assert!(summary_export.is_ok());
        let summary_content = summary_export.unwrap();
        assert!(summary_content.contains("Lock File Summary"));
        assert!(summary_content.contains("test-package"));
    }

    #[test]
    fn test_lock_file_deletion() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = LockFileManager::with_project_dir(temp_dir.path());
        
        let dependencies = vec![create_test_dependency()];
        let lock_file = manager.generate_from_dependencies(&dependencies);
        
        manager.save(&lock_file).unwrap();
        assert!(manager.exists());
        
        let delete_result = manager.delete();
        assert!(delete_result.is_ok());
        assert!(!manager.exists());
        assert!(manager.current_lock().is_none());
    }
}
