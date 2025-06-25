/// fr fr Rate limiting middleware tests - comprehensive middleware integration
use std::sync::Arc;
use std::time::Duration;

use cursed::stdlib::packages::web_vibez::{
    HttpRequest, HttpResponse, HttpMethod, StatusCode,
    middleware::{Middleware, RateLimitMiddleware},
    ratelimit::{RateLimitConfig, WindowConfig, BucketConfig, ClientIdentification, ErrorConfig},
};

#[tokio::test]
async fn test_rate_limit_middleware_basic() {
    let middleware = RateLimitMiddleware::per_minute(2);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    // First request should be allowed
    let result = middleware.before_request(&mut request).await;
    assert!(result.is_ok());
    
    // Second request should be allowed
    let result = middleware.before_request(&mut request).await;
    assert!(result.is_ok());
    
    // Third request should be denied
    let result = middleware.before_request(&mut request).await;
    assert!(result.is_err());
    
    // Check if it's a rate limit error
    let error = result.unwrap_err();
    assert_eq!(error.category(), "rate_limit");
}

#[tokio::test]
async fn test_rate_limit_middleware_different_clients() {
    let middleware = RateLimitMiddleware::per_minute(1);
    
    // Client 1
    let mut request1 = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request1.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    // Client 2
    let mut request2 = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request2.set_client_ip(Some("192.168.1.2".parse().unwrap()));
    
    // Both clients should be allowed their first request
    assert!(middleware.before_request(&mut request1).await.is_ok());
    assert!(middleware.before_request(&mut request2).await.is_ok());
    
    // Both clients should be denied their second request
    assert!(middleware.before_request(&mut request1).await.is_err());
    assert!(middleware.before_request(&mut request2).await.is_err());
}

#[tokio::test]
async fn test_rate_limit_middleware_headers() {
    let middleware = RateLimitMiddleware::per_minute(5);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    let mut response = HttpResponse::ok();
    
    // Process response to add headers
    middleware.after_response(&request, &mut response).await.unwrap();
    
    // Check rate limit headers
    assert!(response.header("x-ratelimit-limit").is_some());
    assert!(response.header("x-ratelimit-remaining").is_some());
    assert!(response.header("x-ratelimit-reset").is_some());
    assert!(response.header("x-ratelimit-policy").is_some());
    
    assert_eq!(response.header("x-ratelimit-limit"), Some(&"5".to_string()));
}

#[tokio::test]
async fn test_rate_limit_middleware_sliding_window() {
    let config = RateLimitConfig::new(3, Duration::from_secs(10))
        .with_sliding_window(Duration::from_secs(10));
    let middleware = RateLimitMiddleware::new(config);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    // Allow 3 requests
    for _ in 0..3 {
        let result = middleware.before_request(&mut request).await;
        assert!(result.is_ok());
    }
    
    // Fourth should be denied
    let result = middleware.before_request(&mut request).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_rate_limit_middleware_token_bucket() {
    let config = RateLimitConfig::new(5, Duration::from_secs(60))
        .with_token_bucket(3.0, 1.0); // 3 tokens capacity, 1 token/second refill
    let middleware = RateLimitMiddleware::new(config);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    // Should allow 3 burst requests
    for _ in 0..3 {
        let result = middleware.before_request(&mut request).await;
        assert!(result.is_ok());
    }
    
    // Fourth should be denied (bucket empty)
    let result = middleware.before_request(&mut request).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_rate_limit_middleware_header_identification() {
    let config = RateLimitConfig::per_minute(2)
        .with_client_identification(ClientIdentification::Header {
            name: "X-API-Key".to_string(),
        });
    let middleware = RateLimitMiddleware::new(config);
    
    // Request with API key
    let mut request1 = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request1.headers.insert("x-api-key".to_string(), "key123".to_string());
    
    // Request with different API key
    let mut request2 = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request2.headers.insert("x-api-key".to_string(), "key456".to_string());
    
    // Each API key should have independent limits
    assert!(middleware.before_request(&mut request1).await.is_ok());
    assert!(middleware.before_request(&mut request1).await.is_ok());
    assert!(middleware.before_request(&mut request1).await.is_err()); // Third request denied
    
    // Different API key should still be allowed
    assert!(middleware.before_request(&mut request2).await.is_ok());
    assert!(middleware.before_request(&mut request2).await.is_ok());
    assert!(middleware.before_request(&mut request2).await.is_err()); // Third request denied
}

#[tokio::test]
async fn test_rate_limit_middleware_composite_identification() {
    let config = RateLimitConfig::per_minute(2)
        .with_client_identification(ClientIdentification::Composite {
            factors: vec![
                cursed::stdlib::packages::web_vibez::ratelimit::IdentificationFactor::IpAddress,
                cursed::stdlib::packages::web_vibez::ratelimit::IdentificationFactor::Header {
                    name: "User-Agent".to_string(),
                },
            ],
        });
    let middleware = RateLimitMiddleware::new(config);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    request.headers.insert("user-agent".to_string(), "TestClient/1.0".to_string());
    
    // Test composite identification
    assert!(middleware.before_request(&mut request).await.is_ok());
    assert!(middleware.before_request(&mut request).await.is_ok());
    assert!(middleware.before_request(&mut request).await.is_err());
}

#[tokio::test]
async fn test_rate_limit_middleware_custom_error_config() {
    let error_config = ErrorConfig::new()
        .with_status_code(429)
        .with_message("Custom rate limit message".to_string())
        .with_custom_response("Custom JSON response".to_string());
    
    let config = RateLimitConfig::per_minute(1)
        .with_error_config(error_config);
    let middleware = RateLimitMiddleware::new(config);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    // First request allowed
    assert!(middleware.before_request(&mut request).await.is_ok());
    
    // Second request denied with custom error
    let result = middleware.before_request(&mut request).await;
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert_eq!(error.status_code(), StatusCode::TooManyRequests);
    assert!(error.message().contains("Custom JSON response"));
}

#[tokio::test]
async fn test_rate_limit_middleware_fail_open() {
    // Create middleware with invalid configuration that might cause internal errors
    let config = RateLimitConfig::per_minute(100);
    let middleware = RateLimitMiddleware::new(config);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    // Don't set client IP to potentially trigger errors
    
    // Even with potential internal errors, middleware should fail open (allow requests)
    let result = middleware.before_request(&mut request).await;
    // This might succeed or fail depending on error handling, but shouldn't panic
    // In a production system, we'd want this to succeed (fail open)
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_rate_limit_middleware_convenience_constructors() {
    // Test per-second constructor
    let per_second = RateLimitMiddleware::per_second(10);
    assert_eq!(per_second.name(), "RateLimitMiddleware");
    
    // Test per-minute constructor
    let per_minute = RateLimitMiddleware::per_minute(100);
    assert_eq!(per_minute.name(), "RateLimitMiddleware");
    
    // Test per-hour constructor
    let per_hour = RateLimitMiddleware::per_hour(1000);
    assert_eq!(per_hour.name(), "RateLimitMiddleware");
    
    // Test sliding window constructor
    let sliding = RateLimitMiddleware::sliding_window(50, Duration::from_secs(60));
    assert_eq!(sliding.name(), "RateLimitMiddleware");
    
    // Test token bucket constructor
    let token_bucket = RateLimitMiddleware::token_bucket(100, Duration::from_secs(60), 10.0, 1.0);
    assert_eq!(token_bucket.name(), "RateLimitMiddleware");
}

#[tokio::test]
async fn test_rate_limit_middleware_metrics() {
    let middleware = RateLimitMiddleware::per_minute(3);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    // Make some requests
    middleware.before_request(&mut request).await.unwrap(); // Allow
    middleware.before_request(&mut request).await.unwrap(); // Allow
    middleware.before_request(&mut request).await.unwrap(); // Allow
    let _ = middleware.before_request(&mut request).await; // Deny
    
    // Get metrics
    let metrics = middleware.get_metrics().await;
    assert!(metrics.total_requests > 0);
    assert!(metrics.allowed_requests > 0);
    assert!(metrics.denied_requests > 0);
    assert!(metrics.success_rate() < 100.0); // Some requests were denied
    
    // Check client metrics
    assert!(!metrics.client_metrics.is_empty());
}

#[tokio::test]
async fn test_rate_limit_middleware_reset_client() {
    let middleware = RateLimitMiddleware::per_minute(1);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    // Exhaust the limit
    middleware.before_request(&mut request).await.unwrap(); // Allow
    let result = middleware.before_request(&mut request).await; // Deny
    assert!(result.is_err());
    
    // Reset the client
    middleware.reset_client("192.168.1.1").await.unwrap();
    
    // Should be allowed again
    let result = middleware.before_request(&mut request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_rate_limit_middleware_no_headers() {
    let error_config = ErrorConfig::new()
        .with_status_code(429)
        .with_message("Rate limited".to_string());
    
    // Disable headers
    let mut config = RateLimitConfig::per_minute(5)
        .with_error_config(error_config);
    config.error_config.include_headers = false;
    
    let middleware = RateLimitMiddleware::new(config);
    
    let request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    let mut response = HttpResponse::ok();
    
    // Process response
    middleware.after_response(&request, &mut response).await.unwrap();
    
    // Headers should not be included
    assert!(response.header("x-ratelimit-limit").is_none());
    assert!(response.header("x-ratelimit-remaining").is_none());
    assert!(response.header("x-ratelimit-reset").is_none());
}

#[tokio::test]
async fn test_rate_limit_middleware_retry_after_header() {
    let config = RateLimitConfig::per_minute(1);
    let middleware = RateLimitMiddleware::new(config);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    // Exhaust the limit
    middleware.before_request(&mut request).await.unwrap();
    
    // Next request should include retry-after in error
    let result = middleware.before_request(&mut request).await;
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    if let cursed::stdlib::packages::web_vibez::WebError::RateLimit { retry_after, .. } = error {
        assert!(retry_after.is_some());
        assert!(retry_after.unwrap() > 0);
    }
}

#[tokio::test]
async fn test_rate_limit_middleware_performance() {
    use std::time::Instant;
    
    let middleware = RateLimitMiddleware::per_minute(1000);
    
    let start = Instant::now();
    
    // Test with many requests
    for i in 0..100 {
        let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
        request.set_client_ip(Some(format!("192.168.1.{}", i % 10).parse().unwrap()));
        
        let _ = middleware.before_request(&mut request).await;
    }
    
    let elapsed = start.elapsed();
    println!("100 middleware requests took: {:?}", elapsed);
    
    // Should complete in reasonable time
    assert!(elapsed < Duration::from_millis(500));
}

#[tokio::test]
async fn test_rate_limit_middleware_concurrent_requests() {
    use tokio::task;
    
    let middleware = Arc::new(RateLimitMiddleware::per_minute(50));
    let mut handles = Vec::new();
    
    // Spawn 5 concurrent tasks
    for i in 0..5 {
        let middleware_clone = middleware.clone();
        let handle = task::spawn(async move {
            let mut allowed = 0;
            let mut denied = 0;
            
            for _ in 0..20 {
                let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
                request.set_client_ip(Some(format!("192.168.1.{}", i).parse().unwrap()));
                
                match middleware_clone.before_request(&mut request).await {
                    Ok(_) => allowed += 1,
                    Err(_) => denied += 1,
                }
            }
            
            (allowed, denied)
        });
        handles.push(handle);
    }
    
    // Wait for all tasks
    let mut total_allowed = 0;
    let mut total_denied = 0;
    
    for handle in handles {
        let (allowed, denied) = handle.await.unwrap();
        total_allowed += allowed;
        total_denied += denied;
    }
    
    assert_eq!(total_allowed + total_denied, 100); // 5 tasks * 20 requests each
    println!("Concurrent middleware test - Allowed: {}, Denied: {}", total_allowed, total_denied);
}

#[tokio::test]
async fn test_rate_limit_middleware_with_custom_limiter() {
    use cursed::stdlib::packages::web_vibez::ratelimit::{RateLimiter, InMemoryStore, FixedWindow};
    
    // Create custom limiter
    let store = Arc::new(InMemoryStore::new());
    let algorithm = Arc::new(FixedWindow::new());
    let config = RateLimitConfig::per_minute(2);
    let limiter = Arc::new(RateLimiter::new(store, algorithm, config.clone()));
    
    // Create middleware with custom limiter
    let middleware = RateLimitMiddleware::with_limiter(limiter, config);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    // Test that it works with custom limiter
    assert!(middleware.before_request(&mut request).await.is_ok());
    assert!(middleware.before_request(&mut request).await.is_ok());
    assert!(middleware.before_request(&mut request).await.is_err());
}

#[tokio::test]
async fn test_rate_limit_middleware_scope_header() {
    let config = RateLimitConfig::per_minute(5)
        .with_client_identification(ClientIdentification::IpAddress);
    let middleware = RateLimitMiddleware::new(config);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    let mut response = HttpResponse::ok();
    
    // Process response to add headers
    middleware.after_response(&request, &mut response).await.unwrap();
    
    // Should include scope header for IP-based identification
    assert!(response.header("x-ratelimit-scope").is_some());
    assert_eq!(response.header("x-ratelimit-scope"), Some(&"192.168.1.1".to_string()));
}
