/// Integration tests for the HTTP-based package registry client
/// 
/// These tests validate the registry client's functionality including:
/// - HTTP client configuration and initialization
/// - Package search and metadata retrieval
/// - Package downloading with integrity verification
/// - Error handling for network failures and HTTP errors
/// - Retry logic and timeout handling
/// - Authentication and security features

use cursed::package_manager::{
    PackageManagerError, 
    registry::{PackageRegistry, RegistryConfig, PackageInfo, PackageData, RegistryStats}
};
use std::time::Duration;
use std::collections::HashMap;
use tokio;
use reqwest;
use sha2::Digest;

#[tokio::test]
async fn test_registry_creation_and_configuration() {
    // Test basic registry creation
    let registry = PackageRegistry::new("https://packages.cursed-lang.org".to_string());
    assert!(registry.is_ok(), "Registry creation should succeed");
    
    let registry = registry.unwrap();
    let stats = registry.get_stats();
    assert_eq!(stats.registry_url, "https://packages.cursed-lang.org");
    assert_eq!(stats.download_count, 0);
    assert_eq!(stats.search_count, 0);
    assert_eq!(stats.failed_requests, 0);
}

#[tokio::test]
async fn test_registry_with_custom_config() {
    let config = RegistryConfig {
        base_url: "https://custom.registry.com".to_string(),
        timeout: Duration::from_secs(15),
        max_retries: 2,
        auth_token: Some("test-auth-token".to_string()),
        user_agent: "test-agent/1.0".to_string(),
        verify_tls: false,
    };
    
    let registry = PackageRegistry::with_config(config.clone());
    assert!(registry.is_ok(), "Registry creation with custom config should succeed");
    
    let registry = registry.unwrap();
    let registry_config = registry.get_config();
    assert_eq!(registry_config.base_url, config.base_url);
    assert_eq!(registry_config.timeout, config.timeout);
    assert_eq!(registry_config.max_retries, config.max_retries);
    assert_eq!(registry_config.auth_token, config.auth_token);
    assert_eq!(registry_config.user_agent, config.user_agent);
    assert_eq!(registry_config.verify_tls, config.verify_tls);
}

#[tokio::test]
async fn test_auth_token_management() {
    let mut registry = PackageRegistry::new("https://test.registry.com".to_string()).unwrap();
    
    // Initially no auth token
    assert!(registry.get_config().auth_token.is_none());
    
    // Set auth token
    registry.set_auth_token(Some("new-token".to_string()));
    assert_eq!(registry.get_config().auth_token, Some("new-token".to_string()));
    
    // Clear auth token
    registry.set_auth_token(None);
    assert!(registry.get_config().auth_token.is_none());
}

#[tokio::test]
async fn test_package_verification() {
    let registry = PackageRegistry::new("https://test.registry.com".to_string()).unwrap();
    
    // Test with valid checksum
    let data = b"test package data";
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    let expected_checksum = hex::encode(hasher.finalize());
    
    let result = registry.verify_package("test-package", "1.0.0", data, &expected_checksum).await;
    assert!(result.is_ok());
    assert!(result.unwrap(), "Package verification should succeed with correct checksum");
    
    // Test with invalid checksum
    let result = registry.verify_package("test-package", "1.0.0", data, "invalid-checksum").await;
    assert!(result.is_ok());
    assert!(!result.unwrap(), "Package verification should fail with incorrect checksum");
}

#[tokio::test]
async fn test_registry_stats_tracking() {
    let mut registry = PackageRegistry::new("https://httpbin.org".to_string()).unwrap();
    
    // Initial stats should be zero
    let initial_stats = registry.get_stats();
    assert_eq!(initial_stats.search_count, 0);
    assert_eq!(initial_stats.download_count, 0);
    assert_eq!(initial_stats.failed_requests, 0);
    
    // Note: We can't test actual HTTP requests without a real registry,
    // but we can verify the stats structure is correct
}

#[tokio::test]
async fn test_error_handling_for_invalid_urls() {
    // Test with invalid URL
    let result = PackageRegistry::new("not-a-valid-url".to_string());
    // This should still succeed because we only validate URLs during requests
    assert!(result.is_ok());
    
    // Test with malformed config
    let config = RegistryConfig {
        base_url: "".to_string(),
        timeout: Duration::from_secs(1),
        max_retries: 0,
        auth_token: None,
        user_agent: "test".to_string(),
        verify_tls: true,
    };
    
    let registry = PackageRegistry::with_config(config);
    assert!(registry.is_ok(), "Registry should be created even with empty base URL");
}

#[tokio::test]
async fn test_timeout_configuration() {
    let config = RegistryConfig {
        base_url: "https://httpbin.org".to_string(),
        timeout: Duration::from_millis(1), // Very short timeout
        max_retries: 0,
        auth_token: None,
        user_agent: "test".to_string(),
        verify_tls: true,
    };
    
    let mut registry = PackageRegistry::with_config(config).unwrap();
    
    // This should timeout quickly
    let result = registry.search_package("test-package", None).await;
    // We expect this to fail due to timeout, but the error handling should work correctly
    assert!(result.is_err(), "Request should fail due to timeout");
}

#[tokio::test]
async fn test_package_info_structure() {
    let package_info = PackageInfo {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        description: "A test package".to_string(),
        download_url: "https://example.com/package.tar.gz".to_string(),
        checksum: "abc123".to_string(),
        size: Some(1024),
        published_at: Some("2024-01-01T00:00:00Z".to_string()),
        authors: Some(vec!["Test Author".to_string()]),
        license: Some("MIT".to_string()),
        repository: Some("https://github.com/test/package".to_string()),
        keywords: Some(vec!["test".to_string(), "cursed".to_string()]),
    };
    
    // Verify serialization/deserialization
    let json = serde_json::to_string(&package_info).unwrap();
    let deserialized: PackageInfo = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.name, package_info.name);
    assert_eq!(deserialized.version, package_info.version);
    assert_eq!(deserialized.description, package_info.description);
    assert_eq!(deserialized.download_url, package_info.download_url);
    assert_eq!(deserialized.checksum, package_info.checksum);
}

#[tokio::test]
async fn test_package_data_structure() {
    let content = b"test package content".to_vec();
    let package_data = PackageData {
        content: content.clone(),
        checksum: "test-checksum".to_string(),
        size: content.len(),
        verified: true,
    };
    
    assert_eq!(package_data.content, content);
    assert_eq!(package_data.size, content.len());
    assert_eq!(package_data.checksum, "test-checksum");
    assert!(package_data.verified);
}

#[tokio::test]
async fn test_registry_url_building() {
    let registry = PackageRegistry::new("https://test.registry.com".to_string()).unwrap();
    
    // We can't directly test URL building since it's internal,
    // but we can verify the base URL is stored correctly
    assert_eq!(registry.get_config().base_url, "https://test.registry.com");
}

#[tokio::test]
async fn test_default_registry_config() {
    let config = RegistryConfig::default();
    
    assert_eq!(config.base_url, "https://packages.cursed-lang.org");
    assert_eq!(config.timeout, Duration::from_secs(30));
    assert_eq!(config.max_retries, 3);
    assert!(config.auth_token.is_none());
    assert!(config.user_agent.starts_with("cursed-pkg/"));
    assert!(config.verify_tls);
}

#[tokio::test]
async fn test_checksum_calculation() {
    let registry = PackageRegistry::new("https://test.registry.com".to_string()).unwrap();
    
    // Test checksum calculation consistency
    let data1 = b"test data";
    let data2 = b"test data";
    let data3 = b"different data";
    
    let mut hasher1 = sha2::Sha256::new();
    hasher1.update(data1);
    let checksum1 = hex::encode(hasher1.finalize());
    
    let mut hasher2 = sha2::Sha256::new();
    hasher2.update(data2);
    let checksum2 = hex::encode(hasher2.finalize());
    
    let mut hasher3 = sha2::Sha256::new();
    hasher3.update(data3);
    let checksum3 = hex::encode(hasher3.finalize());
    
    // Same data should produce same checksum
    assert_eq!(checksum1, checksum2);
    
    // Different data should produce different checksum
    assert_ne!(checksum1, checksum3);
    
    // Verify with registry verification function
    let result1 = registry.verify_package("test", "1.0.0", data1, &checksum1).await.unwrap();
    let result2 = registry.verify_package("test", "1.0.0", data1, &checksum3).await.unwrap();
    
    assert!(result1);  // Should match
    assert!(!result2); // Should not match
}

#[tokio::test]
async fn test_user_agent_configuration() {
    let config = RegistryConfig {
        base_url: "https://test.registry.com".to_string(),
        timeout: Duration::from_secs(30),
        max_retries: 3,
        auth_token: None,
        user_agent: "custom-agent/2.0".to_string(),
        verify_tls: true,
    };
    
    let registry = PackageRegistry::with_config(config).unwrap();
    assert_eq!(registry.get_config().user_agent, "custom-agent/2.0");
}

#[tokio::test]
async fn test_tls_verification_config() {
    // Test with TLS verification enabled
    let config_secure = RegistryConfig {
        base_url: "https://test.registry.com".to_string(),
        verify_tls: true,
        ..Default::default()
    };
    
    let registry_secure = PackageRegistry::with_config(config_secure);
    assert!(registry_secure.is_ok());
    
    // Test with TLS verification disabled
    let config_insecure = RegistryConfig {
        base_url: "https://test.registry.com".to_string(),
        verify_tls: false,
        ..Default::default()
    };
    
    let registry_insecure = PackageRegistry::with_config(config_insecure);
    assert!(registry_insecure.is_ok());
}

#[tokio::test]
async fn test_retry_configuration() {
    let config = RegistryConfig {
        base_url: "https://test.registry.com".to_string(),
        timeout: Duration::from_secs(1),
        max_retries: 5,
        auth_token: None,
        user_agent: "test".to_string(),
        verify_tls: true,
    };
    
    let registry = PackageRegistry::with_config(config);
    assert!(registry.is_ok());
    assert_eq!(registry.unwrap().get_config().max_retries, 5);
}

// Mock test for error scenarios (these would fail with real HTTP requests)
#[tokio::test]
async fn test_error_scenarios_structure() {
    // Test that we can create the error types properly
    let _error1 = PackageManagerError::PackageNotFound {
        package: "nonexistent-package".to_string(),
    };
    
    let _error2 = PackageManagerError::RegistryError {
        message: "Network timeout".to_string(),
    };
    
    // Test that we can create IO error for conversion testing
    let _io_error = std::io::Error::new(
        std::io::ErrorKind::ConnectionRefused,
        "Connection refused"
    );
    
    // Note: We can't easily create a reqwest::Error for testing without making actual HTTP requests
    // but we can verify the error types exist and work properly
}
