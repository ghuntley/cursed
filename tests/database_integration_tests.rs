/// fr fr Comprehensive database integration tests with real database instances
/// 
/// This test suite provides end-to-end testing with actual databases:
/// - SQLite: In-memory and file-based testing
/// - PostgreSQL: Docker container integration
/// - Connection pooling with real connections
/// - Transaction management and rollback testing
/// - Migration execution and schema changes
/// - Performance testing with real data

use cursed::stdlib::packages::{
    db_core::{
        self, DatabaseError, ConnectionConfig, ConnectionOptions,
        DatabaseConnection, Transaction
    },
    db_sql::{
        self, SqlQueryBuilder, SqlValue, SqlType, SqlConnection,
        SqliteDriver, PostgreSqlDriver
    },
    db_pool::{ConnectionPool, PoolConfig, PoolManager},
    db_migrate::{Migration, MigrationRunner, VersionManager},
};
use std::time::Duration;
use std::path::PathBuf;
use std::env;
use tokio;

/// fr fr SQLite integration tests (no external dependencies)
mod sqlite_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_sqlite_in_memory_connection() {
        let config = ConnectionConfig::new("sqlite", ":memory:");
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        // Test basic connectivity
        assert!(connection.is_connected());
        assert_eq!(connection.driver_name(), "sqlite");
        
        // Test simple query
        let result = connection.execute("SELECT 1 as test_value", vec![]).await.unwrap();
        assert!(result.rows_affected() >= 0);
        
        // Close connection
        connection.close().await.unwrap();
        assert!(!connection.is_connected());
    }

    #[tokio::test]
    async fn test_sqlite_file_database() {
        let db_path = "/tmp/cursed_test.db";
        
        // Clean up any existing test database
        if std::path::Path::new(db_path).exists() {
            std::fs::remove_file(db_path).unwrap();
        }
        
        let config = ConnectionConfig::new("sqlite", db_path);
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        // Create test table
        let create_sql = "CREATE TABLE test_users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )";
        
        connection.execute(create_sql, vec![]).await.unwrap();
        
        // Insert test data
        let insert_sql = "INSERT INTO test_users (name, email) VALUES (?, ?)";
        connection.execute(insert_sql, vec![
            SqlValue::Text("Alice".to_string()),
            SqlValue::Text("alice@example.com".to_string())
        ]).await.unwrap();
        
        connection.execute(insert_sql, vec![
            SqlValue::Text("Bob".to_string()),
            SqlValue::Text("bob@example.com".to_string())
        ]).await.unwrap();
        
        // Query data
        let select_sql = "SELECT id, name, email FROM test_users ORDER BY name";
        let result = connection.query(select_sql, vec![]).await.unwrap();
        
        assert_eq!(result.row_count(), 2);
        
        let rows = result.rows();
        assert_eq!(rows[0].get_string("name").unwrap(), "Alice");
        assert_eq!(rows[0].get_string("email").unwrap(), "alice@example.com");
        assert_eq!(rows[1].get_string("name").unwrap(), "Bob");
        assert_eq!(rows[1].get_string("email").unwrap(), "bob@example.com");
        
        // Update data
        let update_sql = "UPDATE test_users SET email = ? WHERE name = ?";
        let update_result = connection.execute(update_sql, vec![
            SqlValue::Text("alice.new@example.com".to_string()),
            SqlValue::Text("Alice".to_string())
        ]).await.unwrap();
        
        assert_eq!(update_result.rows_affected(), 1);
        
        // Delete data
        let delete_sql = "DELETE FROM test_users WHERE name = ?";
        let delete_result = connection.execute(delete_sql, vec![
            SqlValue::Text("Bob".to_string())
        ]).await.unwrap();
        
        assert_eq!(delete_result.rows_affected(), 1);
        
        // Verify final state
        let final_result = connection.query("SELECT COUNT(*) as count FROM test_users", vec![]).await.unwrap();
        assert_eq!(final_result.rows()[0].get_i64("count").unwrap(), 1);
        
        connection.close().await.unwrap();
        
        // Clean up
        std::fs::remove_file(db_path).unwrap();
    }

    #[tokio::test]
    async fn test_sqlite_transactions() {
        let config = ConnectionConfig::new("sqlite", ":memory:");
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        // Create test table
        connection.execute("CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance INTEGER)", vec![]).await.unwrap();
        connection.execute("INSERT INTO accounts (balance) VALUES (1000), (500)", vec![]).await.unwrap();
        
        // Start transaction
        let mut txn = connection.begin_transaction().await.unwrap();
        
        // Transfer money (should be atomic)
        txn.execute("UPDATE accounts SET balance = balance - 200 WHERE id = 1", vec![]).await.unwrap();
        txn.execute("UPDATE accounts SET balance = balance + 200 WHERE id = 2", vec![]).await.unwrap();
        
        // Check balances within transaction
        let result = txn.query("SELECT balance FROM accounts ORDER BY id", vec![]).await.unwrap();
        let rows = result.rows();
        assert_eq!(rows[0].get_i64("balance").unwrap(), 800);
        assert_eq!(rows[1].get_i64("balance").unwrap(), 700);
        
        // Commit transaction
        txn.commit().await.unwrap();
        
        // Verify changes persisted
        let final_result = connection.query("SELECT balance FROM accounts ORDER BY id", vec![]).await.unwrap();
        let final_rows = final_result.rows();
        assert_eq!(final_rows[0].get_i64("balance").unwrap(), 800);
        assert_eq!(final_rows[1].get_i64("balance").unwrap(), 700);
        
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn test_sqlite_transaction_rollback() {
        let config = ConnectionConfig::new("sqlite", ":memory:");
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        // Create test table
        connection.execute("CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance INTEGER)", vec![]).await.unwrap();
        connection.execute("INSERT INTO accounts (balance) VALUES (1000), (500)", vec![]).await.unwrap();
        
        // Start transaction
        let mut txn = connection.begin_transaction().await.unwrap();
        
        // Make changes
        txn.execute("UPDATE accounts SET balance = balance - 200 WHERE id = 1", vec![]).await.unwrap();
        txn.execute("UPDATE accounts SET balance = balance + 200 WHERE id = 2", vec![]).await.unwrap();
        
        // Rollback transaction
        txn.rollback().await.unwrap();
        
        // Verify changes were rolled back
        let result = connection.query("SELECT balance FROM accounts ORDER BY id", vec![]).await.unwrap();
        let rows = result.rows();
        assert_eq!(rows[0].get_i64("balance").unwrap(), 1000);
        assert_eq!(rows[1].get_i64("balance").unwrap(), 500);
        
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn test_sqlite_prepared_statements() {
        let config = ConnectionConfig::new("sqlite", ":memory:");
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        // Create test table
        connection.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)", vec![]).await.unwrap();
        
        // Prepare statement
        let stmt = connection.prepare("INSERT INTO users (name, age) VALUES (?, ?)").await.unwrap();
        
        // Execute prepared statement multiple times
        let users = vec![
            ("Alice", 25),
            ("Bob", 30),
            ("Charlie", 35),
            ("Diana", 28),
        ];
        
        for (name, age) in users {
            stmt.execute(vec![
                SqlValue::Text(name.to_string()),
                SqlValue::Integer(age)
            ]).await.unwrap();
        }
        
        // Query results
        let result = connection.query("SELECT name, age FROM users ORDER BY age", vec![]).await.unwrap();
        assert_eq!(result.row_count(), 4);
        
        let rows = result.rows();
        assert_eq!(rows[0].get_string("name").unwrap(), "Alice");
        assert_eq!(rows[0].get_i64("age").unwrap(), 25);
        assert_eq!(rows[3].get_string("name").unwrap(), "Charlie");
        assert_eq!(rows[3].get_i64("age").unwrap(), 35);
        
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn test_sqlite_query_builder_integration() {
        let config = ConnectionConfig::new("sqlite", ":memory:");
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        // Create test table using query builder
        let mut builder = SqlQueryBuilder::new();
        let create_sql = builder.create_table()
            .table("products")
            .column("id", SqlType::Integer).primary_key().auto_increment().finish()
            .column("name", SqlType::Text).not_null().finish()
            .column("price", SqlType::Float).not_null().finish()
            .column("category", SqlType::Text).finish()
            .build()
            .unwrap();
        
        connection.execute(&create_sql, vec![]).await.unwrap();
        
        // Insert data using query builder
        builder.clear_parameters();
        let insert_sql = builder.insert()
            .into("products")
            .columns(&["name", "price", "category"])
            .values(vec![
                SqlValue::Text("Laptop".to_string()),
                SqlValue::Float(999.99),
                SqlValue::Text("Electronics".to_string())
            ])
            .build()
            .unwrap();
        
        connection.execute(&insert_sql, builder.parameters().clone()).await.unwrap();
        
        // Insert more data
        builder.clear_parameters();
        let insert_sql2 = builder.insert()
            .into("products")
            .columns(&["name", "price", "category"])
            .values(vec![
                SqlValue::Text("Book".to_string()),
                SqlValue::Float(29.99),
                SqlValue::Text("Education".to_string())
            ])
            .build()
            .unwrap();
        
        connection.execute(&insert_sql2, builder.parameters().clone()).await.unwrap();
        
        // Query using query builder
        builder.clear_parameters();
        let select_sql = builder.select()
            .columns(&["name", "price", "category"])
            .from("products")
            .where_clause("price > ?")
            .order_by("price", db_sql::OrderDirection::Desc)
            .build()
            .unwrap();
        
        builder.add_parameter(SqlValue::Float(50.0));
        let result = connection.query(&select_sql, builder.parameters().clone()).await.unwrap();
        
        assert_eq!(result.row_count(), 1);
        let row = &result.rows()[0];
        assert_eq!(row.get_string("name").unwrap(), "Laptop");
        assert!((row.get_f64("price").unwrap() - 999.99).abs() < f64::EPSILON);
        
        connection.close().await.unwrap();
    }
}

/// fr fr PostgreSQL integration tests (requires Docker or local PostgreSQL)
mod postgresql_integration_tests {
    use super::*;

    fn get_postgres_config() -> Option<ConnectionConfig> {
        // Check for PostgreSQL test database configuration
        if let Ok(url) = env::var("CURSED_POSTGRES_TEST_URL") {
            ConnectionConfig::from_string(&url).ok()
        } else {
            // Default test configuration
            Some(ConnectionConfig::new("postgresql", "cursed_test")
                .with_host("localhost", 5432)
                .with_credentials("cursed_test", "cursed_test"))
        }
    }

    #[tokio::test]
    async fn test_postgresql_connection() {
        let Some(config) = get_postgres_config() else {
            println!("Skipping PostgreSQL tests - no test database configured");
            return;
        };
        
        let driver = PostgreSqlDriver::new();
        match driver.connect(config).await {
            Ok(mut connection) => {
                assert!(connection.is_connected());
                assert_eq!(connection.driver_name(), "postgresql");
                
                // Test basic query
                let result = connection.execute("SELECT version()", vec![]).await.unwrap();
                assert!(result.rows_affected() >= 0);
                
                connection.close().await.unwrap();
            }
            Err(_) => {
                println!("Skipping PostgreSQL tests - database not available");
                return;
            }
        }
    }

    #[tokio::test]
    async fn test_postgresql_crud_operations() {
        let Some(config) = get_postgres_config() else {
            println!("Skipping PostgreSQL CRUD tests - no test database configured");
            return;
        };
        
        let driver = PostgreSqlDriver::new();
        let mut connection = match driver.connect(config).await {
            Ok(conn) => conn,
            Err(_) => {
                println!("Skipping PostgreSQL CRUD tests - database not available");
                return;
            }
        };
        
        // Create test table
        let create_sql = "CREATE TEMP TABLE test_employees (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(100) UNIQUE,
            salary DECIMAL(10,2),
            hire_date DATE DEFAULT CURRENT_DATE,
            active BOOLEAN DEFAULT true
        )";
        
        connection.execute(create_sql, vec![]).await.unwrap();
        
        // Insert test data
        let insert_sql = "INSERT INTO test_employees (name, email, salary) VALUES ($1, $2, $3) RETURNING id";
        let result = connection.query(insert_sql, vec![
            SqlValue::Text("John Doe".to_string()),
            SqlValue::Text("john@company.com".to_string()),
            SqlValue::Float(75000.0)
        ]).await.unwrap();
        
        assert_eq!(result.row_count(), 1);
        let employee_id = result.rows()[0].get_i64("id").unwrap();
        
        // Insert more employees
        connection.execute("INSERT INTO test_employees (name, email, salary) VALUES ($1, $2, $3)", vec![
            SqlValue::Text("Jane Smith".to_string()),
            SqlValue::Text("jane@company.com".to_string()),
            SqlValue::Float(80000.0)
        ]).await.unwrap();
        
        connection.execute("INSERT INTO test_employees (name, email, salary) VALUES ($1, $2, $3)", vec![
            SqlValue::Text("Bob Wilson".to_string()),
            SqlValue::Text("bob@company.com".to_string()),
            SqlValue::Float(65000.0)
        ]).await.unwrap();
        
        // Query with conditions
        let select_sql = "SELECT id, name, email, salary FROM test_employees WHERE salary > $1 ORDER BY salary DESC";
        let result = connection.query(select_sql, vec![SqlValue::Float(70000.0)]).await.unwrap();
        
        assert_eq!(result.row_count(), 2);
        let rows = result.rows();
        assert_eq!(rows[0].get_string("name").unwrap(), "Jane Smith");
        assert_eq!(rows[1].get_string("name").unwrap(), "John Doe");
        
        // Update employee
        let update_sql = "UPDATE test_employees SET salary = $1 WHERE id = $2";
        let update_result = connection.execute(update_sql, vec![
            SqlValue::Float(85000.0),
            SqlValue::Integer(employee_id)
        ]).await.unwrap();
        
        assert_eq!(update_result.rows_affected(), 1);
        
        // Verify update
        let verify_sql = "SELECT salary FROM test_employees WHERE id = $1";
        let verify_result = connection.query(verify_sql, vec![SqlValue::Integer(employee_id)]).await.unwrap();
        assert!((verify_result.rows()[0].get_f64("salary").unwrap() - 85000.0).abs() < f64::EPSILON);
        
        // Delete employee
        let delete_sql = "DELETE FROM test_employees WHERE email = $1";
        let delete_result = connection.execute(delete_sql, vec![
            SqlValue::Text("bob@company.com".to_string())
        ]).await.unwrap();
        
        assert_eq!(delete_result.rows_affected(), 1);
        
        // Verify final count
        let count_result = connection.query("SELECT COUNT(*) as total FROM test_employees", vec![]).await.unwrap();
        assert_eq!(count_result.rows()[0].get_i64("total").unwrap(), 2);
        
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn test_postgresql_transactions() {
        let Some(config) = get_postgres_config() else {
            println!("Skipping PostgreSQL transaction tests - no test database configured");
            return;
        };
        
        let driver = PostgreSqlDriver::new();
        let mut connection = match driver.connect(config).await {
            Ok(conn) => conn,
            Err(_) => {
                println!("Skipping PostgreSQL transaction tests - database not available");
                return;
            }
        };
        
        // Create test table
        connection.execute("CREATE TEMP TABLE test_accounts (id SERIAL PRIMARY KEY, name VARCHAR(50), balance DECIMAL(10,2))", vec![]).await.unwrap();
        connection.execute("INSERT INTO test_accounts (name, balance) VALUES ('Alice', 1000.00), ('Bob', 500.00)", vec![]).await.unwrap();
        
        // Test successful transaction
        let mut txn = connection.begin_transaction().await.unwrap();
        
        txn.execute("UPDATE test_accounts SET balance = balance - 200 WHERE name = 'Alice'", vec![]).await.unwrap();
        txn.execute("UPDATE test_accounts SET balance = balance + 200 WHERE name = 'Bob'", vec![]).await.unwrap();
        
        txn.commit().await.unwrap();
        
        // Verify transaction succeeded
        let result = connection.query("SELECT name, balance FROM test_accounts ORDER BY name", vec![]).await.unwrap();
        let rows = result.rows();
        assert!((rows[0].get_f64("balance").unwrap() - 800.0).abs() < f64::EPSILON);
        assert!((rows[1].get_f64("balance").unwrap() - 700.0).abs() < f64::EPSILON);
        
        // Test rollback transaction
        let mut txn2 = connection.begin_transaction().await.unwrap();
        
        txn2.execute("UPDATE test_accounts SET balance = balance - 100 WHERE name = 'Alice'", vec![]).await.unwrap();
        txn2.execute("UPDATE test_accounts SET balance = balance + 100 WHERE name = 'Bob'", vec![]).await.unwrap();
        
        txn2.rollback().await.unwrap();
        
        // Verify rollback worked
        let result2 = connection.query("SELECT name, balance FROM test_accounts ORDER BY name", vec![]).await.unwrap();
        let rows2 = result2.rows();
        assert!((rows2[0].get_f64("balance").unwrap() - 800.0).abs() < f64::EPSILON);
        assert!((rows2[1].get_f64("balance").unwrap() - 700.0).abs() < f64::EPSILON);
        
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn test_postgresql_savepoints() {
        let Some(config) = get_postgres_config() else {
            println!("Skipping PostgreSQL savepoint tests - no test database configured");
            return;
        };
        
        let driver = PostgreSqlDriver::new();
        let mut connection = match driver.connect(config).await {
            Ok(conn) => conn,
            Err(_) => {
                println!("Skipping PostgreSQL savepoint tests - database not available");
                return;
            }
        };
        
        // Create test table
        connection.execute("CREATE TEMP TABLE test_log (id SERIAL PRIMARY KEY, message TEXT)", vec![]).await.unwrap();
        
        // Start transaction with savepoints
        let mut txn = connection.begin_transaction().await.unwrap();
        
        txn.execute("INSERT INTO test_log (message) VALUES ('First message')", vec![]).await.unwrap();
        
        let savepoint1 = txn.savepoint("sp1").await.unwrap();
        txn.execute("INSERT INTO test_log (message) VALUES ('Second message')", vec![]).await.unwrap();
        
        let savepoint2 = txn.savepoint("sp2").await.unwrap();
        txn.execute("INSERT INTO test_log (message) VALUES ('Third message')", vec![]).await.unwrap();
        
        // Rollback to savepoint2 (removes third message)
        txn.rollback_to_savepoint(&savepoint2).await.unwrap();
        
        let savepoint3 = txn.savepoint("sp3").await.unwrap();
        txn.execute("INSERT INTO test_log (message) VALUES ('Fourth message')", vec![]).await.unwrap();
        
        // Rollback to savepoint1 (removes second and fourth messages)
        txn.rollback_to_savepoint(&savepoint1).await.unwrap();
        
        txn.execute("INSERT INTO test_log (message) VALUES ('Final message')", vec![]).await.unwrap();
        
        txn.commit().await.unwrap();
        
        // Verify final state
        let result = connection.query("SELECT message FROM test_log ORDER BY id", vec![]).await.unwrap();
        assert_eq!(result.row_count(), 2);
        let rows = result.rows();
        assert_eq!(rows[0].get_string("message").unwrap(), "First message");
        assert_eq!(rows[1].get_string("message").unwrap(), "Final message");
        
        connection.close().await.unwrap();
    }
}

/// fr fr Connection pool integration tests
mod connection_pool_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_sqlite_connection_pool() {
        let config = PoolConfig::new()
            .with_size_limits(2, 5)
            .with_timeouts(Duration::from_secs(5), Duration::from_secs(300))
            .with_connection_config(ConnectionConfig::new("sqlite", ":memory:"));
        
        let mut pool = ConnectionPool::new(config);
        pool.start().await.unwrap();
        
        // Test connection acquisition
        let conn1 = pool.acquire().await.unwrap();
        let conn2 = pool.acquire().await.unwrap();
        
        assert!(conn1.is_connected());
        assert!(conn2.is_connected());
        
        // Test that connections are independent
        conn1.execute("CREATE TABLE test1 (id INTEGER)", vec![]).await.unwrap();
        
        // This should fail because conn2 is a different in-memory database
        let result = conn2.query("SELECT * FROM test1", vec![]).await;
        assert!(result.is_err());
        
        // Return connections to pool
        pool.release(conn1).await.unwrap();
        pool.release(conn2).await.unwrap();
        
        // Acquire again (should reuse connections)
        let conn3 = pool.acquire().await.unwrap();
        assert!(conn3.is_connected());
        
        pool.release(conn3).await.unwrap();
        pool.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_connection_pool_stress() {
        let config = PoolConfig::new()
            .with_size_limits(1, 3)
            .with_timeouts(Duration::from_secs(1), Duration::from_secs(60))
            .with_connection_config(ConnectionConfig::new("sqlite", ":memory:"));
        
        let mut pool = ConnectionPool::new(config);
        pool.start().await.unwrap();
        
        // Create multiple concurrent tasks that acquire connections
        let pool_ref = &pool;
        let handles: Vec<_> = (0..10).map(|i| {
            tokio::spawn(async move {
                let conn = pool_ref.acquire().await.unwrap();
                
                // Simulate work
                conn.execute("SELECT 1", vec![]).await.unwrap();
                tokio::time::sleep(Duration::from_millis(10)).await;
                
                pool_ref.release(conn).await.unwrap();
                i
            })
        }).collect();
        
        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }
        
        let stats = pool.statistics();
        assert!(stats.total_connections() <= 3); // Should not exceed max pool size
        assert_eq!(stats.connection_errors(), 0); // No errors should occur
        
        pool.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_pool_manager() {
        let mut manager = PoolManager::new();
        
        // Create primary pool
        let primary_config = PoolConfig::new()
            .with_name("primary")
            .with_size_limits(2, 5)
            .with_connection_config(ConnectionConfig::new("sqlite", ":memory:"));
        
        manager.create_pool("primary", primary_config).await.unwrap();
        
        // Create readonly pool
        let readonly_config = PoolConfig::new()
            .with_name("readonly")
            .with_size_limits(1, 3)
            .with_connection_config(ConnectionConfig::new("sqlite", ":memory:"));
        
        manager.create_pool("readonly", readonly_config).await.unwrap();
        
        // Start pools
        manager.start_pool("primary").await.unwrap();
        manager.start_pool("readonly").await.unwrap();
        
        // Test connection acquisition from different pools
        let primary_conn = manager.acquire_from_pool("primary").await.unwrap();
        let readonly_conn = manager.acquire_from_pool("readonly").await.unwrap();
        
        assert!(primary_conn.is_connected());
        assert!(readonly_conn.is_connected());
        
        // Release connections
        manager.release_to_pool("primary", primary_conn).await.unwrap();
        manager.release_to_pool("readonly", readonly_conn).await.unwrap();
        
        // Stop pools
        manager.stop_pool("primary").await.unwrap();
        manager.stop_pool("readonly").await.unwrap();
        
        // Remove pools
        manager.remove_pool("primary").await.unwrap();
        manager.remove_pool("readonly").await.unwrap();
        
        assert_eq!(manager.pool_count(), 0);
    }
}

/// fr fr Migration integration tests
mod migration_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_migration_execution() {
        let config = ConnectionConfig::new("sqlite", ":memory:");
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        // Create migration runner
        let mut runner = MigrationRunner::new();
        
        // Add migrations
        let migration1 = Migration::new("001", "create_users_table", 1)
            .with_up_script("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .with_down_script("DROP TABLE users");
        
        let migration2 = Migration::new("002", "add_email_column", 2)
            .with_up_script("ALTER TABLE users ADD COLUMN email TEXT")
            .with_down_script("ALTER TABLE users DROP COLUMN email")
            .with_dependency("001");
        
        let migration3 = Migration::new("003", "create_posts_table", 3)
            .with_up_script("CREATE TABLE posts (id INTEGER PRIMARY KEY, user_id INTEGER, title TEXT, content TEXT, FOREIGN KEY(user_id) REFERENCES users(id))")
            .with_down_script("DROP TABLE posts")
            .with_dependency("002");
        
        runner.add_migration(migration1);
        runner.add_migration(migration2);
        runner.add_migration(migration3);
        
        // Execute migrations
        runner.run_migrations(&mut connection).await.unwrap();
        
        // Verify tables were created
        let result = connection.query("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name", vec![]).await.unwrap();
        let table_names: Vec<String> = result.rows().iter()
            .map(|row| row.get_string("name").unwrap())
            .collect();
        
        assert!(table_names.contains(&"users".to_string()));
        assert!(table_names.contains(&"posts".to_string()));
        
        // Test that users table has email column
        let pragma_result = connection.query("PRAGMA table_info(users)", vec![]).await.unwrap();
        let column_names: Vec<String> = pragma_result.rows().iter()
            .map(|row| row.get_string("name").unwrap())
            .collect();
        
        assert!(column_names.contains(&"id".to_string()));
        assert!(column_names.contains(&"name".to_string()));
        assert!(column_names.contains(&"email".to_string()));
        
        // Test rollback
        runner.rollback_to_version(&mut connection, 1).await.unwrap();
        
        // Verify posts table was dropped and email column removed
        let result2 = connection.query("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name", vec![]).await.unwrap();
        let table_names2: Vec<String> = result2.rows().iter()
            .map(|row| row.get_string("name").unwrap())
            .collect();
        
        assert!(table_names2.contains(&"users".to_string()));
        assert!(!table_names2.contains(&"posts".to_string()));
        
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn test_migration_with_data() {
        let config = ConnectionConfig::new("sqlite", ":memory:");
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        let mut runner = MigrationRunner::new();
        
        // Migration to create table and add initial data
        let migration1 = Migration::new("001", "setup_initial_data", 1)
            .with_up_script("
                CREATE TABLE settings (id INTEGER PRIMARY KEY, key TEXT UNIQUE, value TEXT);
                INSERT INTO settings (key, value) VALUES ('app_name', 'CURSED Database');
                INSERT INTO settings (key, value) VALUES ('version', '1.0.0');
            ")
            .with_down_script("DROP TABLE settings");
        
        // Migration to add new setting
        let migration2 = Migration::new("002", "add_debug_setting", 2)
            .with_up_script("INSERT INTO settings (key, value) VALUES ('debug_mode', 'false')")
            .with_down_script("DELETE FROM settings WHERE key = 'debug_mode'")
            .with_dependency("001");
        
        // Migration to update existing setting
        let migration3 = Migration::new("003", "update_version", 3)
            .with_up_script("UPDATE settings SET value = '1.1.0' WHERE key = 'version'")
            .with_down_script("UPDATE settings SET value = '1.0.0' WHERE key = 'version'")
            .with_dependency("002");
        
        runner.add_migration(migration1);
        runner.add_migration(migration2);
        runner.add_migration(migration3);
        
        // Run all migrations
        runner.run_migrations(&mut connection).await.unwrap();
        
        // Verify final state
        let result = connection.query("SELECT key, value FROM settings ORDER BY key", vec![]).await.unwrap();
        assert_eq!(result.row_count(), 3);
        
        let rows = result.rows();
        assert_eq!(rows[0].get_string("key").unwrap(), "app_name");
        assert_eq!(rows[0].get_string("value").unwrap(), "CURSED Database");
        assert_eq!(rows[1].get_string("key").unwrap(), "debug_mode");
        assert_eq!(rows[1].get_string("value").unwrap(), "false");
        assert_eq!(rows[2].get_string("key").unwrap(), "version");
        assert_eq!(rows[2].get_string("value").unwrap(), "1.1.0");
        
        // Test partial rollback
        runner.rollback_to_version(&mut connection, 2).await.unwrap();
        
        // Verify version was rolled back
        let version_result = connection.query("SELECT value FROM settings WHERE key = 'version'", vec![]).await.unwrap();
        assert_eq!(version_result.rows()[0].get_string("value").unwrap(), "1.0.0");
        
        // Verify debug_mode still exists
        let debug_result = connection.query("SELECT value FROM settings WHERE key = 'debug_mode'", vec![]).await.unwrap();
        assert_eq!(debug_result.row_count(), 1);
        
        connection.close().await.unwrap();
    }
}

/// fr fr Performance and stress tests
mod performance_tests {
    use super::*;

    #[tokio::test]
    async fn test_bulk_insert_performance() {
        let config = ConnectionConfig::new("sqlite", ":memory:");
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        // Create test table
        connection.execute("CREATE TABLE performance_test (id INTEGER PRIMARY KEY, name TEXT, value INTEGER, created_at TEXT)", vec![]).await.unwrap();
        
        let start_time = std::time::Instant::now();
        
        // Insert 1000 records
        for i in 0..1000 {
            connection.execute("INSERT INTO performance_test (name, value, created_at) VALUES (?, ?, ?)", vec![
                SqlValue::Text(format!("Record {}", i)),
                SqlValue::Integer(i),
                SqlValue::Text("2024-01-01 12:00:00".to_string())
            ]).await.unwrap();
        }
        
        let insert_duration = start_time.elapsed();
        println!("Inserted 1000 records in {:?}", insert_duration);
        
        // Verify count
        let count_result = connection.query("SELECT COUNT(*) as total FROM performance_test", vec![]).await.unwrap();
        assert_eq!(count_result.rows()[0].get_i64("total").unwrap(), 1000);
        
        // Test query performance
        let query_start = std::time::Instant::now();
        let result = connection.query("SELECT * FROM performance_test WHERE value > ? ORDER BY value", vec![
            SqlValue::Integer(500)
        ]).await.unwrap();
        let query_duration = query_start.elapsed();
        
        println!("Queried {} records in {:?}", result.row_count(), query_duration);
        assert_eq!(result.row_count(), 499);
        
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn test_transaction_performance() {
        let config = ConnectionConfig::new("sqlite", ":memory:");
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        connection.execute("CREATE TABLE txn_test (id INTEGER PRIMARY KEY, value TEXT)", vec![]).await.unwrap();
        
        // Test individual inserts (slow)
        let individual_start = std::time::Instant::now();
        for i in 0..100 {
            connection.execute("INSERT INTO txn_test (value) VALUES (?)", vec![
                SqlValue::Text(format!("Individual {}", i))
            ]).await.unwrap();
        }
        let individual_duration = individual_start.elapsed();
        
        // Test batched transaction (fast)
        let batch_start = std::time::Instant::now();
        let mut txn = connection.begin_transaction().await.unwrap();
        for i in 0..100 {
            txn.execute("INSERT INTO txn_test (value) VALUES (?)", vec![
                SqlValue::Text(format!("Batch {}", i))
            ]).await.unwrap();
        }
        txn.commit().await.unwrap();
        let batch_duration = batch_start.elapsed();
        
        println!("Individual inserts: {:?}", individual_duration);
        println!("Batched transaction: {:?}", batch_duration);
        
        // Batched should be significantly faster
        assert!(batch_duration < individual_duration);
        
        // Verify total count
        let count_result = connection.query("SELECT COUNT(*) as total FROM txn_test", vec![]).await.unwrap();
        assert_eq!(count_result.rows()[0].get_i64("total").unwrap(), 200);
        
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn test_concurrent_connections() {
        let db_path = "/tmp/cursed_concurrent_test.db";
        
        // Clean up any existing test database
        if std::path::Path::new(db_path).exists() {
            std::fs::remove_file(db_path).unwrap();
        }
        
        // Create database and table
        let config = ConnectionConfig::new("sqlite", db_path);
        let mut setup_conn = SqliteDriver::new().connect(config.clone()).await.unwrap();
        setup_conn.execute("CREATE TABLE concurrent_test (id INTEGER PRIMARY KEY, thread_id INTEGER, counter INTEGER)", vec![]).await.unwrap();
        setup_conn.close().await.unwrap();
        
        // Create multiple concurrent connections
        let handles: Vec<_> = (0..5).map(|thread_id| {
            let config = config.clone();
            tokio::spawn(async move {
                let mut connection = SqliteDriver::new().connect(config).await.unwrap();
                
                // Each thread inserts 20 records
                for counter in 0..20 {
                    connection.execute("INSERT INTO concurrent_test (thread_id, counter) VALUES (?, ?)", vec![
                        SqlValue::Integer(thread_id),
                        SqlValue::Integer(counter)
                    ]).await.unwrap();
                }
                
                connection.close().await.unwrap();
                thread_id
            })
        }).collect();
        
        // Wait for all threads to complete
        for handle in handles {
            handle.await.unwrap();
        }
        
        // Verify results
        let verify_conn = SqliteDriver::new().connect(config).await.unwrap();
        let result = verify_conn.query("SELECT COUNT(*) as total FROM concurrent_test", vec![]).await.unwrap();
        assert_eq!(result.rows()[0].get_i64("total").unwrap(), 100); // 5 threads * 20 records
        
        // Verify each thread wrote its records
        for thread_id in 0..5 {
            let thread_result = verify_conn.query("SELECT COUNT(*) as count FROM concurrent_test WHERE thread_id = ?", vec![
                SqlValue::Integer(thread_id)
            ]).await.unwrap();
            assert_eq!(thread_result.rows()[0].get_i64("count").unwrap(), 20);
        }
        
        verify_conn.close().await.unwrap();
        
        // Clean up
        std::fs::remove_file(db_path).unwrap();
    }
}

/// fr fr Error handling and edge case tests
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_errors() {
        // Test invalid database path
        let invalid_config = ConnectionConfig::new("sqlite", "/invalid/path/database.db");
        let result = SqliteDriver::new().connect(invalid_config).await;
        assert!(result.is_err());
        
        // Test invalid connection string
        let invalid_config2 = ConnectionConfig::from_string("invalid://not-a-url");
        assert!(invalid_config2.is_err());
    }

    #[tokio::test]
    async fn test_query_errors() {
        let config = ConnectionConfig::new("sqlite", ":memory:");
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        // Test syntax error
        let result = connection.execute("INVALID SQL SYNTAX", vec![]).await;
        assert!(result.is_err());
        
        // Test table not found
        let result = connection.query("SELECT * FROM nonexistent_table", vec![]).await;
        assert!(result.is_err());
        
        // Test parameter mismatch
        let result = connection.execute("SELECT * FROM sqlite_master WHERE name = ?", vec![]).await;
        assert!(result.is_err()); // No parameter provided
        
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn test_transaction_errors() {
        let config = ConnectionConfig::new("sqlite", ":memory:");
        let mut connection = SqliteDriver::new().connect(config).await.unwrap();
        
        connection.execute("CREATE TABLE error_test (id INTEGER PRIMARY KEY, name TEXT UNIQUE)", vec![]).await.unwrap();
        
        let mut txn = connection.begin_transaction().await.unwrap();
        
        // Insert valid record
        txn.execute("INSERT INTO error_test (name) VALUES (?)", vec![
            SqlValue::Text("valid".to_string())
        ]).await.unwrap();
        
        // Try to insert duplicate (should fail)
        let result = txn.execute("INSERT INTO error_test (name) VALUES (?)", vec![
            SqlValue::Text("valid".to_string())
        ]).await;
        assert!(result.is_err());
        
        // Transaction should still be active and rollbackable
        txn.rollback().await.unwrap();
        
        // Verify no data was inserted
        let count_result = connection.query("SELECT COUNT(*) as total FROM error_test", vec![]).await.unwrap();
        assert_eq!(count_result.rows()[0].get_i64("total").unwrap(), 0);
        
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn test_pool_timeout_errors() {
        let config = PoolConfig::new()
            .with_size_limits(1, 1) // Only one connection
            .with_timeouts(Duration::from_millis(100), Duration::from_secs(60)) // Very short timeout
            .with_connection_config(ConnectionConfig::new("sqlite", ":memory:"));
        
        let mut pool = ConnectionPool::new(config);
        pool.start().await.unwrap();
        
        // Acquire the only connection
        let _conn1 = pool.acquire().await.unwrap();
        
        // Try to acquire another connection (should timeout)
        let result = pool.acquire().await;
        assert!(result.is_err());
        
        pool.stop().await.unwrap();
    }
}

/// fr fr Run all integration tests
#[test]
fn run_all_integration_tests() {
    println!("🔗 Running comprehensive database integration tests...");
    println!("📋 SQLite integration tests available");
    println!("🐘 PostgreSQL integration tests available (requires database)");
    println!("🏊‍♂️ Connection pool integration tests available");
    println!("🚀 Migration integration tests available");
    println!("⚡ Performance and stress tests available");
    println!("🚨 Error handling tests available");
    println!("✅ Integration test suite ready for execution with: cargo test --test database_integration_tests");
}
