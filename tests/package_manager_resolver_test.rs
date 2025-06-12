use cursed::package_manager::{
    resolver::{
        DependencyResolver, ResolvedDependency, ConflictInfo, ConflictReason,
        ConflictResolutionStrategy, ExportFormat
    },
    registry::PackageInfo,
    metadata::PackageMetadata,
    PackageManagerError,
};
use std::collections::HashMap;
use semver::{Version, VersionReq};

fn create_test_package_info(name: &str, version: &str) -> PackageInfo {
    PackageInfo {
        name: name.to_string(),
        version: version.to_string(),
        description: format!("Test package {}", name),
        download_url: "https://example.com/download".to_string(),
        checksum: "test-checksum".to_string(),
        size: Some(1024),
        published_at: Some("2023-01-01T00:00:00Z".to_string()),
        authors: Some(vec!["Test Author <test@example.com>".to_string()]),
        repository: None,
        license: Some("MIT".to_string()),
        keywords: Some(vec!["test".to_string()]),
    }
}

fn create_package_info_with_deps(name: &str, version: &str, _deps: HashMap<String, String>) -> PackageInfo {
    // For now, just return basic package info since PackageInfo doesn't store dependencies
    create_test_package_info(name, version)
}

fn create_package_info_with_dev_deps(name: &str, version: &str, _dev_deps: HashMap<String, String>) -> PackageInfo {
    // For now, just return basic package info since PackageInfo doesn't store dev dependencies
    create_test_package_info(name, version)
}

#[tokio::test]
async fn test_resolver_creation() {
    let resolver = DependencyResolver::new();
    let stats = resolver.get_stats();
    
    assert_eq!(stats.resolved_count, 0);
    assert_eq!(stats.cached_count, 0);
    assert_eq!(stats.failed_count, 0);
    assert_eq!(stats.cache_size, 0);
}

#[tokio::test]
async fn test_resolver_with_config() {
    let resolver = DependencyResolver::with_config(
        25,
        false,
        ConflictResolutionStrategy::ConservativeUpdate
    );
    
    assert_eq!(resolver.max_depth, 25);
    assert!(!resolver.allow_dev_dependencies);
}

#[tokio::test]
async fn test_basic_dependency_resolution() {
    let mut resolver = DependencyResolver::new();
    let package = create_test_package_info("basic-test", "1.0.0");
    
    let result = resolver.resolve_dependencies(&package).await;
    assert!(result.is_ok());
    
    let dependencies = result.unwrap();
    assert!(!dependencies.is_empty()); // Should at least include the root package
    
    let stats = resolver.get_stats();
    assert_eq!(stats.resolved_count, 1);
    assert_eq!(stats.failed_count, 0);
}

#[tokio::test]
async fn test_package_with_dependencies() {
    let mut resolver = DependencyResolver::new();
    
    let mut deps = HashMap::new();
    deps.insert("serde".to_string(), "1.0".to_string());
    deps.insert("tokio".to_string(), "1.0".to_string());
    
    let package = create_package_info_with_deps("complex-test", "1.0.0", deps);
    
    let result = resolver.resolve_dependencies(&package).await;
    assert!(result.is_ok());
    
    let dependencies = result.unwrap();
    assert!(dependencies.len() > 1); // Should include dependencies
    
    // Check that serde and tokio are included
    let dep_names: Vec<String> = dependencies.iter().map(|d| d.package.name.clone()).collect();
    assert!(dep_names.contains(&"serde".to_string()));
    assert!(dep_names.contains(&"tokio".to_string()));
}

#[tokio::test]
async fn test_dev_dependencies_resolution() {
    let mut resolver = DependencyResolver::new(); // Allows dev dependencies by default
    
    let mut dev_deps = HashMap::new();
    dev_deps.insert("clap".to_string(), "3.0".to_string());
    
    let package = create_package_info_with_dev_deps("dev-test", "1.0.0", dev_deps);
    
    let result = resolver.resolve_dependencies(&package).await;
    assert!(result.is_ok());
    
    let dependencies = result.unwrap();
    
    // Check that clap dev dependency is included
    let dep_names: Vec<String> = dependencies.iter().map(|d| d.package.name.clone()).collect();
    assert!(dep_names.contains(&"clap".to_string()));
    
    // Check that it's marked as dev dependency
    let clap_dep = dependencies.iter().find(|d| d.package.name == "clap");
    assert!(clap_dep.is_some());
    assert!(clap_dep.unwrap().is_dev_dependency);
}

#[tokio::test]
async fn test_dev_dependencies_disabled() {
    let mut resolver = DependencyResolver::with_config(
        50,
        false, // Disable dev dependencies
        ConflictResolutionStrategy::LatestCompatible
    );
    
    let mut dev_deps = HashMap::new();
    dev_deps.insert("clap".to_string(), "3.0".to_string());
    
    let package = create_package_info_with_dev_deps("no-dev-test", "1.0.0", dev_deps);
    
    let result = resolver.resolve_dependencies(&package).await;
    assert!(result.is_ok());
    
    let dependencies = result.unwrap();
    
    // Check that clap dev dependency is NOT included
    let dep_names: Vec<String> = dependencies.iter().map(|d| d.package.name.clone()).collect();
    assert!(!dep_names.contains(&"clap".to_string()));
}

#[tokio::test]
async fn test_version_requirement_parsing() {
    let valid_requirements = vec![
        "1.0.0",
        "^1.0",
        "~1.2",
        ">=1.0.0",
        "1.0.0 - 2.0.0",
        "*",
    ];
    
    for req_str in valid_requirements {
        let result = VersionReq::parse(req_str);
        assert!(result.is_ok(), "Failed to parse version requirement: {}", req_str);
    }
}

#[tokio::test]
async fn test_invalid_version_constraint() {
    let mut resolver = DependencyResolver::new();
    
    let mut deps = HashMap::new();
    deps.insert("invalid-dep".to_string(), "invalid-version".to_string());
    
    let package = create_package_info_with_deps("invalid-test", "1.0.0", deps);
    
    let result = resolver.resolve_dependencies(&package).await;
    assert!(result.is_err());
    
    match result.unwrap_err() {
        PackageManagerError::InvalidVersion { version, reason: _ } => {
            assert_eq!(version, "invalid-version");
        }
        _ => panic!("Expected InvalidVersion error"),
    }
}

#[tokio::test]
async fn test_caching_functionality() {
    let mut resolver = DependencyResolver::new();
    let package = create_test_package_info("cache-test", "1.0.0");
    
    // First resolution
    let result1 = resolver.resolve_dependencies(&package).await;
    assert!(result1.is_ok());
    
    // Second resolution should use cache
    let result2 = resolver.resolve_dependencies(&package).await;
    assert!(result2.is_ok());
    
    let stats = resolver.get_stats();
    assert_eq!(stats.resolved_count, 1); // Only one actual resolution
    assert_eq!(stats.cached_count, 1);   // One cache hit
}

#[tokio::test]
async fn test_cache_clearing() {
    let mut resolver = DependencyResolver::new();
    let package = create_test_package_info("clear-cache-test", "1.0.0");
    
    // First resolution
    let _result1 = resolver.resolve_dependencies(&package).await;
    
    // Clear cache
    resolver.clear_cache();
    
    let stats = resolver.get_stats();
    assert_eq!(stats.cache_size, 0);
    
    // Second resolution should not use cache
    let _result2 = resolver.resolve_dependencies(&package).await;
    
    let stats_after = resolver.get_stats();
    assert_eq!(stats_after.cached_count, 0); // No cache hits after clearing
}

#[tokio::test]
async fn test_dependency_depth_tracking() {
    let mut resolver = DependencyResolver::new();
    
    let mut deps = HashMap::new();
    deps.insert("serde".to_string(), "1.0".to_string());
    
    let package = create_package_info_with_deps("depth-test", "1.0.0", deps);
    
    let result = resolver.resolve_dependencies(&package).await;
    assert!(result.is_ok());
    
    let dependencies = result.unwrap();
    
    // Check depth tracking
    for dep in &dependencies {
        assert!(dep.depth >= 0);
        assert!(dep.depth <= resolver.max_depth);
    }
    
    // Root dependencies should have depth 1
    let root_deps: Vec<_> = dependencies.iter().filter(|d| d.depth == 1).collect();
    assert!(!root_deps.is_empty());
}

#[tokio::test]
async fn test_conflict_detection() {
    let resolver = DependencyResolver::new();
    
    // Create conflicting dependencies
    let dep1 = ResolvedDependency {
        package: PackageMetadata {
            name: "conflict-pkg".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            authors: vec![],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: vec![],
            categories: vec![],
        },
        depth: 1,
        required_by: vec!["root1".to_string()],
        constraint: "1.0".to_string(),
        resolved_version: Version::parse("1.0.0").unwrap(),
        is_dev_dependency: false,
        optional: false,
    };
    
    let dep2 = ResolvedDependency {
        package: PackageMetadata {
            name: "conflict-pkg".to_string(),
            version: "2.0.0".to_string(),
            description: "Test".to_string(),
            authors: vec![],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: vec![],
            categories: vec![],
        },
        depth: 1,
        required_by: vec!["root2".to_string()],
        constraint: "2.0".to_string(),
        resolved_version: Version::parse("2.0.0").unwrap(),
        is_dev_dependency: false,
        optional: false,
    };
    
    let dependencies = vec![dep1, dep2];
    let conflicts = resolver.check_conflicts(&dependencies);
    
    assert_eq!(conflicts.len(), 1);
    let conflict = &conflicts[0];
    assert_eq!(conflict.package, "conflict-pkg");
    assert_eq!(conflict.conflicting_versions.len(), 2);
    assert!(matches!(conflict.reason, ConflictReason::IncompatibleVersions));
}

#[tokio::test]
async fn test_tree_generation() {
    let resolver = DependencyResolver::new();
    
    let dep1 = ResolvedDependency {
        package: PackageMetadata {
            name: "tree-dep1".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            authors: vec![],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: vec![],
            categories: vec![],
        },
        depth: 1,
        required_by: vec!["root".to_string()],
        constraint: "1.0".to_string(),
        resolved_version: Version::parse("1.0.0").unwrap(),
        is_dev_dependency: false,
        optional: false,
    };
    
    let dep2 = ResolvedDependency {
        package: PackageMetadata {
            name: "tree-dep2".to_string(),
            version: "2.0.0".to_string(),
            description: "Test".to_string(),
            authors: vec![],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: vec![],
            categories: vec![],
        },
        depth: 2,
        required_by: vec!["tree-dep1".to_string()],
        constraint: "2.0".to_string(),
        resolved_version: Version::parse("2.0.0").unwrap(),
        is_dev_dependency: true,
        optional: false,
    };
    
    let dependencies = vec![dep1, dep2];
    let tree = resolver.generate_tree(&dependencies);
    
    assert!(tree.contains("tree-dep1@1.0.0"));
    assert!(tree.contains("tree-dep2@2.0.0"));
    assert!(tree.contains("[dev]")); // Dev dependency marker
    assert!(tree.contains("├──"));   // Tree structure
}

#[tokio::test]
async fn test_export_json_format() {
    let resolver = DependencyResolver::new();
    
    let dep = ResolvedDependency {
        package: PackageMetadata {
            name: "export-test".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            authors: vec![],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: vec![],
            categories: vec![],
        },
        depth: 1,
        required_by: vec!["root".to_string()],
        constraint: "1.0".to_string(),
        resolved_version: Version::parse("1.0.0").unwrap(),
        is_dev_dependency: false,
        optional: false,
    };
    
    let dependencies = vec![dep];
    let json_result = resolver.export_resolution(&dependencies, ExportFormat::Json);
    
    assert!(json_result.is_ok());
    let json_string = json_result.unwrap();
    assert!(json_string.contains("export-test"));
    assert!(json_string.contains("1.0.0"));
}

#[tokio::test]
async fn test_export_tree_format() {
    let resolver = DependencyResolver::new();
    
    let dep = ResolvedDependency {
        package: PackageMetadata {
            name: "tree-export-test".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            authors: vec![],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: vec![],
            categories: vec![],
        },
        depth: 1,
        required_by: vec!["root".to_string()],
        constraint: "1.0".to_string(),
        resolved_version: Version::parse("1.0.0").unwrap(),
        is_dev_dependency: false,
        optional: false,
    };
    
    let dependencies = vec![dep];
    let tree_result = resolver.export_resolution(&dependencies, ExportFormat::Tree);
    
    assert!(tree_result.is_ok());
    let tree_string = tree_result.unwrap();
    assert!(tree_string.contains("tree-export-test@1.0.0"));
    assert!(tree_string.contains("├──"));
}

#[tokio::test]
async fn test_max_depth_limit() {
    let mut resolver = DependencyResolver::with_config(
        1, // Very low depth limit
        true,
        ConflictResolutionStrategy::LatestCompatible
    );
    
    let mut deps = HashMap::new();
    deps.insert("serde".to_string(), "1.0".to_string()); // This will have sub-dependencies
    
    let package = create_package_info_with_deps("depth-limit-test", "1.0.0", deps);
    
    let result = resolver.resolve_dependencies(&package).await;
    
    // Should either succeed with limited depth or fail with depth exceeded error
    if result.is_err() {
        match result.unwrap_err() {
            PackageManagerError::DependencyError { reason } => {
                assert!(reason.contains("depth"));
            }
            _ => panic!("Expected DependencyError with depth limit"),
        }
    }
}

#[tokio::test]
async fn test_dependency_sorting() {
    let mut resolver = DependencyResolver::new();
    
    let mut deps = HashMap::new();
    deps.insert("z-package".to_string(), "1.0".to_string());
    deps.insert("a-package".to_string(), "1.0".to_string());
    deps.insert("m-package".to_string(), "1.0".to_string());
    
    let package = create_package_info_with_deps("sorting-test", "1.0.0", deps);
    
    let result = resolver.resolve_dependencies(&package).await;
    assert!(result.is_ok());
    
    let dependencies = result.unwrap();
    
    // Check that dependencies at the same depth are sorted alphabetically
    let depth_1_deps: Vec<_> = dependencies.iter()
        .filter(|d| d.depth == 1)
        .collect();
    
    for i in 1..depth_1_deps.len() {
        assert!(depth_1_deps[i-1].package.name <= depth_1_deps[i].package.name);
    }
}

#[tokio::test]
async fn test_stats_tracking() {
    let mut resolver = DependencyResolver::new();
    
    let package1 = create_test_package_info("stats-test-1", "1.0.0");
    let package2 = create_test_package_info("stats-test-2", "1.0.0");
    
    // Successful resolution
    let _result1 = resolver.resolve_dependencies(&package1).await;
    
    // Cached resolution
    let _result2 = resolver.resolve_dependencies(&package1).await;
    
    // Another successful resolution
    let _result3 = resolver.resolve_dependencies(&package2).await;
    
    let stats = resolver.get_stats();
    assert_eq!(stats.resolved_count, 2);  // Two unique resolutions
    assert_eq!(stats.cached_count, 1);    // One cache hit
    assert_eq!(stats.failed_count, 0);    // No failures
    assert!(stats.resolution_time_ms > 0); // Should have timing info
    assert!(stats.cache_size > 0);        // Should have cached items
}

#[tokio::test]
async fn test_resolution_with_mixed_dependencies() {
    let mut resolver = DependencyResolver::new();
    
    let mut deps = HashMap::new();
    deps.insert("serde".to_string(), "1.0".to_string());
    
    let mut dev_deps = HashMap::new();
    dev_deps.insert("clap".to_string(), "3.0".to_string());
    
    let mut package = create_test_package_info("mixed-deps-test", "1.0.0");
    package.dependencies = Some(deps);
    package.dev_dependencies = Some(dev_deps);
    
    let result = resolver.resolve_dependencies(&package).await;
    assert!(result.is_ok());
    
    let dependencies = result.unwrap();
    
    // Should have both regular and dev dependencies
    let regular_deps: Vec<_> = dependencies.iter()
        .filter(|d| !d.is_dev_dependency)
        .collect();
    let dev_deps: Vec<_> = dependencies.iter()
        .filter(|d| d.is_dev_dependency)
        .collect();
    
    assert!(!regular_deps.is_empty());
    assert!(!dev_deps.is_empty());
}

#[tokio::test]
async fn test_conflict_resolution_strategies() {
    // Test different conflict resolution strategies
    let strategies = vec![
        ConflictResolutionStrategy::LatestCompatible,
        ConflictResolutionStrategy::ConservativeUpdate,
        ConflictResolutionStrategy::MinimalChange,
        ConflictResolutionStrategy::UserPrompt,
    ];
    
    for strategy in strategies {
        let mut resolver = DependencyResolver::with_config(50, true, strategy);
        
        let package = create_test_package_info("strategy-test", "1.0.0");
        let result = resolver.resolve_dependencies(&package).await;
        
        // All strategies should work for simple cases
        assert!(result.is_ok());
    }
}
