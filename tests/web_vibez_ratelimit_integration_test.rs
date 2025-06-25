/// fr fr Rate limiting integration tests - comprehensive testing
use std::sync::Arc;
use std::time::Duration;

use cursed::stdlib::packages::web_vibez::ratelimit::{
    RateLimiter, RateLimitConfig, RateLimitDecision, InMemoryStore, FixedWindow, SlidingWindow, TokenBucket,
    WindowConfig, BucketConfig, ClientIdentification, ErrorConfig, CleanupConfig,
    extract_client_id, current_timestamp,
};

#[tokio::test]
async fn test_basic_rate_limiting() {
    let store = Arc::new(InMemoryStore::new());
    let algorithm = Arc::new(FixedWindow::new());
    let config = RateLimitConfig::per_minute(5);
    
    let limiter = RateLimiter::new(store, algorithm, config);
    
    // First 5 requests should be allowed
    for i in 0..5 {
        let decision = limiter.check_request(&format!("client_{}", i)).await.unwrap();
        assert!(matches!(decision, RateLimitDecision::Allow { .. }));
    }
    
    // Sixth request should be denied for the same client
    let decision = limiter.check_request("client_0").await.unwrap();
    assert!(matches!(decision, RateLimitDecision::Deny { .. }));
}

#[tokio::test]
async fn test_sliding_window_algorithm() {
    let store = Arc::new(InMemoryStore::new());
    let algorithm = Arc::new(SlidingWindow::new());
    let config = RateLimitConfig {
        max_requests: 3,
        window_config: WindowConfig::Sliding { duration: Duration::from_secs(10) },
        bucket_config: None,
        client_identification: ClientIdentification::IpAddress,
        error_config: ErrorConfig::default(),
        cleanup_config: CleanupConfig::default(),
    };
    
    let limiter = RateLimiter::new(store, algorithm, config);
    
    // Allow 3 requests
    for _ in 0..3 {
        let decision = limiter.check_request("test_client").await.unwrap();
        assert!(matches!(decision, RateLimitDecision::Allow { .. }));
    }
    
    // Fourth should be denied
    let decision = limiter.check_request("test_client").await.unwrap();
    assert!(matches!(decision, RateLimitDecision::Deny { .. }));
}

#[tokio::test]
async fn test_token_bucket_algorithm() {
    let store = Arc::new(InMemoryStore::new());
    let algorithm = Arc::new(TokenBucket::new());
    let config = RateLimitConfig {
        max_requests: 5,
        window_config: WindowConfig::Fixed { duration: Duration::from_secs(60) },
        bucket_config: Some(BucketConfig {
            capacity: 3.0,
            refill_rate: 1.0, // 1 token per second
        }),
        client_identification: ClientIdentification::IpAddress,
        error_config: ErrorConfig::default(),
        cleanup_config: CleanupConfig::default(),
    };
    
    let limiter = RateLimiter::new(store, algorithm, config);
    
    // Should allow 3 requests immediately (burst capacity)
    for _ in 0..3 {
        let decision = limiter.check_request("test_client").await.unwrap();
        assert!(matches!(decision, RateLimitDecision::Allow { .. }));
    }
    
    // Fourth should be denied (bucket empty)
    let decision = limiter.check_request("test_client").await.unwrap();
    assert!(matches!(decision, RateLimitDecision::Deny { .. }));
}

#[tokio::test]
async fn test_multiple_clients() {
    let store = Arc::new(InMemoryStore::new());
    let algorithm = Arc::new(FixedWindow::new());
    let config = RateLimitConfig::per_minute(2);
    
    let limiter = RateLimiter::new(store, algorithm, config);
    
    // Each client should have independent limits
    for client in ["client_1", "client_2", "client_3"] {
        for _ in 0..2 {
            let decision = limiter.check_request(client).await.unwrap();
            assert!(matches!(decision, RateLimitDecision::Allow { .. }));
        }
        
        // Third request should be denied
        let decision = limiter.check_request(client).await.unwrap();
        assert!(matches!(decision, RateLimitDecision::Deny { .. }));
    }
}

#[tokio::test]
async fn test_rate_limit_context() {
    let store = Arc::new(InMemoryStore::new());
    let algorithm = Arc::new(FixedWindow::new());
    let config = RateLimitConfig::per_minute(10);
    
    let limiter = RateLimiter::new(store, algorithm, config);
    
    // Make some requests
    for _ in 0..3 {
        limiter.check_request("test_client").await.unwrap();
    }
    
    // Get context
    let context = limiter.get_context("test_client").await.unwrap();
    assert_eq!(context.limit, 10);
    assert_eq!(context.current_count, 3);
    assert_eq!(context.remaining, 7);
    assert!(context.reset_time > current_timestamp());
}

#[tokio::test]
async fn test_rate_limit_reset() {
    let store = Arc::new(InMemoryStore::new());
    let algorithm = Arc::new(FixedWindow::new());
    let config = RateLimitConfig::per_minute(2);
    
    let limiter = RateLimiter::new(store, algorithm, config);
    
    // Exhaust the limit
    for _ in 0..2 {
        limiter.check_request("test_client").await.unwrap();
    }
    
    // Should be denied
    let decision = limiter.check_request("test_client").await.unwrap();
    assert!(matches!(decision, RateLimitDecision::Deny { .. }));
    
    // Reset the client
    limiter.reset_client("test_client").await.unwrap();
    
    // Should be allowed again
    let decision = limiter.check_request("test_client").await.unwrap();
    assert!(matches!(decision, RateLimitDecision::Allow { .. }));
}

#[tokio::test]
async fn test_metrics_collection() {
    let store = Arc::new(InMemoryStore::new());
    let algorithm = Arc::new(FixedWindow::new());
    let config = RateLimitConfig::per_minute(2);
    
    let limiter = RateLimiter::new(store, algorithm, config);
    
    // Make some requests
    limiter.check_request("client1").await.unwrap(); // Allow
    limiter.check_request("client1").await.unwrap(); // Allow
    limiter.check_request("client1").await.unwrap(); // Deny
    limiter.check_request("client2").await.unwrap(); // Allow
    
    let metrics = limiter.get_metrics().await;
    assert_eq!(metrics.total_requests, 4);
    assert_eq!(metrics.allowed_requests, 3);
    assert_eq!(metrics.denied_requests, 1);
    assert_eq!(metrics.success_rate(), 75.0);
    assert_eq!(metrics.denial_rate(), 25.0);
    
    // Check client-specific metrics
    assert_eq!(metrics.client_metrics.len(), 2);
    let client1_metrics = metrics.client_metrics.get("client1").unwrap();
    assert_eq!(client1_metrics.total_requests, 3);
    assert_eq!(client1_metrics.allowed_requests, 2);
    assert_eq!(client1_metrics.denied_requests, 1);
}

#[tokio::test]
async fn test_configuration_validation() {
    // Test invalid max_requests
    let mut config = RateLimitConfig::per_minute(0);
    config.max_requests = 0;
    assert!(config.validate().is_err());
    
    // Test valid configuration
    let valid_config = RateLimitConfig::per_minute(100);
    assert!(valid_config.validate().is_ok());
    
    // Test bucket configuration validation
    let mut bucket_config = RateLimitConfig::per_minute(10)
        .with_token_bucket(0.0, 1.0); // Invalid capacity
    assert!(bucket_config.validate().is_err());
    
    let valid_bucket_config = RateLimitConfig::per_minute(10)
        .with_token_bucket(10.0, 1.0);
    assert!(valid_bucket_config.validate().is_ok());
}

#[tokio::test]
async fn test_client_identification_strategies() {
    // Test IP address identification
    let ip_config = RateLimitConfig::per_minute(10)
        .with_client_identification(ClientIdentification::IpAddress);
    assert!(matches!(ip_config.client_identification, ClientIdentification::IpAddress));
    
    // Test header-based identification
    let header_config = RateLimitConfig::per_minute(10)
        .with_client_identification(ClientIdentification::Header {
            name: "X-API-Key".to_string(),
        });
    assert!(matches!(header_config.client_identification, ClientIdentification::Header { .. }));
    
    // Test composite identification
    let composite_config = RateLimitConfig::per_minute(10)
        .with_client_identification(ClientIdentification::Composite {
            factors: vec![
                cursed::stdlib::packages::web_vibez::ratelimit::IdentificationFactor::IpAddress,
                cursed::stdlib::packages::web_vibez::ratelimit::IdentificationFactor::Header {
                    name: "User-Agent".to_string(),
                },
            ],
        });
    assert!(matches!(composite_config.client_identification, ClientIdentification::Composite { .. }));
}

#[tokio::test]
async fn test_error_configuration() {
    let error_config = ErrorConfig::new()
        .with_status_code(429)
        .with_message("Rate limit exceeded".to_string())
        .with_custom_response("Too many requests, please try again later".to_string());
    
    assert_eq!(error_config.status_code, 429);
    assert_eq!(error_config.message, "Rate limit exceeded");
    assert!(error_config.custom_response.is_some());
    assert!(error_config.validate().is_ok());
}

#[tokio::test]
async fn test_cleanup_configuration() {
    let cleanup_config = CleanupConfig::new()
        .with_interval(Duration::from_secs(60))
        .with_client_ttl(Duration::from_secs(300))
        .with_max_clients(1000);
    
    assert_eq!(cleanup_config.interval, Duration::from_secs(60));
    assert_eq!(cleanup_config.client_ttl, Duration::from_secs(300));
    assert_eq!(cleanup_config.max_clients, Some(1000));
    assert!(cleanup_config.validate().is_ok());
    
    let disabled_cleanup = CleanupConfig::disabled();
    assert!(!disabled_cleanup.enabled);
    assert!(disabled_cleanup.validate().is_ok());
}

#[tokio::test]
async fn test_store_cleanup() {
    let store = InMemoryStore::new();
    
    // Add some client states
    for i in 0..5 {
        let mut state = cursed::stdlib::packages::web_vibez::ratelimit::ClientState::new();
        state.last_request = current_timestamp() - 3600; // 1 hour ago
        store.update_client_state(&format!("old_client_{}", i), &state).await.unwrap();
    }
    
    // Add some recent client states
    for i in 0..3 {
        let state = cursed::stdlib::packages::web_vibez::ratelimit::ClientState::new();
        store.update_client_state(&format!("new_client_{}", i), &state).await.unwrap();
    }
    
    assert_eq!(store.client_count(), 8);
    
    // Cleanup with 30 minute TTL
    let removed = store.cleanup_expired(Duration::from_secs(1800)).await.unwrap();
    assert_eq!(removed, 5); // Should remove 5 old clients
    assert_eq!(store.client_count(), 3); // Should have 3 remaining
}

#[tokio::test]
async fn test_algorithm_performance() {
    use std::time::Instant;
    
    let store = Arc::new(InMemoryStore::new());
    let algorithm = Arc::new(FixedWindow::new());
    let config = RateLimitConfig::per_minute(1000);
    
    let limiter = RateLimiter::new(store, algorithm, config);
    
    let start = Instant::now();
    
    // Make many requests to test performance
    for i in 0..1000 {
        let client_id = format!("client_{}", i % 100); // 100 different clients
        limiter.check_request(&client_id).await.unwrap();
    }
    
    let elapsed = start.elapsed();
    println!("1000 rate limit checks took: {:?}", elapsed);
    
    // Should complete in reasonable time (< 1 second for 1000 requests)
    assert!(elapsed < Duration::from_secs(1));
}

#[tokio::test]
async fn test_concurrent_requests() {
    use tokio::task;
    
    let store = Arc::new(InMemoryStore::new());
    let algorithm = Arc::new(FixedWindow::new());
    let config = RateLimitConfig::per_minute(100);
    
    let limiter = Arc::new(RateLimiter::new(store, algorithm, config));
    
    let mut handles = Vec::new();
    
    // Spawn 10 concurrent tasks
    for i in 0..10 {
        let limiter_clone = limiter.clone();
        let handle = task::spawn(async move {
            let client_id = format!("concurrent_client_{}", i);
            let mut allowed = 0;
            let mut denied = 0;
            
            // Each task makes 20 requests
            for _ in 0..20 {
                match limiter_clone.check_request(&client_id).await.unwrap() {
                    RateLimitDecision::Allow { .. } => allowed += 1,
                    RateLimitDecision::Deny { .. } => denied += 1,
                }
            }
            
            (allowed, denied)
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    let mut total_allowed = 0;
    let mut total_denied = 0;
    
    for handle in handles {
        let (allowed, denied) = handle.await.unwrap();
        total_allowed += allowed;
        total_denied += denied;
    }
    
    // Each client should have independent limits
    assert_eq!(total_allowed + total_denied, 200); // 10 clients * 20 requests each
    println!("Concurrent test - Allowed: {}, Denied: {}", total_allowed, total_denied);
}

#[tokio::test]
async fn test_rate_limit_headers_simulation() {
    let store = Arc::new(InMemoryStore::new());
    let algorithm = Arc::new(FixedWindow::new());
    let config = RateLimitConfig::per_minute(5);
    
    let limiter = RateLimiter::new(store, algorithm, config);
    
    // Make requests and check the decision details
    for i in 1..=5 {
        let decision = limiter.check_request("test_client").await.unwrap();
        match decision {
            RateLimitDecision::Allow { remaining, reset_time, .. } => {
                assert_eq!(remaining, 5 - i);
                assert!(reset_time > current_timestamp());
            }
            _ => panic!("Expected Allow decision"),
        }
    }
    
    // Next request should be denied
    let decision = limiter.check_request("test_client").await.unwrap();
    match decision {
        RateLimitDecision::Deny { retry_after, reset_time } => {
            assert!(retry_after > 0);
            assert!(reset_time > current_timestamp());
        }
        _ => panic!("Expected Deny decision"),
    }
}

#[tokio::test]
async fn test_extract_client_id() {
    use std::net::{IpAddr, Ipv4Addr};
    
    // Test with IP address
    let ip = Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
    let client_id = extract_client_id(ip);
    assert_eq!(client_id, "192.168.1.1");
    
    // Test with no IP
    let client_id = extract_client_id(None);
    assert_eq!(client_id, "unknown");
}

#[tokio::test]
async fn test_window_config_variants() {
    // Test fixed window
    let fixed_config = RateLimitConfig::new(10, Duration::from_secs(60));
    assert!(matches!(fixed_config.window_config, WindowConfig::Fixed { .. }));
    
    // Test sliding window
    let sliding_config = RateLimitConfig::new(10, Duration::from_secs(60))
        .with_sliding_window(Duration::from_secs(60));
    assert!(matches!(sliding_config.window_config, WindowConfig::Sliding { .. }));
}

#[tokio::test]
async fn test_rate_limit_builder_pattern() {
    let config = RateLimitConfig::per_minute(100)
        .with_sliding_window(Duration::from_secs(60))
        .with_token_bucket(10.0, 1.0)
        .with_client_identification(ClientIdentification::Header {
            name: "X-API-Key".to_string(),
        })
        .with_error_config(ErrorConfig::new().with_status_code(429))
        .with_cleanup_config(CleanupConfig::new().with_interval(Duration::from_secs(300)));
    
    assert_eq!(config.max_requests, 100);
    assert!(matches!(config.window_config, WindowConfig::Sliding { .. }));
    assert!(config.bucket_config.is_some());
    assert!(matches!(config.client_identification, ClientIdentification::Header { .. }));
    assert_eq!(config.error_config.status_code, 429);
    assert_eq!(config.cleanup_config.interval, Duration::from_secs(300));
    
    assert!(config.validate().is_ok());
}
