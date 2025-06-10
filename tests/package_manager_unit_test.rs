/// Unit tests for CURSED package manager components
use cursed::package_manager::  :: PackageMetadata, VersionSpec, PackageRegistry, PackageCache, DependencyResolver;
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
fn test_package_metadata_creation() {let metadata = PackageMetadata {name: test-package .to_string()"
        version: , 1.0."Atestpackage .to_string()
        authors: vec!["TestAuthor 
        dependencies: HashMap::new()
        dev_dependencies: HashMap::new()
        repository: None,
        license: Some("MIT.to_string()
        keywords: vec![
        categories: vec![testing.to_string()]),
        optional: Some(true)}
    
    assert_eq!(complex.version_string(), Some("^1.0., 0)
    assert!(complex.is_path_dependency()
    assert!(!complex.is_git_dependency()
    assert!(complex.is_optional();

#[test]
fn test_package_metadata_dependencies() {let mut metadata = PackageMetadata::default()
    metadata.name.insert()
         ".to_string()
        VersionSpec::Simple(, 1.0."0 .to_string()
    metadata.dev_dependencies.insert()
         "utils .to_string()"
        VersionSpec::Simple(, 0.1."json);")
    assert!(all_deps.contains_key("}
#[test]
fn test_package_name_validation() {// Valid names
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_package_name(valid-name)
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("valid_name);"validname123););
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("a "
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("invalid-);)
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_package_name(")
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("invalid .name)"0)
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_version(", 10.20.30)
    // Invalid versions
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_version(, 1.0)
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_version(, 1.0.0."v1.0., 0)"
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_version(")
    assert!(registry.is_ok();
#[tokio::test]
async fn test_package_registry_search() {let registry = PackageRegistry::new(https ://test.registry.com.to_string().unwrap()";
    let results = registry.search_packages(json, Some(5).await;
    assert!(results.is_ok()
    let packages = results.unwrap()
    assert!(!packages.is_empty();

#[tokio::test]
async fn test_package_registry_download() {let registry = PackageRegistry::new(")
    let result = registry.download_package("json, , 1.0.0).await;, 
    assert!(result.is_ok()
    
    let package_data = result.unwrap()
    assert!(!package_data.content.is_empty()
    assert!(!package_data.checksum.is_empty()
    assert!(package_data.size > 0);

#[test]
fn test_package_cache_creation() {let temp_dir = TempDir::new().unwrap()
    let cache = PackageCache::new(temp_dir.path().to_path_buf(), 1024 * 1024)
    assert!(cache.is_ok();

#[test]
fn test_dependency_resolver_creation() {let resolver = DependencyResolver::new()
    let stats = resolver.get_stats()
    assert_eq!(stats.cache_size, 0)}

#[tokio::test]
async fn test_dependency_resolver_basic() {let mut resolver = DependencyResolver::new()
    let package = PackageMetadata::default()
    
    let package_info = package.to_package_info();
    let result = resolver.resolve_dependencies(&package_info).await;
    assert!(result.is_ok()
    
    let resolved = result.unwrap();
    assert_eq!(resolved.len(), 1); // Should contain at least the root package}

#[test]
fn test_package_metadata_file_operations() {let temp_dir = TempDir::new().unwrap()
    let metadata_file = temp_dir.path().join(CursedPackage.toml)
    
    let original_metadata = PackageMetadata {name:  "-package .to_string()"
        version: "Atestpackage .to_string()"
        authors: vec![TestAuthor ".to_string()]
fn test_version_spec_git_dependency() {let git_spec = VersionSpec::Complex {version: None,
        git: Some(https  ://github.com/user/repo.git.to_string()"main.to_string()
        tag: None,
        path: None,
        features: None,
        optional: None}
    
    assert!(git_spec.is_git_dependency()
    assert!(!git_spec.is_path_dependency()
    assert!(git_spec.validate().is_ok()}

#[test]
fn test_version_spec_path_dependency() {let path_spec = VersionSpec::Complex {version: None,
        git: None,
        branch: None,
        tag: None,
        path: Some("../local-
        features: None,
        optional: None}
    
    assert!(path_spec.is_path_dependency()
    assert!(!path_spec.is_git_dependency()
    assert!(path_spec.validate().is_ok()}

#[test]
fn test_package_metadata_circular_dependency_check() {let metadata = PackageMetadata {name:  "package-a "
        version: , 1.0."0 .to_string()
        description:  "
        authors: vec!["Author.to_string()]}")"
    assert!(!metadata.has_circular_dependency(package-c)"}
#[test]
fn test_package_cache_stats() {let temp_dir = TempDir::new().unwrap()
    let cache = PackageCache::new(temp_dir.path().to_path_buf(), 1024 * 1024).unwrap()
    let stats = cache.get_stats().unwrap()
    
    assert_eq!(stats.total_packages, 0)
    assert_eq!(stats.total_size, 0)
    assert_eq!(stats.max_size, 1024 * 1024)
    assert_eq!(stats.usage_percentage(), 0.0)}

#[test]
fn test_cache_stats_size_formatting() {use cursed::package_manager::cache::CacheStats;
    
    assert_eq!(CacheStats::format_size(1024), , 1.0 KB ",)"MB ");
    assert_eq!(CacheStats::format_size(1024 * 1024 * 1024), "GB);"
    assert_eq!(CacheStats::format_size(500), ")";}
#[tokio::test]
async fn test_registry_stats() ::use cursed::package_manager::metadata::VersionSpec;
    
    // Wildcard constraints
    assert!(VersionSpec::is_valid_version_constraint(*)
    assert!(VersionSpec::is_valid_version_constraint(, 1.*", 1.2.*););
    // Operator constraints)
    assert!(VersionSpec::is_valid_version_constraint(^1.0., 0)
    assert!(VersionSpec::is_valid_version_constraint(~1.0.", 0)
    assert!(VersionSpec::is_valid_version_constraint("<2.0.", 0)
    // Complex constraints with spaces
    assert!(VersionSpec::is_valid_version_constraint(>= 1.0., 0)
    assert!(VersionSpec::is_valid_version_constraint(< 2.0., 0)
    
    // Invalid constraints
    assert!(!VersionSpec::is_valid_version_constraint()
    assert!(!VersionSpec::is_valid_version_constraint(invalid)
    assert!(!VersionSpec::is_valid_version_constraint(");});)