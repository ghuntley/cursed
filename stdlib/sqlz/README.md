# sqlz Module

The `sqlz` module provides comprehensive SQL database operations with connection pooling, query building, ORM functionality, and transaction management. It serves as a standardized alias for the `sql_slay` module while maintaining the expected "z" suffix naming convention.

## Features

### Database Connectivity
- Multiple database driver support (PostgreSQL, MySQL, SQLite, MongoDB)
- Connection string parsing and validation
- Connection pooling with configurable parameters
- Automatic connection lifecycle management

### Query Operations
- Raw SQL query execution
- Prepared statement support with parameter binding
- Query result processing and row iteration
- Batch operation support

### Query Building
- Fluent query builder interface
- Type-safe SELECT, INSERT, UPDATE, DELETE operations
- Join operations and subquery support
- Conditional query building

### ORM Functionality
- Model definition and field mapping
- Automatic CRUD operations
- Relationship handling
- Data validation and type conversion

### Transaction Management
- Transaction begin/commit/rollback operations
- Savepoint support for nested transactions
- Automatic rollback on errors
- Transaction isolation level control

### Migration System
- Schema migration creation and execution
- Version tracking and rollback support
- Database schema introspection
- Automatic migration dependency resolution

## Usage Examples

### Database Connection
```cursed
yeet "sqlz"

// Connect to PostgreSQL
sus conn SqlConnection = sqlz.connect("postgresql://user:pass@localhost:5432/mydb")

// Connection pooling
sus pool_config PoolConfig = PoolConfig{
    max_connections: 10,
    min_connections: 2,
    connection_timeout: 30,
    idle_timeout: 300
}
sus pool ConnectionPool = sqlz.connect_pool(pool_config)
sus pooled_conn SqlConnection = sqlz.get_connection(pool)
```

### Basic Query Operations
```cursed
// Execute raw SQL
sus result QueryResult = sqlz.execute(conn, "SELECT * FROM users")

// Query single row
sus user_row Row = sqlz.query_row(conn, "SELECT * FROM users WHERE id = 1")

// Prepared statements
sus stmt PreparedStatement = sqlz.prepare(conn, "SELECT * FROM users WHERE age > ?")
sqlz.bind_param(stmt, 0, "18")
sus adult_users QueryResult = sqlz.execute_prepared(conn, stmt, [])
```

### Query Building
```cursed
// SELECT query building
sus columns []tea = ["id", "name", "email"]
sus select_builder SelectBuilder = sqlz.select_query("users", columns)
// Additional WHERE, JOIN, ORDER BY would be chained here

// INSERT query building
sus insert_builder InsertBuilder = sqlz.insert_query("users")
// Values would be added to the builder

// UPDATE query building  
sus update_builder UpdateBuilder = sqlz.update_query("users")
// SET clauses and WHERE conditions would be added

// DELETE query building
sus delete_builder DeleteBuilder = sqlz.delete_query("users")
// WHERE conditions would be added
```

### ORM Operations
```cursed
// Define model
sus user_fields []FieldDefinition = [
    FieldDefinition{name: "id", type: "integer", primary_key: based},
    FieldDefinition{name: "name", type: "varchar", max_length: 255},
    FieldDefinition{name: "email", type: "varchar", max_length: 255, unique: based}
]
sus user_model Model = sqlz.define_model("User", user_fields)

// Save model instance
sqlz.save_model(conn, user_model)

// Find operations
sus user Model = sqlz.find_by_id(conn, user_model, "123")
sus all_users []Model = sqlz.find_all(conn, user_model)
```

### Transaction Management
```cursed
// Begin transaction
sus tx Transaction = sqlz.begin_transaction(conn)

// Perform operations within transaction
sus result1 QueryResult = sqlz.execute(conn, "INSERT INTO users ...")
sus result2 QueryResult = sqlz.execute(conn, "UPDATE accounts ...")

// Commit or rollback
ready (all_operations_successful) {
    sqlz.commit_transaction(tx)
} yikes {
    sqlz.rollback_transaction(tx)
}
```

### Migration System
```cursed
// Create migration
sus migration Migration = sqlz.create_migration("create_users_table")

// Run migrations
sus migrations []Migration = [migration]
sqlz.run_migrations(conn, migrations)
```

### Database Introspection
```cursed
// List all tables
sus tables []tea = sqlz.list_tables(conn)

// Describe table structure
sus schema TableSchema = sqlz.describe_table(conn, "users")
```

### Security and Sanitization
```cursed
// SQL injection prevention
sus unsafe_input tea = "'; DROP TABLE users; --"
sus safe_input tea = sqlz.escape_string(unsafe_input)

// Query sanitization
sus unsafe_query tea = "SELECT * FROM users WHERE input = '" + user_input + "'"
sus safe_query tea = sqlz.sanitize_query(unsafe_query)
```

## Data Types

### Core Types
- `SqlConnection` - Database connection handle
- `ConnectionPool` - Connection pool manager
- `QueryResult` - Query execution result
- `Row` - Single database row
- `PreparedStatement` - Prepared SQL statement
- `Transaction` - Database transaction

### Query Builder Types
- `SelectBuilder` - SELECT query builder
- `InsertBuilder` - INSERT query builder
- `UpdateBuilder` - UPDATE query builder
- `DeleteBuilder` - DELETE query builder

### ORM Types
- `Model` - ORM model definition
- `FieldDefinition` - Model field definition
- `TableSchema` - Database table schema
- `ColumnInfo` - Table column information

### Configuration Types
- `PoolConfig` - Connection pool configuration
- `Migration` - Database migration definition

## Function Reference

### Connection Management
- `connect(connection_string)` - Establish database connection
- `connect_pool(config)` - Create connection pool
- `close_connection(conn)` - Close database connection
- `get_connection(pool)` - Get connection from pool
- `return_connection(pool, conn)` - Return connection to pool

### Query Execution
- `execute(conn, query)` - Execute raw SQL query
- `execute_prepared(conn, stmt, params)` - Execute prepared statement
- `query(conn, sql)` - Execute query and return result set
- `query_row(conn, sql)` - Execute query and return single row

### Transaction Operations
- `begin_transaction(conn)` - Start new transaction
- `commit_transaction(tx)` - Commit transaction changes
- `rollback_transaction(tx)` - Rollback transaction changes

### Prepared Statements
- `prepare(conn, sql)` - Create prepared statement
- `bind_param(stmt, index, value)` - Bind parameter to statement

### Query Building
- `select_query(table, columns)` - Create SELECT builder
- `insert_query(table)` - Create INSERT builder
- `update_query(table)` - Create UPDATE builder
- `delete_query(table)` - Create DELETE builder

### ORM Operations
- `define_model(name, fields)` - Define ORM model
- `save_model(conn, model)` - Save model instance
- `find_by_id(conn, model, id)` - Find model by primary key
- `find_all(conn, model)` - Find all model instances

### Migration System
- `create_migration(name)` - Create new migration
- `run_migrations(conn, migrations)` - Execute migrations

### Security Utilities
- `escape_string(value)` - Escape string for SQL safety
- `sanitize_query(query)` - Sanitize SQL query

### Database Introspection
- `list_tables(conn)` - List all database tables
- `describe_table(conn, table_name)` - Get table schema

### Error Handling
- `get_last_error()` - Get last error message
- `clear_errors()` - Clear error state

## Configuration

### Connection Strings
```
PostgreSQL: postgresql://user:password@host:port/database
MySQL:      mysql://user:password@host:port/database  
SQLite:     sqlite:///path/to/database.db
MongoDB:    mongodb://user:password@host:port/database
```

### Pool Configuration
```cursed
PoolConfig{
    max_connections: 20,        // Maximum concurrent connections
    min_connections: 5,         // Minimum idle connections
    connection_timeout: 30,     // Connection timeout in seconds
    idle_timeout: 600,          // Idle connection timeout in seconds
    max_lifetime: 3600          // Maximum connection lifetime in seconds
}
```

## Security Features

### SQL Injection Prevention
- Automatic parameter escaping
- Prepared statement support
- Query sanitization functions
- Input validation and type checking

### Connection Security
- Encrypted connection support
- SSL/TLS configuration
- Connection pool isolation
- Automatic connection cleanup

## Error Handling

### Error Types
- Connection errors (timeout, authentication, network)
- Query syntax errors
- Constraint violation errors
- Transaction errors

### Error Recovery
- Automatic retry for transient errors
- Connection pool health monitoring
- Graceful degradation patterns
- Comprehensive error logging

## Performance Optimization

### Connection Pooling
- Configurable pool sizes
- Connection lifecycle management
- Health check monitoring
- Load balancing across connections

### Query Optimization
- Prepared statement caching
- Query plan optimization hints
- Batch operation support
- Result set streaming

## Testing
Run the comprehensive test suite:
```bash
./zig-out/bin/cursed stdlib/sqlz/test_sqlz.csd
```

The test suite covers:
- Database connection and pooling
- Query building and execution
- ORM operations and model management
- Transaction handling
- Migration system
- Security and sanitization
- Error handling scenarios

## Implementation Notes

### Module Relationship
The `sqlz` module is an alias wrapper around `sql_slay`, providing:
- Standardized naming convention (z suffix)
- Backward compatibility
- Consistent API surface
- All functionality from `sql_slay`

### Database Driver Support
- PostgreSQL (primary)
- MySQL/MariaDB
- SQLite (embedded)
- MongoDB (NoSQL)

### Thread Safety
- Connection pool thread safety
- Transaction isolation
- Concurrent query execution
- Safe connection sharing patterns

## Best Practices

### Connection Management
- Always use connection pooling in production
- Configure appropriate timeout values
- Monitor connection pool metrics
- Clean up connections properly

### Query Security
- Always use prepared statements for user input
- Sanitize all external data
- Validate input types and ranges
- Use parameter binding instead of string concatenation

### Transaction Usage
- Keep transactions short and focused
- Handle rollback scenarios gracefully
- Use appropriate isolation levels
- Avoid nested transactions when possible

### Performance
- Use batch operations for bulk data
- Index frequently queried columns
- Monitor query performance
- Cache prepared statements
