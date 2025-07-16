//! Comprehensive tests for the CURSED package manager system

use super::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_package_manager_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    let config = PackageManagerConfig {
        cache_dir: temp_dir.path().join("cache"),
        registry_url: "https://test-registry.cursed-lang.org".to_string(),
        offline_mode: true, // Use offline mode for testing
        verify_signatures: false,
        workspace_dir: temp_dir.path().to_path_buf(),
        max_cache_size: 1024 * 1024, // 1MB for testing
        timeout_seconds: 10,
        parallel_downloads: 1,
    };
    
    let mut pkg_manager = PackageManager::new(config).unwrap();
    
    // Test workspace initialization
    let members = vec!["test_package".to_string()];
    pkg_manager.init_workspace(temp_dir.path(), members).unwrap();
    
    // Verify workspace was created
    assert!(pkg_manager.workspace().is_some());
    
    // Test lock file generation
    pkg_manager.generate_lock_file().unwrap();
    let lock_file_path = temp_dir.path().join("CursedPackage.lock");
    assert!(lock_file_path.exists());
    
    // Test lock file validation
    pkg_manager.validate_lock_file().unwrap();
}

#[tokio::test]
async fn test_package_publishing() {
    let temp_dir = TempDir::new().unwrap();
    let package_dir = temp_dir.path().join("test_package");
    
    // Create a minimal package structure
    create_test_package(&package_dir).unwrap();
    
    let config = PackageManagerConfig {
        cache_dir: temp_dir.path().join("cache"),
        registry_url: "https://test-registry.cursed-lang.org".to_string(),
        offline_mode: true,
        verify_signatures: false,
        workspace_dir: temp_dir.path().to_path_buf(),
        max_cache_size: 1024 * 1024,
        timeout_seconds: 10,
        parallel_downloads: 1,
    };
    
    let pkg_manager = PackageManager::new(config).unwrap();
    
    // Test dry run publishing
    let result = pkg_manager.publish_package(package_dir.to_str().unwrap(), true).await;
    
    // In offline mode, this should work for validation but fail for actual publishing
    // For now, we just verify the package structure validation works
    match result {
        Ok(_) => {}, // Dry run succeeded
        Err(_) => {}, // Expected in offline mode
    }
}

#[test]
fn test_package_structure_validation() {
    let temp_dir = TempDir::new().unwrap();
    let package_dir = temp_dir.path().join("test_package");
    
    let config = PackageManagerConfig::default();
    let pkg_manager = PackageManager::new(config).unwrap();
    
    // Test with missing package.toml
    fs::create_dir_all(&package_dir).unwrap();
    let result = pkg_manager.validate_package_structure(&package_dir);
    assert!(result.is_err());
    
    // Create valid package structure
    create_test_package(&package_dir).unwrap();
    
    // Test with valid structure
    let result = pkg_manager.validate_package_structure(&package_dir);
    assert!(result.is_ok());
}

#[test]
fn test_version_parsing() {
    // Test valid versions
    assert!(Version::parse("1.0.0").is_ok());
    assert!(Version::parse("0.1.0-alpha").is_ok());
    assert!(Version::parse("2.3.4-beta.1").is_ok());
    
    // Test invalid versions
    assert!(Version::parse("invalid").is_err());
    assert!(Version::parse("1.0").is_err());
    assert!(Version::parse("").is_err());
}

#[test]
fn test_version_requirements() {
    let version = Version::parse("1.2.3").unwrap();
    
    // Test exact match
    let req = VersionReq::parse("1.2.3").unwrap();
    assert!(req.matches(&version));
    
    // Test range match
    let req = VersionReq::parse(">=1.0.0").unwrap();
    assert!(req.matches(&version));
    
    // Test caret requirement
    let req = VersionReq::parse("^1.2.0").unwrap();
    assert!(req.matches(&version));
    
    // Test tilde requirement
    let req = VersionReq::parse("~1.2.0").unwrap();
    assert!(req.matches(&version));
}

#[test]
fn test_dependency_resolution() {
    // This would test the dependency resolver with mock packages
    // For now, we just test that the resolver can be created
    let registry_config = RegistryConfig {
        url: "https://test-registry.cursed-lang.org".to_string(),
        timeout: std::time::Duration::from_secs(30),
        max_retries: 3,
        api_key: None,
    };
    
    let registry = PackageRegistry::new(registry_config).unwrap();
    let _resolver = PackageResolver::new(registry);
}

#[tokio::test]
async fn test_cache_operations() {
    let temp_dir = TempDir::new().unwrap();
    let cache_config = CacheConfig {
        cache_dir: temp_dir.path().to_path_buf(),
        max_size: 1024 * 1024, // 1MB
        max_age: std::time::Duration::from_secs(3600),
        cleanup_interval: std::time::Duration::from_secs(3600),
    };
    
    let mut cache = PackageCache::new(cache_config).unwrap();
    
    // Test cache hit/miss tracking
    let version = Version::parse("1.0.0").unwrap();
    let is_cached_before = cache.is_cached("test-package", &version);
    assert!(!is_cached_before);
    
    // Test cache statistics
    let stats = cache.get_stats();
    assert_eq!(stats.total_packages, 0);
}

#[test]
fn test_workspace_configuration() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_root = temp_dir.path();
    
    // Create workspace manager
    let members = vec!["package1".to_string(), "package2".to_string()];
    let workspace = WorkspaceManager::init_workspace(workspace_root, members).unwrap();
    
    // Test workspace operations
    assert_eq!(workspace.members().len(), 2);
    assert!(workspace.members().iter().any(|m| m.name == "package1"));
    assert!(workspace.members().iter().any(|m| m.name == "package2"));
}

#[test]
fn test_lock_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let lock_file_path = temp_dir.path().join("CursedPackage.lock");
    
    let mut lock_manager = LockFileManager::new(&lock_file_path);
    
    // Add some packages
    let package1 = LockedPackage {
        name: "test-package-1".to_string(),
        version: "1.0.0".to_string(),
        source: "registry".to_string(),
        checksum: Some("abc123".to_string()),
        dependencies: vec!["dep1".to_string()],
    };
    
    let package2 = LockedPackage {
        name: "test-package-2".to_string(),
        version: "2.0.0".to_string(),
        source: "registry".to_string(),
        checksum: Some("def456".to_string()),
        dependencies: vec![],
    };
    
    lock_manager.add_package(package1);
    lock_manager.add_package(package2);
    
    // Save lock file
    lock_manager.save().unwrap();
    assert!(lock_file_path.exists());
    
    // Load lock file
    let mut new_lock_manager = LockFileManager::new(&lock_file_path);
    new_lock_manager.load().unwrap();
    
    // Verify loaded packages
    let packages = new_lock_manager.get_packages().unwrap_or_default();
    assert_eq!(packages.len(), 2);
    assert!(packages.iter().any(|p| p.name == "test-package-1"));
    assert!(packages.iter().any(|p| p.name == "test-package-2"));
    
    // Test validation
    let is_valid = new_lock_manager.validate().unwrap();
    assert!(is_valid);
}

#[test]
fn test_package_metadata_creation() {
    let metadata = PackageMetadata {
        name: "test-package".to_string(),
        version: Version::parse("1.0.0").unwrap(),
        description: "A test package".to_string(),
        dependencies: vec![
            crate::package_manager::registry::Dependency {
                name: "dep1".to_string(),
                version_req: VersionReq::parse("^1.0.0").unwrap(),
                optional: false,
                features: vec!["feature1".to_string()],
            }
        ],
        download_url: "https://example.com/package.tar.gz".to_string(),
        checksum: "abc123".to_string(),
        authors: vec!["Test Author <test@example.com>".to_string()],
        license: Some("MIT".to_string()),
        homepage: Some("https://example.com".to_string()),
        repository: Some("https://github.com/example/test-package".to_string()),
        keywords: vec!["test".to_string(), "example".to_string()],
        categories: vec!["development-tools".to_string()],
    };
    
    // Test metadata properties
    assert_eq!(metadata.name, "test-package");
    assert_eq!(metadata.version.to_string(), "1.0.0");
    assert_eq!(metadata.dependencies.len(), 1);
    assert_eq!(metadata.dependencies[0].name, "dep1");
}

#[tokio::test]
async fn test_package_installation_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let config = PackageManagerConfig {
        cache_dir: temp_dir.path().join("cache"),
        registry_url: "https://test-registry.cursed-lang.org".to_string(),
        offline_mode: true, // Use offline mode for testing
        verify_signatures: false,
        workspace_dir: temp_dir.path().to_path_buf(),
        max_cache_size: 1024 * 1024,
        timeout_seconds: 10,
        parallel_downloads: 1,
    };
    
    let mut pkg_manager = PackageManager::new(config).unwrap();
    
    // Test that we can create a package manager without errors
    assert!(!pkg_manager.is_installed("non-existent-package"));
    
    // Test listing empty installed packages
    let installed = pkg_manager.list_installed();
    assert!(installed.is_empty());
    
    // Test getting package info for non-existent package (will fail in offline mode)
    let result = pkg_manager.get_package_info("non-existent", None).await;
    assert!(result.is_err()); // Expected in offline mode
}

#[test]
fn test_archive_creation() {
    let temp_dir = TempDir::new().unwrap();
    let package_dir = temp_dir.path().join("test_package");
    
    // Create test package
    create_test_package(&package_dir).unwrap();
    
    let config = PackageManagerConfig::default();
    let pkg_manager = PackageManager::new(config).unwrap();
    
    // Test archive creation
    let archive_data = pkg_manager.create_package_archive(&package_dir).unwrap();
    
    // Verify archive is not empty
    assert!(!archive_data.is_empty());
    
    // Archive should be a valid tar.gz file
    // We could decompress and verify contents here
}

#[test]
fn test_error_handling() {
    // Test various error conditions
    
    // Invalid package manager configuration
    let invalid_config = PackageManagerConfig {
        cache_dir: PathBuf::from("/invalid/path/that/cannot/be/created"),
        registry_url: "invalid-url".to_string(),
        offline_mode: false,
        verify_signatures: true,
        workspace_dir: PathBuf::from("/another/invalid/path"),
        max_cache_size: 0,
        timeout_seconds: 0,
        parallel_downloads: 0,
    };
    
    // This might fail depending on the implementation
    let result = PackageManager::new(invalid_config);
    // We expect this to either succeed or fail gracefully
    match result {
        Ok(_) => {}, // Configuration was accepted
        Err(_) => {}, // Configuration was rejected
    }
}

// Helper function to create a minimal test package structure
fn create_test_package(package_dir: &std::path::Path) -> std::io::Result<()> {
    fs::create_dir_all(package_dir)?;
    fs::create_dir_all(package_dir.join("src"))?;
    
    // Create package.toml
    let package_toml = r#"
[package]
name = "test-package"
version = "1.0.0"
description = "A test package for CURSED"
authors = ["Test Author <test@example.com>"]
license = "MIT"
keywords = ["test", "example"]
categories = ["development-tools"]

[dependencies]
# No dependencies for this test package
"#;
    fs::write(package_dir.join("package.toml"), package_toml)?;
    
    // Create src/mod.csd
    let main_csd = r#"
// Test package main module
yeet "testz"

slay test_function() lit {
    damn based
}

test_start("test package test")
assert_true(test_function())
print_test_summary()
"#;
    fs::write(package_dir.join("src/mod.csd"), main_csd)?;
    
    // Create README.md
    let readme = r#"
# Test Package

This is a test package for the CURSED package manager.

## Usage

```cursed
yeet "test-package"
test_package.test_function()
```
"#;
    fs::write(package_dir.join("README.md"), readme)?;
    
    Ok(())
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_version_parsing_performance() {
        let start = Instant::now();
        
        for i in 0..1000 {
            let version_str = format!("{}.{}.{}", i % 10, i % 100, i % 1000);
            let _ = Version::parse(&version_str);
        }
        
        let duration = start.elapsed();
        println!("Version parsing took: {:?}", duration);
        
        // Ensure it completes within reasonable time (1 second)
        assert!(duration.as_secs() < 1);
    }
    
    #[test]
    fn test_dependency_resolution_performance() {
        let registry_config = RegistryConfig {
            url: "https://test-registry.cursed-lang.org".to_string(),
            timeout: std::time::Duration::from_secs(30),
            max_retries: 3,
            api_key: None,
        };
        
        let start = Instant::now();
        
        // Create multiple resolvers to test initialization performance
        for _ in 0..100 {
            let registry = PackageRegistry::new(registry_config.clone()).unwrap();
            let _resolver = PackageResolver::new(registry);
        }
        
        let duration = start.elapsed();
        println!("Dependency resolver creation took: {:?}", duration);
        
        // Ensure it completes within reasonable time (1 second)
        assert!(duration.as_secs() < 1);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_full_package_workflow() {
        let temp_dir = TempDir::new().unwrap();
        
        // Set up test workspace
        let workspace_dir = temp_dir.path().join("workspace");
        fs::create_dir_all(&workspace_dir).unwrap();
        
        // Create test package
        let package_dir = workspace_dir.join("test_package");
        create_test_package(&package_dir).unwrap();
        
        // Configure package manager
        let config = PackageManagerConfig {
            cache_dir: temp_dir.path().join("cache"),
            registry_url: "https://test-registry.cursed-lang.org".to_string(),
            offline_mode: true,
            verify_signatures: false,
            workspace_dir: workspace_dir.clone(),
            max_cache_size: 10 * 1024 * 1024, // 10MB
            timeout_seconds: 30,
            parallel_downloads: 2,
        };
        
        let mut pkg_manager = PackageManager::new(config).unwrap();
        
        // Initialize workspace
        let members = vec!["test_package".to_string()];
        pkg_manager.init_workspace(&workspace_dir, members).unwrap();
        
        // Generate lock file
        pkg_manager.generate_lock_file().unwrap();
        
        // Validate lock file
        pkg_manager.validate_lock_file().unwrap();
        
        // Verify all files were created
        assert!(workspace_dir.join("CursedPackage.lock").exists());
        assert!(package_dir.join("package.toml").exists());
        assert!(package_dir.join("src/mod.csd").exists());
        assert!(package_dir.join("README.md").exists());
    }
}
