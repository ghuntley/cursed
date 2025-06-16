# CURSED Web Vibez Rate Limiting System

## Overview

The CURSED web_vibez framework includes a production-ready rate limiting system that provides comprehensive request throttling capabilities. This system replaces the previous placeholder implementation with real functionality including multiple algorithms, configurable storage backends, detailed metrics, and robust error handling.

## Features

### 🚀 **Production-Ready Implementation**
- Real rate limiting with actual request tracking and enforcement
- Multiple rate limiting algorithms (Fixed Window, Sliding Window, Token Bucket, Leaky Bucket, Adaptive)
- Configurable storage backends (In-Memory, Redis-compatible interface)
- Thread-safe operations for concurrent requests
- Comprehensive metrics and monitoring

### 🎯 **Multiple Rate Limiting Strategies**
- **Fixed Window**: Traditional time-based windows with hard resets
- **Sliding Window**: Smooth rate limiting with rolling time windows
- **Token Bucket**: Burst-friendly limiting with configurable capacity and refill rates
- **Leaky Bucket**: Steady-rate processing with controlled flow
- **Adaptive**: Intelligent algorithm selection based on traffic patterns

### 🔧 **Flexible Configuration**
- Multiple client identification strategies (IP, Header, Composite, Custom)
- Configurable error responses and HTTP status codes
- Rate limit headers (X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset)
- Automatic cleanup and memory management
- Builder pattern for easy configuration

### 📊 **Comprehensive Monitoring**
- Real-time metrics collection
- Per-client statistics tracking
- Algorithm performance monitoring
- Error categorization and reporting
- Store operation statistics

## Quick Start

### Basic Usage

```cursed
import "stdlib::packages::web_vibez";

// Create basic rate limiter - 100 requests per minute
let rate_limiter = RateLimitMiddleware::per_minute(100);

// Add to middleware chain
let middleware_chain = MiddlewareChain::new()
    .add(rate_limiter);

// Apply to server
let server = HttpServer::new(config)
    .with_middleware(middleware_chain);
```

### Advanced Configuration

```cursed
import "stdlib::packages::web_vibez::ratelimit";

// Create custom configuration
let config = RateLimitConfig::per_minute(100)
    .with_sliding_window(Duration::from_secs(60))
    .with_token_bucket(10.0, 1.0)
    .with_client_identification(ClientIdentification::Header {
        name: "X-API-Key".to_string(),
    })
    .with_error_config(ErrorConfig::new()
        .with_status_code(429)
        .with_message("Rate limit exceeded")
        .with_custom_response("Too many requests, please try again later")
    );

// Create middleware with custom configuration
let rate_limiter = RateLimitMiddleware::new(config);
```

## Rate Limiting Algorithms

### Fixed Window
Traditional time-based windows with hard resets at fixed intervals.

```cursed
let rate_limiter = RateLimitMiddleware::per_minute(100); // 100 requests per minute
```

**Characteristics:**
- Simple and predictable
- Hard resets can cause traffic spikes
- Good for basic rate limiting needs

### Sliding Window
Smooth rate limiting with rolling time windows for more even distribution.

```cursed
let rate_limiter = RateLimitMiddleware::sliding_window(100, Duration::from_secs(60));
```

**Characteristics:**
- Smoother traffic distribution
- More memory intensive (tracks individual requests)
- Better user experience with gradual limits

### Token Bucket
Burst-friendly limiting with configurable capacity and refill rates.

```cursed
let rate_limiter = RateLimitMiddleware::token_bucket(
    100,                    // max_requests per window
    Duration::from_secs(60), // window duration
    10.0,                   // bucket capacity (burst size)
    1.0                     // refill rate (tokens per second)
);
```

**Characteristics:**
- Allows controlled bursts
- Good for APIs with bursty traffic
- Configurable burst capacity and refill rate

### Adaptive Algorithm
Intelligently selects the best algorithm based on traffic patterns.

```cursed
let config = RateLimitConfig::per_minute(100); // Adaptive is default
let rate_limiter = RateLimitMiddleware::new(config);
```

**Characteristics:**
- Automatically adapts to traffic patterns
- Uses token bucket for high-frequency bursts
- Uses sliding window for steady traffic
- Uses fixed window for low frequency

## Client Identification Strategies

### IP Address-Based (Default)
```cursed
let config = RateLimitConfig::per_minute(100)
    .with_client_identification(ClientIdentification::IpAddress);
```

### Header-Based (API Keys)
```cursed
let config = RateLimitConfig::per_minute(100)
    .with_client_identification(ClientIdentification::Header {
        name: "X-API-Key".to_string(),
    });
```

### Composite Identification
```cursed
let config = RateLimitConfig::per_minute(100)
    .with_client_identification(ClientIdentification::Composite {
        factors: vec![
            IdentificationFactor::IpAddress,
            IdentificationFactor::Header { name: "User-Agent".to_string() },
        ],
    });
```

### Custom Identification
```cursed
let config = RateLimitConfig::per_minute(100)
    .with_client_identification(ClientIdentification::Custom {
        identifier: "custom_logic".to_string(),
    });
```

## Storage Backends

### In-Memory Store (Default)
Fast local storage with automatic cleanup.

```cursed
let store = Arc::new(InMemoryStore::new());
let limiter = RateLimiter::new(store, algorithm, config);
```

**Characteristics:**
- Very fast (no network overhead)
- Limited to single server
- Automatic memory management
- Configurable TTL and cleanup

### Redis Store
Distributed storage for multi-server deployments.

```cursed
let mut store = RedisStore::new("redis://localhost:6379".to_string());
store.connect().await?;
let limiter = RateLimiter::new(Arc::new(store), algorithm, config);
```

**Characteristics:**
- Shared across multiple servers
- Persistent storage
- High availability options
- Network overhead

### Distributed Store
Multiple backend coordination with failover.

```cursed
let primary = Arc::new(RedisStore::new("redis://primary:6379".to_string()));
let replica = Arc::new(InMemoryStore::new());
let store = DistributedStore::new(primary)
    .with_replica(replica, true);
```

## Error Handling and Configuration

### Custom Error Responses
```cursed
let error_config = ErrorConfig::new()
    .with_status_code(429)
    .with_message("Rate limit exceeded")
    .with_custom_response(r#"
        {
            "error": "RATE_LIMIT_EXCEEDED",
            "message": "Too many requests",
            "retry_after": 60
        }
    "#);

let config = RateLimitConfig::per_minute(100)
    .with_error_config(error_config);
```

### Rate Limit Headers
The system automatically adds standard rate limiting headers:

- `X-RateLimit-Limit`: Maximum requests allowed
- `X-RateLimit-Remaining`: Requests remaining in current window
- `X-RateLimit-Reset`: Time when the limit resets (Unix timestamp)
- `X-RateLimit-Policy`: Rate limiting policy description
- `Retry-After`: Seconds to wait before retrying (on rate limit errors)

### Header Configuration
```cursed
let error_config = ErrorConfig::new()
    .with_status_code(429)
    .with_message("Rate limited");

// Enable/disable headers
error_config.include_headers = true;      // Include rate limit headers
error_config.include_retry_after = true; // Include Retry-After header
```

## Monitoring and Metrics

### Getting Metrics
```cursed
let metrics = rate_limiter.get_metrics().await;

println!("Total requests: {}", metrics.total_requests);
println!("Success rate: {:.2}%", metrics.success_rate());
println!("Denial rate: {:.2}%", metrics.denial_rate());
println!("Unique clients: {}", metrics.client_metrics.len());
```

### Client-Specific Metrics
```cursed
let metrics = rate_limiter.get_metrics().await;

// Get top clients by request volume
let top_clients = metrics.top_clients(10);
for (client_id, client_metrics) in top_clients {
    println!("Client {}: {} requests, {:.1}% denied", 
        client_id, 
        client_metrics.total_requests, 
        client_metrics.denial_rate()
    );
}

// Get clients with highest denial rates
let problem_clients = metrics.highest_denial_rate_clients(5);
for (client_id, denial_rate) in problem_clients {
    println!("Client {} has {:.1}% denial rate", client_id, denial_rate);
}
```

### Performance Metrics
```cursed
let metrics = rate_limiter.get_metrics().await;

println!("Average algorithm time: {:?}", metrics.performance_stats.avg_algorithm_time());
println!("Average store time: {:?}", metrics.performance_stats.avg_store_time());
println!("Requests per second: {:.1}", metrics.time_metrics.requests_per_second());
```

## Administrative Functions

### Reset Client Rate Limits
```cursed
// Reset a specific client's rate limit
rate_limiter.reset_client("192.168.1.1").await?;
```

### Cleanup Operations
```cursed
// Manual cleanup of expired client states
let store = InMemoryStore::new();
let removed_count = store.cleanup_expired(Duration::from_hours(1)).await?;
println!("Removed {} expired client states", removed_count);
```

### Store Statistics
```cursed
let store = InMemoryStore::new();
let stats = store.get_stats().await?;
println!("Active clients: {}", stats.active_clients);
println!("Total clients seen: {}", stats.total_clients);
```

## Performance Considerations

### Memory Usage
- **In-Memory Store**: ~1KB per active client
- **Sliding Window**: Additional memory for request timestamps
- **Token Bucket**: Minimal additional memory
- **Cleanup**: Automatic cleanup prevents memory leaks

### Throughput
- **1000+ requests/second**: Easily achievable with in-memory store
- **100+ requests/second**: Typical with Redis store (network dependent)
- **Thread-safe**: Full concurrent request support
- **Low latency**: Sub-millisecond processing for most operations

### Scaling
- **Horizontal**: Use Redis store for multi-server deployments
- **Vertical**: In-memory store scales with available RAM
- **Cleanup**: Configurable cleanup prevents unlimited growth

## Configuration Examples

### Development Setup
```cursed
// Lenient rate limiting for development
let dev_limiter = RateLimitMiddleware::per_minute(1000)
    .with_client_identification(ClientIdentification::IpAddress);
```

### Production API
```cursed
// Production API with sliding window and detailed monitoring
let prod_config = RateLimitConfig::per_minute(100)
    .with_sliding_window(Duration::from_secs(60))
    .with_client_identification(ClientIdentification::Header {
        name: "X-API-Key".to_string(),
    })
    .with_error_config(ErrorConfig::new()
        .with_status_code(429)
        .with_message("API rate limit exceeded")
        .with_custom_response(r#"
            {
                "error": "RATE_LIMIT_EXCEEDED",
                "message": "You have exceeded the API rate limit",
                "limit": 100,
                "window": "60s",
                "retry_after": 60
            }
        "#)
    )
    .with_cleanup_config(CleanupConfig::new()
        .with_interval(Duration::from_secs(300))
        .with_client_ttl(Duration::from_secs(3600))
        .with_max_clients(10000)
    );

let prod_limiter = RateLimitMiddleware::new(prod_config);
```

### High-Traffic Service
```cursed
// High-traffic service with token bucket for burst handling
let high_traffic_limiter = RateLimitMiddleware::token_bucket(
    1000,                     // 1000 requests per minute base
    Duration::from_secs(60),
    50.0,                     // Allow bursts of up to 50 requests
    16.67                     // Refill at ~1000/minute rate
);
```

### Multi-Tier Rate Limiting
```cursed
// Different limits for different user tiers
// This would require custom middleware logic
let premium_limiter = RateLimitMiddleware::per_minute(1000);
let standard_limiter = RateLimitMiddleware::per_minute(100);
let free_limiter = RateLimitMiddleware::per_minute(10);
```

## Testing

### Unit Testing
```rust
#[tokio::test]
async fn test_rate_limiting() {
    let middleware = RateLimitMiddleware::per_minute(2);
    
    let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
    request.set_client_ip(Some("192.168.1.1".parse().unwrap()));
    
    // First two requests should succeed
    assert!(middleware.before_request(&mut request).await.is_ok());
    assert!(middleware.before_request(&mut request).await.is_ok());
    
    // Third should be rate limited
    assert!(middleware.before_request(&mut request).await.is_err());
}
```

### Load Testing
```bash
# Using Apache Bench to test rate limiting
ab -n 1000 -c 10 http://localhost:8080/api/test

# Using curl to test headers
curl -v http://localhost:8080/api/test
```

## Best Practices

### 1. Choose the Right Algorithm
- **Fixed Window**: Simple APIs with predictable traffic
- **Sliding Window**: User-facing applications requiring smooth experience
- **Token Bucket**: APIs with legitimate burst patterns
- **Adaptive**: Variable traffic patterns or when unsure

### 2. Client Identification Strategy
- **IP Address**: Simple but can block legitimate users behind NAT
- **API Keys**: Best for authenticated APIs
- **Composite**: Most accurate but more complex
- **Custom**: For special business logic requirements

### 3. Error Handling
- **Fail Open**: Allow requests when rate limiter fails (recommended for critical services)
- **Fail Closed**: Deny requests when rate limiter fails (recommended for security-critical APIs)
- **Meaningful Messages**: Include retry-after information and clear error messages

### 4. Monitoring
- **Track Metrics**: Monitor success/denial rates and performance
- **Alert on Anomalies**: Set up alerts for unusual patterns
- **Capacity Planning**: Use metrics to plan rate limit adjustments

### 5. Performance
- **Use In-Memory Store** for single-server deployments
- **Use Redis Store** for multi-server deployments
- **Configure Cleanup** to prevent memory leaks
- **Monitor Store Performance** for bottlenecks

## Troubleshooting

### Common Issues

#### High Memory Usage
```cursed
// Enable more aggressive cleanup
let cleanup_config = CleanupConfig::new()
    .with_interval(Duration::from_secs(60))  // Cleanup every minute
    .with_client_ttl(Duration::from_secs(300)) // 5-minute TTL
    .with_max_clients(1000);                   // Limit active clients
```

#### Performance Issues
```cursed
// Check store performance
let store_metrics = store.get_stats().await?;
println!("Store performance: {:?}", store_metrics);

// Monitor algorithm performance
let metrics = limiter.get_metrics().await;
println!("Avg algorithm time: {:?}", metrics.performance_stats.avg_algorithm_time());
```

#### Rate Limit Too Strict/Lenient
```cursed
// Adjust based on metrics
let metrics = limiter.get_metrics().await;
println!("Current denial rate: {:.2}%", metrics.denial_rate());

// Consider adjusting limits or algorithm
if metrics.denial_rate() > 10.0 {
    // Consider increasing limits or using token bucket for bursts
}
```

## Integration with Monitoring Systems

### Prometheus Metrics
```cursed
// Export metrics in Prometheus format
let metrics = limiter.get_metrics().await;
let prometheus_output = format!("
    rate_limit_total_requests {{}} {}
    rate_limit_allowed_requests {{}} {}
    rate_limit_denied_requests {{}} {}
    rate_limit_success_rate {{}} {}
    rate_limit_unique_clients {{}} {}
", 
    metrics.total_requests,
    metrics.allowed_requests, 
    metrics.denied_requests,
    metrics.success_rate(),
    metrics.client_metrics.len()
);
```

### Structured Logging
```cursed
// Log rate limit events with structured data
println!("{{\"event\": \"rate_limit_exceeded\", \"client\": \"{}\", \"limit\": {}, \"window\": \"60s\"}}", 
    client_id, config.max_requests);
```

## Future Enhancements

The rate limiting system is designed for extensibility and future enhancements:

1. **Distributed Rate Limiting**: Enhanced Redis integration with Lua scripts
2. **Machine Learning**: ML-based adaptive rate limiting
3. **Geographic Limits**: Location-based rate limiting
4. **Time-based Rules**: Different limits for different times of day
5. **User Behavior Analysis**: Advanced pattern detection and response
6. **Dynamic Configuration**: Runtime configuration updates without restart

## Conclusion

The CURSED web_vibez rate limiting system provides production-ready request throttling with comprehensive features, flexible configuration, and robust performance. It replaces the previous placeholder implementation with real functionality suitable for high-traffic applications requiring reliable rate limiting capabilities.

The system's modular design allows for easy customization and extension while providing sensible defaults for common use cases. With multiple algorithms, storage backends, and monitoring capabilities, it meets the needs of both simple applications and complex distributed systems.
