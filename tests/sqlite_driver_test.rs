/// fr fr Comprehensive SQLite driver tests - making sure the database vibes work periodt

use cursed::stdlib::packages::{
    db_core::{
        ConnectionConfig, DatabaseDriver, DatabaseConnection, 
        Parameter, ParameterDirection, ExecuteResult,
        utils
    },
    db_sql::{
        sqlite::{SqliteDriver, SqliteConnection},
        SqlDriver, SqlValue
    }
};
use std::fs;
use tempfile::TempDir;

/// slay Set up test database with sample data
async fn setup_test_db() -> (TempDir, String) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    let db_path_str = db_path.to_str().expect("Invalid path").to_string();
    
    // Create connection and set up tables
    let mut conn = SqliteConnection::new(&db_path_str).expect("Failed to create connection");
    
    // Create test table
    let create_table_sql = r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE,
            age INTEGER,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    "#;
    
    let _ = conn.execute(create_table_sql, &[]).await.expect("Failed to create table");
    
    // Insert test data
    let insert_sql = "INSERT INTO users (name, email, age) VALUES (?, ?, ?)";
    let test_users = [
        ("Alice Johnson", "alice@example.com", "25"),
        ("Bob Smith", "bob@example.com", "30"),
        ("Carol Brown", "carol@example.com", "28"),
    ];
    
    for (name, email, age) in test_users.iter() {
        let params = [
            Parameter::input(name),
            Parameter::input(email),
            Parameter::input(age),
        ];
        let _ = conn.execute(insert_sql, &params).await.expect("Failed to insert test data");
    }
    
    (temp_dir, db_path_str)
}

#[tokio::test]
async fn test_sqlite_driver_basic_operations() {
    let (_temp_dir, db_path) = setup_test_db().await;
    let driver = SqliteDriver::new();
    
    // Test driver info
    let info = driver.driver_info();
    assert_eq!(info.name, "sqlite");
    assert_eq!(info.version, "1.0.0");
    
    // Test connection
    let config = ConnectionConfig::from_string(&format!("sqlite://{}", db_path)).unwrap();
    let mut conn = driver.connect(config).await.unwrap();
    
    // Test ping
    conn.ping().await.unwrap();
    
    println!("✅ SQLite driver basic operations test passed");
}

#[tokio::test]
async fn test_sqlite_query_operations() {
    let (_temp_dir, db_path) = setup_test_db().await;
    let mut conn = SqliteConnection::new(&db_path).unwrap();
    
    // Test simple query
    let mut result = conn.query("SELECT COUNT(*) FROM users", &[]).await.unwrap();
    
    // Check result
    let first_row = result.next().await.unwrap();
    assert!(first_row.is_some());
    
    // Test parameterized query
    let params = [Parameter::input("Alice Johnson")];
    let mut result = conn.query(
        "SELECT name, email FROM users WHERE name = ?", 
        &params
    ).await.unwrap();
    
    let row = result.next().await.unwrap();
    assert!(row.is_some());
    
    println!("✅ SQLite query operations test passed");
}

#[tokio::test]
async fn test_sqlite_execute_operations() {
    let (_temp_dir, db_path) = setup_test_db().await;
    let mut conn = SqliteConnection::new(&db_path).unwrap();
    
    // Test INSERT
    let params = [
        Parameter::input("David Wilson"),
        Parameter::input("david@example.com"),
        Parameter::input("35"),
    ];
    let result = conn.execute(
        "INSERT INTO users (name, email, age) VALUES (?, ?, ?)", 
        &params
    ).await.unwrap();
    
    assert!(result.affected_rows > 0);
    assert!(result.last_insert_id.is_some());
    
    // Test UPDATE
    let update_params = [
        Parameter::input("36"),
        Parameter::input("David Wilson"),
    ];
    let result = conn.execute(
        "UPDATE users SET age = ? WHERE name = ?", 
        &update_params
    ).await.unwrap();
    
    assert!(result.affected_rows > 0);
    
    // Test DELETE
    let delete_params = [Parameter::input("David Wilson")];
    let result = conn.execute(
        "DELETE FROM users WHERE name = ?", 
        &delete_params
    ).await.unwrap();
    
    assert!(result.affected_rows > 0);
    
    println!("✅ SQLite execute operations test passed");
}

#[tokio::test]
async fn test_sqlite_prepared_statements() {
    let (_temp_dir, db_path) = setup_test_db().await;
    let mut conn = SqliteConnection::new(&db_path).unwrap();
    
    // Test prepared statement for queries
    let mut stmt = conn.prepare("SELECT name, age FROM users WHERE age > ?").await.unwrap();
    
    let params = [Parameter::input("25")];
    let mut result = stmt.query(&params).await.unwrap();
    
    let mut count = 0;
    while (result.next().await.unwrap()).is_some() {
        count += 1;
    }
    assert!(count > 0);
    
    // Test prepared statement for execute
    let mut insert_stmt = conn.prepare("INSERT INTO users (name, email, age) VALUES (?, ?, ?)").await.unwrap();
    
    let insert_params = [
        Parameter::input("Emma Davis"),
        Parameter::input("emma@example.com"),
        Parameter::input("27"),
    ];
    let result = insert_stmt.execute(&insert_params).await.unwrap();
    
    assert!(result.affected_rows > 0);
    assert!(result.last_insert_id.is_some());
    
    println!("✅ SQLite prepared statements test passed");
}

#[tokio::test]
async fn test_sqlite_transactions() {
    let (_temp_dir, db_path) = setup_test_db().await;
    let mut conn = SqliteConnection::new(&db_path).unwrap();
    
    // Test successful transaction
    {
        let mut tx = conn.begin_transaction(None).await.unwrap();
        
        let params = [
            Parameter::input("Transaction User"),
            Parameter::input("tx@example.com"),
            Parameter::input("40"),
        ];
        let result = tx.execute(
            "INSERT INTO users (name, email, age) VALUES (?, ?, ?)", 
            &params
        ).await.unwrap();
        
        assert!(result.affected_rows > 0);
        
        // Commit transaction
        tx.commit().await.unwrap();
    }
    
    // Verify data was committed
    let mut result = conn.query("SELECT name FROM users WHERE name = 'Transaction User'", &[]).await.unwrap();
    let row = result.next().await.unwrap();
    assert!(row.is_some());
    
    // Test rollback transaction
    {
        let mut tx = conn.begin_transaction(None).await.unwrap();
        
        let params = [
            Parameter::input("Rollback User"),
            Parameter::input("rollback@example.com"),
            Parameter::input("45"),
        ];
        let _result = tx.execute(
            "INSERT INTO users (name, email, age) VALUES (?, ?, ?)", 
            &params
        ).await.unwrap();
        
        // Rollback transaction
        tx.rollback().await.unwrap();
    }
    
    // Verify data was rolled back
    let mut result = conn.query("SELECT name FROM users WHERE name = 'Rollback User'", &[]).await.unwrap();
    let row = result.next().await.unwrap();
    assert!(row.is_none());
    
    println!("✅ SQLite transactions test passed");
}

#[tokio::test]
async fn test_sqlite_savepoints() {
    let (_temp_dir, db_path) = setup_test_db().await;
    let mut conn = SqliteConnection::new(&db_path).unwrap();
    
    let mut tx = conn.begin_transaction(None).await.unwrap();
    
    // Create savepoint
    let savepoint = tx.savepoint("test_savepoint").await.unwrap();
    
    // Insert data after savepoint
    let params = [
        Parameter::input("Savepoint User"),
        Parameter::input("savepoint@example.com"),
        Parameter::input("50"),
    ];
    let _result = tx.execute(
        "INSERT INTO users (name, email, age) VALUES (?, ?, ?)", 
        &params
    ).await.unwrap();
    
    // Rollback to savepoint
    tx.rollback_to_savepoint(&savepoint).await.unwrap();
    
    // Commit transaction (should not include the rolled back data)
    tx.commit().await.unwrap();
    
    // Verify data was rolled back
    let mut result = conn.query("SELECT name FROM users WHERE name = 'Savepoint User'", &[]).await.unwrap();
    let row = result.next().await.unwrap();
    assert!(row.is_none());
    
    println!("✅ SQLite savepoints test passed");
}

#[tokio::test]
async fn test_sqlite_error_handling() {
    let (_temp_dir, db_path) = setup_test_db().await;
    let mut conn = SqliteConnection::new(&db_path).unwrap();
    
    // Test syntax error
    let result = conn.query("SELECT * FROM nonexistent_table", &[]).await;
    assert!(result.is_err());
    
    // Test constraint violation (duplicate email)
    let params = [
        Parameter::input("Duplicate User"),
        Parameter::input("alice@example.com"), // This email already exists
        Parameter::input("25"),
    ];
    let result = conn.execute(
        "INSERT INTO users (name, email, age) VALUES (?, ?, ?)", 
        &params
    ).await;
    assert!(result.is_err());
    
    println!("✅ SQLite error handling test passed");
}

#[tokio::test]
async fn test_sqlite_data_types() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("types_test.db");
    let db_path_str = db_path.to_str().expect("Invalid path").to_string();
    
    let mut conn = SqliteConnection::new(&db_path_str).unwrap();
    
    // Create table with various data types
    let create_sql = r#"
        CREATE TABLE type_test (
            id INTEGER PRIMARY KEY,
            text_field TEXT,
            integer_field INTEGER,
            real_field REAL,
            blob_field BLOB,
            null_field TEXT
        )
    "#;
    
    conn.execute(create_sql, &[]).await.unwrap();
    
    // Insert data with different types
    let params = [
        Parameter::input("1"),
        Parameter::input("Hello World"),
        Parameter::input("42"),
        Parameter::input("3.14159"),
        Parameter::input("binary_data"), // Will be stored as text in SQLite
    ];
    
    let result = conn.execute(
        "INSERT INTO type_test (id, text_field, integer_field, real_field, blob_field) VALUES (?, ?, ?, ?, ?)", 
        &params
    ).await.unwrap();
    
    assert!(result.affected_rows > 0);
    
    // Query back the data
    let mut result = conn.query("SELECT * FROM type_test WHERE id = 1", &[]).await.unwrap();
    let row = result.next().await.unwrap();
    assert!(row.is_some());
    
    let row_data = row.unwrap();
    assert_eq!(row_data.values.len(), 6); // 6 columns
    
    println!("✅ SQLite data types test passed");
}

#[tokio::test]
async fn test_sqlite_concurrent_access() {
    let (_temp_dir, db_path) = setup_test_db().await;
    
    // Test multiple connections to the same database
    let mut conn1 = SqliteConnection::new(&db_path).unwrap();
    let mut conn2 = SqliteConnection::new(&db_path).unwrap();
    
    // Both connections should work
    conn1.ping().await.unwrap();
    conn2.ping().await.unwrap();
    
    // Both should be able to query
    let mut result1 = conn1.query("SELECT COUNT(*) FROM users", &[]).await.unwrap();
    let mut result2 = conn2.query("SELECT COUNT(*) FROM users", &[]).await.unwrap();
    
    assert!(result1.next().await.unwrap().is_some());
    assert!(result2.next().await.unwrap().is_some());
    
    println!("✅ SQLite concurrent access test passed");
}

#[tokio::test] 
async fn test_sqlite_in_memory_database() {
    let mut conn = SqliteConnection::new_in_memory().unwrap();
    
    // Create table in memory
    let create_sql = r#"
        CREATE TABLE memory_test (
            id INTEGER PRIMARY KEY,
            data TEXT
        )
    "#;
    
    conn.execute(create_sql, &[]).await.unwrap();
    
    // Insert and query data
    let params = [Parameter::input("1"), Parameter::input("In Memory Data")];
    let result = conn.execute("INSERT INTO memory_test (id, data) VALUES (?, ?)", &params).await.unwrap();
    assert!(result.affected_rows > 0);
    
    let mut result = conn.query("SELECT data FROM memory_test WHERE id = 1", &[]).await.unwrap();
    let row = result.next().await.unwrap();
    assert!(row.is_some());
    
    println!("✅ SQLite in-memory database test passed");
}
