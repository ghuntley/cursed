# CURSED Database Module (dbz)

A pure CURSED implementation of database connectivity and operations supporting SQLite, PostgreSQL, and MySQL.

## Features

- ✅ **Multi-Database Support**: SQLite, PostgreSQL, MySQL connections
- ✅ **Connection Management**: Multiple concurrent database connections
- ✅ **SQL Execution**: Query execution with result handling
- ✅ **Prepared Statements**: Parameter binding and execution
- ✅ **Transaction Support**: Begin, commit, rollback transactions
- ✅ **Schema Operations**: Create/drop tables, table introspection
- ✅ **CRUD Operations**: Create, read, update, delete helpers
- ✅ **SQL Injection Protection**: Input validation and escaping
- ✅ **Query Builder**: Programmatic query construction

## Quick Start

```cursed
yeet "dbz"

# Connect to SQLite database
sus conn_id drip = db_connect_sqlite("/data/app.db")

# Execute schema creation
db_execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)")

# Insert data
db_insert("users", "name, email", "'John Doe', 'john@example.com'")

# Query data
sus users []tea = db_select("users", "*", "")
bestie i := 0; i < len(users); i++ {
    vibez.spill("User:", users[i])
}

# Disconnect
db_disconnect(conn_id)
```

## Connection Management

### SQLite
```cursed
sus sqlite_conn drip = db_connect_sqlite("/path/to/database.db")
ready (db_is_connected(sqlite_conn)) {
    vibez.spill("Connected to SQLite")
}
```

### PostgreSQL
```cursed
sus pg_conn drip = db_connect_postgres("localhost", 5432, "mydb", "user", "password")
ready (db_is_connected(pg_conn)) {
    vibez.spill("Connected to PostgreSQL")
}
```

### MySQL
```cursed
sus mysql_conn drip = db_connect_mysql("localhost", 3306, "mydb", "user", "password")
ready (db_is_connected(mysql_conn)) {
    vibez.spill("Connected to MySQL")
}
```

## SQL Execution

### Basic Queries
```cursed
# Execute DDL/DML statements
sus result drip = db_execute("CREATE TABLE products (id INT, name TEXT, price DECIMAL)")
ready (result == DB_SUCCESS) {
    vibez.spill("Table created successfully")
}

# Execute SELECT queries
sus results []tea = db_query("SELECT * FROM users WHERE active = 1")
bestie i := 0; i < len(results); i++ {
    vibez.spill("Row:", results[i])
}
```

### Single Value Queries
```cursed
# Get first row
sus first_user tea = db_query_single("SELECT * FROM users LIMIT 1")

# Get scalar value
sus user_count tea = db_query_scalar("SELECT COUNT(*) FROM users")
```

## Prepared Statements

```cursed
# Prepare statement
sus stmt_id drip = db_prepare("INSERT INTO users (name, email) VALUES (?, ?)")

# Bind parameters
db_bind_param(stmt_id, 1, "Jane Smith")
db_bind_param(stmt_id, 2, "jane@example.com")

# Execute
sus result drip = db_execute_prepared(stmt_id)

# Clean up
db_finalize_statement(stmt_id)
```

## Transaction Management

```cursed
# Begin transaction
db_begin_transaction()

# Execute multiple operations
db_insert("users", "name, email", "'User 1', 'user1@example.com'")
db_insert("users", "name, email", "'User 2', 'user2@example.com'")

# Commit or rollback
ready (all_operations_successful) {
    db_commit_transaction()
} otherwise {
    db_rollback_transaction()
}
```

## Schema Operations

### Table Management
```cursed
# Create table
db_create_table("products", "id INTEGER PRIMARY KEY, name TEXT NOT NULL, price DECIMAL")

# Check if table exists
ready (db_table_exists("products")) {
    vibez.spill("Products table exists")
}

# List all tables
sus tables []tea = db_list_tables()
bestie i := 0; i < len(tables); i++ {
    vibez.spill("Table:", tables[i])
}

# Describe table structure
sus columns []tea = db_describe_table("users")
bestie i := 0; i < len(columns); i++ {
    vibez.spill("Column:", columns[i])
}

# Drop table
db_drop_table("old_table")
```

## CRUD Operations

### Create
```cursed
db_insert("users", "name, email, active", "'John Doe', 'john@example.com', 1")
```

### Read
```cursed
# Select all
sus all_users []tea = db_select("users", "*", "")

# Select with conditions
sus active_users []tea = db_select("users", "name, email", "active = 1")

# Count records
sus user_count drip = db_count("users", "active = 1")
```

### Update
```cursed
db_update("users", "email='newemail@example.com'", "id = 1")
```

### Delete
```cursed
db_delete("users", "active = 0")
```

## SQL Injection Protection

### Input Validation
```cursed
sus user_input tea = "'; DROP TABLE users; --"
ready (!validate_sql_injection(user_input)) {
    vibez.spill("Dangerous input detected!")
    damn  # Exit early
}
```

### String Escaping
```cursed
sus unsafe_name tea = "O'Reilly"
sus safe_name tea = escape_sql_string(unsafe_name)  # "O''Reilly"

sus safe_table tea = sanitize_table_name("user_data_123")  # Only alphanumeric + underscore
sus safe_column tea = sanitize_column_name("user_name")
```

## Query Builder

### SELECT Queries
```cursed
sus columns []tea = ["name", "email", "created_at"]
sus conditions []tea = ["active = 1", "email IS NOT NULL"]
sus query tea = build_select_query("users", columns, conditions)
# Result: "SELECT name, email, created_at FROM users WHERE active = 1 AND email IS NOT NULL"
```

### INSERT Queries
```cursed
# Using map data (simplified implementation)
sus user_data map[tea]tea = {"name": "John", "email": "john@example.com"}
sus insert_query tea = build_insert_query("users", user_data)
```

## Error Handling

### Return Codes
- `DB_SUCCESS (0)` - Operation successful
- `DB_ERROR (-1)` - Operation failed
- `DB_NO_ROWS (1)` - Query returned no results
- `DB_CONSTRAINT_VIOLATION (2)` - Constraint violation

### Example
```cursed
sus result drip = db_execute("INSERT INTO users (email) VALUES ('duplicate@example.com')")
ready (result == DB_ERROR) {
    vibez.spill("Insert failed")
} otherwise ready (result == DB_SUCCESS) {
    vibez.spill("Insert successful")
}
```

## Database-Specific Features

### SQLite
```cursed
# SQLite-specific query
sus tables []tea = db_query("SELECT name FROM sqlite_master WHERE type='table'")
```

### PostgreSQL
```cursed
# PostgreSQL-specific query
sus tables []tea = db_query("SELECT tablename FROM pg_tables WHERE schemaname='public'")
```

### Connection Type Detection
The module automatically adapts queries based on the current database type for maximum compatibility.

## Usage Examples

### User Management System
```cursed
yeet "dbz"

# Setup
sus conn drip = db_connect_sqlite("users.db")
db_create_table("users", "id INTEGER PRIMARY KEY, username TEXT UNIQUE, email TEXT, created_at DATETIME")

# Create user
slay create_user(username tea, email tea) drip {
    ready (!validate_sql_injection(username) || !validate_sql_injection(email)) {
        damn DB_ERROR
    }
    
    sus safe_username tea = escape_sql_string(username)
    sus safe_email tea = escape_sql_string(email)
    
    damn db_insert("users", "username, email, created_at", 
                   "'" + safe_username + "', '" + safe_email + "', datetime('now')")
}

# Get user by username
slay get_user_by_username(username tea) tea {
    sus safe_username tea = escape_sql_string(username)
    damn db_query_single("SELECT * FROM users WHERE username = '" + safe_username + "'")
}

# Update user email
slay update_user_email(username tea, new_email tea) drip {
    sus safe_username tea = escape_sql_string(username)
    sus safe_email tea = escape_sql_string(new_email)
    
    damn db_update("users", "email = '" + safe_email + "'", "username = '" + safe_username + "'")
}
```

### E-commerce Product Catalog
```cursed
# Product management
db_create_table("products", "id INTEGER PRIMARY KEY, name TEXT, price DECIMAL, category_id INTEGER")
db_create_table("categories", "id INTEGER PRIMARY KEY, name TEXT")

# Add products with transaction
db_begin_transaction()

db_insert("categories", "name", "'Electronics'")
db_insert("products", "name, price, category_id", "'Laptop', 999.99, 1")
db_insert("products", "name, price, category_id", "'Mouse', 29.99, 1")

db_commit_transaction()

# Search products
sus electronics []tea = db_select("products", "*", "category_id = 1")
bestie i := 0; i < len(electronics); i++ {
    vibez.spill("Product:", electronics[i])
}
```

## Testing

Run the comprehensive test suite:

```bash
./zig-out/bin/cursed-zig stdlib/dbz/test_dbz.csd
```

The test suite covers:
- Connection management for all database types
- SQL query execution and result handling
- Prepared statements and parameter binding
- Transaction management
- Schema operations
- CRUD operations
- SQL injection protection
- Query building functionality

## Integration

The dbz module integrates well with other CURSED stdlib modules:

- **httpz**: Database-backed REST APIs
- **jsonz**: JSON serialization of query results
- **cryptz**: Password hashing for user authentication
- **stringz**: String manipulation for query building
- **testz**: Database testing and fixtures

## Performance Considerations

- Use prepared statements for repeated queries
- Implement connection pooling for high-concurrency applications
- Use transactions for multiple related operations
- Validate and sanitize all user inputs
- Use appropriate indexes for frequently queried columns

## Limitations

- Simplified database simulation (not real database connections)
- Basic SQL injection protection (not comprehensive)
- Limited prepared statement parameter types
- Simplified query result parsing
- No connection pooling implementation

For production use, this module provides a foundation that can be extended with actual database drivers and more sophisticated features.
