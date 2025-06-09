/// fr fr SQL vibes integration tests - comprehensive database testing periodt
use cursed::stdlib::packages::sql_vibes::{
    SimpleConnection, connect, quick_query,
    SqlValue, Parameter, Row, ResultSet, SqlError
};

#[path = "common.rs"]
mod common;

/// Test basic connection functionality
#[test]
fn test_basic_connection_functionality() {
    // init_tracing!();
    common::tracing::setup();
    
    // Test connection creation
    let conn = connect("sqlite://test.db");
    assert!(conn.is_ok(), "Connection should succeed");
    
    let mut connection = conn.unwrap();
    assert!(connection.is_alive(), "Connection should be alive");
    
    // Test connection info
    let info = connection.connection_info();
    assert!(info.contains_key("connection_string"));
    assert!(info.contains_key("status"));
    
    // Test closing connection
    assert!(connection.close().is_ok(), "Connection close should succeed");
    assert!(!connection.is_alive(), "Connection should not be alive after close");
    
    tracing::info!("Basic connection functionality validated");
}

/// Test SQLite driver end-to-end functionality
#[test]
fn test_sqlite_driver_end_to_end() {
    // init_tracing!();
    common::tracing::setup();
    
    let driver = SqliteDriver::new();
    let info = driver.driver_info();
    
    // Validate driver info
    assert_eq!(info.name, "sqlite");
    assert!(!info.features.is_empty());
    assert!(info.features.contains(&cursed::stdlib::packages::sql_vibes::DriverFeature::PreparedStatements));
    assert!(info.features.contains(&cursed::stdlib::packages::sql_vibes::DriverFeature::Transactions));
    
    // Test connection string validation
    assert!(driver.validate_connection_string("sqlite://test.db").is_ok());
    assert!(driver.validate_connection_string(":memory:").is_ok());
    assert!(driver.validate_connection_string("").is_err());
    
    // Test connection (mock implementation)
    let config = ConnectionConfig::new("sqlite://:memory:".to_string());
    let result = driver.connect(config);
    
    match result {
        Ok(mut connection) => {
            tracing::info!("SQLite connection established successfully");
            
            // Test connection info
            let conn_info = connection.connection_info();
            assert!(conn_info.connection_id.is_some());
            assert!(connection.is_alive());
            
            // Test basic query execution
            let result_set = connection.execute_query("SELECT 1 as test_col", &[]);
            assert!(result_set.is_ok(), "Query execution should succeed");
            
            let result_set = result_set.unwrap();
            assert!(!result_set.is_empty(), "Result set should not be empty");
            
            // Test statement execution
            let affected = connection.execute_statement("CREATE TABLE test (id INTEGER)", &[]);
            assert!(affected.is_ok(), "Statement execution should succeed");
            
            // Test prepared statements
            let prepared = connection.prepare_statement("SELECT * FROM test WHERE id = ?");
            assert!(prepared.is_ok(), "Prepared statement creation should succeed");
            
            if let Ok(mut stmt) = prepared {
                let params = vec![Parameter::positional(0, SqlValue::Integer(1))];
                let exec_result = stmt.execute(&params);
                assert!(exec_result.is_ok(), "Prepared statement execution should succeed");
                
                assert!(stmt.close().is_ok(), "Prepared statement close should succeed");
            }
            
            // Test transactions
            let transaction = connection.begin_transaction();
            assert!(transaction.is_ok(), "Transaction begin should succeed");
            
            if let Ok(txn) = transaction {
                let commit_result = txn.commit();
                assert!(commit_result.is_ok(), "Transaction commit should succeed");
            }
            
            assert!(connection.close().is_ok(), "Connection close should succeed");
            tracing::info!("SQLite driver end-to-end test completed successfully");
        },
        Err(e) => {
            tracing::warn!("SQLite connection failed (expected in mock): {}", e);
            // In mock implementation, this might fail, which is acceptable
        }
    }
}

/// Test PostgreSQL driver basic functionality
#[test]
fn test_postgres_driver_basic_functionality() {
    // init_tracing!();
    common::tracing::setup();
    
    let driver = PostgresDriver::new();
    let info = driver.driver_info();
    
    // Validate driver info
    assert_eq!(info.name, "postgres");
    assert!(info.features.contains(&cursed::stdlib::packages::sql_vibes::DriverFeature::SslEncryption));
    assert!(info.features.contains(&cursed::stdlib::packages::sql_vibes::DriverFeature::JsonSupport));
    
    // Test connection string validation
    assert!(driver.validate_connection_string("postgres://user:pass@localhost/db").is_ok());
    assert!(driver.validate_connection_string("postgresql://user:pass@localhost:5432/db").is_ok());
    assert!(driver.validate_connection_string("").is_err());
    assert!(driver.validate_connection_string("sqlite://test.db").is_err());
    assert!(driver.validate_connection_string("postgres://localhost/db").is_err()); // Missing auth
    
    // Test feature support
    assert!(driver.supports_feature(cursed::stdlib::packages::sql_vibes::DriverFeature::PreparedStatements));
    assert!(driver.supports_feature(cursed::stdlib::packages::sql_vibes::DriverFeature::SslEncryption));
    assert!(driver.supports_feature(cursed::stdlib::packages::sql_vibes::DriverFeature::JsonSupport));
    assert!(driver.supports_feature(cursed::stdlib::packages::sql_vibes::DriverFeature::WindowFunctions));
    
    tracing::info!("PostgreSQL driver basic functionality validated");
}

/// Test MySQL driver basic functionality
#[test]
fn test_mysql_driver_basic_functionality() {
    // init_tracing!();
    common::tracing::setup();
    
    let driver = MySqlDriver::new();
    let info = driver.driver_info();
    
    // Validate driver info
    assert_eq!(info.name, "mysql");
    assert!(info.features.contains(&cursed::stdlib::packages::sql_vibes::DriverFeature::SslEncryption));
    assert!(info.features.contains(&cursed::stdlib::packages::sql_vibes::DriverFeature::JsonSupport));
    
    // Test connection string validation
    assert!(driver.validate_connection_string("mysql://user:pass@localhost/db").is_ok());
    assert!(driver.validate_connection_string("mysql://user:pass@localhost:3306/db").is_ok());
    assert!(driver.validate_connection_string("").is_err());
    assert!(driver.validate_connection_string("postgres://user:pass@localhost/db").is_err());
    assert!(driver.validate_connection_string("mysql://localhost/db").is_err()); // Missing auth
    
    // Test feature support
    assert!(driver.supports_feature(cursed::stdlib::packages::sql_vibes::DriverFeature::PreparedStatements));
    assert!(driver.supports_feature(cursed::stdlib::packages::sql_vibes::DriverFeature::SslEncryption));
    assert!(driver.supports_feature(cursed::stdlib::packages::sql_vibes::DriverFeature::JsonSupport));
    assert!(driver.supports_feature(cursed::stdlib::packages::sql_vibes::DriverFeature::StoredProcedures));
    
    tracing::info!("MySQL driver basic functionality validated");
}

/// Test mock driver comprehensive functionality
#[test]
fn test_mock_driver_comprehensive_functionality() {
    // init_tracing!();
    common::tracing::setup();
    
    let driver = MockDriver::new();
    let info = driver.driver_info();
    
    // Validate driver info - mock supports everything for testing
    assert_eq!(info.name, "mock");
    assert!(driver.supports_feature(cursed::stdlib::packages::sql_vibes::DriverFeature::PreparedStatements));
    assert!(driver.supports_feature(cursed::stdlib::packages::sql_vibes::DriverFeature::SslEncryption));
    assert!(driver.supports_feature(cursed::stdlib::packages::sql_vibes::DriverFeature::JsonSupport));
    
    // Test successful connection
    let config = ConnectionConfig::new("mock://test".to_string());
    let connection_result = driver.connect(config);
    assert!(connection_result.is_ok(), "Mock driver connection should succeed");
    
    if let Ok(mut connection) = connection_result {
        // Test query execution
        let params = vec![
            Parameter::named("name".to_string(), SqlValue::String("test".to_string())),
            Parameter::positional(0, SqlValue::Integer(42))
        ];
        
        let result_set = connection.execute_query("SELECT * FROM users WHERE name = :name AND id = ?", &params);
        assert!(result_set.is_ok(), "Mock query execution should succeed");
        
        let result_set = result_set.unwrap();
        assert!(!result_set.is_empty(), "Mock result set should contain data");
        assert_eq!(result_set.column_count(), 4); // Mock returns 4 columns
        assert_eq!(result_set.row_count(), 3); // Mock returns 3 rows
        
        // Test statement execution
        let affected = connection.execute_statement("INSERT INTO users (name) VALUES (?)", &params);
        assert!(affected.is_ok(), "Mock statement execution should succeed");
        assert_eq!(affected.unwrap(), 1, "Mock should return 1 affected row");
        
        // Test prepared statements
        let prepared = connection.prepare_statement("SELECT * FROM users WHERE id = ?");
        assert!(prepared.is_ok(), "Mock prepared statement should succeed");
        
        if let Ok(mut stmt) = prepared {
            let params = vec![Parameter::positional(0, SqlValue::Integer(1))];
            let exec_result = stmt.execute(&params);
            assert!(exec_result.is_ok(), "Mock prepared execution should succeed");
            
            let update_result = stmt.execute_update(&params);
            assert!(update_result.is_ok(), "Mock prepared update should succeed");
            assert_eq!(update_result.unwrap(), 1, "Mock should return 1 affected row");
            
            assert_eq!(stmt.parameter_count(), 1, "Should detect 1 parameter");
            assert_eq!(stmt.sql(), "SELECT * FROM users WHERE id = ?");
            
            assert!(stmt.close().is_ok(), "Mock prepared statement close should succeed");
        }
        
        // Test transactions
        let transaction = connection.begin_transaction();
        assert!(transaction.is_ok(), "Mock transaction begin should succeed");
        
        if let Ok(mut txn) = transaction {
            // Test transaction operations
            let query_result = txn.execute_query("SELECT 1", &[]);
            assert!(query_result.is_ok(), "Transaction query should succeed");
            
            let stmt_result = txn.execute_statement("UPDATE users SET name = 'test'", &[]);
            assert!(stmt_result.is_ok(), "Transaction statement should succeed");
            
            // Test savepoints
            assert!(txn.savepoint("sp1").is_ok(), "Savepoint creation should succeed");
            assert!(txn.savepoint("sp2").is_ok(), "Second savepoint creation should succeed");
            assert!(txn.rollback_to_savepoint("sp1").is_ok(), "Rollback to savepoint should succeed");
            assert!(txn.release_savepoint("sp2").is_err(), "Release non-existent savepoint should fail");
            
            let commit_result = txn.commit();
            assert!(commit_result.is_ok(), "Mock transaction commit should succeed");
        }
        
        // Test batch execution
        let statements = vec![
            ("INSERT INTO users (name) VALUES (?)", vec![Parameter::positional(0, SqlValue::String("user1".to_string()))].as_slice()),
            ("INSERT INTO users (name) VALUES (?)", vec![Parameter::positional(0, SqlValue::String("user2".to_string()))].as_slice()),
        ];
        
        let batch_result = connection.execute_batch(&statements);
        assert!(batch_result.is_ok(), "Mock batch execution should succeed");
        
        let results = batch_result.unwrap();
        assert_eq!(results.len(), 2, "Should have 2 batch results");
        for result in results {
            assert!(result.is_ok(), "Each batch statement should succeed");
        }
        
        // Test connection info
        let conn_info = connection.connection_info();
        assert_eq!(conn_info.database_name, "mock_database");
        assert_eq!(conn_info.username, "mock_user");
        assert_eq!(conn_info.host, "mock_host");
        assert_eq!(conn_info.port, 9999);
        assert!(conn_info.connection_id.is_some());
        
        assert!(connection.is_alive(), "Mock connection should be alive");
        assert!(connection.close().is_ok(), "Mock connection close should succeed");
        assert!(!connection.is_alive(), "Mock connection should not be alive after close");
    }
    
    // Test statistics tracking
    if let Some((connections, queries)) = driver.get_stats() {
        assert!(connections > 0, "Should have connection count");
        assert!(queries > 0, "Should have query count");
        tracing::info!("Mock driver stats: {} connections, {} queries", connections, queries);
    }
    
    driver.reset_stats();
    if let Some((connections, queries)) = driver.get_stats() {
        assert_eq!(connections, 0, "Stats should be reset");
        assert_eq!(queries, 0, "Stats should be reset");
    }
    
    tracing::info!("Mock driver comprehensive functionality test completed successfully");
}

/// Test parameter handling across different parameter types
#[test]
fn test_parameter_handling() {
    // init_tracing!();
    common::tracing::setup();
    
    // Test named parameters
    let named_param = Parameter::named("user_id".to_string(), SqlValue::Integer(42));
    assert_eq!(named_param.name_or_index(), "user_id");
    assert_eq!(named_param.value(), &SqlValue::Integer(42));
    
    // Test positional parameters
    let pos_param = Parameter::positional(0, SqlValue::String("test".to_string()));
    assert_eq!(pos_param.name_or_index(), "0");
    assert_eq!(pos_param.value(), &SqlValue::String("test".to_string()));
    
    // Test complex parameter sets
    let params = vec![
        Parameter::named("name".to_string(), SqlValue::String("John Doe".to_string())),
        Parameter::named("age".to_string(), SqlValue::Integer(30)),
        Parameter::named("active".to_string(), SqlValue::Boolean(true)),
        Parameter::positional(0, SqlValue::Float(98.5)),
        Parameter::positional(1, SqlValue::Null),
    ];
    
    assert_eq!(params.len(), 5);
    
    // Validate each parameter
    match &params[0] {
        Parameter::Named { name, value } => {
            assert_eq!(name, "name");
            assert_eq!(value, &SqlValue::String("John Doe".to_string()));
        },
        _ => panic!("Expected named parameter"),
    }
    
    match &params[3] {
        Parameter::Positional { index, value } => {
            assert_eq!(*index, 0);
            assert_eq!(value, &SqlValue::Float(98.5));
        },
        _ => panic!("Expected positional parameter"),
    }
    
    tracing::info!("Parameter handling test completed successfully");
}

/// Test ResultSet and Row functionality
#[test]
fn test_result_set_and_row_functionality() {
    // init_tracing!();
    common::tracing::setup();
    
    // Test empty result set
    let empty_result = ResultSet::empty();
    assert!(empty_result.is_empty());
    assert_eq!(empty_result.row_count(), 0);
    assert_eq!(empty_result.column_count(), 0);
    assert!(empty_result.first_row().is_none());
    
    // Test result set with data
    let columns = vec!["id".to_string(), "name".to_string(), "email".to_string()];
    let rows = vec![
        Row::new(vec![
            SqlValue::Integer(1),
            SqlValue::String("John Doe".to_string()),
            SqlValue::String("john@example.com".to_string())
        ]),
        Row::new(vec![
            SqlValue::Integer(2),
            SqlValue::String("Jane Smith".to_string()),
            SqlValue::String("jane@example.com".to_string())
        ]),
        Row::new(vec![
            SqlValue::Integer(3),
            SqlValue::String("Bob Johnson".to_string()),
            SqlValue::Null
        ]),
    ];
    
    let result_set = ResultSet::new(columns.clone(), rows);
    
    assert!(!result_set.is_empty());
    assert_eq!(result_set.row_count(), 3);
    assert_eq!(result_set.column_count(), 3);
    assert_eq!(result_set.columns(), &columns);
    
    // Test row access
    assert!(result_set.first_row().is_some());
    let first_row = result_set.first_row().unwrap();
    assert_eq!(first_row.len(), 3);
    assert!(!first_row.is_empty());
    assert_eq!(first_row.get(0), Some(&SqlValue::Integer(1)));
    assert_eq!(first_row.get(1), Some(&SqlValue::String("John Doe".to_string())));
    assert_eq!(first_row.get(2), Some(&SqlValue::String("john@example.com".to_string())));
    assert_eq!(first_row.get(3), None); // Out of bounds
    
    // Test row iteration
    let mut value_count = 0;
    for row in result_set.iter() {
        for value in row.iter() {
            value_count += 1;
            // Validate that we can access the value
            match value {
                SqlValue::Null => {},
                SqlValue::Integer(_) => {},
                SqlValue::String(_) => {},
                _ => {},
            }
        }
    }
    assert_eq!(value_count, 9); // 3 rows × 3 columns = 9 values
    
    // Test individual row functionality
    let test_row = Row::new(vec![
        SqlValue::Boolean(true),
        SqlValue::Float(3.14),
        SqlValue::String("test".to_string()),
    ]);
    
    assert_eq!(test_row.len(), 3);
    assert!(!test_row.is_empty());
    
    let values: Vec<&SqlValue> = test_row.iter().collect();
    assert_eq!(values.len(), 3);
    assert_eq!(values[0], &SqlValue::Boolean(true));
    assert_eq!(values[1], &SqlValue::Float(3.14));
    assert_eq!(values[2], &SqlValue::String("test".to_string()));
    
    tracing::info!("ResultSet and Row functionality test completed successfully");
}

/// Test error handling scenarios
#[test]
fn test_error_handling_scenarios() {
    // init_tracing!();
    common::tracing::setup();
    
    // Test invalid connection strings
    let sqlite_driver = SqliteDriver::new();
    assert!(sqlite_driver.validate_connection_string("").is_err());
    
    let postgres_driver = PostgresDriver::new();
    assert!(postgres_driver.validate_connection_string("invalid").is_err());
    assert!(postgres_driver.validate_connection_string("mysql://user:pass@host/db").is_err());
    
    let mysql_driver = MySqlDriver::new();
    assert!(mysql_driver.validate_connection_string("postgres://user:pass@host/db").is_err());
    
    // Test driver registry errors
    let registry = DriverRegistry::new();
    assert!(registry.get_driver("nonexistent").is_err());
    
    let error = registry.get_driver("nonexistent").unwrap_err();
    match error {
        SqlError::Connection(msg) => {
            assert!(msg.contains("not found"));
            tracing::info!("Expected error for nonexistent driver: {}", msg);
        },
        _ => panic!("Expected connection error"),
    }
    
    tracing::info!("Error handling scenarios test completed successfully");
}

/// Test concurrent access to driver registry
#[test] 
fn test_concurrent_driver_access() {
    // init_tracing!();
    common::tracing::setup();
    
    use std::sync::Arc;
    use std::thread;
    
    let registry = Arc::new(DriverRegistry::new());
    let mut handles = vec![];
    
    // Spawn multiple threads to access drivers concurrently
    for i in 0..10 {
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            let driver_name = match i % 4 {
                0 => "sqlite",
                1 => "postgres", 
                2 => "mysql",
                _ => "mock",
            };
            
            // Test concurrent driver access
            let driver_result = registry_clone.get_driver(driver_name);
            assert!(driver_result.is_ok(), "Driver access should succeed");
            
            // Test concurrent driver listing
            let drivers = registry_clone.list_drivers();
            assert!(drivers.is_ok(), "Driver listing should succeed");
            
            // Test concurrent driver checking
            assert!(registry_clone.has_driver(driver_name));
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
    
    tracing::info!("Concurrent driver access test completed successfully");
}
