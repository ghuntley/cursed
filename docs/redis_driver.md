# CURSED Redis Database Driver

## Overview

The CURSED Redis driver provides comprehensive Redis database connectivity and operations for the CURSED programming language. It offers high-performance caching, session storage, and key-value operations with full async support and connection pooling.

## Features

### 🔥 Core Features
- **Full Redis Operations**: GET, SET, DEL, EXISTS, EXPIRE, TTL, INCR, DECR
- **Data Structures**: Lists, Sets, Hashes with all operations
- **Connection Pooling**: Efficient connection management with configurable pools
- **Async/Await Support**: Non-blocking operations with tokio integration
- **Error Handling**: Comprehensive error types with retry logic
- **Performance Monitoring**: Built-in statistics and metrics tracking

### 🚀 Advanced Features
- **CURSED Value Integration**: Seamless conversion between CURSED and Redis types
- **Configuration Management**: Flexible configuration with URL parsing
- **SSL/TLS Support**: Secure connections with certificate validation
- **Connection Retry**: Automatic reconnection with exponential backoff
- **Pattern Matching**: KEYS and SCAN operations for flexible queries
- **Expiration Management**: TTL operations and automatic key expiration

## Quick Start

### Installation

The Redis driver is included in the CURSED standard library:

```cursed
import "stdlib::packages::db_nosql::redis";
```

### Basic Usage

```cursed
async slay main() {
    // Create driver and connect
    facts driver = RedisDriver::new();
    sus connection = driver.connect("redis://localhost:6379").await?;
    
    // Basic key-value operations
    connection.set("user:1", &Value::string("John Doe")).await?;
    facts user = connection.get("user:1").await?;
    println("User: {:?}", user)?;
}
```

## Configuration

### Connection Strings

```cursed
// Basic connection
"redis://localhost:6379"

// With authentication
"redis://user:password@localhost:6379"

// With database selection
"redis://localhost:6379/2"

// With SSL and parameters
"redis://user:pass@redis.example.com:6380/1?connection_timeout=5000"
```

### Configuration Object

```cursed
facts config = RedisConfig {
    url: "redis://production-redis:6379".to_string(),
    database: 0,
    password: Some("secure-password".to_string()),
    connection_timeout: 5000,
    response_timeout: 30000,
    max_connections: 20,
    min_connections: 2,
    use_tls: true,
    verify_ssl: true,
    retry_attempts: 3,
    retry_delay: 1000,
    ..Default::default()
};

facts driver = RedisDriver::with_config(config);
```

## Operations Reference

### Basic Operations

```cursed
// SET and GET
connection.set("key", &Value::string("value")).await?;
facts value = connection.get("key").await?;

// SET with expiration
connection.set_ex("session:123", &Value::string("data"), 3600).await?;

// Check existence
facts exists = connection.exists("key").await?;

// Delete keys
connection.del(&["key1", "key2"]).await?;

// Increment/Decrement
connection.incr("counter").await?;
connection.decr_by("counter", 5).await?;
```

### List Operations

```cursed
// Push to list
facts values = vec![Value::string("item1"), Value::string("item2")];
connection.rpush("list", &values).await?;
connection.lpush("list", &[Value::string("urgent")]).await?;

// Pop from list
facts item = connection.lpop("list").await?;
facts last = connection.rpop("list").await?;

// Get range
facts all_items = connection.lrange("list", 0, -1).await?;
facts length = connection.llen("list").await?;
```

### Set Operations

```cursed
// Add to set
facts members = vec![Value::string("a"), Value::string("b")];
connection.sadd("set", &members).await?;

// Check membership
facts is_member = connection.sismember("set", &Value::string("a")).await?;

// Get all members
facts all_members = connection.smembers("set").await?;

// Remove members
connection.srem("set", &[Value::string("b")]).await?;
```

### Hash Operations

```cursed
// Set hash fields
connection.hset("user:1", "name", &Value::string("John")).await?;
connection.hset("user:1", "age", &Value::integer(30)).await?;

// Get field
facts name = connection.hget("user:1", "name").await?;

// Get all fields
facts profile = connection.hgetall("user:1").await?;

// Delete field
connection.hdel("user:1", &["age"]).await?;
```

### Advanced Operations

```cursed
// Pattern matching
facts keys = connection.keys("user:*").await?;

// Incremental scanning
facts (cursor, keys) = connection.scan(0, Some("session:*"), Some(10)).await?;

// Server info
facts info = connection.info(Some("memory")).await?;

// Test connection
facts pong = connection.ping(Some("Hello Redis")).await?;
```

## Error Handling

```cursed
use crate::stdlib::packages::db_core::error::{DatabaseError, ConnectionError};

match connection.get("key").await {
    Ok(Some(value)) => println("Found: {:?}", value)?,
    Ok(None) => println("Key not found")?,
    Err(DatabaseError { kind: ErrorKind::Connection(ConnectionError::Timeout), .. }) => {
        println("Connection timeout - retrying...")?,
        // Implement retry logic
    },
    Err(e) => println("Error: {}", e)?
}
```

## Performance Monitoring

```cursed
// Get connection statistics
facts stats = connection.get_stats().await;
println("Operations: {}", stats.total_operations)?;
println("Success rate: {:.2}%", 
       (stats.successful_operations as f64 / stats.total_operations as f64) * 100.0)?;
println("Avg response time: {}μs", stats.avg_response_time_us)?;
```

## Testing

### Unit Tests (No Redis Required)

```bash
make redis-test-unit
```

### Integration Tests (Redis Required)

```bash
# Start Redis
redis-server

# Run tests
make redis-test-integration
```

### Performance Tests

```bash
make redis-test-performance
```

### Complete Test Suite

```bash
make redis-test-all
```

## Configuration Examples

### Development

```cursed
facts dev_config = RedisConfig {
    url: "redis://localhost:6379".to_string(),
    database: 0,
    max_connections: 5,
    connection_timeout: 5000,
    ..Default::default()
};
```

### Production

```cursed
facts prod_config = RedisConfig {
    url: "redis://redis-cluster.production.com:6379".to_string(),
    database: 0,
    password: Some(env::var("REDIS_PASSWORD")?),
    max_connections: 50,
    min_connections: 10,
    connection_timeout: 10000,
    response_timeout: 60000,
    use_tls: true,
    verify_ssl: true,
    retry_attempts: 5,
    retry_delay: 2000,
    ..Default::default()
};
```

### High Availability

```cursed
facts ha_config = RedisConfig {
    url: "redis://redis-sentinel:26379".to_string(),
    max_connections: 100,
    min_connections: 20,
    idle_timeout: 600,
    retry_attempts: 10,
    retry_delay: 1000,
    ..Default::default()
};
```

## Best Practices

### Connection Management

1. **Use Connection Pooling**: Configure appropriate pool sizes for your workload
2. **Handle Retries**: Implement exponential backoff for transient failures
3. **Monitor Performance**: Track statistics and adjust configuration accordingly
4. **Graceful Shutdown**: Properly close connections when shutting down

### Data Organization

1. **Key Naming**: Use consistent patterns like `prefix:id:field`
2. **Expiration**: Set appropriate TTLs for temporary data
3. **Data Types**: Choose the right Redis data structure for your use case
4. **Memory Efficiency**: Use compression for large values

### Error Handling

1. **Categorize Errors**: Handle different error types appropriately
2. **Implement Fallbacks**: Have backup strategies for critical operations
3. **Log Appropriately**: Include context in error messages
4. **Monitor Health**: Track error rates and patterns

## Troubleshooting

### Common Issues

1. **Connection Refused**: Check if Redis is running and accessible
2. **Authentication Failed**: Verify credentials and user permissions
3. **Timeout Errors**: Adjust timeout settings or check network connectivity
4. **Memory Issues**: Monitor Redis memory usage and configure appropriately

### Debug Mode

```cursed
// Enable debug logging
use tracing::{info, debug, error};

debug!("Connecting to Redis at {}", config.url);
info!("Operation completed successfully");
```

### Health Checks

```cursed
async slay health_check() -> DatabaseResult<bool> {
    match connection.ping(None).await {
        Ok(_) => {
            info!("Redis health check passed");
            Ok(true)
        }
        Err(e) => {
            error!("Redis health check failed: {}", e);
            Ok(false)
        }
    }
}
```

## Integration with CURSED

The Redis driver seamlessly integrates with CURSED's type system and error handling:

- **Value Types**: All CURSED values are automatically converted to/from Redis formats
- **Error System**: Uses CURSED's unified error handling with proper categorization
- **Async Support**: Built on tokio for non-blocking operations
- **Memory Safety**: Automatic resource management and cleanup

## Conclusion

The CURSED Redis driver provides production-ready Redis connectivity with comprehensive features, excellent performance, and seamless integration with the CURSED ecosystem. It's suitable for caching, session storage, and high-performance key-value operations in modern applications.

For more examples and advanced usage, see the `examples/redis_demo.csd` file and the comprehensive test suite.
