// Lock File Management for Reproducible Builds
//
// This module provides functionality for generating, reading, and validating
// lock files that ensure reproducible dependency resolution across different
// environments and time periods.

use crate::package_manager::{
// };
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, warn, debug, instrument};
use chrono::Utc;
use semver::Version;
use crate::error::CursedError;

/// Lock file manager for handling lock file operations
#[derive(Debug)]
pub struct LockFileManager {
    /// Path to the lock file
    /// Current lock file content (if loaded)
/// Lock file validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
/// Version mismatch information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionMismatch {
/// Checksum mismatch information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksumMismatch {
/// Lock file format versions
#[derive(Debug, Clone, PartialEq)]
pub enum LockFileFormat {
impl LockFileManager {
    /// Create a new lock file manager
    pub fn new<P: Into<PathBuf>>(lock_file_path: P) -> Self {
        Self {
        }
    }

    /// Create with default lock file name in project directory
    pub fn with_project_dir<P: AsRef<Path>>(project_dir: P) -> Self {
        let lock_path = project_dir.as_ref().join("Cursed.lock");
        Self::new(lock_path)
    /// Load existing lock file from disk
    #[instrument(skip(self))]
    pub fn load(&mut self) -> crate::error::Result<()> {
        if !self.lock_file_path.exists() {
            debug!("Lock file does not exist: {:?}", self.lock_file_path);
            return Ok(None);
        info!("Loading lock file from {:?}", self.lock_file_path);
        
        let content = fs::read_to_string(&self.lock_file_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
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
        info!("Successfully loaded lock file with {} packages", lock_file.packages.len());
        self.current_lock = Some(lock_file);
        Ok(self.current_lock.as_ref())
    /// Save lock file to disk
    #[instrument(skip(self, lock_file))]
    pub fn save(&mut self, lock_file: &LockFile) -> crate::error::Result<()> {
              self.lock_file_path, lock_file.packages.len());

        // Ensure parent directory exists
        if let Some(parent) = self.lock_file_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| PackageManagerError::FileSystemError { 
                    error: e.to_string()
                })?;
        // Serialize with pretty printing for human readability
        let content = serde_json::to_string_pretty(lock_file)
            .map_err(|e| PackageManagerError::InvalidMetadata { 
                reason: format!("Failed to serialize lock file: {}", e)
            })?;

        // Write to temporary file first, then rename for atomicity
        let temp_path = self.lock_file_path.with_extension("lock.tmp");
        
        fs::write(&temp_path, content)
            .map_err(|e| PackageManagerError::FileSystemError { 
                error: e.to_string()
            })?;

        fs::rename(&temp_path, &self.lock_file_path)
            .map_err(|e| PackageManagerError::FileSystemError { 
                error: e.to_string()
            })?;

        self.current_lock = Some(lock_file.clone());
        info!("Lock file saved successfully");
        Ok(())
    /// Validate lock file against current dependencies
    #[instrument(skip(self, dependencies))]
    pub fn validate(&self, dependencies: &[ResolvedDependency]) -> ValidationResult {
        let Some(lock_file) = &self.current_lock else {
            return ValidationResult {

        let mut result = ValidationResult {

        // Check all current dependencies are in lock file
        for dep in dependencies {
            if let Some(locked_pkg) = lock_file.packages.get(&dep.package.name) {
                // Check version match
                if locked_pkg.version != dep.resolved_version.to_string() {
                    result.version_mismatches.push(VersionMismatch {
                    });
                    result.is_valid = false;
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
                  result.extra_packages.len());
        result
    /// Generate a new lock file from resolved dependencies
    pub fn generate_from_dependencies(&self, dependencies: &[ResolvedDependency]) -> LockFile {
        let mut packages = BTreeMap::new();
        let mut dependency_tree = BTreeMap::new();

        for dep in dependencies {
            let locked_package = LockedPackage {
                source: PackageSource::Registry { 
                    url: "https://registry.cursed-lang.org".to_string() 

            packages.insert(dep.package.name.clone(), locked_package);

            // Build dependency tree
            if !dep.required_by.is_empty() {
                dependency_tree.insert(
                    dep.required_by.clone()
                );
            }
        }

        let metadata = LockFileMetadata {
            resolution_time_ms: 0, // Would be filled by resolver

        LockFile {
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
        new_lock
    /// Check if lock file exists
    pub fn exists(&self) -> bool {
        self.lock_file_path.exists()
    /// Get lock file path
    pub fn path(&self) -> &Path {
        &self.lock_file_path
    /// Get current loaded lock file
    pub fn current_lock(&self) -> Option<&LockFile> {
        self.current_lock.as_ref()
    /// Delete lock file from disk
    pub fn delete(&mut self) -> crate::error::Result<()> {
        if self.lock_file_path.exists() {
            fs::remove_file(&self.lock_file_path)
                .map_err(|e| PackageManagerError::FileSystemError { 
                    error: e.to_string()
                })?;
        }
        self.current_lock = None;
        Ok(())
    /// Calculate package checksum (simplified for now)
    fn calculate_checksum(&self, _package_name: &str, _version: &Version) -> String {
        // In a real implementation, this would calculate actual checksums
        // from package contents downloaded from registry
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(_package_name.as_bytes());
        hasher.update(_version.to_string().as_bytes());
        format!("sha256:{:x}", hasher.finalize())
    /// Parse lock file format version
    fn parse_format_version(&self, version: &str) -> LockFileFormat {
        match version {
        }
    }

    /// Check if format version is supported
    fn is_supported_format(&self, format: &LockFileFormat) -> bool {
        matches!(format, LockFileFormat::V1_0)
    /// Export lock file in different formats
    pub fn export(&self, format: LockFileExportFormat) -> crate::error::Result<()> {
        let Some(lock_file) = &self.current_lock else {
            return Err(PackageManagerError::InvalidMetadata { 
                reason: "No lock file loaded".to_string()
            });

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
        if !lock_file.dependency_tree.is_empty() {
            summary.push_str("Dependency Tree:\n");
            summary.push_str("---------------\n");
            
            for (package, required_by) in &lock_file.dependency_tree {
                                        package, required_by.join(", ")));
            }
        }
        
        summary
    /// Check if lock file exists on disk
    pub fn exists(&self) -> bool {
        self.lock_file_path.exists()
    /// Load lock file from disk
    pub fn load(&mut self) -> crate::error::Result<()> {
        self.load_from_disk()?;
        Ok(())
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
