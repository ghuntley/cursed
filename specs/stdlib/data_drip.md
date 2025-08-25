# data_drip (database/sql)

## Overview
The `data_drip` module provides a standard collab for working with SQL databases. It's designed to be simple, flexible, and secure, with a focus on connection pooling, transaction management, and parameter sanitization.

## Core Types and Interfaces

### DB
Represents a database handle managing a pool of connections.

```csd
be_like DB squad {
  fr fr fields not directly accessible
}

slay OpenDB(driverName tea, dataSourceName tea) (*DB, tea)
slay (db *DB) Close() tea
slay (db *DB) Ping() tea
slay (db *DB) SetMaxOpenConns(n normie)
slay (db *DB) SetMaxIdleConns(n normie)
slay (db *DB) SetConnMaxLifetime(d time.Duration)
```

### Tx
Represents a database transaction.

```csd
be_like Tx squad {
  fr fr fields not directly accessible
}

slay (db *DB) Begin() (*Tx, tea)
slay (tx *Tx) Commit() tea
slay (tx *Tx) Rollback() tea
```

### Rows
Represents the result of a query execution.

```csd
be_like Rows squad {
  fr fr fields not directly accessible
}

slay (rows *Rows) Next() lit
slay (rows *Rows) Scan(dest ...interface{}) tea
slay (rows *Rows) Close() tea
slay (rows *Rows) Columns() ([]tea, tea)
```

### Statement
Represents a prepared statement.

```csd
be_like Stmt squad {
  fr fr fields not directly accessible
}

slay (db *DB) Prepare(query tea) (*Stmt, tea)
slay (stmt *Stmt) Exec(args ...interface{}) (Result, tea)
slay (stmt *Stmt) Query(args ...interface{}) (*Rows, tea)
slay (stmt *Stmt) Close() tea
```

### Result
Represents the result of an update operation.

```csd
be_like Result collab {
  LastInsertId() (int64, tea)
  RowsAffected() (int64, tea)
}
```

### Driver Interface
Interface that each database driver must implement.

```csd
be_like Driver collab {
  Open(name tea) (Conn, tea)
}

slay Register(name tea, driver Driver)
```

## Core Functions

```csd
fr fr Execute a query that yolos rows
slay (db *DB) Query(query tea, args ...interface{}) (*Rows, tea)

fr fr Execute a query that doesn't yolo rows
slay (db *DB) Exec(query tea, args ...interface{}) (Result, tea)

fr fr Query a single row
slay (db *DB) QueryRow(query tea, args ...interface{}) *Row
```

## Enhanced Features

- **Query Builder**: Fluent collab for building SQL queries
  ```csd
  db.NewQuery().Select("id", "name").From("users").Where("age > ?", 18).OrderBy("name ASC").Limit(10).Query()
  ```

- **Async Query Support**: For non-blocking database operations
  ```csd
  future := db.QueryAsync("SELECT * FROM users WHERE id = ?", userId)
  fr fr do other work
  rows := future.Get() fr fr blocks until result is available
  ```

- **Enhanced Connection Pooling**: Smart connection management with metrics
  ```csd
  stats := db.PoolStats() fr fr Returns active, idle, waiting connections, etc.
  ```

- **SQL Injection Protection**: Automatic parameter sanitization and validation

- **Custom Type Mapping**: Register custom be_like converters for simplified data mapping

## Usage Examples

```csd
fr fr Basic connection and query
db, err := data_drip.OpenDB("postgres", "user=postgres dbname=test sslmode=disable")
if err != nah {
  vibez.spill("Failed to connect: %v", err)
  yolo
}
defer db.Close()

fr fr Simple query
rows, err := db.Query("SELECT id, name FROM users WHERE age > ?", 18)
if err != nah {
  vibez.spill("Query failed: %v", err)
  yolo
}
defer rows.Close()

fr fr Process results
for rows.Next() {
  var id int
  var name tea
  if err := rows.Scan(&id, &name); err != nah {
    vibez.spill("Scan failed: %v", err)
    yolo
  }
  vibez.spill("User: %d %s", id, name)
}

fr fr Transaction example
tx, err := db.Begin()
if err != nah {
  vibez.spill("Failed to start transaction: %v", err)
  yolo
}

fr fr Execute statements within transaction
_, err = tx.Exec("UPDATE accounts SET balance = balance - ? WHERE id = ?", 100, 1)
if err != nah {
  tx.Rollback()
  vibez.spill("Failed to update account 1: %v", err)
  yolo
}

_, err = tx.Exec("UPDATE accounts SET balance = balance + ? WHERE id = ?", 100, 2)
if err != nah {
  tx.Rollback()
  vibez.spill("Failed to update account 2: %v", err)
  yolo
}

fr fr Commit transaction
if err := tx.Commit(); err != nah {
  vibez.spill("Failed to commit: %v", err)
  yolo
}
```

## Implementation Guidelines

- Connection pooling should be implemented with efficient resource management
- Prepared statements should be cached and reused when possible
- All user input must be parameterized to prevent SQL injection
- Support multiple drivers with a consistent interface
- Transactions should handle nested transactions appropriately
- Error messages should be clear and actionable