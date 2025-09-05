# Database ORM Module

A comprehensive Object-Relational Mapping (ORM) system implemented in pure CURSED without FFI dependencies. This module provides enterprise-grade ORM functionality including entity mapping, query building, schema management, connection pooling, and transaction management.

## Features

### Core ORM Functionality
- **Entity Mapping**: Map CURSED structures to database tables with metadata-driven configuration
- **Relationships**: Support for belongs_to, has_one, has_many, and many_to_many relationships
- **Query Builder**: Fluent SQL query builder with complex joins, conditions, and aggregations
- **Schema Management**: Database schema definition and migration system
- **Connection Pooling**: Enterprise-grade connection pool with configurable limits and health monitoring
- **Transaction Management**: ACID-compliant transactions with savepoints and isolation levels

### Advanced Features
- **Repository Pattern**: Clean separation of data access logic
- **Query Caching**: Configurable query result caching for performance optimization
- **Event System**: Event-driven architecture with middleware support
- **Validation**: Entity validation with customizable rules
- **Migration System**: Version-controlled database schema migrations
- **Lazy Loading**: Efficient relationship loading strategies

## Quick Start

### Basic Entity Definition

```cursed
yeet "database_orm"

# Create entity metadata
sus user_metadata EntityMetadata = create_entity_metadata("users", "id")
user_metadata = add_field_metadata(user_metadata, "name", "name", "VARCHAR(255)", cap)
user_metadata = add_field_metadata(user_metadata, "email", "email", "VARCHAR(255)", cap)
user_metadata = add_field_metadata(user_metadata, "created_at", "created_at", "TIMESTAMP", cap)

# Add relationships
user_metadata = add_relationship_metadata(user_metadata, "profile", "has_one", "profiles", "user_id", "id")
user_metadata = add_relationship_metadata(user_metadata, "posts", "has_many", "posts", "user_id", "id")

# Create entity instance
sus user Entity = create_entity(user_metadata)
user = set_attribute(user, "name", "John Doe")
user = set_attribute(user, "email", "john@example.com")
```

### Query Builder Usage

```cursed
# Create query builder
sus builder QueryBuilder = create_query_builder("User", "users")

# Build complex query
sus fields []tea = ["users.id", "users.name", "profiles.bio"]
builder = select_fields(builder, fields)
builder = join_table(builder, "LEFT", "profiles", "p", "users.id = p.user_id")
builder = where_condition(builder, "users.active", "=", "1")
builder = where_condition(builder, "users.age", ">", "18")
sus status_values []tea = ["active", "premium"]
builder = where_in(builder, "users.status", status_values)
builder = order_by(builder, "users.name", "ASC")
builder = limit_results(builder, 10)

# Generate SQL
sus query tea = build_select_query(builder)
vibez.spill(query)
```

### Repository Pattern

```cursed
# Create connection pool
sus config ConnectionPoolConfig = ConnectionPoolConfig{
    max_connections: 20,
    min_connections: 5,
    max_idle_time_seconds: 300,
    connection_timeout_seconds: 30,
    validation_query: "SELECT 1",
    retry_attempts: 3,
    retry_delay_ms: 100
}

sus pool ConnectionPool = create_connection_pool(config)

# Create repository
sus user_repository Repository = create_repository("User", pool)

# Find operations
sus user Entity = find_by_id(user_repository, "123")
sus all_users []Entity = find_all(user_repository)

# Save operations
user = save_entity(user_repository, user)

# Delete operations
sus deleted lit = delete_entity(user_repository, user)
```

### Transaction Management

```cursed
# Begin transaction
sus connection DatabaseConnection = create_database_connection()
sus transaction TransactionContext = begin_orm_transaction(connection, "READ_COMMITTED")

# Add savepoint
transaction = add_savepoint(transaction, "before_update")

# Perform operations...

# Commit or rollback
sus success lit = commit_orm_transaction(transaction)
# or
sus rolled_back lit = rollback_orm_transaction(transaction)
```

### Schema Management

```cursed
# Create table schema
sus users_table TableSchema = create_table_schema("users", "InnoDB", "utf8mb4")
users_table = add_column_to_table(users_table, "id", "INT", cap)
users_table = add_column_to_table(users_table, "name", "VARCHAR(255)", cap)
users_table = add_column_to_table(users_table, "email", "VARCHAR(255)", cap)
users_table = add_column_to_table(users_table, "created_at", "TIMESTAMP", cap)

# Set primary key
users_table.primary_keys = ["id"]

# Generate CREATE TABLE SQL
sus create_sql tea = generate_create_table_sql(users_table)
vibez.spill(create_sql)
```

### Migration System

```cursed
# Create migration
sus up_sql []tea = [
    "CREATE TABLE users (id INT AUTO_INCREMENT PRIMARY KEY, name VARCHAR(255) NOT NULL)",
    "CREATE INDEX idx_users_name ON users(name)",
    "INSERT INTO users (name) VALUES ('Admin User')"
]

sus down_sql []tea = [
    "DROP INDEX idx_users_name",
    "DROP TABLE users"
]

sus migration Migration = create_migration("001", "Create users table", up_sql, down_sql)

# Apply migration
sus connection DatabaseConnection = create_database_connection()
sus applied lit = apply_migration(connection, migration)

# Rollback if needed
sus rolled_back lit = rollback_migration(connection, migration)
```

## API Reference

### Core Types

#### EntityMetadata
Defines the structure and relationships of a database entity.

```cursed
be_like EntityMetadata = struct {
    table_name tea
    primary_key tea
    fields map[tea]FieldMetadata
    relationships map[tea]RelationshipMetadata
    indexes []IndexMetadata
    created_at tea
    updated_at tea
}
```

#### FieldMetadata
Defines column mapping and constraints for entity fields.

```cursed
be_like FieldMetadata = struct {
    column_name tea
    field_type tea
    nullable lit
    auto_increment lit
    default_value tea
    constraints []tea
    serializer tea
}
```

#### RelationshipMetadata
Defines associations between entities.

```cursed
be_like RelationshipMetadata = struct {
    relation_type tea  # "belongs_to", "has_one", "has_many", "many_to_many"
    target_entity tea
    foreign_key tea
    local_key tea
    through_table tea
    cascade_delete lit
    lazy_loading lit
}
```

#### Entity
Represents a database record with attributes and metadata.

```cursed
be_like Entity = struct {
    metadata EntityMetadata
    attributes map[tea]tea
    original_attributes map[tea]tea
    is_persisted lit
    is_dirty lit
    validation_errors []tea
    relationships_loaded map[tea]lit
}
```

### Query Builder

#### QueryBuilder
Fluent interface for building complex SQL queries.

```cursed
be_like QueryBuilder = struct {
    entity_type tea
    table_name tea
    select_fields []tea
    joins []JoinClause
    where_conditions []WhereCondition
    group_by_fields []tea
    having_conditions []WhereCondition
    order_by_clauses []OrderByClause
    limit_value normie
    offset_value normie
    distinct_fields []tea
    for_update lit
    lock_mode tea
}
```

#### Key Functions

- `create_query_builder(entity_type tea, table_name tea) QueryBuilder`
- `select_fields(builder QueryBuilder, fields []tea) QueryBuilder`
- `where_condition(builder QueryBuilder, field tea, operator tea, value tea) QueryBuilder`
- `where_in(builder QueryBuilder, field tea, values []tea) QueryBuilder`
- `or_where(builder QueryBuilder, field tea, operator tea, value tea) QueryBuilder`
- `join_table(builder QueryBuilder, join_type tea, table_name tea, table_alias tea, on_condition tea) QueryBuilder`
- `order_by(builder QueryBuilder, field tea, direction tea) QueryBuilder`
- `group_by(builder QueryBuilder, fields []tea) QueryBuilder`
- `limit_results(builder QueryBuilder, limit normie) QueryBuilder`
- `offset_results(builder QueryBuilder, offset normie) QueryBuilder`
- `build_select_query(builder QueryBuilder) tea`

### Schema Management

#### Schema
Represents a complete database schema with tables, migrations, and constraints.

```cursed
be_like Schema = struct {
    version tea
    tables map[tea]TableSchema
    migrations []Migration
    foreign_keys []ForeignKeyConstraint
    indexes []IndexSchema
}
```

#### TableSchema
Defines the structure of a database table.

```cursed
be_like TableSchema = struct {
    name tea
    columns map[tea]ColumnSchema
    primary_keys []tea
    foreign_keys []ForeignKeyConstraint
    indexes []IndexSchema
    constraints []TableConstraint
    engine tea
    charset tea
}
```

#### Key Functions

- `create_schema(version tea) Schema`
- `create_table_schema(name tea, engine tea, charset tea) TableSchema`
- `add_column_to_table(table_schema TableSchema, column_name tea, data_type tea, nullable lit) TableSchema`
- `generate_create_table_sql(table_schema TableSchema) tea`

### Connection Management

#### ConnectionPool
Enterprise-grade connection pooling with health monitoring.

```cursed
be_like ConnectionPool = struct {
    config ConnectionPoolConfig
    connections []DatabaseConnection
    active_connections normie
    available_connections normie
    total_connections_created normie
    pool_stats ConnectionPoolStats
    is_closed lit
}
```

#### ConnectionPoolConfig
Configuration for connection pool behavior.

```cursed
be_like ConnectionPoolConfig = struct {
    max_connections normie
    min_connections normie
    max_idle_time_seconds normie
    connection_timeout_seconds normie
    validation_query tea
    retry_attempts normie
    retry_delay_ms normie
}
```

#### Key Functions

- `create_connection_pool(config ConnectionPoolConfig) ConnectionPool`
- `get_connection_from_pool(pool ConnectionPool) DatabaseConnection`
- `return_connection_to_pool(pool ConnectionPool, connection DatabaseConnection) lit`

### Transaction Management

#### TransactionContext
ACID-compliant transaction context with isolation levels.

```cursed
be_like TransactionContext = struct {
    connection_id tea
    transaction_id tea
    isolation_level tea
    is_active lit
    savepoints []tea
    started_at timez.Time
    timeout_seconds normie
    rollback_only lit
}
```

#### Key Functions

- `begin_orm_transaction(connection DatabaseConnection, isolation_level tea) TransactionContext`
- `commit_orm_transaction(transaction TransactionContext) lit`
- `rollback_orm_transaction(transaction TransactionContext) lit`
- `add_savepoint(transaction TransactionContext, savepoint_name tea) TransactionContext`

### Repository Pattern

#### Repository
Clean data access layer with caching and event support.

```cursed
be_like Repository = struct {
    entity_type tea
    connection_pool ConnectionPool
    query_cache QueryCache
    event_dispatcher EventDispatcher
}
```

#### Key Functions

- `create_repository(entity_type tea, pool ConnectionPool) Repository`
- `find_by_id(repository Repository, id tea) Entity`
- `find_where(repository Repository, conditions []WhereCondition) []Entity`
- `find_all(repository Repository) []Entity`
- `save_entity(repository Repository, entity Entity) Entity`
- `delete_entity(repository Repository, entity Entity) lit`

### Migration System

#### Migration
Version-controlled database schema changes.

```cursed
be_like Migration = struct {
    version tea
    description tea
    up_sql []tea
    down_sql []tea
    checksum tea
    applied_at tea
    execution_time_ms normie
}
```

#### Key Functions

- `create_migration(version tea, description tea, up_sql []tea, down_sql []tea) Migration`
- `apply_migration(connection DatabaseConnection, migration Migration) lit`
- `rollback_migration(connection DatabaseConnection, migration Migration) lit`

## Supported SQL Operations

### Query Types
- SELECT with complex JOINs, WHERE clauses, GROUP BY, HAVING, ORDER BY
- INSERT with multiple values and ON DUPLICATE KEY UPDATE
- UPDATE with conditional WHERE clauses
- DELETE with safety constraints

### Join Types
- INNER JOIN
- LEFT JOIN (LEFT OUTER JOIN)
- RIGHT JOIN (RIGHT OUTER JOIN)  
- FULL JOIN (FULL OUTER JOIN)

### WHERE Operators
- `=`, `!=`, `<>`, `<`, `>`, `<=`, `>=`
- `IN`, `NOT IN`
- `LIKE`, `NOT LIKE`
- `BETWEEN`, `NOT BETWEEN`
- `IS NULL`, `IS NOT NULL`

### Advanced Features
- Subqueries and CTEs (Common Table Expressions)
- Window functions
- Aggregate functions (COUNT, SUM, AVG, MIN, MAX)
- DISTINCT and GROUP BY operations
- LIMIT and OFFSET for pagination
- FOR UPDATE locking

## Database Support

The ORM is designed to work with multiple database systems:

### SQL Databases
- **PostgreSQL**: Full feature support including advanced data types
- **MySQL/MariaDB**: Complete compatibility with MySQL-specific features
- **SQLite**: Lightweight embedded database support
- **SQL Server**: Enterprise database integration
- **Oracle**: Enterprise-grade Oracle database support

### NoSQL Databases
- **MongoDB**: Document-oriented database support
- **Redis**: Key-value store integration
- **Cassandra**: Distributed database support

## Performance Optimization

### Query Optimization
- **Query Caching**: Configurable result caching with TTL
- **Connection Pooling**: Efficient connection reuse
- **Prepared Statements**: SQL injection prevention and performance
- **Lazy Loading**: Load relationships only when needed
- **Batch Operations**: Bulk insert/update operations

### Monitoring
- **Query Performance**: Execution time tracking
- **Connection Health**: Pool statistics and monitoring
- **Memory Usage**: Entity lifecycle management
- **Error Tracking**: Comprehensive error logging

## Security Features

### SQL Injection Prevention
- **Parameter Binding**: All queries use parameter binding
- **Input Sanitization**: Automatic escaping of SQL special characters
- **Query Validation**: Syntax validation before execution

### Access Control
- **Connection Security**: Secure connection string handling
- **Transaction Isolation**: ACID compliance with configurable isolation levels
- **Audit Logging**: Track all database operations

## Error Handling

The ORM provides comprehensive error handling:

```cursed
# Entity validation errors
sus errors []tea = validate_entity(entity)
yikes stringz.length(errors) > 0 {
    bestie i := 0; i < stringz.length(errors); i++ {
        vibez.spill(stringz.concat("Validation error: ", errors[i]))
    }
}

# Transaction error handling
sus transaction TransactionContext = begin_orm_transaction(connection, "READ_COMMITTED")
# ... perform operations
yikes transaction.rollback_only {
    rollback_orm_transaction(transaction)
    vibez.spill("Transaction rolled back due to error")
} shook {
    commit_orm_transaction(transaction)
}
```

## Testing

The module includes comprehensive test coverage:

```bash
# Run all ORM tests
cargo run --bin cursed stdlib/database_orm/test_database_orm.💀

# Test specific functionality
cargo run --bin cursed -- compile stdlib/database_orm/test_database_orm.💀
./test_database_orm
```

### Test Categories
- **Entity Management**: Create, read, update, delete operations
- **Query Builder**: Complex query construction and SQL generation
- **Schema Management**: Table creation, migrations, constraints
- **Connection Pooling**: Pool lifecycle and resource management
- **Transaction Management**: ACID compliance and rollback scenarios
- **Repository Pattern**: Data access layer functionality
- **Validation**: Entity validation and error handling
- **Relationships**: Association loading and management

## Best Practices

### Entity Design
1. **Use meaningful table and column names**
2. **Define relationships explicitly**
3. **Implement validation rules**
4. **Use appropriate data types**

### Query Optimization
1. **Use indexes for frequently queried columns**
2. **Limit result sets with LIMIT/OFFSET**
3. **Use joins instead of multiple queries**
4. **Cache frequently accessed data**

### Transaction Management
1. **Keep transactions short**
2. **Use appropriate isolation levels**
3. **Handle rollback scenarios**
4. **Use savepoints for complex operations**

### Connection Management
1. **Configure appropriate pool sizes**
2. **Monitor connection health**
3. **Handle connection failures gracefully**
4. **Close connections properly**

## Integration with Other Modules

The database ORM integrates seamlessly with other CURSED stdlib modules:

- **timez**: Timestamp and date/time handling
- **stringz**: String manipulation and formatting
- **error_drip**: Error handling and propagation
- **concurrenz**: Concurrent operations and thread safety
- **atomic_drip**: Atomic operations for counters and flags
- **collections**: Data structure support
- **reflect**: Runtime type inspection

## Contributing

This is a pure CURSED implementation without FFI dependencies. When contributing:

1. **Maintain FFI-free design**
2. **Follow existing code patterns**
3. **Add comprehensive tests**
4. **Update documentation**
5. **Ensure backward compatibility**

## License

This module is part of the CURSED standard library and follows the project's licensing terms.
