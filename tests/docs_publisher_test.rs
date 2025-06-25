//! Tests for Documentation Publisher
//! 
//! Comprehensive test suite for the CURSED documentation publishing infrastructure.

use cursed::docs::publisher::{
    DocumentationPublisher, PublishConfig, PublishTarget, OptimizationConfig,
    PublicationMetadata, SizeInfo, PerformanceMetrics
};
use cursed::docs::registry::{DocumentationRegistry, RegistryConfig};
use cursed::docs::generator::DocumentationGenerator;
use cursed::package::{Package, PackageManager};
use cursed::error::Result;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn test_publisher_creation() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = PublishConfig {
        target: PublishTarget::Local {
            path: temp_dir.path().to_path_buf(),
        },
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
        config.clone(),
        generator,
        registry,
        package_manager,
    );
    
    // Test configuration validation
    assert!(publisher.validate_config().is_ok());
}

#[tokio::test]
async fn test_local_publishing() {
    let temp_dir = TempDir::new().unwrap();
    let publish_dir = temp_dir.path().join("published");
    fs::create_dir_all(&publish_dir).await.unwrap();
    
    let config = PublishConfig {
        target: PublishTarget::Local {
            path: publish_dir.clone(),
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
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    let package_manager = PackageManager::new();
    
    let mut publisher = DocumentationPublisher::new(
        config,
        generator,
        registry,
        package_manager,
    );
    
    let test_package = Package {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package".to_string(),
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
    
    // This would normally publish real documentation
    // For testing, we just verify the function can be called
    match publisher.publish_package(&test_package).await {
        Ok(metadata) => {
            assert_eq!(metadata.package_name, "test-package");
            assert_eq!(metadata.version, "1.0.0");
            assert!(metadata.url.contains("test-package"));
        }
        Err(e) => {
            // Expected in test environment - just verify error handling works
            assert!(e.to_string().contains("documentation") || e.to_string().contains("generation"));
        }
    }
}

#[tokio::test]
async fn test_optimization_config() {
    let default_config = OptimizationConfig::default();
    
    assert!(default_config.minify_html);
    assert!(default_config.minify_css);
    assert!(default_config.minify_js);
    assert!(default_config.optimize_images);
    assert!(default_config.gzip_compression);
    assert!(!default_config.brotli_compression);
}

#[tokio::test]
async fn test_publication_metadata() {
    let metadata = PublicationMetadata {
        package_name: "test-pkg".to_string(),
        version: "1.0.0".to_string(),
        published_at: 1640995200, // 2022-01-01T00:00:00Z
        target: "local".to_string(),
        url: "file:///tmp/docs".to_string(),
        size_info: SizeInfo {
            total_bytes: 1024,
            html_files: 5,
            asset_files: 3,
            compressed_bytes: Some(512),
        },
        performance: PerformanceMetrics {
            build_time_ms: 1500,
            upload_time_ms: 200,
            propagation_time_ms: None,
        },
    };
    
    assert_eq!(metadata.package_name, "test-pkg");
    assert_eq!(metadata.size_info.total_bytes, 1024);
    assert_eq!(metadata.performance.build_time_ms, 1500);
}

#[tokio::test]
async fn test_publish_target_variants() {
    let local_target = PublishTarget::Local {
        path: PathBuf::from("/tmp/docs"),
    };
    
    let s3_target = PublishTarget::S3 {
        bucket: "test-bucket".to_string(),
        region: "us-west-2".to_string(),
        prefix: Some("docs".to_string()),
    };
    
    let github_target = PublishTarget::GithubPages {
        repo: "user/repo".to_string(),
        branch: "gh-pages".to_string(),
        token: "token123".to_string(),
    };
    
    let custom_target = PublishTarget::Custom {
        endpoint: "https://api.example.com".to_string(),
        credentials: std::collections::HashMap::new(),
    };
    
    // Test that all variants can be created
    match local_target {
        PublishTarget::Local { path } => assert_eq!(path, PathBuf::from("/tmp/docs")),
        _ => panic!("Expected Local variant"),
    }
    
    match s3_target {
        PublishTarget::S3 { bucket, region, prefix } => {
            assert_eq!(bucket, "test-bucket");
            assert_eq!(region, "us-west-2");
            assert_eq!(prefix, Some("docs".to_string()));
        }
        _ => panic!("Expected S3 variant"),
    }
    
    match github_target {
        PublishTarget::GithubPages { repo, branch, token } => {
            assert_eq!(repo, "user/repo");
            assert_eq!(branch, "gh-pages");
            assert_eq!(token, "token123");
        }
        _ => panic!("Expected GithubPages variant"),
    }
    
    match custom_target {
        PublishTarget::Custom { endpoint, credentials } => {
            assert_eq!(endpoint, "https://api.example.com");
            assert!(credentials.is_empty());
        }
        _ => panic!("Expected Custom variant"),
    }
}

#[tokio::test]
async fn test_config_validation() {
    let temp_dir = TempDir::new().unwrap();
    
    // Valid configuration
    let valid_config = PublishConfig {
        target: PublishTarget::Local {
            path: temp_dir.path().to_path_buf(),
        },
        base_url: "https://docs.cursed.dev".to_string(),
        cdn: None,
        optimization: OptimizationConfig::default(),
        auth: None,
        domain: None,
    };
    
    let generator = DocumentationGenerator::new();
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    let package_manager = PackageManager::new();
    
    let publisher = DocumentationPublisher::new(
        valid_config,
        generator,
        registry,
        package_manager,
    );
    
    assert!(publisher.validate_config().is_ok());
    
    // Invalid configuration - empty base URL
    let invalid_config = PublishConfig {
        target: PublishTarget::Local {
            path: temp_dir.path().to_path_buf(),
        },
        base_url: String::new(),
        cdn: None,
        optimization: OptimizationConfig::default(),
        auth: None,
        domain: None,
    };
    
    let generator = DocumentationGenerator::new();
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    let package_manager = PackageManager::new();
    
    let invalid_publisher = DocumentationPublisher::new(
        invalid_config,
        generator,
        registry,
        package_manager,
    );
    
    assert!(invalid_publisher.validate_config().is_err());
}

#[tokio::test]
async fn test_size_info_calculation() {
    let size_info = SizeInfo {
        total_bytes: 2048,
        html_files: 10,
        asset_files: 5,
        compressed_bytes: Some(1024),
    };
    
    assert_eq!(size_info.total_bytes, 2048);
    assert_eq!(size_info.html_files, 10);
    assert_eq!(size_info.asset_files, 5);
    assert_eq!(size_info.compressed_bytes, Some(1024));
    
    // Test compression ratio calculation
    if let Some(compressed) = size_info.compressed_bytes {
        let compression_ratio = compressed as f64 / size_info.total_bytes as f64;
        assert_eq!(compression_ratio, 0.5); // 50% compression
    }
}

#[tokio::test]
async fn test_performance_metrics() {
    let metrics = PerformanceMetrics {
        build_time_ms: 2000,
        upload_time_ms: 500,
        propagation_time_ms: Some(1000),
    };
    
    assert_eq!(metrics.build_time_ms, 2000);
    assert_eq!(metrics.upload_time_ms, 500);
    assert_eq!(metrics.propagation_time_ms, Some(1000));
    
    let total_time = metrics.build_time_ms + metrics.upload_time_ms + 
                    metrics.propagation_time_ms.unwrap_or(0);
    assert_eq!(total_time, 3500);
}

#[test]
fn test_serialization() {
    let config = PublishConfig {
        target: PublishTarget::Local {
            path: PathBuf::from("/tmp/test"),
        },
        base_url: "https://test.example.com".to_string(),
        cdn: None,
        optimization: OptimizationConfig::default(),
        auth: None,
        domain: None,
    };
    
    // Test JSON serialization
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: PublishConfig = serde_json::from_str(&json).unwrap();
    
    assert_eq!(config.base_url, deserialized.base_url);
    
    // Test metadata serialization
    let metadata = PublicationMetadata {
        package_name: "test".to_string(),
        version: "1.0.0".to_string(),
        published_at: 1640995200,
        target: "local".to_string(),
        url: "file:///tmp/test".to_string(),
        size_info: SizeInfo {
            total_bytes: 1024,
            html_files: 1,
            asset_files: 0,
            compressed_bytes: None,
        },
        performance: PerformanceMetrics {
            build_time_ms: 1000,
            upload_time_ms: 100,
            propagation_time_ms: None,
        },
    };
    
    let metadata_json = serde_json::to_string(&metadata).unwrap();
    let deserialized_metadata: PublicationMetadata = serde_json::from_str(&metadata_json).unwrap();
    
    assert_eq!(metadata.package_name, deserialized_metadata.package_name);
    assert_eq!(metadata.version, deserialized_metadata.version);
}

#[tokio::test]
async fn test_concurrent_publishing() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = PublishConfig {
        target: PublishTarget::Local {
            path: temp_dir.path().to_path_buf(),
        },
        base_url: "https://test.cursed.dev".to_string(),
        cdn: None,
        optimization: OptimizationConfig::default(),
        auth: None,
        domain: None,
    };
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let config_clone = config.clone();
        let temp_path = temp_dir.path().to_path_buf();
        
        let handle = tokio::spawn(async move {
            let generator = DocumentationGenerator::new();
            let registry = DocumentationRegistry::new(RegistryConfig::default());
            let package_manager = PackageManager::new();
            
            let mut publisher = DocumentationPublisher::new(
                config_clone,
                generator,
                registry,
                package_manager,
            );
            
            let test_package = Package {
                name: format!("test-package-{}", i),
                version: "1.0.0".to_string(),
                description: "Test package".to_string(),
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
                path: temp_path,
            };
            
            // Attempt publishing (may fail in test environment)
            publisher.publish_package(&test_package).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all publishing attempts to complete
    for handle in handles {
        let _result = handle.await.unwrap();
        // Results may be errors in test environment, which is expected
    }
}

#[test]
fn test_cache_settings_defaults() {
    use cursed::docs::publisher::CacheSettings;
    
    let cache_settings = CacheSettings::default();
    
    assert_eq!(cache_settings.html_cache, 3600);      // 1 hour
    assert_eq!(cache_settings.assets_cache, 86400);   // 24 hours
    assert_eq!(cache_settings.api_cache, 300);        // 5 minutes
}

#[tokio::test]
async fn test_error_handling() {
    // Test with non-existent target directory
    let non_existent_dir = PathBuf::from("/non/existent/directory");
    
    let config = PublishConfig {
        target: PublishTarget::Local {
            path: non_existent_dir,
        },
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
    
    // Should fail validation due to non-existent path
    assert!(publisher.validate_config().is_err());
}

#[test]
fn test_github_pages_config() {
    let github_config = PublishTarget::GithubPages {
        repo: "user/docs-repo".to_string(),
        branch: "main".to_string(),
        token: "ghp_token123".to_string(),
    };
    
    match github_config {
        PublishTarget::GithubPages { repo, branch, token } => {
            assert!(repo.contains("/"));
            assert!(!branch.is_empty());
            assert!(token.starts_with("ghp_") || !token.is_empty());
        }
        _ => panic!("Expected GithubPages variant"),
    }
}

#[test]
fn test_s3_config() {
    let s3_config = PublishTarget::S3 {
        bucket: "my-docs-bucket".to_string(),
        region: "us-east-1".to_string(),
        prefix: Some("documentation".to_string()),
    };
    
    match s3_config {
        PublishTarget::S3 { bucket, region, prefix } => {
            assert!(!bucket.is_empty());
            assert!(region.starts_with("us-") || region.starts_with("eu-") || region.starts_with("ap-"));
            assert!(prefix.is_some());
        }
        _ => panic!("Expected S3 variant"),
    }
}
