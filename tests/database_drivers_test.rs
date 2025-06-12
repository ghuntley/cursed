/// Tests for real database driver implementations
/// 
/// This test suite validates that the database drivers have real functionality
/// instead of placeholder stubs that panic at runtime.

use cursed::stdlib::database::sqlite::real_connection::RealSqliteConnection;
use cursed::stdlib::database::sqlite::{SqliteConfig, SqliteError};
use cursed::stdlib::database::{DriverConn, SqlValue, TxOptions};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_sqlite_connection_creation() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    
    let config = SqliteConfig {
        database_path: db_path.to_string_lossy().to_string(),
        ..Default::default()
    };
    
    let result = RealSqliteConnection::new(config);
    assert!(result.is_ok(), "Failed to create SQLite connection: {:?}", result.err());
    
    let conn = result.unwrap();
    assert!(!conn.connection_id().is_empty());
}

#[test]
fn test_sqlite_ping() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    
    let config = SqliteConfig {
        database_path: db_path.to_string_lossy().to_string(),
        ..Default::default()
    };
    
    let conn = RealSqliteConnection::new(config).expect("Failed to create connection");
    
    let ping_result = conn.ping();
    assert!(ping_result.is_ok(), "Ping failed: {:?}", ping_result.err());
    assert!(conn.is_alive());
}

#[test]
fn test_sqlite_create_table() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    
    let config = SqliteConfig {
        database_path: db_path.to_string_lossy().to_string(),
        ..Default::default()
    };
    
    let conn = RealSqliteConnection::new(config).expect("Failed to create connection");
    
    let create_sql = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)";
    let result = conn.execute(create_sql, &[]);
    
    assert!(result.is_ok(), "Failed to create table: {:?}", result.err());
    
    let execute_result = result.unwrap();
    assert_eq!(execute_result.rows_affected, 0); // DDL statements typically return 0
}

#[test]
fn test_sqlite_insert_and_query() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    
    let config = SqliteConfig {
        database_path: db_path.to_string_lossy().to_string(),
        ..Default::default()
    };
    
    let conn = RealSqliteConnection::new(config).expect("Failed to create connection");
    
    // Create table
    conn.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)", &[])
        .expect("Failed to create table");
    
    // Insert data
    let insert_result = conn.execute(
        "INSERT INTO users (name) VALUES (?)", 
        &[SqlValue::Text("Alice".to_string())]
    );
    assert!(insert_result.is_ok(), "Failed to insert: {:?}", insert_result.err());
    
    let insert_result = insert_result.unwrap();
    assert_eq!(insert_result.rows_affected, 1);
    assert!(insert_result.last_insert_id.is_some());
    
    // Query data
    let query_result = conn.query("SELECT id, name FROM users", &[]);
    assert!(query_result.is_ok(), "Failed to query: {:?}", query_result.err());
    
    let query_result = query_result.unwrap();
    assert_eq!(query_result.columns.len(), 2);
    assert_eq!(query_result.columns[0], "id");
    assert_eq!(query_result.columns[1], "name");
    assert_eq!(query_result.rows.len(), 1);
    
    // Check the data
    let row = &query_result.rows[0];
    assert_eq!(row.len(), 2);
    match &row[1] {
        SqlValue::Text(name) => assert_eq!(name, "Alice"),
        _ => panic!("Expected text value for name"),
    }
}

#[test]
fn test_sqlite_transaction() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    
    let config = SqliteConfig {
        database_path: db_path.to_string_lossy().to_string(),
        ..Default::default()
    };
    
    let conn = RealSqliteConnection::new(config).expect("Failed to create connection");
    
    // Create table
    conn.execute("CREATE TABLE counters (id INTEGER PRIMARY KEY, value INTEGER)", &[])
        .expect("Failed to create table");
    
    // Begin transaction
    let tx_result = conn.begin_transaction(TxOptions::default());
    assert!(tx_result.is_ok(), "Failed to begin transaction: {:?}", tx_result.err());
    
    let mut tx = tx_result.unwrap();
    
    // Test commit
    let commit_result = tx.commit();
    assert!(commit_result.is_ok(), "Failed to commit transaction: {:?}", commit_result.err());
}

#[test]
fn test_sqlite_prepared_statement() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    
    let config = SqliteConfig {
        database_path: db_path.to_string_lossy().to_string(),
        ..Default::default()
    };
    
    let conn = RealSqliteConnection::new(config).expect("Failed to create connection");
    
    // Create table
    conn.execute("CREATE TABLE items (id INTEGER PRIMARY KEY, description TEXT)", &[])
        .expect("Failed to create table");
    
    // Prepare statement
    let stmt_result = conn.prepare("INSERT INTO items (description) VALUES (?)");
    assert!(stmt_result.is_ok(), "Failed to prepare statement: {:?}", stmt_result.err());
    
    let _stmt = stmt_result.unwrap();
    // Note: Statement execution would require additional implementation
}

#[test]
fn test_sqlite_connection_metadata() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    
    let config = SqliteConfig {
        database_path: db_path.to_string_lossy().to_string(),
        ..Default::default()
    };
    
    let conn = RealSqliteConnection::new(config).expect("Failed to create connection");
    
    let metadata = conn.metadata();
    assert!(!metadata.connection_id.is_empty());
    assert_eq!(metadata.driver_name, "SQLite");
    assert!(!metadata.database_name.is_empty());
    assert!(!metadata.is_read_only);
}

#[test]
fn test_sqlite_close_connection() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    
    let config = SqliteConfig {
        database_path: db_path.to_string_lossy().to_string(),
        ..Default::default()
    };
    
    let conn = RealSqliteConnection::new(config).expect("Failed to create connection");
    
    // Test close
    let close_result = conn.close();
    assert!(close_result.is_ok(), "Failed to close connection: {:?}", close_result.err());
}

#[test]
fn test_memory_database() {
    let config = SqliteConfig {
        database_path: ":memory:".to_string(),
        ..Default::default()
    };
    
    let conn = RealSqliteConnection::new(config).expect("Failed to create in-memory connection");
    
    // Test that memory database works
    assert!(conn.ping().is_ok());
    
    // Create and use table
    conn.execute("CREATE TABLE temp (id INTEGER)", &[])
        .expect("Failed to create table in memory database");
    
    conn.execute("INSERT INTO temp (id) VALUES (42)", &[])
        .expect("Failed to insert into memory database");
    
    let result = conn.query("SELECT id FROM temp", &[])
        .expect("Failed to query memory database");
    
    assert_eq!(result.rows.len(), 1);
}

// Integration test to verify we're not using placeholder stubs
#[test]
fn test_no_placeholder_stubs() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    
    let config = SqliteConfig {
        database_path: db_path.to_string_lossy().to_string(),
        ..Default::default()
    };
    
    let conn = RealSqliteConnection::new(config).expect("Failed to create connection");
    
    // These operations should NOT panic or return "NotImplemented" errors
    
    // Test ping - should work
    let ping_result = conn.ping();
    assert!(ping_result.is_ok(), "Ping should work, got: {:?}", ping_result.err());
    
    // Test execute - should work for valid SQL
    let execute_result = conn.execute("CREATE TABLE test (id INTEGER)", &[]);
    assert!(execute_result.is_ok(), "Execute should work, got: {:?}", execute_result.err());
    
    // Test query - should work for valid SQL
    let query_result = conn.query("SELECT 1 as test_col", &[]);
    assert!(query_result.is_ok(), "Query should work, got: {:?}", query_result.err());
    
    // Test begin_transaction - should work
    let tx_result = conn.begin_transaction(TxOptions::default());
    assert!(tx_result.is_ok(), "Begin transaction should work, got: {:?}", tx_result.err());
    
    // Test prepare - should work
    let prepare_result = conn.prepare("SELECT * FROM test");
    assert!(prepare_result.is_ok(), "Prepare should work, got: {:?}", prepare_result.err());
    
    println!("✅ All database operations work - no placeholder stubs found!");
}
