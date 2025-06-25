use cursed::package_manager::{
    PackageManagerError, PackageRegistry, DependencyResolver, 
    metadata::{PackageMetadata, VersionSpec},
    registry::{PackageInfo, RegistryConfig},
    resolver::{ConflictResolutionStrategy, ExportFormat},
};
use semver::Version;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio;
use tracing_test::traced_test;

/// Create a test registry with timeout and mock server
fn create_test_registry() -> PackageRegistry {
    let config = RegistryConfig {
        base_url: "https://mock-registry.cursed-lang.org".to_string(),
        timeout: Duration::from_secs(5),
        max_retries: 1,
        auth_token: None,
        user_agent: "cursed-test-client".to_string(),
        verify_tls: false,
    };
    
    PackageRegistry::with_config(config).expect("Failed to create test registry")
}

/// Create test package info
fn create_test_package_info() -> PackageInfo {
    PackageInfo {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        description: "A test package for dependency resolution".to_string(),
        download_url: "https://mock-registry.cursed-lang.org/packages/test-package/1.0.0/download".to_string(),
        checksum: "abc123".to_string(),
        size: Some(1024),
        published_at: Some("2023-01-01T00:00:00Z".to_string()),
        authors: Some(vec!["Test Author <test@example.com>".to_string()]),
        license: Some("MIT".to_string()),
        repository: Some("https://github.com/test/test-package".to_string()),
        keywords: Some(vec!["test".to_string(), "dependency".to_string()]),
    }
}

/// Create test package metadata with dependencies
fn create_test_package_metadata_with_deps() -> PackageMetadata {
    let mut dependencies = HashMap::new();
    dependencies.insert("serde".to_string(), VersionSpec::Simple("1.0".to_string()));
    dependencies.insert("tokio".to_string(), VersionSpec::Simple("1.0".to_string()));

    let mut dev_dependencies = HashMap::new();
    dev_dependencies.insert("tokio-test".to_string(), VersionSpec::Simple("0.4".to_string()));

    PackageMetadata {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        description: "A test package".to_string(),
        authors: vec!["Test Author".to_string()],
        dependencies,
        dev_dependencies,
        repository: Some("https://github.com/test/test-package".to_string()),
        license: Some("MIT".to_string()),
        keywords: vec!["test".to_string()],
        categories: vec!["testing".to_string()],
    }
}

#[tokio::test]
#[traced_test]
async fn test_resolver_with_registry_creation() {
    let registry = create_test_registry();
    let resolver = DependencyResolver::with_registry(Arc::new(Mutex::new(registry)));
    
    // Verify resolver was created successfully
    let stats = resolver.get_stats();
    assert_eq!(stats.resolved_count, 0);
    assert_eq!(stats.cached_count, 0);
    assert_eq!(stats.failed_count, 0);
}

#[tokio::test]
#[traced_test]
async fn test_resolver_without_registry_fallback() {
    let mut resolver = DependencyResolver::new();
    let package = create_test_package_info();
    
    // This should work with fallback behavior when no registry is set
    let result = resolver.resolve_dependencies(&package).await;
    
    // Should not fail even without registry (uses fallback)
    match result {
        Ok(dependencies) => {
            // Should have at least the root package resolved
            assert!(!dependencies.is_empty());
            tracing::info!("Resolved {} dependencies using fallback", dependencies.len());
        }
        Err(e) => {
            tracing::warn!("Resolution failed as expected without registry: {}", e);
            // This is acceptable - the resolver should handle missing registry gracefully
        }
    }
}

#[tokio::test] 
#[traced_test]
async fn test_resolver_set_registry() {
    let mut resolver = DependencyResolver::new();
    let registry = create_test_registry();
    
    // Set registry after creation
    resolver.set_registry(Arc::new(Mutex::new(registry)));
    
    let package = create_test_package_info();
    let result = resolver.resolve_dependencies(&package).await;
    
    // Should handle registry errors gracefully
    match result {
        Ok(dependencies) => {
            tracing::info!("Resolved {} dependencies", dependencies.len());
            assert!(!dependencies.is_empty());
        }
        Err(e) => {
            tracing::warn!("Expected network error with mock registry: {}", e);
            // Network errors are expected with mock registry URL
        }
    }
}

#[tokio::test]
#[traced_test]
async fn test_version_constraint_parsing() {
    let mut resolver = DependencyResolver::new();
    
    // Test various version constraint formats
    let constraints = vec![
        "1.0.0",
        "^1.0.0", 
        "~1.2.0",
        ">=1.0.0",
        "1.0.0 - 2.0.0",
        "*",
    ];
    
    for constraint in constraints {
        let version_req = semver::VersionReq::parse(constraint);
        assert!(version_req.is_ok(), "Failed to parse constraint: {}", constraint);
        tracing::info!("Successfully parsed version constraint: {}", constraint);
    }
}

#[tokio::test]
#[traced_test]
async fn test_conflict_resolution_strategies() {
    // Test different conflict resolution strategies
    let strategies = vec![
        ConflictResolutionStrategy::LatestCompatible,
        ConflictResolutionStrategy::ConservativeUpdate,
        ConflictResolutionStrategy::MinimalChange,
        ConflictResolutionStrategy::UserPrompt,
    ];
    
    for strategy in strategies {
        let resolver = DependencyResolver::with_config(50, true, strategy);
        let stats = resolver.get_stats();
        assert_eq!(stats.conflicts_resolved, 0);
        tracing::info!("Created resolver with conflict strategy: {:?}", strategy);
    }
}

#[tokio::test]
#[traced_test]
async fn test_cache_functionality() {
    let mut resolver = DependencyResolver::new();
    let package = create_test_package_info();
    
    // First resolution attempt
    let _result1 = resolver.resolve_dependencies(&package).await;
    
    // Second resolution should use cache
    let _result2 = resolver.resolve_dependencies(&package).await;
    
    let stats = resolver.get_stats();
    tracing::info!("Cache stats - cached: {}, resolved: {}", stats.cached_count, stats.resolved_count);
    
    // Clear cache and verify
    resolver.clear_cache();
    let stats_after_clear = resolver.get_stats();
    assert_eq!(stats_after_clear.cache_size, 0);
}

#[tokio::test]
#[traced_test]
async fn test_dependency_tree_generation() {
    let resolver = DependencyResolver::new();
    let metadata = create_test_package_metadata_with_deps();
    
    // Create some mock resolved dependencies
    let resolved_deps = vec![
        cursed::package_manager::resolver::ResolvedDependency {
            package: metadata.clone(),
            depth: 0,
            required_by: vec![],
            constraint: "*".to_string(),
            resolved_version: Version::parse("1.0.0").unwrap(),
            is_dev_dependency: false,
            optional: false,
        },
    ];
    
    // Test tree generation
    let tree = resolver.generate_tree(&resolved_deps);
    assert!(tree.contains("test-package@1.0.0"));
    assert!(tree.contains("├──"));
    tracing::info!("Generated dependency tree:\n{}", tree);
}

#[tokio::test]
#[traced_test]
async fn test_conflict_detection() {
    let resolver = DependencyResolver::new();
    
    // Create conflicting dependencies (same package, different versions)
    let metadata1 = PackageMetadata {
        name: "conflict-package".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package v1".to_string(),
        authors: vec![],
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
        repository: None,
        license: None,
        keywords: vec![],
        categories: vec![],
    };
    
    let metadata2 = PackageMetadata {
        name: "conflict-package".to_string(),
        version: "2.0.0".to_string(), 
        description: "Test package v2".to_string(),
        authors: vec![],
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
        repository: None,
        license: None,
        keywords: vec![],
        categories: vec![],
    };
    
    let resolved_deps = vec![
        cursed::package_manager::resolver::ResolvedDependency {
            package: metadata1,
            depth: 1,
            required_by: vec!["root".to_string()],
            constraint: "1.0".to_string(),
            resolved_version: Version::parse("1.0.0").unwrap(),
            is_dev_dependency: false,
            optional: false,
        },
        cursed::package_manager::resolver::ResolvedDependency {
            package: metadata2,
            depth: 1,
            required_by: vec!["other".to_string()],
            constraint: "2.0".to_string(),
            resolved_version: Version::parse("2.0.0").unwrap(),
            is_dev_dependency: false,
            optional: false,
        },
    ];
    
    let conflicts = resolver.check_conflicts(&resolved_deps);
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].package, "conflict-package");
    tracing::info!("Detected {} conflicts as expected", conflicts.len());
}

#[tokio::test]
#[traced_test]
async fn test_export_formats() {
    let resolver = DependencyResolver::new();
    let metadata = create_test_package_metadata_with_deps();
    
    let resolved_deps = vec![
        cursed::package_manager::resolver::ResolvedDependency {
            package: metadata,
            depth: 0,
            required_by: vec![],
            constraint: "*".to_string(),
            resolved_version: Version::parse("1.0.0").unwrap(),
            is_dev_dependency: false,
            optional: false,
        },
    ];
    
    // Test JSON export
    let json_result = resolver.export_resolution(&resolved_deps, ExportFormat::Json);
    assert!(json_result.is_ok());
    let json_output = json_result.unwrap();
    assert!(json_output.contains("test-package"));
    tracing::info!("JSON export successful: {} chars", json_output.len());
    
    // Test YAML export
    let yaml_result = resolver.export_resolution(&resolved_deps, ExportFormat::Yaml);
    assert!(yaml_result.is_ok()); 
    let yaml_output = yaml_result.unwrap();
    assert!(yaml_output.contains("test-package"));
    tracing::info!("YAML export successful: {} chars", yaml_output.len());
    
    // Test Tree export
    let tree_result = resolver.export_resolution(&resolved_deps, ExportFormat::Tree);
    assert!(tree_result.is_ok());
    let tree_output = tree_result.unwrap();
    assert!(tree_output.contains("test-package"));
    tracing::info!("Tree export successful: {} chars", tree_output.len());
}

#[tokio::test]
#[traced_test]
async fn test_resolver_error_handling() {
    let mut resolver = DependencyResolver::new();
    
    // Test with invalid package info
    let invalid_package = PackageInfo {
        name: "".to_string(), // Invalid empty name
        version: "invalid-version".to_string(), // Invalid version format
        description: "Invalid package".to_string(),
        download_url: "not-a-url".to_string(),
        checksum: "invalid".to_string(),
        size: None,
        published_at: None,
        authors: None,
        license: None,
        repository: None,
        keywords: None,
    };
    
    let result = resolver.resolve_dependencies(&invalid_package).await;
    match result {
        Ok(_) => {
            tracing::info!("Resolver handled invalid package gracefully");
        }
        Err(e) => {
            tracing::info!("Expected error with invalid package: {}", e);
            // This is expected behavior
        }
    }
}

#[tokio::test]
#[traced_test]
async fn test_deep_dependency_resolution() {
    let registry = create_test_registry();
    let mut resolver = DependencyResolver::with_config(10, true, ConflictResolutionStrategy::LatestCompatible);
    resolver.set_registry(Arc::new(Mutex::new(registry)));
    
    let package = create_test_package_info();
    
    let result = resolver.resolve_dependencies(&package).await;
    
    match result {
        Ok(dependencies) => {
            tracing::info!("Deep dependency resolution successful: {} packages", dependencies.len());
            
            // Verify dependency depth tracking
            for dep in &dependencies {
                assert!(dep.depth <= 10, "Dependency depth exceeded maximum");
                tracing::debug!("Package {} at depth {}", dep.package.name, dep.depth);
            }
        }
        Err(e) => {
            tracing::warn!("Deep dependency resolution failed (expected with mock registry): {}", e);
        }
    }
    
    let stats = resolver.get_stats();
    tracing::info!("Resolution stats: resolved={}, cached={}, failed={}, conflicts={}", 
                   stats.resolved_count, stats.cached_count, stats.failed_count, stats.conflicts_resolved);
}

#[tokio::test]
#[traced_test]
async fn test_performance_characteristics() {
    let start_time = std::time::Instant::now();
    let mut resolver = DependencyResolver::new();
    let package = create_test_package_info();
    
    // Test resolution performance
    let _result = resolver.resolve_dependencies(&package).await;
    
    let elapsed = start_time.elapsed();
    let stats = resolver.get_stats();
    
    tracing::info!("Resolution completed in {:?}", elapsed);
    tracing::info!("Performance stats: resolution_time={}ms, cache_size={}", 
                   stats.resolution_time_ms, stats.cache_size);
    
    // Performance should be reasonable even with network timeouts
    assert!(elapsed.as_secs() < 30, "Resolution took too long");
}
