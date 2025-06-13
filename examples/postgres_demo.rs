/// PostgreSQL Driver Demo
/// 
/// Demonstrates the complete PostgreSQL driver functionality including:
/// - Connection management with configuration
/// - Connection pooling for high performance
/// - Query execution with parameter binding
/// - Prepared statements for efficiency
/// - Transaction management with savepoints
/// - Error handling and recovery
/// - Type mapping between PostgreSQL and CURSED types
/// 
/// Prerequisites:
/// - PostgreSQL server running locally or accessible
/// - Create a test database: createdb postgres_demo_db
/// - Optionally set environment variables for connection

use std::env;
use std::time::Duration;
use cursed::stdlib::database::{
    SqlValue, TxOptions, SqlIsolationLevel,
    postgres::{
        PostgresConfig, PostgresConnectionString, PostgresDriver, PostgresPool,
        PostgresConnection, SslMode, PostgresError
    }
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("PostgreSQL Driver Demo for CURSED Language");
    println!("==========================================\n");

    // Get configuration from environment or use defaults
    let config = get_demo_config();
    println!("Configuration:");
    println!("  Host: {}:{}", config.host, config.port);
    println!("  Database: {}", config.database);
    println!("  Username: {}", config.username);
    println!("  SSL Mode: {}", config.ssl_mode);
    println!();

    // Demo 1: Basic connection and simple queries
    demo_basic_connection(&config).await?;

    // Demo 2: Connection pooling
    demo_connection_pooling(&config).await?;

    // Demo 3: Parameterized queries and type mapping
    demo_parameterized_queries(&config).await?;

    // Demo 4: Prepared statements
    demo_prepared_statements(&config).await?;

    // Demo 5: Transaction management
    demo_transactions(&config).await?;

    // Demo 6: Error handling
    demo_error_handling(&config).await?;

    // Demo 7: Advanced features
    demo_advanced_features(&config).await?;

    println!("Demo completed successfully!");
    Ok(())
}

/// Get demo configuration from environment or defaults
fn get_demo_config() -> PostgresConfig {
    PostgresConfig {
        host: env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
        port: env::var("POSTGRES_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse()
            .unwrap_or(5432),
        database: env::var("POSTGRES_DB").unwrap_or_else(|_| "postgres_demo_db".to_string()),
        username: env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string()),
        password: env::var("POSTGRES_PASSWORD").ok(),
        ssl_mode: SslMode::Disable, // Use plain connection for demo
        connect_timeout: Duration::from_secs(10),
        query_timeout: Duration::from_secs(30),
        application_name: "CURSED PostgreSQL Demo".to_string(),
        max_connections: 20,
        min_connections: 5,
        max_lifetime: Some(Duration::from_secs(1800)), // 30 minutes
        idle_timeout: Some(Duration::from_secs(300)),   // 5 minutes
        retry_attempts: 3,
        retry_delay: Duration::from_secs(1),
    }
}

/// Demo 1: Basic connection and simple queries
async fn demo_basic_connection(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 1: Basic Connection and Simple Queries");
    println!("============================================");

    // Create connection
    let mut conn = PostgresConnection::new(config.clone()).await?;
    println!("✓ Connected to PostgreSQL server");

    // Check connection health
    if conn.is_alive().await {
        println!("✓ Connection is alive and healthy");
    }

    // Get connection metadata
    let metadata = conn.metadata();
    println!("✓ Connection metadata:");
    println!("  - Driver: {}", metadata.driver_name);
    println!("  - Database: {}", metadata.database_name);
    println!("  - Connection ID: {}", metadata.connection_id);

    // Execute simple query
    let result = conn.execute_query("SELECT version()", &[]).await?;
    if let SqlValue::String(version) = &result.rows[0][0] {
        println!("✓ PostgreSQL version: {}", version);
    }

    // Test basic data types
    let result = conn.execute_query(
        "SELECT 42 as int_val, 3.14 as float_val, 'hello' as text_val, true as bool_val",
        &[]
    ).await?;
    
    println!("✓ Basic data types query:");
    for (i, column) in result.columns.iter().enumerate() {
        println!("  - {}: {:?}", column, result.rows[0][i]);
    }

    println!();
    Ok(())
}

/// Demo 2: Connection pooling
async fn demo_connection_pooling(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 2: Connection Pooling");
    println!("==========================");

    // Create connection pool
    let pool = PostgresPool::new(config.clone()).await?;
    println!("✓ Created connection pool");

    // Get pool health information
    let health = pool.get_health();
    println!("✓ Pool health:");
    println!("  - Status: {}", if health.is_healthy { "Healthy" } else { "Unhealthy" });
    println!("  - Health Score: {:.1}%", health.health_score * 100.0);
    println!("  - Max Connections: {}", health.max_connections);
    println!("  - Current Connections: {}", health.total_connections);

    // Use connections from pool
    let mut handles = Vec::new();
    for i in 0..5 {
        let pool_clone = pool;
        let handle = tokio::spawn(async move {
            let conn = pool_clone.get_connection().await?;
            let result = conn.query(
                "SELECT $1::int as worker_id, pg_backend_pid() as backend_pid",
                &[&i]
            ).await?;
            
            println!("  Worker {}: Backend PID {}", i, result[0].get::<i32>(1));
            Ok::<(), PostgresError>(())
        });
        handles.push(handle);
    }

    // Wait for all workers to complete
    for handle in handles {
        handle.await.unwrap()?;
    }
    println!("✓ Completed concurrent pool operations");

    // Show updated pool statistics
    let stats = pool.get_statistics();
    println!("✓ Pool statistics:");
    println!("  - Total checkouts: {}", stats.total_checkouts);
    println!("  - Checkout failures: {}", stats.total_checkout_failures);
    println!("  - Average checkout time: {:.2}ms", stats.avg_checkout_time_ms);

    println!();
    Ok(())
}

/// Demo 3: Parameterized queries and type mapping
async fn demo_parameterized_queries(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 3: Parameterized Queries and Type Mapping");
    println!("===============================================");

    let mut conn = PostgresConnection::new(config.clone()).await?;

    // Test various parameter types
    let test_cases = vec![
        ("Integer", SqlValue::Integer(42)),
        ("Float", SqlValue::Float(3.14159)),
        ("String", SqlValue::String("Hello, PostgreSQL!".to_string())),
        ("Boolean (true)", SqlValue::Boolean(true)),
        ("Boolean (false)", SqlValue::Boolean(false)),
        ("Null", SqlValue::Null),
    ];

    println!("✓ Testing parameter types:");
    for (name, param) in test_cases {
        let result = conn.execute_query("SELECT $1", &[param.clone()]).await?;
        println!("  - {}: {:?} -> {:?}", name, param, result.rows[0][0]);
    }

    // Test complex parameterized query
    let args = vec![
        SqlValue::String("John Doe".to_string()),
        SqlValue::Integer(30),
        SqlValue::Boolean(true),
        SqlValue::Float(75000.50),
    ];

    let result = conn.execute_query(
        r#"
        SELECT 
            $1::text as name,
            $2::int as age,
            $3::bool as is_active,
            $4::numeric as salary,
            CASE WHEN $2 >= 18 THEN 'Adult' ELSE 'Minor' END as category
        "#,
        &args
    ).await?;

    println!("✓ Complex parameterized query result:");
    for (i, column) in result.columns.iter().enumerate() {
        println!("  - {}: {:?}", column, result.rows[0][i]);
    }

    println!();
    Ok(())
}

/// Demo 4: Prepared statements
async fn demo_prepared_statements(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 4: Prepared Statements");
    println!("===========================");

    let mut conn = PostgresConnection::new(config.clone()).await?;

    // Prepare a statement
    let stmt = conn.prepare_statement(
        "SELECT $1::text as name, $2::int as value, $1 || ' has value ' || $2 as description"
    ).await?;

    println!("✓ Prepared statement:");
    let info = stmt.info();
    println!("  - Query: {}", info.query);
    println!("  - Parameters: {}", info.parameter_count);
    println!("  - Columns: {}", info.column_count);
    println!("  - Column names: {:?}", info.column_names);

    // Execute prepared statement multiple times with different parameters
    let test_data = vec![
        ("Alice", 100),
        ("Bob", 200),
        ("Charlie", 300),
    ];

    println!("✓ Executing prepared statement:");
    for (name, value) in test_data {
        // Note: In the actual implementation, prepared statement execution would be:
        // let result = stmt.query_async(&client, &[
        //     SqlValue::String(name.to_string()),
        //     SqlValue::Integer(value),
        // ]).await?;
        
        // For demo purposes, we'll use the connection directly
        let result = conn.execute_query(
            "SELECT $1::text as name, $2::int as value, $1 || ' has value ' || $2 as description",
            &[SqlValue::String(name.to_string()), SqlValue::Integer(value)]
        ).await?;

        if let (SqlValue::String(n), SqlValue::Integer(v), SqlValue::String(d)) = 
            (&result.rows[0][0], &result.rows[0][1], &result.rows[0][2]) {
            println!("  - {}: {} ({})", n, v, d);
        }
    }

    // Show statement statistics
    let stats = stmt.get_stats();
    println!("✓ Statement statistics:");
    println!("  - Executions: {}", stats.executions);
    println!("  - Total rows returned: {}", stats.total_rows_returned);
    println!("  - Total execution time: {}ms", stats.total_execution_time_ms);

    println!();
    Ok(())
}

/// Demo 5: Transaction management
async fn demo_transactions(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 5: Transaction Management");
    println!("==============================");

    let mut conn = PostgresConnection::new(config.clone()).await?;

    // Create a temporary table for transaction demo
    conn.execute_statement(
        r#"
        CREATE TEMPORARY TABLE demo_accounts (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            balance DECIMAL(10,2) NOT NULL
        )
        "#,
        &[]
    ).await?;

    // Insert initial data
    conn.execute_statement(
        "INSERT INTO demo_accounts (name, balance) VALUES ($1, $2), ($3, $4)",
        &[
            SqlValue::String("Alice".to_string()),
            SqlValue::Float(1000.0),
            SqlValue::String("Bob".to_string()),
            SqlValue::Float(500.0),
        ]
    ).await?;

    println!("✓ Created temporary table with initial data");

    // Demo transaction with commit
    {
        let mut tx = conn.begin_transaction(TxOptions {
            isolation_level: Some(SqlIsolationLevel::LevelReadCommitted),
            read_only: false,
        }).await?;

        println!("✓ Started transaction");

        // Transfer money from Alice to Bob
        tx.execute(
            "UPDATE demo_accounts SET balance = balance - $1 WHERE name = $2",
            &[SqlValue::Float(100.0), SqlValue::String("Alice".to_string())]
        ).await?;

        tx.execute(
            "UPDATE demo_accounts SET balance = balance + $1 WHERE name = $2",
            &[SqlValue::Float(100.0), SqlValue::String("Bob".to_string())]
        ).await?;

        // Check balances within transaction
        let result = tx.query("SELECT name, balance FROM demo_accounts ORDER BY name", &[]).await?;
        println!("✓ Balances within transaction:");
        for row in &result.rows {
            if let (SqlValue::String(name), SqlValue::Float(balance)) = (&row[0], &row[1]) {
                println!("  - {}: ${:.2}", name, balance);
            }
        }

        tx.commit().await?;
        println!("✓ Transaction committed");
    }

    // Demo transaction with savepoints and rollback
    {
        let mut tx = conn.begin_transaction(TxOptions::default()).await?;
        println!("✓ Started new transaction for savepoint demo");

        // Create savepoint
        let savepoint = tx.savepoint("before_transfer").await?;
        println!("✓ Created savepoint: {}", savepoint);

        // Make a change
        tx.execute(
            "UPDATE demo_accounts SET balance = balance - $1 WHERE name = $2",
            &[SqlValue::Float(200.0), SqlValue::String("Alice".to_string())]
        ).await?;

        // Check balance
        let result = tx.query("SELECT balance FROM demo_accounts WHERE name = $1", 
                             &[SqlValue::String("Alice".to_string())]).await?;
        if let SqlValue::Float(balance) = &result.rows[0][0] {
            println!("  - Alice's balance after change: ${:.2}", balance);
        }

        // Rollback to savepoint
        tx.rollback_to_savepoint(&savepoint).await?;
        println!("✓ Rolled back to savepoint");

        // Check balance again
        let result = tx.query("SELECT balance FROM demo_accounts WHERE name = $1", 
                             &[SqlValue::String("Alice".to_string())]).await?;
        if let SqlValue::Float(balance) = &result.rows[0][0] {
            println!("  - Alice's balance after rollback: ${:.2}", balance);
        }

        tx.rollback().await?;
        println!("✓ Transaction rolled back");
    }

    // Show final balances
    let result = conn.execute_query("SELECT name, balance FROM demo_accounts ORDER BY name", &[]).await?;
    println!("✓ Final balances:");
    for row in &result.rows {
        if let (SqlValue::String(name), SqlValue::Float(balance)) = (&row[0], &row[1]) {
            println!("  - {}: ${:.2}", name, balance);
        }
    }

    println!();
    Ok(())
}

/// Demo 6: Error handling
async fn demo_error_handling(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 6: Error Handling");
    println!("======================");

    let mut conn = PostgresConnection::new(config.clone()).await?;

    // Test syntax error
    println!("✓ Testing syntax error:");
    match conn.execute_query("INVALID SQL SYNTAX", &[]).await {
        Ok(_) => println!("  Unexpected success!"),
        Err(e) => {
            println!("  - Error kind: {:?}", e.kind);
            println!("  - Message: {}", e.message);
            if let Some(sqlstate) = e.sqlstate() {
                println!("  - SQLSTATE: {}", sqlstate);
            }
            println!("  - Is retryable: {}", e.is_retryable());
        }
    }

    // Test constraint violation (if we had constraints)
    println!("✓ Testing division by zero:");
    match conn.execute_query("SELECT 1/0", &[]).await {
        Ok(_) => println!("  Unexpected success!"),
        Err(e) => {
            println!("  - Error kind: {:?}", e.kind);
            println!("  - Message: {}", e.message);
        }
    }

    // Test connection error recovery
    println!("✓ Testing connection health and recovery:");
    if conn.is_alive().await {
        println!("  - Connection is healthy");
    } else {
        println!("  - Connection lost, attempting recovery...");
        // In a real scenario, you might attempt reconnection here
    }

    println!();
    Ok(())
}

/// Demo 7: Advanced features
async fn demo_advanced_features(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 7: Advanced Features");
    println!("=========================");

    let mut conn = PostgresConnection::new(config.clone()).await?;

    // Test JSON support
    println!("✓ Testing JSON support:");
    let json_data = serde_json::json!({
        "name": "John Doe",
        "age": 30,
        "skills": ["Rust", "PostgreSQL", "CURSED"]
    });

    let result = conn.execute_query(
        "SELECT $1::jsonb as data, $1::jsonb->>'name' as name, $1::jsonb->'age' as age",
        &[SqlValue::Json(json_data)]
    ).await?;

    for (i, column) in result.columns.iter().enumerate() {
        println!("  - {}: {:?}", column, result.rows[0][i]);
    }

    // Test array operations (PostgreSQL-specific)
    println!("✓ Testing array operations:");
    let result = conn.execute_query(
        "SELECT ARRAY[1,2,3,4,5] as numbers, 'hello,world,test'::text[] as words",
        &[]
    ).await?;

    for (i, column) in result.columns.iter().enumerate() {
        println!("  - {}: {:?}", column, result.rows[0][i]);
    }

    // Test timestamp operations
    println!("✓ Testing timestamp operations:");
    let result = conn.execute_query(
        "SELECT NOW() as current_time, NOW() + INTERVAL '1 day' as tomorrow, EXTRACT(epoch FROM NOW()) as epoch",
        &[]
    ).await?;

    for (i, column) in result.columns.iter().enumerate() {
        println!("  - {}: {:?}", column, result.rows[0][i]);
    }

    // Connection statistics
    let stats = conn.get_stats();
    println!("✓ Connection statistics:");
    println!("  - Queries executed: {}", stats.queries_executed);
    println!("  - Statements prepared: {}", stats.statements_prepared);
    println!("  - Transactions started: {}", stats.transactions_started);
    println!("  - Errors encountered: {}", stats.errors_encountered);
    println!("  - Reconnections: {}", stats.reconnections);

    println!();
    Ok(())
}
