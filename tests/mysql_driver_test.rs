/// fr fr Comprehensive MySQL driver tests
/// 
/// This module provides comprehensive testing for the MySQL driver implementation,
/// including unit tests, integration tests, and error handling validation.

use std::time::Duration;
use cursed::stdlib::database::{
    Driver, DriverConn, SqlValue, TxOptions, SqlIsolationLevel,
    DatabaseError, DatabaseErrorKind
};
use cursed::stdlib::database::mysql::{
    MySqlDriver, MySqlConfig, MySqlError, MySqlResult,
    create_mysql_driver, parse_mysql_dsn, validate_mysql_dsn
};

/// Test MySQL driver creation and basic properties
#[test]
fn test_mysql_driver_creation() {
    let driver = MySqlDriver::new();
    
    assert_eq!(driver.name(), "MySQL Driver for CURSED");
    
    let capabilities = driver.capabilities();
    assert!(capabilities.supports_transactions);
    assert!(capabilities.supports_prepared_statements);
    assert!(capabilities.supports_multiple_result_sets);
    assert!(capabilities.supports_stored_procedures);
    assert!(capabilities.supports_batch_operations);
    assert!(capabilities.supports_concurrent_connections);
    assert_eq!(capabilities.max_connections, Some(100));
    assert_eq!(capabilities.supported_isolation_levels.len(), 4);
    assert_eq!(capabilities.max_query_length, Some(16_777_216));
    assert_eq!(capabilities.max_parameter_count, Some(65535));
}

/// Test MySQL driver with custom configuration
#[test]
fn test_mysql_driver_with_config() {
    let mut config = MySqlConfig::default();
    config.max_connections = 50;
    config.connection_timeout = Duration::from_secs(10);
    config.charset = "utf8".to_string();
    config.ssl_enabled = true;
    
    let driver = MySqlDriver::with_config(config.clone());
    
    assert_eq!(driver.config().max_connections, 50);
    assert_eq!(driver.config().connection_timeout, Duration::from_secs(10));
    assert_eq!(driver.config().charset, "utf8");
    assert!(driver.config().ssl_enabled);
}

/// Test MySQL driver cloning
#[test]
fn test_mysql_driver_cloning() {
    let driver1 = MySqlDriver::new();
    let driver2 = driver1.clone();
    
    assert_eq!(driver1.name(), driver2.name());
    assert_eq!(driver1.config().max_connections, driver2.config().max_connections);
    
    // Test boxed driver cloning
    let boxed_driver: Box<dyn Driver> = Box::new(MySqlDriver::new());
    let cloned_driver = boxed_driver.clone_driver();
    
    assert_eq!(boxed_driver.name(), cloned_driver.name());
    assert_eq!(boxed_driver.capabilities().max_connections, cloned_driver.capabilities().max_connections);
}

/// Test MySQL connection string parsing
#[test]
fn test_mysql_dsn_parsing() {
    // Test full MySQL URL format
    let dsn1 = "mysql://user:password@localhost:3306/testdb";
    let info1 = parse_mysql_dsn(dsn1).unwrap();
    
    assert_eq!(info1.user, "user");
    assert_eq!(info1.password, "password");
    assert_eq!(info1.host, "localhost");
    assert_eq!(info1.port, 3306);
    assert_eq!(info1.database, "testdb");
    
    // Test simplified format
    let dsn2 = "user:password@localhost:3306/testdb";
    let info2 = parse_mysql_dsn(dsn2).unwrap();
    
    assert_eq!(info2.user, "user");
    assert_eq!(info2.password, "password");
    assert_eq!(info2.host, "localhost");
    assert_eq!(info2.port, 3306);
    assert_eq!(info2.database, "testdb");
    
    // Test with default port
    let dsn3 = "user:password@localhost/testdb";
    let info3 = parse_mysql_dsn(dsn3).unwrap();
    
    assert_eq!(info3.user, "user");
    assert_eq!(info3.password, "password");
    assert_eq!(info3.host, "localhost");
    assert_eq!(info3.port, 3306); // Default port
    assert_eq!(info3.database, "testdb");
}

/// Test MySQL connection string validation
#[test]
fn test_mysql_dsn_validation() {
    // Valid DSNs
    assert!(validate_mysql_dsn("mysql://user:pass@localhost:3306/db").is_ok());
    assert!(validate_mysql_dsn("user:pass@localhost:3306/db").is_ok());
    assert!(validate_mysql_dsn("user:@localhost/db").is_ok());
    
    // Invalid DSNs
    assert!(validate_mysql_dsn("").is_err());
    assert!(validate_mysql_dsn("invalid_format").is_err());
    assert!(validate_mysql_dsn("localhost:3306").is_err());
}

/// Test MySQL error types and conversions
#[test]
fn test_mysql_error_types() {
    // Test error creation
    let conn_err = MySqlError::connection_error("Connection failed");
    assert!(matches!(conn_err, MySqlError::Connection(_)));
    
    let query_err = MySqlError::query_error("Query failed", Some("SELECT * FROM table"));
    assert!(matches!(query_err, MySqlError::Query(_, _)));
    
    let tx_err = MySqlError::transaction_error("Transaction failed");
    assert!(matches!(tx_err, MySqlError::Transaction(_)));
    
    let constraint_err = MySqlError::constraint_violation("Constraint violated", Some("uk_email"));
    assert!(matches!(constraint_err, MySqlError::ConstraintViolation(_, _)));
    
    let type_err = MySqlError::type_conversion_error("String", "Integer");
    assert!(matches!(type_err, MySqlError::TypeConversion(_, _)));
    
    // Test error to DatabaseError conversion
    let db_err = conn_err.to_database_error();
    assert!(matches!(db_err.kind, DatabaseErrorKind::ConnectionError));
    
    let query_db_err = query_err.to_database_error();
    assert!(matches!(query_db_err.kind, DatabaseErrorKind::QueryError));
}

/// Test MySQL configuration defaults
#[test]
fn test_mysql_config_defaults() {
    let config = MySqlConfig::default();
    
    assert_eq!(config.max_connections, 100);
    assert_eq!(config.min_connections, 1);
    assert_eq!(config.connection_timeout, Duration::from_secs(30));
    assert_eq!(config.query_timeout, Duration::from_secs(300));
    assert_eq!(config.max_lifetime, Some(Duration::from_secs(3600)));
    assert_eq!(config.idle_timeout, Some(Duration::from_secs(600)));
    assert!(!config.ssl_enabled);
    assert_eq!(config.ssl_cert_path, None);
    assert_eq!(config.ssl_key_path, None);
    assert_eq!(config.ssl_ca_path, None);
    assert!(config.ssl_verify);
    assert!(!config.compression);
    assert_eq!(config.charset, "utf8mb4");
    assert_eq!(config.timezone, None);
    assert!(config.additional_params.is_empty());
}

/// Test MySQL driver capabilities
#[test]
fn test_mysql_driver_capabilities() {
    let driver = MySqlDriver::new();
    let caps = driver.capabilities();
    
    // Test all required capabilities
    assert!(caps.supports_transactions);
    assert!(caps.supports_prepared_statements);
    assert!(caps.supports_multiple_result_sets);
    assert!(caps.supports_stored_procedures);
    assert!(caps.supports_batch_operations);
    assert!(caps.supports_concurrent_connections);
    
    // Test limits
    assert_eq!(caps.max_connections, Some(100));
    assert_eq!(caps.max_query_length, Some(16_777_216));
    assert_eq!(caps.max_parameter_count, Some(65535));
    
    // Test supported isolation levels
    let isolation_levels = &caps.supported_isolation_levels;
    assert!(isolation_levels.contains(&SqlIsolationLevel::LevelReadUncommitted));
    assert!(isolation_levels.contains(&SqlIsolationLevel::LevelReadCommitted));
    assert!(isolation_levels.contains(&SqlIsolationLevel::LevelRepeatableRead));
    assert!(isolation_levels.contains(&SqlIsolationLevel::LevelSerializable));
}

/// Test MySQL driver factory functions
#[test]
fn test_mysql_driver_factory_functions() {
    // Test create_mysql_driver
    let driver1 = create_mysql_driver();
    assert_eq!(driver1.name(), "MySQL Driver for CURSED");
    
    // Test create_mysql_driver_with_config
    let mut config = MySqlConfig::default();
    config.max_connections = 200;
    
    let driver2 = cursed::stdlib::database::mysql::create_mysql_driver_with_config(config);
    assert_eq!(driver2.config().max_connections, 200);
}

/// Test transaction options
#[test]
fn test_transaction_options() {
    let default_opts = TxOptions::default();
    assert_eq!(default_opts.isolation, SqlIsolationLevel::LevelDefault);
    assert!(!default_opts.read_only);
    
    let custom_opts = TxOptions {
        isolation: SqlIsolationLevel::LevelSerializable,
        read_only: true,
    };
    assert_eq!(custom_opts.isolation, SqlIsolationLevel::LevelSerializable);
    assert!(custom_opts.read_only);
}

/// Test MySQL error display formatting
#[test]
fn test_mysql_error_display() {
    let conn_err = MySqlError::connection_error("Connection timeout");
    assert!(conn_err.to_string().contains("MySQL Connection Error"));
    assert!(conn_err.to_string().contains("Connection timeout"));
    
    let query_err = MySqlError::query_error("Syntax error", Some("SELECT invalid"));
    assert!(query_err.to_string().contains("MySQL Query Error"));
    assert!(query_err.to_string().contains("Syntax error"));
    assert!(query_err.to_string().contains("SELECT invalid"));
    
    let server_err = MySqlError::server_error(1045, "Access denied");
    assert!(server_err.to_string().contains("MySQL Server Error 1045"));
    assert!(server_err.to_string().contains("Access denied"));
}

/// Test MySQL error severity mapping
#[test]
fn test_mysql_error_severity() {
    let conn_err = MySqlError::connection_error("Connection failed");
    let db_err = conn_err.to_database_error();
    // Connection errors should be warnings (retryable)
    assert!(db_err.is_retryable());
    
    let constraint_err = MySqlError::constraint_violation("Duplicate key", Some("primary"));
    let constraint_db_err = constraint_err.to_database_error();
    assert!(constraint_db_err.is_constraint_violation());
    
    let query_err = MySqlError::query_error("Table not found", None);
    let query_db_err = query_err.to_database_error();
    assert!(!query_db_err.is_retryable()); // Query errors are not retryable
}

/// Mock test for driver open functionality
/// Note: This test cannot actually open a connection without a real MySQL server
#[test]
fn test_mysql_driver_open_validation() {
    let driver = MySqlDriver::new();
    
    // Test with invalid DSN
    let result = driver.open("");
    assert!(result.is_err());
    
    // Test with malformed DSN
    let result2 = driver.open("invalid_dsn");
    assert!(result2.is_err());
    
    // We can't test successful connection without a real MySQL server
    // In a real integration test environment, you would test:
    // let result3 = driver.open("mysql://test:test@localhost:3306/testdb");
    // assert!(result3.is_ok());
}

/// Test type conversion functions
#[test]
fn test_type_conversions() {
    use cursed::stdlib::database::mysql::types::{escape_string, build_placeholders};
    
    // Test string escaping
    let input = "test'string\"with\nnewlines";
    let escaped = escape_string(input);
    assert!(escaped.contains("\\'"));
    assert!(escaped.contains("\\\""));
    assert!(escaped.contains("\\n"));
    
    // Test placeholder generation
    assert_eq!(build_placeholders(0), "");
    assert_eq!(build_placeholders(1), "?");
    assert_eq!(build_placeholders(3), "?, ?, ?");
}

/// Test MySQL pool configuration
#[test]
fn test_mysql_pool_config() {
    use cursed::stdlib::database::mysql::pool::MySqlPoolConfig;
    
    let config = MySqlPoolConfig::default();
    
    assert_eq!(config.min_connections, 1);
    assert_eq!(config.max_connections, 100);
    assert_eq!(config.connection_timeout, Duration::from_secs(30));
    assert_eq!(config.test_query, "SELECT 1");
    assert_eq!(config.health_check_interval, Duration::from_secs(60));
    assert_eq!(config.max_retries, 3);
    assert_eq!(config.retry_delay, Duration::from_millis(1000));
}

/// Test MySQL initialization function
#[test]
fn test_mysql_initialization() {
    // Test that MySQL driver can be initialized
    // Note: This would normally register the driver globally
    let driver = cursed::stdlib::database::mysql::new_mysql_driver();
    assert_eq!(driver.name(), "MySQL Driver for CURSED");
    
    let mut config = MySqlConfig::default();
    config.max_connections = 75;
    
    let driver_with_config = cursed::stdlib::database::mysql::new_mysql_driver_with_config(config);
    assert_eq!(driver_with_config.config().max_connections, 75);
}

/// Integration test for comprehensive driver functionality
/// Note: This test requires a running MySQL instance to pass fully
#[test]
#[ignore] // Ignored by default since it requires MySQL server
fn test_mysql_driver_integration() {
    let driver = MySqlDriver::new();
    let dsn = "mysql://test:test@localhost:3306/test";
    
    // Test connection opening
    match driver.open(dsn) {
        Ok(conn) => {
            // Test basic operations
            assert!(conn.is_alive());
            
            let metadata = conn.metadata();
            assert!(!metadata.database_name.is_empty());
            assert!(!metadata.server_version.is_empty());
            
            // Test ping
            assert!(conn.ping().is_ok());
            
            // Test close
            assert!(conn.close().is_ok());
        }
        Err(e) => {
            // Expected if no MySQL server is running
            println!("MySQL integration test skipped: {}", e);
        }
    }
}
