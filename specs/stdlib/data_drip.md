# data_drip (database/sql)

## Overview
The `data_drip` module provides a standard interface for working with SQL databases. It's designed to be simple, flexible, and secure, with a focus on connection pooling, transaction management, and parameter sanitization.

## Core Types and Interfaces

### DB
Represents a database handle managing a pool of connections.

```csd
type DB struct {
  // fields not directly accessible
}

func OpenDB(driverName string, dataSourceName string) (*DB, error)
func (db *DB) Close() error
func (db *DB) Ping() error
func (db *DB) SetMaxOpenConns(n int)
func (db *DB) SetMaxIdleConns(n int)
func (db *DB) SetConnMaxLifetime(d time.Duration)
```

### Tx
Represents a database transaction.

```csd
type Tx struct {
  // fields not directly accessible
}

func (db *DB) Begin() (*Tx, error)
func (tx *Tx) Commit() error
func (tx *Tx) Rollback() error
```

### Rows
Represents the result of a query execution.

```csd
type Rows struct {
  // fields not directly accessible
}

func (rows *Rows) Next() bool
func (rows *Rows) Scan(dest ...interface{}) error
func (rows *Rows) Close() error
func (rows *Rows) Columns() ([]string, error)
```

### Statement
Represents a prepared statement.

```csd
type Stmt struct {
  // fields not directly accessible
}

func (db *DB) Prepare(query string) (*Stmt, error)
func (stmt *Stmt) Exec(args ...interface{}) (Result, error)
func (stmt *Stmt) Query(args ...interface{}) (*Rows, error)
func (stmt *Stmt) Close() error
```

### Result
Represents the result of an update operation.

```csd
type Result interface {
  LastInsertId() (int64, error)
  RowsAffected() (int64, error)
}
```

### Driver Interface
Interface that each database driver must implement.

```csd
type Driver interface {
  Open(name string) (Conn, error)
}

func Register(name string, driver Driver)
```

## Core Functions

```csd
// Execute a query that returns rows
func (db *DB) Query(query string, args ...interface{}) (*Rows, error)

// Execute a query that doesn't return rows
func (db *DB) Exec(query string, args ...interface{}) (Result, error)

// Query a single row
func (db *DB) QueryRow(query string, args ...interface{}) *Row
```

## Enhanced Features

- **Query Builder**: Fluent interface for building SQL queries
  ```csd
  db.NewQuery().Select("id", "name").From("users").Where("age > ?", 18).OrderBy("name ASC").Limit(10).Query()
  ```

- **Async Query Support**: For non-blocking database operations
  ```csd
  future := db.QueryAsync("SELECT * FROM users WHERE id = ?", userId)
  // do other work
  rows := future.Get() // blocks until result is available
  ```

- **Enhanced Connection Pooling**: Smart connection management with metrics
  ```csd
  stats := db.PoolStats() // Returns active, idle, waiting connections, etc.
  ```

- **SQL Injection Protection**: Automatic parameter sanitization and validation

- **Custom Type Mapping**: Register custom type converters for simplified data mapping

## Usage Examples

```csd
// Basic connection and query
db, err := data_drip.OpenDB("postgres", "user=postgres dbname=test sslmode=disable")
if err != nil {
  vibez.spill("Failed to connect: %v", err)
  return
}
defer db.Close()

// Simple query
rows, err := db.Query("SELECT id, name FROM users WHERE age > ?", 18)
if err != nil {
  vibez.spill("Query failed: %v", err)
  return
}
defer rows.Close()

// Process results
for rows.Next() {
  var id int
  var name string
  if err := rows.Scan(&id, &name); err != nil {
    vibez.spill("Scan failed: %v", err)
    return
  }
  vibez.spill("User: %d %s", id, name)
}

// Transaction example
tx, err := db.Begin()
if err != nil {
  vibez.spill("Failed to start transaction: %v", err)
  return
}

// Execute statements within transaction
_, err = tx.Exec("UPDATE accounts SET balance = balance - ? WHERE id = ?", 100, 1)
if err != nil {
  tx.Rollback()
  vibez.spill("Failed to update account 1: %v", err)
  return
}

_, err = tx.Exec("UPDATE accounts SET balance = balance + ? WHERE id = ?", 100, 2)
if err != nil {
  tx.Rollback()
  vibez.spill("Failed to update account 2: %v", err)
  return
}

// Commit transaction
if err := tx.Commit(); err != nil {
  vibez.spill("Failed to commit: %v", err)
  return
}
```

## Implementation Guidelines

- Connection pooling should be implemented with efficient resource management
- Prepared statements should be cached and reused when possible
- All user input must be parameterized to prevent SQL injection
- Support multiple drivers with a consistent interface
- Transactions should handle nested transactions appropriately
- Error messages should be clear and actionable