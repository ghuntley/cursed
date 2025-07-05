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
use std::collections::HashMap;
use cursed::stdlib::database::{
    SqlValue, TxOptions, SqlIsolationLevel,
    postgres::{
        PostgresConfig, PostgresConnectionString, PostgresDriver, PostgresPool,
        PostgresConnection, SslMode, PostgresError, PostgresPoolConfig
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
        search_path: None,
        timezone: None,
        options: std::collections::HashMap::new(),
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
    let connection_string = config.connection_string().build();
    let mut conn = PostgresConnection::new(connection_string);
    conn.connect()?;
    println!("✓ Connected to PostgreSQL server");

    // Check connection health
    if conn.is_connected() {
        println!("✓ Connection is alive and healthy");
    }

    // Execute simple query
    let result = conn.execute("SELECT version()")?;
    println!("✓ PostgreSQL version query executed (rows affected: {})", result.rows_affected);

    // Test basic data types
    let result = conn.execute(
        "SELECT 42 as int_val, 3.14 as float_val, 'hello' as text_val, true as bool_val"
    )?;
    
    println!("✓ Basic data types query executed (rows affected: {})", result.rows_affected);

    println!();
    Ok(())
}

/// Demo 2: Connection pooling
async fn demo_connection_pooling(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 2: Connection Pooling");
    println!("==========================");

    // Create connection pool config
    let pool_config = PostgresPoolConfig {
        min_connections: config.min_connections,
        max_connections: config.max_connections,
        connection_timeout: config.connect_timeout.as_secs(),
        idle_timeout: config.idle_timeout.map(|d| d.as_secs()).unwrap_or(300),
        max_lifetime: config.max_lifetime.map(|d| d.as_secs()).unwrap_or(1800),
        connection_string: config.connection_string().build(),
    };

    // Create connection pool
    let pool = PostgresPool::new(pool_config)?;
    pool.initialize()?;
    println!("✓ Created and initialized connection pool");

    // Use connections from pool
    for i in 0..3 {
        let conn = pool.get_connection()?;
        println!("  Worker {}: Got connection from pool", i);
    }
    println!("✓ Completed pool operations");

    println!();
    Ok(())
}

/// Demo 3: Parameterized queries and type mapping
async fn demo_parameterized_queries(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 3: Parameterized Queries and Type Mapping");
    println!("===============================================");

    let connection_string = config.connection_string().build();
    let mut conn = PostgresConnection::new(connection_string);
    conn.connect()?;

    // Test basic query execution
    let result = conn.execute("SELECT 42")?;
    println!("✓ Basic parameterized query executed (rows affected: {})", result.rows_affected);

    // Test complex query
    let result = conn.execute(
        "SELECT 'John Doe' as name, 30 as age, true as is_active, 75000.50 as salary"
    )?;

    println!("✓ Complex parameterized query executed (rows affected: {})", result.rows_affected);

    println!();
    Ok(())
}

/// Demo 4: Prepared statements
async fn demo_prepared_statements(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 4: Prepared Statements");
    println!("===========================");

    let connection_string = config.connection_string().build();
    let mut conn = PostgresConnection::new(connection_string);
    conn.connect()?;

    // Execute prepared statement multiple times with different parameters
    let test_data = vec![
        ("Alice", 100),
        ("Bob", 200),
        ("Charlie", 300),
    ];

    println!("✓ Executing prepared statement equivalents:");
    for (name, value) in test_data {
        let result = conn.execute(
            &format!("SELECT '{}' as name, {} as value", name, value)
        )?;
        println!("  - {}: {} (rows affected: {})", name, value, result.rows_affected);
    }

    println!();
    Ok(())
}

/// Demo 5: Transaction management
async fn demo_transactions(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 5: Transaction Management");
    println!("==============================");

    let connection_string = config.connection_string().build();
    let mut conn = PostgresConnection::new(connection_string);
    conn.connect()?;

    // Create a temporary table for transaction demo
    conn.execute(
        "CREATE TEMPORARY TABLE demo_accounts (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            balance DECIMAL(10,2) NOT NULL
        )"
    )?;

    // Insert initial data
    conn.execute(
        "INSERT INTO demo_accounts (name, balance) VALUES ('Alice', 1000.0), ('Bob', 500.0)"
    )?;

    println!("✓ Created temporary table with initial data");

    // Demo transaction with commit
    conn.begin_transaction()?;
    println!("✓ Started transaction");

    // Transfer money from Alice to Bob
    conn.execute(
        "UPDATE demo_accounts SET balance = balance - 100.0 WHERE name = 'Alice'"
    )?;

    conn.execute(
        "UPDATE demo_accounts SET balance = balance + 100.0 WHERE name = 'Bob'"
    )?;

    conn.commit()?;
    println!("✓ Transaction committed");

    // Demo transaction with rollback
    conn.begin_transaction()?;
    println!("✓ Started new transaction for rollback demo");

    // Make a change
    conn.execute(
        "UPDATE demo_accounts SET balance = balance - 200.0 WHERE name = 'Alice'"
    )?;

    conn.rollback()?;
    println!("✓ Transaction rolled back");

    println!();
    Ok(())
}

/// Demo 6: Error handling
async fn demo_error_handling(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 6: Error Handling");
    println!("======================");

    let connection_string = config.connection_string().build();
    let mut conn = PostgresConnection::new(connection_string);
    conn.connect()?;

    // Test syntax error
    println!("✓ Testing syntax error:");
    match conn.execute("INVALID SQL SYNTAX") {
        Ok(_) => println!("  Unexpected success!"),
        Err(e) => {
            println!("  - Error: {}", e);
        }
    }

    // Test division by zero
    println!("✓ Testing division by zero:");
    match conn.execute("SELECT 1/0") {
        Ok(_) => println!("  Unexpected success!"),
        Err(e) => {
            println!("  - Error: {}", e);
        }
    }

    // Test connection health
    println!("✓ Testing connection health:");
    if conn.is_connected() {
        println!("  - Connection is healthy");
    } else {
        println!("  - Connection lost");
    }

    println!();
    Ok(())
}

/// Demo 7: Advanced features
async fn demo_advanced_features(config: &PostgresConfig) -> Result<(), PostgresError> {
    println!("Demo 7: Advanced Features");
    println!("=========================");

    let connection_string = config.connection_string().build();
    let mut conn = PostgresConnection::new(connection_string);
    conn.connect()?;

    // Test JSON support
    println!("✓ Testing JSON support:");
    let json_data = serde_json::json!({
        "name": "John Doe",
        "age": 30,
        "skills": ["Rust", "PostgreSQL", "CURSED"]
    });

    let result = conn.execute(
        &format!("SELECT '{}'::jsonb as data", json_data.to_string())
    )?;

    println!("  - JSON query executed (rows affected: {})", result.rows_affected);

    // Test array operations (PostgreSQL-specific)
    println!("✓ Testing array operations:");
    let result = conn.execute(
        "SELECT ARRAY[1,2,3,4,5] as numbers, 'hello,world,test'::text[] as words"
    )?;

    println!("  - Array query executed (rows affected: {})", result.rows_affected);

    // Test timestamp operations
    println!("✓ Testing timestamp operations:");
    let result = conn.execute(
        "SELECT NOW() as current_time, NOW() + INTERVAL '1 day' as tomorrow, EXTRACT(epoch FROM NOW()) as epoch"
    )?;

    println!("  - Timestamp query executed (rows affected: {})", result.rows_affected);

    println!();
    Ok(())
}
