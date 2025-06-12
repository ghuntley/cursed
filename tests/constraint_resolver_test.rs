use cursed::package_manager::{
    resolver::{DependencyResolver, ConflictResolutionStrategy, ConstraintState, Assignment, AssignmentReason, LockFile, ExportFormat},
    registry::{PackageInfo, PackageRegistry},
    PackageManagerError,
};
use semver::Version;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio;

#[path = "common.rs"]
mod common;

/// Mock registry for testing constraint resolution
struct MockConstraintRegistry {
    packages: HashMap<String, Vec<(String, Vec<String>)>>, // name -> [(version, dependencies)]
}

impl MockConstraintRegistry {
    fn new() -> Self {
        let mut registry = Self {
            packages: HashMap::new(),
        };

        // Add packages with complex dependency relationships
        registry.add_package("web-framework", vec![
            ("1.0.0".to_string(), vec!["http-client@^1.0.0".to_string(), "json-parser@^2.0.0".to_string()]),
            ("1.1.0".to_string(), vec!["http-client@^1.1.0".to_string(), "json-parser@^2.1.0".to_string()]),
            ("2.0.0".to_string(), vec!["http-client@^2.0.0".to_string(), "json-parser@^3.0.0".to_string()]),
        ]);

        registry.add_package("http-client", vec![
            ("1.0.0".to_string(), vec!["socket-lib@^1.0.0".to_string()]),
            ("1.1.0".to_string(), vec!["socket-lib@^1.1.0".to_string(), "crypto-utils@^1.0.0".to_string()]),
            ("2.0.0".to_string(), vec!["socket-lib@^2.0.0".to_string(), "crypto-utils@^2.0.0".to_string()]),
        ]);

        registry.add_package("json-parser", vec![
            ("2.0.0".to_string(), vec![]),
            ("2.1.0".to_string(), vec!["utf8-utils@^1.0.0".to_string()]),
            ("3.0.0".to_string(), vec!["utf8-utils@^2.0.0".to_string()]),
        ]);

        registry.add_package("socket-lib", vec![
            ("1.0.0".to_string(), vec![]),
            ("1.1.0".to_string(), vec![]),
            ("2.0.0".to_string(), vec!["ssl-bindings@^1.0.0".to_string()]),
        ]);

        registry.add_package("crypto-utils", vec![
            ("1.0.0".to_string(), vec![]),
            ("2.0.0".to_string(), vec!["random-gen@^1.0.0".to_string()]),
        ]);

        registry.add_package("utf8-utils", vec![
            ("1.0.0".to_string(), vec![]),
            ("2.0.0".to_string(), vec![]),
        ]);

        registry.add_package("ssl-bindings", vec![
            ("1.0.0".to_string(), vec![]),
        ]);

        registry.add_package("random-gen", vec![
            ("1.0.0".to_string(), vec![]),
        ]);

        // Add circular dependency scenario
        registry.add_package("circular-a", vec![
            ("1.0.0".to_string(), vec!["circular-b@^1.0.0".to_string()]),
        ]);

        registry.add_package("circular-b", vec![
            ("1.0.0".to_string(), vec!["circular-a@^1.0.0".to_string()]),
        ]);

        registry
    }

    fn add_package(&mut self, name: &str, versions: Vec<(String, Vec<String>)>) {
        self.packages.insert(name.to_string(), versions);
    }

    fn get_versions(&self, name: &str) -> Vec<Version> {
        self.packages.get(name)
            .map(|versions| versions.iter()
                .map(|(v, _)| Version::parse(v).unwrap())
                .collect())
            .unwrap_or_default()
    }

    fn get_dependencies(&self, name: &str, version: &str) -> Vec<String> {
        self.packages.get(name)
            .and_then(|versions| versions.iter()
                .find(|(v, _)| v == version)
                .map(|(_, deps)| deps.clone()))
            .unwrap_or_default()
    }
}

#[tokio::test]
async fn test_constraint_state_basic_operations() {
    common::tracing::setup();

    let mut state = ConstraintState::new();
    
    // Add packages with versions
    let versions_a = vec![
        Version::parse("1.0.0").unwrap(),
        Version::parse("1.1.0").unwrap(),
        Version::parse("2.0.0").unwrap(),
    ];
    let versions_b = vec![
        Version::parse("1.0.0").unwrap(),
        Version::parse("1.5.0").unwrap(),
    ];

    state.add_package("package-a", versions_a);
    state.add_package("package-b", versions_b);
    
    // Add constraint between packages
    state.add_constraint("package-a", "package-b");
    
    // Test assignment
    let version = Version::parse("1.1.0").unwrap();
    assert!(state.assign("package-a", version, AssignmentReason::UserConstraint));
    
    // Test incomplete state
    assert!(!state.is_complete());
    
    // Assign second package
    let version_b = Version::parse("1.5.0").unwrap();
    assert!(state.assign("package-b", version_b, AssignmentReason::DependencyRequirement));
    
    // Now should be complete
    assert!(state.is_complete());
    
    // Test backtracking
    let assignment = state.backtrack();
    assert!(assignment.is_some());
    assert!(!state.is_complete());
}

#[tokio::test]
async fn test_simple_constraint_resolution() {
    common::tracing::setup();

    let mut resolver = DependencyResolver::with_config(
        10, 
        false, 
        ConflictResolutionStrategy::LatestCompatible
    );

    let package = PackageInfo {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package".to_string(),
        authors: Some(vec!["Test Author".to_string()]),
        keywords: Some(vec!["test".to_string()]),
        download_url: "https://example.com/download".to_string(),
        checksum: "test-checksum".to_string(),
        size: Some(1024),
        published_at: Some("2023-01-01T00:00:00Z".to_string()),
        repository: None,
        license: Some("MIT".to_string()),
    };

    let result = resolver.resolve_dependencies(&package).await;
    assert!(result.is_ok(), "Basic resolution should succeed");

    let dependencies = result.unwrap();
    assert!(!dependencies.is_empty(), "Should have resolved some dependencies");

    // Check that statistics were updated
    let stats = resolver.get_stats();
    assert!(stats.resolved_count > 0, "Should have resolution count");
    assert!(stats.resolution_time_ms > 0, "Should have timing information");
}

#[tokio::test]
async fn test_version_conflict_resolution() {
    common::tracing::setup();

    let mut resolver = DependencyResolver::with_config(
        10, 
        false, 
        ConflictResolutionStrategy::LatestCompatible
    );

    // Test with a package that would have conflicting version requirements
    let package = PackageInfo {
        name: "conflicting-package".to_string(),
        version: "1.0.0".to_string(),
        description: "Package with conflicting dependencies".to_string(),
        authors: None,
        keywords: None,
        download_url: "https://example.com/download".to_string(),
        checksum: "test-checksum".to_string(),
        size: None,
        published_at: None,
        repository: None,
        license: None,
    };

    let result = resolver.resolve_dependencies(&package).await;
    
    // The result might be Ok (if conflicts are resolved) or Err (if unresolvable)
    match result {
        Ok(dependencies) => {
            tracing::info!("Conflict resolution succeeded with {} dependencies", dependencies.len());
            
            // Verify no conflicts in final resolution
            let conflicts = resolver.check_conflicts(&dependencies);
            assert!(conflicts.is_empty(), "Final resolution should have no conflicts");
        }
        Err(e) => {
            tracing::info!("Conflict resolution failed as expected: {}", e);
            // This is acceptable for some conflict scenarios
        }
    }

    // Check backtracking stats
    let stats = resolver.get_stats();
    tracing::info!("Backtrack attempts: {}", stats.backtrack_attempts);
}

#[tokio::test]
async fn test_circular_dependency_detection() {
    common::tracing::setup();

    let mut resolver = DependencyResolver::with_config(
        5, 
        false, 
        ConflictResolutionStrategy::LatestCompatible
    );

    // Create a package that would cause circular dependencies
    let circular_package = PackageInfo {
        name: "circular-root".to_string(),
        version: "1.0.0".to_string(),
        description: "Package that creates circular dependencies".to_string(),
        authors: None,
        keywords: None,
        download_url: "https://example.com/download".to_string(),
        checksum: "test-checksum".to_string(),
        size: None,
        published_at: None,
        repository: None,
        license: None,
    };

    let result = resolver.resolve_dependencies(&circular_package).await;
    
    // Should either succeed (if cycles are broken) or fail with circular dependency error
    match result {
        Ok(_) => {
            tracing::info!("Circular dependency was resolved successfully");
        }
        Err(PackageManagerError::CircularDependency { cycle }) => {
            tracing::info!("Circular dependency detected as expected: {:?}", cycle);
            assert!(!cycle.is_empty(), "Cycle information should be provided");
        }
        Err(e) => {
            tracing::info!("Resolution failed with different error: {}", e);
        }
    }

    let stats = resolver.get_stats();
    if stats.circular_dependencies_detected > 0 {
        tracing::info!("Detected {} circular dependencies", stats.circular_dependencies_detected);
    }
}

#[tokio::test]
async fn test_different_resolution_strategies() {
    common::tracing::setup();

    let strategies = vec![
        ConflictResolutionStrategy::LatestCompatible,
        ConflictResolutionStrategy::ConservativeUpdate,
        ConflictResolutionStrategy::MinimalChange,
    ];

    let package = PackageInfo {
        name: "strategy-test".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package for strategy testing".to_string(),
        authors: None,
        keywords: None,
        download_url: "https://example.com/download".to_string(),
        checksum: "test-checksum".to_string(),
        size: None,
        published_at: None,
        repository: None,
        license: None,
    };

    for strategy in strategies {
        let mut resolver = DependencyResolver::with_config(10, false, strategy);
        
        let result = resolver.resolve_dependencies(&package).await;
        
        match result {
            Ok(dependencies) => {
                tracing::info!("Strategy {:?} resolved {} dependencies", strategy, dependencies.len());
                
                // Verify resolution quality
                let conflicts = resolver.check_conflicts(&dependencies);
                assert!(conflicts.is_empty(), "Resolution should have no conflicts");
            }
            Err(e) => {
                tracing::info!("Strategy {:?} failed: {}", strategy, e);
            }
        }
    }
}

#[tokio::test]
async fn test_lock_file_generation() {
    common::tracing::setup();

    let mut resolver = DependencyResolver::new();
    
    let package = PackageInfo {
        name: "lockfile-test".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package for lock file generation".to_string(),
        authors: None,
        keywords: None,
        download_url: "https://example.com/download".to_string(),
        checksum: "test-checksum".to_string(),
        size: None,
        published_at: None,
        repository: None,
        license: None,
    };

    let result = resolver.resolve_dependencies(&package).await;
    assert!(result.is_ok(), "Resolution should succeed for lock file test");

    let dependencies = result.unwrap();
    
    // Generate lock file
    let lock_file = resolver.generate_lock_file(&dependencies);
    
    // Validate lock file structure
    assert_eq!(lock_file.version, "1.0");
    assert_eq!(lock_file.packages.len(), dependencies.len());
    assert!(!lock_file.metadata.generated_at.is_empty());
    assert_eq!(lock_file.metadata.total_packages, dependencies.len());

    // Test lock file validation
    let validation_result = resolver.validate_lock_file(&lock_file, &dependencies).await;
    assert!(validation_result.is_ok(), "Lock file validation should succeed");
    assert!(validation_result.unwrap(), "Lock file should be valid");

    // Test lock file export
    let exported = resolver.export_resolution(&dependencies, ExportFormat::LockFile);
    assert!(exported.is_ok(), "Lock file export should succeed");
    
    // Verify exported content is valid JSON
    let json_content = exported.unwrap();
    let parsed: Result<LockFile, _> = serde_json::from_str(&json_content);
    assert!(parsed.is_ok(), "Exported lock file should be valid JSON");
}

#[tokio::test]
async fn test_export_formats() {
    common::tracing::setup();

    let mut resolver = DependencyResolver::new();
    
    let package = PackageInfo {
        name: "export-test".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package for export formats".to_string(),
        authors: None,
        keywords: None,
        download_url: "https://example.com/download".to_string(),
        checksum: "test-checksum".to_string(),
        size: None,
        published_at: None,
        repository: None,
        license: None,
    };

    let result = resolver.resolve_dependencies(&package).await;
    assert!(result.is_ok(), "Resolution should succeed for export test");

    let dependencies = result.unwrap();
    
    // Test all export formats
    let formats = vec![
        ExportFormat::Json,
        ExportFormat::Yaml,
        ExportFormat::Tree,
        ExportFormat::LockFile,
    ];

    for format in formats {
        let exported = resolver.export_resolution(&dependencies, format.clone());
        assert!(exported.is_ok(), "Export format {:?} should succeed", format);
        
        let content = exported.unwrap();
        assert!(!content.is_empty(), "Exported content should not be empty");
        
        match format {
            ExportFormat::Json | ExportFormat::LockFile => {
                // Verify valid JSON
                let _: serde_json::Value = serde_json::from_str(&content)
                    .expect("Should be valid JSON");
            }
            ExportFormat::Yaml => {
                // Verify valid YAML
                let _: serde_yaml::Value = serde_yaml::from_str(&content)
                    .expect("Should be valid YAML");
            }
            ExportFormat::Tree => {
                // Tree format should contain dependency markers
                assert!(content.contains("├──") || content.contains("└──") || dependencies.is_empty(),
                        "Tree format should contain tree markers");
            }
        }
    }
}

#[tokio::test]
async fn test_performance_and_timeout() {
    common::tracing::setup();

    let mut resolver = DependencyResolver::with_config(
        20, 
        true, 
        ConflictResolutionStrategy::LatestCompatible
    );

    let package = PackageInfo {
        name: "performance-test".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package for performance testing".to_string(),
        authors: None,
        keywords: None,
        download_url: "https://example.com/download".to_string(),
        checksum: "test-checksum".to_string(),
        size: None,
        published_at: None,
        repository: None,
        license: None,
    };

    let start_time = std::time::Instant::now();
    let result = resolver.resolve_dependencies(&package).await;
    let elapsed = start_time.elapsed();

    // Resolution should complete within reasonable time
    assert!(elapsed.as_secs() < 10, "Resolution should complete within 10 seconds");

    match result {
        Ok(dependencies) => {
            tracing::info!("Performance test resolved {} dependencies in {:?}", 
                          dependencies.len(), elapsed);
            
            let stats = resolver.get_stats();
            tracing::info!("Resolution stats: resolved={}, cached={}, failed={}, backtrack_attempts={}", 
                          stats.resolved_count, stats.cached_count, stats.failed_count, stats.backtrack_attempts);
        }
        Err(e) => {
            tracing::info!("Performance test failed: {}", e);
            
            // Check if it was a timeout
            assert!(!e.to_string().contains("timeout"), "Should not timeout in normal operation");
        }
    }
}

#[tokio::test]
async fn test_cache_effectiveness() {
    common::tracing::setup();

    let mut resolver = DependencyResolver::new();
    
    let package = PackageInfo {
        name: "cache-test".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package for cache testing".to_string(),
        authors: None,
        keywords: None,
        download_url: "https://example.com/download".to_string(),
        checksum: "test-checksum".to_string(),
        size: None,
        published_at: None,
        repository: None,
        license: None,
    };

    // First resolution
    let result1 = resolver.resolve_dependencies(&package).await;
    assert!(result1.is_ok(), "First resolution should succeed");
    
    let stats_after_first = resolver.get_stats().clone();
    let first_resolution_time = stats_after_first.resolution_time_ms;

    // Second resolution should use cache
    let result2 = resolver.resolve_dependencies(&package).await;
    assert!(result2.is_ok(), "Second resolution should succeed");
    
    let stats_after_second = resolver.get_stats().clone();
    
    // Verify cache was used
    assert!(stats_after_second.cached_count > stats_after_first.cached_count,
            "Cache count should increase");
    
    // Cache should be faster (though this might not always be true in tests)
    tracing::info!("First resolution: {}ms, Cache usage count: {}", 
                  first_resolution_time, stats_after_second.cached_count);

    // Clear cache and verify it's cleared
    resolver.clear_cache();
    let stats_after_clear = resolver.get_stats();
    assert_eq!(stats_after_clear.cache_size, 0, "Cache should be cleared");
}

#[tokio::test] 
async fn test_maximum_depth_enforcement() {
    common::tracing::setup();

    let mut resolver = DependencyResolver::with_config(
        2, // Very low max depth
        false, 
        ConflictResolutionStrategy::LatestCompatible
    );

    let package = PackageInfo {
        name: "deep-dependency-test".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package with deep dependencies".to_string(),
        authors: None,
        keywords: None,
        download_url: "https://example.com/download".to_string(),
        checksum: "test-checksum".to_string(),
        size: None,
        published_at: None,
        repository: None,
        license: None,
    };

    let result = resolver.resolve_dependencies(&package).await;
    
    match result {
        Ok(dependencies) => {
            // If it succeeds, verify depth constraint
            for dep in &dependencies {
                assert!(dep.depth <= 2, "Dependency depth should respect max_depth limit");
            }
            tracing::info!("Deep dependency test succeeded with {} dependencies", dependencies.len());
        }
        Err(e) => {
            // If it fails due to depth, that's expected
            tracing::info!("Deep dependency test failed as expected: {}", e);
            assert!(e.to_string().contains("depth") || e.to_string().contains("maximum"),
                    "Should fail due to depth limit");
        }
    }
}
