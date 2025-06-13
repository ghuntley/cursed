# PostgreSQL Database Driver for CURSED

The CURSED PostgreSQL driver provides production-ready database connectivity with comprehensive features for building robust applications.

## Features

### Core Functionality
- **Full PostgreSQL Support**: Compatible with PostgreSQL 9.6+ including latest versions
- **Async/Await Support**: Built on tokio-postgres for high-performance async operations
- **Connection Pooling**: bb8-based connection pooling with health monitoring
- **Prepared Statements**: Efficient query execution with parameter binding
- **Transaction Management**: Full transaction support with savepoints and isolation levels
- **Type Safety**: Comprehensive type mapping between PostgreSQL and CURSED types

### Advanced Features
- **SSL/TLS Support**: Secure connections with configurable SSL modes
- **Connection Health Monitoring**: Automatic health checks and reconnection
- **Query Timeout Management**: Configurable timeouts for connections and queries
- **Error Recovery**: Intelligent error handling with retry mechanisms
- **Statistics and Monitoring**: Comprehensive metrics for performance analysis
- **Concurrent Operations**: Thread-safe operations with connection sharing

## Quick Start

### Basic Usage

```rust
use cursed::stdlib::database::postgres::{PostgresConfig, PostgresConnection};
use cursed::stdlib::database::SqlValue;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = PostgresConfig::new("localhost", 5432, "mydb", "user")
        .with_password("password")
        .with_ssl_mode(SslMode::Prefer);

    // Connect to database
    let mut conn = PostgresConnection::new(config).await?;

    // Execute query
    let result = conn.execute_query(
        "SELECT name, age FROM users WHERE id = $1",
        &[SqlValue::Integer(123)]
    ).await?;

    // Process results
    for row in result.rows {
        println!("Name: {:?}, Age: {:?}", row[0], row[1]);
    }

    Ok(())
}
```

### Connection Pooling

```rust
use cursed::stdlib::database::postgres::{PostgresConfig, PostgresPool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = PostgresConfig::new("localhost", 5432, "mydb", "user")
        .with_password("password")
        .with_pool_limits(10, 100); // min: 10, max: 100 connections

    // Create connection pool
    let pool = PostgresPool::new(config).await?;

    // Get connection from pool
    let conn = pool.get_connection().await?;
    
    // Use connection
    let result = conn.query("SELECT version()", &[]).await?;
    
    // Connection automatically returned to pool when dropped
    Ok(())
}
```

### Using the Driver Builder

```rust
use cursed::stdlib::database::postgres::{PostgresDriverBuilder, SslMode};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = PostgresDriverBuilder::new()
        .connection("localhost", 5432, "mydb", "user")
        .password("secret")
        .ssl_mode(SslMode::Require)
        .timeouts(Duration::from_secs(10), Duration::from_secs(60))
        .with_pool()
        .build()
        .await?;

    // Use driver...
    Ok(())
}
```

## Configuration

### Connection String Formats

#### URL Format
```
postgresql://user:password@host:port/database?param=value
postgres://user:password@host:port/database?sslmode=require
```

#### Key-Value Format
```
host=localhost port=5432 dbname=mydb user=myuser password=mypass sslmode=prefer
```

### Configuration Options

```rust
use cursed::stdlib::database::postgres::{PostgresConfig, SslMode};
use std::time::Duration;

let config = PostgresConfig {
    host: "localhost".to_string(),
    port: 5432,
    database: "mydb".to_string(),
    username: "user".to_string(),
    password: Some("password".to_string()),
    ssl_mode: SslMode::Prefer,
    connect_timeout: Duration::from_secs(30),
    query_timeout: Duration::from_secs(300),
    application_name: "My CURSED App".to_string(),
    max_connections: 100,
    min_connections: 10,
    max_lifetime: Some(Duration::from_secs(3600)),
    idle_timeout: Some(Duration::from_secs(600)),
    retry_attempts: 3,
    retry_delay: Duration::from_secs(1),
};
```

### SSL/TLS Modes

| Mode | Description |
|------|-------------|
| `Disable` | No SSL connection |
| `Allow` | SSL if available, plain otherwise |
| `Prefer` | Prefer SSL, fallback to plain |
| `Require` | Require SSL connection |
| `VerifyCa` | Require SSL + verify CA |
| `VerifyFull` | Require SSL + verify CA + hostname |

## Transaction Management

### Basic Transactions

```rust
let mut tx = conn.begin_transaction(TxOptions {
    isolation_level: Some(SqlIsolationLevel::LevelReadCommitted),
    read_only: false,
}).await?;

// Execute queries within transaction
tx.execute("INSERT INTO users (name) VALUES ($1)", 
          &[SqlValue::String("John".to_string())]).await?;

// Commit transaction
tx.commit().await?;
```

### Savepoints

```rust
let mut tx = conn.begin_transaction(TxOptions::default()).await?;

// Create savepoint
let savepoint = tx.savepoint("my_savepoint").await?;

// Execute some operations
tx.execute("UPDATE accounts SET balance = balance - 100 WHERE id = 1", &[]).await?;

// Rollback to savepoint if needed
tx.rollback_to_savepoint(&savepoint).await?;

// Or release savepoint
tx.release_savepoint(&savepoint).await?;

tx.commit().await?;
```

## Type Mapping

| PostgreSQL Type | CURSED SqlValue | Notes |
|----------------|------------------|-------|
| `BOOLEAN` | `SqlValue::Boolean` | |
| `SMALLINT`, `INTEGER`, `BIGINT` | `SqlValue::Integer` | Converted to i64 |
| `REAL`, `DOUBLE PRECISION` | `SqlValue::Float` | |
| `NUMERIC`, `DECIMAL` | `SqlValue::Float` | Precision may be lost |
| `TEXT`, `VARCHAR`, `CHAR` | `SqlValue::String` | |
| `BYTEA` | `SqlValue::Bytes` | |
| `TIMESTAMP`, `TIMESTAMPTZ` | `SqlValue::Timestamp` | |
| `JSON`, `JSONB` | `SqlValue::Json` | |
| `NULL` | `SqlValue::Null` | |

## Error Handling

### Error Types

The driver provides comprehensive error handling with specific error types:

```rust
use cursed::stdlib::database::postgres::{PostgresError, PostgresErrorKind};

match result {
    Ok(data) => { /* handle success */ },
    Err(PostgresError { kind, message, sqlstate, .. }) => {
        match kind {
            PostgresErrorKind::ConnectionFailed => {
                // Handle connection errors - may be retryable
                if error.is_retryable() {
                    // Implement retry logic
                }
            },
            PostgresErrorKind::SyntaxError => {
                // Handle SQL syntax errors
                println!("SQL Error: {}", message);
            },
            PostgresErrorKind::ConstraintViolation => {
                // Handle constraint violations
                if let Some(constraint) = error.constraint {
                    println!("Constraint '{}' violated", constraint);
                }
            },
            // ... other error types
        }
    }
}
```

### Error Recovery

```rust
async fn execute_with_retry<F, T>(
    mut operation: F,
    max_retries: u32,
) -> Result<T, PostgresError>
where
    F: FnMut() -> Result<T, PostgresError>,
{
    let mut retries = 0;
    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(error) => {
                if error.is_retryable() && retries < max_retries {
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(100 * retries as u64)).await;
                    continue;
                }
                return Err(error);
            }
        }
    }
}
```

## Performance and Monitoring

### Connection Pool Monitoring

```rust
let pool = PostgresPool::new(config).await?;

// Get pool health
let health = pool.get_health();
println!("Pool Health: {:.1}%", health.health_score * 100.0);
println!("Active Connections: {}/{}", health.active_connections, health.max_connections);

// Get detailed statistics
let stats = pool.get_statistics();
println!("Total Checkouts: {}", stats.total_checkouts);
println!("Average Checkout Time: {:.2}ms", stats.avg_checkout_time_ms);
println!("Checkout Success Rate: {:.1}%", 
         (stats.total_checkouts - stats.total_checkout_failures) as f64 / 
         stats.total_checkouts as f64 * 100.0);
```

### Connection Statistics

```rust
let stats = conn.get_stats();
println!("Queries Executed: {}", stats.queries_executed);
println!("Statements Prepared: {}", stats.statements_prepared);
println!("Transactions Started: {}", stats.transactions_started);
println!("Errors Encountered: {}", stats.errors_encountered);
```

## Best Practices

### Connection Management

1. **Use Connection Pooling**: For applications with multiple concurrent operations
2. **Configure Timeouts**: Set appropriate connection and query timeouts
3. **Monitor Health**: Regularly check pool health and connection statistics
4. **Handle Errors Gracefully**: Implement proper error handling and retry logic

### Query Optimization

1. **Use Prepared Statements**: For frequently executed queries
2. **Parameter Binding**: Always use parameterized queries to prevent SQL injection
3. **Batch Operations**: Group related operations in transactions
4. **Monitor Performance**: Track query execution times and optimize slow queries

### Security

1. **Use SSL/TLS**: Enable secure connections in production
2. **Credential Management**: Store credentials securely, use environment variables
3. **Connection Limits**: Configure appropriate connection pool limits
4. **Input Validation**: Validate all user inputs before database operations

## Integration with CURSED Database System

The PostgreSQL driver integrates seamlessly with the CURSED database abstraction layer:

```rust
use cursed::stdlib::database::{Driver, DriverRegistry};
use cursed::stdlib::database::postgres::init_postgres;

// Initialize PostgreSQL driver
init_postgres()?;

// Use through generic database interface
let driver = DriverRegistry::get("postgres")?;
let conn = driver.open("postgresql://user:pass@localhost/db")?;

// Use standard database operations
let result = conn.query("SELECT * FROM users", &[])?;
```

## Examples

See `examples/postgres_demo.rs` for a comprehensive demonstration of all driver features.

## Testing

Run the integration tests with a PostgreSQL server:

```bash
# Set environment variables for test database
export POSTGRES_HOST=localhost
export POSTGRES_PORT=5432
export POSTGRES_DB=test
export POSTGRES_USER=postgres
export POSTGRES_PASSWORD=

# Run tests
cargo test postgres_integration_test
```

## Troubleshooting

### Common Issues

1. **Connection Refused**: Ensure PostgreSQL server is running and accessible
2. **Authentication Failed**: Check username/password and pg_hba.conf settings
3. **SSL Errors**: Verify SSL configuration and certificates
4. **Timeout Errors**: Increase timeout values or check network connectivity
5. **Pool Exhaustion**: Increase pool size or reduce connection holding time

### Debug Logging

Enable debug logging to troubleshoot issues:

```rust
env_logger::init();
log::debug!("Connection attempt to {}:{}", config.host, config.port);
```

Set `RUST_LOG=debug` environment variable to see detailed logs.

## Dependencies

The PostgreSQL driver relies on these high-quality Rust crates:

- `tokio-postgres`: Async PostgreSQL client
- `postgres-types`: PostgreSQL type system
- `bb8` + `bb8-postgres`: Connection pooling
- `uuid`: Connection ID generation
- `url`: Connection string parsing
- `serde_json`: JSON type support
- `chrono`: Date/time handling

## Version Compatibility

| CURSED Version | PostgreSQL Versions | Rust Version |
|----------------|---------------------|--------------|
| 0.1.x | 9.6, 10, 11, 12, 13, 14, 15, 16 | 1.70+ |

## License

This PostgreSQL driver is part of the CURSED programming language and is licensed under the MIT License.
