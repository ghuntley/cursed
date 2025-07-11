# Database Module

The database module provides comprehensive database connectivity and ORM functionality for the CURSED language. It supports multiple database types and provides a clean, type-safe API for database operations.

## Features

- **Multi-Database Support**: MySQL, PostgreSQL, SQLite, MongoDB
- **ORM Functionality**: Create, Read, Update, Delete operations
- **Transaction Management**: Begin, commit, rollback transactions
- **Connection Pooling**: Efficient connection management
- **Schema Management**: Create/drop schemas and migrations
- **Backup/Restore**: Database backup and restore operations
- **Pure CURSED Implementation**: No external dependencies

## Database Types

```cursed
ConnectionType_MySQL = 1
ConnectionType_PostgreSQL = 2
ConnectionType_SQLite = 3
ConnectionType_MongoDB = 4
```

## Query Result Types

```cursed
QueryResult_Success = 1
QueryResult_Error = 2
QueryResult_NotFound = 3
```

## Basic Usage

### Connection Management

```cursed
yeet "database"

# Connect to database
sus connected lit = database_connect("localhost:5432/mydb", ConnectionType_PostgreSQL)

# Execute query
sus result normie = database_execute(1, "SELECT * FROM users")

# Validate connection
sus valid lit = database_validate_connection(1)
```

### Transaction Management

```cursed
# Begin transaction
database_begin_transaction(1)

# Execute operations
database_execute(1, "INSERT INTO users (name) VALUES ('John')")
database_execute(1, "UPDATE users SET active = true WHERE id = 1")

# Commit or rollback
database_commit_transaction(1)
# OR
database_rollback_transaction(1)
```

### ORM Operations

```cursed
# Create table
orm_create_table("users", "id INT PRIMARY KEY, name VARCHAR(255), active BOOLEAN")

# Insert record
sus user_id normie = orm_insert_record("users", "{\"name\": \"John\", \"active\": true}")

# Select records
sus records tea = orm_select_records("users", "active = true")

# Update record
orm_update_record("users", user_id, "{\"name\": \"Jane\", \"active\": false}")

# Delete record
orm_delete_record("users", user_id)
```

### Connection Pooling

```cursed
# Create connection pool
sus pool_id normie = database_create_pool("localhost:5432/mydb", 10)

# Get connection from pool
sus conn_id normie = database_get_connection_from_pool(pool_id)

# Use connection
database_execute(conn_id, "SELECT COUNT(*) FROM users")

# Return connection to pool
database_return_connection_to_pool(pool_id, conn_id)
```

### Schema Management

```cursed
# Create schema
database_create_schema("myapp_schema")

# Run migration
database_run_migration("001_create_users_table.sql")

# Rollback migration
database_rollback_migration(1)

# Drop schema
database_drop_schema("myapp_schema")
```

### Backup and Restore

```cursed
# Backup database
database_backup(1, "backup_20240107.sql")

# Restore database
database_restore(1, "backup_20240107.sql")
```

## Functions

### Connection Functions
- `database_connect(connection_string tea, db_type smol) lit` - Connect to database
- `database_validate_connection(connection_id normie) lit` - Validate connection

### Query Functions
- `database_execute(connection_id normie, query tea) normie` - Execute SQL query
- `database_escape_string(input tea) tea` - Escape string for SQL

### Transaction Functions
- `database_begin_transaction(connection_id normie) lit` - Begin transaction
- `database_commit_transaction(connection_id normie) lit` - Commit transaction
- `database_rollback_transaction(connection_id normie) lit` - Rollback transaction

### ORM Functions
- `orm_create_table(table_name tea, columns tea) lit` - Create table
- `orm_insert_record(table_name tea, data tea) normie` - Insert record
- `orm_select_records(table_name tea, conditions tea) tea` - Select records
- `orm_update_record(table_name tea, id normie, data tea) lit` - Update record
- `orm_delete_record(table_name tea, id normie) lit` - Delete record

### Schema Functions
- `database_create_schema(schema_name tea) lit` - Create schema
- `database_drop_schema(schema_name tea) lit` - Drop schema
- `database_run_migration(migration_file tea) lit` - Run migration
- `database_rollback_migration(migration_version normie) lit` - Rollback migration

### Pool Functions
- `database_create_pool(connection_string tea, pool_size normie) normie` - Create pool
- `database_get_connection_from_pool(pool_id normie) normie` - Get connection
- `database_return_connection_to_pool(pool_id normie, connection_id normie) lit` - Return connection

### Utility Functions
- `database_get_last_insert_id(connection_id normie) normie` - Get last insert ID
- `database_get_affected_rows(connection_id normie) normie` - Get affected rows
- `database_backup(connection_id normie, backup_file tea) lit` - Backup database
- `database_restore(connection_id normie, backup_file tea) lit` - Restore database

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/database/test_database.csd
```

Test both interpretation and compilation modes:

```bash
cargo run --bin cursed stdlib/database/test_database.csd
cargo run --bin cursed -- compile stdlib/database/test_database.csd
./test_database
```

## Error Handling

All functions return appropriate error values:
- Boolean functions return `cap` (false) on error
- Integer functions return -1 on error
- String functions return empty string on error

## Security

- Input validation on all parameters
- SQL injection prevention through parameter validation
- Connection validation before operations
- Transaction isolation support

## Performance

- Connection pooling for efficient resource management
- Minimal memory allocation
- Fast query execution
- Optimized for both interpretation and compilation modes

## Dependencies

- `testz` - Testing framework
- `string` - String manipulation
- `collections` - Data structures
- `json` - JSON handling

## License

Part of the CURSED language standard library.
