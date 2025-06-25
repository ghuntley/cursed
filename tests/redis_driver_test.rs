/// fr fr Comprehensive Redis driver tests - making sure our Redis game is strong periodt!
///
/// This test suite validates the Redis driver functionality including basic operations,
/// data structures, connection management, error handling, and performance characteristics.

use std::collections::HashMap;
use tokio::time::{sleep, Duration};

use cursed::stdlib::packages::db_nosql::redis::{
    RedisDriver, RedisConnection, RedisConfig, RedisConnectionPool, redis_string_to_value, value_to_redis_string
};
use cursed::stdlib::packages::db_nosql::drivers::{NoSqlDriver, NoSqlConnection};
use cursed::stdlib::value::Value;

/// fr fr Test helper to create Redis connection
async fn create_test_connection() -> Result<Box<dyn NoSqlConnection>, Box<dyn std::error::Error>> {
    let driver = RedisDriver::new();
    driver.connect("redis://localhost:6379").await
}

/// fr fr Test helper to create Redis connection with custom config
async fn create_connection_with_config(config: RedisConfig) -> Result<RedisConnection, Box<dyn std::error::Error>> {
    let pool = RedisConnectionPool::new(config).await?;
    let connection = RedisConnection::new(pool).await?;
    Ok(connection)
}

#[tokio::test]
async fn test_redis_config_creation() {
    // Test default configuration
    let default_config = RedisConfig::default();
    assert_eq!(default_config.url, "redis://localhost:6379");
    assert_eq!(default_config.database, 0);
    assert_eq!(default_config.max_connections, 10);
    assert_eq!(default_config.min_connections, 1);
    
    // Test configuration validation
    assert!(default_config.validate().is_ok());
}

#[tokio::test]
async fn test_redis_config_from_url() {
    // Test URL parsing with all components
    let config = RedisConfig::from_url("redis://user:pass@localhost:6380/1?connection_timeout=10000&db=5").unwrap();
    
    assert_eq!(config.username, Some("user".to_string()));
    assert_eq!(config.password, Some("pass".to_string()));
    assert_eq!(config.database, 5); // db query param should override URL path
    assert_eq!(config.connection_timeout, 10000);
    
    // Test simple URL
    let simple_config = RedisConfig::from_url("redis://localhost").unwrap();
    assert_eq!(simple_config.url, "redis://localhost");
    assert_eq!(simple_config.database, 0);
}

#[tokio::test]
async fn test_redis_config_validation() {
    let mut config = RedisConfig::default();
    
    // Valid configuration
    assert!(config.validate().is_ok());
    
    // Empty URL should fail
    config.url = "".to_string();
    assert!(config.validate().is_err());
    
    // Zero max connections should fail
    config.url = "redis://localhost".to_string();
    config.max_connections = 0;
    assert!(config.validate().is_err());
    
    // Min > Max connections should fail
    config.max_connections = 10;
    config.min_connections = 20;
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_value_conversions() {
    // Test value to Redis string conversion
    assert_eq!(value_to_redis_string(&Value::Null), "");
    assert_eq!(value_to_redis_string(&Value::Bool(true)), "true");
    assert_eq!(value_to_redis_string(&Value::Bool(false)), "false");
    assert_eq!(value_to_redis_string(&Value::Integer(42)), "42");
    assert_eq!(value_to_redis_string(&Value::Number(3.14)), "3.14");
    assert_eq!(value_to_redis_string(&Value::String("hello".to_string())), "hello");
    
    // Test array conversion (should be JSON)
    let array = Value::Array(vec![Value::Integer(1), Value::String("test".to_string())]);
    let array_str = value_to_redis_string(&array);
    assert!(array_str.contains("1"));
    assert!(array_str.contains("test"));
    
    // Test object conversion (should be JSON)
    let mut obj_map = HashMap::new();
    obj_map.insert("key".to_string(), Value::String("value".to_string()));
    let object = Value::Object(obj_map);
    let obj_str = value_to_redis_string(&object);
    assert!(obj_str.contains("key"));
    assert!(obj_str.contains("value"));
}

#[tokio::test]
async fn test_redis_string_to_value_conversions() {
    // Test integer conversion
    assert_eq!(redis_string_to_value("42"), Value::Integer(42));
    assert_eq!(redis_string_to_value("-123"), Value::Integer(-123));
    
    // Test float conversion
    assert_eq!(redis_string_to_value("3.14"), Value::Number(3.14));
    assert_eq!(redis_string_to_value("-2.5"), Value::Number(-2.5));
    
    // Test boolean conversion
    assert_eq!(redis_string_to_value("true"), Value::Bool(true));
    assert_eq!(redis_string_to_value("false"), Value::Bool(false));
    assert_eq!(redis_string_to_value("TRUE"), Value::Bool(true));
    assert_eq!(redis_string_to_value("FALSE"), Value::Bool(false));
    
    // Test string conversion (fallback)
    assert_eq!(redis_string_to_value("hello world"), Value::String("hello world".to_string()));
    assert_eq!(redis_string_to_value(""), Value::String("".to_string()));
    
    // Test JSON array conversion
    let json_array = r#"[1,"test",true]"#;
    let result = redis_string_to_value(json_array);
    assert!(result.is_array());
    
    // Test JSON object conversion
    let json_object = r#"{"name":"test","value":42}"#;
    let result = redis_string_to_value(json_object);
    assert!(result.is_object());
}

#[tokio::test]
async fn test_driver_creation() {
    // Test default driver creation
    let driver = RedisDriver::new();
    assert_eq!(driver.config.url, "redis://localhost:6379");
    
    // Test driver with custom config
    let custom_config = RedisConfig {
        url: "redis://example.com:6379".to_string(),
        database: 2,
        max_connections: 5,
        ..Default::default()
    };
    let driver_with_config = RedisDriver::with_config(custom_config.clone());
    assert_eq!(driver_with_config.config.url, "redis://example.com:6379");
    assert_eq!(driver_with_config.config.database, 2);
    assert_eq!(driver_with_config.config.max_connections, 5);
    
    // Test driver from URL
    let driver_from_url = RedisDriver::from_url("redis://test.redis:1234").unwrap();
    assert_eq!(driver_from_url.config.url, "redis://test.redis:1234");
}

// Integration tests - these require a running Redis instance
#[tokio::test]
#[ignore] // Run with --ignored flag when Redis is available
async fn test_redis_connection() {
    let driver = RedisDriver::new();
    let connection_result = driver.connect("redis://localhost:6379").await;
    
    match connection_result {
        Ok(_connection) => {
            println!("✅ Successfully connected to Redis");
        }
        Err(e) => {
            println!("❌ Failed to connect to Redis: {}", e);
            println!("   Make sure Redis is running on localhost:6379");
        }
    }
}

#[tokio::test]
#[ignore] // Run with --ignored flag when Redis is available
async fn test_basic_redis_operations() {
    let connection_result = create_test_connection().await;
    if connection_result.is_err() {
        println!("Skipping test - Redis not available");
        return;
    }
    
    let mut connection = connection_result.unwrap();
    
    // Cast to RedisConnection for specific operations
    let redis_conn = connection.as_any_mut().downcast_mut::<RedisConnection>().unwrap();
    
    // Test SET and GET
    redis_conn.set("test:key", &Value::String("test value".to_string())).await.unwrap();
    let result = redis_conn.get("test:key").await.unwrap();
    assert_eq!(result, Some(Value::String("test value".to_string())));
    
    // Test different value types
    redis_conn.set("test:int", &Value::Integer(42)).await.unwrap();
    redis_conn.set("test:bool", &Value::Bool(true)).await.unwrap();
    redis_conn.set("test:float", &Value::Number(3.14)).await.unwrap();
    
    let int_result = redis_conn.get("test:int").await.unwrap();
    let bool_result = redis_conn.get("test:bool").await.unwrap();
    let float_result = redis_conn.get("test:float").await.unwrap();
    
    assert!(int_result.is_some());
    assert!(bool_result.is_some());
    assert!(float_result.is_some());
    
    // Test EXISTS
    let exists = redis_conn.exists("test:key").await.unwrap();
    assert!(exists);
    
    let not_exists = redis_conn.exists("test:nonexistent").await.unwrap();
    assert!(!not_exists);
    
    // Test DEL
    let deleted = redis_conn.del(&["test:key"]).await.unwrap();
    assert_eq!(deleted, 1);
    
    let exists_after_del = redis_conn.exists("test:key").await.unwrap();
    assert!(!exists_after_del);
    
    // Cleanup
    redis_conn.del(&["test:int", "test:bool", "test:float"]).await.ok();
}

#[tokio::test]
#[ignore] // Run with --ignored flag when Redis is available
async fn test_redis_expiration() {
    let connection_result = create_test_connection().await;
    if connection_result.is_err() {
        println!("Skipping test - Redis not available");
        return;
    }
    
    let mut connection = connection_result.unwrap();
    let redis_conn = connection.as_any_mut().downcast_mut::<RedisConnection>().unwrap();
    
    // Test SET with expiration
    redis_conn.set_ex("test:expire", &Value::String("expires soon".to_string()), 2).await.unwrap();
    
    // Check TTL
    let ttl = redis_conn.ttl("test:expire").await.unwrap();
    assert!(ttl > 0 && ttl <= 2);
    
    // Key should exist initially
    let exists = redis_conn.exists("test:expire").await.unwrap();
    assert!(exists);
    
    // Test EXPIRE command
    redis_conn.set("test:expire2", &Value::String("test".to_string())).await.unwrap();
    let expire_result = redis_conn.expire("test:expire2", 1).await.unwrap();
    assert!(expire_result);
    
    // Wait for expiration (in a real test environment, you might want a longer wait)
    sleep(Duration::from_secs(3)).await;
    
    // Keys should not exist after expiration
    let exists_after = redis_conn.exists("test:expire").await.unwrap();
    let exists_after2 = redis_conn.exists("test:expire2").await.unwrap();
    assert!(!exists_after);
    assert!(!exists_after2);
}

#[tokio::test]
#[ignore] // Run with --ignored flag when Redis is available
async fn test_redis_increment_operations() {
    let connection_result = create_test_connection().await;
    if connection_result.is_err() {
        println!("Skipping test - Redis not available");
        return;
    }
    
    let mut connection = connection_result.unwrap();
    let redis_conn = connection.as_any_mut().downcast_mut::<RedisConnection>().unwrap();
    
    // Test INCR
    redis_conn.set("test:counter", &Value::Integer(10)).await.unwrap();
    let result = redis_conn.incr("test:counter").await.unwrap();
    assert_eq!(result, 11);
    
    // Test INCRBY
    let result2 = redis_conn.incr_by("test:counter", 5).await.unwrap();
    assert_eq!(result2, 16);
    
    // Test DECR
    let result3 = redis_conn.decr("test:counter").await.unwrap();
    assert_eq!(result3, 15);
    
    // Test DECRBY
    let result4 = redis_conn.decr_by("test:counter", 3).await.unwrap();
    assert_eq!(result4, 12);
    
    // Cleanup
    redis_conn.del(&["test:counter"]).await.ok();
}

#[tokio::test]
#[ignore] // Run with --ignored flag when Redis is available
async fn test_redis_list_operations() {
    let connection_result = create_test_connection().await;
    if connection_result.is_err() {
        println!("Skipping test - Redis not available");
        return;
    }
    
    let mut connection = connection_result.unwrap();
    let redis_conn = connection.as_any_mut().downcast_mut::<RedisConnection>().unwrap();
    
    let test_values = vec![
        Value::String("item1".to_string()),
        Value::String("item2".to_string()),
        Value::String("item3".to_string()),
    ];
    
    // Test RPUSH
    let length = redis_conn.rpush("test:list", &test_values).await.unwrap();
    assert_eq!(length, 3);
    
    // Test LLEN
    let list_length = redis_conn.llen("test:list").await.unwrap();
    assert_eq!(list_length, 3);
    
    // Test LPUSH
    redis_conn.lpush("test:list", &[Value::String("item0".to_string())]).await.unwrap();
    let new_length = redis_conn.llen("test:list").await.unwrap();
    assert_eq!(new_length, 4);
    
    // Test LRANGE
    let all_items = redis_conn.lrange("test:list", 0, -1).await.unwrap();
    assert_eq!(all_items.len(), 4);
    assert_eq!(all_items[0], Value::String("item0".to_string()));
    
    // Test LPOP
    let popped = redis_conn.lpop("test:list").await.unwrap();
    assert_eq!(popped, Some(Value::String("item0".to_string())));
    
    // Test RPOP
    let popped_right = redis_conn.rpop("test:list").await.unwrap();
    assert_eq!(popped_right, Some(Value::String("item3".to_string())));
    
    // Check final length
    let final_length = redis_conn.llen("test:list").await.unwrap();
    assert_eq!(final_length, 2);
    
    // Cleanup
    redis_conn.del(&["test:list"]).await.ok();
}

#[tokio::test]
#[ignore] // Run with --ignored flag when Redis is available
async fn test_redis_set_operations() {
    let connection_result = create_test_connection().await;
    if connection_result.is_err() {
        println!("Skipping test - Redis not available");
        return;
    }
    
    let mut connection = connection_result.unwrap();
    let redis_conn = connection.as_any_mut().downcast_mut::<RedisConnection>().unwrap();
    
    let test_members = vec![
        Value::String("member1".to_string()),
        Value::String("member2".to_string()),
        Value::String("member3".to_string()),
    ];
    
    // Test SADD
    let added = redis_conn.sadd("test:set", &test_members).await.unwrap();
    assert_eq!(added, 3);
    
    // Test SCARD
    let cardinality = redis_conn.scard("test:set").await.unwrap();
    assert_eq!(cardinality, 3);
    
    // Test SISMEMBER
    let is_member = redis_conn.sismember("test:set", &Value::String("member1".to_string())).await.unwrap();
    assert!(is_member);
    
    let is_not_member = redis_conn.sismember("test:set", &Value::String("nonexistent".to_string())).await.unwrap();
    assert!(!is_not_member);
    
    // Test SMEMBERS
    let members = redis_conn.smembers("test:set").await.unwrap();
    assert_eq!(members.len(), 3);
    assert!(members.contains(&Value::String("member1".to_string())));
    assert!(members.contains(&Value::String("member2".to_string())));
    assert!(members.contains(&Value::String("member3".to_string())));
    
    // Test SREM
    let removed = redis_conn.srem("test:set", &[Value::String("member2".to_string())]).await.unwrap();
    assert_eq!(removed, 1);
    
    let new_cardinality = redis_conn.scard("test:set").await.unwrap();
    assert_eq!(new_cardinality, 2);
    
    // Cleanup
    redis_conn.del(&["test:set"]).await.ok();
}

#[tokio::test]
#[ignore] // Run with --ignored flag when Redis is available
async fn test_redis_hash_operations() {
    let connection_result = create_test_connection().await;
    if connection_result.is_err() {
        println!("Skipping test - Redis not available");
        return;
    }
    
    let mut connection = connection_result.unwrap();
    let redis_conn = connection.as_any_mut().downcast_mut::<RedisConnection>().unwrap();
    
    // Test HSET
    let is_new = redis_conn.hset("test:hash", "field1", &Value::String("value1".to_string())).await.unwrap();
    assert!(is_new);
    
    redis_conn.hset("test:hash", "field2", &Value::Integer(42)).await.unwrap();
    redis_conn.hset("test:hash", "field3", &Value::Bool(true)).await.unwrap();
    
    // Test HGET
    let value1 = redis_conn.hget("test:hash", "field1").await.unwrap();
    assert_eq!(value1, Some(Value::String("value1".to_string())));
    
    // Test HEXISTS
    let exists = redis_conn.hexists("test:hash", "field1").await.unwrap();
    assert!(exists);
    
    let not_exists = redis_conn.hexists("test:hash", "nonexistent").await.unwrap();
    assert!(!not_exists);
    
    // Test HLEN
    let hash_length = redis_conn.hlen("test:hash").await.unwrap();
    assert_eq!(hash_length, 3);
    
    // Test HKEYS
    let keys = redis_conn.hkeys("test:hash").await.unwrap();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&"field1".to_string()));
    assert!(keys.contains(&"field2".to_string()));
    assert!(keys.contains(&"field3".to_string()));
    
    // Test HVALS
    let values = redis_conn.hvals("test:hash").await.unwrap();
    assert_eq!(values.len(), 3);
    
    // Test HGETALL
    let all_fields = redis_conn.hgetall("test:hash").await.unwrap();
    assert_eq!(all_fields.len(), 3);
    assert_eq!(all_fields.get("field1"), Some(&Value::String("value1".to_string())));
    
    // Test HDEL
    let deleted = redis_conn.hdel("test:hash", &["field2"]).await.unwrap();
    assert_eq!(deleted, 1);
    
    let new_length = redis_conn.hlen("test:hash").await.unwrap();
    assert_eq!(new_length, 2);
    
    // Cleanup
    redis_conn.del(&["test:hash"]).await.ok();
}

#[tokio::test]
#[ignore] // Run with --ignored flag when Redis is available
async fn test_redis_advanced_operations() {
    let connection_result = create_test_connection().await;
    if connection_result.is_err() {
        println!("Skipping test - Redis not available");
        return;
    }
    
    let mut connection = connection_result.unwrap();
    let redis_conn = connection.as_any_mut().downcast_mut::<RedisConnection>().unwrap();
    
    // Set up test data
    redis_conn.set("test:advanced:1", &Value::String("value1".to_string())).await.unwrap();
    redis_conn.set("test:advanced:2", &Value::String("value2".to_string())).await.unwrap();
    redis_conn.set("other:key", &Value::String("other".to_string())).await.unwrap();
    
    // Test KEYS pattern matching
    let test_keys = redis_conn.keys("test:advanced:*").await.unwrap();
    assert!(test_keys.len() >= 2);
    assert!(test_keys.contains(&"test:advanced:1".to_string()));
    assert!(test_keys.contains(&"test:advanced:2".to_string()));
    
    // Test SCAN
    let (cursor, keys) = redis_conn.scan(0, Some("test:*"), Some(10)).await.unwrap();
    assert!(keys.len() >= 2);
    
    // Test PING
    let ping_result = redis_conn.ping(None).await.unwrap();
    assert!(ping_result.contains("PONG") || ping_result == "PONG");
    
    let custom_ping = redis_conn.ping(Some("Hello Redis")).await.unwrap();
    assert_eq!(custom_ping, "Hello Redis");
    
    // Test INFO
    let info = redis_conn.info(Some("server")).await.unwrap();
    assert!(info.contains("redis_version") || !info.is_empty());
    
    // Cleanup
    redis_conn.del(&["test:advanced:1", "test:advanced:2", "other:key"]).await.ok();
}

#[tokio::test]
#[ignore] // Run with --ignored flag when Redis is available  
async fn test_connection_statistics() {
    let config = RedisConfig::default();
    let connection_result = create_connection_with_config(config).await;
    
    if connection_result.is_err() {
        println!("Skipping test - Redis not available");
        return;
    }
    
    let mut connection = connection_result.unwrap();
    
    // Perform some operations to generate statistics
    connection.set("stats:test", &Value::String("test".to_string())).await.unwrap();
    connection.get("stats:test").await.unwrap();
    connection.exists("stats:test").await.unwrap();
    connection.del(&["stats:test"]).await.unwrap();
    
    // Check statistics
    let stats = connection.get_stats().await;
    assert!(stats.total_operations >= 4);
    assert!(stats.successful_operations >= 4);
    assert_eq!(stats.failed_operations, 0);
    assert!(stats.connections_created >= 1);
    assert_eq!(stats.active_connections, 1);
    
    println!("Redis connection statistics:");
    println!("  Total operations: {}", stats.total_operations);
    println!("  Successful operations: {}", stats.successful_operations);
    println!("  Failed operations: {}", stats.failed_operations);
    println!("  Average response time: {}μs", stats.avg_response_time_us);
    println!("  Connections created: {}", stats.connections_created);
    println!("  Active connections: {}", stats.active_connections);
}

#[tokio::test]
#[ignore] // Run with --ignored flag when Redis is available
async fn test_nosql_trait_implementation() {
    let connection_result = create_test_connection().await;
    if connection_result.is_err() {
        println!("Skipping test - Redis not available");
        return;
    }
    
    let mut connection = connection_result.unwrap();
    
    // Test insert via NoSQL trait
    let document = serde_json::json!({
        "name": "Test Document",
        "value": 42,
        "active": true
    });
    
    let doc_id = connection.insert("documents", document.clone()).await.unwrap();
    assert!(doc_id.starts_with("documents:"));
    
    // Test find via NoSQL trait
    let query = serde_json::json!({});
    let results = connection.find("documents", query).await.unwrap();
    assert!(results.len() >= 1);
    
    // Verify document content
    let found_doc = &results[0];
    assert_eq!(found_doc["name"], "Test Document");
    assert_eq!(found_doc["value"], 42);
    assert_eq!(found_doc["active"], true);
    
    // Cleanup - try to delete the document key
    if let Ok(mut redis_conn) = connection.as_any_mut().downcast_mut::<RedisConnection>() {
        redis_conn.del(&[&doc_id]).await.ok();
    }
}

// Performance and stress tests
#[tokio::test]
#[ignore] // Run with --ignored flag for performance testing
async fn test_redis_performance() {
    let connection_result = create_test_connection().await;
    if connection_result.is_err() {
        println!("Skipping test - Redis not available");
        return;
    }
    
    let mut connection = connection_result.unwrap();
    let redis_conn = connection.as_any_mut().downcast_mut::<RedisConnection>().unwrap();
    
    let operations = 100;
    let start_time = std::time::Instant::now();
    
    // Benchmark SET operations
    for i in 0..operations {
        let key = format!("perf:test:{}", i);
        redis_conn.set(&key, &Value::Integer(i as i64)).await.unwrap();
    }
    
    let set_duration = start_time.elapsed();
    
    // Benchmark GET operations
    let get_start = std::time::Instant::now();
    for i in 0..operations {
        let key = format!("perf:test:{}", i);
        redis_conn.get(&key).await.unwrap();
    }
    
    let get_duration = get_start.elapsed();
    
    println!("Performance results:");
    println!("  SET {} operations: {:?} ({:.2} ops/sec)", 
             operations, set_duration, 
             operations as f64 / set_duration.as_secs_f64());
    println!("  GET {} operations: {:?} ({:.2} ops/sec)", 
             operations, get_duration, 
             operations as f64 / get_duration.as_secs_f64());
    
    // Cleanup
    for i in 0..operations {
        let key = format!("perf:test:{}", i);
        redis_conn.del(&[&key]).await.ok();
    }
    
    // Verify performance is reasonable (adjust thresholds as needed)
    assert!(set_duration.as_secs() < 10, "SET operations took too long");
    assert!(get_duration.as_secs() < 10, "GET operations took too long");
}

// Error handling tests
#[tokio::test]
async fn test_error_handling() {
    // Test invalid connection URL
    let invalid_driver = RedisDriver::from_url("invalid://not-a-redis-url").unwrap();
    let connection_result = invalid_driver.connect("").await;
    assert!(connection_result.is_err());
    
    // Test connection to non-existent host
    let unreachable_driver = RedisDriver::from_url("redis://nonexistent-host:6379").unwrap();
    let connection_result = unreachable_driver.connect("").await;
    assert!(connection_result.is_err());
    
    println!("Error handling tests completed successfully");
}

#[tokio::test]
async fn test_config_edge_cases() {
    // Test empty URL validation
    let mut config = RedisConfig::default();
    config.url = "".to_string();
    assert!(config.validate().is_err());
    
    // Test zero max connections
    config.url = "redis://localhost".to_string();
    config.max_connections = 0;
    assert!(config.validate().is_err());
    
    // Test min > max connections
    config.max_connections = 5;
    config.min_connections = 10;
    assert!(config.validate().is_err());
    
    println!("Configuration edge case tests completed successfully");
}
