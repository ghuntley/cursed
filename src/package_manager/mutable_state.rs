//! Simplified mutable state management for package manager
//!
//! This module provides safe mutable state handling using the borrowing system
//! to prevent data races and ensure memory safety in package operations.

use crate::error_types::{Error, Result as CursedResult};
use crate::runtime::borrowing::{get_global_borrow_checker, BorrowMode, ValueId};
use crate::package_manager::{PackageMetadata, InstalledPackageInfo};
use crate::package_manager::version::Version;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;

/// Tracked package with borrow checking
#[derive(Debug, Clone)]
pub struct TrackedPackage {
    package: InstalledPackageInfo,
    borrow_id: ValueId,
}

impl TrackedPackage {
    pub fn new(package: InstalledPackageInfo) -> Self {
        let checker = get_global_borrow_checker();
        let borrow_id = checker.new_value_id();
        Self { package, borrow_id }
    }
    
    pub fn get_package(&self) -> &InstalledPackageInfo {
        &self.package
    }
    
    pub fn get_package_mut(&mut self) -> CursedResult<&mut InstalledPackageInfo> {
        let checker = get_global_borrow_checker();
        if !checker.can_access(self.borrow_id, BorrowMode::Mutable) {
            return Err(Error::Runtime("Cannot get mutable access: conflicting borrows".to_string()));
        }
        Ok(&mut self.package)
    }
    
    pub fn borrow_id(&self) -> ValueId {
        self.borrow_id
    }
}

/// Thread-safe package manager state with borrowing integration
#[derive(Debug)]
pub struct PackageManagerState {
    /// Installed packages with borrow tracking
    installed_packages: Arc<RwLock<HashMap<String, TrackedPackage>>>,
    /// Package metadata cache
    metadata_cache: Arc<RwLock<HashMap<String, PackageMetadata>>>,
    /// Download cache directory
    cache_dir: PathBuf,
    /// Package registry URLs
    registries: Vec<String>,
}

impl PackageManagerState {
    /// Create new package manager state
    pub fn new(cache_dir: PathBuf) -> CursedResult<Self> {
        Ok(Self {
            installed_packages: Arc::new(RwLock::new(HashMap::new())),
            metadata_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_dir,
            registries: vec![
                "https://registry.cursed.dev".to_string(),
                "https://packages.cursed.dev".to_string(),
            ],
        })
    }

    /// Add an installed package with borrow tracking
    pub fn add_installed_package(&self, package: InstalledPackageInfo) -> CursedResult<()> {
        let package_name = package.name.clone();
        let tracked_package = TrackedPackage::new(package);
        
        let mut packages = self.installed_packages.write()
            .map_err(|_| Error::Runtime("Failed to acquire write lock on installed packages".to_string()))?;
        
        packages.insert(package_name, tracked_package);
        Ok(())
    }

    /// Get an installed package
    pub fn get_installed_package(&self, name: &str) -> CursedResult<Option<InstalledPackageInfo>> {
        let packages = self.installed_packages.read()
            .map_err(|_| Error::Runtime("Failed to acquire read lock on installed packages".to_string()))?;
        
        if let Some(tracked) = packages.get(name) {
            // Check borrow access
            let checker = get_global_borrow_checker();
            if !checker.can_access(tracked.borrow_id(), BorrowMode::Shared) {
                return Err(Error::Runtime("Cannot get shared access: conflicting borrows".to_string()));
            }
            Ok(Some(tracked.get_package().clone()))
        } else {
            Ok(None)
        }
    }

    /// Update an installed package with proper borrow checking
    pub fn update_installed_package<F>(&self, name: &str, updater: F) -> CursedResult<()>
    where
        F: FnOnce(&mut InstalledPackageInfo) -> CursedResult<()>,
    {
        let mut packages = self.installed_packages.write()
            .map_err(|_| Error::Runtime("Failed to acquire write lock on installed packages".to_string()))?;
        
        if let Some(tracked) = packages.get_mut(name) {
            let package = tracked.get_package_mut()?;
            updater(package)?;
        }
        
        Ok(())
    }

    /// Remove an installed package
    pub fn remove_installed_package(&self, name: &str) -> CursedResult<Option<InstalledPackageInfo>> {
        let mut packages = self.installed_packages.write()
            .map_err(|_| Error::Runtime("Failed to acquire write lock on installed packages".to_string()))?;
        
        if let Some(tracked) = packages.remove(name) {
            Ok(Some(tracked.package))
        } else {
            Ok(None)
        }
    }

    /// Cache package metadata
    pub fn cache_metadata(&self, metadata: PackageMetadata) -> CursedResult<()> {
        let package_name = metadata.name.clone();
        
        let mut cache = self.metadata_cache.write()
            .map_err(|_| Error::Runtime("Failed to acquire write lock on metadata cache".to_string()))?;
        
        cache.insert(package_name, metadata);
        Ok(())
    }

    /// Get cached metadata
    pub fn get_cached_metadata(&self, name: &str) -> CursedResult<Option<PackageMetadata>> {
        let cache = self.metadata_cache.read()
            .map_err(|_| Error::Runtime("Failed to acquire read lock on metadata cache".to_string()))?;
        
        Ok(cache.get(name).cloned())
    }

    /// List all installed packages
    pub fn list_installed_packages(&self) -> CursedResult<Vec<String>> {
        let packages = self.installed_packages.read()
            .map_err(|_| Error::Runtime("Failed to acquire read lock on installed packages".to_string()))?;
        
        Ok(packages.keys().cloned().collect())
    }

    /// Check if a package is installed
    pub fn is_package_installed(&self, name: &str) -> CursedResult<bool> {
        let packages = self.installed_packages.read()
            .map_err(|_| Error::Runtime("Failed to acquire read lock on installed packages".to_string()))?;
        
        Ok(packages.contains_key(name))
    }

    /// Get cache directory
    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    /// Get registry URLs
    pub fn registries(&self) -> &[String] {
        &self.registries
    }

    /// Add a registry URL
    pub fn add_registry(&mut self, url: String) {
        self.registries.push(url);
    }

    /// Clean up expired references
    pub fn cleanup(&self) -> CursedResult<()> {
        let checker = get_global_borrow_checker();
        checker.cleanup_expired_references();
        Ok(())
    }
}

/// Safe package operation wrapper
pub struct PackageOperation<'a> {
    state: &'a PackageManagerState,
    operation_name: String,
}

impl<'a> PackageOperation<'a> {
    /// Create a new package operation
    pub fn new(state: &'a PackageManagerState, operation_name: String) -> Self {
        Self {
            state,
            operation_name,
        }
    }

    /// Execute a safe package installation
    pub fn install_package(&self, package: InstalledPackageInfo) -> CursedResult<()> {
        tracing::info!("Starting package installation: {}", package.name);
        
        if self.state.is_package_installed(&package.name)? {
            return Err(Error::Runtime(format!(
                "Package {} is already installed", package.name
            )));
        }
        
        self.state.add_installed_package(package)?;
        tracing::info!("Package installation completed successfully");
        Ok(())
    }

    /// Execute a safe package update
    pub fn update_package(&self, name: &str, new_version: Version) -> CursedResult<()> {
        tracing::info!("Starting package update: {} -> {}", name, new_version);
        
        self.state.update_installed_package(name, |package| {
            package.version = new_version.clone();
            Ok(())
        })?;
        
        tracing::info!("Package update completed successfully");
        Ok(())
    }

    /// Execute a safe package removal
    pub fn remove_package(&self, name: &str) -> CursedResult<()> {
        tracing::info!("Starting package removal: {}", name);
        
        if let Some(_removed_package) = self.state.remove_installed_package(name)? {
            tracing::info!("Package removal completed successfully");
            Ok(())
        } else {
            Err(Error::Runtime(format!("Package {} is not installed", name)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::time::SystemTime;

    fn create_test_package() -> InstalledPackageInfo {
        InstalledPackageInfo {
            name: "test-package".to_string(),
            version: Version::parse("1.0.0").unwrap(),
            install_path: PathBuf::from("/tmp/test"),
            installed_files: vec![],
            dependencies: vec![],
            install_time: SystemTime::now(),
            checksum: "abc123".to_string(),
            source_url: "https://test.com".to_string(),
        }
    }

    #[test]
    fn test_package_manager_state() -> CursedResult<()> {
        let temp_dir = tempdir().unwrap();
        let state = PackageManagerState::new(temp_dir.path().to_path_buf())?;
        
        let package = create_test_package();
        state.add_installed_package(package)?;
        
        let retrieved = state.get_installed_package("test-package")?;
        assert!(retrieved.is_some());
        
        let packages = state.list_installed_packages()?;
        assert_eq!(packages.len(), 1);
        assert!(packages.contains(&"test-package".to_string()));
        
        Ok(())
    }

    #[test]
    fn test_borrow_checking() -> CursedResult<()> {
        let temp_dir = tempdir().unwrap();
        let state = PackageManagerState::new(temp_dir.path().to_path_buf())?;
        
        let package = create_test_package();
        state.add_installed_package(package)?;
        
        // Test shared access
        let _pkg1 = state.get_installed_package("test-package")?;
        let _pkg2 = state.get_installed_package("test-package")?;
        
        // Both should succeed
        assert!(_pkg1.is_some());
        assert!(_pkg2.is_some());
        
        Ok(())
    }
}
