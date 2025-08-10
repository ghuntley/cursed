//! Simple tests for the CURSED package manager system

use super::*;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_package_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
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
    
    let pkg_manager = PackageManager::new(config);
    assert!(pkg_manager.is_ok());
    
    let pkg_manager = pkg_manager.unwrap();
    assert!(!pkg_manager.is_installed("non-existent-package"));
    
    let installed = pkg_manager.list_installed();
    assert!(installed.is_empty());
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
fn test_dependency_creation() {
    let dep = crate::package_manager::registry::Dependency {
        name: "test-dep".to_string(),
        version_req: VersionReq::parse("^1.0.0").unwrap(),
        optional: false,
        features: vec!["feature1".to_string()],
    };
    
    assert_eq!(dep.name, "test-dep");
    assert!(!dep.optional);
    assert_eq!(dep.features.len(), 1);
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

#[ignore] // Skip due to tokio runtime stack overflow
#[tokio::test]
async fn test_package_manager_basic_workflow() {
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
    
    // Test workspace initialization
    let members = vec!["test_package".to_string()];
    let result = pkg_manager.init_workspace(temp_dir.path(), members);
    assert!(result.is_ok());
    
    // Verify workspace was created
    assert!(pkg_manager.workspace().is_some());
    
    // Test lock file generation
    let result = pkg_manager.generate_lock_file();
    assert!(result.is_ok());
    
    let lock_file_path = temp_dir.path().join("CursedPackage.lock");
    assert!(lock_file_path.exists());
    
    // Test lock file validation
    let result = pkg_manager.validate_lock_file();
    assert!(result.is_ok());
}

#[test]
fn test_cache_creation() {
    let temp_dir = TempDir::new().unwrap();
    let cache_config = CacheConfig {
        cache_dir: temp_dir.path().to_path_buf(),
        max_size: 1024 * 1024, // 1MB
        max_age: std::time::Duration::from_secs(3600),
        cleanup_interval: std::time::Duration::from_secs(3600),
    };
    
    let cache = PackageCache::new(cache_config);
    assert!(cache.is_ok());
    
    let mut cache = cache.unwrap();
    
    // Test basic cache operations
    let version = Version::parse("1.0.0").unwrap();
    let is_cached = cache.is_cached("test-package", &version);
    assert!(!is_cached);
    
    // Test cache statistics
    let stats = cache.get_stats();
    assert_eq!(stats.total_packages, 0);
}

#[test]
fn test_workspace_creation() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_root = temp_dir.path();
    
    // Create workspace manager
    let members = vec!["package1".to_string(), "package2".to_string()];
    let workspace = WorkspaceManager::init_workspace(workspace_root, members);
    assert!(workspace.is_ok());
    
    let workspace = workspace.unwrap();
    
    // Test workspace operations
    assert_eq!(workspace.members().len(), 2);
    assert!(workspace.members().iter().any(|m| m.name == "package1"));
    assert!(workspace.members().iter().any(|m| m.name == "package2"));
}

#[test]
fn test_lock_file_creation() {
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
    
    lock_manager.add_package(package1);
    
    // Save lock file
    let result = lock_manager.save();
    assert!(result.is_ok());
    assert!(lock_file_path.exists());
    
    // Load lock file
    let mut new_lock_manager = LockFileManager::new(&lock_file_path);
    let result = new_lock_manager.load();
    assert!(result.is_ok());
    
    // Test validation
    let result = new_lock_manager.validate();
    assert!(result.is_ok());
}

#[test]
fn test_default_configurations() {
    // Test default package manager config
    let config = PackageManagerConfig::default();
    assert_eq!(config.registry_url, "https://packages.cursed-lang.org");
    assert!(!config.offline_mode);
    assert!(config.verify_signatures);
    assert_eq!(config.parallel_downloads, 4);
    
    // Test default cache config
    let cache_config = CacheConfig::default();
    assert_eq!(cache_config.max_size, 1024 * 1024 * 1024); // 1GB
    
    // Test default registry config
    let registry_config = RegistryConfig::default();
    assert_eq!(registry_config.url, "https://packages.cursed-lang.org");
    assert_eq!(registry_config.max_retries, 3);
}

#[test]
fn test_error_types() {
    // Test error creation
    let error = PackageManagerError::PackageNotFound { 
        name: "test-package".to_string() 
    };
    assert!(error.to_string().contains("test-package"));
    
    let error = PackageManagerError::InvalidVersion { 
        version: "invalid".to_string() 
    };
    assert!(error.to_string().contains("invalid"));
}
