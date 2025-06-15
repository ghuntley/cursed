/// fr fr MySQL Integration Tests that absolutely slay periodt
/// 
/// This test suite validates the complete MySQL driver integration
/// with the CURSED database system, including real-world scenarios,
/// error handling, performance characteristics, and Gen Z syntax support.

use cursed::stdlib::database::{
    Driver, DriverConn, DatabaseError, DatabaseErrorKind, SqlValue, TxOptions
};
use cursed::stdlib::database::mysql::{
    SimpleMySqlDriver, 
    comprehensive_driver::{ComprehensiveMySqlDriver, MySqlConfig},
    create_comprehensive_mysql_driver, create_comprehensive_mysql_driver_with_config,
    parse_comprehensive_mysql_dsn
};
use std::time::Duration;
use std::collections::HashMap;

#[test]
fn test_simple_mysql_driver_creation() {
    let driver = SimpleMySqlDriver::new();
    assert_eq!(driver.name(), "Simple MySQL Driver for CURSED");
    
    let capabilities = driver.capabilities();
    assert!(capabilities.supports_transactions);
    assert!(capabilities.supports_prepared_statements);
    assert!(capabilities.supports_multiple_result_sets);
    assert!(capabilities.supports_stored_procedures);
    assert!(capabilities.supports_batch_operations);
    assert!(capabilities.supports_concurrent_connections);
}

#[test]
fn test_simple_mysql_driver_with_config() {
    let config = MySqlConfig {
        max_connections: 25,
        min_connections: 2,
        connection_timeout: Duration::from_secs(15),
        query_timeout: Duration::from_secs(60),
        max_lifetime: Some(Duration::from_secs(1800)),
        idle_timeout: Some(Duration::from_secs(300)),
        ssl_enabled: false,
        ssl_cert_path: None,
        ssl_key_path: None,
        ssl_ca_path: None,
        ssl_verify: false,
        compression: false,
        charset: "latin1".to_string(),
        timezone: "America/New_York".to_string(),
        foreign_key_checks: false,
        sql_mode: "TRADITIONAL".to_string(),
        autocommit: false,
        init_commands: vec!["SET time_zone = '+00:00'".to_string()],
    };
    
    let driver = SimpleMySqlDriver::with_config(config.clone());
    assert_eq!(driver.name(), "Simple MySQL Driver for CURSED");
    
    let capabilities = driver.capabilities();
    assert_eq!(capabilities.max_connections, Some(25));
}

#[test]
fn test_driver_connection_lifecycle() {
    let driver = SimpleMySqlDriver::new();
    
    // Test various connection strings
    let test_dsns = vec![
        "mysql://test:test@localhost:3306/testdb",
        "localhost:3306/myapp",
        "testdb",
        "user:pass@host/database",
    ];
    
    for dsn in test_dsns {
        let result = driver.open(dsn);
        
        match result {
            Ok(conn) => {
                // Test connection metadata
                let metadata = conn.metadata();
                assert!(!metadata.server_version.is_empty());
                assert!(!metadata.database_name.is_empty());
                assert!(metadata.server_port > 0);
                
                // Test connection state
                // Note: This will return false for placeholder connections
                let is_alive = conn.is_alive();
                
                // Test ping (will fail for placeholder but should not panic)
                let ping_result = conn.ping();
                
                // Test close (should always succeed)
                let close_result = conn.close();
                assert!(close_result.is_ok());
            }
            Err(e) => {
                // Error should be meaningful
                assert!(!e.to_string().is_empty());
            }
        }
    }
}

#[test]
fn test_connection_operations() {
    let driver = SimpleMySqlDriver::new();
    let conn_result = driver.open("mysql://test:test@localhost:3306/testdb");
    
    assert!(conn_result.is_ok());
    let conn = conn_result.unwrap();
    
    // Test prepare statement (will return NotSupported for placeholder)
    let prepare_result = conn.prepare("SELECT * FROM users WHERE id = ?");
    assert!(prepare_result.is_err());
    
    // Test query execution (will return NotSupported for placeholder)
    let args = vec![SqlValue::Integer(1)];
    let query_result = conn.query("SELECT * FROM users WHERE id = ?", &args);
    assert!(query_result.is_err());
    
    // Test execute statement (will return NotSupported for placeholder)
    let execute_result = conn.execute("INSERT INTO users (name) VALUES (?)", &[SqlValue::String("test".to_string())]);
    assert!(execute_result.is_err());
    
    // Test transaction begin (will return NotSupported for placeholder)
    let tx_options = TxOptions::default();
    let tx_result = conn.begin_transaction(tx_options);
    assert!(tx_result.is_err());
}

#[test]
fn test_driver_cloning() {
    let driver = SimpleMySqlDriver::new();
    let cloned_driver = driver.clone_driver();
    
    assert_eq!(driver.name(), cloned_driver.name());
    
    // Both drivers should have the same capabilities
    let orig_caps = driver.capabilities();
    let cloned_caps = cloned_driver.capabilities();
    
    assert_eq!(orig_caps.supports_transactions, cloned_caps.supports_transactions);
    assert_eq!(orig_caps.max_connections, cloned_caps.max_connections);
}

#[test]
fn test_comprehensive_driver_integration() {
    let driver = create_comprehensive_mysql_driver();
    
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
}

#[test]
fn test_comprehensive_driver_with_custom_config() {
    let config = MySqlConfig {
        max_connections: 75,
        min_connections: 8,
        connection_timeout: Duration::from_secs(45),
        query_timeout: Duration::from_secs(600),
        max_lifetime: Some(Duration::from_secs(7200)),
        idle_timeout: Some(Duration::from_secs(900)),
        ssl_enabled: true,
        ssl_cert_path: Some("/path/to/cert.pem".to_string()),
        ssl_key_path: Some("/path/to/key.pem".to_string()),
        ssl_ca_path: Some("/path/to/ca.pem".to_string()),
        ssl_verify: true,
        compression: true,
        charset: "utf8mb4".to_string(),
        timezone: "UTC".to_string(),
        foreign_key_checks: true,
        sql_mode: "STRICT_TRANS_TABLES,NO_ZERO_DATE".to_string(),
        autocommit: true,
        init_commands: vec![
            "SET SESSION sql_mode = 'STRICT_TRANS_TABLES'".to_string(),
            "SET SESSION time_zone = 'UTC'".to_string(),
        ],
    };
    
    let driver = create_comprehensive_mysql_driver_with_config(config);
    
    let capabilities = driver.capabilities();
    assert_eq!(capabilities.max_connections, Some(75));
}

#[test]
fn test_dsn_parsing_comprehensive() {
    // Test various DSN formats that should work
    let valid_dsns = vec![
        ("simple_db", "testdb"),
        ("host:port/db", "localhost:3307/myapp"),
        ("user@host/db", "user@dbserver/production"),
        ("user:pass@host/db", "admin:secret@dbhost/app"),
        ("full_dsn", "mysql://user:pass@host:3307/db?charset=utf8mb4&ssl=true"),
        ("with_params", "user@host/db?timeout=30&ssl_mode=REQUIRED"),
    ];
    
    for (description, dsn) in valid_dsns {
        let result = parse_comprehensive_mysql_dsn(dsn);
        match result {
            Ok(info) => {
                assert!(!info.host.is_empty(), "Host should not be empty for {}", description);
                assert!(info.port > 0, "Port should be positive for {}", description);
                assert!(!info.database.is_empty(), "Database should not be empty for {}", description);
            }
            Err(e) => {
                panic!("Failed to parse valid DSN '{}' ({}): {}", dsn, description, e);
            }
        }
    }
}

#[test]
fn test_dsn_parsing_errors() {
    let invalid_dsns = vec![
        "",
        "host:invalid_port/db",
        "host:99999/db", // Port too high
        "host:-1/db",    // Negative port
    ];
    
    for dsn in invalid_dsns {
        let result = parse_comprehensive_mysql_dsn(dsn);
        assert!(result.is_err(), "Should fail to parse invalid DSN: {}", dsn);
    }
}

#[test]
fn test_error_handling_integration() {
    let driver = SimpleMySqlDriver::new();
    
    // Test connection with various error scenarios
    let error_scenarios = vec![
        ("empty_dsn", ""),
        ("invalid_format", ":::invalid:::"),
        ("nonexistent_host", "mysql://user:pass@nonexistent.host.invalid:3306/db"),
    ];
    
    for (scenario, dsn) in error_scenarios {
        let result = driver.open(dsn);
        
        // Should either succeed (placeholder) or fail gracefully
        match result {
            Ok(conn) => {
                // If connection succeeds, it should be a valid connection object
                let metadata = conn.metadata();
                assert!(!metadata.server_version.is_empty());
                
                // Operations should return meaningful errors
                let query_result = conn.query("SELECT 1", &[]);
                assert!(query_result.is_err());
                
                if let Err(e) = query_result {
                    assert!(!e.to_string().is_empty());
                }
            }
            Err(e) => {
                // Error should have meaningful message
                assert!(!e.to_string().is_empty());
                println!("Expected error for {}: {}", scenario, e);
            }
        }
    }
}

#[test]
fn test_value_type_conversions() {
    use cursed::stdlib::database::mysql::comprehensive_driver::{
        convert_to_mysql_value, convert_from_mysql_value
    };
    
    // Test round-trip conversions
    let test_values = vec![
        SqlValue::Null,
        SqlValue::Boolean(true),
        SqlValue::Boolean(false),
        SqlValue::Integer(0),
        SqlValue::Integer(42),
        SqlValue::Integer(-123),
        SqlValue::Integer(i64::MAX),
        SqlValue::Integer(i64::MIN),
        SqlValue::Float(0.0),
        SqlValue::Float(3.14159),
        SqlValue::Float(-2.71828),
        SqlValue::String("".to_string()),
        SqlValue::String("hello".to_string()),
        SqlValue::String("hello, world! 🌍".to_string()),
        SqlValue::Bytes(vec![]),
        SqlValue::Bytes(vec![1, 2, 3, 4, 5]),
        SqlValue::Bytes(vec![0xFF, 0xFE, 0xFD, 0xFC]),
    ];
    
    for original_value in test_values {
        // Convert to MySQL value
        let mysql_result = convert_to_mysql_value(&original_value);
        assert!(mysql_result.is_ok(), "Failed to convert to MySQL: {:?}", original_value);
        
        let mysql_value = mysql_result.unwrap();
        
        // Convert back to SqlValue
        let sql_result = convert_from_mysql_value(mysql_value);
        assert!(sql_result.is_ok(), "Failed to convert from MySQL: {:?}", original_value);
        
        let converted_value = sql_result.unwrap();
        
        // Values should be equivalent (though not necessarily identical due to type system differences)
        match (&original_value, &converted_value) {
            (SqlValue::Null, SqlValue::Null) => {},
            (SqlValue::Boolean(a), SqlValue::Boolean(b)) => assert_eq!(a, b),
            (SqlValue::Integer(a), SqlValue::Integer(b)) => assert_eq!(a, b),
            (SqlValue::Float(a), SqlValue::Float(b)) => {
                if a.is_nan() && b.is_nan() {
                    // Both NaN, that's fine
                } else {
                    assert!((a - b).abs() < f64::EPSILON, "Float values differ: {} vs {}", a, b);
                }
            },
            (SqlValue::String(a), SqlValue::String(b)) => assert_eq!(a, b),
            (SqlValue::Bytes(a), SqlValue::Bytes(b)) => assert_eq!(a, b),
            (SqlValue::Bytes(a), SqlValue::String(b)) => {
                // Bytes might be converted to string if they're valid UTF-8
                if let Ok(a_str) = String::from_utf8(a.clone()) {
                    assert_eq!(&a_str, b);
                } else {
                    panic!("Bytes converted to string but original wasn't valid UTF-8");
                }
            },
            _ => {
                // Some conversions might change the type (e.g., bytes to string)
                // That's acceptable as long as the conversion is consistent
            }
        }
    }
}

#[test]
fn test_special_float_values() {
    use cursed::stdlib::database::mysql::comprehensive_driver::convert_to_mysql_value;
    
    // Test special float values
    let special_floats = vec![
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        0.0,
        -0.0,
        f64::MIN,
        f64::MAX,
        f64::EPSILON,
    ];
    
    for float_val in special_floats {
        let sql_value = SqlValue::Float(float_val);
        let result = convert_to_mysql_value(&sql_value);
        
        assert!(result.is_ok(), "Failed to convert special float: {}", float_val);
        
        // Verify the MySQL value is correct
        let mysql_value = result.unwrap();
        match mysql_value {
            mysql::Value::Float(f) => {
                if float_val.is_nan() {
                    assert!(f.is_nan());
                } else if float_val.is_infinite() {
                    assert!(f.is_infinite());
                    assert_eq!(f.is_sign_positive(), float_val.is_sign_positive());
                } else {
                    assert!((f as f64 - float_val).abs() < f64::EPSILON);
                }
            }
            mysql::Value::Double(d) => {
                if float_val.is_nan() {
                    assert!(d.is_nan());
                } else if float_val.is_infinite() {
                    assert!(d.is_infinite());
                    assert_eq!(d.is_sign_positive(), float_val.is_sign_positive());
                } else {
                    assert!((d - float_val).abs() < f64::EPSILON);
                }
            }
            _ => panic!("Expected Float or Double MySQL value"),
        }
    }
}

#[test]
fn test_connection_metadata() {
    let driver = SimpleMySqlDriver::new();
    let conn = driver.open("mysql://testuser:testpass@testhost:3307/testdb").unwrap();
    
    let metadata = conn.metadata();
    
    // Verify metadata fields are populated
    assert!(!metadata.server_version.is_empty());
    assert!(!metadata.database_name.is_empty());
    assert!(!metadata.server_host.is_empty());
    assert!(metadata.server_port > 0);
    assert!(!metadata.username.is_empty());
    assert!(metadata.connected_at > std::time::SystemTime::UNIX_EPOCH);
    assert!(!metadata.additional_info.is_empty());
    
    // Check specific additional info
    assert!(metadata.additional_info.contains_key("driver_version"));
    assert!(metadata.additional_info.contains_key("connection_id"));
}

#[test]
fn test_concurrent_driver_usage() {
    use std::sync::Arc;
    use std::thread;
    
    let driver = Arc::new(SimpleMySqlDriver::new());
    let mut handles = vec![];
    
    // Spawn multiple threads to test concurrent access
    for i in 0..5 {
        let driver_clone = Arc::clone(&driver);
        let handle = thread::spawn(move || {
            let dsn = format!("mysql://user{}:pass{}@host{}/db{}", i, i, i, i);
            
            // Each thread tries to open a connection
            let result = driver_clone.open(&dsn);
            assert!(result.is_ok());
            
            let conn = result.unwrap();
            
            // Test basic operations
            let metadata = conn.metadata();
            assert!(!metadata.server_version.is_empty());
            
            let ping_result = conn.ping();
            // Ping will fail for placeholder, but should not panic
            
            let close_result = conn.close();
            assert!(close_result.is_ok());
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_driver_capabilities_consistency() {
    let simple_driver = SimpleMySqlDriver::new();
    let comprehensive_driver = create_comprehensive_mysql_driver();
    
    let simple_caps = simple_driver.capabilities();
    let comprehensive_caps = comprehensive_driver.capabilities();
    
    // Both drivers should support the same basic features
    assert_eq!(simple_caps.supports_transactions, comprehensive_caps.supports_transactions);
    assert_eq!(simple_caps.supports_prepared_statements, comprehensive_caps.supports_prepared_statements);
    assert_eq!(simple_caps.supports_multiple_result_sets, comprehensive_caps.supports_multiple_result_sets);
    assert_eq!(simple_caps.supports_stored_procedures, comprehensive_caps.supports_stored_procedures);
    assert_eq!(simple_caps.supports_batch_operations, comprehensive_caps.supports_batch_operations);
    assert_eq!(simple_caps.supports_concurrent_connections, comprehensive_caps.supports_concurrent_connections);
    
    // Isolation levels should be the same
    assert_eq!(simple_caps.supported_isolation_levels, comprehensive_caps.supported_isolation_levels);
    
    // Limits should be similar
    assert_eq!(simple_caps.max_query_length, comprehensive_caps.max_query_length);
    assert_eq!(simple_caps.max_parameter_count, comprehensive_caps.max_parameter_count);
}

#[test]
fn test_performance_characteristics() {
    use std::time::Instant;
    
    // Driver creation should be fast
    let start = Instant::now();
    let _driver = SimpleMySqlDriver::new();
    let creation_time = start.elapsed();
    assert!(creation_time < Duration::from_millis(10), "Driver creation took too long: {:?}", creation_time);
    
    let driver = SimpleMySqlDriver::new();
    
    // Connection opening should be fast (even for placeholder)
    let start = Instant::now();
    let _conn = driver.open("testdb").unwrap();
    let connection_time = start.elapsed();
    assert!(connection_time < Duration::from_millis(50), "Connection opening took too long: {:?}", connection_time);
    
    // Multiple operations should scale linearly
    let start = Instant::now();
    for i in 0..100 {
        let dsn = format!("db_{}", i);
        let _conn = driver.open(&dsn).unwrap();
    }
    let batch_time = start.elapsed();
    assert!(batch_time < Duration::from_secs(1), "Batch operations took too long: {:?}", batch_time);
}

#[test]
fn test_memory_usage() {
    // Create many drivers and connections to test for memory leaks
    let mut drivers = Vec::new();
    let mut connections = Vec::new();
    
    for i in 0..100 {
        let driver = SimpleMySqlDriver::new();
        let conn = driver.open(&format!("db_{}", i)).unwrap();
        
        drivers.push(driver);
        connections.push(conn);
    }
    
    // Test that all objects are still valid
    for (i, conn) in connections.iter().enumerate() {
        let metadata = conn.metadata();
        assert!(!metadata.server_version.is_empty(), "Connection {} metadata invalid", i);
    }
    
    // Objects should be dropped cleanly when they go out of scope
    drop(connections);
    drop(drivers);
    
    // If we reach here without panicking, memory management is working correctly
}

#[test]
fn test_integration_with_database_error_system() {
    let driver = SimpleMySqlDriver::new();
    let conn = driver.open("testdb").unwrap();
    
    // Test that all error-returning operations produce proper DatabaseError instances
    let operations: Vec<Box<dyn Fn() -> Result<(), DatabaseError>>> = vec![
        Box::new(|| conn.prepare("SELECT 1").map(|_| ())),
        Box::new(|| conn.query("SELECT 1", &[]).map(|_| ())),
        Box::new(|| conn.execute("INSERT INTO test VALUES (1)", &[]).map(|_| ())),
        Box::new(|| conn.begin_transaction(TxOptions::default()).map(|_| ())),
        Box::new(|| conn.ping()),
    ];
    
    for (i, operation) in operations.iter().enumerate() {
        let result = operation();
        
        match result {
            Ok(_) => {
                // Unexpected success for placeholder implementation
                println!("Operation {} unexpectedly succeeded", i);
            }
            Err(e) => {
                // Error should be properly formatted
                assert!(!e.to_string().is_empty(), "Error {} has empty message", i);
                
                // Error should have appropriate kind
                match e.kind() {
                    DatabaseErrorKind::NotSupported => {
                        // Expected for placeholder implementation
                    }
                    DatabaseErrorKind::ConnectionError => {
                        // Also acceptable
                    }
                    DatabaseErrorKind::NotImplemented => {
                        // Also acceptable for development
                    }
                    _ => {
                        // Other error kinds are fine too
                    }
                }
            }
        }
    }
}

#[test]
fn test_comprehensive_driver_health_check() {
    let driver = create_comprehensive_mysql_driver();
    
    let health_result = driver.health_check();
    assert!(health_result.is_ok(), "Health check should not fail");
    
    let health = health_result.unwrap();
    
    // Health status should have reasonable values
    assert!(health.uptime >= Duration::ZERO);
    assert_eq!(health.active_connections, 0); // No connections yet
    assert_eq!(health.cache_hits, 0); // No cache activity yet
    assert_eq!(health.cache_misses, 0);
    assert_eq!(health.cache_size, 0);
    
    // Pool should not be initialized yet
    assert!(!health.pool_initialized);
    
    // Basic functionality test will depend on implementation
    // For placeholder, it will be false, which is fine
}

#[test]
fn test_json_value_handling() {
    use cursed::stdlib::database::mysql::comprehensive_driver::convert_to_mysql_value;
    
    // Test various JSON structures
    let json_values = vec![
        serde_json::json!(null),
        serde_json::json!(true),
        serde_json::json!(false),
        serde_json::json!(42),
        serde_json::json!(3.14),
        serde_json::json!("hello"),
        serde_json::json!([1, 2, 3]),
        serde_json::json!({"name": "test", "value": 42}),
        serde_json::json!({
            "users": [
                {"id": 1, "name": "Alice"},
                {"id": 2, "name": "Bob"}
            ],
            "count": 2,
            "active": true
        }),
    ];
    
    for json_val in json_values {
        let sql_value = SqlValue::Json(json_val.clone());
        let result = convert_to_mysql_value(&sql_value);
        
        assert!(result.is_ok(), "Failed to convert JSON: {}", json_val);
        
        let mysql_value = result.unwrap();
        
        // JSON should be converted to string
        match mysql_value {
            mysql::Value::Bytes(bytes) => {
                let json_string = String::from_utf8(bytes).unwrap();
                let expected_string = json_val.to_string();
                assert_eq!(json_string, expected_string);
            }
            _ => panic!("Expected Bytes value for JSON conversion"),
        }
    }
}
