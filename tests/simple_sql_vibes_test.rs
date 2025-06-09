/// fr fr Simple SQL vibes integration tests - basic functionality testing periodt
use cursed::stdlib::packages::sql_vibes::{
    SimpleConnection, connect, quick_query,
    SqlValue, Parameter, Row, ResultSet, SqlError
};

#[path = "common.rs"]
mod common;

/// Test basic connection functionality
#[test]
fn test_basic_connection_functionality() {
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
    assert_eq!(info.get("connection_string"), Some(&"sqlite://test.db".to_string()));
    assert_eq!(info.get("status"), Some(&"open".to_string()));
    
    // Test closing connection
    assert!(connection.close().is_ok(), "Connection close should succeed");
    assert!(!connection.is_alive(), "Connection should not be alive after close");
    
    tracing::info!("Basic connection functionality validated");
}

/// Test query execution functionality
#[test] 
fn test_query_execution() {
    common::tracing::setup();
    
    let mut conn = connect("sqlite://test.db").unwrap();
    
    // Test SELECT query
    let result = conn.execute_query("SELECT * FROM users", &[]);
    assert!(result.is_ok(), "SELECT query should succeed");
    
    let result_set = result.unwrap();
    assert!(!result_set.is_empty(), "Result set should not be empty");
    assert_eq!(result_set.column_count(), 3, "Should have 3 columns");
    assert_eq!(result_set.row_count(), 2, "Should have 2 rows");
    
    // Test first row access
    let first_row = result_set.first_row().unwrap();
    assert_eq!(first_row.len(), 3, "First row should have 3 values");
    
    // Test value access
    let id_value = first_row.get(0).unwrap();
    assert_eq!(id_value, &SqlValue::Integer(1));
    
    let name_value = first_row.get(1).unwrap();
    assert_eq!(name_value, &SqlValue::String("Mock Row 1".to_string()));
    
    // Test non-SELECT query
    let empty_result = conn.execute_query("CREATE TABLE test (id INTEGER)", &[]);
    assert!(empty_result.is_ok(), "CREATE query should succeed");
    let empty_set = empty_result.unwrap();
    assert!(empty_set.is_empty(), "CREATE result should be empty");
    
    conn.close().unwrap();
    tracing::info!("Query execution functionality validated");
}

/// Test statement execution functionality
#[test]
fn test_statement_execution() {
    common::tracing::setup();
    
    let mut conn = connect("sqlite://test.db").unwrap();
    
    // Test INSERT statement
    let params = vec![Parameter::positional(0, SqlValue::String("John Doe".to_string()))];
    let result = conn.execute_statement("INSERT INTO users (name) VALUES (?)", &params);
    assert!(result.is_ok(), "INSERT statement should succeed");
    assert_eq!(result.unwrap(), 1, "Should affect 1 row");
    
    // Test UPDATE statement
    let update_params = vec![
        Parameter::positional(0, SqlValue::String("Jane Doe".to_string())),
        Parameter::positional(1, SqlValue::Integer(1)),
    ];
    let update_result = conn.execute_statement("UPDATE users SET name = ? WHERE id = ?", &update_params);
    assert!(update_result.is_ok(), "UPDATE statement should succeed");
    assert_eq!(update_result.unwrap(), 2, "Should affect 2 rows (number of params)");
    
    // Test DELETE statement
    let delete_result = conn.execute_statement("DELETE FROM users WHERE id = ?", &params);
    assert!(delete_result.is_ok(), "DELETE statement should succeed");
    assert_eq!(delete_result.unwrap(), 1, "Should affect 1 row");
    
    // Test other statement types
    let other_result = conn.execute_statement("CREATE INDEX idx_name ON users(name)", &[]);
    assert!(other_result.is_ok(), "Other statement should succeed");
    assert_eq!(other_result.unwrap(), 0, "Should affect 0 rows");
    
    conn.close().unwrap();
    tracing::info!("Statement execution functionality validated");
}

/// Test parameter handling with different types
#[test]
fn test_parameter_handling() {
    common::tracing::setup();
    
    // Test named parameters
    let named_param = Parameter::named("user_id".to_string(), SqlValue::Integer(42));
    assert_eq!(named_param.name_or_index(), "user_id");
    assert_eq!(named_param.value(), &SqlValue::Integer(42));
    
    // Test positional parameters
    let pos_param = Parameter::positional(0, SqlValue::String("test".to_string()));
    assert_eq!(pos_param.name_or_index(), "0");
    assert_eq!(pos_param.value(), &SqlValue::String("test".to_string()));
    
    // Test complex parameter set
    let params = vec![
        Parameter::named("name".to_string(), SqlValue::String("John Doe".to_string())),
        Parameter::named("age".to_string(), SqlValue::Integer(30)),
        Parameter::named("active".to_string(), SqlValue::Boolean(true)),
        Parameter::positional(0, SqlValue::Float(98.5)),
        Parameter::positional(1, SqlValue::Null),
    ];
    
    assert_eq!(params.len(), 5);
    
    // Validate parameter types
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
    
    tracing::info!("Parameter handling functionality validated");
}

/// Test ResultSet and Row functionality
#[test]
fn test_result_set_functionality() {
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
    ];
    
    let result_set = ResultSet::new(columns.clone(), rows);
    
    assert!(!result_set.is_empty());
    assert_eq!(result_set.row_count(), 2);
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
    
    // Test iteration
    let mut row_count = 0;
    for row in result_set.iter() {
        row_count += 1;
        for value in row.iter() {
            // Validate that we can access values
            match value {
                SqlValue::Integer(_) | SqlValue::String(_) => {},
                _ => {},
            }
        }
    }
    assert_eq!(row_count, 2);
    
    tracing::info!("ResultSet functionality validated");
}

/// Test error handling scenarios
#[test]
fn test_error_handling() {
    common::tracing::setup();
    
    // Test invalid connection string
    let bad_conn = connect("");
    assert!(bad_conn.is_err(), "Empty connection string should fail");
    
    match bad_conn.unwrap_err() {
        SqlError::Connection(msg) => {
            assert!(msg.contains("empty"));
            tracing::info!("Expected connection error: {}", msg);
        },
        _ => panic!("Expected connection error"),
    }
    
    // Test operations on closed connection
    let mut conn = connect("sqlite://test.db").unwrap();
    conn.close().unwrap();
    
    let query_result = conn.execute_query("SELECT 1", &[]);
    assert!(query_result.is_err(), "Query on closed connection should fail");
    
    let stmt_result = conn.execute_statement("INSERT INTO test VALUES (1)", &[]);
    assert!(stmt_result.is_err(), "Statement on closed connection should fail");
    
    // Test empty SQL
    let mut valid_conn = connect("sqlite://test.db").unwrap();
    let empty_sql_result = valid_conn.execute_query("", &[]);
    assert!(empty_sql_result.is_err(), "Empty SQL should fail");
    
    valid_conn.close().unwrap();
    tracing::info!("Error handling scenarios validated");
}

/// Test quick_query helper function
#[test]
fn test_quick_query_helper() {
    common::tracing::setup();
    
    // Test successful quick query
    let result = quick_query("sqlite://test.db", "SELECT 1, 'test', 42");
    assert!(result.is_ok(), "Quick query should succeed");
    
    let result_set = result.unwrap();
    assert!(!result_set.is_empty(), "Quick query result should not be empty");
    assert_eq!(result_set.row_count(), 2, "Should have 2 mock rows");
    assert_eq!(result_set.column_count(), 3, "Should have 3 mock columns");
    
    // Test quick query with invalid connection string
    let bad_result = quick_query("", "SELECT 1");
    assert!(bad_result.is_err(), "Quick query with bad connection should fail");
    
    // Test quick query with empty SQL
    let empty_sql_result = quick_query("sqlite://test.db", "");
    assert!(empty_sql_result.is_err(), "Quick query with empty SQL should fail");
    
    tracing::info!("Quick query helper functionality validated");
}

/// Test SQL value types and conversions
#[test]
fn test_sql_value_types() {
    common::tracing::setup();
    
    // Test different SQL value types
    let integer_val = SqlValue::Integer(42);
    let string_val = SqlValue::String("test".to_string());
    let boolean_val = SqlValue::Boolean(true);
    let float_val = SqlValue::Float(3.14);
    let null_val = SqlValue::Null;
    
    // Test type checking
    assert_eq!(integer_val.sql_type().to_string(), "INTEGER");
    assert_eq!(string_val.sql_type().to_string(), "STRING");
    assert_eq!(boolean_val.sql_type().to_string(), "BOOLEAN");
    assert_eq!(float_val.sql_type().to_string(), "FLOAT");
    assert_eq!(null_val.sql_type().to_string(), "NULL");
    
    // Test null checking
    assert!(!integer_val.is_null());
    assert!(!string_val.is_null());
    assert!(!boolean_val.is_null());
    assert!(!float_val.is_null());
    assert!(null_val.is_null());
    
    // Test conversions
    assert_eq!(integer_val.as_i32(), Some(42));
    assert_eq!(integer_val.as_i64(), Some(42));
    assert_eq!(integer_val.as_string(), None);
    
    assert_eq!(string_val.as_string(), Some("test".to_string()));
    assert_eq!(string_val.as_i32(), None);
    
    assert_eq!(boolean_val.as_bool(), Some(true));
    assert_eq!(float_val.as_f64(), Some(3.14));
    
    tracing::info!("SQL value types and conversions validated");
}

/// Test concurrent access to connections
#[test] 
fn test_concurrent_connections() {
    common::tracing::setup();
    
    use std::thread;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let success_count = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    // Spawn multiple threads to create connections
    for i in 0..5 {
        let success_count_clone = Arc::clone(&success_count);
        let handle = thread::spawn(move || {
            let connection_string = format!("sqlite://test_{}.db", i);
            
            // Test connection creation
            let conn_result = connect(&connection_string);
            if conn_result.is_ok() {
                let mut conn = conn_result.unwrap();
                
                // Test query execution
                let query_result = conn.execute_query("SELECT 1", &[]);
                if query_result.is_ok() {
                    // Test statement execution
                    let stmt_result = conn.execute_statement("INSERT INTO test VALUES (1)", &[]);
                    if stmt_result.is_ok() {
                        success_count_clone.fetch_add(1, Ordering::SeqCst);
                    }
                }
                
                let _ = conn.close();
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
    
    let final_count = success_count.load(Ordering::SeqCst);
    assert_eq!(final_count, 5, "All concurrent operations should succeed");
    
    tracing::info!("Concurrent connection access validated - {} successful operations", final_count);
}
