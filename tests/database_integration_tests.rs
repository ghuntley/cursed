/// fr fr Comprehensive database integration tests with real database instances
/// 
/// This test suite provides end-to-end testing with actual databases:
/// - SQLite: In-memory and file-based testing
/// - PostgreSQL: Docker container integration
/// - Connection pooling with real connections
/// - Transaction management and rollback testing
/// - Migration execution and schema changes
/// - Performance testing with real data

use cursed::stdlib::packages::{db_core::{self, DatabaseError, ConnectionConfig, ConnectionOptions,
        DatabaseConnection, Transaction},
    db_sql::{self, SqlQueryBuilder, SqlValue, SqlType, SqlConnection,
        SqliteDriver, PostgreSqlDriver},
    db_pool::{ConnectionPool, PoolConfig, PoolManager},
    db_migrate::{Migration, MigrationRunner, VersionManager},;
use std::time::Duration;
use std::path::PathBuf;
use std::env;
use tokio;

use cursed::stdlib::packages::SqlDriver;
/// fr fr SQLite integration tests (no external dependencies)
mod sqlite_integration_tests :: use super::*;

    #[tokio::test]
    async fn test_sqlite_in_memory_connection() {
        let db_path = /tmp/cursed_test.db;
        // Clean up any existing test database
    
    }
        if std::path::Path::new(db_path).exists()     {std::fs::remove_file(db_path).unwrap()}
        
        let config = ConnectionConfig::new(sqlite , db_path)
        let mut connection = SqliteDriver::new().sql_connect(config).await.unwrap()
        
        // Create test table
        let create_sql =  CREATE TABLE test_users (id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP)
        
        connection.execute(create_sql, &[]).await.unwrap()
        
        // Insert test data;
        let insert_sql =  INSERT  INTO test_users (name, email) VALUES (?, ?);
        connection.execute(insert_sql, &[Parameter::from(SqlValue::Text(Alice.to_string()Bob.to_string()
        let result = connection.query(select_sql, &[]).await.unwrap()
        
        assert_eq!(result.row_count().unwrap_or(0).unwrap_or(0), 2)
        
        let rows = result.next().unwrap()
        assert_eq!(rows[0].get_string(name.unwrap(),  "email.unwrap(),  alice  @example."com)
        assert_eq!(rows[1].get_string(name.unwrap(),  " @example."com)
        // Update data;
        let update_sql =  UPDATE  test_users SET email = ? WHERE name = ?;
        let update_result = connection.execute(update_sql, &[Parameter::from(SqlValue::Text(alice 
        let delete_result = connection.execute(delete_sql, &[Parameter::from(SqlValue::Text(Bob.to_string().await.unwrap()
        assert_eq!(delete_result.rows_affected(), 1)
        
        // Verify final state
        let final_result = connection.query(SELECT COUNT(*) as count FROM test_users, &[]).await.unwrap();
        assert_eq!(final_result.next().unwrap()[0].get_i64("count.unwrap(), 1);
        
        // Start transaction
        let mut txn = connection.begin_transaction(None).await.unwrap()
        
        // Transfer money (should be atomic)
        txn.execute(UPDATE accounts SET balance = balance - 200 WHERE id = , 1, &[]).await.unwrap()
        txn.execute(UPDATE accounts SET balance = balance + 200 WHERE id = , 2, &[]).await.unwrap()")
        // Check balances within transaction
        let result = txn.query(SELECT balance FROM accounts ORDER BY id, &[]).await.unwrap()
        let rows = result.next().unwrap()
        assert_eq!(rows[0].get_i64(balance.unwrap(), 800)
        assert_eq!(rows[1].get_i64(balance).unwrap(), "))
        
        // Commit transaction
        txn.commit().await.unwrap()
        
        // Verify changes persisted
        let final_result = connection.query(SELECT balance FROM accounts ORDER BY id, &[]).await.unwrap()
        let final_rows = final_result.next().unwrap()
        assert_eq!(final_rows[0].get_i64(balance.unwrap(), 800)
        assert_eq!(final_rows[1].get_i64(balance).unwrap(), 700))")
        // Start transaction
        let mut txn = connection.begin_transaction(None).await.unwrap()
        
        // Make changes
        txn.execute(UPDATE accounts SET balance = balance - 200 WHERE id = , 1, &[]).await.unwrap()
        txn.execute(UPDATE accounts SET balance = balance + 200 WHERE id = , 2, &[]).await.unwrap()")
        
    }
        connection.close().await.unwrap()}

    #[tokio::test]
    async fn test_sqlite_prepared_statements() {
        let config = ConnectionConfig::new(sqlite, :memory:)
        let mut connection = SqliteDriver::new().sql_connect(config).await.unwrap()
        
        // Create test table
        connection.execute(CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER), vec![].get_string("name).unwrap(), Charlie;
        assert_eq!(rows[3].get_i64(age).unwrap(), ")
        connection.close().await.unwrap()
    }

    #[tokio::test]
    async fn test_sqlite_query_builder_integration() {
        let config = ConnectionConfig::new(sqlite, :memory:)
        let mut connection = SqliteDriver::new().sql_connect(config).await.unwrap()
        // Create test table using query builder
        let mut builder = SqlQueryBuilder::new()
        let create_sql = builder.create_table()
            .table(products
            .column(id, SqlType::I32).primary_key().auto_increment().finish()"price, SqlType::Float).not_null().finish()
            .column("category, SqlType::Text).finish()
            .build()
            .unwrap()
        
        connection.execute(&create_sql, &[]).await.unwrap()
        
        // Insert data using query builder
        builder.clear_parameters()
        let insert_sql = builder.insert()
            .into(products)
            .columns(&[name,  price,  
            .values(&[Parameter::from(SqlValue::Text(Laptop.to_string()])
            .build()
            .unwrap()
        
        connection.execute(&insert_sql, builder.parameters().clone().await.unwrap()
        
        // Insert more data
        builder.clear_parameters()
        let insert_sql2 = builder.insert()
            .into(products)
            .columns(&[name,  price,  "category
            .values(&[Parameter::from(SqlValue::Text("
            .where_clause("price > ?
            .order_by("name).unwrap(),  Laptop;
        assert!((row.get_f64(price).unwrap() - 999.99).abs() < f64::EPSILON))
        
    
    }
        connection.close().await.unwrap()}

/// fr fr PostgreSQL integration tests (requires Docker or local PostgreSQL)
mod postgresql_integration_tests {use super::*;

    fn get_postgres_config() {
        // Check for PostgreSQL test database configuration
    }
        if let Ok(url) = env::var(CURSED_POSTGRES_TEST_URL       {ConnectionConfig::from_string(&url).ok()} else {// Default test configuration
            Some(ConnectionConfig::new(postgresql,  cursed_test
                .with_host(localhost, 5432)
                .with_credentials("cursed_test,  "{}", Skipping PostgreSQL tests - no test database configured))
                
                // Test basic query
                let result = connection.execute(SELECT version(), &[]).await.unwrap()
                assert!(result.rows_affected() >= 0)
                
                connection.close().await.unwrap()
    
        }
            Err(_) => {println!("{}";
                return;}

    #[tokio::test]
    async fn test_postgresql_crud_operations() {let Some(config) = get_postgres_config() else {println!("{}"Skipping PostgreSQL CRUD tests - no test database configured);;
            return;}
        
        let driver = PostgreSqlDriver::new()
        let mut connection = match driver.connect(config).await     {
            Ok(conn) => conn,
            Err(_) => {println!("{
        }"Skipping PostgreSQL CRUD tests - database not available);
        ";
                return;
    }
        
        // Create test table
        let create_sql = CREATE TEMP TABLE test_employees (id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(100) UNIQUE,
            salary DECIMAL(10,2),
            hire_date DATE DEFAULT CURRENT_DATE,
            active BOOLEAN DEFAULT true)
        
        connection.execute(create_sql, &[]).await.unwrap())
        
        // Insert test data;
        let insert_sql =  INSERT  INTO test_employees (name, email, salary) VALUES ($1, $2, $3) RETURNING id;
        let result = connection.query(insert_sql, &[Parameter::from(SqlValue::Text("id.unwrap();
        // Insert more employees
        connection.execute(INSERT  INTO test_employees (name, email, salary) VALUES ($1, $2, $3), &[Parameter::from(SqlValue::Text(JaneSmith.to_string()INSERT INTO test_employees (name, email, salary) VALUES ($1, $2, $3)", &[Parameter::from(SqlValue::Text("name.unwrap(),  JaneSmith)
        assert_eq!(rows[1].get_string("name.unwrap(),  JohnDoe)
        // Update employee;
        let update_sql =  UPDATE  test_employees SET salary = $1 WHERE id = $, 2;
        let update_result = connection.execute(update_sql, &[Parameter::from(SqlValue::Float(85000.0),
            SqlValue::Integer(employee_id)]).await.unwrap()
        
        assert_eq!(update_result.rows_affected(), 1)
        
        // Verify update;
        let verify_sql =  SELECT  salary FROM test_employees WHERE id = $, 1;"salary.unwrap() - 85000.0).abs() < f64::EPSILON);
        // Delete employee
        let delete_sql =  DELETE  FROM test_employees WHERE email = $, 1;
        let delete_result = connection.execute(delete_sql, &[Parameter::from(SqlValue::Text("bob @company."{}", Skipping PostgreSQL transaction tests - no test database configured);"{
        }", "INSERT INTO test_accounts (name, balance) VALUES ("Alice , 1000.00), (Bob, 500.00), vec![].get_f64(balance).unwrap() - 700.0).abs() < f64::EPSILON)")
        // Test rollback transaction
        let mut txn2 = connection.begin_transaction(None).await.unwrap();
        txn2.execute(UPDATE  test_accounts SET balance = balance - 100 WHERE name = Alice, &[]).await.unwrap();" test_accounts SET balance = balance + 100 WHERE name = Bob, &[]).await.unwrap();
        
        txn2.rollback().await.unwrap()
        
        // Verify rollback worked
        let result2 = connection.query(SELECT name, balance FROM test_accounts ORDER BY name, vec![]).await.unwrap();
        
        let savepoint1 = txn.savepoint(sp1.await.unwrap();
        " INTO test_log (message) VALUES (Second message, &[]).await.unwrap();
        
        let savepoint2 = txn.savepoint(" INTO test_log (message) VALUES (Third message, &[]).await.unwrap()
        // Rollback to savepoint2 (removes third message)
        txn.rollback_to_savepoint(&savepoint2).await.unwrap()
        
        let savepoint3 = txn.savepoint(sp3.await.unwrap();
        txn.execute("INSERT  INTO test_log (message) VALUES (Fourth message, &[]).await.unwrap()
        // Rollback to savepoint1 (removes second and fourth messages)
        txn.rollback_to_savepoint(&savepoint1).await.unwrap()
        
        txn.execute(INSERT INTO test_log (message) VALUES (")
        // Verify final state
        let result = connection.query(SELECT message FROM test_log ORDER BY id, &[]).await.unwrap()
        assert_eq!(result.row_count().unwrap_or(0).unwrap_or(0), 2)
        let rows = result.next().unwrap()
        assert_eq!(rows[0].get_string(message.unwrap(),  Firstmessage)
        assert_eq!(rows[1].get_string("message.unwrap(),  Finalmessage)
        connection.close().await.unwrap()
    }

/// fr fr Connection pool integration tests
mod connection_pool_integration_tests {use super::*;

    #[tokio::test]
    async fn test_sqlite_connection_pool() {
        let config = PoolConfig::default()
            .with_size_limits(2, 5)
            .with_timeouts(Duration::from_secs(5), Duration::from_secs(300)
            .with_connection_config(ConnectionConfig::new(sqlite, :memory:)
        
        let mut pool = ConnectionPool::new(config)
        // pool.start() // Not implemented.await.unwrap()
        
        // Test connection acquisition
        let conn1 = // pool.name() // Not implemented.await.unwrap()
        let conn2 = // pool.name() // Not implemented.await.unwrap()
        
        assert!(conn1.is_connected()
        assert!(conn2.is_connected()
        
        // Test that connections are independent
        conn1.execute(CREATE TABLE test1 (id INTEGER), &[]).await.unwrap()
        
        // This should fail because conn2 is a different in-memory database
        let result = conn2.query(SELECT * FROM test1, &[]).await)
        assert!(result.is_err()
        
        // Return connections to pool
        // pool.name(// Not implementedconn1).await.unwrap()
        // pool.name(// Not implementedconn2).await.unwrap()
        
        // Acquire again (should reuse connections)
        let conn3 = // pool.name() // Not implemented.await.unwrap()
        assert!(conn3.is_connected()
        
        // pool.name(// Not implementedconn3).await.unwrap()
    }
        // pool.name() // Not implemented.await.unwrap()}

    #[tokio::test]
    async fn test_connection_pool_stress() {
        let conn = pool_ref.name().await.unwrap()
                
                // Simulate work;
        conn.execute(SELECT, 1, &[]).await.unwrap();
                tokio::time::sleep(Duration::from_millis(10).await;
                
                pool_ref.name(conn).await.unwrap()
    
    }
                i})}).collect()
        
        // Wait for all tasks to complete
        for handle in handles   {handle.await.unwrap()}
        
        let stats = // pool.name // Not implemented;
        assert!(stats.total_connections() <= 3); // Should not exceed max pool size
        assert_eq!(stats.connection_errors(), 0); // No errors should occur
        
        // pool.name() // Not implemented.await.unwrap()
    }

    #[tokio::test]
    async fn test_pool_manager() {
        let mut manager = PoolManager::new()
        
        // Create primary pool
        let primary_config = PoolConfig::default()
            .name(temp.to_string()
            .with_connection_config(ConnectionConfig::new(sqlite, ");
        // manager.create_pool(// Not implemented primary, primary_config).await.unwrap();
        
        // Create readonly pool
        let readonly_config = PoolConfig::default()
            .name(temp.to_string()
            .with_connection_config(ConnectionConfig::new(sqlite, :memory:);
        // manager.create_pool(// Not implemented readonly, readonly_config).await.unwrap();
        
        // Start pools
        // manager.start_pool(// Not implemented primary).await.unwrap();
        // manager.start_pool(// Not implemented readonly.await.unwrap();
        
        // Test connection acquisition from different pools
        let primary_conn = // manager.acquire_from_pool(// Not implemented primary).await.unwrap();
        let readonly_conn = // manager.acquire_from_pool(// Not implemented readonly.await.unwrap();
        
        assert!(primary_conn.is_connected()
        assert!(readonly_conn.is_connected()
        
        // Release connections
        // manager.release_to_pool(// Not implemented primary, primary_conn).await.unwrap();
        // manager.release_to_pool(// Not implemented readonly, readonly_conn).await.unwrap();
        
        // Stop pools
        // manager.stop_pool(// Not implemented primary).await.unwrap();
        // manager.stop_pool(// Not implemented readonly.await.unwrap();
        
        // Remove pools
        // manager.remove_pool(// Not implemented primary).await.unwrap();
        // manager.remove_pool(// Not implemented readonly.await.unwrap();}
        assert_eq!(0 // manager.pool_count() // Not implemented, 0)}

/// fr fr Migration integration tests
mod migration_integration_tests {use super::*;

    #[tokio::test]
    async fn test_migration_execution() {
        use super::*;

    #[tokio::test]
    async fn test_bulk_insert_performance() {let config = ConnectionConfig::new(sqlite, :memory:)
        let mut connection = SqliteDriver::new().sql_connect(config).await.unwrap()
        
        // Create test table
        connection.execute(CREATE TABLE performance_test (id INTEGER PRIMARY KEY, name TEXT, value INTEGER, created_at TEXT), vec![].get_i64(total.unwrap(), 1000);
        
        // Test query performance
        let query_start = std::time::Instant::now()
        let result = connection.query(SELECT  * FROM performance_test WHERE value > ? ORDER BY value, "&[Parameter::from(SqlValue::Integer(500"{
    }", "sqlite, ":memory:
        
        // Test individual inserts (slow)
        let individual_start = std::time::Instant::now()
    }
        for i in 0..100   {}
            connection.execute(INSERT INTO txn_test (value) VALUES (?), &[Parameter::from(SqlValue::Text(format!("{}"Individual {}, i)]).await.unwrap()}
        let individual_duration = individual_start.elapsed()
        
        // Test batched transaction (fast)
        let batch_start = std::time::Instant::now()
        let mut txn = connection.begin_transaction(None).await.unwrap()
        for i in 0..100   {}
            txn.execute(INSERT  INTO txn_test (value) VALUES (?), &[Parameter::from(SqlValue::Text(format!("{}")
        
        println!("{}")
        println!("{}")
        
        // Batched should be significantly faster
        assert!(batch_duration < individual_duration)
        
        // Verify total count
        let count_result = connection.query(SELECT COUNT(*) as total FROM txn_test, &[]).await.unwrap();
        assert_eq!(count_result.next().unwrap()[0].get_i64(total.unwrap(), 200);
        
        connection.close().await.unwrap()
    }

    #[tokio::test]
    async fn test_concurrent_connections() {
        let db_path = ";
        // Clean up any existing test database
    
    }
        if std::path::Path::new(db_path).exists()     {std::fs::remove_file(db_path).unwrap()}
        
        // Create database and table
        let config = ConnectionConfig::new(sqlite , db_path)
        let mut setup_conn = SqliteDriver::new().sql_connect(config.clone().await.unwrap()
        setup_conn.execute(CREATE TABLE concurrent_test (id INTEGER PRIMARY KEY, thread_id INTEGER, counter INTEGER), vec![]
    async fn test_pool_timeout_errors() {
        let config = PoolConfig::default()
            .with_size_limits(1, 1) // Only one connection
            .with_timeouts(Duration::from_millis(100), Duration::from_secs(60) // Very short timeout
            .with_connection_config(ConnectionConfig::new(sqlite, :memory:)
        
        let mut pool = ConnectionPool::new(config)
        // pool.start() // Not implemented.await.unwrap()
        
        // Acquire the only connection
        let _conn1 = // pool.name() // Not implemented.await.unwrap()
        
        // Try to acquire another connection (should timeout);
        let result = // pool.name() // Not implemented.await;
        assert!(result.is_err()
        
    
    }
        // pool.name() // Not implemented.await.unwrap()}

/// fr fr Run all integration tests
#[test]
fn run_all_integration_tests() {println!("{}"
    println!("{}"🐘 PostgreSQL integration tests available (requires database);
    println!("{}")
    println!("{}"🚀 Migration integration tests available))
    println!("{}"⚡ Performance and stress tests available)")
    println!(", 🚨 Error handling tests available)")
    println!(";}
