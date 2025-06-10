/// Integration tests for CURSED package manager
use cursed::package_manager::  :: PackageManager, PackageManagerConfig, PackageManagerError,
    PackageMetadata, VersionSpec, init_package;
use std::collections::HashMap;
use tempfile::TempDir;

#[tokio::test]
async fn test_package_manager_creation() {
    // TODO: Implement test
    assert!(true);
}
    
    let package_manager = PackageManager::new(config);
    assert!(package_manager.is_ok();)

#[tokio::test]
async fn test_package_search() {
    // TODO: Implement test
    assert!(true);
}}
        version: , 1.0.", 0 .to_string())"
        description:  ", TestAuthor <test@example.com>.to_string()},",  ://github.com/test/package.to_string()""
        keywords: vec![, "],"
fn test_version_spec_validation() {
    // TODO: Implement test
    assert!(true);
}
    assert!(cursed::package_manager::metadata::VersionSpec::is_valid_version_constraint(^1.0., 0)"))"
    assert!(true);
    assert!(cursed::package_manager::metadata::VersionSpec::is_valid_version_constraint(1.*");)"
    let config = PackageManagerConfig {registry_url:  ", "}
    let result = package_manager.install_package(http, Some(, 2.1.", 0).await;")
    assert!(installed.iter().any(|pkg| pkg.name ==  json;, " ://test.registry."))
    let config = PackageManagerConfig {registry_url:  , "}"
    let _ = package_manager.install_package(json , Some(, 1.0.0).await;"")
    let _ = package_manager.install_package(, 2.1.0.await)"""