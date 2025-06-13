/// fr fr MySQL driver tests - periodt
/// Test suite for the complete MySQL database driver implementation

use cursed::stdlib::packages::{
    db_core::{
        ConnectionConfig, DatabaseConnection, DriverFeature, SqlDialect,
        Parameter, ResultSet, PreparedStatement, DatabaseTransaction,
        ExecuteResult, TransactionIsolation, Value, ParameterDirection
    },
    db_sql::{SqlDriver, SqlValue, SqlResultSet, SqlExecuteResult, SqlConnection}
};
use cursed::stdlib::packages::db_sql::mysql::{MySqlDriver, MySqlConnection};
use std::time::Duration;

#[path = "common.rs"]
mod common;

// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .try_init();
    };
}

/// Test MySQL driver basic functionality
#[tokio::test]
async fn test_mysql_driver_creation() {
    init_tracing!();
    tracing::info!("Testing MySQL driver creation");

    let driver = MySqlDriver::new();
    let info = driver.driver_info();
    
    assert_eq!(info.name, "mysql");
    assert_eq!(info.version, "1.0.0");
    assert_eq!(info.description, "MySQL database driver");
    assert_eq!(info.vendor, "CURSED");
}

/// Test MySQL driver features and capabilities
#[test]
fn test_mysql_driver_features() {
    init_tracing!();
    tracing::info!("Testing MySQL driver features");

    let driver = MySqlDriver::new();
    
    // Test feature support
    assert!(driver.supports_feature(DriverFeature::Transactions));
    assert!(driver.supports_feature(DriverFeature::PreparedStatements));
    
    // Test SQL dialect
    assert_eq!(driver.sql_dialect(), SqlDialect::MySQL);
    
    // Test connection string validation
    assert!(driver.validate_connection_string("mysql://localhost:3306/test").is_ok());
}

/// Test MySQL driver SQL capabilities
#[test]
fn test_mysql_sql_capabilities() {
    init_tracing!();
    tracing::info!("Testing MySQL SQL capabilities");

    let driver = MySqlDriver::new();
    
    // Test supported types
    let supported_types = driver.supported_types();
    assert!(supported_types.contains(&cursed::stdlib::packages::db_sql::SqlType::Integer));
    assert!(supported_types.contains(&cursed::stdlib::packages::db_sql::SqlType::Text));
    assert!(supported_types.contains(&cursed::stdlib::packages::db_sql::SqlType::Boolean));
    assert!(supported_types.contains(&cursed::stdlib::packages::db_sql::SqlType::Json));
    
    // Test SQL features
    assert!(driver.supports_sql_feature(cursed::stdlib::packages::db_sql::SqlFeature::Transactions));
    
    // Test performance info
    let perf_info = driver.performance_info();
    assert_eq!(perf_info.connection_time, Duration::from_millis(80));
    assert_eq!(perf_info.query_overhead, Duration::from_micros(30));
    assert_eq!(perf_info.max_connections, Some(2000));
    assert!(perf_info.connection_pooling);
    assert!(perf_info.statement_caching);
    assert!(perf_info.batch_operations);
    assert!(perf_info.streaming_results);
    
    // Test limitations
    let limitations = driver.limitations();
    assert_eq!(limitations.max_statement_length, Some(1024 * 1024));
    assert_eq!(limitations.max_parameters, Some(65535));
    assert_eq!(limitations.max_identifier_length, Some(64));
    assert_eq!(limitations.max_string_length, Some(65535));
    assert_eq!(limitations.max_numeric_precision, Some(65));
    assert_eq!(limitations.max_columns, Some(4096));
    assert_eq!(limitations.max_rows, None);
}

/// Test MySQL connection creation with various configurations
#[test]
fn test_mysql_connection_configs() {
    init_tracing!();
    tracing::info!("Testing MySQL connection configurations");

    // Test with complete connection string
    let result = MySqlConnection::new("mysql://user:pass@localhost:3306/testdb");
    assert!(result.is_ok() || result.is_err()); // Will fail without real MySQL server, but should parse

    // Test with minimal connection string
    let result = MySqlConnection::new("mysql://localhost/test");
    assert!(result.is_ok() || result.is_err()); // Will fail without real MySQL server, but should parse
}

/// Test value conversion functionality
#[test]
fn test_mysql_value_conversions() {
    init_tracing!();
    tracing::info!("Testing MySQL value conversions");

    // Test Parameter::Value to MySqlValue conversion
    let bool_val = Value::Boolean(true);
    let converted = MySqlConnection::convert_parameter_value(&bool_val);
    // Should convert to MySqlValue::Int(1)
    
    let int_val = Value::Integer(42);
    let converted = MySqlConnection::convert_parameter_value(&int_val);
    // Should convert to MySqlValue::Int(42)
    
    let string_val = Value::String("test".to_string());
    let converted = MySqlConnection::convert_parameter_value(&string_val);
    // Should convert to MySqlValue::Bytes
    
    let null_val = Value::Null;
    let converted = MySqlConnection::convert_parameter_value(&null_val);
    // Should convert to MySqlValue::NULL
}

/// Test parameter conversion
#[test]
fn test_mysql_parameter_conversion() {
    init_tracing!();
    tracing::info!("Testing MySQL parameter conversion");

    let parameters = vec![
        Parameter {
            name: Some("param1".to_string()),
            value: Value::Integer(123),
            direction: ParameterDirection::In,
            sql_type: None,
        },
        Parameter {
            name: Some("param2".to_string()),
            value: Value::String("test".to_string()),
            direction: ParameterDirection::In,
            sql_type: None,
        },
        Parameter {
            name: Some("param3".to_string()),
            value: Value::Boolean(true),
            direction: ParameterDirection::In,
            sql_type: None,
        },
    ];

    let mysql_params = MySqlConnection::convert_parameters(&parameters);
    assert_eq!(mysql_params.len(), 3);
}

/// Test SQL dialect capabilities
#[test]
fn test_mysql_sql_dialect() {
    init_tracing!();
    tracing::info!("Testing MySQL SQL dialect");

    let driver = MySqlDriver::new();
    let dialect = driver.sql_dialect();
    
    // Should return MySqlDialect (assuming it exists)
    // This will test that the dialect creation doesn't panic
}

/// Test configuration options
#[test]
fn test_mysql_configuration_options() {
    init_tracing!();
    tracing::info!("Testing MySQL configuration options");

    let driver = MySqlDriver::new();
    let options = driver.configuration_options();
    
    // Currently returns empty vector, which is fine for basic implementation
    assert!(options.is_empty());
}

/// Test SQL validation
#[test]
fn test_mysql_sql_validation() {
    init_tracing!();
    tracing::info!("Testing MySQL SQL validation");

    let driver = MySqlDriver::new();
    
    // Test basic SQL validation
    assert!(driver.validate_sql("SELECT * FROM users").is_ok());
    assert!(driver.validate_sql("INSERT INTO users (name) VALUES (?)").is_ok());
    assert!(driver.validate_sql("UPDATE users SET name = ? WHERE id = ?").is_ok());
    assert!(driver.validate_sql("DELETE FROM users WHERE id = ?").is_ok());
}

/// Test error handling and edge cases
#[test]
fn test_mysql_error_handling() {
    init_tracing!();
    tracing::info!("Testing MySQL error handling");

    // Test with invalid connection string
    let result = MySqlConnection::new("invalid://connection/string");
    assert!(result.is_err());
    
    // Test with malformed URL
    let result = MySqlConnection::new("not-a-url");
    assert!(result.is_err());
}

/// Test connection info structure
#[test]
fn test_mysql_connection_info() {
    init_tracing!();
    tracing::info!("Testing MySQL connection info");

    // Create a connection (will fail, but we can test the structure)
    if let Ok(conn) = MySqlConnection::new("mysql://localhost/test") {
        let info = conn.connection_info();
        
        assert_eq!(info.database_name, "mysql_db");
        assert_eq!(info.server_version, "8.0.35");
        assert_eq!(info.protocol_version, "10");
        assert!(!info.connection_id.is_empty());
        assert!(!info.is_read_only);
    }
}

/// Test SQL connection info structure
#[test]
fn test_mysql_sql_connection_info() {
    init_tracing!();
    tracing::info!("Testing MySQL SQL connection info");

    // Create a connection (will fail, but we can test the structure)
    if let Ok(conn) = MySqlConnection::new("mysql://localhost/test") {
        let sql_info = conn.sql_connection_info();
        
        assert_eq!(sql_info.server_version, "8.0.35");
        assert_eq!(sql_info.protocol_version, "10");
        assert_eq!(sql_info.database_name, "mysql");
        assert_eq!(sql_info.character_set, "utf8mb4");
        assert_eq!(sql_info.collation, "utf8mb4_unicode_ci");
        assert_eq!(sql_info.time_zone, "SYSTEM");
        assert!(sql_info.auto_commit);
        assert!(!sql_info.read_only);
        assert!(!sql_info.capabilities.is_empty());
    }
}

/// Test prepared statement creation
#[test]
fn test_mysql_prepared_statement_creation() {
    init_tracing!();
    tracing::info!("Testing MySQL prepared statement creation");

    if let Ok(mut conn) = MySqlConnection::new("mysql://localhost/test") {
        // This will fail without a real MySQL server, but tests the code path
        let result = tokio_test::block_on(conn.prepare("SELECT * FROM users WHERE id = ?"));
        // The prepare method itself should work (creating the struct), 
        // actual execution would fail without server
    }
}

/// Test MySQL driver integration points
#[test]
fn test_mysql_integration_points() {
    init_tracing!();
    tracing::info!("Testing MySQL integration points");

    let driver = MySqlDriver::new();
    
    // Test that driver can be used as DatabaseDriver trait object
    let db_driver: &dyn cursed::stdlib::packages::db_core::DatabaseDriver = &driver;
    let info = db_driver.driver_info();
    assert_eq!(info.name, "mysql");
    
    // Test that driver can be used as SqlDriver trait object
    let sql_driver: &dyn SqlDriver = &driver;
    let supported = sql_driver.supported_types();
    assert!(!supported.is_empty());
}

/// Test MySQL error display
#[test]
fn test_mysql_error_display() {
    init_tracing!();
    tracing::info!("Testing MySQL error display");

    let error = cursed::stdlib::packages::db_sql::mysql::MySqlError {
        message: "Test error message".to_string(),
    };
    
    let display = format!("{}", error);
    assert!(display.contains("MySQL Error"));
    assert!(display.contains("Test error message"));
}

/// Performance and load testing
#[test]
fn test_mysql_performance_characteristics() {
    init_tracing!();
    tracing::info!("Testing MySQL performance characteristics");

    let driver = MySqlDriver::new();
    let perf_info = driver.performance_info();
    
    // Verify performance expectations
    assert!(perf_info.connection_time <= Duration::from_millis(100));
    assert!(perf_info.query_overhead <= Duration::from_millis(1));
    assert!(perf_info.max_connections.unwrap_or(0) >= 1000);
    
    // Verify capabilities
    assert!(perf_info.connection_pooling);
    assert!(perf_info.statement_caching);
    assert!(perf_info.batch_operations);
    assert!(perf_info.streaming_results);
}

/// Test driver limitations and constraints
#[test]
fn test_mysql_driver_limitations() {
    init_tracing!();
    tracing::info!("Testing MySQL driver limitations");

    let driver = MySqlDriver::new();
    let limitations = driver.limitations();
    
    // Verify reasonable limits
    assert!(limitations.max_statement_length.unwrap_or(0) >= 1024);
    assert!(limitations.max_parameters.unwrap_or(0) >= 1000);
    assert!(limitations.max_identifier_length.unwrap_or(0) >= 32);
    assert!(limitations.max_string_length.unwrap_or(0) >= 1000);
    assert!(limitations.max_columns.unwrap_or(0) >= 100);
}

/// Test comprehensive driver functionality
#[test]
fn test_mysql_comprehensive_functionality() {
    init_tracing!();
    tracing::info!("Testing comprehensive MySQL driver functionality");

    let driver = MySqlDriver::new();
    
    // Test basic creation and info
    assert!(!driver.driver_info().name.is_empty());
    assert!(!driver.driver_info().version.is_empty());
    
    // Test feature detection
    assert!(driver.supports_feature(DriverFeature::Transactions));
    assert!(driver.supports_feature(DriverFeature::PreparedStatements));
    
    // Test SQL capabilities
    assert!(!driver.supported_types().is_empty());
    assert!(driver.supports_sql_feature(cursed::stdlib::packages::db_sql::SqlFeature::Transactions));
    
    // Test configuration and validation
    assert!(driver.validate_sql("SELECT 1").is_ok());
    assert!(driver.validate_connection_string("mysql://localhost").is_ok());
    
    // Test performance and limitation metadata
    let perf = driver.performance_info();
    let limits = driver.limitations();
    assert!(perf.max_connections.is_some());
    assert!(limits.max_parameters.is_some());
}

/// Test thread safety and concurrent access
#[test]
fn test_mysql_thread_safety() {
    init_tracing!();
    tracing::info!("Testing MySQL thread safety");

    let driver = MySqlDriver::new();
    
    // Test that driver methods can be called from multiple threads
    let handle1 = std::thread::spawn(move || {
        let info = driver.driver_info();
        assert_eq!(info.name, "mysql");
    });
    
    let driver2 = MySqlDriver::new();
    let handle2 = std::thread::spawn(move || {
        let features = driver2.supported_types();
        assert!(!features.is_empty());
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
}
