use crate::error::CursedError;
/// Package Installation System
/// 
/// Provides comprehensive package extraction, installation, and management functionality:
/// - Safe package extraction from tar.gz/zip archives
/// - File conflict resolution and rollback capabilities
/// - Package upgrade and downgrade handling
/// - Cross-platform file permissions management

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs::{self, File, Permissions};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::sync::{Arc, Mutex};
use flate2::read::GzDecoder;
use tar::{Archive, Entry};
use zip::ZipArchive;
use tempfile::TempDir;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, debug, instrument};
use chrono;

use super::{PackageManagerError, PackageMetadata};
use super::database::{PackageDatabase, InstalledPackage};
use super::scripts::{ScriptExecutor, InstallScript, ScriptContext};

/// Package installer with rollback capabilities
#[derive(Debug)]
pub struct PackageInstaller {
    database: Arc<Mutex<PackageDatabase>>,
    script_executor: ScriptExecutor,
    config: InstallerConfig,
    temp_dir: TempDir,
}

/// Installer configuration
#[derive(Debug, Clone)]
pub struct InstallerConfig {
    pub project_root: PathBuf,
    pub extract_to: PathBuf,
    pub allow_overwrites: bool,
    pub backup_existing: bool,
    pub verify_checksums: bool,
    pub enable_scripts: bool,
    pub max_file_size: usize,
    pub preserve_permissions: bool,
}

/// Installation context for a package
#[derive(Debug)]
pub struct InstallationContext {
    pub package: PackageMetadata,
    pub temp_extract_dir: PathBuf,
    pub target_dir: PathBuf,
    pub backup_dir: Option<PathBuf>,
    pub installed_files: Vec<PathBuf>,
    pub scripts: Vec<InstallScript>,
}

/// File operation record for rollback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperation {
    pub operation_type: FileOperationType,
    pub path: PathBuf,
    pub backup_path: Option<PathBuf>,
    pub permissions: Option<u32>,
    pub size: u64,
    pub checksum: Option<String>,
}

/// Types of file operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOperationType {
    Created,
    Overwritten,
    Modified,
    Deleted,
}

/// Package format types
#[derive(Debug, Clone)]
pub enum PackageFormat {
    TarGz,
    Zip,
    Directory,
}

/// Installation errors
#[derive(CursedError, Debug)]
pub enum InstallError {
    #[error("Package format not supported: {format}")]
    UnsupportedFormat { format: String },
    
    #[error("File conflict: {path} already exists")]
    FileConflict { path: PathBuf },
    
    #[error("Extraction failed: {reason}")]
    ExtractionFailed { reason: String },
    
    #[error("Script execution failed: {script} - {error}")]
    ScriptFailed { script: String, error: String },
    
    #[error("Rollback failed: {reason}")]
    RollbackFailed { reason: String },
    
    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },
    
    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },
    
    #[error("File too large: {size} bytes exceeds limit {limit}")]
    FileTooLarge { size: u64, limit: u64 },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Script error: {0}")]
    Script(String),
}

impl PackageInstaller {
    /// Create a new package installer
    pub fn new(
        database: Arc<Mutex<PackageDatabase>>,
        config: InstallerConfig,
    ) -> crate::error::Result<()> {
        let temp_dir = TempDir::new()
            .map_err(|e| InstallError::Io(e))?;
        
        let script_executor = ScriptExecutor::new(
            config.enable_scripts,
            config.project_root.clone(),
        );
        
        Ok(Self {
            database,
            script_executor,
            config,
            temp_dir,
        })
    }
    
    /// Install a package from archive data
    #[instrument(skip(self, package_data), fields(package = %package.name, version = %package.version))]
    pub async fn install_package(
        &mut self,
        package: &PackageMetadata,
        package_data: &[u8],
    ) -> crate::error::Result<()> {
        info!("Starting installation");
        
        // Check if already installed
        if let Ok(existing) = self.get_installed_package(&package.name) {
            if existing.version == package.version {
                info!("Package already installed with same version");
                return Ok(existing);
            }
            
            // Handle upgrade/downgrade
            return self.upgrade_package(package, package_data).await;
        }
        
        // Create installation context
        let mut context = self.create_installation_context(package)?;
        
        // Extract package
        self.extract_package(package_data, &mut context).await?;
        
        // Validate extracted files
        self.validate_extracted_files(&context)?;
        
        // Execute pre-install scripts
        self.execute_pre_install_scripts(&context).await?;
        
        // Install files with backup
        let file_operations = self.install_files(&mut context)?;
        
        // Execute post-install scripts
        self.execute_post_install_scripts(&context).await?;
        
        // Register in database
        let installed_package = self.register_package(&context, file_operations)?;
        
        info!("Installation completed successfully");
        Ok(installed_package)
    }
    
    /// Upgrade an existing package
    #[instrument(skip(self, package_data), fields(package = %package.name, version = %package.version))]
    pub async fn upgrade_package(
        &mut self,
        package: &PackageMetadata,
        package_data: &[u8],
    ) -> crate::error::Result<()> {
        info!("Starting package upgrade");
        
        let existing = self.get_installed_package(&package.name)?;
        
        // Create rollback point
        let rollback_data = self.create_rollback_point(&existing)?;
        
        match Box::pin(self.perform_upgrade(package, package_data)).await {
            Ok(installed) => {
                // Clean up rollback data on success
                self.cleanup_rollback_point(rollback_data)?;
                Ok(installed)
            }
            Err(e) => {
                // Rollback on failure
                error!("Upgrade failed, rolling back: {}", e);
                self.perform_rollback(rollback_data)?;
                Err(e)
            }
        }
    }
    
    /// Uninstall a package
    #[instrument(skip(self), fields(package_name))]
    pub fn uninstall_package(&mut self, package_name: &str) -> crate::error::Result<()> {
        info!("Starting package uninstallation");
        
        let installed = self.get_installed_package(package_name)?;
        
        // Check dependencies
        self.check_uninstall_dependencies(&installed)?;
        
        // Execute pre-uninstall scripts
        self.execute_uninstall_scripts(&installed, "pre")?;
        
        // Remove files
        self.remove_package_files(&installed)?;
        
        // Execute post-uninstall scripts
        self.execute_uninstall_scripts(&installed, "post")?;
        
        // Remove from database
        self.unregister_package(package_name)?;
        
        info!("Uninstallation completed successfully");
        Ok(())
    }
    
    /// List all installed packages
    pub fn list_installed(&self) -> crate::error::Result<()> {
        let db = self.database.lock()
            .map_err(|e| InstallError::Database(format!("Lock failed: {}", e)))?;
        db.list_packages()
            .map_err(|e| InstallError::Database(e.to_string()))
    }
    
    /// Get specific installed package
    pub fn get_installed_package(&self, name: &str) -> crate::error::Result<()> {
        let db = self.database.lock()
            .map_err(|e| InstallError::Database(format!("Lock failed: {}", e)))?;
        db.get_package(name)
            .map_err(|e| InstallError::Database(e.to_string()))
    }
    
    /// Check package integrity
    pub fn verify_package(&self, package_name: &str) -> crate::error::Result<()> {
        let installed = self.get_installed_package(package_name)?;
        
        for file_op in &installed.file_operations {
            if !file_op.path.exists() {
                warn!("Missing file: {:?}", file_op.path);
                return Ok(false);
            }
            
            // Check checksum if available
            if let Some(expected_checksum) = &file_op.checksum {
                let actual_checksum = self.calculate_file_checksum(&file_op.path)?;
                if actual_checksum != *expected_checksum {
                    warn!("Checksum mismatch for {:?}", file_op.path);
                    return Ok(false);
                }
            }
        }
        
        Ok(true)
    }
    
    /// Create installation context
    fn create_installation_context(
        &self,
        package: &PackageMetadata,
    ) -> crate::error::Result<()> {
        let temp_extract_dir = self.temp_dir.path().join(format!("{}-{}", package.name, package.version));
        fs::create_dir_all(&temp_extract_dir)?;
        
        let target_dir = self.config.extract_to.join(&package.name);
        
        let backup_dir = if self.config.backup_existing {
            let backup = self.temp_dir.path().join(format!("backup-{}-{}", package.name, package.version));
            fs::create_dir_all(&backup)?;
            Some(backup)
        } else {
            None
        };
        
        Ok(InstallationContext {
            package: package.clone(),
            temp_extract_dir,
            target_dir,
            backup_dir,
            installed_files: Vec::new(),
            scripts: Vec::new(),
        })
    }
    
    /// Extract package based on format
    async fn extract_package(
        &self,
        package_data: &[u8],
        context: &mut InstallationContext,
    ) -> crate::error::Result<()> {
        let format = self.detect_package_format(package_data)?;
        
        match format {
            PackageFormat::TarGz => self.extract_tar_gz(package_data, context).await,
            PackageFormat::Zip => self.extract_zip(package_data, context).await,
            PackageFormat::Directory => Err(InstallError::UnsupportedFormat {
                format: "Directory format not supported for package data".to_string(),
            }),
        }
    }
    
    /// Extract tar.gz archive
    async fn extract_tar_gz(
        &self,
        package_data: &[u8],
        context: &mut InstallationContext,
    ) -> crate::error::Result<()> {
        let gz_decoder = GzDecoder::new(package_data);
        let mut archive = Archive::new(gz_decoder);
        
        for entry_result in archive.entries()? {
            let mut entry = entry_result?;
            let path = entry.path()?.into_owned();
            
            // Security check - prevent path traversal
            if path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
                warn!("Skipping potentially dangerous path: {:?}", path);
                continue;
            }
            
            let extract_path = context.temp_extract_dir.join(&path);
            
            // Check file size
            let size = entry.header().size()?;
            if size > self.config.max_file_size as u64 {
                return Err(InstallError::FileTooLarge {
                    size,
                    limit: self.config.max_file_size as u64,
                });
            }
            
            // Create parent directories
            if let Some(parent) = extract_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Extract file
            entry.unpack(&extract_path)?;
            
            // Set permissions if configured
            if self.config.preserve_permissions {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mode = entry.header().mode()?;
                    fs::set_permissions(&extract_path, Permissions::from_mode(mode))?;
                }
            }
            
            context.installed_files.push(extract_path);
        }
        
        Ok(())
    }
    
    /// Extract zip archive
    async fn extract_zip(
        &self,
        package_data: &[u8],
        context: &mut InstallationContext,
    ) -> crate::error::Result<()> {
        let cursor = std::io::Cursor::new(package_data);
        let mut archive = ZipArchive::new(cursor)
            .map_err(|e| InstallError::ExtractionFailed { reason: e.to_string() })?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| InstallError::ExtractionFailed { reason: e.to_string() })?;
            
            let file_path = file.mangled_name();
            
            // Security check
            if file_path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
                warn!("Skipping potentially dangerous path: {:?}", file_path);
                continue;
            }
            
            let extract_path = context.temp_extract_dir.join(&file_path);
            
            // Check file size
            if file.size() > self.config.max_file_size as u64 {
                return Err(InstallError::FileTooLarge {
                    size: file.size(),
                    limit: self.config.max_file_size as u64,
                });
            }
            
            if file.is_dir() {
                fs::create_dir_all(&extract_path)?;
            } else {
                // Create parent directories
                if let Some(parent) = extract_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                
                // Extract file
                let mut output_file = File::create(&extract_path)?;
                io::copy(&mut file, &mut output_file)?;
                
                // Set permissions
                #[cfg(unix)]
                if self.config.preserve_permissions {
                    use std::os::unix::fs::PermissionsExt;
                    if let Some(mode) = file.unix_mode() {
                        fs::set_permissions(&extract_path, Permissions::from_mode(mode))?;
                    }
                }
            }
            
            context.installed_files.push(extract_path);
        }
        
        Ok(())
    }
    
    /// Detect package format from data
    fn detect_package_format(&self, data: &[u8]) -> crate::error::Result<()> {
        if data.len() < 4 {
            return Err(InstallError::UnsupportedFormat {
                format: "Unknown format - insufficient data".to_string(),
            });
        }
        
        // Check for gzip magic bytes
        if data[0] == 0x1f && data[1] == 0x8b {
            return Ok(PackageFormat::TarGz);
        }
        
        // Check for zip magic bytes
        if data[0] == 0x50 && data[1] == 0x4b {
            return Ok(PackageFormat::Zip);
        }
        
        Err(InstallError::UnsupportedFormat {
            format: "Unknown archive format".to_string(),
        })
    }
    
    /// Validate extracted files
    fn validate_extracted_files(&self, context: &InstallationContext) -> crate::error::Result<()> {
        // Check for required files (customize based on package requirements)
        let required_files = vec!["CursedPackage.toml"];
        
        for required in required_files {
            let required_path = context.temp_extract_dir.join(required);
            if !required_path.exists() {
                return Err(InstallError::ExtractionFailed {
                    reason: format!("Required file missing: {}", required),
                });
            }
        }
        
        Ok(())
    }
    
    /// Execute pre-install scripts
    async fn execute_pre_install_scripts(
        &self,
        context: &InstallationContext,
    ) -> crate::error::Result<()> {
        for script in &context.scripts {
            if script.phase == "pre-install" {
                let script_context = ScriptContext {
                    package_name: context.package.name.clone(),
                    package_version: context.package.version.clone(),
                    install_dir: context.target_dir.clone(),
                    temp_dir: context.temp_extract_dir.clone(),
                };
                
                self.script_executor.execute_script(script, &script_context).await
                    .map_err(|e| InstallError::ScriptFailed {
                        script: script.name.clone(),
                        error: e.to_string(),
                    })?;
            }
        }
        
        Ok(())
    }
    
    /// Execute post-install scripts
    async fn execute_post_install_scripts(
        &self,
        context: &InstallationContext,
    ) -> crate::error::Result<()> {
        for script in &context.scripts {
            if script.phase == "post-install" {
                let script_context = ScriptContext {
                    package_name: context.package.name.clone(),
                    package_version: context.package.version.clone(),
                    install_dir: context.target_dir.clone(),
                    temp_dir: context.temp_extract_dir.clone(),
                };
                
                self.script_executor.execute_script(script, &script_context).await
                    .map_err(|e| InstallError::ScriptFailed {
                        script: script.name.clone(),
                        error: e.to_string(),
                    })?;
            }
        }
        
        Ok(())
    }
    
    /// Install files with conflict resolution
    fn install_files(
        &mut self,
        context: &mut InstallationContext,
    ) -> crate::error::Result<()> {
        let mut file_operations = Vec::new();
        
        for file_path in &context.installed_files {
            let relative_path = file_path.strip_prefix(&context.temp_extract_dir)
                .map_err(|e| InstallError::ExtractionFailed { reason: e.to_string() })?;
            
            let target_path = context.target_dir.join(relative_path);
            
            // Handle file conflicts
            let operation = if target_path.exists() {
                if !self.config.allow_overwrites {
                    return Err(InstallError::FileConflict { path: target_path });
                }
                
                // Backup existing file
                if let Some(backup_dir) = &context.backup_dir {
                    let backup_path = backup_dir.join(relative_path);
                    if let Some(parent) = backup_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::copy(&target_path, &backup_path)?;
                }
                
                FileOperationType::Overwritten
            } else {
                FileOperationType::Created
            };
            
            // Create target directory
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Copy file
            fs::copy(file_path, &target_path)?;
            
            // Calculate checksum if verification enabled
            let checksum = if self.config.verify_checksums {
                Some(self.calculate_file_checksum(&target_path)?)
            } else {
                None
            };
            
            // Get file metadata
            let metadata = fs::metadata(&target_path)?;
            let permissions = if self.config.preserve_permissions {
                Some(self.get_file_permissions(&metadata))
            } else {
                None
            };
            
            file_operations.push(FileOperation {
                operation_type: operation,
                path: target_path,
                backup_path: context.backup_dir.as_ref().map(|d| d.join(relative_path)),
                permissions,
                size: metadata.len(),
                checksum,
            });
        }
        
        Ok(file_operations)
    }
    
    /// Register package in database
    fn register_package(
        &self,
        context: &InstallationContext,
        file_operations: Vec<FileOperation>,
    ) -> crate::error::Result<()> {
        let installed_package = InstalledPackage {
            name: context.package.name.clone(),
            version: context.package.version.clone(),
            install_time: chrono::Utc::now(),
            install_path: context.target_dir.clone(),
            file_operations,
            metadata: context.package.clone(),
        };
        
        let mut db = self.database.lock()
            .map_err(|e| InstallError::Database(format!("Lock failed: {}", e)))?;
        
        db.add_package(&installed_package)
            .map_err(|e| InstallError::Database(e.to_string()))?;
        
        Ok(installed_package)
    }
    
    /// Calculate file checksum (SHA-256)
    fn calculate_file_checksum(&self, path: &Path) -> crate::error::Result<()> {
        use sha2::{Sha256, Digest};
        
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];
        
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }
    
    /// Get file permissions (cross-platform)
    fn get_file_permissions(&self, metadata: &fs::Metadata) -> u32 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            metadata.mode()
        }
        
        #[cfg(windows)]
        {
            // Windows doesn't have UNIX-style permissions
            // Return a default value or implement Windows-specific logic
            0o644
        }
        
        #[cfg(not(any(unix, windows)))]
        {
            0o644
        }
    }
    
    /// Create rollback point for upgrade
    fn create_rollback_point(&self, package: &InstalledPackage) -> crate::error::Result<()> {
        info!("Creating rollback point for package {}", package.name);
        
        let rollback_dir = self.temp_dir.path().join(format!("rollback-{}-{}", package.name, package.version));
        fs::create_dir_all(&rollback_dir)?;
        
        let mut backed_up_files = Vec::new();
        let mut registry_snapshot = Vec::new();
        
        // Backup all files from the current installation
        for file_op in &package.file_operations {
            if file_op.path.exists() {
                let relative_path = file_op.path.strip_prefix(&package.install_path)
                    .unwrap_or(&file_op.path);
                let backup_path = rollback_dir.join("files").join(relative_path);
                
                // Create parent directories
                if let Some(parent) = backup_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                
                // Copy file with metadata preservation
                fs::copy(&file_op.path, &backup_path)?;
                
                // Preserve permissions if configured
                if self.config.preserve_permissions {
                    let metadata = fs::metadata(&file_op.path)?;
                    let permissions = self.get_file_permissions(&metadata);
                    
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        fs::set_permissions(&backup_path, fs::Permissions::from_mode(permissions))?;
                    }
                }
                
                backed_up_files.push(RollbackFileInfo {
                    original_path: file_op.path.clone(),
                    backup_path: backup_path.clone(),
                    operation_type: file_op.operation_type.clone(),
                    checksum: file_op.checksum.clone(),
                    permissions: file_op.permissions,
                });
            }
        }
        
        // Create registry snapshot
        let registry_backup_path = rollback_dir.join("registry_snapshot.json");
        {
            let db = self.database.lock()
                .map_err(|e| InstallError::Database(format!("Lock failed: {}", e)))?;
            
            if let Ok(all_packages) = db.list_packages() {
                registry_snapshot = all_packages;
                
                // Serialize and save registry state
                let snapshot_data = serde_json::to_string_pretty(&registry_snapshot)
                    .map_err(|e| InstallError::Database(format!("Serialization failed: {}", e)))?;
                fs::write(&registry_backup_path, snapshot_data)?;
            }
        }
        
        // Create metadata snapshot
        let metadata_backup_path = rollback_dir.join("package_metadata.json");
        let metadata_data = serde_json::to_string_pretty(package)
            .map_err(|e| InstallError::Database(format!("Metadata serialization failed: {}", e)))?;
        fs::write(&metadata_backup_path, metadata_data)?;
        
        let rollback_data = RollbackData {
            package_name: package.name.clone(),
            package_version: package.version.clone(),
            rollback_dir: rollback_dir.clone(),
            backed_up_files,
            registry_snapshot,
            registry_backup_path,
            metadata_backup_path,
            created_at: chrono::Utc::now(),
        };
        
        // Save rollback manifest
        let manifest_path = rollback_dir.join("rollback_manifest.json");
        let manifest_data = serde_json::to_string_pretty(&rollback_data)
            .map_err(|e| InstallError::Database(format!("Manifest serialization failed: {}", e)))?;
        fs::write(&manifest_path, manifest_data)?;
        
        info!("Rollback point created successfully at {:?}", rollback_dir);
        Ok(rollback_data)
    }
    
    /// Perform actual upgrade
    async fn perform_upgrade(
        &mut self,
        package: &PackageMetadata,
        package_data: &[u8],
    ) -> crate::error::Result<()> {
        // Remove old version first
        self.uninstall_package(&package.name)?;
        
        // Install new version without going through upgrade path
        self.install_package_direct(package, package_data).await
    }
    
    /// Install package directly without upgrade check
    async fn install_package_direct(
        &mut self,
        package: &PackageMetadata,
        package_data: &[u8],
    ) -> crate::error::Result<()> {
        info!("Starting direct installation");
        
        // Create installation context
        let mut context = self.create_installation_context(package)?;
        
        // Extract package
        self.extract_package(package_data, &mut context).await?;
        
        // Validate extracted files
        self.validate_extracted_files(&context)?;
        
        // Execute pre-install scripts
        self.execute_pre_install_scripts(&context).await?;
        
        // Install files with backup
        let file_operations = self.install_files(&mut context)?;
        
        // Execute post-install scripts
        self.execute_post_install_scripts(&context).await?;
        
        // Register in database
        let installed_package = self.register_package(&context, file_operations)?;
        
        info!("Direct installation completed successfully");
        Ok(installed_package)
    }
    
    /// Cleanup rollback point
    fn cleanup_rollback_point(&self, _rollback_data: RollbackData) -> crate::error::Result<()> {
        // Implementation would clean up rollback data
        Ok(())
    }
    
    /// Perform rollback
    fn perform_rollback(&self, rollback_data: RollbackData) -> crate::error::Result<()> {
        info!("Performing rollback for package {}@{}", rollback_data.package_name, rollback_data.package_version);
        
        // Verify rollback data exists
        if !rollback_data.rollback_dir.exists() {
            return Err(InstallError::RollbackFailed {
                reason: "Rollback directory not found".to_string(),
            });
        }
        
        // Step 1: Remove current package files
        {
            let db = self.database.lock()
                .map_err(|e| InstallError::Database(format!("Lock failed: {}", e)))?;
            
            if let Ok(current_package) = db.get_package(&rollback_data.package_name) {
                debug!("Removing current package files before rollback");
                
                // Remove files in reverse order to handle dependencies
                let mut file_ops = current_package.file_operations.clone();
                file_ops.reverse();
                
                for file_op in &file_ops {
                    if file_op.path.exists() {
                        match file_op.operation_type {
                            FileOperationType::Created => {
                                // Remove files that were created
                                if file_op.path.is_dir() {
                                    if let Err(e) = fs::remove_dir_all(&file_op.path) {
                                        warn!("Failed to remove directory {:?}: {}", file_op.path, e);
                                    }
                                } else if let Err(e) = fs::remove_file(&file_op.path) {
                                    warn!("Failed to remove file {:?}: {}", file_op.path, e);
                                }
                            }
                            FileOperationType::Overwritten | FileOperationType::Modified => {
                                // These will be restored from backup
                                if let Err(e) = fs::remove_file(&file_op.path) {
                                    warn!("Failed to remove modified file {:?}: {}", file_op.path, e);
                                }
                            }
                            FileOperationType::Deleted => {
                                // File was deleted during installation, will be restored
                            }
                        }
                    }
                }
            }
        }
        
        // Step 2: Restore backed up files
        debug!("Restoring backed up files");
        for file_info in &rollback_data.backed_up_files {
            if file_info.backup_path.exists() {
                // Create parent directories for original path
                if let Some(parent) = file_info.original_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                
                // Restore file
                fs::copy(&file_info.backup_path, &file_info.original_path)
                    .map_err(|e| InstallError::RollbackFailed {
                        reason: format!("Failed to restore file {:?}: {}", file_info.original_path, e),
                    })?;
                
                // Restore permissions
                if let Some(permissions) = file_info.permissions {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        fs::set_permissions(&file_info.original_path, fs::Permissions::from_mode(permissions))
                            .map_err(|e| InstallError::RollbackFailed {
                                reason: format!("Failed to restore permissions for {:?}: {}", file_info.original_path, e),
                            })?;
                    }
                }
                
                // Verify checksum if available
                if let Some(expected_checksum) = &file_info.checksum {
                    let actual_checksum = self.calculate_file_checksum(&file_info.original_path)?;
                    if actual_checksum != *expected_checksum {
                        warn!("Checksum mismatch after rollback for {:?}", file_info.original_path);
                    }
                }
            } else {
                warn!("Backup file not found: {:?}", file_info.backup_path);
            }
        }
        
        // Step 3: Restore registry state
        debug!("Restoring registry state");
        {
            let mut db = self.database.lock()
                .map_err(|e| InstallError::Database(format!("Lock failed: {}", e)))?;
            
            // Remove current package from registry
            if let Err(e) = db.remove_package(&rollback_data.package_name) {
                warn!("Failed to remove package from registry during rollback: {}", e);
            }
            
            // Restore original package from snapshot
            if rollback_data.registry_backup_path.exists() {
                let snapshot_data = fs::read_to_string(&rollback_data.registry_backup_path)
                    .map_err(|e| InstallError::RollbackFailed {
                        reason: format!("Failed to read registry snapshot: {}", e),
                    })?;
                
                let original_packages: Vec<InstalledPackage> = serde_json::from_str(&snapshot_data)
                    .map_err(|e| InstallError::RollbackFailed {
                        reason: format!("Failed to deserialize registry snapshot: {}", e),
                    })?;
                
                // Find and restore the original package
                for package in original_packages {
                    if package.name == rollback_data.package_name {
                        db.add_package(&package)
                            .map_err(|e| InstallError::RollbackFailed {
                                reason: format!("Failed to restore package to registry: {}", e),
                            })?;
                        break;
                    }
                }
            }
        }
        
        // Step 4: Verify rollback success
        debug!("Verifying rollback success");
        if let Ok(restored_package) = self.get_installed_package(&rollback_data.package_name) {
            if restored_package.version != rollback_data.package_version {
                return Err(InstallError::RollbackFailed {
                    reason: format!(
                        "Rollback verification failed: expected version {}, got {}",
                        rollback_data.package_version,
                        restored_package.version
                    ),
                });
            }
            
            // Verify a few key files exist
            let mut verified_files = 0;
            for file_info in rollback_data.backed_up_files.iter().take(5) {
                if file_info.original_path.exists() {
                    verified_files += 1;
                }
            }
            
            if verified_files == 0 && !rollback_data.backed_up_files.is_empty() {
                return Err(InstallError::RollbackFailed {
                    reason: "No backed up files were successfully restored".to_string(),
                });
            }
        }
        
        info!("Rollback completed successfully for package {}@{}", 
              rollback_data.package_name, rollback_data.package_version);
        Ok(())
    }
    
    /// Check dependencies before uninstall
    fn check_uninstall_dependencies(&self, _package: &InstalledPackage) -> crate::error::Result<()> {
        // Check if other packages depend on this one
        // For now, just succeed
        Ok(())
    }
    
    /// Execute uninstall scripts
    fn execute_uninstall_scripts(&self, _package: &InstalledPackage, _phase: &str) -> crate::error::Result<()> {
        // Implementation would execute uninstall scripts
        Ok(())
    }
    
    /// Remove package files
    fn remove_package_files(&self, package: &InstalledPackage) -> crate::error::Result<()> {
        for file_op in &package.file_operations {
            if file_op.path.exists() {
                if file_op.path.is_dir() {
                    fs::remove_dir_all(&file_op.path)?;
                } else {
                    fs::remove_file(&file_op.path)?;
                }
            }
        }
        
        // Clean up empty directories
        if package.install_path.exists() {
            if let Ok(mut entries) = fs::read_dir(&package.install_path) {
                if entries.next().is_none() {
                    fs::remove_dir(&package.install_path)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Unregister package from database
    fn unregister_package(&self, package_name: &str) -> crate::error::Result<()> {
        let mut db = self.database.lock()
            .map_err(|e| InstallError::Database(format!("Lock failed: {}", e)))?;
        
        db.remove_package(package_name)
            .map_err(|e| InstallError::Database(e.to_string()))?;
        
        Ok(())
    }
}

/// Rollback data structure
#[derive(Debug, Serialize, Deserialize)]
struct RollbackData {
    package_name: String,
    package_version: String,
    rollback_dir: PathBuf,
    backed_up_files: Vec<RollbackFileInfo>,
    registry_snapshot: Vec<InstalledPackage>,
    registry_backup_path: PathBuf,
    metadata_backup_path: PathBuf,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// Information about a backed up file for rollback
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RollbackFileInfo {
    original_path: PathBuf,
    backup_path: PathBuf,
    operation_type: FileOperationType,
    checksum: Option<String>,
    permissions: Option<u32>,
}

impl Default for InstallerConfig {
    fn default() -> Self {
        Self {
            project_root: std::env::current_dir().unwrap_or_default(),
            extract_to: PathBuf::from("cursed_packages"),
            allow_overwrites: true,
            backup_existing: true,
            verify_checksums: true,
            enable_scripts: true,
            max_file_size: 100 * 1024 * 1024, // 100MB
            preserve_permissions: true,
        }
    }
}
