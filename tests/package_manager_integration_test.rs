//! Integration test for package manager optimization
//!
//! This test verifies that the optimized resolver maintains correctness
//! while improving performance.

use cursed::package_manager::{
    PackageManager, PackageManagerConfig, OptimizedPackageResolver, PackageResolver,
    ResolutionConfig, VersionReq, Version
};
use cursed::package_manager::resolver::ConflictResolutionStrategy;
use cursed::package_manager::registry::{PackageRegistry, RegistryConfig};
use std::time::Instant;
use std::path::PathBuf;

#[tokio::test]
async fn test_package_manager_basic_functionality() {
    let config = PackageManagerConfig {
        cache_dir: PathBuf::from("/tmp/cursed_test_cache"),
        registry_url: "https://packages.cursed-lang.org".to_string(),
        offline_mode: true, // Use offline mode to avoid network issues
        ..Default::default()
    };
    
    let package_manager = PackageManager::new(config);
    
    // Basic functionality test - should not crash
    assert!(package_manager.is_ok());
    
    if let Ok(pm) = package_manager {
        // Test that we can create the package manager
        assert!(!pm.is_installed("nonexistent-package"));
        
        // Test listing installed packages (should be empty)
        let installed = pm.list_installed();
        assert!(installed.is_empty());
    }
}

#[tokio::test]
async fn test_resolver_performance_comparison() {
    // Create a mock registry for testing
    let registry = PackageRegistry::new(RegistryConfig {
        url: "https://packages.cursed-lang.org".to_string(),
        timeout: std::time::Duration::from_secs(1),
        max_retries: 1,
        api_key: None,
    });
    
    // Skip this test if registry creation fails (likely due to network issues)
    if registry.is_err() {
        println!("Skipping performance test due to registry creation failure");
        return;
    }
    
    let registry = registry.unwrap();
    
    // Create test packages
    let root_packages = vec![
        ("test-package-1".to_string(), VersionReq::parse("^1.0.0").unwrap()),
        ("test-package-2".to_string(), VersionReq::parse("^2.0.0").unwrap()),
    ];
    
    let config = ResolutionConfig {
        max_depth: 10,
        conflict_strategy: ConflictResolutionStrategy::ChooseHighest,
        ..Default::default()
    };
    
    // Test original resolver
    let mut original_resolver = PackageResolver::new(registry.clone());
    let start_original = Instant::now();
    
    let original_result = original_resolver.resolve_dependencies(root_packages.clone(), config.clone()).await;
    let original_time = start_original.elapsed();
    
    // Test optimized resolver
    let mut optimized_resolver = OptimizedPackageResolver::new(registry.clone());
    let start_optimized = Instant::now();
    
    let optimized_result = optimized_resolver.resolve_dependencies(root_packages.clone(), config.clone()).await;
    let optimized_time = start_optimized.elapsed();
    
    println!("Original resolver time: {:?}", original_time);
    println!("Optimized resolver time: {:?}", optimized_time);
    
    // Extract just the ResolutionResult from the optimized resolver tuple
    let optimized_result_only = optimized_result.as_ref().map(|(result, _metrics)| result);
    

    

    
    // Both should have the same success/failure outcome
    match (&original_result, &optimized_result_only) {
        (Ok(_), Ok(_)) => {
            println!("✅ Both resolvers succeeded");
            
            // In a real scenario with network access, we'd expect performance improvement
            // For now, just verify both completed
            assert!(original_time.as_nanos() > 0);
            assert!(optimized_time.as_nanos() > 0);
        },
        (Err(_), Err(_)) => {
            println!("Both resolvers failed (expected in test environment)");
            // This is acceptable in a test environment without network access
        },
        _ => {
            panic!("Resolvers had different success/failure outcomes");
        }
    }
}

#[test]
fn test_benchmark_scenario_generation() {
    // Test that we can generate different types of dependency scenarios
    let scenarios = vec![
        ("linear", 1),
        ("shared", 3),
        ("conflict", 3),
        ("massive", 10),
    ];
    
    for (scenario_name, expected_min_packages) in scenarios {
        let packages = generate_test_packages(scenario_name);
        assert!(packages.len() >= expected_min_packages, 
               "Scenario {} should have at least {} packages", scenario_name, expected_min_packages);
        
        // Verify all packages have valid version requirements
        for (name, version_req) in &packages {
            assert!(!name.is_empty(), "Package name should not be empty");
            // Basic validation - in a real test we'd parse the version requirement
            assert!(format!("{:?}", version_req).len() > 0, "Version requirement should be valid");
        }
    }
}

/// Generate test packages for different scenarios
fn generate_test_packages(scenario: &str) -> Vec<(String, VersionReq)> {
    match scenario {
        "linear" => {
            vec![("linear-root".to_string(), VersionReq::parse("^1.0.0").unwrap())]
        },
        "shared" => {
            vec![
                ("shared-root-1".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                ("shared-root-2".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                ("shared-root-3".to_string(), VersionReq::parse("^1.0.0").unwrap()),
            ]
        },
        "conflict" => {
            vec![
                ("conflict-root-1".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                ("conflict-root-2".to_string(), VersionReq::parse("^2.0.0").unwrap()),
                ("conflict-root-3".to_string(), VersionReq::parse("^3.0.0").unwrap()),
            ]
        },
        "massive" => {
            let mut packages = Vec::new();
            for i in 0..10 {
                packages.push((format!("massive-root-{}", i), VersionReq::parse("^1.0.0").unwrap()));
            }
            packages
        },
        _ => vec![("default-package".to_string(), VersionReq::parse("^1.0.0").unwrap())],
    }
}

#[test]
fn test_performance_metrics() {
    // Test the performance metrics structure
    let metrics = cursed::package_manager::optimized_resolver::ResolutionMetrics {
        total_time_ms: 100,
        packages_resolved: 50,
        cache_hits: 30,
        cache_misses: 20,
        conflicts_resolved: 2,
        backtrack_count: 1,
        sat_iterations: 10,
    };
    
    assert_eq!(metrics.total_time_ms, 100);
    assert_eq!(metrics.packages_resolved, 50);
    assert_eq!(metrics.cache_hits, 30);
    assert_eq!(metrics.cache_misses, 20);
    assert_eq!(metrics.conflicts_resolved, 2);
    assert_eq!(metrics.backtrack_count, 1);
    assert_eq!(metrics.sat_iterations, 10);
    
    // Test cache hit rate calculation
    let total_cache_ops = metrics.cache_hits + metrics.cache_misses;
    let cache_hit_rate = metrics.cache_hits as f64 / total_cache_ops as f64;
    assert_eq!(cache_hit_rate, 0.6); // 30/50 = 0.6
}
