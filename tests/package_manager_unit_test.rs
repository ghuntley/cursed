/// Unit tests for CURSED package manager components
use cursed::package_manager::{
    PackageMetadata, VersionSpec, PackageRegistry, PackageCache, DependencyResolver
};
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
fn test_package_metadata_creation() {
    let metadata = PackageMetadata {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        description: "A test package".to_string(),
        authors: vec!["Test Author <test@example.com>".to_string()],
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
        repository: None,
        license: Some("MIT".to_string()),
        keywords: vec!["test".to_string()],
        categories: vec!["testing".to_string()],
    };
    
    assert_eq!(metadata.name, "test-package");
    assert_eq!(metadata.version, "1.0.0");
    assert_eq!(metadata.package_id(), "test-package@1.0.0");
}

#[test]
fn test_version_spec_types() {
    let simple = VersionSpec::Simple("1.0.0".to_string());
    assert_eq!(simple.version_string(), Some("1.0.0"));
    assert!(!simple.is_path_dependency());
    assert!(!simple.is_git_dependency());
    assert!(!simple.is_optional());
    
    let complex = VersionSpec::Complex {
        version: Some("^1.0.0".to_string()),
        git: None,
        branch: None,
        tag: None,
        path: Some("../local-package".to_string()),
        features: Some(vec!["feature1".to_string()]),
        optional: Some(true),
    };
    
    assert_eq!(complex.version_string(), Some("^1.0.0"));
    assert!(complex.is_path_dependency());
    assert!(!complex.is_git_dependency());
    assert!(complex.is_optional());
}

#[test]
fn test_package_metadata_dependencies() {
    let mut metadata = PackageMetadata::default();
    metadata.dependencies.insert(
        "json".to_string(),
        VersionSpec::Simple("1.0.0".to_string())
    );
    metadata.dev_dependencies.insert(
        "test-utils".to_string(),
        VersionSpec::Simple("0.1.0".to_string())
    );
    
    let all_deps = metadata.all_dependencies();
    assert_eq!(all_deps.len(), 2);
    assert!(all_deps.contains_key("json"));
    assert!(all_deps.contains_key("test-utils"));
}

#[test]
fn test_package_name_validation() {
    // Valid names
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("valid-name"));
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("valid_name"));
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("validname123"));
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("a"));
    
    // Invalid names
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_package_name(""));
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("-invalid"));
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("invalid-"));
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("invalid@name"));
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_package_name("invalid.name"));
}

#[test]
fn test_version_validation() {
    // Valid versions
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_version("1.0.0"));
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_version("0.1.0"));
    assert!(cursed::package_manager::metadata::PackageMetadata::is_valid_version("10.20.30"));
    
    // Invalid versions
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_version("1.0"));
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_version("1.0.0.0"));
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_version("v1.0.0"));
    assert!(!cursed::package_manager::metadata::PackageMetadata::is_valid_version(""));
}

#[tokio::test]
async fn test_package_registry_creation() {
    let registry = PackageRegistry::new("https://test.registry.com".to_string());
    assert!(registry.is_ok());
}

#[tokio::test]
async fn test_package_registry_search() {
    let registry = PackageRegistry::new("https://test.registry.com".to_string()).unwrap();
    let results = registry.search_packages("json", Some(5)).await;
    assert!(results.is_ok());
    
    let packages = results.unwrap();
    assert!(!packages.is_empty());
}

#[tokio::test]
async fn test_package_registry_download() {
    let registry = PackageRegistry::new("https://test.registry.com".to_string()).unwrap();
    let result = registry.download_package("json", "1.0.0").await;
    assert!(result.is_ok());
    
    let package_data = result.unwrap();
    assert!(!package_data.content.is_empty());
    assert!(!package_data.checksum.is_empty());
    assert!(package_data.size > 0);
}

#[test]
fn test_package_cache_creation() {
    let temp_dir = TempDir::new().unwrap();
    let cache = PackageCache::new(temp_dir.path().to_path_buf(), 1024 * 1024);
    assert!(cache.is_ok());
}

#[test]
fn test_dependency_resolver_creation() {
    let resolver = DependencyResolver::new();
    let stats = resolver.get_stats();
    assert_eq!(stats.cache_size, 0);
}

#[tokio::test]
async fn test_dependency_resolver_basic() {
    let mut resolver = DependencyResolver::new();
    let package = PackageMetadata::default();
    
    let package_info = package.to_package_info();
    let result = resolver.resolve_dependencies(&package_info).await;
    assert!(result.is_ok());
    
    let resolved = result.unwrap();
    assert_eq!(resolved.len(), 1); // Should contain at least the root package
}

#[test]
fn test_package_metadata_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let metadata_file = temp_dir.path().join("CursedPackage.toml");
    
    let original_metadata = PackageMetadata {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        description: "A test package".to_string(),
        authors: vec!["Test Author <test@example.com>".to_string()],
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
        repository: None,
        license: Some("MIT".to_string()),
        keywords: vec!["test".to_string()],
        categories: vec!["testing".to_string()],
    };
    
    // Save to file
    let save_result = original_metadata.save_to_file(&metadata_file);
    assert!(save_result.is_ok());
    assert!(metadata_file.exists());
    
    // Load from file
    let loaded_result = PackageMetadata::from_file(&metadata_file);
    assert!(loaded_result.is_ok());
    
    let loaded_metadata = loaded_result.unwrap();
    assert_eq!(loaded_metadata.name, original_metadata.name);
    assert_eq!(loaded_metadata.version, original_metadata.version);
    assert_eq!(loaded_metadata.description, original_metadata.description);
}

#[test]
fn test_version_spec_git_dependency() {
    let git_spec = VersionSpec::Complex {
        version: None,
        git: Some("https://github.com/user/repo.git".to_string()),
        branch: Some("main".to_string()),
        tag: None,
        path: None,
        features: None,
        optional: None,
    };
    
    assert!(git_spec.is_git_dependency());
    assert!(!git_spec.is_path_dependency());
    assert!(git_spec.validate().is_ok());
}

#[test]
fn test_version_spec_path_dependency() {
    let path_spec = VersionSpec::Complex {
        version: None,
        git: None,
        branch: None,
        tag: None,
        path: Some("../local-package".to_string()),
        features: None,
        optional: None,
    };
    
    assert!(path_spec.is_path_dependency());
    assert!(!path_spec.is_git_dependency());
    assert!(path_spec.validate().is_ok());
}

#[test]
fn test_package_metadata_circular_dependency_check() {
    let metadata = PackageMetadata {
        name: "package-a".to_string(),
        version: "1.0.0".to_string(),
        description: "Package A".to_string(),
        authors: vec!["Author".to_string()],
        dependencies: HashMap::from([
            ("package-b".to_string(), VersionSpec::Simple("1.0.0".to_string())),
        ]),
        dev_dependencies: HashMap::new(),
        repository: None,
        license: None,
        keywords: vec![],
        categories: vec![],
    };
    
    assert!(metadata.has_circular_dependency("package-b"));
    assert!(!metadata.has_circular_dependency("package-c"));
}

#[test]
fn test_package_cache_stats() {
    let temp_dir = TempDir::new().unwrap();
    let cache = PackageCache::new(temp_dir.path().to_path_buf(), 1024 * 1024).unwrap();
    let stats = cache.get_stats().unwrap();
    
    assert_eq!(stats.total_packages, 0);
    assert_eq!(stats.total_size, 0);
    assert_eq!(stats.max_size, 1024 * 1024);
    assert_eq!(stats.usage_percentage(), 0.0);
}

#[test]
fn test_cache_stats_size_formatting() {
    use cursed::package_manager::cache::CacheStats;
    
    assert_eq!(CacheStats::format_size(1024), "1.0 KB");
    assert_eq!(CacheStats::format_size(1024 * 1024), "1.0 MB");
    assert_eq!(CacheStats::format_size(1024 * 1024 * 1024), "1.0 GB");
    assert_eq!(CacheStats::format_size(500), "500.0 B");
}

#[tokio::test]
async fn test_registry_stats() {
    let registry = PackageRegistry::new("https://test.registry.com".to_string()).unwrap();
    let stats = registry.get_stats();
    
    assert_eq!(stats.registry_url, "https://test.registry.com");
    assert!(stats.last_updated.is_none()); // Should be None initially
}

#[test]
fn test_version_constraint_edge_cases() {
    use cursed::package_manager::metadata::VersionSpec;
    
    // Wildcard constraints
    assert!(VersionSpec::is_valid_version_constraint("*"));
    assert!(VersionSpec::is_valid_version_constraint("1.*"));
    assert!(VersionSpec::is_valid_version_constraint("1.2.*"));
    
    // Operator constraints
    assert!(VersionSpec::is_valid_version_constraint("^1.0.0"));
    assert!(VersionSpec::is_valid_version_constraint("~1.0.0"));
    assert!(VersionSpec::is_valid_version_constraint(">=1.0.0"));
    assert!(VersionSpec::is_valid_version_constraint("<2.0.0"));
    
    // Complex constraints with spaces
    assert!(VersionSpec::is_valid_version_constraint(">= 1.0.0"));
    assert!(VersionSpec::is_valid_version_constraint("< 2.0.0"));
    
    // Invalid constraints
    assert!(!VersionSpec::is_valid_version_constraint(""));
    assert!(!VersionSpec::is_valid_version_constraint("invalid"));
    assert!(!VersionSpec::is_valid_version_constraint("1.0.0.0"));
}
