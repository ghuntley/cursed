
use cursed::stdlib::database::{Database, Connection, QueryResult};
use cursed::stdlib::database::drivers::{SqliteDriver, PostgresDriver};
use cursed::error::CursedError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_connection() {
        let driver = SqliteDriver::new();
        let result = driver.connect(":memory:");
        
        // Should be able to connect to in-memory SQLite
        assert!(result.is_ok());
    }

    #[test]
    fn test_sqlite_query_execution() {
        let driver = SqliteDriver::new();
        if let Ok(mut connection) = driver.connect(":memory:") {
            // Create a test table
            let create_sql = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)";
            let result = connection.execute(create_sql, &[]);
            
            assert!(result.is_ok());
            
            // Insert test data
            let insert_sql = "INSERT INTO users (name) VALUES (?)";
            let result = connection.execute(insert_sql, &["Alice".into()]);
            
            assert!(result.is_ok());
            
            // Query the data
            let select_sql = "SELECT * FROM users";
            let result = connection.query(select_sql, &[]);
            
            assert!(result.is_ok());
            if let Ok(query_result) = result {
                assert!(query_result.rows.len() > 0);
            }
        }
    }

    #[test]
    fn test_sqlite_prepared_statements() {
        let driver = SqliteDriver::new();
        if let Ok(mut connection) = driver.connect(":memory:") {
            // Create table
            let _ = connection.execute("CREATE TABLE items (id INTEGER, value TEXT)", &[]);
            
            // Prepare statement
            let sql = "INSERT INTO items (id, value) VALUES (?, ?)";
            let result = connection.prepare(sql);
            
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_sqlite_transactions() {
        let driver = SqliteDriver::new();
        if let Ok(mut connection) = driver.connect(":memory:") {
            // Create table
            let _ = connection.execute("CREATE TABLE accounts (id INTEGER, balance REAL)", &[]);
            
            // Begin transaction
            let result = connection.begin_transaction();
            assert!(result.is_ok());
            
            // Insert data
            let _ = connection.execute("INSERT INTO accounts VALUES (1, 100.0)", &[]);
            let _ = connection.execute("INSERT INTO accounts VALUES (2, 200.0)", &[]);
            
            // Commit transaction
            let result = connection.commit();
            assert!(result.is_ok());
        }
    }

    #[test] 
    fn test_database_error_handling() {
        let driver = SqliteDriver::new();
        if let Ok(mut connection) = driver.connect(":memory:") {
            // Try to execute invalid SQL
            let result = connection.execute("INVALID SQL STATEMENT", &[]);
            
            // Should return an error
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_multiple_connections() {
        let driver = SqliteDriver::new();
        
        let conn1 = driver.connect(":memory:");
        let conn2 = driver.connect(":memory:");
        
        // Both connections should succeed
        assert!(conn1.is_ok());
        assert!(conn2.is_ok());
    }

    #[test]
    fn test_database_factory() {
        // Test creating different database types
        let sqlite_db = Database::new("sqlite", ":memory:");
        assert!(sqlite_db.is_ok());
        
        // PostgreSQL would require actual server (skip if not available)
        let postgres_result = Database::new("postgres", "postgresql://localhost/test");
        // Don't assert on this since server may not be available
        let _ = postgres_result;
    }

    #[test]
    fn test_query_parameters() {
        let driver = SqliteDriver::new();
        if let Ok(mut connection) = driver.connect(":memory:") {
            let _ = connection.execute("CREATE TABLE params_test (id INTEGER, name TEXT, age INTEGER)", &[]);
            
            // Test parameter binding
            let sql = "INSERT INTO params_test VALUES (?, ?, ?)";
            let params = vec![
                1.into(),
                "John".to_string().into(),
                30.into()
            ];
            
            let result = connection.execute(sql, &params);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_query_result_parsing() {
        let driver = SqliteDriver::new();
        if let Ok(mut connection) = driver.connect(":memory:") {
            let _ = connection.execute("CREATE TABLE data (id INTEGER, name TEXT)", &[]);
            let _ = connection.execute("INSERT INTO data VALUES (1, 'Test')", &[]);
            
            let result = connection.query("SELECT * FROM data", &[]);
            
            if let Ok(query_result) = result {
                assert_eq!(query_result.rows.len(), 1);
                assert!(query_result.columns.len() >= 2);
            }
        }
    }

    #[test]
    fn test_connection_pooling() {
        // Test that multiple connections can be managed
        let driver = SqliteDriver::new();
        let mut connections = Vec::new();
        
        for _ in 0..5 {
            if let Ok(conn) = driver.connect(":memory:") {
                connections.push(conn);
            }
        }
        
        assert_eq!(connections.len(), 5);
    }

    #[test]
    fn test_database_metadata() {
        let driver = SqliteDriver::new();
        if let Ok(mut connection) = driver.connect(":memory:") {
            let _ = connection.execute("CREATE TABLE meta_test (id INTEGER PRIMARY KEY, name TEXT NOT NULL)", &[]);
            
            // Test getting table information (if implemented)
            let result = connection.query("SELECT name FROM sqlite_master WHERE type='table'", &[]);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_concurrent_database_access() {
        use std::thread;
        use std::sync::Arc;
        
        let driver = Arc::new(SqliteDriver::new());
        let mut handles = vec![];
        
        for i in 0..3 {
            let driver_clone = Arc::clone(&driver);
            let handle = thread::spawn(move || {
                if let Ok(mut conn) = driver_clone.connect(":memory:") {
                    let table_name = format!("test_table_{}", i);
                    let sql = format!("CREATE TABLE {} (id INTEGER)", table_name);
                    conn.execute(&sql, &[])
                } else {
                    Err(CursedError::Database("Failed to connect".to_string()))
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_large_query_results() {
        let driver = SqliteDriver::new();
        if let Ok(mut connection) = driver.connect(":memory:") {
            let _ = connection.execute("CREATE TABLE large_test (id INTEGER, data TEXT)", &[]);
            
            // Insert multiple rows
            for i in 0..100 {
                let sql = "INSERT INTO large_test VALUES (?, ?)";
                let params = vec![i.into(), format!("data_{}", i).into()];
                let _ = connection.execute(sql, &params);
            }
            
            // Query all rows
            let result = connection.query("SELECT COUNT(*) FROM large_test", &[]);
            assert!(result.is_ok());
        }
    }
}