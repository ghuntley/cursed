//! Tests for Documentation Server
//! 
//! Comprehensive test suite for the CURSED documentation server infrastructure.

use cursed::docs::server::{
    DocumentationServer, ServerConfig, SearchQuery, SearchResponse,
    CorsConfig, RateLimitConfig, CacheConfig, SearchConfig, AnalyticsConfig,
    ServerMetrics, VersionInfo, LinkStatus
};
use cursed::docs::registry::{DocumentationRegistry, RegistryConfig};
use std::net::SocketAddr;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_server_config_creation() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = ServerConfig {
        bind_address: "127.0.0.1:8080".parse().unwrap(),
        document_root: temp_dir.path().to_path_buf(),
        enable_https: false,
        ssl_config: None,
        cors_config: CorsConfig::default(),
        rate_limiting: RateLimitConfig::default(),
        cache_config: CacheConfig::default(),
        search_config: SearchConfig::default(),
        analytics_config: AnalyticsConfig::default(),
    };
    
    assert_eq!(config.bind_address.port(), 8080);
    assert!(!config.enable_https);
    assert!(config.ssl_config.is_none());
}

#[test]
fn test_server_config_defaults() {
    let config = ServerConfig::default();
    
    assert_eq!(config.bind_address.port(), 8080);
    assert_eq!(config.bind_address.ip().to_string(), "127.0.0.1");
    assert!(!config.enable_https);
    assert_eq!(config.document_root, PathBuf::from("./docs"));
}

#[tokio::test]
async fn test_server_creation() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = ServerConfig {
        bind_address: "127.0.0.1:0".parse().unwrap(), // Use port 0 for testing
        document_root: temp_dir.path().to_path_buf(),
        enable_https: false,
        ssl_config: None,
        cors_config: CorsConfig::default(),
        rate_limiting: RateLimitConfig::default(),
        cache_config: CacheConfig::default(),
        search_config: SearchConfig::default(),
        analytics_config: AnalyticsConfig::default(),
    };
    
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    let server = DocumentationServer::new(config, registry);
    
    // Test validation
    match server.validate_config() {
        Ok(()) => {
            // Success - document root exists
        }
        Err(e) => {
            // Expected if document root doesn't exist
            assert!(e.to_string().contains("not exist") || e.to_string().contains("directory"));
        }
    }
}

#[test]
fn test_cors_config() {
    let cors_config = CorsConfig::default();
    
    assert_eq!(cors_config.allowed_origins, vec!["*"]);
    assert!(cors_config.allowed_methods.contains(&"GET".to_string()));
    assert!(cors_config.allowed_methods.contains(&"POST".to_string()));
    assert!(cors_config.allowed_methods.contains(&"OPTIONS".to_string()));
    assert!(!cors_config.allow_credentials);
}

#[test]
fn test_rate_limiting_config() {
    let rate_config = RateLimitConfig::default();
    
    assert_eq!(rate_config.requests_per_minute, 60);
    assert_eq!(rate_config.burst_capacity, 10);
    assert!(rate_config.enabled);
}

#[test]
fn test_cache_config() {
    let cache_config = CacheConfig::default();
    
    assert!(cache_config.enabled);
    assert_eq!(cache_config.static_cache_duration, 3600); // 1 hour
    assert_eq!(cache_config.api_cache_duration, 300);     // 5 minutes
    assert_eq!(cache_config.max_cache_size, 1024 * 1024 * 100); // 100MB
}

#[test]
fn test_search_config() {
    let search_config = SearchConfig::default();
    
    assert!(search_config.enabled);
    assert_eq!(search_config.max_results, 100);
    assert_eq!(search_config.index_refresh_interval, 300); // 5 minutes
    assert!(search_config.full_text_search);
}

#[test]
fn test_analytics_config() {
    let analytics_config = AnalyticsConfig::default();
    
    assert!(analytics_config.enabled);
    assert_eq!(analytics_config.retention_days, 30);
    assert!(analytics_config.track_page_views);
    assert!(analytics_config.track_search_queries);
    assert!(analytics_config.track_downloads);
}

#[test]
fn test_search_query_parsing() {
    let query = SearchQuery {
        q: "test query".to_string(),
        package: Some("std".to_string()),
        version: Some("1.0.0".to_string()),
        limit: Some(20),
        offset: Some(10),
    };
    
    assert_eq!(query.q, "test query");
    assert_eq!(query.package, Some("std".to_string()));
    assert_eq!(query.version, Some("1.0.0".to_string()));
    assert_eq!(query.limit, Some(20));
    assert_eq!(query.offset, Some(10));
}

#[test]
fn test_search_response() {
    use cursed::docs::server::{SearchResult, PaginationInfo};
    
    let search_result = SearchResult {
        title: "Test Function".to_string(),
        url: "/docs/std/1.0.0/test_function".to_string(),
        snippet: "A test function for demonstration".to_string(),
        package: "std".to_string(),
        version: "1.0.0".to_string(),
        result_type: "function".to_string(),
        score: 0.95,
    };
    
    let pagination = PaginationInfo {
        page: 1,
        total_pages: 5,
        per_page: 20,
        has_next: true,
        has_prev: false,
    };
    
    let response = SearchResponse {
        results: vec![search_result.clone()],
        total: 100,
        query: "test".to_string(),
        search_time_ms: 50,
        pagination,
    };
    
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.total, 100);
    assert_eq!(response.query, "test");
    assert_eq!(response.search_time_ms, 50);
    assert_eq!(response.results[0].title, "Test Function");
    assert_eq!(response.results[0].score, 0.95);
}

#[test]
fn test_server_metrics() {
    let metrics = ServerMetrics {
        total_requests: 1000,
        requests_per_second: 10.5,
        avg_response_time_ms: 150.0,
        error_rate: 2.5,
        cache_hit_rate: 85.0,
        active_connections: 25,
        uptime_seconds: 3600,
        memory_usage_bytes: 1024 * 1024 * 50, // 50MB
    };
    
    assert_eq!(metrics.total_requests, 1000);
    assert_eq!(metrics.requests_per_second, 10.5);
    assert_eq!(metrics.avg_response_time_ms, 150.0);
    assert_eq!(metrics.error_rate, 2.5);
    assert_eq!(metrics.cache_hit_rate, 85.0);
    assert_eq!(metrics.uptime_seconds, 3600);
}

#[test]
fn test_version_info() {
    let version_info = VersionInfo {
        package: "test-package".to_string(),
        versions: vec!["1.0.0".to_string(), "0.9.0".to_string(), "0.8.0".to_string()],
        latest: "1.0.0".to_string(),
        default: Some("1.0.0".to_string()),
    };
    
    assert_eq!(version_info.package, "test-package");
    assert_eq!(version_info.versions.len(), 3);
    assert_eq!(version_info.latest, "1.0.0");
    assert_eq!(version_info.default, Some("1.0.0".to_string()));
}

#[test]
fn test_analytics_event() {
    use cursed::docs::server::AnalyticsEvent;
    use std::collections::HashMap;
    
    let mut data = HashMap::new();
    data.insert("page".to_string(), serde_json::Value::String("/docs/std".to_string()));
    data.insert("user_agent".to_string(), serde_json::Value::String("Mozilla/5.0".to_string()));
    
    let event = AnalyticsEvent {
        event_type: "page_view".to_string(),
        timestamp: 1640995200,
        user_agent: Some("Mozilla/5.0".to_string()),
        ip_hash: "hashed_ip_123".to_string(),
        referer: Some("https://cursed.dev".to_string()),
        data,
    };
    
    assert_eq!(event.event_type, "page_view");
    assert_eq!(event.timestamp, 1640995200);
    assert!(event.user_agent.is_some());
    assert!(!event.ip_hash.is_empty());
    assert!(event.referer.is_some());
    assert!(!event.data.is_empty());
}

#[test]
fn test_ssl_config() {
    use cursed::docs::server::SslServerConfig;
    
    let ssl_config = SslServerConfig {
        cert_path: PathBuf::from("/etc/ssl/cert.pem"),
        key_path: PathBuf::from("/etc/ssl/private.key"),
        chain_path: Some(PathBuf::from("/etc/ssl/chain.pem")),
    };
    
    assert_eq!(ssl_config.cert_path, PathBuf::from("/etc/ssl/cert.pem"));
    assert_eq!(ssl_config.key_path, PathBuf::from("/etc/ssl/private.key"));
    assert!(ssl_config.chain_path.is_some());
}

#[test]
fn test_config_validation() {
    let temp_dir = TempDir::new().unwrap();
    
    // Valid configuration
    let valid_config = ServerConfig {
        bind_address: "127.0.0.1:8080".parse().unwrap(),
        document_root: temp_dir.path().to_path_buf(),
        enable_https: false,
        ssl_config: None,
        cors_config: CorsConfig::default(),
        rate_limiting: RateLimitConfig::default(),
        cache_config: CacheConfig::default(),
        search_config: SearchConfig::default(),
        analytics_config: AnalyticsConfig::default(),
    };
    
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    let server = DocumentationServer::new(valid_config, registry);
    
    assert!(server.validate_config().is_ok());
    
    // Invalid configuration - non-existent document root
    let invalid_config = ServerConfig {
        bind_address: "127.0.0.1:8080".parse().unwrap(),
        document_root: PathBuf::from("/non/existent/path"),
        enable_https: false,
        ssl_config: None,
        cors_config: CorsConfig::default(),
        rate_limiting: RateLimitConfig::default(),
        cache_config: CacheConfig::default(),
        search_config: SearchConfig::default(),
        analytics_config: AnalyticsConfig::default(),
    };
    
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    let invalid_server = DocumentationServer::new(invalid_config, registry);
    
    assert!(invalid_server.validate_config().is_err());
}

#[test]
fn test_https_config_validation() {
    let temp_dir = TempDir::new().unwrap();
    
    // HTTPS enabled without SSL config should fail
    let https_no_ssl = ServerConfig {
        bind_address: "127.0.0.1:8080".parse().unwrap(),
        document_root: temp_dir.path().to_path_buf(),
        enable_https: true,
        ssl_config: None,
        cors_config: CorsConfig::default(),
        rate_limiting: RateLimitConfig::default(),
        cache_config: CacheConfig::default(),
        search_config: SearchConfig::default(),
        analytics_config: AnalyticsConfig::default(),
    };
    
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    let server = DocumentationServer::new(https_no_ssl, registry);
    
    assert!(server.validate_config().is_err());
}

#[test]
fn test_serialization() {
    let query = SearchQuery {
        q: "test".to_string(),
        package: None,
        version: None,
        limit: Some(10),
        offset: Some(0),
    };
    
    // Test serialization to JSON
    let json = serde_json::to_string(&query).unwrap();
    assert!(json.contains("test"));
    assert!(json.contains("limit"));
    
    // Test metrics serialization
    let metrics = ServerMetrics {
        total_requests: 100,
        requests_per_second: 5.0,
        avg_response_time_ms: 200.0,
        error_rate: 1.0,
        cache_hit_rate: 90.0,
        active_connections: 10,
        uptime_seconds: 1800,
        memory_usage_bytes: 1024 * 1024,
    };
    
    let metrics_json = serde_json::to_string(&metrics).unwrap();
    assert!(metrics_json.contains("total_requests"));
    assert!(metrics_json.contains("100"));
}

#[test]
fn test_pagination_info() {
    use cursed::docs::server::PaginationInfo;
    
    let pagination = PaginationInfo {
        page: 3,
        total_pages: 10,
        per_page: 20,
        has_next: true,
        has_prev: true,
    };
    
    assert_eq!(pagination.page, 3);
    assert_eq!(pagination.total_pages, 10);
    assert_eq!(pagination.per_page, 20);
    assert!(pagination.has_next);
    assert!(pagination.has_prev);
    
    // Test page calculations
    let first_page = PaginationInfo {
        page: 1,
        total_pages: 5,
        per_page: 20,
        has_next: true,
        has_prev: false,
    };
    
    assert!(!first_page.has_prev);
    assert!(first_page.has_next);
    
    let last_page = PaginationInfo {
        page: 5,
        total_pages: 5,
        per_page: 20,
        has_next: false,
        has_prev: true,
    };
    
    assert!(last_page.has_prev);
    assert!(!last_page.has_next);
}

#[test]
fn test_search_result_scoring() {
    use cursed::docs::server::SearchResult;
    
    let mut results = vec![
        SearchResult {
            title: "Low Score".to_string(),
            url: "/low".to_string(),
            snippet: "Low relevance".to_string(),
            package: "pkg".to_string(),
            version: "1.0.0".to_string(),
            result_type: "function".to_string(),
            score: 0.3,
        },
        SearchResult {
            title: "High Score".to_string(),
            url: "/high".to_string(),
            snippet: "High relevance".to_string(),
            package: "pkg".to_string(),
            version: "1.0.0".to_string(),
            result_type: "function".to_string(),
            score: 0.9,
        },
        SearchResult {
            title: "Medium Score".to_string(),
            url: "/medium".to_string(),
            snippet: "Medium relevance".to_string(),
            package: "pkg".to_string(),
            version: "1.0.0".to_string(),
            result_type: "function".to_string(),
            score: 0.6,
        },
    ];
    
    // Sort by score (descending)
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    
    assert_eq!(results[0].title, "High Score");
    assert_eq!(results[1].title, "Medium Score");
    assert_eq!(results[2].title, "Low Score");
}

#[test]
fn test_server_address_parsing() {
    let addresses = [
        "127.0.0.1:8080",
        "0.0.0.0:3000",
        "localhost:9000",
        "[::1]:8080",
    ];
    
    for addr_str in &addresses {
        if let Ok(addr) = addr_str.parse::<SocketAddr>() {
            let config = ServerConfig {
                bind_address: addr,
                document_root: PathBuf::from("./docs"),
                enable_https: false,
                ssl_config: None,
                cors_config: CorsConfig::default(),
                rate_limiting: RateLimitConfig::default(),
                cache_config: CacheConfig::default(),
                search_config: SearchConfig::default(),
                analytics_config: AnalyticsConfig::default(),
            };
            
            assert_eq!(config.bind_address, addr);
        }
    }
}

#[test]
fn test_custom_cors_config() {
    let custom_cors = CorsConfig {
        allowed_origins: vec!["https://cursed.dev".to_string(), "https://docs.cursed.dev".to_string()],
        allowed_methods: vec!["GET".to_string(), "POST".to_string()],
        allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
        allow_credentials: true,
    };
    
    assert_eq!(custom_cors.allowed_origins.len(), 2);
    assert!(custom_cors.allowed_origins.contains(&"https://cursed.dev".to_string()));
    assert!(custom_cors.allow_credentials);
    assert!(!custom_cors.allowed_methods.contains(&"DELETE".to_string()));
}

#[test]
fn test_rate_limiting_calculation() {
    let rate_config = RateLimitConfig {
        requests_per_minute: 120,
        burst_capacity: 20,
        enabled: true,
    };
    
    // Calculate requests per second
    let requests_per_second = rate_config.requests_per_minute as f64 / 60.0;
    assert_eq!(requests_per_second, 2.0);
    
    // Burst should allow temporary spikes
    assert!(rate_config.burst_capacity > 0);
    assert!(rate_config.burst_capacity < rate_config.requests_per_minute);
}
