# CURSED Web Framework Timeout System

## Overview

The CURSED web framework provides comprehensive timeout mechanisms to ensure robust and reliable web applications. The timeout system includes request timeouts, connection timeouts, session timeouts, and database operation timeouts, all integrated with the `parking_lot` crate's timeout support for efficient async operations.

## Features

### 1. Request Timeouts
- **Purpose**: Prevent requests from hanging indefinitely
- **Configuration**: `ServerConfig.request_timeout`
- **Default**: 30 seconds
- **Behavior**: Automatically terminates requests that exceed the timeout duration

### 2. Connection Timeouts
- **Purpose**: Manage connection lifecycle and prevent resource leaks
- **Configuration**: `ServerConfig.connection_timeout`
- **Default**: 15 seconds
- **Behavior**: Tracks connection establishment and activity, cleaning up stale connections

### 3. Session Timeouts
- **Purpose**: Automatic session expiration for security and resource management
- **Configuration**: `SessionConfig.session_timeout`
- **Default**: 30 minutes
- **Behavior**: Tracks session activity and expires inactive sessions

### 4. Database Timeouts
- **Purpose**: Prevent database operations from blocking indefinitely
- **Configuration**: `SessionConfig.database_timeout`
- **Default**: 10 seconds
- **Behavior**: Wraps database operations with timeout enforcement

## Configuration

### Server Configuration

```toml
[server]
host = "127.0.0.1"
port = 8080
max_connections = 1000
request_timeout = 30           # Request timeout in seconds
keep_alive_timeout = 5         # Keep-alive timeout in seconds
header_timeout = 10            # Header reading timeout in seconds
connection_timeout = 15        # Connection establishment timeout in seconds
max_header_size = 8192
max_body_size = 10485760       # 10MB
```

### Session Configuration

```toml
[session]
cookie_name = "cursed_session"
max_age = 86400                # 24 hours in seconds
secure = false
http_only = true
same_site = "Lax"
store_type = "Memory"
cleanup_interval = 300         # 5 minutes in seconds
database_timeout = 10          # Database operation timeout in seconds
session_timeout = 1800         # Session timeout in seconds (30 minutes)
```

### Timeout Middleware Configuration

```cursed
sus timeout_config = web::TimeoutConfig {
    enable_request_timeout: true,
    enable_connection_timeout: true,
    enable_session_timeout: true,
    enable_database_timeout: true,
    graceful_shutdown_timeout: time::Duration::from_secs(10),
    cleanup_interval: time::Duration::from_secs(60),
    log_timeout_events: true,
};
```

## Usage

### Basic Setup

```cursed
// Create server and session configurations
sus server_config = web::ServerConfig::default();
sus session_config = web::SessionConfig::default();

// Create timeout middleware
sus timeout_middleware = web::TimeoutMiddleware::new(server_config, session_config)
    .with_config(timeout_config);

// Add to middleware chain
sus middleware_chain = web::MiddlewareChain::new([
    web::LoggingMiddleware::new(),
    timeout_middleware,
    web::CorsMiddleware::new(),
]);
```

### Request Timeout Handling

The timeout middleware automatically tracks request timing:

```cursed
router.get("/api/data", |context, response| async {
    // This request is automatically tracked for timeout
    // If it exceeds request_timeout, it will be terminated
    
    // Your handler logic here
    let data = fetch_data().await;
    response.set_json(&data);
    Ok(())
});
```

### Session Timeout Management

```cursed
// Create session manager with timeout support
let session_manager = web::TimeoutSessionManager::new(session_config)?;

// Load session with timeout enforcement
match session_manager.load_session_with_timeout(session_id, &timeout_middleware).await {
    Ok(Some(session)) => {
        // Session loaded successfully
        // Activity is automatically updated
    }
    Ok(None) => {
        // Session not found or expired
        let new_session = session_manager.create_session_with_timeout(&timeout_middleware).await?;
    }
    Err(timeout_error) => {
        // Session operation timed out
        response.set_status(web::StatusCode::GATEWAY_TIMEOUT);
        response.set_text(&format!("Session timeout: {}", timeout_error));
    }
}
```

### Database Operation Timeouts

```cursed
// Wrap database operations with timeout
let result = timeout_middleware.with_database_timeout(
    "query_123".to_string(),      // Operation ID
    "SELECT".to_string(),         // Operation type
    async {
        // Your database operation
        database.execute_query("SELECT * FROM users").await
    }
).await;

match result {
    Ok(data) => {
        // Operation completed within timeout
        response.set_json(&data);
    }
    Err(web::TimeoutError::DatabaseTimeout { elapsed, timeout, operation }) => {
        // Operation timed out
        response.set_status(web::StatusCode::GATEWAY_TIMEOUT);
        response.set_text(&format!("Database {} timed out after {}ms", operation, elapsed.as_millis()));
    }
}
```

### Connection Timeout Tracking

```cursed
// Start tracking a new connection
timeout_middleware.start_connection_timeout(
    connection_id.clone(),
    Some(client_ip.clone())
);

// Update connection activity (called automatically by middleware)
timeout_middleware.update_connection_activity(&connection_id);

// Stop tracking when connection closes
timeout_middleware.stop_connection_timeout(&connection_id);
```

## Timeout Error Types

The timeout system provides specific error types for different timeout scenarios:

```cursed
pub enum TimeoutError {
    RequestTimeout { elapsed: Duration, timeout: Duration },
    ConnectionTimeout { elapsed: Duration, timeout: Duration },
    SessionTimeout { elapsed: Duration, timeout: Duration },
    DatabaseTimeout { elapsed: Duration, timeout: Duration, operation: String },
    GracefulShutdownTimeout,
}
```

## Monitoring and Statistics

### Get Timeout Statistics

```cursed
let stats = timeout_middleware.get_timeout_statistics();
println!("Active requests: {}", stats.active_requests);
println!("Active connections: {}", stats.active_connections);
println!("Active sessions: {}", stats.active_sessions);
println!("Active database operations: {}", stats.active_database_operations);
```

### Cleanup Expired Timeouts

```cursed
// Manual cleanup (also runs automatically)
timeout_middleware.cleanup_expired_timeouts();
```

## Best Practices

### 1. Appropriate Timeout Values

- **Request timeouts**: 30-60 seconds for typical web requests
- **Connection timeouts**: 10-30 seconds for connection establishment
- **Session timeouts**: 15-60 minutes depending on application security requirements
- **Database timeouts**: 5-30 seconds depending on query complexity

### 2. Error Handling

Always handle timeout errors gracefully:

```cursed
match operation_result {
    Ok(data) => {
        // Success path
    }
    Err(TimeoutError::RequestTimeout { elapsed, timeout }) => {
        response.set_status(StatusCode::REQUEST_TIMEOUT);
        response.set_text("Request took too long to process");
    }
    Err(TimeoutError::DatabaseTimeout { operation, .. }) => {
        response.set_status(StatusCode::GATEWAY_TIMEOUT);
        response.set_text(&format!("Database operation '{}' timed out", operation));
    }
    // Handle other timeout errors...
}
```

### 3. Logging and Monitoring

Enable timeout event logging for debugging and monitoring:

```cursed
sus timeout_config = web::TimeoutConfig {
    log_timeout_events: true,
    // ... other config
};
```

### 4. Graceful Shutdown

Implement graceful shutdown with timeout:

```cursed
tokio::select! {
    result = server.run() => {
        // Server completed
    }
    _ = shutdown_signal => {
        println!("Shutting down gracefully...");
    }
    _ = time::sleep(graceful_shutdown_timeout) => {
        println!("Graceful shutdown timeout, forcing exit");
    }
}
```

### 5. Session Store Timeouts

Use timeout-aware session stores for better reliability:

```cursed
// Memory store with timeout support
let store = TimeoutMemorySessionStore::new(session_config);

// File store with timeout support
let store = TimeoutFileSessionStore::new(path, session_config)?;
```

## Integration with Async Operations

The timeout system is fully integrated with Rust's async ecosystem:

- Uses `tokio::time::timeout` for async operation wrapping
- Compatible with `parking_lot` for efficient synchronization
- Provides `Future`-based timeout wrappers
- Supports cooperative cancellation

## Performance Considerations

- **Minimal overhead**: Timeout tracking adds minimal performance impact
- **Efficient cleanup**: Automatic cleanup runs periodically to prevent memory leaks
- **Lock-free operations**: Uses `parking_lot` for efficient concurrent access
- **Batched processing**: Cleanup operations are batched for efficiency

## Testing Timeout Behavior

### Unit Tests

```cursed
#[tokio::test]
async fn test_request_timeout() {
    let middleware = TimeoutMiddleware::new(server_config, session_config);
    
    // Test that requests are tracked
    let context = RequestContext::new("GET".to_string(), "/test".to_string());
    middleware.start_request_timeout(&context);
    
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_requests, 1);
}
```

### Integration Tests

```cursed
#[tokio::test]
async fn test_database_timeout() {
    let middleware = TimeoutMiddleware::new(server_config, session_config);
    
    // Test that long operations timeout
    let result = middleware.with_database_timeout(
        "test_op".to_string(),
        "SLOW_QUERY".to_string(),
        async {
            time::sleep(Duration::from_secs(20)).await; // Longer than timeout
            "result"
        }
    ).await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TimeoutError::DatabaseTimeout { .. }));
}
```

## Common Patterns

### Retry with Timeout

```cursed
async facts retry_with_timeout<F, T>(
    timeout_middleware: &TimeoutMiddleware,
    operation_id: String,
    operation_type: String,
    operation: F,
    max_retries: usize
) -> TimeoutResult<T>
where
    F: Fn() -> impl Future<Output = Result<T, Box<dyn std::error::Error>>>,
{
    let mut retries = 0;
    
    lowkey (retries < max_retries) {
        match timeout_middleware.with_database_timeout(
            format!("{}_{}", operation_id, retries),
            operation_type.clone(),
            operation()
        ).await {
            Ok(result) => match result {
                Ok(value) => return Ok(value),
                Err(_) => {
                    retries += 1;
                    time::sleep(Duration::from_millis(100)).await;
                }
            },
            Err(timeout_error) => return Err(timeout_error),
        }
    }
    
    Err(TimeoutError::DatabaseTimeout {
        elapsed: Duration::from_secs(0),
        timeout: Duration::from_secs(0),
        operation: format!("Max retries exceeded for {}", operation_type),
    })
}
```

### Circuit Breaker with Timeout

```cursed
struct CircuitBreaker {
    failure_count: AtomicUsize,
    last_failure: Mutex<Option<Instant>>,
    timeout_threshold: Duration,
    failure_threshold: usize,
}

impl CircuitBreaker {
    async facts call_with_timeout<F, T>(
        &self,
        timeout_middleware: &TimeoutMiddleware,
        operation_id: String,
        operation_type: String,
        operation: F
    ) -> TimeoutResult<T>
    where
        F: Future<Output = T>,
    {
        if self.is_open() {
            return Err(TimeoutError::DatabaseTimeout {
                elapsed: Duration::from_secs(0),
                timeout: Duration::from_secs(0),
                operation: "Circuit breaker is open".to_string(),
            });
        }
        
        match timeout_middleware.with_database_timeout(operation_id, operation_type, operation).await {
            Ok(result) => {
                self.failure_count.store(0, Ordering::Relaxed);
                Ok(result)
            }
            Err(timeout_error) => {
                self.failure_count.fetch_add(1, Ordering::Relaxed);
                *self.last_failure.lock().unwrap() = Some(Instant::now());
                Err(timeout_error)
            }
        }
    }
    
    facts is_open(&self) -> bool {
        let failure_count = self.failure_count.load(Ordering::Relaxed);
        if failure_count < self.failure_threshold {
            return false;
        }
        
        if let Some(last_failure) = *self.last_failure.lock().unwrap() {
            last_failure.elapsed() < self.timeout_threshold
        } else {
            false
        }
    }
}
```

This comprehensive timeout system ensures that CURSED web applications are robust, responsive, and capable of handling various timeout scenarios gracefully while providing excellent observability and monitoring capabilities.
