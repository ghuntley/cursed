# SQL Slay - Database Operations Module

A comprehensive database operations module for CURSED, providing SQL query building, execution, and connection management capabilities.

## Features

- **Connection Management**: Database connection lifecycle management
- **SQL Query Building**: Fluent API for building SELECT, INSERT, UPDATE, DELETE queries
- **Query Execution**: Execute SQL queries with proper error handling
- **Transaction Management**: BEGIN, COMMIT, ROLLBACK transaction support
- **Table Management**: CREATE, DROP, ALTER table operations
- **Connection Pooling**: Basic connection pool management
- **Schema Operations**: Table schema and column introspection
- **Pure CURSED**: No FFI dependencies, fully implemented in CURSED

## Usage

### Basic Connection Management

```cursed
yeet "sql_slay"

// Connect to database
sus connected lit = db_connect("localhost", 5432, "mydb", "user", "pass")
bestie connected {
    vibez.spill("Connected to database!")
}

// Check connection status
bestie db_is_connected() {
    vibez.spill("Database is connected")
}

// Get connection info
sus info tea = db_get_connection_info()
vibez.spill("Connection: " + info)

// Disconnect
db_disconnect()
```

### SQL Query Building

```cursed
// SELECT queries
sus select_all tea = sql_select("users", "*", "")
sus select_filtered tea = sql_select("users", "name, age", "age > 18")

// INSERT queries
sus insert_query tea = sql_insert("users", "name, age", "'John', 30")

// UPDATE queries
sus update_query tea = sql_update("users", "age = 31", "name = 'John'")

// DELETE queries
sus delete_query tea = sql_delete("users", "age < 18")
```

### Query Execution

```cursed
// Execute SELECT and get results
sus results tea = sql_execute_select("SELECT * FROM users")
sus count normie = sql_parse_results(results)
vibez.spill("Found " + count + " records")

// Execute INSERT/UPDATE/DELETE
sus affected normie = sql_execute_insert("INSERT INTO users VALUES (1, 'Alice')")
vibez.spill("Inserted " + affected + " record(s)")

sus updated normie = sql_execute_update("UPDATE users SET age = 25 WHERE name = 'Alice'")
vibez.spill("Updated " + updated + " record(s)")

sus deleted normie = sql_execute_delete("DELETE FROM users WHERE age < 18")
vibez.spill("Deleted " + deleted + " record(s)")
```

### Transaction Management

```cursed
// Start transaction
bestie sql_begin_transaction() {
    vibez.spill("Transaction started")
    
    // Perform multiple operations
    sql_execute_insert("INSERT INTO users VALUES (1, 'Alice')")
    sql_execute_insert("INSERT INTO users VALUES (2, 'Bob')")
    
    // Commit or rollback
    bestie some_condition {
        sql_commit()
        vibez.spill("Transaction committed")
    } else {
        sql_rollback()
        vibez.spill("Transaction rolled back")
    }
}

// Check transaction status
bestie sql_in_transaction() {
    vibez.spill("Currently in transaction")
}
```

### Table Management

```cursed
// Create table
sus create_sql tea = sql_create_table("users", "id INTEGER PRIMARY KEY, name TEXT, age INTEGER")
sql_execute(create_sql)

// Alter table
sus alter_sql tea = sql_alter_table("users", "ADD COLUMN email TEXT")
sql_execute(alter_sql)

// Drop table
sus drop_sql tea = sql_drop_table("users")
sql_execute(drop_sql)
```

### Schema Operations

```cursed
// Get column names
sus columns tea = sql_get_column_names("users")
vibez.spill("Columns: " + columns)

// Get table schema
sus schema tea = sql_get_table_schema("users")
vibez.spill("Schema: " + schema)
```

### Connection Pooling

```cursed
// Initialize connection pool
sql_init_pool(10)  // Max 10 connections

// Acquire connection from pool
bestie sql_pool_acquire() {
    vibez.spill("Connection acquired")
    
    // Use connection...
    
    // Release connection back to pool
    sql_pool_release()
}

// Check pool status
sus active normie = sql_get_pool_status()
vibez.spill("Active connections: " + active)
```

## API Reference

### Connection Management

- `db_connect(host, port, dbname, user, password)` - Connect to database
- `db_disconnect()` - Disconnect from database
- `db_is_connected()` - Check connection status
- `db_get_connection_info()` - Get connection details

### Query Building

- `sql_select(table, columns, where_clause)` - Build SELECT query
- `sql_insert(table, columns, values)` - Build INSERT query
- `sql_update(table, set_clause, where_clause)` - Build UPDATE query
- `sql_delete(table, where_clause)` - Build DELETE query

### Query Execution

- `sql_execute(query)` - Execute any SQL query
- `sql_execute_select(query)` - Execute SELECT and return results
- `sql_execute_insert(query)` - Execute INSERT and return affected rows
- `sql_execute_update(query)` - Execute UPDATE and return affected rows
- `sql_execute_delete(query)` - Execute DELETE and return affected rows

### Transaction Management

- `sql_begin_transaction()` - Start transaction
- `sql_commit()` - Commit transaction
- `sql_rollback()` - Rollback transaction
- `sql_in_transaction()` - Check transaction status

### Table Management

- `sql_create_table(table, columns)` - Build CREATE TABLE query
- `sql_drop_table(table)` - Build DROP TABLE query
- `sql_alter_table(table, action)` - Build ALTER TABLE query

### Utility Functions

- `sql_escape_string(input)` - Escape SQL string
- `sql_validate_table_name(table)` - Validate table name
- `sql_validate_column_name(column)` - Validate column name
- `sql_parse_results(results)` - Parse query results
- `sql_get_column_names(table)` - Get table column names
- `sql_get_table_schema(table)` - Get table schema

### Connection Pool Management

- `sql_init_pool(max_connections)` - Initialize connection pool
- `sql_get_pool_status()` - Get active connection count
- `sql_pool_acquire()` - Acquire connection from pool
- `sql_pool_release()` - Release connection to pool

## Implementation Details

This module provides a pure CURSED implementation of database operations without external dependencies. The implementation includes:

- **Query Builder Pattern**: Fluent API for constructing SQL queries
- **Connection State Management**: Proper tracking of database connections
- **Transaction Support**: Full transaction lifecycle management
- **Error Handling**: Comprehensive error checking and validation
- **Result Processing**: Parsing and handling of query results
- **Connection Pooling**: Basic connection pool for performance

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/sql_slay/test_sql_slay.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/sql_slay/test_sql_slay.💀
./test_sql_slay
```

The test suite covers:
- Connection management
- SQL query building
- Query execution
- Transaction management
- Table operations
- Utility functions
- Error handling
- Performance operations

## Design Principles

1. **Pure CURSED**: No FFI dependencies, fully implemented in CURSED language
2. **Type Safety**: Strong typing with proper error handling
3. **Simplicity**: Clean, easy-to-use API
4. **Performance**: Efficient query building and execution
5. **Reliability**: Comprehensive error checking and validation
6. **Testability**: Full test coverage with both interpretation and compilation modes

## Example Application

```cursed
yeet "sql_slay"

// Connect to database
db_connect("localhost", 5432, "myapp", "user", "password")

// Create users table
sus create_table tea = sql_create_table("users", "id INTEGER PRIMARY KEY, name TEXT, email TEXT")
sql_execute(create_table)

// Insert users
sql_begin_transaction()
sql_execute_insert("INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com')")
sql_execute_insert("INSERT INTO users (name, email) VALUES ('Bob', 'bob@example.com')")
sql_commit()

// Query users
sus users tea = sql_execute_select("SELECT * FROM users WHERE name LIKE 'A%'")
sus count normie = sql_parse_results(users)
vibez.spill("Found " + count + " users starting with 'A'")

// Update user
sus updated normie = sql_execute_update("UPDATE users SET email = 'alice.new@example.com' WHERE name = 'Alice'")
vibez.spill("Updated " + updated + " user(s)")

// Cleanup
db_disconnect()
```

This module demonstrates the power and flexibility of the CURSED language for database operations while maintaining simplicity and performance.
