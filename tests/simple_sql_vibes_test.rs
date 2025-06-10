/// Simple SQL vibes integration tests - basic functionality testing
use cursed::stdlib::packages::sql_vibes::{SimpleConnection, connect, quick_query,
    SqlValue, Parameter, Row, ResultSet, SqlError};

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
    assert_eq!(info.get("connection_string").unwrap(), "sqlite://test.db");
    
    // Test close
    assert!(connection.close().is_ok());
    assert!(!connection.is_alive(), "Connection should not be alive after close");
    
    tracing::info!("Basic connection test passed");
}

/// Test query execution functionality
#[test] 
fn test_query_execution() {
    common::tracing::setup();
    
    let mut conn = connect("sqlite://test.db").unwrap();
    
    // Create a simple table
    let create_result = conn.execute("CREATE TABLE IF NOT EXISTS test (id INTEGER, name TEXT)", &[]);
    assert!(create_result.is_ok(), "Table creation should succeed");
    
    // Insert some data
    let insert_result = conn.execute("INSERT INTO test (id, name) VALUES (?, ?)", 
        &[Parameter::from(1), Parameter::from("test")]);
    assert!(insert_result.is_ok(), "Insert should succeed");
    
    // Query the data
    let query_result = conn.query("SELECT id, name FROM test WHERE id = ?", &[Parameter::from(1)]);
    assert!(query_result.is_ok(), "Query should succeed");
    
    let result_set = query_result.unwrap();
    assert!(result_set.has_rows(), "Should have rows");
    
    tracing::info!("Query execution test passed");
}

/// Test parameter handling
#[test]
fn test_parameter_handling() {
    common::tracing::setup();
    
    // Test named parameters
    let named_param = Parameter::named("id", SqlValue::Integer(42));
    assert_eq!(named_param.name(), Some("id"));
    
    // Test positional parameters
    let pos_param = Parameter::positional(SqlValue::Text("hello".to_string()));
    assert!(pos_param.name().is_none());
    
    tracing::info!("Parameter handling test passed");
}

/// Test ResultSet and Row functionality
#[test]
fn test_result_set_functionality() {
    common::tracing::setup();
    
    let mut conn = connect("sqlite://test.db").unwrap();
    
    // Setup test data
    let _ = conn.execute("CREATE TABLE IF NOT EXISTS result_test (id INTEGER, value TEXT)", &[]);
    let _ = conn.execute("INSERT INTO result_test VALUES (1, 'first')", &[]);
    let _ = conn.execute("INSERT INTO result_test VALUES (2, 'second')", &[]);
    
    let result_set = conn.query("SELECT * FROM result_test ORDER BY id", &[]).unwrap();
    
    assert!(result_set.has_rows());
    assert_eq!(result_set.row_count(), 2);
    
    let rows: Vec<Row> = result_set.into_iter().collect();
    assert_eq!(rows.len(), 2);
    
    tracing::info!("ResultSet functionality test passed");
}
