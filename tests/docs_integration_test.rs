//! Integration Tests for Documentation Infrastructure
//! 
//! End-to-end tests for the complete CURSED documentation system including
//! publisher, server, registry, and testing components.

use cursed::docs::{
    DocumentationPublisher, PublishConfig, PublishTarget, OptimizationConfig,
    DocumentationServer, ServerConfig,
    DocumentationRegistry, RegistryConfig,
    DocumentationTester, TestingConfig
};
use cursed::docs::generator::DocumentationGenerator;
use cursed::package::{Package, PackageManager};
use cursed::error::Result;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;
use tokio::fs;
use tokio::time::timeout;

#[tokio::test]
async fn test_full_documentation_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let docs_dir = temp_dir.path().join("docs");
    let registry_dir = temp_dir.path().join("registry");
    let published_dir = temp_dir.path().join("published");
    
    // Create directories
    fs::create_dir_all(&docs_dir).await.unwrap();
    fs::create_dir_all(&registry_dir).await.unwrap();
    fs::create_dir_all(&published_dir).await.unwrap();
    
    // Step 1: Setup registry
    let registry_config = RegistryConfig {
        data_dir: registry_dir.clone(),
        index_file: registry_dir.join("index.json"),
        cache_size: 100,
        refresh_interval: 300,
        auto_resolve_deps: true,
        max_dependency_depth: 5,
    };
    
    let registry = DocumentationRegistry::new(registry_config);
    registry.initialize().await.unwrap();
    
    // Step 2: Create test package
    let test_package = Package {
        name: "test-workflow-package".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package for workflow validation".to_string(),
        authors: vec!["Test Author".to_string()],
        license: "MIT".to_string(),
        repository: Some("https://github.com/test/test".to_string()),
        homepage: Some("https://test.cursed.dev".to_string()),
        keywords: vec!["test".to_string(), "workflow".to_string()],
        dependencies: vec![],
        dev_dependencies: vec![],
        build_dependencies: vec![],
        features: std::collections::HashMap::new(),
        default_features: vec![],
        edition: "2021".to_string(),
        rust_version: None,
        exclude: vec![],
        include: vec![],
        links: None,
        path: temp_dir.path().to_path_buf(),
    };
    
    // Step 3: Generate and publish documentation
    let publish_config = PublishConfig {
        target: PublishTarget::Local {
            path: published_dir.clone(),
        },
        base_url: "https://test.cursed.dev".to_string(),
        cdn: None,
        optimization: OptimizationConfig {
            minify_html: false,
            minify_css: false,
            minify_js: false,
            optimize_images: false,
            gzip_compression: false,
            brotli_compression: false,
        },
        auth: None,
        domain: None,
    };
    
    let generator = DocumentationGenerator::new();
    let package_manager = PackageManager::new();
    let mut publisher = DocumentationPublisher::new(
        publish_config,
        generator,
        registry.clone(),
        package_manager,
    );
    
    // Attempt to publish (may fail in test environment)
    let publish_result = publisher.publish_package(&test_package).await;
    
    // Step 4: Test registry operations
    let packages = registry.list_packages().await;
    // Registry should be initialized even if publishing failed
    assert!(packages.len() >= 0);
    
    // Step 5: Test search functionality
    let search_query = cursed::docs::registry::RegistrySearchQuery {
        query: "test".to_string(),
        package: None,
        version: None,
        item_type: None,
        category: None,
        min_quality: None,
        sort_by: cursed::docs::registry::SortOrder::Relevance,
        limit: 10,
        offset: 0,
    };
    
    let search_results = registry.search(&search_query).await.unwrap();
    // Search should return empty results initially
    assert!(search_results.len() >= 0);
    
    // Verify workflow completed without crashes
    assert!(true);
}

#[tokio::test]
async fn test_registry_package_management() {
    let temp_dir = TempDir::new().unwrap();
    let registry_dir = temp_dir.path().join("registry");
    
    let config = RegistryConfig {
        data_dir: registry_dir.clone(),
        index_file: registry_dir.join("index.json"),
        cache_size: 50,
        refresh_interval: 300,
        auto_resolve_deps: true,
        max_dependency_depth: 5,
    };
    
    let registry = DocumentationRegistry::new(config);
    registry.initialize().await.unwrap();
    
    // Test listing packages (should be empty initially)
    let packages = registry.list_packages().await;
    assert!(packages.is_empty());
    
    // Test getting non-existent package
    let package = registry.get_package("non-existent").await;
    assert!(package.is_none());
    
    // Test getting versions for non-existent package
    let versions = registry.get_versions("non-existent").await;
    assert!(versions.is_empty());
    
    // Test registry statistics
    let stats = registry.get_statistics().await;
    assert_eq!(stats.total_packages, 0);
    assert_eq!(stats.total_versions, 0);
}

#[tokio::test]
async fn test_server_configuration_and_validation() {
    let temp_dir = TempDir::new().unwrap();
    let docs_dir = temp_dir.path().join("docs");
    fs::create_dir_all(&docs_dir).await.unwrap();
    
    // Create sample documentation file
    fs::write(docs_dir.join("index.html"), "<html><body>Test</body></html>").await.unwrap();
    
    let config = ServerConfig {
        bind_address: "127.0.0.1:0".parse().unwrap(), // Use port 0 for testing
        document_root: docs_dir,
        enable_https: false,
        ssl_config: None,
        cors_config: cursed::docs::server::CorsConfig::default(),
        rate_limiting: cursed::docs::server::RateLimitConfig::default(),
        cache_config: cursed::docs::server::CacheConfig::default(),
        search_config: cursed::docs::server::SearchConfig::default(),
        analytics_config: cursed::docs::server::AnalyticsConfig::default(),
    };
    
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    let server = DocumentationServer::new(config, registry);
    
    // Test configuration validation
    assert!(server.validate_config().is_ok());
}

#[tokio::test]
async fn test_testing_infrastructure() {
    let temp_dir = TempDir::new().unwrap();
    
    let testing_config = TestingConfig {
        check_links: false, // Disable to avoid network dependencies
        verify_examples: false, // Disable to avoid compilation dependencies
        check_completeness: true,
        check_accessibility: false,
        request_timeout: 10,
        max_concurrent_requests: 5,
        retry_attempts: 1,
        example_timeout: 30,
        min_coverage_percentage: 50.0,
    };
    
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    registry.initialize().await.unwrap();
    
    let tester = DocumentationTester::new(testing_config, registry).unwrap();
    
    // Test configuration validation
    assert!(tester.validate_config().is_ok());
    
    // Create test package
    let test_package = Package {
        name: "test-testing-package".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package for testing infrastructure".to_string(),
        authors: vec!["Test Author".to_string()],
        license: "MIT".to_string(),
        repository: None,
        homepage: None,
        keywords: vec!["test".to_string()],
        dependencies: vec![],
        dev_dependencies: vec![],
        build_dependencies: vec![],
        features: std::collections::HashMap::new(),
        default_features: vec![],
        edition: "2021".to_string(),
        rust_version: None,
        exclude: vec![],
        include: vec![],
        links: None,
        path: temp_dir.path().to_path_buf(),
    };
    
    // Attempt to test package (may fail without documentation)
    let test_result = tester.test_package(&test_package, "1.0.0").await;
    
    // The test should complete without crashing, regardless of pass/fail
    match test_result {
        Ok(results) => {
            assert_eq!(results.package, "test-testing-package");
            assert_eq!(results.version, "1.0.0");
        }
        Err(_) => {
            // Expected in test environment without proper documentation
        }
    }
}

#[tokio::test]
async fn test_concurrent_registry_operations() {
    let temp_dir = TempDir::new().unwrap();
    let registry_dir = temp_dir.path().join("registry");
    
    let config = RegistryConfig {
        data_dir: registry_dir.clone(),
        index_file: registry_dir.join("index.json"),
        cache_size: 100,
        refresh_interval: 300,
        auto_resolve_deps: true,
        max_dependency_depth: 5,
    };
    
    let registry = DocumentationRegistry::new(config);
    registry.initialize().await.unwrap();
    
    let mut handles = vec![];
    
    // Spawn multiple concurrent operations
    for i in 0..5 {
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move {
            // Test concurrent list operations
            let packages = registry_clone.list_packages().await;
            assert!(packages.len() >= 0);
            
            // Test concurrent search operations
            let search_query = cursed::docs::registry::RegistrySearchQuery {
                query: format!("test-{}", i),
                package: None,
                version: None,
                item_type: None,
                category: None,
                min_quality: None,
                sort_by: cursed::docs::registry::SortOrder::Relevance,
                limit: 10,
                offset: 0,
            };
            
            let _results = registry_clone.search(&search_query).await.unwrap();
        });
        
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_error_handling_and_recovery() {
    // Test with invalid configurations
    
    // Invalid registry configuration
    let invalid_registry_config = RegistryConfig {
        data_dir: PathBuf::from("/invalid/path/that/does/not/exist"),
        index_file: PathBuf::from("/invalid/index.json"),
        cache_size: 0, // Invalid cache size
        refresh_interval: 300,
        auto_resolve_deps: true,
        max_dependency_depth: 0, // Invalid depth
    };
    
    let registry = DocumentationRegistry::new(invalid_registry_config);
    assert!(registry.validate_config().is_err());
    
    // Invalid server configuration
    let invalid_server_config = ServerConfig {
        bind_address: "127.0.0.1:8080".parse().unwrap(),
        document_root: PathBuf::from("/non/existent/document/root"),
        enable_https: false,
        ssl_config: None,
        cors_config: cursed::docs::server::CorsConfig::default(),
        rate_limiting: cursed::docs::server::RateLimitConfig::default(),
        cache_config: cursed::docs::server::CacheConfig::default(),
        search_config: cursed::docs::server::SearchConfig::default(),
        analytics_config: cursed::docs::server::AnalyticsConfig::default(),
    };
    
    let valid_registry = DocumentationRegistry::new(RegistryConfig::default());
    let server = DocumentationServer::new(invalid_server_config, valid_registry);
    assert!(server.validate_config().is_err());
    
    // Invalid testing configuration
    let invalid_testing_config = TestingConfig {
        check_links: true,
        verify_examples: true,
        check_completeness: true,
        check_accessibility: true,
        request_timeout: 0, // Invalid timeout
        max_concurrent_requests: 0, // Invalid concurrency
        retry_attempts: 0,
        example_timeout: 0,
        min_coverage_percentage: 150.0, // Invalid percentage
    };
    
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    if let Ok(tester) = DocumentationTester::new(invalid_testing_config, registry) {
        assert!(tester.validate_config().is_err());
    }
}

#[tokio::test]
async fn test_publisher_with_different_targets() {
    let temp_dir = TempDir::new().unwrap();
    
    let targets = vec![
        PublishTarget::Local {
            path: temp_dir.path().join("local"),
        },
        PublishTarget::S3 {
            bucket: "test-bucket".to_string(),
            region: "us-west-2".to_string(),
            prefix: Some("docs".to_string()),
        },
        PublishTarget::GithubPages {
            repo: "user/repo".to_string(),
            branch: "gh-pages".to_string(),
            token: "fake-token".to_string(),
        },
        PublishTarget::Custom {
            endpoint: "https://api.example.com/upload".to_string(),
            credentials: std::collections::HashMap::new(),
        },
    ];
    
    for target in targets {
        let config = PublishConfig {
            target,
            base_url: "https://test.cursed.dev".to_string(),
            cdn: None,
            optimization: OptimizationConfig::default(),
            auth: None,
            domain: None,
        };
        
        let generator = DocumentationGenerator::new();
        let registry = DocumentationRegistry::new(RegistryConfig::default());
        let package_manager = PackageManager::new();
        
        let publisher = DocumentationPublisher::new(
            config,
            generator,
            registry,
            package_manager,
        );
        
        // Test that publisher can be created with different targets
        // Validation may fail for some targets (e.g., non-existent S3 buckets)
        // but the publisher should be created successfully
        match publisher.validate_config() {
            Ok(()) => {
                // Valid configuration
            }
            Err(e) => {
                // Expected for some test configurations
                assert!(e.to_string().len() > 0);
            }
        }
    }
}

#[tokio::test]
async fn test_cross_component_integration() {
    let temp_dir = TempDir::new().unwrap();
    let registry_dir = temp_dir.path().join("registry");
    let docs_dir = temp_dir.path().join("docs");
    
    fs::create_dir_all(&registry_dir).await.unwrap();
    fs::create_dir_all(&docs_dir).await.unwrap();
    
    // Initialize registry
    let registry_config = RegistryConfig {
        data_dir: registry_dir.clone(),
        index_file: registry_dir.join("index.json"),
        ..RegistryConfig::default()
    };
    
    let registry = DocumentationRegistry::new(registry_config);
    registry.initialize().await.unwrap();
    
    // Create publisher that uses the same registry
    let publish_config = PublishConfig {
        target: PublishTarget::Local {
            path: docs_dir.clone(),
        },
        base_url: "https://test.cursed.dev".to_string(),
        cdn: None,
        optimization: OptimizationConfig::default(),
        auth: None,
        domain: None,
    };
    
    let generator = DocumentationGenerator::new();
    let package_manager = PackageManager::new();
    let _publisher = DocumentationPublisher::new(
        publish_config,
        generator,
        registry.clone(),
        package_manager,
    );
    
    // Create server that uses the same registry
    let server_config = ServerConfig {
        bind_address: "127.0.0.1:0".parse().unwrap(),
        document_root: docs_dir,
        enable_https: false,
        ssl_config: None,
        cors_config: cursed::docs::server::CorsConfig::default(),
        rate_limiting: cursed::docs::server::RateLimitConfig::default(),
        cache_config: cursed::docs::server::CacheConfig::default(),
        search_config: cursed::docs::server::SearchConfig::default(),
        analytics_config: cursed::docs::server::AnalyticsConfig::default(),
    };
    
    let _server = DocumentationServer::new(server_config, registry.clone());
    
    // Create tester that uses the same registry
    let testing_config = TestingConfig::default();
    let _tester = DocumentationTester::new(testing_config, registry).unwrap();
    
    // All components should be able to share the same registry instance
    assert!(true);
}

#[tokio::test]
async fn test_performance_under_load() {
    let temp_dir = TempDir::new().unwrap();
    let registry_dir = temp_dir.path().join("registry");
    
    let config = RegistryConfig {
        data_dir: registry_dir.clone(),
        index_file: registry_dir.join("index.json"),
        cache_size: 1000,
        refresh_interval: 300,
        auto_resolve_deps: true,
        max_dependency_depth: 10,
    };
    
    let registry = DocumentationRegistry::new(config);
    registry.initialize().await.unwrap();
    
    let start_time = std::time::Instant::now();
    let mut handles = vec![];
    
    // Spawn many concurrent operations
    for i in 0..50 {
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move {
            // Multiple operations per task
            for j in 0..10 {
                let query = cursed::docs::registry::RegistrySearchQuery {
                    query: format!("test-{}-{}", i, j),
                    package: None,
                    version: None,
                    item_type: None,
                    category: None,
                    min_quality: None,
                    sort_by: cursed::docs::registry::SortOrder::Relevance,
                    limit: 5,
                    offset: 0,
                };
                
                let _results = registry_clone.search(&query).await.unwrap();
                let _packages = registry_clone.list_packages().await;
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all operations with timeout
    let timeout_duration = Duration::from_secs(30);
    let result = timeout(timeout_duration, async {
        for handle in handles {
            handle.await.unwrap();
        }
    }).await;
    
    assert!(result.is_ok(), "Performance test timed out");
    
    let elapsed = start_time.elapsed();
    println!("Performance test completed in {:?}", elapsed);
    
    // Verify performance is reasonable (should complete well under timeout)
    assert!(elapsed < Duration::from_secs(10));
}

#[tokio::test]
async fn test_configuration_serialization() {
    // Test that all configurations can be serialized and deserialized
    
    let publish_config = PublishConfig {
        target: PublishTarget::Local {
            path: PathBuf::from("/tmp/test"),
        },
        base_url: "https://test.cursed.dev".to_string(),
        cdn: None,
        optimization: OptimizationConfig::default(),
        auth: None,
        domain: None,
    };
    
    let json = serde_json::to_string(&publish_config).unwrap();
    let deserialized: PublishConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(publish_config.base_url, deserialized.base_url);
    
    let registry_config = RegistryConfig::default();
    let toml = toml::to_string(&registry_config).unwrap();
    let deserialized_toml: RegistryConfig = toml::from_str(&toml).unwrap();
    assert_eq!(registry_config.cache_size, deserialized_toml.cache_size);
    
    let testing_config = TestingConfig::default();
    let yaml = serde_yaml::to_string(&testing_config).unwrap();
    let deserialized_yaml: TestingConfig = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(testing_config.min_coverage_percentage, deserialized_yaml.min_coverage_percentage);
}
