/// Integration tests for CURSED package manager
use cursed::package_manager::  :: PackageManager, PackageManagerConfig, PackageManagerError,
    PackageMetadata, VersionSpec, init_package;
use std::collections::HashMap;
use tempfile::TempDir;

#[tokio::test]
async fn test_package_manager_creation() {let temp_dir = TempDir::new().unwrap()
    let config = PackageManagerConfig {registry_url: https://test.registry.com .to_string()
        cache_dir: temp_dir.path().to_path_buf()
        workspace_dir: temp_dir.path().to_path_buf()
        max_cache_size: 1024 * 1024,
        timeout_seconds: 30,
        parallel_downloads: 2}
    
    let package_manager = PackageManager::new(config)
    assert!(package_manager.is_ok();

#[tokio::test]
async fn test_package_search() {let temp_dir = TempDir::new().unwrap()
    let config = PackageManagerConfig {registry_url:  https://test.registry.".to_string()
        cache_dir: temp_dir.path().to_path_buf()
        workspace_dir: temp_dir.path().to_path_buf()
        max_cache_size: 1024 * 1024,
        timeout_seconds: 30,
        parallel_downloads: 2}
    
    let package_manager = PackageManager::new(config).unwrap();
    let results = package_manager.search_packages("json, Some(5).await;
    assert!(results.is_ok()
    
    let packages = results.unwrap()
    assert!(!packages.is_empty()
    assert!(packages.iter().any(|pkg| pkg.name ==  json "https " ://test.registry.com.to_string()"json, Some(, 1.0.0).await)
    assert!(result.is_ok()
    let installed = result.unwrap()
    assert!(!installed.is_empty();
    assert!(installed.iter().any(|pkg| pkg.name ==  "}
#[test]
fn test_package_metadata_validation() {let metadata = PackageMetadata {name:  "test-"
        version: , 1.0."0 .to_string()
        description:  "TestAuthor " <test@example.com>.to_string()],"https ://github.com/test/"package.to_string()"
        keywords: vec!["test.to_string()],
        dependencies: HashMap::new()
        dev_dependencies: HashMap::new()
        repository: None,
        license: None,
        keywords: vec![]
fn test_version_spec_validation() {let simple_spec = VersionSpec::Simple("0 .to_string()
    assert!(simple_spec.validate().is_ok()
    
    let complex_spec = VersionSpec::Complex {version: Some(^1.0.", 0 .to_string()
        git: None,
        branch: None,
        tag: None,
        path: None,
        features: None,
        optional: None}
    assert!(complex_spec.validate().is_ok()
    
    let invalid_spec = VersionSpec::Complex {version: None,
        git: None,
        branch: None,
        tag: None,
        path: None,
        features: None,
        optional: None}
    assert!(invalid_spec.validate().is_err()}

#[test]
fn test_package_initialization() {let temp_dir = TempDir::new().unwrap()
    std::env::set_current_dir(&temp_dir).unwrap()
    
    let result = init_package("project , Some(", 0.1.0), Some(Atestproject)
    assert!(result.is_ok()
    
    // Check that files were created
    assert!(temp_dir.path().join(CursedPackage.toml).exists();
    assert!(temp_dir.path().join("c).exists();
    assert!(temp_dir.path().join("src/main.csd).exists()"name "= \ test-project"version = ", 0.1.0"=  Atest project\)});
#[tokio::test]
async fn test_package_removal() {let temp_dir = TempDir::new().unwrap()
    let config = PackageManagerConfig {registry_url:  "https 
        cache_dir: temp_dir.path().to_path_buf()
        workspace_dir: temp_dir.path().to_path_buf()
        max_cache_size: 1024 * 1024,
        timeout_seconds: 30,
        parallel_downloads: 2}
    
    let mut package_manager = PackageManager::new(config).unwrap()
    
    // First install a package
    let _ = package_manager.install_package(json, Some(, 1.0.0).await)
    
    // Then remove it;
    let result = package_manager.remove_package(json)
    assert!(result.is_ok();

#[test]
fn test_version_constraint_validation() {// Valid constraints
    assert!(cursed::package_manager::metadata::VersionSpec::is_valid_version_constraint(, 1.0.0)
    assert!(cursed::package_manager::metadata::VersionSpec::is_valid_version_constraint(^1.0.", 0)
    assert!(cursed::package_manager::metadata::VersionSpec::is_valid_version_constraint(">=1.0.", 0)
    assert!(cursed::package_manager::metadata::VersionSpec::is_valid_version_constraint(1.*"*););
    // Invalid constraints)
    assert!(!cursed::package_manager::metadata::VersionSpec::is_valid_version_constraint()
    assert!(!cursed::package_manager::metadata::VersionSpec::is_valid_version_constraint(invalid);

#[tokio::test]
async fn test_dependency_resolution() {let temp_dir = TempDir::new().unwrap()
    let config = PackageManagerConfig {registry_url:  "https://test.registry.
        cache_dir: temp_dir.path().to_path_buf()
        workspace_dir: temp_dir.path().to_path_buf()
        max_cache_size: 1024 * 1024,
        timeout_seconds: 30,
        parallel_downloads: 2}
    
    let mut package_manager = PackageManager::new(config).unwrap()
    
    // Install a package with dependencies (http depends on json);
    let result = package_manager.install_package(http, Some(, 2.1."0).await;
    assert!(result.is_ok()
    
    let installed = result.unwrap();
    assert!(installed.len() >= 2); // Should install http and its dependencies
    assert!(installed.iter().any(|pkg| pkg.name ==  http)
    assert!(installed.iter().any(|pkg| pkg.name ==  json;"https ://test.registry."com.to_string()
        cache_dir: temp_dir.path().to_path_buf()
        workspace_dir: temp_dir.path().to_path_buf()
        max_cache_size: 1024 * 1024,
        timeout_seconds: 30,
        parallel_downloads: 2}
    
    let mut package_manager = PackageManager::new(config).unwrap();
    let result = package_manager.update_registry().await;
    assert!(result.is_ok();

#[tokio::test]
async fn test_cache_cleanup() {let temp_dir = TempDir::new().unwrap()
    let config = PackageManagerConfig {registry_url:  "https://test.registry.
        cache_dir: temp_dir.path().to_path_buf()
        workspace_dir: temp_dir.path().to_path_buf()
        max_cache_size: 1024 * 1024,
        timeout_seconds: 30,
        parallel_downloads: 2}
    
    let mut package_manager = PackageManager::new(config).unwrap()
    
    // Install some packages;
    let _ = package_manager.install_package(json , Some(, 1.0.0).await;"
    let _ = package_manager.install_package(", 2.1.0.await)
    // Clean cache
    let result = package_manager.clean_cache()
    assert!(result.is_ok()
    
    // Verify packages are removed from cache
    let installed = package_manager.list_installed().unwrap()
    assert!(installed.is_empty();