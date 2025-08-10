//! Mutable state management for package manager
//!
//! This module provides safe mutable state handling using the borrowing system
//! to prevent data races and ensure memory safety in package operations.

use crate::error_types::{Error, Result as CursedResult};
use crate::runtime::borrowing::{MutableRef, SharedRef, get_global_borrow_checker, BorrowMode};
use crate::package_manager::{PackageMetadata, InstalledPackageInfo};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::path::PathBuf;

/// Thread-safe package manager state with borrowing integration
#[derive(Debug)]
pub struct PackageManagerState {
    /// Installed packages with mutable reference tracking
    installed_packages: Arc<RwLock<HashMap<String, MutableRef<InstalledPackageInfo>>>>,
    /// Package metadata cache with shared reference tracking
    metadata_cache: Arc<RwLock<HashMap<String, SharedRef<PackageMetadata>>>>,
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

    /// Add an installed package with mutable reference tracking
    pub fn add_installed_package(&self, package: InstalledPackage) -> CursedResult<()> {
        let package_name = package.name.clone();
        let location = format!("PackageManagerState::add_installed_package({})", package_name);
        
        // Create mutable reference for the package
        let package_ref = MutableRef::new(package, location)?;
        
        // Get write lock on installed packages
        let mut packages = self.installed_packages.write()
            .map_err(|_| Error::Runtime("Failed to acquire write lock on installed packages".to_string()))?;
        
        packages.insert(package_name, package_ref);
        Ok(())
    }

    /// Get an installed package with borrow checking
    pub fn get_installed_package(&self, name: &str) -> CursedResult<Option<SharedRef<InstalledPackage>>> {
        let packages = self.installed_packages.read()
            .map_err(|_| Error::Runtime("Failed to acquire read lock on installed packages".to_string()))?;
        
        if let Some(package_ref) = packages.get(name) {
            // Convert mutable ref to shared ref for read access
            let package = package_ref.get_mut().clone();
            let location = format!("PackageManagerState::get_installed_package({})", name);
            let shared_ref = SharedRef::new(package, location)?;
            Ok(Some(shared_ref))
        } else {
            Ok(None)
        }
    }

    /// Update an installed package with proper borrow checking
    pub fn update_installed_package<F>(&self, name: &str, updater: F) -> CursedResult<()>
    where
        F: FnOnce(&mut InstalledPackage) -> CursedResult<()>,
    {
        let mut packages = self.installed_packages.write()
            .map_err(|_| Error::Runtime("Failed to acquire write lock on installed packages".to_string()))?;
        
        if let Some(package_ref) = packages.get_mut(name) {
            let location = format!("PackageManagerState::update_installed_package({})", name);
            
            // Check if we can get mutable access
            let checker = get_global_borrow_checker();
            if !checker.can_access(package_ref.value_id(), BorrowMode::Mutable) {
                return Err(Error::Runtime(format!(
                    "Cannot update package {}: conflicting borrows", name
                )));
            }
            
            // Apply the update
            let mut package = package_ref.get_mut();
            updater(&mut package)?;
        }
        
        Ok(())
    }

    /// Remove an installed package with proper cleanup
    pub fn remove_installed_package(&self, name: &str) -> CursedResult<Option<InstalledPackage>> {
        let mut packages = self.installed_packages.write()
            .map_err(|_| Error::Runtime("Failed to acquire write lock on installed packages".to_string()))?;
        
        if let Some(package_ref) = packages.remove(name) {
            let package = package_ref.get_mut().clone();
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }

    /// Add package metadata to cache with shared reference tracking
    pub fn cache_metadata(&self, metadata: PackageMetadata) -> CursedResult<()> {
        let package_name = metadata.name.clone();
        let location = format!("PackageManagerState::cache_metadata({})", package_name);
        
        // Create shared reference for the metadata
        let metadata_ref = SharedRef::new(metadata, location)?;
        
        // Get write lock on metadata cache
        let mut cache = self.metadata_cache.write()
            .map_err(|_| Error::Runtime("Failed to acquire write lock on metadata cache".to_string()))?;
        
        cache.insert(package_name, metadata_ref);
        Ok(())
    }

    /// Get cached metadata with borrow checking
    pub fn get_cached_metadata(&self, name: &str) -> CursedResult<Option<SharedRef<PackageMetadata>>> {
        let cache = self.metadata_cache.read()
            .map_err(|_| Error::Runtime("Failed to acquire read lock on metadata cache".to_string()))?;
        
        Ok(cache.get(name).cloned())
    }

    /// Update dependency graph with proper mutable access control
    pub fn update_dependency_graph<F>(&mut self, updater: F) -> CursedResult<()>
    where
        F: FnOnce(&mut DependencyGraph) -> CursedResult<()>,
    {
        let mut graph = self.dependency_graph.get_mut();
        updater(&mut graph)
    }

    /// Get read-only access to dependency graph
    pub fn with_dependency_graph<F, R>(&self, reader: F) -> CursedResult<R>
    where
        F: FnOnce(&DependencyGraph) -> R,
    {
        let graph = self.dependency_graph.get_mut();
        Ok(reader(&graph))
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

    /// Clean up expired references and perform garbage collection
    pub fn cleanup(&self) -> CursedResult<()> {
        // Clean up expired weak references in borrow checker
        let checker = get_global_borrow_checker();
        checker.cleanup_expired_references();
        
        // Clean up empty entries in caches
        {
            let mut packages = self.installed_packages.write()
                .map_err(|_| Error::Runtime("Failed to acquire write lock for cleanup".to_string()))?;
            
            // Remove packages that have no strong references
            packages.retain(|_, package_ref| {
                // Keep packages that are still being used
                checker.can_access(package_ref.value_id(), BorrowMode::Shared)
            });
        }
        
        {
            let mut cache = self.metadata_cache.write()
                .map_err(|_| Error::Runtime("Failed to acquire write lock for cleanup".to_string()))?;
            
            // Remove metadata that has no strong references
            cache.retain(|_, metadata_ref| {
                // Keep metadata that is still being used
                checker.can_access(metadata_ref.value_id(), BorrowMode::Shared)
            });
        }
        
        Ok(())
    }
}

impl Clone for PackageManagerState {
    fn clone(&self) -> Self {
        // Create a new dependency graph clone
        let graph_clone = self.dependency_graph.get_mut().clone();
        let graph_ref = MutableRef::new(graph_clone, "PackageManagerState::clone".to_string())
            .expect("Failed to create mutable reference during clone");

        Self {
            installed_packages: Arc::clone(&self.installed_packages),
            metadata_cache: Arc::clone(&self.metadata_cache),
            dependency_graph: graph_ref,
            cache_dir: self.cache_dir.clone(),
            registries: self.registries.clone(),
        }
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
    pub fn install_package(&self, package: InstalledPackage) -> CursedResult<()> {
        tracing::info!("Starting package installation: {}", package.name);
        
        // Check if package is already installed
        if self.state.is_package_installed(&package.name)? {
            return Err(Error::Runtime(format!(
                "Package {} is already installed", package.name
            )));
        }
        
        // Add the package with proper borrow tracking
        self.state.add_installed_package(package)?;
        
        tracing::info!("Package installation completed successfully");
        Ok(())
    }

    /// Execute a safe package update
    pub fn update_package(&self, name: &str, new_version: String) -> CursedResult<()> {
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

    #[test]
    fn test_package_manager_state() -> CursedResult<()> {
        let temp_dir = tempdir().unwrap();
        let state = PackageManagerState::new(temp_dir.path().to_path_buf())?;
        
        // Test package installation
        let package = InstalledPackage {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            path: temp_dir.path().join("test-package"),
            dependencies: vec![],
        };
        
        state.add_installed_package(package)?;
        
        // Test package retrieval
        let retrieved = state.get_installed_package("test-package")?;
        assert!(retrieved.is_some());
        
        // Test package listing
        let packages = state.list_installed_packages()?;
        assert_eq!(packages.len(), 1);
        assert!(packages.contains(&"test-package".to_string()));
        
        Ok(())
    }

    #[test]
    fn test_package_operation() -> CursedResult<()> {
        let temp_dir = tempdir().unwrap();
        let state = PackageManagerState::new(temp_dir.path().to_path_buf())?;
        let operation = PackageOperation::new(&state, "test-operation".to_string());
        
        // Test package installation through operation
        let package = InstalledPackage {
            name: "operation-test".to_string(),
            version: "1.0.0".to_string(),
            path: temp_dir.path().join("operation-test"),
            dependencies: vec![],
        };
        
        operation.install_package(package)?;
        
        // Test package update
        operation.update_package("operation-test", "1.1.0".to_string())?;
        
        // Verify update
        let updated = state.get_installed_package("operation-test")?;
        assert!(updated.is_some());
        
        Ok(())
    }

    #[test]
    fn test_borrow_checking() -> CursedResult<()> {
        let temp_dir = tempdir().unwrap();
        let state = PackageManagerState::new(temp_dir.path().to_path_buf())?;
        
        let package = InstalledPackage {
            name: "borrow-test".to_string(),
            version: "1.0.0".to_string(),
            path: temp_dir.path().join("borrow-test"),
            dependencies: vec![],
        };
        
        state.add_installed_package(package)?;
        
        // Test concurrent access
        let _shared1 = state.get_installed_package("borrow-test")?;
        let _shared2 = state.get_installed_package("borrow-test")?;
        
        // Both shared accesses should succeed
        assert!(_shared1.is_some());
        assert!(_shared2.is_some());
        
        Ok(())
    }
}
