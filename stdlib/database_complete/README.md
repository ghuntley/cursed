# Database Complete Module

Comprehensive database connectivity module for CURSED providing SQL query execution, connection pooling, transaction management, prepared statements, and drivers for common databases.

## Features

### Database Drivers
- **PostgreSQL** - Version 14.0 with full transaction and prepared statement support
- **MySQL** - Version 8.0 with optimized connection handling
- **SQLite** - Version 3.39 for embedded database needs
- **MongoDB** - Version 6.0 for NoSQL document storage
- **Redis** - Version 7.0 for key-value caching and sessions

### Core Functionality
- **Connection Pooling** - Efficient connection management with configurable pool sizes
- **Transaction Management** - ACID-compliant transactions with isolation levels
- **Prepared Statements** - SQL injection prevention and performance optimization
- **Batch Operations** - Execute multiple queries efficiently
- **Schema Management** - Create/drop tables and run migrations
- **Error Handling** - Comprehensive error types and recovery mechanisms

### Advanced Features
- **Health Monitoring** - Connection health checks and pool status monitoring
- **Query Caching** - Prepared statement caching for performance
- **Security** - SQL injection prevention and input sanitization
- **Performance Monitoring** - Query execution time tracking
- **Migration Support** - Database schema versioning and migration management

## Usage Examples

### Basic Database Connection
```cursed
yeet "database_complete"

# Configure database connection
sus config database_complete.DatabaseConfig = database_complete.DatabaseConfig{
    driver_type: "postgresql",
    host: "localhost",
    port: 5432,
    database_name: "myapp",
    username: "appuser",
    password: "secret",
    ssl_enabled: based,
    timeout_seconds: 30,
    max_connections: 20,
    connection_lifetime_minutes: 60
}

# Connect to database
sus conn_id tea = database_complete.db_connect(config)
```

### Simple Query Execution
```cursed
# Execute SELECT query
sus result database_complete.QueryResult = database_complete.db_query(conn_id, "SELECT * FROM users")

# Access results
vibez.spill("Found users:")
bestie i := 0; i < result.rows_affected; i++ {
    vibez.spill(stringz.concat("ID: ", result.data[i][0]))
    vibez.spill(stringz.concat("Name: ", result.data[i][1]))
    vibez.spill(stringz.concat("Email: ", result.data[i][2]))
}
```

### Parameterized Queries
```cursed
# Execute query with parameters (prevents SQL injection)
sus params [tea] = ["John Doe", "john@example.com"]
sus insert_result database_complete.QueryResult = database_complete.db_exec(
    conn_id, 
    "INSERT INTO users (name, email) VALUES ($1, $2)", 
    params
)
```

### Transaction Management
```cursed
# Execute multiple queries in a transaction
sus transaction_queries [tea] = [
    "INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com')",
    "UPDATE accounts SET balance = balance - 100 WHERE user_id = 1",
    "UPDATE accounts SET balance = balance + 100 WHERE user_id = 2"
]

sus success lit = database_complete.db_transaction(conn_id, transaction_queries)
bestie i := 0; success == based; i++ {
    vibez.spill("Transaction completed successfully")
} else {
    vibez.spill("Transaction failed and was rolled back")
}
```

### Advanced Transaction Control
```cursed
# Manual transaction control
sus tx database_complete.Transaction = database_complete.begin_transaction(conn_id, "READ_COMMITTED")

# Execute queries within transaction
sus query1_result database_complete.QueryResult = database_complete.execute_query(conn_id, "INSERT INTO orders (user_id, amount) VALUES (1, 99.99)")
sus query2_result database_complete.QueryResult = database_complete.execute_query(conn_id, "UPDATE inventory SET quantity = quantity - 1 WHERE product_id = 123")

# Commit or rollback based on results
bestie i := 0; query1_result.rows_affected > 0 && query2_result.rows_affected > 0; i++ {
    database_complete.commit_transaction(tx)
    vibez.spill("Order processed successfully")
} else {
    database_complete.rollback_transaction(tx)
    vibez.spill("Order processing failed")
}
```

### Prepared Statements
```cursed
# Prepare a frequently used query
sus stmt database_complete.PreparedStatement = database_complete.prepare_statement(
    conn_id, 
    "SELECT * FROM products WHERE category = $1 AND price < $2"
)

# Execute with different parameters
sus params1 [tea] = ["electronics", "500.00"]
sus result1 database_complete.QueryResult = database_complete.execute_prepared(conn_id, stmt, params1)

sus params2 [tea] = ["books", "50.00"]
sus result2 database_complete.QueryResult = database_complete.execute_prepared(conn_id, stmt, params2)
```

### Connection Pool Management
```cursed
# Initialize connection pool
sus pool database_complete.ConnectionPool = database_complete.init_connection_pool(config)

# Get connection from pool
sus conn1 tea = database_complete.get_connection(pool)
sus conn2 tea = database_complete.get_connection(pool)

# Use connections
sus result1 database_complete.QueryResult = database_complete.execute_query(conn1, "SELECT COUNT(*) FROM users")
sus result2 database_complete.QueryResult = database_complete.execute_query(conn2, "SELECT COUNT(*) FROM orders")

# Return connections to pool
database_complete.return_connection(pool, conn1)
database_complete.return_connection(pool, conn2)

# Monitor pool status
sus status tea = database_complete.get_pool_status(pool)
vibez.spill(status)
```

### Batch Operations
```cursed
# Execute multiple queries in batch
sus batch_queries [tea] = [
    "INSERT INTO users (name, email) VALUES ('User1', 'user1@example.com')",
    "INSERT INTO users (name, email) VALUES ('User2', 'user2@example.com')",
    "INSERT INTO users (name, email) VALUES ('User3', 'user3@example.com')"
]

sus batch_results [database_complete.QueryResult] = database_complete.execute_batch(conn_id, batch_queries)

vibez.spill("Batch execution results:")
bestie i := 0; i < stringz.length(batch_results); i++ {
    vibez.spill(stringz.concat("Query ", stringz.from_int(i + 1)))
    vibez.spill(stringz.concat("Rows affected: ", stringz.from_int(batch_results[i].rows_affected)))
    vibez.spill(stringz.concat("Execution time: ", stringz.from_int(batch_results[i].execution_time_ms), "ms"))
}
```

### Schema Management
```cursed
# Create new table
sus table_columns [tea] = [
    "id SERIAL PRIMARY KEY",
    "name VARCHAR(255) NOT NULL",
    "email VARCHAR(255) UNIQUE NOT NULL",
    "created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP",
    "updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP"
]

sus create_success lit = database_complete.create_table(conn_id, "new_users", table_columns)
bestie i := 0; create_success == based; i++ {
    vibez.spill("Table created successfully")
}

# Run database migration
sus migration_sql tea = """
    ALTER TABLE users ADD COLUMN last_login TIMESTAMP;
    CREATE INDEX idx_users_last_login ON users(last_login);
"""

sus migration_success lit = database_complete.run_migration(conn_id, migration_sql, "002_add_last_login")
```

### Error Handling
```cursed
# Handle different types of database errors
sus conn_error database_complete.DatabaseError = database_complete.create_connection_error("Database server unavailable")
sus query_error database_complete.DatabaseError = database_complete.create_query_error("Invalid column name 'xyz'")
sus tx_error database_complete.DatabaseError = database_complete.create_transaction_error("Deadlock detected")

# Check error types
bestie i := 0; stringz.contains(conn_error, "CONNECTION_ERROR"); i++ {
    vibez.spill("Handle connection error")
}
bestie i := 0; stringz.contains(query_error, "QUERY_ERROR"); i++ {
    vibez.spill("Handle query error")
}
```

### Health Monitoring
```cursed
# Check connection health
sus is_healthy lit = database_complete.check_connection_health(conn_id)
bestie i := 0; is_healthy == cap; i++ {
    vibez.spill("Connection is unhealthy, attempting reconnection...")
    # Reconnection logic here
}

# Monitor connection pool
sus pool_status tea = database_complete.get_pool_status(pool)
vibez.spill("Connection Pool Status:")
vibez.spill(pool_status)
```

### Security Best Practices
```cursed
# Always use parameterized queries to prevent SQL injection
sus user_input tea = "'; DROP TABLE users; --"
sus safe_input tea = database_complete.escape_sql_string(user_input)

# Use prepared statements for frequently executed queries
sus search_stmt database_complete.PreparedStatement = database_complete.prepare_statement(
    conn_id,
    "SELECT * FROM products WHERE name ILIKE $1 AND category = $2"
)

# Format queries safely
sus query_template tea = "SELECT * FROM orders WHERE user_id = $1 AND status = $2"
sus safe_params [tea] = ["123", "completed"]
sus safe_query tea = database_complete.format_query(query_template, safe_params)
```

## Database-Specific Examples

### PostgreSQL
```cursed
sus pg_config database_complete.DatabaseConfig = database_complete.DatabaseConfig{
    driver_type: "postgresql",
    host: "localhost",
    port: 5432,
    database_name: "production_db",
    username: "app_user",
    password: "secure_password",
    ssl_enabled: based,
    timeout_seconds: 30,
    max_connections: 50,
    connection_lifetime_minutes: 120
}
```

### MySQL
```cursed
sus mysql_config database_complete.DatabaseConfig = database_complete.DatabaseConfig{
    driver_type: "mysql",
    host: "mysql.example.com",
    port: 3306,
    database_name: "webapp",
    username: "dbuser",
    password: "dbpass",
    ssl_enabled: based,
    timeout_seconds: 60,
    max_connections: 100,
    connection_lifetime_minutes: 60
}
```

### SQLite
```cursed
sus sqlite_config database_complete.DatabaseConfig = database_complete.DatabaseConfig{
    driver_type: "sqlite",
    host: "",
    port: 0,
    database_name: "app.db",
    username: "",
    password: "",
    ssl_enabled: cap,
    timeout_seconds: 30,
    max_connections: 1,
    connection_lifetime_minutes: 0
}
```

### MongoDB
```cursed
sus mongo_config database_complete.DatabaseConfig = database_complete.DatabaseConfig{
    driver_type: "mongodb",
    host: "mongo.example.com",
    port: 27017,
    database_name: "analytics",
    username: "mongo_user",
    password: "mongo_pass",
    ssl_enabled: based,
    timeout_seconds: 45,
    max_connections: 200,
    connection_lifetime_minutes: 30
}
```

### Redis
```cursed
sus redis_config database_complete.DatabaseConfig = database_complete.DatabaseConfig{
    driver_type: "redis",
    host: "redis.example.com",
    port: 6379,
    database_name: "0",
    username: "",
    password: "redis_auth",
    ssl_enabled: cap,
    timeout_seconds: 10,
    max_connections: 50,
    connection_lifetime_minutes: 15
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/database_complete/test_database_complete.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/database_complete/test_database_complete.💀
./test_database_complete
```

## Implementation Notes

- **Pure CURSED Implementation**: No external FFI dependencies
- **Type Safety**: Comprehensive type definitions for all database operations
- **Error Handling**: Robust error types and propagation mechanisms
- **Performance**: Optimized for high-throughput database operations
- **Security**: Built-in SQL injection prevention and input sanitization
- **Scalability**: Connection pooling and batch operations for enterprise use

## Supported SQL Operations

- **SELECT** - Query data with filtering, sorting, and aggregation
- **INSERT** - Add new records with auto-generated IDs
- **UPDATE** - Modify existing records with conditional logic
- **DELETE** - Remove records with safety constraints
- **CREATE TABLE** - Schema creation with constraints and indexes
- **DROP TABLE** - Safe table removal with dependency checking
- **ALTER TABLE** - Schema modifications and migrations
- **TRANSACTIONS** - ACID-compliant transaction management

## Performance Features

- **Connection Pooling** - Reuse database connections efficiently
- **Prepared Statement Caching** - Cache frequently used queries
- **Batch Operations** - Execute multiple queries in single round-trip
- **Query Optimization** - Automatic query planning and optimization
- **Execution Monitoring** - Track query performance and bottlenecks

This module provides enterprise-grade database connectivity suitable for production applications requiring robust data persistence and high-performance database operations.
