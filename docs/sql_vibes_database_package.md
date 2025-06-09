# SQL Vibes Database Package Documentation

The `sql_vibes` package provides comprehensive database connectivity for the CURSED programming language. Built with Gen Z energy and enterprise-grade features, it supports multiple database backends with a unified interface.

## Table of Contents

1. [Overview](#overview)
2. [Supported Databases](#supported-databases)
3. [Basic Usage](#basic-usage)
4. [Connection Management](#connection-management)
5. [Query Execution](#query-execution)
6. [Prepared Statements](#prepared-statements)
7. [Transactions](#transactions)
8. [Connection Pooling](#connection-pooling)
9. [Query Builder](#query-builder)
10. [Error Handling](#error-handling)
11. [Best Practices](#best-practices)
12. [API Reference](#api-reference)

## Overview

The `sql_vibes` package is designed to make database operations simple, safe, and performant. It follows the CURSED language conventions while providing all the features you need for serious database work.

### Key Features

- **Multiple Database Support**: SQLite, PostgreSQL, MySQL/MariaDB
- **Connection Pooling**: Enterprise-grade connection management
- **Prepared Statements**: SQL injection prevention with performance benefits
- **Transactions**: Full ACID compliance with savepoint support
- **Query Builder**: Type-safe SQL generation
- **Error Handling**: Comprehensive error types with helpful messages
- **Concurrent Access**: Thread-safe operations for high-performance applications
- **Mock Testing**: Built-in mock driver for testing

## Supported Databases

### SQLite
- **Connection String**: `sqlite://path/to/file.db` or `sqlite://:memory:`
- **Features**: Lightweight, file-based, perfect for development and small applications
- **Use Cases**: Local storage, prototyping, embedded applications

### PostgreSQL
- **Connection String**: `postgres://user:password@host:port/database`
- **Features**: Advanced SQL features, JSON support, excellent for web applications
- **Use Cases**: Web applications, analytical workloads, complex queries

### MySQL/MariaDB
- **Connection String**: `mysql://user:password@host:port/database`
- **Features**: Popular web database, good performance, wide hosting support
- **Use Cases**: Web applications, content management, traditional RDBMS needs

### Mock Driver
- **Connection String**: `mock://test`
- **Features**: In-memory simulation for testing
- **Use Cases**: Unit testing, development, CI/CD pipelines

## Basic Usage

### Connecting to a Database

```cursed
import sql_vibes

slay main() {
    // Connect to SQLite database
    sus connection_result = sql_vibes.connect("sqlite://users.db")
    lowkey connection_result.is_error() {
        vibez.spill("Connection failed:", connection_result.error())
        return
    }
    
    facts conn = connection_result.value()
    defer conn.close() // Always close connections periodt
    
    vibez.spill("Connected successfully!")
}
```

### Simple Query

```cursed
// Execute a simple query
sus result = conn.query("SELECT * FROM users WHERE active = ?", true)
lowkey result.is_error() {
    vibez.spill("Query failed:", result.error())
    return
}

facts result_set = result.value()
periodt row : result_set {
    sus name = row.get(1).as_string()
    sus email = row.get(2).as_string()
    vibez.spill("User:", name, "-", email)
}
```

## Connection Management

### Creating Connections

```cursed
// Different database types
sus sqlite_conn = sql_vibes.connect("sqlite://:memory:")
sus postgres_conn = sql_vibes.connect("postgres://user:pass@localhost/mydb")
sus mysql_conn = sql_vibes.connect("mysql://user:pass@localhost/mydb")
```

### Connection Configuration

```cursed
// Create connection with custom configuration
sus config = sql_vibes.ConnectionConfig {
    connection_string: "postgres://user:pass@localhost/mydb",
    timeout: 30, // seconds
    ssl_mode: "require",
    max_retries: 3
}

sus conn = sql_vibes.connect_with_config(config)
```

### Connection Health

```cursed
// Check if connection is alive
lowkey conn.is_alive() {
    vibez.spill("Connection is healthy")
} bestie {
    vibez.spill("Connection is dead - need to reconnect")
}

// Get connection information
sus info = conn.connection_info()
vibez.spill("Connected to:", info.database_name, "on", info.host)
```

## Query Execution

### Basic Queries

```cursed
// SELECT query
sus users = conn.query("SELECT id, name, email FROM users")

// INSERT with parameters
sus insert_result = conn.execute(
    "INSERT INTO users (name, email) VALUES (?, ?)",
    "John Doe", "john@example.com"
)

vibez.spill("Inserted user with", insert_result.value(), "rows affected")
```

### Batch Operations

```cursed
// Execute multiple statements
sus statements = [
    ["INSERT INTO users (name) VALUES (?)", ["Alice"]],
    ["INSERT INTO users (name) VALUES (?)", ["Bob"]],
    ["INSERT INTO users (name) VALUES (?)", ["Charlie"]]
]

sus batch_result = conn.execute_batch(statements)
periodt result : batch_result.value() {
    lowkey result.is_error() {
        vibez.spill("Batch statement failed:", result.error())
    }
}
```

## Prepared Statements

Prepared statements provide performance benefits and prevent SQL injection attacks.

### Creating and Using Prepared Statements

```cursed
// Prepare a statement
sus stmt_result = conn.prepare("SELECT * FROM users WHERE department = ? AND active = ?")
lowkey stmt_result.is_error() {
    vibez.spill("Failed to prepare statement:", stmt_result.error())
    return
}

facts stmt = stmt_result.value()
defer stmt.close() // Always close prepared statements

// Execute with different parameters
sus engineering_users = stmt.execute("Engineering", true)
sus marketing_users = stmt.execute("Marketing", true)
sus inactive_users = stmt.execute("Engineering", false)
```

### Prepared Statement for Updates

```cursed
sus update_stmt = conn.prepare("UPDATE users SET last_login = ? WHERE id = ?")
facts stmt = update_stmt.value()
defer stmt.close()

// Update multiple users
sus user_ids = [1, 2, 3, 4, 5]
periodt user_id : user_ids {
    sus affected = stmt.execute_update(timez.now(), user_id)
    vibez.spill("Updated user", user_id, "-", affected.value(), "rows")
}
```

## Transactions

Transactions ensure data consistency and allow rollback of changes.

### Basic Transaction Usage

```cursed
// Begin transaction
sus txn_result = conn.begin_transaction()
facts txn = txn_result.value()

// Execute operations within transaction
sus insert1 = txn.execute_statement("INSERT INTO accounts (name, balance) VALUES (?, ?)", "Alice", 1000)
sus insert2 = txn.execute_statement("INSERT INTO accounts (name, balance) VALUES (?, ?)", "Bob", 500)

// Transfer money between accounts
sus debit = txn.execute_statement("UPDATE accounts SET balance = balance - ? WHERE name = ?", 100, "Alice")
sus credit = txn.execute_statement("UPDATE accounts SET balance = balance + ? WHERE name = ?", 100, "Bob")

// Check if all operations succeeded
lowkey insert1.is_error() || insert2.is_error() || debit.is_error() || credit.is_error() {
    vibez.spill("Transaction failed, rolling back")
    txn.rollback()
    return
}

// Commit the transaction
sus commit_result = txn.commit()
lowkey commit_result.is_error() {
    vibez.spill("Failed to commit:", commit_result.error())
} bestie {
    vibez.spill("Money transfer completed successfully!")
}
```

### Savepoints

```cursed
// Create savepoints for partial rollback
sus txn = conn.begin_transaction().value()

sus initial_insert = txn.execute_statement("INSERT INTO logs (message) VALUES (?)", "Transaction started")

// Create savepoint
txn.savepoint("before_risky_operation")

sus risky_operation = txn.execute_statement("UPDATE critical_data SET value = ?", "new_value")
lowkey risky_operation.is_error() {
    // Rollback to savepoint instead of entire transaction
    txn.rollback_to_savepoint("before_risky_operation")
    vibez.spill("Rolled back risky operation, but keeping initial insert")
}

txn.commit()
```

## Connection Pooling

Connection pooling improves performance in multi-threaded applications.

### Creating a Connection Pool

```cursed
// Create pool with configuration
sus pool_config = sql_vibes.PoolConfig {
    max_connections: 10,
    min_connections: 2,
    connection_timeout: 30,
    idle_timeout: 300
}

sus pool = sql_vibes.create_pool("postgres://user:pass@localhost/mydb", pool_config)
defer pool.close()
```

### Using Pooled Connections

```cursed
// Get connection from pool
sus conn_result = pool.get_connection()
facts conn = conn_result.value()
defer pool.return_connection(conn) // Return to pool when done

// Use connection normally
sus result = conn.query("SELECT COUNT(*) FROM users")
```

### Pool Statistics

```cursed
// Monitor pool health
sus stats = pool.stats()
vibez.spill("Pool stats:")
vibez.spill("  Active connections:", stats.active_connections)
vibez.spill("  Idle connections:", stats.idle_connections)
vibez.spill("  Total requests:", stats.total_requests)
vibez.spill("  Failed requests:", stats.failed_requests)
```

## Query Builder

The query builder provides a type-safe way to construct SQL queries.

### SELECT Queries

```cursed
// Build SELECT query
sus query = sql_vibes.select("id", "name", "email")
    .from("users")
    .where("active = ?", true)
    .where("department = ?", "Engineering")
    .order_by("name ASC")
    .limit(10)

sus result = conn.query(query.build(), query.parameters())
```

### INSERT Queries

```cursed
// Build INSERT query
sus insert_query = sql_vibes.insert_into("users")
    .values("name", "John Doe")
    .values("email", "john@example.com")
    .values("department", "Engineering")
    .values("active", true)

sus result = conn.execute(insert_query.build(), insert_query.parameters())
```

### UPDATE Queries

```cursed
// Build UPDATE query
sus update_query = sql_vibes.update("users")
    .set("last_login", timez.now())
    .set("login_count", "login_count + 1")
    .where("id = ?", user_id)

sus affected = conn.execute(update_query.build(), update_query.parameters())
```

### Complex Joins

```cursed
// Build complex query with joins
sus complex_query = sql_vibes.select("u.name", "d.department_name", "p.project_name")
    .from("users u")
    .inner_join("departments d", "u.department_id = d.id")
    .left_join("user_projects up", "u.id = up.user_id")
    .left_join("projects p", "up.project_id = p.id")
    .where("u.active = ?", true)
    .order_by("u.name, p.project_name")

sus result = conn.query(complex_query.build(), complex_query.parameters())
```

## Error Handling

The `sql_vibes` package provides comprehensive error handling with descriptive error types.

### Error Types

```cursed
// Handle different error types
sus result = conn.query("SELECT * FROM nonexistent_table")
lowkey result.is_error() {
    vibe_check result.error() {
        mood sql_vibes.ConnectionError(msg) -> {
            vibez.spill("Connection error:", msg)
        }
        mood sql_vibes.QueryError(msg) -> {
            vibez.spill("Query error:", msg)
        }
        mood sql_vibes.TransactionError(msg) -> {
            vibez.spill("Transaction error:", msg)
        }
        basic -> {
            vibez.spill("Unknown database error:", result.error())
        }
    }
}
```

### Error Recovery

```cursed
slay robust_query_with_retry(conn: sql_vibes.Connection, sql: String, max_retries: i32) -> sql_vibes.ResultSet? {
    periodt attempt : 1..=max_retries {
        sus result = conn.query(sql)
        lowkey result.is_ok() {
            return result.value()
        }
        
        vibez.spill("Query attempt", attempt, "failed:", result.error())
        lowkey attempt < max_retries {
            timez.sleep(1000) // Wait 1 second before retry
        }
    }
    
    return nil
}
```

## Best Practices

### 1. Always Close Resources

```cursed
// Use defer to ensure resources are cleaned up
sus conn = sql_vibes.connect("sqlite://data.db").value()
defer conn.close()

sus stmt = conn.prepare("SELECT * FROM users WHERE id = ?").value()
defer stmt.close()
```

### 2. Use Transactions for Related Operations

```cursed
// Group related operations in transactions
slay transfer_money(conn: sql_vibes.Connection, from_account: i32, to_account: i32, amount: f64) -> Bool {
    sus txn = conn.begin_transaction().value()
    
    // All operations succeed or all fail
    sus debit = txn.execute_statement("UPDATE accounts SET balance = balance - ? WHERE id = ?", amount, from_account)
    sus credit = txn.execute_statement("UPDATE accounts SET balance = balance + ? WHERE id = ?", amount, to_account)
    
    lowkey debit.is_error() || credit.is_error() {
        txn.rollback()
        return false
    }
    
    return txn.commit().is_ok()
}
```

### 3. Use Prepared Statements for Repeated Queries

```cursed
// Prepare once, execute many times
sus stmt = conn.prepare("INSERT INTO logs (timestamp, level, message) VALUES (?, ?, ?)").value()
defer stmt.close()

periodt log_entry : log_entries {
    stmt.execute_update(log_entry.timestamp, log_entry.level, log_entry.message)
}
```

### 4. Use Connection Pooling for Concurrent Applications

```cursed
// Share a pool across goroutines
sus pool = sql_vibes.create_pool("postgres://user:pass@localhost/mydb", pool_config).value()

periodt i : 0..10 {
    yolo {
        sus conn = pool.get_connection().value()
        defer pool.return_connection(conn)
        
        // Do work with connection
        process_data(conn)
    }()
}
```

### 5. Validate Input and Handle Errors

```cursed
slay safe_user_lookup(conn: sql_vibes.Connection, user_id: i32) -> User? {
    lowkey user_id <= 0 {
        vibez.spill("Invalid user ID:", user_id)
        return nil
    }
    
    sus result = conn.query("SELECT id, name, email FROM users WHERE id = ?", user_id)
    lowkey result.is_error() {
        vibez.spill("Database error:", result.error())
        return nil
    }
    
    facts result_set = result.value()
    lowkey result_set.is_empty() {
        return nil // User not found
    }
    
    facts row = result_set.first_row()
    return User {
        id: row.get(0).as_integer(),
        name: row.get(1).as_string(),
        email: row.get(2).as_string()
    }
}
```

## API Reference

### Connection Functions

| Function | Description | Example |
|----------|-------------|---------|
| `sql_vibes.connect(connection_string)` | Connect to database | `sql_vibes.connect("sqlite://db.sqlite")` |
| `sql_vibes.create_pool(connection_string, config)` | Create connection pool | `sql_vibes.create_pool("postgres://...", config)` |
| `connection.close()` | Close connection | `conn.close()` |
| `connection.is_alive()` | Check connection health | `conn.is_alive()` |

### Query Functions

| Function | Description | Example |
|----------|-------------|---------|
| `connection.query(sql, params...)` | Execute SELECT query | `conn.query("SELECT * FROM users WHERE id = ?", 1)` |
| `connection.execute(sql, params...)` | Execute INSERT/UPDATE/DELETE | `conn.execute("INSERT INTO users (name) VALUES (?)", "John")` |
| `connection.execute_batch(statements)` | Execute multiple statements | `conn.execute_batch(statements)` |

### Prepared Statement Functions

| Function | Description | Example |
|----------|-------------|---------|
| `connection.prepare(sql)` | Prepare statement | `conn.prepare("SELECT * FROM users WHERE id = ?")` |
| `statement.execute(params...)` | Execute prepared query | `stmt.execute(1)` |
| `statement.execute_update(params...)` | Execute prepared update | `stmt.execute_update("John", 1)` |
| `statement.close()` | Close prepared statement | `stmt.close()` |

### Transaction Functions

| Function | Description | Example |
|----------|-------------|---------|
| `connection.begin_transaction()` | Start transaction | `conn.begin_transaction()` |
| `transaction.commit()` | Commit transaction | `txn.commit()` |
| `transaction.rollback()` | Rollback transaction | `txn.rollback()` |
| `transaction.savepoint(name)` | Create savepoint | `txn.savepoint("sp1")` |
| `transaction.rollback_to_savepoint(name)` | Rollback to savepoint | `txn.rollback_to_savepoint("sp1")` |

### Query Builder Functions

| Function | Description | Example |
|----------|-------------|---------|
| `sql_vibes.select(columns...)` | Start SELECT builder | `sql_vibes.select("id", "name")` |
| `sql_vibes.insert_into(table)` | Start INSERT builder | `sql_vibes.insert_into("users")` |
| `sql_vibes.update(table)` | Start UPDATE builder | `sql_vibes.update("users")` |
| `sql_vibes.delete_from(table)` | Start DELETE builder | `sql_vibes.delete_from("users")` |

---

*This documentation covers the major features of the `sql_vibes` package. For more examples and advanced usage, check out the example programs in the `examples/` directory. The package is designed to be intuitive and follows CURSED language conventions while providing enterprise-grade database functionality periodt!*
