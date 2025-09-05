# CURSED Database Layer

A comprehensive database abstraction layer for CURSED with support for PostgreSQL, MySQL, and SQLite. This module provides connection management, query building, ORM functionality, and database-specific optimizations.

## Features

- **Multi-Database Support**: PostgreSQL, MySQL, and SQLite
- **Connection Pooling**: Efficient connection management with configurable pools
- **Query Builder**: Fluent API for building SQL queries
- **ORM Functionality**: Simple object-relational mapping
- **Prepared Statements**: Secure parameterized queries
- **Transaction Support**: ACID transaction management
- **Migration System**: Schema versioning and migration management
- **Database-Specific Optimizations**: Leverages unique features of each database
- **Pure CURSED Implementation**: Minimal FFI dependencies

## Quick Start

```cursed
yeet "database"

# Create database configuration
sus config DatabaseConfig = create_database_config(
    DB_POSTGRES,
    "localhost",
    5432,
    "myapp",
    "username",
    "password"
)

# Connect to database
sus conn tea = connect_database(config)

# Execute query
sus result QueryResult = execute_query(
    conn,
    "SELECT * FROM users WHERE age > ?",
    ["18"]
)

# Process results
bestie i := 0; i < result.rows.length; i++ {
    vibez.spill(stringz.format("User: {}", result.rows[i][1]))
}
```

## Database Types

The module supports three database types:

- `DB_POSTGRES` (1) - PostgreSQL
- `DB_MYSQL` (2) - MySQL/MariaDB  
- `DB_SQLITE` (3) - SQLite

## Core Structures

### DatabaseConfig

Configuration for database connections:

```cursed
be_like DatabaseConfig = {
    db_type DatabaseType
    host tea
    port normie
    database tea
    username tea
    password tea
    connection_string tea
    pool_size normie
    timeout normie
}
```

### QueryResult

Results from query execution:

```cursed
be_like QueryResult = {
    rows [][]tea           # Result rows
    columns []tea          # Column names
    affected_rows normie   # Number of affected rows
    last_insert_id tea     # Last inserted ID
    error_message tea      # Error message if any
    success lit           # Success status
}
```

## Connection Management

### Basic Connection

```cursed
# PostgreSQL
sus pg_config DatabaseConfig = create_database_config(
    DB_POSTGRES, "localhost", 5432, "mydb", "user", "pass"
)
sus pg_conn tea = connect_database(pg_config)

# MySQL
sus mysql_config DatabaseConfig = create_database_config(
    DB_MYSQL, "127.0.0.1", 3306, "mydb", "root", "password"
)
sus mysql_conn tea = connect_database(mysql_config)

# SQLite
sus sqlite_conn tea = connect_sqlite("/path/to/database.db")
```

### Connection Pooling

```cursed
# Create connection pool
sus pool ConnectionPool = create_connection_pool(config)

# Get connection from pool
sus conn tea = get_connection_from_pool(pool)

# Use connection...

# Return connection to pool
return_connection_to_pool(pool, conn)
```

## Query Execution

### Basic Queries

```cursed
# SELECT query
sus result QueryResult = execute_query(
    conn,
    "SELECT id, name, email FROM users WHERE active = ?",
    ["true"]
)

# INSERT query
sus insert_result QueryResult = execute_query(
    conn,
    "INSERT INTO users (name, email) VALUES (?, ?)",
    ["John Doe", "john@example.com"]
)

# UPDATE query
sus update_result QueryResult = execute_query(
    conn,
    "UPDATE users SET email = ? WHERE id = ?",
    ["newemail@example.com", "123"]
)

# DELETE query
sus delete_result QueryResult = execute_query(
    conn,
    "DELETE FROM users WHERE id = ?",
    ["123"]
)
```

### Prepared Statements

```cursed
# Prepare statement
sus stmt PreparedStatement = prepare_statement(
    conn,
    "SELECT * FROM users WHERE age > ? AND city = ?"
)

# Execute prepared statement
sus result QueryResult = execute_prepared_statement(
    stmt,
    ["21", "New York"]
)
```

## Query Builder

Build SQL queries programmatically:

```cursed
# Create query builder
sus builder QueryBuilder = new_query_builder("users")

# Build SELECT query
builder = query_select(builder, ["id", "name", "email"])
builder = query_where(builder, "age > 18")
builder = query_where(builder, "status = 'active'")
builder = query_order_by(builder, "name ASC")
builder = query_limit(builder, 10)
builder = query_offset(builder, 20)

sus sql tea = build_select_query(builder)
# Result: SELECT id, name, email FROM users WHERE age > 18 AND status = 'active' ORDER BY name ASC LIMIT 10 OFFSET 20
```

## Transaction Management

```cursed
# Begin transaction
sus tx Transaction = begin_transaction(conn)

# Execute queries within transaction
sus result1 QueryResult = execute_query(conn, "INSERT INTO users ...", [...])
sus result2 QueryResult = execute_query(conn, "UPDATE accounts ...", [...])

# Commit or rollback
yikes result1.success && result2.success {
    commit_transaction(tx)
} shook {
    rollback_transaction(tx)
}
```

## ORM Functionality

Simple object-relational mapping:

```cursed
# Create new record
sus user Record = new_record("users")

# Set fields
set_field(user, "name", "Alice Johnson")
set_field(user, "email", "alice@example.com")
set_field(user, "age", "28")

# Save record (INSERT)
sus success lit = save_record(conn, user)

# Get field value
sus name tea = get_field(user, "name")
```

## Migration System

Database schema versioning:

```cursed
# Create migration
sus migration Migration = create_migration(
    "001",
    "Create users table",
    "CREATE TABLE users (id SERIAL PRIMARY KEY, name VARCHAR(255), email VARCHAR(255))",
    "DROP TABLE users"
)

# Apply migration
sus success lit = apply_migration(conn, migration)

# Rollback migration
sus rollback_success lit = rollback_migration(conn, migration)
```

## Database-Specific Features

### PostgreSQL

```cursed
yeet "database/postgres"

# PostgreSQL configuration
sus pg_config PostgresConfig = postgres_create_config(
    "localhost", 5432, "mydb", "user", "pass"
)

# JSON operations
sus json_data tea = postgres_json_extract("data", "user.name")
sus json_text tea = postgres_json_extract_text("data", "user.email")

# Array operations
sus array_contains tea = postgres_array_contains("tags", "'programming'")

# Full-text search
sus fts_query tea = postgres_full_text_search("content", "'search terms'", "english")

# Window functions
sus row_num tea = postgres_row_number(["department"], ["salary DESC"])
```

### MySQL

```cursed
yeet "database/mysql"

# MySQL configuration
sus mysql_config MySQLConfig = mysql_create_config(
    "127.0.0.1", 3306, "mydb", "root", "password"
)

# JSON operations (MySQL 5.7+)
sus json_extract tea = mysql_json_extract("data", "user.name")
sus json_valid tea = mysql_json_unquote(json_extract)

# Full-text search
sus fts_query tea = mysql_fulltext_search(["title", "content"], "search terms", "boolean")

# UPSERT (INSERT ... ON DUPLICATE KEY UPDATE)
sus upsert_sql tea = mysql_upsert_query("users", ["name", "email"], ["email"])
```

### SQLite

```cursed
yeet "database/sqlite"

# SQLite configuration
sus sqlite_config SQLiteConfig = sqlite_create_config("/path/to/db.sqlite")

# Pragmas
sus enable_fk tea = sqlite_enable_foreign_keys()
sus wal_mode tea = sqlite_set_journal_mode("WAL")

# JSON operations (with JSON1 extension)
sus json_extract tea = sqlite_json_extract("data", "user.name")
sus json_valid tea = sqlite_json_valid("data")

# Full-text search (FTS5)
sus fts_table tea = sqlite_create_fts_table("search_index", ["title", "content"], "")
sus fts_query tea = sqlite_fts_search("search_index", "search terms")

# Date functions
sus current_time tea = sqlite_current_timestamp()
sus date_diff tea = sqlite_date_diff("'2023-12-31'", "'2023-01-01'", "days")
```

## Error Handling

The database layer provides comprehensive error handling:

```cursed
# Execute query with error handling
sus result QueryResult = execute_query(conn, sql, params)

yikes !result.success {
    vibez.spill(stringz.format("Query failed: {}", result.error_message))
    
    # Parse database-specific errors
    sus error_type tea = postgres_parse_error(result.error_message)
    ready error_type {
        "DUPLICATE_KEY_ERROR" -> {
            vibez.spill("Duplicate key constraint violation")
        }
        "FOREIGN_KEY_ERROR" -> {
            vibez.spill("Foreign key constraint violation")
        }
        basic -> {
            vibez.spill("Unknown database error")
        }
    }
}
```

## Performance Optimization

### Connection Pooling

```cursed
# Create optimized connection pool
sus config DatabaseConfig = create_database_config(...)
config.pool_size = 20  # Adjust based on load

sus pool ConnectionPool = create_connection_pool(config)
```

### Query Optimization

```cursed
# Use prepared statements for repeated queries
sus stmt PreparedStatement = prepare_statement(conn, sql)

# Use query builder for complex queries
sus builder QueryBuilder = new_query_builder("users")
builder = query_select(builder, ["id", "name"])  # Select only needed columns
builder = query_where(builder, "indexed_column = ?")  # Use indexed columns
builder = query_limit(builder, 100)  # Limit result size
```

### Database-Specific Optimizations

```cursed
# PostgreSQL: Use indexes
sus index_sql tea = postgres_add_index("users", "idx_email", ["email"], based)

# MySQL: Use storage engines
sus innodb_table tea = mysql_create_innodb_table("users", [...], "ROW_FORMAT=COMPRESSED")

# SQLite: Memory optimization
sus pragmas []tea = sqlite_memory_optimization()
bestie pragma := range pragmas {
    execute_query(conn, pragma, [])
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/database/test_database.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/database/test_database.💀
./test_database
```

## Advanced Usage

### Custom Query Builder Extensions

```cursed
# Extend query builder for specific use cases
slay advanced_user_query(min_age normie, city tea, limit normie) tea {
    sus builder QueryBuilder = new_query_builder("users")
    builder = query_select(builder, ["id", "name", "email", "created_at"])
    builder = query_where(builder, stringz.format("age >= {}", min_age))
    builder = query_where(builder, stringz.format("city = '{}'", city))
    builder = query_order_by(builder, "created_at DESC")
    builder = query_limit(builder, limit)
    
    damn build_select_query(builder)
}
```

### Migration Management

```cursed
# Migration runner
slay run_migrations(conn tea, migrations []Migration) lit {
    bestie migration := range migrations {
        sus success lit = apply_migration(conn, migration)
        yikes !success {
            vibez.spill(stringz.format("Migration {} failed", migration.version))
            damn cap
        }
        vibez.spill(stringz.format("Applied migration {}: {}", migration.version, migration.description))
    }
    damn based
}
```

### Connection Health Monitoring

```cursed
# Health check function
slay check_connection_health(conn tea) lit {
    sus result QueryResult = execute_query(conn, "SELECT 1", [])
    damn result.success
}
```

## Best Practices

1. **Always use parameterized queries** to prevent SQL injection
2. **Use connection pooling** for high-traffic applications
3. **Close connections** when done to free resources
4. **Use transactions** for multi-step operations
5. **Handle errors gracefully** with proper error checking
6. **Choose appropriate data types** for your database
7. **Use indexes** for frequently queried columns
8. **Monitor connection pool** usage and adjust size as needed

## Contributing

When extending the database layer:

1. Follow the pure CURSED implementation pattern
2. Add comprehensive tests for new features
3. Document all public functions
4. Support all three database backends when possible
5. Include error handling for edge cases

## License

This module is part of the CURSED standard library and follows the same license terms.
