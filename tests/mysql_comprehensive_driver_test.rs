/// fr fr Comprehensive tests for MySQL driver implementation periodt
/// 
/// This test suite validates the comprehensive MySQL driver functionality
/// including configuration, connection management, type conversions,
/// error handling, and integration with the CURSED database system.

use cursed::stdlib::database::mysql::comprehensive_driver::{
    ComprehensiveMySqlDriver, MySqlConfig, parse_mysql_dsn, convert_to_mysql_value, 
    convert_from_mysql_value, StatementCache, DriverHealthStatus, MySqlPoolStats
};
use cursed::stdlib::database::{SqlValue, DatabaseError, DatabaseErrorKind};
use std::time::{Duration, SystemTime};
use std::collections::HashMap;

#[test]
fn test_mysql_config_validation() {
    // Test valid default configuration
    let config = MySqlConfig::default();
    assert!(config.validate().is_ok());
    
    // Test max_connections validation
    let mut invalid_config = config.clone();
    invalid_config.max_connections = 0;
    assert!(invalid_config.validate().is_err());
    
    // Test min/max connection relationship
    let mut invalid_config = config.clone();
    invalid_config.min_connections = 150;
    invalid_config.max_connections = 100;
    assert!(invalid_config.validate().is_err());
    
    // Test timeout validation
    let mut invalid_config = config.clone();
    invalid_config.connection_timeout = Duration::ZERO;
    assert!(invalid_config.validate().is_err());
    
    let mut invalid_config = config.clone();
    invalid_config.query_timeout = Duration::ZERO;
    assert!(invalid_config.validate().is_err());
    
    // Test charset validation
    let mut invalid_config = config.clone();
    invalid_config.charset = "".to_string();
    assert!(invalid_config.validate().is_err());
}

#[test]
fn test_mysql_config_defaults() {
    let config = MySqlConfig::default();
    
    assert_eq!(config.max_connections, 100);
    assert_eq!(config.min_connections, 10);
    assert_eq!(config.connection_timeout, Duration::from_secs(30));
    assert_eq!(config.query_timeout, Duration::from_secs(300));
    assert_eq!(config.max_lifetime, Some(Duration::from_secs(3600)));
    assert_eq!(config.idle_timeout, Some(Duration::from_secs(600)));
    assert!(config.ssl_enabled);
    assert!(config.ssl_verify);
    assert!(config.compression);
    assert_eq!(config.charset, "utf8mb4");
    assert_eq!(config.timezone, "UTC");
    assert!(config.foreign_key_checks);
    assert!(config.autocommit);
    assert_eq!(config.sql_mode, "STRICT_TRANS_TABLES,NO_ZERO_DATE,NO_ZERO_IN_DATE,ERROR_FOR_DIVISION_BY_ZERO");
    assert!(config.init_commands.is_empty());
}

#[test]
fn test_parse_mysql_dsn_simple() {
    // Test simple database name
    let info = parse_mysql_dsn("testdb").unwrap();
    assert_eq!(info.database, "testdb");
    assert_eq!(info.host, "localhost");
    assert_eq!(info.port, 3306);
    assert_eq!(info.username, "root");
    assert_eq!(info.password, "");
    assert!(info.parameters.is_empty());
}

#[test]
fn test_parse_mysql_dsn_full() {
    // Test full DSN with protocol
    let info = parse_mysql_dsn("mysql://user:pass@host:3307/database?charset=utf8&ssl=true").unwrap();
    assert_eq!(info.username, "user");
    assert_eq!(info.password, "pass");
    assert_eq!(info.host, "host");
    assert_eq!(info.port, 3307);
    assert_eq!(info.database, "database");
    assert_eq!(info.parameters.get("charset"), Some(&"utf8".to_string()));
    assert_eq!(info.parameters.get("ssl"), Some(&"true".to_string()));
}

#[test]
fn test_parse_mysql_dsn_host_port() {
    // Test host:port/database format
    let info = parse_mysql_dsn("myhost:3308/mydb").unwrap();
    assert_eq!(info.host, "myhost");
    assert_eq!(info.port, 3308);
    assert_eq!(info.database, "mydb");
    assert_eq!(info.username, "root");
    assert_eq!(info.password, "");
}

#[test]
fn test_parse_mysql_dsn_with_auth() {
    // Test with authentication
    let info = parse_mysql_dsn("admin:secret@dbserver:3306/production").unwrap();
    assert_eq!(info.username, "admin");
    assert_eq!(info.password, "secret");
    assert_eq!(info.host, "dbserver");
    assert_eq!(info.port, 3306);
    assert_eq!(info.database, "production");
}

#[test]
fn test_parse_mysql_dsn_complex_params() {
    // Test complex parameter parsing
    let info = parse_mysql_dsn("user@host/db?timeout=30&ssl_mode=REQUIRED&charset=utf8mb4").unwrap();
    assert_eq!(info.username, "user");
    assert_eq!(info.password, "");
    assert_eq!(info.host, "host");
    assert_eq!(info.database, "db");
    assert_eq!(info.parameters.get("timeout"), Some(&"30".to_string()));
    assert_eq!(info.parameters.get("ssl_mode"), Some(&"REQUIRED".to_string()));
    assert_eq!(info.parameters.get("charset"), Some(&"utf8mb4".to_string()));
}

#[test]
fn test_parse_mysql_dsn_errors() {
    // Test empty DSN
    assert!(parse_mysql_dsn("").is_err());
    
    // Test invalid port
    assert!(parse_mysql_dsn("host:invalid_port/db").is_err());
    assert!(parse_mysql_dsn("host:99999/db").is_err());
}

#[test]
fn test_mysql_value_conversions() {
    // Test null conversion
    let sql_null = SqlValue::Null;
    let mysql_null = convert_to_mysql_value(&sql_null).unwrap();
    assert_eq!(mysql_null, mysql::Value::NULL);
    
    // Test boolean conversion
    let sql_bool = SqlValue::Boolean(true);
    let mysql_bool = convert_to_mysql_value(&sql_bool).unwrap();
    assert_eq!(mysql_bool, mysql::Value::from(true));
    
    let sql_bool_false = SqlValue::Boolean(false);
    let mysql_bool_false = convert_to_mysql_value(&sql_bool_false).unwrap();
    assert_eq!(mysql_bool_false, mysql::Value::from(false));
    
    // Test integer conversion
    let sql_int = SqlValue::Integer(42);
    let mysql_int = convert_to_mysql_value(&sql_int).unwrap();
    assert_eq!(mysql_int, mysql::Value::from(42i64));
    
    let sql_int_negative = SqlValue::Integer(-123);
    let mysql_int_negative = convert_to_mysql_value(&sql_int_negative).unwrap();
    assert_eq!(mysql_int_negative, mysql::Value::from(-123i64));
    
    // Test float conversion
    let sql_float = SqlValue::Float(3.14159);
    let mysql_float = convert_to_mysql_value(&sql_float).unwrap();
    assert_eq!(mysql_float, mysql::Value::from(3.14159f64));
    
    // Test string conversion
    let sql_string = SqlValue::String("hello world".to_string());
    let mysql_string = convert_to_mysql_value(&sql_string).unwrap();
    assert_eq!(mysql_string, mysql::Value::from("hello world"));
    
    // Test empty string
    let sql_empty = SqlValue::String("".to_string());
    let mysql_empty = convert_to_mysql_value(&sql_empty).unwrap();
    assert_eq!(mysql_empty, mysql::Value::from(""));
    
    // Test bytes conversion
    let sql_bytes = SqlValue::Bytes(vec![1, 2, 3, 4, 5]);
    let mysql_bytes = convert_to_mysql_value(&sql_bytes).unwrap();
    assert_eq!(mysql_bytes, mysql::Value::from(vec![1, 2, 3, 4, 5]));
    
    // Test empty bytes
    let sql_empty_bytes = SqlValue::Bytes(vec![]);
    let mysql_empty_bytes = convert_to_mysql_value(&sql_empty_bytes).unwrap();
    assert_eq!(mysql_empty_bytes, mysql::Value::from(Vec::<u8>::new()));
}

#[test]
fn test_mysql_value_conversions_timestamps() {
    use std::time::UNIX_EPOCH;
    
    // Test timestamp conversion
    let timestamp = UNIX_EPOCH + Duration::from_secs(1640995200); // 2022-01-01 00:00:00 UTC
    let sql_timestamp = SqlValue::Timestamp(timestamp);
    
    // This should not panic and should produce a reasonable MySQL value
    let result = convert_to_mysql_value(&sql_timestamp);
    assert!(result.is_ok());
}

#[test]
fn test_mysql_value_conversions_json() {
    // Test JSON conversion
    let json_value = serde_json::json!({
        "name": "test",
        "count": 42,
        "active": true
    });
    let sql_json = SqlValue::Json(json_value.clone());
    let mysql_json = convert_to_mysql_value(&sql_json).unwrap();
    
    // Should convert to string representation
    assert_eq!(mysql_json, mysql::Value::from(json_value.to_string()));
    
    // Test simple JSON values
    let simple_json = serde_json::json!("simple string");
    let sql_simple = SqlValue::Json(simple_json.clone());
    let mysql_simple = convert_to_mysql_value(&sql_simple).unwrap();
    assert_eq!(mysql_simple, mysql::Value::from(simple_json.to_string()));
}

#[test]
fn test_reverse_mysql_value_conversions() {
    // Test converting MySQL values back to SqlValue
    
    // Null
    let mysql_null = mysql::Value::NULL;
    let sql_null = convert_from_mysql_value(mysql_null).unwrap();
    assert_eq!(sql_null, SqlValue::Null);
    
    // Integer
    let mysql_int = mysql::Value::Int(42);
    let sql_int = convert_from_mysql_value(mysql_int).unwrap();
    assert_eq!(sql_int, SqlValue::Integer(42));
    
    // UInt
    let mysql_uint = mysql::Value::UInt(123);
    let sql_uint = convert_from_mysql_value(mysql_uint).unwrap();
    assert_eq!(sql_uint, SqlValue::Integer(123));
    
    // Float
    let mysql_float = mysql::Value::Float(3.14);
    let sql_float = convert_from_mysql_value(mysql_float).unwrap();
    assert_eq!(sql_float, SqlValue::Float(3.14));
    
    // Double
    let mysql_double = mysql::Value::Double(2.71828);
    let sql_double = convert_from_mysql_value(mysql_double).unwrap();
    assert_eq!(sql_double, SqlValue::Float(2.71828));
    
    // Bytes (UTF-8 string)
    let mysql_string_bytes = mysql::Value::Bytes("hello".as_bytes().to_vec());
    let sql_string = convert_from_mysql_value(mysql_string_bytes).unwrap();
    assert_eq!(sql_string, SqlValue::String("hello".to_string()));
    
    // Bytes (binary data)
    let mysql_binary = mysql::Value::Bytes(vec![0xFF, 0xFE, 0xFD]);
    let sql_binary = convert_from_mysql_value(mysql_binary).unwrap();
    assert_eq!(sql_binary, SqlValue::Bytes(vec![0xFF, 0xFE, 0xFD]));
}

#[test]
fn test_statement_cache() {
    let cache = StatementCache::new(3);
    
    // Test cache miss
    assert!(cache.get("SELECT 1").is_none());
    
    // Test cache insert and hit
    cache.insert("SELECT 1".to_string(), vec![1, 2, 3]);
    assert_eq!(cache.get("SELECT 1"), Some(vec![1, 2, 3]));
    
    // Test multiple inserts
    cache.insert("SELECT 2".to_string(), vec![4, 5, 6]);
    cache.insert("SELECT 3".to_string(), vec![7, 8, 9]);
    
    // All should be available
    assert_eq!(cache.get("SELECT 1"), Some(vec![1, 2, 3]));
    assert_eq!(cache.get("SELECT 2"), Some(vec![4, 5, 6]));
    assert_eq!(cache.get("SELECT 3"), Some(vec![7, 8, 9]));
    
    // Test LRU eviction
    cache.insert("SELECT 4".to_string(), vec![10, 11, 12]);
    
    // First query should be evicted
    assert!(cache.get("SELECT 1").is_none());
    assert_eq!(cache.get("SELECT 2"), Some(vec![4, 5, 6]));
    assert_eq!(cache.get("SELECT 3"), Some(vec![7, 8, 9]));
    assert_eq!(cache.get("SELECT 4"), Some(vec![10, 11, 12]));
    
    // Check statistics
    let (hits, misses, size) = cache.stats();
    assert_eq!(hits, 6); // 6 successful retrievals
    assert_eq!(misses, 2); // 2 cache misses (initial miss + evicted item)
    assert_eq!(size, 3); // Cache size is 3
}

#[test]
fn test_statement_cache_empty() {
    let cache = StatementCache::new(0); // Zero-size cache
    
    cache.insert("SELECT 1".to_string(), vec![1, 2, 3]);
    assert!(cache.get("SELECT 1").is_none()); // Should not be cached
    
    let (hits, misses, size) = cache.stats();
    assert_eq!(hits, 0);
    assert_eq!(misses, 1);
    assert_eq!(size, 0);
}

#[test]
fn test_comprehensive_mysql_driver_creation() {
    let driver = ComprehensiveMySqlDriver::new();
    
    assert_eq!(driver.name(), "Comprehensive MySQL Driver for CURSED");
    
    let capabilities = driver.capabilities();
    assert!(capabilities.supports_transactions);
    assert!(capabilities.supports_prepared_statements);
    assert!(capabilities.supports_multiple_result_sets);
    assert!(capabilities.supports_stored_procedures);
    assert!(capabilities.supports_batch_operations);
    assert!(capabilities.supports_concurrent_connections);
    assert_eq!(capabilities.max_connections, Some(100));
    assert_eq!(capabilities.max_query_length, Some(16_777_216));
    assert_eq!(capabilities.max_parameter_count, Some(65535));
    
    // Check supported isolation levels
    let isolation_levels = &capabilities.supported_isolation_levels;
    assert!(isolation_levels.contains(&cursed::stdlib::database::SqlIsolationLevel::LevelReadUncommitted));
    assert!(isolation_levels.contains(&cursed::stdlib::database::SqlIsolationLevel::LevelReadCommitted));
    assert!(isolation_levels.contains(&cursed::stdlib::database::SqlIsolationLevel::LevelRepeatableRead));
    assert!(isolation_levels.contains(&cursed::stdlib::database::SqlIsolationLevel::LevelSerializable));
}

#[test]
fn test_comprehensive_mysql_driver_with_config() {
    let mut config = MySqlConfig::default();
    config.max_connections = 50;
    config.min_connections = 5;
    config.charset = "latin1".to_string();
    config.ssl_enabled = false;
    
    let driver = ComprehensiveMySqlDriver::with_config(config);
    
    let capabilities = driver.capabilities();
    assert_eq!(capabilities.max_connections, Some(50));
}

#[test]
fn test_mysql_pool_stats() {
    let mut stats = MySqlPoolStats::default();
    
    assert_eq!(stats.active_connections, 0);
    assert_eq!(stats.idle_connections, 0);
    assert_eq!(stats.total_connections, 0);
    assert_eq!(stats.connections_created, 0);
    assert_eq!(stats.query_count, 0);
    
    // Simulate some activity
    stats.active_connections = 5;
    stats.total_connections = 10;
    stats.connections_created = 15;
    stats.query_count = 100;
    stats.query_errors = 2;
    stats.update();
    
    assert_eq!(stats.active_connections, 5);
    assert_eq!(stats.total_connections, 10);
    assert_eq!(stats.connections_created, 15);
    assert_eq!(stats.query_count, 100);
    assert_eq!(stats.query_errors, 2);
    assert!(stats.last_updated > SystemTime::UNIX_EPOCH);
}

#[test]
fn test_driver_health_status() {
    let mut status = DriverHealthStatus::new();
    
    assert!(!status.overall_health);
    assert!(!status.pool_initialized);
    assert!(!status.basic_functionality);
    assert_eq!(status.active_connections, 0);
    assert_eq!(status.connection_errors, 0);
    assert_eq!(status.cache_hits, 0);
    
    // Simulate healthy status
    status.overall_health = true;
    status.pool_initialized = true;
    status.basic_functionality = true;
    status.active_connections = 5;
    status.cache_hits = 100;
    status.cache_misses = 10;
    status.uptime = Duration::from_secs(3600);
    
    assert!(status.overall_health);
    assert!(status.pool_initialized);
    assert!(status.basic_functionality);
    assert_eq!(status.active_connections, 5);
    assert_eq!(status.cache_hits, 100);
    assert_eq!(status.cache_misses, 10);
    assert_eq!(status.uptime, Duration::from_secs(3600));
}

#[test]
fn test_driver_clone() {
    let driver = ComprehensiveMySqlDriver::new();
    let cloned_driver = driver.clone();
    
    assert_eq!(driver.name(), cloned_driver.name());
    assert_eq!(driver.capabilities().max_connections, cloned_driver.capabilities().max_connections);
}

#[test]
fn test_cache_stats_concurrent_access() {
    use std::sync::Arc;
    use std::thread;
    
    let cache = Arc::new(StatementCache::new(10));
    let mut handles = vec![];
    
    // Spawn multiple threads to test concurrent access
    for i in 0..5 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let key = format!("SELECT {} FROM table{}", i, j);
                let value = vec![i as u8, j as u8];
                cache_clone.insert(key.clone(), value.clone());
                
                // Try to retrieve it
                let retrieved = cache_clone.get(&key);
                if retrieved.is_some() {
                    assert_eq!(retrieved.unwrap(), value);
                }
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Check final stats
    let (hits, misses, size) = cache.stats();
    assert!(hits > 0 || misses > 0); // Some activity should have occurred
    assert!(size <= 10); // Should not exceed cache size
}

#[test]
fn test_comprehensive_driver_stats_operations() {
    let driver = ComprehensiveMySqlDriver::new();
    
    // Test getting initial stats
    let stats = driver.get_stats().unwrap();
    assert_eq!(stats.active_connections, 0);
    assert_eq!(stats.query_count, 0);
    
    // Test updating stats
    let result = driver.update_stats(|stats| {
        stats.query_count += 10;
        stats.active_connections = 3;
    });
    assert!(result.is_ok());
    
    // Verify updated stats
    let updated_stats = driver.get_stats().unwrap();
    assert_eq!(updated_stats.query_count, 10);
    assert_eq!(updated_stats.active_connections, 3);
}

#[test]
fn test_edge_cases_dsn_parsing() {
    // Test DSN with only parameters
    let info = parse_mysql_dsn("?charset=utf8&timeout=30").unwrap();
    assert_eq!(info.host, "localhost");
    assert_eq!(info.database, "test");
    assert_eq!(info.parameters.get("charset"), Some(&"utf8".to_string()));
    
    // Test DSN with empty database
    let info = parse_mysql_dsn("user:pass@host:3306/").unwrap();
    assert_eq!(info.username, "user");
    assert_eq!(info.host, "host");
    assert_eq!(info.database, "test"); // Should use default
    
    // Test DSN with just host
    let info = parse_mysql_dsn("myhost").unwrap();
    assert_eq!(info.host, "myhost");
    assert_eq!(info.port, 3306);
    assert_eq!(info.database, "test");
    
    // Test DSN with special characters in password
    let info = parse_mysql_dsn("user:p@ssw0rd!@host/db").unwrap();
    assert_eq!(info.username, "user");
    assert_eq!(info.password, "p@ssw0rd!");
    assert_eq!(info.host, "host");
    assert_eq!(info.database, "db");
}

#[test]
fn test_value_conversion_edge_cases() {
    // Test large integers
    let large_int = SqlValue::Integer(i64::MAX);
    let mysql_large = convert_to_mysql_value(&large_int).unwrap();
    assert_eq!(mysql_large, mysql::Value::from(i64::MAX));
    
    let negative_int = SqlValue::Integer(i64::MIN);
    let mysql_negative = convert_to_mysql_value(&negative_int).unwrap();
    assert_eq!(mysql_negative, mysql::Value::from(i64::MIN));
    
    // Test special float values
    let nan_float = SqlValue::Float(f64::NAN);
    let mysql_nan = convert_to_mysql_value(&nan_float).unwrap();
    if let mysql::Value::Double(d) = mysql_nan {
        assert!(d.is_nan());
    } else {
        panic!("Expected Double value");
    }
    
    let inf_float = SqlValue::Float(f64::INFINITY);
    let mysql_inf = convert_to_mysql_value(&inf_float).unwrap();
    if let mysql::Value::Double(d) = mysql_inf {
        assert!(d.is_infinite() && d.is_sign_positive());
    } else {
        panic!("Expected Double value");
    }
    
    let neg_inf_float = SqlValue::Float(f64::NEG_INFINITY);
    let mysql_neg_inf = convert_to_mysql_value(&neg_inf_float).unwrap();
    if let mysql::Value::Double(d) = mysql_neg_inf {
        assert!(d.is_infinite() && d.is_sign_negative());
    } else {
        panic!("Expected Double value");
    }
}

#[test]
fn test_driver_error_scenarios() {
    let driver = ComprehensiveMySqlDriver::new();
    
    // Test opening connection without pool initialization should handle gracefully
    let result = driver.open("mysql://nonexistent:invalid@nowhere:9999/nodata");
    // This should not panic and should return a connection (even if it's a placeholder)
    assert!(result.is_ok());
}

#[test]
fn test_performance_considerations() {
    // Test that driver creation is fast
    let start = std::time::Instant::now();
    let _driver = ComprehensiveMySqlDriver::new();
    let duration = start.elapsed();
    
    // Driver creation should be very fast (less than 10ms)
    assert!(duration < Duration::from_millis(10));
    
    // Test that stats operations are fast
    let driver = ComprehensiveMySqlDriver::new();
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = driver.get_stats();
    }
    let duration = start.elapsed();
    
    // 1000 stats operations should complete quickly (less than 100ms)
    assert!(duration < Duration::from_millis(100));
}
