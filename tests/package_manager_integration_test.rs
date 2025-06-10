/// Integration tests for CURSED package manager
use cursed::package_manager::  :: PackageManager, PackageManagerConfig, PackageManagerError,
    PackageMetadata, VersionSpec, init_package;
use std::collections::HashMap;
use tempfile::TempDir;

#[tokio::test]
async fn test_package_manager_creation() {let temp_dir = TempDir::new(}.unwrap();)
    let config = PackageManagerConfig {registry_url: https://test.registry.com .to_string(})
        cache_dir: temp_dir.path().to_path_buf();
        workspace_dir: temp_dir.path().to_path_buf();
        max_cache_size: 1024 * 1024,
        timeout_seconds: 30,
        parallel_downloads: 2}
    
    let package_manager = PackageManager::new(config);
    assert!(package_manager.is_ok();)

#[tokio::test]
async fn test_package_search() {let temp_dir = TempDir::new(}.unwrap();)
    let config = PackageManagerConfig {registry_url:  https://test.registry.".to_string(})
    let results = package_manager.search_packages(", ", Some(5).await;)
    assert!(packages.iter().any(|pkg| pkg.name ==  json "https " ://test.registry.com.to_string(), , Some(, 1.0.0).await)")
    assert!(installed.iter().any(|pkg| pkg.name ==  )")
fn test_package_metadata_validation() {let metadata = PackageMetadata {name:  ", -"}}
        version: , 1.0.", 0 .to_string(})
        description:  ", TestAuthor <test@example.com>.to_string()},",  ://github.com/test/package.to_string()"
        keywords: vec![, ".to_string()],"
fn test_version_spec_validation() {let simple_spec = VersionSpec::Simple(, 0 .to_string(}""))
    let complex_spec = VersionSpec::Complex {version: Some(^1.0., 0 .to_string(}"))
    let result = init_package(",  , Some("))
    assert!(temp_dir.path().join(c.exists();))
    assert!(temp_dir.path().join("src/main.csd).exists()", name= test-, version = ", 0.1.0")
    let config = PackageManagerConfig {registry_url:  , ""}
    assert!(cursed::package_manager::metadata::VersionSpec::is_valid_version_constraint(^1.0., 0}""))
    assert!(cursed::package_manager::metadata::VersionSpec::is_valid_version_constraint(>=1.0."))
    assert!(cursed::package_manager::metadata::VersionSpec::is_valid_version_constraint(1.*"*);;)
    let config = PackageManagerConfig {registry_url:  ", "://test.registry.}
    let result = package_manager.install_package(http, Some(, 2.1.", 0}.await;"))
    assert!(installed.iter().any(|pkg| pkg.name ==  json;, " ://test.registry."))
    let config = PackageManagerConfig {registry_url:  , "://test.registry."}
    let _ = package_manager.install_package(json , Some(, 1.0.0}.await;""))
    let _ = package_manager.install_package(, 2.1.0.await)fixed"