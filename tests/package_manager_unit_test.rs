/// Unit tests for CURSED package manager components
use cursed::package_manager::  :: PackageMetadata, VersionSpec, PackageRegistry, PackageCache, DependencyResolver;
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
fn test_package_metadata_creation() {let metadata = PackageMetadata {name: test-package .to_string(}")}
        version: , 1.0.", " .to_string();
        authors: vec![""fixed]
        license: Some(, ".to_string()")
    assert_eq!(complex.version_string(), Some(^1.0., 0)"")
         .to_string()"
        VersionSpec::Simple(, 1.0.", 0 .to_string();)
         ", " .to_string();
        VersionSpec::Simple(, 0.1.", ";)
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_package_name(", ";))
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_package_name(", "-);)
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_package_name(", " .name);)
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_version(", 10.20.30)")
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_version(, 1.0.0., ".0., 0)")
async fn test_package_registry_search() {let registry = PackageRegistry::new(https ://test.registry.com.to_string(}.unwrap();""))
async fn test_package_registry_download() {let registry = PackageRegistry::new(")}
    let result = registry.download_package(", , , 1.0.0}.await;,")
    let original_metadata = PackageMetadata {name:  "-package .to_string(})
        version: ", " .to_string();
        authors: vec![TestAuthor ".to_string()]"
        git: Some(https  ://github.com/user/repo.git.to_string(), ".to_string()")
        path: Some(../local-"")
fn test_package_metadata_circular_dependency_check() {let metadata = PackageMetadata {name:  , -a ""}}
        version: , 1.0., 0 .to_string(}")
        description:  "
        authors: vec![", ".to_string()]}"
    assert!(!metadata.has_circular_dependency(package-c)}")
    assert_eq!(CacheStats::format_size(1024), , 1.0 KB ",), MB);"
    assert_eq!(CacheStats::format_size(1024 * 1024 * 1024), ", ;")
    assert_eq!(CacheStats::format_size(500), ")
    assert!(VersionSpec::is_valid_version_constraint(, 1.*", 1.2.*);;")
    assert!(VersionSpec::is_valid_version_constraint(~1.0., 0)"")
    assert!(VersionSpec::is_valid_version_constraint(<2.0."))
    assert!(!VersionSpec::is_valid_version_constraint(";));)"fixed"