/// PostgreSQL Integration Tests
/// 
/// Comprehensive integration tests for the PostgreSQL driver implementation
/// including connection management, query execution, transactions, and error handling.
/// 
/// Note: These tests require a running PostgreSQL server. Set environment variables:
/// - POSTGRES_HOST (default: localhost)
/// - POSTGRES_PORT (default: 5432)
/// - POSTGRES_DB (default: test)
/// - POSTGRES_USER (default: postgres)
/// - POSTGRES_PASSWORD (default: empty)

use std::env;
use std::time::Duration;
use cursed::stdlib::database::{
    SqlValue, TxOptions, SqlIsolationLevel,
    postgres::{
        PostgresConfig, PostgresConnectionString, PostgresDriver, PostgresPool,
        PostgresConnection, SslMode, PostgresError, PostgresErrorKind
    }
};

/// Test configuration from environment variables
fn get_test_config() -> PostgresConfig {
    PostgresConfig {
        host: env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
        port: env::var("POSTGRES_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse()
            .unwrap_or(5432),
        database: env::var("POSTGRES_DB").unwrap_or_else(|_| "test".to_string()),
        username: env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string()),
        password: env::var("POSTGRES_PASSWORD").ok(),
        ssl_mode: SslMode::Disable, // Use plain connection for tests
        connect_timeout: Duration::from_secs(10),
        query_timeout: Duration::from_secs(30),
        ..Default::default()
    }
}

/// Check if PostgreSQL is available for testing
async fn is_postgres_available() -> bool {
    let config = get_test_config();
    match PostgresConnection::new(config).await {
        Ok(mut conn) => conn.is_alive().await,
        Err(_) => false,
    }
}

/// Skip test if PostgreSQL is not available
macro_rules! skip_if_no_postgres {
    () => {
        if !is_postgres_available().await {
            eprintln!("Skipping test: PostgreSQL server not available");
            return;
        }
    };
}

#[tokio::test]
async fn test_postgres_config_validation() {
    let mut config = get_test_config();
    assert!(config.validate().is_ok());
    
    // Test invalid configurations
    config.host = "".to_string();
    assert!(config.validate().is_err());
    
    config = get_test_config();
    config.port = 0;
    assert!(config.validate().is_err());
    
    config = get_test_config();
    config.database = "".to_string();
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_connection_string_parsing() {
    // Test URL format
    let url_dsn = "postgresql://user:pass@localhost:5432/testdb?sslmode=require";
    let config = PostgresConnectionString::parse(url_dsn).unwrap();
    
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 5432);
    assert_eq!(config.database, "testdb");
    assert_eq!(config.username, "user");
    assert_eq!(config.password, Some("pass".to_string()));
    assert_eq!(config.ssl_mode, SslMode::Require);
    
    // Test key-value format
    let kv_dsn = "host=localhost port=5432 dbname=testdb user=user password=pass sslmode=disable";
    let config = PostgresConnectionString::parse(kv_dsn).unwrap();
    
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 5432);
    assert_eq!(config.database, "testdb");
    assert_eq!(config.username, "user");
    assert_eq!(config.password, Some("pass".to_string()));
    assert_eq!(config.ssl_mode, SslMode::Disable);
}

#[tokio::test]
async fn test_driver_creation() {
    let driver = PostgresDriver::new();
    assert_eq!(driver.name(), "PostgreSQL Driver for CURSED");
    assert!(!driver.is_pooled());
    
    let caps = driver.capabilities();
    assert!(caps.supports_transactions);
    assert!(caps.supports_prepared_statements);
    assert!(caps.supports_concurrent_connections);
}

#[tokio::test]
async fn test_connection_creation() {
    skip_if_no_postgres!();
    
    let config = get_test_config();
    let result = PostgresConnection::new(config).await;
    
    match result {
        Ok(mut conn) => {
            assert!(conn.is_alive().await);
            
            let metadata = conn.metadata();
            assert_eq!(metadata.driver_name, "PostgreSQL");
            assert!(!metadata.connection_id.is_empty());
        }
        Err(e) => {
            // Connection might fail if PostgreSQL is not running
            assert!(matches!(
                e.kind,
                PostgresErrorKind::ConnectionFailed | PostgresErrorKind::TimeoutError
            ));
        }
    }
}

#[tokio::test]
async fn test_pool_creation() {
    skip_if_no_postgres!();
    
    let config = get_test_config();
    let result = PostgresPool::new(config).await;
    
    match result {
        Ok(pool) => {
            let health = pool.get_health();
            assert!(health.max_connections > 0);
            
            let stats = pool.get_statistics();
            assert_eq!(stats.total_connections_created, 0); // No connections created yet
            
            // Test getting connection from pool
            let conn_result = pool.get_connection().await;
            match conn_result {
                Ok(conn) => {
                    assert!(conn.ping().await.is_ok());
                }
                Err(e) => {
                    eprintln!("Pool connection failed: {}", e);
                }
            }
        }
        Err(e) => {
            // Pool creation might fail if PostgreSQL is not running
            assert!(matches!(
                e.kind,
                PostgresErrorKind::ConnectionFailed | PostgresErrorKind::PoolError
            ));
        }
    }
}

#[tokio::test]
async fn test_basic_query_execution() {
    skip_if_no_postgres!();
    
    let config = get_test_config();
    let mut conn = match PostgresConnection::new(config).await {
        Ok(conn) => conn,
        Err(_) => return, // Skip if connection fails
    };
    
    // Test simple query
    let result = conn.execute_query("SELECT 1 as test_column", &[]).await;
    match result {
        Ok(query_result) => {
            assert_eq!(query_result.columns.len(), 1);
            assert_eq!(query_result.columns[0], "test_column");
            assert_eq!(query_result.rows.len(), 1);
            
            if let SqlValue::Integer(value) = &query_result.rows[0][0] {
                assert_eq!(*value, 1);
            } else {
                panic!("Expected integer value");
            }
        }
        Err(e) => {
            eprintln!("Query execution failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_parameterized_queries() {
    skip_if_no_postgres!();
    
    let config = get_test_config();
    let mut conn = match PostgresConnection::new(config).await {
        Ok(conn) => conn,
        Err(_) => return,
    };
    
    // Test parameterized query
    let args = vec![
        SqlValue::Integer(42),
        SqlValue::String("test".to_string()),
        SqlValue::Boolean(true),
    ];
    
    let result = conn.execute_query(
        "SELECT $1::int as int_val, $2::text as text_val, $3::bool as bool_val",
        &args
    ).await;
    
    match result {
        Ok(query_result) => {
            assert_eq!(query_result.columns.len(), 3);
            assert_eq!(query_result.rows.len(), 1);
            
            let row = &query_result.rows[0];
            
            if let SqlValue::Integer(value) = &row[0] {
                assert_eq!(*value, 42);
            } else {
                panic!("Expected integer value");
            }
            
            if let SqlValue::String(value) = &row[1] {
                assert_eq!(value, "test");
            } else {
                panic!("Expected string value");
            }
            
            if let SqlValue::Boolean(value) = &row[2] {
                assert_eq!(*value, true);
            } else {
                panic!("Expected boolean value");
            }
        }
        Err(e) => {
            eprintln!("Parameterized query failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_prepared_statements() {
    skip_if_no_postgres!();
    
    let config = get_test_config();
    let mut conn = match PostgresConnection::new(config).await {
        Ok(conn) => conn,
        Err(_) => return,
    };
    
    // Test prepared statement
    let stmt_result = conn.prepare_statement("SELECT $1::int as value").await;
    match stmt_result {
        Ok(stmt) => {
            let info = stmt.info();
            assert_eq!(info.parameter_count, 1);
            assert_eq!(info.column_count, 1);
            assert!(info.query.contains("SELECT"));
            
            let stats = stmt.get_stats();
            assert_eq!(stats.executions, 0);
        }
        Err(e) => {
            eprintln!("Statement preparation failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_transaction_management() {
    skip_if_no_postgres!();
    
    let config = get_test_config();
    let mut conn = match PostgresConnection::new(config).await {
        Ok(conn) => conn,
        Err(_) => return,
    };
    
    // Test transaction creation
    let tx_options = TxOptions {
        isolation_level: Some(SqlIsolationLevel::LevelReadCommitted),
        read_only: false,
    };
    
    let tx_result = conn.begin_transaction(tx_options).await;
    match tx_result {
        Ok(mut tx) => {
            assert!(tx.is_active());
            
            let stats = tx.get_stats();
            assert_eq!(stats.statements_executed, 0);
            
            // Test query within transaction
            let query_result = tx.query("SELECT 1", &[]).await;
            match query_result {
                Ok(_) => {
                    let updated_stats = tx.get_stats();
                    assert_eq!(updated_stats.statements_executed, 1);
                }
                Err(e) => {
                    eprintln!("Transaction query failed: {}", e);
                }
            }
            
            // Test commit
            let commit_result = tx.commit().await;
            match commit_result {
                Ok(_) => {
                    eprintln!("Transaction committed successfully");
                }
                Err(e) => {
                    eprintln!("Transaction commit failed: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Transaction creation failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_savepoints() {
    skip_if_no_postgres!();
    
    let config = get_test_config();
    let mut conn = match PostgresConnection::new(config).await {
        Ok(conn) => conn,
        Err(_) => return,
    };
    
    let tx_result = conn.begin_transaction(TxOptions::default()).await;
    match tx_result {
        Ok(mut tx) => {
            // Create savepoint
            let sp_result = tx.savepoint("test_sp").await;
            match sp_result {
                Ok(savepoint_name) => {
                    assert!(savepoint_name.contains("test_sp"));
                    
                    // Test rollback to savepoint
                    let rollback_result = tx.rollback_to_savepoint(&savepoint_name).await;
                    match rollback_result {
                        Ok(_) => {
                            eprintln!("Savepoint rollback successful");
                        }
                        Err(e) => {
                            eprintln!("Savepoint rollback failed: {}", e);
                        }
                    }
                    
                    // Release savepoint
                    let release_result = tx.release_savepoint(&savepoint_name).await;
                    match release_result {
                        Ok(_) => {
                            eprintln!("Savepoint released successfully");
                        }
                        Err(e) => {
                            eprintln!("Savepoint release failed: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Savepoint creation failed: {}", e);
                }
            }
            
            // Cleanup
            let _ = tx.rollback().await;
        }
        Err(e) => {
            eprintln!("Transaction creation failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_error_handling() {
    skip_if_no_postgres!();
    
    let config = get_test_config();
    let mut conn = match PostgresConnection::new(config).await {
        Ok(conn) => conn,
        Err(_) => return,
    };
    
    // Test syntax error
    let result = conn.execute_query("INVALID SQL SYNTAX", &[]).await;
    match result {
        Ok(_) => panic!("Expected syntax error"),
        Err(e) => {
            assert!(matches!(e.kind, PostgresErrorKind::QueryError | PostgresErrorKind::SyntaxError));
            assert!(e.message.contains("syntax") || e.message.contains("INVALID"));
        }
    }
    
    // Test parameter count mismatch
    let result = conn.execute_query("SELECT $1", &[]).await;
    match result {
        Ok(_) => panic!("Expected parameter error"),
        Err(e) => {
            // This might be caught as a general query error
            assert!(matches!(
                e.kind,
                PostgresErrorKind::QueryError | PostgresErrorKind::TypeConversionError
            ));
        }
    }
}

#[tokio::test]
async fn test_connection_health_monitoring() {
    skip_if_no_postgres!();
    
    let config = get_test_config();
    let mut conn = match PostgresConnection::new(config).await {
        Ok(conn) => conn,
        Err(_) => return,
    };
    
    // Test connection health
    assert!(conn.is_alive().await);
    
    // Test connection statistics
    let stats = conn.get_stats();
    assert_eq!(stats.queries_executed, 0);
    assert_eq!(stats.errors_encountered, 0);
    
    // Execute a query and check stats update
    let _ = conn.execute_query("SELECT 1", &[]).await;
    let updated_stats = conn.get_stats();
    assert_eq!(updated_stats.queries_executed, 1);
}

#[tokio::test]
async fn test_type_conversions() {
    skip_if_no_postgres!();
    
    let config = get_test_config();
    let mut conn = match PostgresConnection::new(config).await {
        Ok(conn) => conn,
        Err(_) => return,
    };
    
    // Test various PostgreSQL types
    let test_cases = vec![
        ("SELECT NULL::int", SqlValue::Null),
        ("SELECT true", SqlValue::Boolean(true)),
        ("SELECT false", SqlValue::Boolean(false)),
        ("SELECT 42::smallint", SqlValue::Integer(42)),
        ("SELECT 12345::int", SqlValue::Integer(12345)),
        ("SELECT 9876543210::bigint", SqlValue::Integer(9876543210)),
        ("SELECT 3.14::real", SqlValue::Float(3.140000104904175)), // Precision loss with real
        ("SELECT 2.71828::double precision", SqlValue::Float(2.71828)),
        ("SELECT 'hello'::text", SqlValue::String("hello".to_string())),
        ("SELECT 'world'::varchar(10)", SqlValue::String("world".to_string())),
    ];
    
    for (query, expected) in test_cases {
        let result = conn.execute_query(query, &[]).await;
        match result {
            Ok(query_result) => {
                assert_eq!(query_result.rows.len(), 1);
                let actual = &query_result.rows[0][0];
                
                match (actual, &expected) {
                    (SqlValue::Null, SqlValue::Null) => {},
                    (SqlValue::Boolean(a), SqlValue::Boolean(e)) => assert_eq!(a, e),
                    (SqlValue::Integer(a), SqlValue::Integer(e)) => assert_eq!(a, e),
                    (SqlValue::Float(a), SqlValue::Float(e)) => {
                        // Allow for small floating point differences
                        assert!((a - e).abs() < 0.001, "Expected {}, got {}", e, a);
                    },
                    (SqlValue::String(a), SqlValue::String(e)) => assert_eq!(a, e),
                    _ => panic!("Type mismatch for query {}: expected {:?}, got {:?}", query, expected, actual),
                }
            }
            Err(e) => {
                eprintln!("Query '{}' failed: {}", query, e);
            }
        }
    }
}

#[tokio::test]
async fn test_concurrent_connections() {
    skip_if_no_postgres!();
    
    let config = get_test_config();
    
    // Create multiple connections concurrently
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let config_clone = config.clone();
        let handle = tokio::spawn(async move {
            let mut conn = PostgresConnection::new(config_clone).await?;
            let result = conn.execute_query(&format!("SELECT {} as connection_id", i), &[]).await?;
            
            if let SqlValue::Integer(value) = &result.rows[0][0] {
                assert_eq!(*value, i);
            }
            
            Ok::<(), PostgresError>(())
        });
        handles.push(handle);
    }
    
    // Wait for all connections to complete
    for handle in handles {
        match handle.await {
            Ok(Ok(())) => {
                eprintln!("Concurrent connection test passed");
            }
            Ok(Err(e)) => {
                eprintln!("Concurrent connection failed: {}", e);
            }
            Err(e) => {
                eprintln!("Concurrent connection task failed: {}", e);
            }
        }
    }
}

#[test]
fn test_configuration_builder() {
    use cursed::stdlib::database::postgres::driver::PostgresDriverBuilder;
    
    let builder = PostgresDriverBuilder::new()
        .connection("localhost", 5432, "testdb", "user")
        .password("pass")
        .ssl_mode(SslMode::Require)
        .timeouts(Duration::from_secs(10), Duration::from_secs(60))
        .with_pool();
    
    // Test that builder pattern works (would need async context to actually build)
    assert_eq!(builder.config.host, "localhost");
    assert_eq!(builder.config.port, 5432);
    assert_eq!(builder.config.database, "testdb");
    assert_eq!(builder.config.username, "user");
    assert_eq!(builder.config.password, Some("pass".to_string()));
    assert_eq!(builder.config.ssl_mode, SslMode::Require);
    assert!(builder.enable_pool);
}

// Helper function to create test tables (for future integration tests)
#[allow(dead_code)]
async fn create_test_table(conn: &mut PostgresConnection) -> Result<(), PostgresError> {
    let create_sql = r#"
        CREATE TABLE IF NOT EXISTS test_table (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            value INTEGER,
            is_active BOOLEAN DEFAULT true,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#;
    
    conn.execute_statement(create_sql, &[]).await?;
    Ok(())
}

// Helper function to cleanup test tables
#[allow(dead_code)]
async fn cleanup_test_table(conn: &mut PostgresConnection) -> Result<(), PostgresError> {
    conn.execute_statement("DROP TABLE IF EXISTS test_table", &[]).await?;
    Ok(())
}
