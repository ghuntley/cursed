//! Package installer for CURSED
//!
//! This module handles installing and removing packages

use crate::error::{CursedError, Result};
use crate::package_manager::version::Version;
use crate::package_manager::downloader::{DownloadedPackage, PackageDownloader, PackageDownloadRequest};
use crate::package_manager::resolver::{ResolvedPackage, ResolutionResult};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tokio::fs as async_fs;

/// Installation configuration
#[derive(Debug, Clone)]
pub struct InstallConfig {
    pub install_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub parallel_installs: usize,
    pub verify_integrity: bool,
    pub create_symlinks: bool,
    pub backup_on_upgrade: bool,
}

/// Installed package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPackageInfo {
    pub name: String,
    pub version: Version,
    pub install_path: PathBuf,
    pub installed_files: Vec<PathBuf>,
    pub dependencies: Vec<String>,
    pub install_time: std::time::SystemTime,
    pub checksum: String,
    pub source_url: String,
}

/// Installation result
#[derive(Debug, Clone)]
pub struct InstallResult {
    pub installed_packages: Vec<InstalledPackageInfo>,
    pub failed_packages: Vec<(String, CursedError)>,
    pub total_time: std::time::Duration,
}

/// Uninstallation result
#[derive(Debug, Clone)]
pub struct UninstallResult {
    pub removed_packages: Vec<String>,
    pub failed_removals: Vec<(String, CursedError)>,
    pub orphaned_dependencies: Vec<String>,
}

/// Package installer
#[derive(Debug)]
pub struct PackageInstaller {
    config: InstallConfig,
    downloader: PackageDownloader,
    installed_packages: HashMap<String, InstalledPackageInfo>,
}

impl Default for InstallConfig {
    fn default() -> Self {
        Self {
            install_dir: PathBuf::from("target/packages"),
            temp_dir: PathBuf::from("target/temp"),
            parallel_installs: 4,
            verify_integrity: true,
            create_symlinks: false,
            backup_on_upgrade: true,
        }
    }
}

impl PackageInstaller {
    /// Create a new package installer
    pub fn new(config: InstallConfig, downloader: PackageDownloader) -> Result<Self> {
        // Ensure directories exist
        fs::create_dir_all(&config.install_dir)?;
        fs::create_dir_all(&config.temp_dir)?;

        let mut installer = Self {
            config,
            downloader,
            installed_packages: HashMap::new(),
        };

        // Load existing installation information
        installer.load_installed_packages()?;

        Ok(installer)
    }

    /// Install packages from resolution result
    pub async fn install_packages(&mut self, resolution: ResolutionResult) -> Result<InstallResult> {
        let start_time = std::time::Instant::now();
        
        tracing::info!("Installing {} resolved packages", resolution.resolved_packages.len());

        let mut installed_packages = Vec::new();
        let mut failed_packages = Vec::new();

        // Download all packages first
        let download_requests = self.create_download_requests(&resolution.resolved_packages)?;
        let downloaded_packages = match self.downloader.download_packages(download_requests).await {
            Ok(packages) => packages,
            Err(e) => {
                tracing::error!("Failed to download packages: {}", e);
                return Ok(InstallResult {
                    installed_packages,
                    failed_packages: vec![("download".to_string(), e)],
                    total_time: start_time.elapsed(),
                });
            }
        };

        // Install packages in resolution order
        for package_name in &resolution.resolution_order {
            if let Some(resolved_package) = resolution.resolved_packages.iter()
                .find(|p| &p.name == package_name) {
                
                if let Some(downloaded_package) = downloaded_packages.iter()
                    .find(|p| &p.name == package_name) {
                    
                    match self.install_single_package(resolved_package, downloaded_package).await {
                        Ok(installed_info) => {
                            installed_packages.push(installed_info);
                        }
                        Err(e) => {
                            tracing::error!("Failed to install {}: {}", package_name, e);
                            failed_packages.push((package_name.clone(), e));
                        }
                    }
                }
            }
        }

        // Update installed packages database
        self.save_installed_packages()?;

        let result = InstallResult {
            installed_packages,
            failed_packages,
            total_time: start_time.elapsed(),
        };

        tracing::info!("Installation completed in {:?}. {} successful, {} failed",
                      result.total_time, result.installed_packages.len(), result.failed_packages.len());

        Ok(result)
    }

    /// Install a single package
    async fn install_single_package(&mut self, 
                                   resolved: &ResolvedPackage,
                                   downloaded: &DownloadedPackage) -> Result<InstalledPackageInfo> {
        
        tracing::info!("Installing {} {}", resolved.name, resolved.version);

        // Check if already installed with same version
        if let Some(existing) = self.installed_packages.get(&resolved.name) {
            if existing.version == resolved.version {
                tracing::info!("Package {} {} already installed", resolved.name, resolved.version);
                return Ok(existing.clone());
            } else if self.config.backup_on_upgrade {
                self.backup_package(existing).await?;
            }
        }

        // Create installation directory
        let install_path = self.get_package_install_path(&resolved.name, &resolved.version);
        async_fs::create_dir_all(&install_path).await?;

        // Extract package
        let installed_files = self.extract_package(&downloaded.local_path, &install_path).await?;

        // Verify installation
        if self.config.verify_integrity {
            self.verify_installation(&install_path, &installed_files).await?;
        }

        // Create package metadata
        let installed_info = InstalledPackageInfo {
            name: resolved.name.clone(),
            version: resolved.version.clone(),
            install_path: install_path,
            installed_files,
            dependencies: resolved.dependencies.iter().map(|d| d.name.clone()).collect(),
            install_time: std::time::SystemTime::now(),
            checksum: downloaded.checksum.clone(),
            source_url: downloaded.download_url.clone(),
        };

        // Update installed packages registry
        self.installed_packages.insert(resolved.name.clone(), installed_info.clone());

        // Create symlinks if configured
        if self.config.create_symlinks {
            self.create_package_symlinks(&installed_info).await?;
        }

        tracing::info!("Successfully installed {} {}", resolved.name, resolved.version);
        Ok(installed_info)
    }

    /// Uninstall a package
    pub async fn uninstall_package(&mut self, package_name: &str, remove_dependencies: bool) -> Result<UninstallResult> {
        tracing::info!("Uninstalling package: {}", package_name);

        let mut removed_packages = Vec::new();
        let mut failed_removals = Vec::new();
        let mut orphaned_dependencies = Vec::new();

        // Check if package is installed
        let installed_info = match self.installed_packages.get(package_name) {
            Some(info) => info.clone(),
            None => {
                return Err(CursedError::General(format!("Package {} is not installed", package_name)));
            }
        };

        // Check for dependent packages
        let dependents = self.find_dependent_packages(package_name);
        if !dependents.is_empty() && !remove_dependencies {
            return Err(CursedError::General(format!(
                "Cannot uninstall {}: still required by {:?}", 
                package_name, dependents
            )));
        }

        // Remove the package
        match self.remove_package_files(&installed_info).await {
            Ok(_) => {
                self.installed_packages.remove(package_name);
                removed_packages.push(package_name.to_string());
                tracing::info!("Removed package: {}", package_name);
            }
            Err(e) => {
                failed_removals.push((package_name.to_string(), e));
            }
        }

        // Remove dependencies if requested
        if remove_dependencies {
            // Collect dependencies to remove to avoid recursion issues
            let deps_to_remove: Vec<String> = installed_info.dependencies.iter()
                .filter(|dep_name| !self.is_package_needed(dep_name, &[package_name.to_string()]))
                .cloned()
                .collect();
                
            for dep_name in deps_to_remove {
                // For now, just mark as orphaned to avoid recursion
                // In a real implementation, we'd use a proper topological sort
                orphaned_dependencies.push(dep_name);
            }
        }

        // Update installed packages database
        self.save_installed_packages()?;

        Ok(UninstallResult {
            removed_packages,
            failed_removals,
            orphaned_dependencies,
        })
    }

    /// List all installed packages
    pub fn list_installed_packages(&self) -> Vec<&InstalledPackageInfo> {
        self.installed_packages.values().collect()
    }

    /// Check if a package is installed
    pub fn is_package_installed(&self, name: &str) -> bool {
        self.installed_packages.contains_key(name)
    }

    /// Get installed package information
    pub fn get_installed_package(&self, name: &str) -> Option<&InstalledPackageInfo> {
        self.installed_packages.get(name)
    }

    /// Create download requests from resolved packages
    fn create_download_requests(&self, packages: &[ResolvedPackage]) -> Result<Vec<PackageDownloadRequest>> {
        let mut requests = Vec::new();

        for package in packages {
            let download_path = self.config.temp_dir
                .join(format!("{}-{}.tar.gz", package.name, package.version));

            let request = PackageDownloadRequest::new(
                package.name.clone(),
                package.version.clone(),
                package.download_url.clone(),
                download_path,
            ).with_checksum(package.checksum.clone());

            requests.push(request);
        }

        Ok(requests)
    }

    /// Get installation path for a package
    fn get_package_install_path(&self, name: &str, version: &Version) -> PathBuf {
        self.config.install_dir
            .join(name)
            .join(version.to_string())
    }

    /// Extract package archive
    async fn extract_package(&self, archive_path: &Path, install_path: &Path) -> Result<Vec<PathBuf>> {
        tracing::debug!("Extracting {} to {:?}", archive_path.display(), install_path);

        // Mock extraction - in real implementation would use tar/zip libraries
        let mock_files = vec![
            install_path.join("lib.csd"),
            install_path.join("README.md"),
            install_path.join("package.toml"),
        ];

        // Create mock files
        for file_path in &mock_files {
            if let Some(parent) = file_path.parent() {
                async_fs::create_dir_all(parent).await?;
            }
            async_fs::write(file_path, "// Mock package file\n").await?;
        }

        tracing::debug!("Extracted {} files", mock_files.len());
        Ok(mock_files)
    }

    /// Verify package installation integrity
    async fn verify_installation(&self, install_path: &Path, files: &[PathBuf]) -> Result<()> {
        tracing::debug!("Verifying installation at {:?}", install_path);

        // Check that all files exist
        for file_path in files {
            if !file_path.exists() {
                return Err(CursedError::General(format!(
                    "Installation verification failed: missing file {:?}", file_path
                )));
            }
        }

        // Additional integrity checks would go here
        tracing::debug!("Installation verification passed");
        Ok(())
    }

    /// Create symlinks for package binaries
    async fn create_package_symlinks(&self, installed_info: &InstalledPackageInfo) -> Result<()> {
        tracing::debug!("Creating symlinks for {}", installed_info.name);

        // In a real implementation, would create symlinks to package binaries
        // in a system bin directory
        
        Ok(())
    }

    /// Backup existing package before upgrade
    async fn backup_package(&self, installed_info: &InstalledPackageInfo) -> Result<()> {
        let backup_path = self.config.temp_dir
            .join("backups")
            .join(format!("{}-{}", installed_info.name, installed_info.version));

        async_fs::create_dir_all(&backup_path).await?;

        tracing::info!("Backing up {} {} to {:?}", 
                      installed_info.name, installed_info.version, backup_path);

        // Copy installation to backup location
        // In real implementation would recursively copy directory
        
        Ok(())
    }

    /// Remove package files
    async fn remove_package_files(&self, installed_info: &InstalledPackageInfo) -> Result<()> {
        tracing::debug!("Removing files for {}", installed_info.name);

        // Remove installed files
        for file_path in &installed_info.installed_files {
            if file_path.exists() {
                if file_path.is_dir() {
                    async_fs::remove_dir_all(file_path).await?;
                } else {
                    async_fs::remove_file(file_path).await?;
                }
                tracing::debug!("Removed {:?}", file_path);
            }
        }

        // Remove installation directory if empty
        if installed_info.install_path.exists() {
            let _ = async_fs::remove_dir(&installed_info.install_path).await;
        }

        Ok(())
    }

    /// Find packages that depend on the given package
    fn find_dependent_packages(&self, package_name: &str) -> Vec<String> {
        self.installed_packages.values()
            .filter(|info| info.dependencies.contains(&package_name.to_string()))
            .map(|info| info.name.clone())
            .collect()
    }

    /// Check if a package is needed by any other installed packages
    fn is_package_needed(&self, package_name: &str, excluding: &[String]) -> bool {
        self.installed_packages.values()
            .filter(|info| !excluding.contains(&info.name))
            .any(|info| info.dependencies.contains(&package_name.to_string()))
    }

    /// Load installed packages from disk
    fn load_installed_packages(&mut self) -> Result<()> {
        let db_path = self.config.install_dir.join("installed_packages.json");
        
        if !db_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&db_path)?;
        self.installed_packages = serde_json::from_str(&content)
            .map_err(|e| CursedError::General(format!("Failed to parse installed packages: {}", e)))?;

        tracing::debug!("Loaded {} installed packages", self.installed_packages.len());
        Ok(())
    }

    /// Save installed packages to disk
    fn save_installed_packages(&self) -> Result<()> {
        let db_path = self.config.install_dir.join("installed_packages.json");
        let content = serde_json::to_string_pretty(&self.installed_packages)
            .map_err(|e| CursedError::General(format!("Failed to serialize installed packages: {}", e)))?;

        fs::write(&db_path, content)?;
        tracing::debug!("Saved {} installed packages", self.installed_packages.len());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package_manager::downloader::DownloadConfig;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_installer_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = InstallConfig {
            install_dir: temp_dir.path().join("install"),
            temp_dir: temp_dir.path().join("temp"),
            ..Default::default()
        };

        let downloader = PackageDownloader::new(DownloadConfig::default()).unwrap();
        let installer = PackageInstaller::new(config, downloader);
        
        assert!(installer.is_ok());
    }

    #[test]
    fn test_installed_package_info() {
        let info = InstalledPackageInfo {
            name: "test-package".to_string(),
            version: Version::new(1, 0, 0),
            install_path: PathBuf::from("/opt/packages/test-package/1.0.0"),
            installed_files: vec![],
            dependencies: vec![],
            install_time: std::time::SystemTime::now(),
            checksum: "sha256:test".to_string(),
            source_url: "https://example.com/test.tar.gz".to_string(),
        };

        assert_eq!(info.name, "test-package");
        assert_eq!(info.version, Version::new(1, 0, 0));
    }
}
