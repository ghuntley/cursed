/// Comprehensive test suite for production SQLite driver
/// 
/// This test suite validates all aspects of the production SQLite driver including:
/// - Connection management and pooling
/// - Prepared statements and parameter binding
/// - Transaction management with savepoints
/// - Type conversion and result processing
/// - Error handling and edge cases
/// - Thread safety and concurrent access
/// - Performance characteristics
/// - Memory safety and resource cleanup

use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};
use std::fs;
use tempfile::{tempdir, NamedTempFile};
use cursed::stdlib::database::sqlite::production_driver::{
    ProductionSqliteConnection, ProductionSqliteStatement, ProductionSqliteTransaction
};
use cursed::stdlib::database::sqlite::{SqliteConfig, SqliteError};
use cursed::stdlib::database::{
    DriverConn, DriverStmt, DriverTx, SqlValue, TxOptions, DatabaseError,
    DatabaseErrorKind, SqlIsolationLevel
};

/// Create test database configuration
fn create_test_config() -> SqliteConfig {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_string_lossy().to_string();
    std::mem::forget(temp_file); // Keep file around
    
    SqliteConfig {
        database_path: path,
        read_only: false,
        enable_wal: true,
        enable_foreign_keys: true,
        enable_shared_cache: false,
        busy_timeout: Duration::from_secs(30),
        cache_size: 16384,
        page_size: 4096,
        journal_mode: "WAL".to_string(),
        synchronous: "NORMAL".to_string(),
        temp_store: "MEMORY".to_string(),
        mmap_size: 134217728, // 128MB
        max_page_count: Some(1073741823),
        ..SqliteConfig::default()
    }
}

/// Create in-memory database configuration
fn create_memory_config() -> SqliteConfig {
    SqliteConfig {
        database_path: ":memory:".to_string(),
        ..SqliteConfig::default()
    }
}

#[test]
fn test_connection_creation_and_basic_operations() {
    let config = create_test_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Test connection properties
    assert!(!conn.connection_id().is_empty());
    assert!(!conn.is_pooled());
    assert!(conn.is_alive());
    
    // Test ping
    assert!(conn.ping().is_ok());
    
    // Test metadata
    let metadata = conn.metadata();
    assert_eq!(metadata.server_host, "localhost");
    assert_eq!(metadata.server_port, 0);
    assert!(metadata.additional_info.contains_key("driver_name"));
    assert_eq!(metadata.additional_info["driver_name"], "ProductionSQLite");
    
    // Test cleanup
    assert!(conn.close().is_ok());
    assert!(!conn.is_alive());
}

#[test]
fn test_pooled_connection_creation() {
    let config = create_test_config();
    let conn = ProductionSqliteConnection::new_pooled(
        config,
        "test_pool".to_string(),
        0
    ).unwrap();
    
    assert!(conn.is_pooled());
    let pool_info = conn.pool_info().unwrap();
    assert_eq!(pool_info.pool_id, "test_pool");
    assert_eq!(pool_info.connection_index, 0);
}

#[test]
fn test_database_operations() {
    let config = create_memory_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Create table
    let create_result = conn.execute(
        "CREATE TABLE test_users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)",
        &[]
    );
    assert!(create_result.is_ok());
    
    // Insert data
    let insert_result = conn.execute(
        "INSERT INTO test_users (name, age) VALUES (?, ?)",
        &[SqlValue::String("Alice".to_string()), SqlValue::Integer(30)]
    );
    assert!(insert_result.is_ok());
    let insert_result = insert_result.unwrap();
    assert_eq!(insert_result.rows_affected, 1);
    assert!(insert_result.last_insert_id.is_some());
    
    // Query data
    let query_result = conn.query(
        "SELECT id, name, age FROM test_users WHERE name = ?",
        &[SqlValue::String("Alice".to_string())]
    );
    assert!(query_result.is_ok());
    let query_result = query_result.unwrap();
    assert_eq!(query_result.column_names, vec!["id", "name", "age"]);
    assert_eq!(query_result.rows.len(), 1);
    
    let row = &query_result.rows[0];
    assert_eq!(row.len(), 3);
    match &row[1] {
        SqlValue::String(name) => assert_eq!(name, "Alice"),
        _ => panic!("Expected string value"),
    }
    match &row[2] {
        SqlValue::Integer(age) => assert_eq!(*age, 30),
        _ => panic!("Expected integer value"),
    }
}

#[test]
fn test_prepared_statements() {
    let config = create_memory_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Create table
    conn.execute(
        "CREATE TABLE test_items (id INTEGER PRIMARY KEY, value TEXT)",
        &[]
    ).unwrap();
    
    // Test prepared statement
    let stmt = conn.prepare("INSERT INTO test_items (value) VALUES (?)").unwrap();
    
    // Execute multiple times
    for i in 0..5 {
        let result = stmt.execute(&[SqlValue::String(format!("Item {}", i))]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().rows_affected, 1);
    }
    
    // Query with prepared statement
    let query_stmt = conn.prepare("SELECT COUNT(*) FROM test_items").unwrap();
    let result = query_stmt.query(&[]);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 1);
    match &result.rows[0][0] {
        SqlValue::Integer(count) => assert_eq!(*count, 5),
        _ => panic!("Expected integer count"),
    }
}

#[test]
fn test_transaction_management() {
    let config = create_memory_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Create table
    conn.execute(
        "CREATE TABLE test_accounts (id INTEGER PRIMARY KEY, balance INTEGER)",
        &[]
    ).unwrap();
    
    // Insert initial data
    conn.execute(
        "INSERT INTO test_accounts (balance) VALUES (?), (?)",
        &[SqlValue::Integer(1000), SqlValue::Integer(500)]
    ).unwrap();
    
    // Test successful transaction
    {
        let tx = conn.begin_transaction(TxOptions::default()).unwrap();
        assert!(tx.is_active());
        
        tx.execute(
            "UPDATE test_accounts SET balance = balance - 100 WHERE id = 1",
            &[]
        ).unwrap();
        
        tx.execute(
            "UPDATE test_accounts SET balance = balance + 100 WHERE id = 2",
            &[]
        ).unwrap();
        
        assert!(tx.commit().is_ok());
    }
    
    // Verify transaction results
    let result = conn.query("SELECT balance FROM test_accounts ORDER BY id", &[]).unwrap();
    match (&result.rows[0][0], &result.rows[1][0]) {
        (SqlValue::Integer(bal1), SqlValue::Integer(bal2)) => {
            assert_eq!(*bal1, 900);
            assert_eq!(*bal2, 600);
        }
        _ => panic!("Expected integer balances"),
    }
    
    // Test rollback transaction
    {
        let tx = conn.begin_transaction(TxOptions::default()).unwrap();
        
        tx.execute(
            "UPDATE test_accounts SET balance = 0 WHERE id = 1",
            &[]
        ).unwrap();
        
        assert!(tx.rollback().is_ok());
    }
    
    // Verify rollback - balance should be unchanged
    let result = conn.query("SELECT balance FROM test_accounts WHERE id = 1", &[]).unwrap();
    match &result.rows[0][0] {
        SqlValue::Integer(balance) => assert_eq!(*balance, 900),
        _ => panic!("Expected integer balance"),
    }
}

#[test]
fn test_savepoint_transactions() {
    let config = create_memory_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Create table
    conn.execute(
        "CREATE TABLE test_values (id INTEGER PRIMARY KEY, value INTEGER)",
        &[]
    ).unwrap();
    
    // Begin main transaction
    let tx1 = conn.begin_transaction(TxOptions::default()).unwrap();
    
    tx1.execute("INSERT INTO test_values (value) VALUES (1)", &[]).unwrap();
    
    // Create savepoint
    let tx2 = conn.begin_transaction(TxOptions::default()).unwrap();
    assert!(tx2.is_active());
    
    tx2.execute("INSERT INTO test_values (value) VALUES (2)", &[]).unwrap();
    
    // Rollback savepoint
    assert!(tx2.rollback().is_ok());
    
    // Commit main transaction
    assert!(tx1.commit().is_ok());
    
    // Verify only first insert exists
    let result = conn.query("SELECT COUNT(*) FROM test_values", &[]).unwrap();
    match &result.rows[0][0] {
        SqlValue::Integer(count) => assert_eq!(*count, 1),
        _ => panic!("Expected integer count"),
    }
}

#[test]
fn test_type_conversions() {
    let config = create_memory_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Create table with various types
    conn.execute(
        "CREATE TABLE test_types (
            id INTEGER PRIMARY KEY,
            bool_val INTEGER,
            int_val INTEGER,
            float_val REAL,
            text_val TEXT,
            blob_val BLOB,
            null_val NULL
        )",
        &[]
    ).unwrap();
    
    // Insert test data
    let test_blob = vec![1, 2, 3, 4, 5];
    conn.execute(
        "INSERT INTO test_types (bool_val, int_val, float_val, text_val, blob_val, null_val) VALUES (?, ?, ?, ?, ?, ?)",
        &[
            SqlValue::Boolean(true),
            SqlValue::Integer(12345),
            SqlValue::Float(3.14159),
            SqlValue::String("Hello World".to_string()),
            SqlValue::Bytes(test_blob.clone()),
            SqlValue::Null,
        ]
    ).unwrap();
    
    // Query and verify types
    let result = conn.query(
        "SELECT bool_val, int_val, float_val, text_val, blob_val, null_val FROM test_types",
        &[]
    ).unwrap();
    
    assert_eq!(result.rows.len(), 1);
    let row = &result.rows[0];
    
    // Note: SQLite stores boolean as integer
    match &row[0] {
        SqlValue::Integer(val) => assert_eq!(*val, 1), // true as 1
        _ => panic!("Expected integer for boolean"),
    }
    
    match &row[1] {
        SqlValue::Integer(val) => assert_eq!(*val, 12345),
        _ => panic!("Expected integer"),
    }
    
    match &row[2] {
        SqlValue::Float(val) => assert!((val - 3.14159).abs() < 0.001),
        _ => panic!("Expected float"),
    }
    
    match &row[3] {
        SqlValue::String(val) => assert_eq!(val, "Hello World"),
        _ => panic!("Expected string"),
    }
    
    match &row[4] {
        SqlValue::Bytes(val) => assert_eq!(*val, test_blob),
        _ => panic!("Expected bytes"),
    }
    
    match &row[5] {
        SqlValue::Null => {}, // Expected
        _ => panic!("Expected null"),
    }
}

#[test]
fn test_error_handling() {
    let config = create_memory_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Test syntax error
    let result = conn.execute("INVALID SQL SYNTAX", &[]);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.kind(), &DatabaseErrorKind::QueryError);
    
    // Test table not found
    let result = conn.query("SELECT * FROM nonexistent_table", &[]);
    assert!(result.is_err());
    
    // Test constraint violation
    conn.execute(
        "CREATE TABLE test_unique (id INTEGER PRIMARY KEY, value TEXT UNIQUE)",
        &[]
    ).unwrap();
    
    conn.execute(
        "INSERT INTO test_unique (value) VALUES (?)",
        &[SqlValue::String("test".to_string())]
    ).unwrap();
    
    let result = conn.execute(
        "INSERT INTO test_unique (value) VALUES (?)",
        &[SqlValue::String("test".to_string())]
    );
    assert!(result.is_err());
}

#[test]
fn test_concurrent_access() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("concurrent_test.db");
    
    let config = SqliteConfig {
        database_path: db_path.to_string_lossy().to_string(),
        enable_wal: true, // Enable WAL for better concurrency
        ..SqliteConfig::default()
    };
    
    // Create table
    {
        let conn = ProductionSqliteConnection::new(config.clone()).unwrap();
        conn.execute(
            "CREATE TABLE test_concurrent (id INTEGER PRIMARY KEY, thread_id INTEGER, value INTEGER)",
            &[]
        ).unwrap();
    }
    
    let num_threads = 4;
    let operations_per_thread = 10;
    let barrier = Arc::new(Barrier::new(num_threads));
    
    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let config = config.clone();
        let barrier = barrier.clone();
        
        thread::spawn(move || {
            let conn = ProductionSqliteConnection::new(config).unwrap();
            
            // Wait for all threads to be ready
            barrier.wait();
            
            // Perform operations
            for i in 0..operations_per_thread {
                let result = conn.execute(
                    "INSERT INTO test_concurrent (thread_id, value) VALUES (?, ?)",
                    &[SqlValue::Integer(thread_id as i64), SqlValue::Integer(i)]
                );
                assert!(result.is_ok(), "Thread {} operation {} failed: {:?}", thread_id, i, result);
            }
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all data was inserted
    let conn = ProductionSqliteConnection::new(config).unwrap();
    let result = conn.query("SELECT COUNT(*) FROM test_concurrent", &[]).unwrap();
    match &result.rows[0][0] {
        SqlValue::Integer(count) => assert_eq!(*count, (num_threads * operations_per_thread) as i64),
        _ => panic!("Expected integer count"),
    }
}

#[test]
fn test_performance_characteristics() {
    let config = create_memory_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Create test table
    conn.execute(
        "CREATE TABLE test_performance (id INTEGER PRIMARY KEY, data TEXT)",
        &[]
    ).unwrap();
    
    // Test batch insert performance
    let num_records = 1000;
    let start = Instant::now();
    
    // Use transaction for better performance
    let tx = conn.begin_transaction(TxOptions::default()).unwrap();
    let stmt = tx.prepare("INSERT INTO test_performance (data) VALUES (?)").unwrap();
    
    for i in 0..num_records {
        stmt.execute(&[SqlValue::String(format!("Record {}", i))]).unwrap();
    }
    
    tx.commit().unwrap();
    let insert_duration = start.elapsed();
    
    println!("Inserted {} records in {:?}", num_records, insert_duration);
    assert!(insert_duration < Duration::from_secs(1), "Insert took too long: {:?}", insert_duration);
    
    // Test query performance
    let start = Instant::now();
    let result = conn.query("SELECT COUNT(*) FROM test_performance", &[]).unwrap();
    let query_duration = start.elapsed();
    
    match &result.rows[0][0] {
        SqlValue::Integer(count) => assert_eq!(*count, num_records),
        _ => panic!("Expected integer count"),
    }
    
    println!("Count query took {:?}", query_duration);
    assert!(query_duration < Duration::from_millis(100), "Query took too long: {:?}", query_duration);
}

#[test]
fn test_connection_statistics() {
    let config = create_memory_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Get initial statistics
    let initial_stats = conn.get_stats().unwrap();
    assert_eq!(initial_stats.queries_executed, 0);
    assert_eq!(initial_stats.statements_prepared, 0);
    
    // Execute some operations
    conn.execute("CREATE TABLE test_stats (id INTEGER PRIMARY KEY)", &[]).unwrap();
    
    let stmt = conn.prepare("INSERT INTO test_stats DEFAULT VALUES").unwrap();
    stmt.execute(&[]).unwrap();
    stmt.execute(&[]).unwrap();
    
    conn.query("SELECT COUNT(*) FROM test_stats", &[]).unwrap();
    
    // Check updated statistics
    let updated_stats = conn.get_stats().unwrap();
    assert!(updated_stats.queries_executed > initial_stats.queries_executed);
    assert!(updated_stats.statements_prepared > initial_stats.statements_prepared);
    assert!(updated_stats.total_query_time > Duration::ZERO);
}

#[test]
fn test_database_maintenance_operations() {
    let config = create_test_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Create some data
    conn.execute(
        "CREATE TABLE test_maintenance (id INTEGER PRIMARY KEY, data BLOB)",
        &[]
    ).unwrap();
    
    for i in 0..100 {
        let data = vec![i as u8; 1000]; // 1KB of data per row
        conn.execute(
            "INSERT INTO test_maintenance (data) VALUES (?)",
            &[SqlValue::Bytes(data)]
        ).unwrap();
    }
    
    // Test database size
    let initial_size = conn.get_database_size().unwrap();
    assert!(initial_size > 0);
    
    // Delete some data
    conn.execute("DELETE FROM test_maintenance WHERE id % 2 = 0", &[]).unwrap();
    
    // Test vacuum
    assert!(conn.vacuum().is_ok());
    
    // Test analyze
    assert!(conn.analyze().is_ok());
    
    // Size might be smaller after vacuum
    let final_size = conn.get_database_size().unwrap();
    assert!(final_size > 0);
}

#[test]
fn test_connection_timeout_and_busy_handling() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("timeout_test.db");
    
    let config = SqliteConfig {
        database_path: db_path.to_string_lossy().to_string(),
        busy_timeout: Duration::from_millis(100),
        ..SqliteConfig::default()
    };
    
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Test timeout setting
    assert!(conn.set_busy_timeout(Duration::from_millis(500)).is_ok());
    
    // Create table
    conn.execute(
        "CREATE TABLE test_timeout (id INTEGER PRIMARY KEY, data TEXT)",
        &[]
    ).unwrap();
    
    // Normal operation should work
    assert!(conn.execute(
        "INSERT INTO test_timeout (data) VALUES (?)",
        &[SqlValue::String("test".to_string())]
    ).is_ok());
}

#[test]
fn test_memory_safety_and_cleanup() {
    // Test that connections can be created and dropped without leaks
    for i in 0..10 {
        let config = create_memory_config();
        let conn = ProductionSqliteConnection::new(config).unwrap();
        
        // Do some work
        conn.execute(
            &format!("CREATE TABLE test_{} (id INTEGER)", i),
            &[]
        ).unwrap();
        
        let stmt = conn.prepare(&format!("INSERT INTO test_{} DEFAULT VALUES", i)).unwrap();
        stmt.execute(&[]).unwrap();
        
        let tx = conn.begin_transaction(TxOptions::default()).unwrap();
        tx.execute(&format!("INSERT INTO test_{} DEFAULT VALUES", i), &[]).unwrap();
        tx.commit().unwrap();
        
        // Connection will be dropped automatically
    }
}

#[test]
fn test_transaction_isolation_levels() {
    let config = create_test_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Create table
    conn.execute(
        "CREATE TABLE test_isolation (id INTEGER PRIMARY KEY, value INTEGER)",
        &[]
    ).unwrap();
    
    // Test different isolation levels
    let isolation_levels = vec![
        SqlIsolationLevel::LevelDefault,
        SqlIsolationLevel::LevelReadCommitted,
        SqlIsolationLevel::LevelSerializable,
    ];
    
    for isolation in isolation_levels {
        let opts = TxOptions {
            isolation_level: Some(isolation),
            read_only: false,
            timeout: None,
        };
        
        let tx = conn.begin_transaction(opts).unwrap();
        assert!(tx.is_active());
        
        tx.execute(
            "INSERT INTO test_isolation (value) VALUES (?)",
            &[SqlValue::Integer(1)]
        ).unwrap();
        
        assert!(tx.commit().is_ok());
    }
}

#[test]
fn test_large_data_handling() {
    let config = create_memory_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Create table for large data
    conn.execute(
        "CREATE TABLE test_large_data (id INTEGER PRIMARY KEY, large_text TEXT, large_blob BLOB)",
        &[]
    ).unwrap();
    
    // Test large text (1MB)
    let large_text = "A".repeat(1024 * 1024);
    
    // Test large blob (1MB)
    let large_blob = vec![0xAB; 1024 * 1024];
    
    let result = conn.execute(
        "INSERT INTO test_large_data (large_text, large_blob) VALUES (?, ?)",
        &[SqlValue::String(large_text.clone()), SqlValue::Bytes(large_blob.clone())]
    );
    assert!(result.is_ok());
    
    // Query back the large data
    let result = conn.query(
        "SELECT large_text, large_blob FROM test_large_data",
        &[]
    ).unwrap();
    
    assert_eq!(result.rows.len(), 1);
    match (&result.rows[0][0], &result.rows[0][1]) {
        (SqlValue::String(text), SqlValue::Bytes(blob)) => {
            assert_eq!(text.len(), large_text.len());
            assert_eq!(blob.len(), large_blob.len());
            assert_eq!(text, &large_text);
            assert_eq!(blob, &large_blob);
        }
        _ => panic!("Expected string and bytes"),
    }
}

#[test]
fn test_connection_cloning() {
    let config = create_memory_config();
    let conn1 = ProductionSqliteConnection::new(config).unwrap();
    
    // Create table in first connection
    conn1.execute(
        "CREATE TABLE test_clone (id INTEGER PRIMARY KEY, data TEXT)",
        &[]
    ).unwrap();
    
    // Clone connection (creates new connection with same config)
    let conn2 = conn1.clone();
    
    // Both connections should work independently
    conn1.execute(
        "INSERT INTO test_clone (data) VALUES (?)",
        &[SqlValue::String("from_conn1".to_string())]
    ).unwrap();
    
    // Note: SQLite cloning creates a new connection to the same database
    // For memory databases, this creates a separate database
    // For file databases, both would share the same file
}

/// Integration test demonstrating comprehensive driver usage
#[test]
fn test_comprehensive_driver_usage() {
    let config = create_test_config();
    let conn = ProductionSqliteConnection::new(config).unwrap();
    
    // Create comprehensive schema
    conn.execute(
        "CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            last_login TEXT
        )",
        &[]
    ).unwrap();
    
    conn.execute(
        "CREATE TABLE posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            content TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
        &[]
    ).unwrap();
    
    // Insert test data with transaction
    let tx = conn.begin_transaction(TxOptions::default()).unwrap();
    
    // Insert users
    let user_stmt = tx.prepare(
        "INSERT INTO users (username, email) VALUES (?, ?) RETURNING id"
    ).unwrap();
    
    let users = vec![
        ("alice", "alice@example.com"),
        ("bob", "bob@example.com"),
        ("charlie", "charlie@example.com"),
    ];
    
    let mut user_ids = Vec::new();
    for (username, email) in users {
        let result = user_stmt.query(&[
            SqlValue::String(username.to_string()),
            SqlValue::String(email.to_string())
        ]).unwrap();
        
        if !result.rows.is_empty() {
            if let SqlValue::Integer(id) = &result.rows[0][0] {
                user_ids.push(*id);
            }
        }
    }
    
    // Insert posts
    let post_stmt = tx.prepare(
        "INSERT INTO posts (user_id, title, content) VALUES (?, ?, ?)"
    ).unwrap();
    
    for user_id in &user_ids {
        for i in 1..=3 {
            post_stmt.execute(&[
                SqlValue::Integer(*user_id),
                SqlValue::String(format!("Post {} by user {}", i, user_id)),
                SqlValue::String(format!("Content of post {} by user {}", i, user_id))
            ]).unwrap();
        }
    }
    
    tx.commit().unwrap();
    
    // Complex query with JOIN
    let result = conn.query(
        "SELECT u.username, COUNT(p.id) as post_count 
         FROM users u 
         LEFT JOIN posts p ON u.id = p.user_id 
         GROUP BY u.id, u.username 
         ORDER BY post_count DESC",
        &[]
    ).unwrap();
    
    assert_eq!(result.rows.len(), 3);
    assert_eq!(result.column_names, vec!["username", "post_count"]);
    
    // Verify data
    for row in &result.rows {
        match (&row[0], &row[1]) {
            (SqlValue::String(_username), SqlValue::Integer(count)) => {
                assert_eq!(*count, 3); // Each user has 3 posts
            }
            _ => panic!("Expected string and integer"),
        }
    }
    
    // Test statistics
    let stats = conn.get_stats().unwrap();
    assert!(stats.queries_executed > 0);
    assert!(stats.statements_prepared > 0);
    assert!(stats.transactions_committed > 0);
    
    println!("Final statistics: {:?}", stats);
}
