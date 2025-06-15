# MySQL Production Driver for CURSED

## Overview

The MySQL Production Driver is a comprehensive, production-ready database driver implementation for the CURSED programming language. It provides full MySQL connectivity with advanced features including connection pooling, prepared statements, transaction management, and comprehensive security measures.

## Features

### Core Database Operations
- **Connection Management**: Advanced connection pooling with configurable limits and lifecycle management
- **Prepared Statements**: Full support for prepared statements with parameter binding and type safety
- **Transactions**: Complete ACID transaction support with isolation levels and rollback capabilities
- **Query Execution**: Efficient query execution with result processing and error handling

### Security Features
- **SQL Injection Prevention**: Comprehensive SQL sanitization and validation
- **SSL/TLS Support**: Multiple SSL modes including certificate verification
- **Authentication**: Secure authentication with credential management
- **Parameter Binding**: Safe parameter binding to prevent injection attacks

### Performance Optimizations
- **Connection Pooling**: Efficient connection reuse with configurable pool sizes
- **Statement Caching**: Prepared statement caching for improved performance
- **Type Conversions**: Optimized type conversions between CURSED and MySQL types
- **Concurrent Operations**: Thread-safe operations with minimal contention

### Monitoring and Diagnostics
- **Health Monitoring**: Comprehensive health checks and status reporting
- **Performance Metrics**: Detailed statistics and performance monitoring
- **Error Reporting**: Enhanced error reporting with context and recovery information
- **Logging**: Structured logging with configurable levels

## Architecture

### Driver Components

```
ProductionMySqlDriver
├── Configuration Management (ProductionMySqlConfig)
├── Connection Pool (mysql::Pool)
├── Statement Cache (HashMap<String, Vec<u8>>)
├── Statistics Tracking (ProductionPoolStats)
└── Health Monitoring (DriverHealthReport)

ProductionMySqlConnection
├── Pooled Connection (PooledConn)
├── Connection Metadata
├── Transaction State
└── Driver Reference

ProductionMySqlStatement
├── Connection Reference
├── Query String
├── Parameter Information
└── Statistics Tracking

ProductionMySqlTransaction
├── Connection Reference
├── Transaction State
├── Driver Reference
└── Lifecycle Management
```

### Security Architecture

```
SQL Input → SqlSanitizer → Parameter Binding → Prepared Statement → MySQL Server
    ↓            ↓              ↓                    ↓
Validation   Identifier     Type Safety        Execution
             Escaping
```

## Configuration

### Basic Configuration

```rust
use cursed::stdlib::database::mysql::production_driver::{
    ProductionMySqlConfig, SslMode, create_production_mysql_driver_with_config
};

let config = ProductionMySqlConfig {
    host: "localhost".to_string(),
    port: 3306,
    username: "app_user".to_string(),
    password: "secure_password".to_string(),
    database: "production_db".to_string(),
    
    // Connection pool settings
    min_connections: 5,
    max_connections: 100,
    connection_timeout: Duration::from_secs(30),
    idle_timeout: Duration::from_secs(600),
    max_lifetime: Duration::from_secs(3600),
    
    // Security settings
    ssl_mode: SslMode::Required,
    ssl_ca_path: Some("/path/to/ca.pem".to_string()),
    verify_ssl: true,
    
    // Performance settings
    statement_cache_size: 1000,
    query_timeout: Duration::from_secs(300),
    
    // MySQL specific settings
    charset: "utf8mb4".to_string(),
    collation: "utf8mb4_unicode_ci".to_string(),
    timezone: "UTC".to_string(),
    
    ..ProductionMySqlConfig::default()
};

let driver = create_production_mysql_driver_with_config(config);
```

### SSL/TLS Configuration

The driver supports multiple SSL modes for secure connections:

- **`SslMode::Disabled`**: No SSL encryption
- **`SslMode::Preferred`**: Use SSL if available, fallback to unencrypted
- **`SslMode::Required`**: Require SSL encryption
- **`SslMode::VerifyCA`**: Require SSL and verify server certificate
- **`SslMode::VerifyIdentity`**: Full certificate verification including hostname

```rust
let secure_config = ProductionMySqlConfig {
    ssl_mode: SslMode::VerifyIdentity,
    ssl_ca_path: Some("/etc/ssl/certs/mysql-ca.pem".to_string()),
    ssl_cert_path: Some("/etc/ssl/certs/mysql-client.pem".to_string()),
    ssl_key_path: Some("/etc/ssl/private/mysql-client.key".to_string()),
    verify_ssl: true,
    ..ProductionMySqlConfig::default()
};
```

## Usage Examples

### Basic Database Operations

```rust
use cursed::stdlib::database::mysql::production_driver::*;
use cursed::stdlib::database::{SqlValue, TxOptions};

// Initialize driver
let driver = create_production_mysql_driver();
driver.initialize()?;

// Get connection
let conn = driver.get_connection()?;

// Simple query
let result = conn.query(
    "SELECT id, name, email FROM users WHERE active = ?",
    &[SqlValue::Boolean(true)]
)?;

for row in result.rows {
    println!("User: {} - {}", row[1], row[2]);
}

// Execute statement
let affected = conn.execute(
    "UPDATE users SET last_login = NOW() WHERE id = ?",
    &[SqlValue::Integer(user_id)]
)?;

println!("Updated {} rows", affected.rows_affected);
```

### Prepared Statements

```rust
// Prepare statement
let mut stmt = conn.prepare(
    "INSERT INTO orders (user_id, product_id, quantity, price) VALUES (?, ?, ?, ?)"
)?;

// Execute with parameters
let result = stmt.execute(&[
    SqlValue::Integer(user_id),
    SqlValue::Integer(product_id),
    SqlValue::Integer(quantity),
    SqlValue::Float(price),
])?;

println!("Created order with ID: {}", result.last_insert_id.unwrap());
```

### Transaction Management

```rust
// Begin transaction with specific isolation level
let tx_opts = TxOptions {
    isolation_level: Some(SqlIsolationLevel::LevelSerializable),
    read_only: false,
};

let mut tx = conn.begin_transaction(tx_opts)?;

// Perform operations within transaction
let order_result = conn.execute(
    "INSERT INTO orders (user_id, total) VALUES (?, ?)",
    &[SqlValue::Integer(user_id), SqlValue::Float(total)]
)?;

let order_id = order_result.last_insert_id.unwrap();

// Add order items
for item in order_items {
    conn.execute(
        "INSERT INTO order_items (order_id, product_id, quantity) VALUES (?, ?, ?)",
        &[SqlValue::Integer(order_id), SqlValue::Integer(item.product_id), SqlValue::Integer(item.quantity)]
    )?;
}

// Commit transaction
tx.commit()?;
```

### Error Handling

```rust
use cursed::stdlib::database::{DatabaseError, DatabaseErrorKind};

match conn.query("SELECT * FROM users", &[]) {
    Ok(result) => {
        // Process successful result
        for row in result.rows {
            // Handle each row
        }
    }
    Err(e) => {
        match e.kind() {
            DatabaseErrorKind::ConnectionError => {
                eprintln!("Connection failed: {}", e.message());
                // Attempt reconnection
            }
            DatabaseErrorKind::QueryError => {
                eprintln!("Query failed: {}", e.message());
                // Log query error and continue
            }
            DatabaseErrorKind::TransactionError => {
                eprintln!("Transaction failed: {}", e.message());
                // Rollback and retry
            }
            _ => {
                eprintln!("Database error: {}", e);
                // Generic error handling
            }
        }
    }
}
```

## Security Considerations

### SQL Injection Prevention

The driver includes comprehensive SQL injection prevention mechanisms:

```rust
use cursed::stdlib::database::mysql::production_driver::SqlSanitizer;

// Validate and sanitize identifiers
let safe_table_name = SqlSanitizer::sanitize_identifier("user_data")?;
// Result: `user_data`

// Query validation
SqlSanitizer::validate_query("SELECT * FROM users WHERE id = ?")?;
// Validates query structure and warns about suspicious patterns

// Always use parameter binding
let result = conn.query(
    "SELECT * FROM users WHERE email = ? AND status = ?",
    &[SqlValue::String(email), SqlValue::String(status)]
)?;
// Parameters are safely bound, preventing injection
```

### Secure Configuration

```rust
let secure_config = ProductionMySqlConfig {
    // Use strong SSL configuration
    ssl_mode: SslMode::VerifyIdentity,
    verify_ssl: true,
    
    // Enable security features
    foreign_key_checks: true,
    sql_mode: "STRICT_TRANS_TABLES,NO_ZERO_DATE,NO_ZERO_IN_DATE,ERROR_FOR_DIVISION_BY_ZERO".to_string(),
    
    // Secure connection limits
    max_connections: 50, // Limit concurrent connections
    connection_timeout: Duration::from_secs(30),
    query_timeout: Duration::from_secs(60),
    
    ..ProductionMySqlConfig::default()
};
```

## Performance Tuning

### Connection Pool Optimization

```rust
let performance_config = ProductionMySqlConfig {
    // Optimize pool size for workload
    min_connections: 10,        // Keep minimum connections ready
    max_connections: 200,       // Allow scaling under load
    connection_timeout: Duration::from_secs(10),  // Quick timeout for responsiveness
    idle_timeout: Duration::from_secs(300),       // Recycle idle connections
    max_lifetime: Duration::from_secs(1800),      // Refresh connections regularly
    
    // Enable performance features
    statement_cache_size: 2000, // Cache more prepared statements
    enable_compression: true,   // Reduce network overhead
    binary_protocol: true,      // Use efficient binary protocol
    
    ..ProductionMySqlConfig::default()
};
```

### Monitoring Performance

```rust
// Get driver statistics
let stats = driver.get_stats()?;
println!("Active connections: {}", stats.active_connections);
println!("Query success rate: {:.2}%", 
    stats.successful_queries as f64 / (stats.successful_queries + stats.failed_queries) as f64 * 100.0
);
println!("Average query time: {:?}", stats.average_query_time);

// Health check
let health = driver.health_check()?;
if !health.overall_health {
    eprintln!("Driver health issues detected:");
    eprintln!("  Pool initialized: {}", health.pool_initialized);
    eprintln!("  Connectivity: {}", health.connectivity);
    eprintln!("  Connection errors: {}", health.connection_errors);
}
```

## Type Conversions

The driver provides comprehensive type conversions between CURSED and MySQL types:

| CURSED Type | MySQL Type | Notes |
|-------------|------------|-------|
| `SqlValue::Null` | `NULL` | Represents SQL NULL values |
| `SqlValue::Boolean` | `TINYINT(1)` | MySQL boolean representation |
| `SqlValue::Integer` | `BIGINT` | 64-bit signed integer |
| `SqlValue::Float` | `DOUBLE` | Double precision floating point |
| `SqlValue::String` | `VARCHAR/TEXT` | UTF-8 encoded strings |
| `SqlValue::Bytes` | `BLOB` | Binary data |
| `SqlValue::Timestamp` | `DATETIME/TIMESTAMP` | Date and time values |
| `SqlValue::Json` | `JSON` | JSON documents (MySQL 5.7+) |

### Custom Type Handling

```rust
use cursed::stdlib::database::mysql::production_driver::{
    convert_to_mysql_value, convert_from_mysql_value
};

// Convert CURSED value to MySQL
let cursed_value = SqlValue::String("Hello, MySQL!".to_string());
let mysql_value = convert_to_mysql_value(&cursed_value)?;

// Convert MySQL value back to CURSED
let converted_back = convert_from_mysql_value(mysql_value)?;
```

## Error Handling and Recovery

### Error Categories

The driver provides detailed error categorization:

- **Connection Errors**: Network issues, authentication failures, SSL problems
- **Query Errors**: SQL syntax errors, constraint violations, data type mismatches
- **Transaction Errors**: Deadlocks, isolation failures, rollback issues
- **Configuration Errors**: Invalid settings, missing parameters
- **Timeout Errors**: Query timeouts, connection timeouts

### Automatic Recovery

```rust
// Connection retry logic
let mut retries = 3;
loop {
    match driver.get_connection() {
        Ok(conn) => {
            // Use connection
            break;
        }
        Err(e) if retries > 0 => {
            eprintln!("Connection failed, retrying: {}", e);
            retries -= 1;
            std::thread::sleep(Duration::from_millis(100));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
```

## Testing

### Running Tests

```bash
# Run all MySQL production driver tests
make mysql-production-test

# Run quick validation tests
make mysql-production-test-quick

# Run comprehensive test suite
make mysql-production-test-all

# Run tests with coverage analysis
make mysql-production-test-coverage

# Validate implementation
make mysql-production-validate
```

### Test Categories

1. **Unit Tests**: Configuration validation, type conversions, SQL sanitization
2. **Integration Tests**: End-to-end database operations with mock connections
3. **Security Tests**: SQL injection prevention, SSL configuration validation
4. **Performance Tests**: Connection pooling, statement caching, concurrent operations
5. **Error Handling Tests**: Comprehensive error scenario coverage

## Production Deployment

### Recommended Configuration

```rust
let production_config = ProductionMySqlConfig {
    // Connection settings for production
    host: env::var("MYSQL_HOST").unwrap_or_else(|_| "localhost".to_string()),
    port: env::var("MYSQL_PORT").unwrap_or_else(|_| "3306".to_string()).parse().unwrap(),
    username: env::var("MYSQL_USER").expect("MYSQL_USER environment variable required"),
    password: env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD environment variable required"),
    database: env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE environment variable required"),
    
    // Production pool settings
    min_connections: 20,
    max_connections: 100,
    connection_timeout: Duration::from_secs(30),
    idle_timeout: Duration::from_secs(600),
    max_lifetime: Duration::from_secs(3600),
    
    // Security settings
    ssl_mode: SslMode::Required,
    verify_ssl: true,
    
    // Performance settings
    statement_cache_size: 1000,
    query_timeout: Duration::from_secs(300),
    retry_attempts: 3,
    retry_delay: Duration::from_millis(100),
    
    // MySQL settings
    charset: "utf8mb4".to_string(),
    collation: "utf8mb4_unicode_ci".to_string(),
    timezone: "UTC".to_string(),
    foreign_key_checks: true,
    sql_mode: "STRICT_TRANS_TABLES,NO_ZERO_DATE,NO_ZERO_IN_DATE,ERROR_FOR_DIVISION_BY_ZERO".to_string(),
    
    ..ProductionMySqlConfig::default()
};
```

### Monitoring and Maintenance

1. **Health Checks**: Implement regular health checks using `driver.health_check()`
2. **Metrics Collection**: Monitor connection pool statistics and query performance
3. **Log Analysis**: Analyze driver logs for performance issues and errors
4. **Connection Tuning**: Adjust pool sizes based on application load patterns
5. **Security Audits**: Regular security reviews of SQL injection prevention measures

## Integration with CURSED Language

### Database Interface

The production MySQL driver integrates seamlessly with the CURSED database interface:

```cursed
import "stdlib::database::mysql";

// Initialize MySQL driver
sus mysql_driver = mysql::create_production_mysql_driver();
mysql_driver.initialize("mysql://user:pass@localhost:3306/mydb")?;

// Use with database operations
sus conn = mysql_driver.get_connection()?;
sus users = conn.query("SELECT * FROM users WHERE active = ?", [true])?;

lowkey (sus user : users.rows) {
    println("User: {}", user[1]);
}
```

### Error Integration

MySQL errors integrate with the CURSED error handling system:

```cursed
yolo {
    sus result = conn.execute("INSERT INTO users (name) VALUES (?)", [name])?;
    println("Created user with ID: {}", result.last_insert_id);
} catch (DatabaseError e) {
    match e.kind() {
        ConnectionError => println("Connection failed: {}", e.message()),
        QueryError => println("Query failed: {}", e.message()),
        _ => println("Database error: {}", e),
    }
}
```

## Troubleshooting

### Common Issues

1. **Connection Pool Exhaustion**
   - Increase `max_connections` in configuration
   - Implement connection timeout handling
   - Monitor connection lifecycle

2. **SSL/TLS Connection Issues**
   - Verify certificate paths and permissions
   - Check SSL mode compatibility with MySQL server
   - Validate certificate chain

3. **Query Performance Issues**
   - Enable statement caching
   - Optimize query timeout settings
   - Monitor query execution times

4. **Transaction Deadlocks**
   - Implement retry logic for deadlock errors
   - Use appropriate isolation levels
   - Minimize transaction duration

### Debug Mode

Enable debug logging for troubleshooting:

```rust
env::set_var("RUST_LOG", "cursed::stdlib::database::mysql=debug");
env_logger::init();
```

## API Reference

### Core Types

- `ProductionMySqlDriver`: Main driver implementation
- `ProductionMySqlConfig`: Driver configuration
- `ProductionMySqlConnection`: Database connection
- `ProductionMySqlStatement`: Prepared statement
- `ProductionMySqlTransaction`: Transaction management
- `SqlSanitizer`: SQL injection prevention utilities
- `DriverHealthReport`: Health monitoring information

### Factory Functions

- `create_production_mysql_driver()`: Create driver with default configuration
- `create_production_mysql_driver_with_config(config)`: Create driver with custom configuration

### Utility Functions

- `convert_to_mysql_value(value)`: Convert CURSED value to MySQL value
- `convert_from_mysql_value(value)`: Convert MySQL value to CURSED value

## Contributing

When contributing to the MySQL production driver:

1. Ensure all tests pass: `make mysql-production-test-all`
2. Add comprehensive test coverage for new features
3. Follow security best practices for SQL handling
4. Update documentation for configuration changes
5. Validate performance impact of modifications

## License

The MySQL Production Driver is part of the CURSED programming language and is licensed under the same terms as the main project.
