# SQLSlay (database/sql package)

## Overview
SQLSlay provides a generic interface around SQL (or SQL-like) databases with a focus on elegance and simplicity. It's inspired by Go's database/sql package but with modern enhancements and a "slaying it" approach to database access.

## Core Types

### `DB`
Represents a database connection pool.

```go
type DB struct {}

// Constructor
func Open(driverName, dataSourceName string) (*DB, error)

// Methods
func (db *DB) Begin() (*Tx, error)
func (db *DB) BeginTx(ctx VibeContext, opts *TxOptions) (*Tx, error)
func (db *DB) Close() error
func (db *DB) Conn(ctx VibeContext) (*Conn, error)
func (db *DB) Driver() driver.Driver
func (db *DB) Exec(query string, args ...interface{}) (Result, error)
func (db *DB) ExecContext(ctx VibeContext, query string, args ...interface{}) (Result, error)
func (db *DB) Ping() error
func (db *DB) PingContext(ctx VibeContext) error
func (db *DB) Prepare(query string) (*Stmt, error)
func (db *DB) PrepareContext(ctx VibeContext, query string) (*Stmt, error)
func (db *DB) Query(query string, args ...interface{}) (*Rows, error)
func (db *DB) QueryContext(ctx VibeContext, query string, args ...interface{}) (*Rows, error)
func (db *DB) QueryRow(query string, args ...interface{}) *Row
func (db *DB) QueryRowContext(ctx VibeContext, query string, args ...interface{}) *Row
func (db *DB) SetConnMaxIdleTime(d time.Duration)
func (db *DB) SetConnMaxLifetime(d time.Duration)
func (db *DB) SetMaxIdleConns(n int)
func (db *DB) SetMaxOpenConns(n int)
func (db *DB) Stats() DBStats

// Enhanced Methods
func (db *DB) SlayQuery(query string, args ...interface{}) *SlayRows
func (db *DB) SlayExec(query string, args ...interface{}) (SlayResult, error)
func (db *DB) MapQuery(query string, args ...interface{}) ([]map[string]interface{}, error)
func (db *DB) StructQuery(query string, dest interface{}, args ...interface{}) error
func (db *DB) BatchExec(queries []string) ([]SlayResult, error)
```

### `Conn`
Represents a single database connection rather than a pool.

```go
type Conn struct {}

// Methods
func (c *Conn) BeginTx(ctx VibeContext, opts *TxOptions) (*Tx, error)
func (c *Conn) Close() error
func (c *Conn) ExecContext(ctx VibeContext, query string, args ...interface{}) (Result, error)
func (c *Conn) PingContext(ctx VibeContext) error
func (c *Conn) PrepareContext(ctx VibeContext, query string) (*Stmt, error)
func (c *Conn) QueryContext(ctx VibeContext, query string, args ...interface{}) (*Rows, error)
func (c *Conn) QueryRowContext(ctx VibeContext, query string, args ...interface{}) *Row
func (c *Conn) Raw(f func(driverConn interface{}) error) error
```

### `Tx`
Represents a database transaction.

```go
type Tx struct {}

// Methods
func (tx *Tx) Commit() error
func (tx *Tx) Exec(query string, args ...interface{}) (Result, error)
func (tx *Tx) ExecContext(ctx VibeContext, query string, args ...interface{}) (Result, error)
func (tx *Tx) Prepare(query string) (*Stmt, error)
func (tx *Tx) PrepareContext(ctx VibeContext, query string) (*Stmt, error)
func (tx *Tx) Query(query string, args ...interface{}) (*Rows, error)
func (tx *Tx) QueryContext(ctx VibeContext, query string, args ...interface{}) (*Rows, error)
func (tx *Tx) QueryRow(query string, args ...interface{}) *Row
func (tx *Tx) QueryRowContext(ctx VibeContext, query string, args ...interface{}) *Row
func (tx *Tx) Rollback() error
func (tx *Tx) Stmt(stmt *Stmt) *Stmt
func (tx *Tx) StmtContext(ctx VibeContext, stmt *Stmt) *Stmt

// Enhanced Methods
func (tx *Tx) SlayQuery(query string, args ...interface{}) *SlayRows
func (tx *Tx) SlayExec(query string, args ...interface{}) (SlayResult, error)
func (tx *Tx) MapQuery(query string, args ...interface{}) ([]map[string]interface{}, error)
func (tx *Tx) StructQuery(query string, dest interface{}, args ...interface{}) error
```

### `Stmt`
Represents a prepared statement.

```go
type Stmt struct {}

// Methods
func (s *Stmt) Close() error
func (s *Stmt) Exec(args ...interface{}) (Result, error)
func (s *Stmt) ExecContext(ctx VibeContext, args ...interface{}) (Result, error)
func (s *Stmt) Query(args ...interface{}) (*Rows, error)
func (s *Stmt) QueryContext(ctx VibeContext, args ...interface{}) (*Rows, error)
func (s *Stmt) QueryRow(args ...interface{}) *Row
func (s *Stmt) QueryRowContext(ctx VibeContext, args ...interface{}) *Row
```

### `Row`
Represents a single row returned by a query.

```go
type Row struct {}

// Methods
func (r *Row) Err() error
func (r *Row) Scan(dest ...interface{}) error

// Enhanced Methods
func (r *Row) ScanMap() (map[string]interface{}, error)
func (r *Row) ScanStruct(dest interface{}) error
```

### `Rows`
Represents multiple rows returned by a query.

```go
type Rows struct {}

// Methods
func (r *Rows) Close() error
func (r *Rows) ColumnTypes() ([]*ColumnType, error)
func (r *Rows) Columns() ([]string, error)
func (r *Rows) Err() error
func (r *Rows) Next() bool
func (r *Rows) NextResultSet() bool
func (r *Rows) Scan(dest ...interface{}) error

// Enhanced Methods
func (r *Rows) ScanMap() (map[string]interface{}, error)
func (r *Rows) ScanStruct(dest interface{}) error
func (r *Rows) ScanAll(dest interface{}) error // Scans all rows into a slice of structs
```

### `SlayResult`
Represents the result of a database operation.

```go
type SlayResult interface {
    LastInsertId() (int64, error)
    RowsAffected() (int64, error)
    // Enhanced Methods
    Success() bool
    Error() error
    String() string
}
```

### `SlayRows`
An enhanced version of Rows with additional functionality.

```go
type SlayRows struct {
    *Rows
}

// Methods (in addition to Rows methods)
func (r *SlayRows) All() ([]map[string]interface{}, error)
func (r *SlayRows) AllStructs(dest interface{}) error
func (r *SlayRows) First() (map[string]interface{}, error)
func (r *SlayRows) FirstStruct(dest interface{}) error
func (r *SlayRows) Count() (int, error)
func (r *SlayRows) ForEach(fn func(map[string]interface{}) error) error
func (r *SlayRows) ToJSON() ([]byte, error)
```

## Transaction Options

```go
type TxOptions struct {
    Isolation IsolationLevel
    ReadOnly  bool
}

type IsolationLevel int

const (
    LevelDefault IsolationLevel = iota
    LevelReadUncommitted
    LevelReadCommitted
    LevelWriteCommitted
    LevelRepeatableRead
    LevelSnapshot
    LevelSerializable
    LevelLinearizable
)
```

## Driver Interface

```go
type Driver interface {
    Open(name string) (Conn, error)
}

type Conn interface {
    Prepare(query string) (Stmt, error)
    Close() error
    Begin() (Tx, error)
}

func Register(name string, driver Driver)
```

## Query Builder

```go
type QueryBuilder struct {}

// Constructors
func NewQueryBuilder() *QueryBuilder
func NewSelectBuilder(table string) *SelectBuilder
func NewInsertBuilder(table string) *InsertBuilder
func NewUpdateBuilder(table string) *UpdateBuilder
func NewDeleteBuilder(table string) *DeleteBuilder

// Select Builder
type SelectBuilder struct {}

func (b *SelectBuilder) Columns(cols ...string) *SelectBuilder
func (b *SelectBuilder) From(table string) *SelectBuilder
func (b *SelectBuilder) Where(condition string, args ...interface{}) *SelectBuilder
func (b *SelectBuilder) OrderBy(orderBy string) *SelectBuilder
func (b *SelectBuilder) GroupBy(groupBy string) *SelectBuilder
func (b *SelectBuilder) Having(having string, args ...interface{}) *SelectBuilder
func (b *SelectBuilder) Limit(limit int) *SelectBuilder
func (b *SelectBuilder) Offset(offset int) *SelectBuilder
func (b *SelectBuilder) Join(joinType, table, condition string) *SelectBuilder
func (b *SelectBuilder) Build() (string, []interface{})
func (b *SelectBuilder) Exec(db *DB) (*SlayRows, error)
func (b *SelectBuilder) One(db *DB) (map[string]interface{}, error)
func (b *SelectBuilder) All(db *DB) ([]map[string]interface{}, error)
func (b *SelectBuilder) Count(db *DB) (int64, error)

// Insert Builder
type InsertBuilder struct {}

func (b *InsertBuilder) Columns(cols ...string) *InsertBuilder
func (b *InsertBuilder) Values(values ...interface{}) *InsertBuilder
func (b *InsertBuilder) Record(record interface{}) *InsertBuilder
func (b *InsertBuilder) Build() (string, []interface{})
func (b *InsertBuilder) Exec(db *DB) (SlayResult, error)
func (b *InsertBuilder) BatchInsert(records []interface{}) *InsertBuilder

// Update Builder
type UpdateBuilder struct {}

func (b *UpdateBuilder) Set(column string, value interface{}) *UpdateBuilder
func (b *UpdateBuilder) SetMap(data map[string]interface{}) *UpdateBuilder
func (b *UpdateBuilder) SetStruct(data interface{}) *UpdateBuilder
func (b *UpdateBuilder) Where(condition string, args ...interface{}) *UpdateBuilder
func (b *UpdateBuilder) Build() (string, []interface{})
func (b *UpdateBuilder) Exec(db *DB) (SlayResult, error)

// Delete Builder
type DeleteBuilder struct {}

func (b *DeleteBuilder) Where(condition string, args ...interface{}) *DeleteBuilder
func (b *DeleteBuilder) Build() (string, []interface{})
func (b *DeleteBuilder) Exec(db *DB) (SlayResult, error)
```

## Migrations

```go
type Migration struct {
    Version     int
    Description string
    Up          string
    Down        string
}

type Migrator struct {}

// Constructor
func NewMigrator(db *DB) *Migrator

// Methods
func (m *Migrator) AddMigration(migration Migration)
func (m *Migrator) MigrateUp() error
func (m *Migrator) MigrateDown() error
func (m *Migrator) MigrateTo(version int) error
func (m *Migrator) CurrentVersion() (int, error)
func (m *Migrator) ListMigrations() ([]Migration, error)
```

## Connection Pool Monitoring

```go
type DBStats struct {
    MaxOpenConnections int // Maximum number of open connections
    OpenConnections    int // Current number of open connections
    InUse              int // Number of connections currently in use
    Idle               int // Number of idle connections
    WaitCount          int64 // Total number of connections waited for
    WaitDuration       time.Duration // Total time waited for connections
    MaxIdleClosed      int64 // Total number of connections closed due to SetMaxIdleConns
    MaxLifetimeClosed  int64 // Total number of connections closed due to SetConnMaxLifetime
}

func (db *DB) StatsJSON() ([]byte, error)
func (db *DB) MonitorStats(interval time.Duration, callback func(stats DBStats))
```

## Usage Example

```go
// Open a database connection
db, err := sql_slay.Open("mysql", "user:password@tcp(localhost:3306)/dbname")
if err != nil {
    vibez.spill("Failed to connect to database:", err)
    return
}
defer db.Close()

// Simple query
rows, err := db.Query("SELECT id, name FROM users WHERE age > ?", 18)
if err != nil {
    vibez.spill("Query failed:", err)
    return
}
defer rows.Close()

// Iterating over rows
for rows.Next() {
    var id int
    var name string
    if err := rows.Scan(&id, &name); err != nil {
        vibez.spill("Scan failed:", err)
        return
    }
    vibez.spill(id, name)
}

// Enhanced queries
slayRows := db.SlayQuery("SELECT * FROM users WHERE age > ?", 18)
users, err := slayRows.All()
if err != nil {
    vibez.spill("Query failed:", err)
    return
}

for _, user := range users {
    vibez.spill(user["id"], user["name"])
}

// Struct mapping
type User struct {
    ID   int    `db:"id"`
    Name string `db:"name"`
    Age  int    `db:"age"`
}

var allUsers []User
if err := db.StructQuery("SELECT * FROM users", &allUsers); err != nil {
    vibez.spill("Query failed:", err)
    return
}

for _, user := range allUsers {
    vibez.spill(user.ID, user.Name, user.Age)
}

// Transactions
tx, err := db.Begin()
if err != nil {
    vibez.spill("Failed to start transaction:", err)
    return
}

// Perform multiple operations in a transaction
result, err := tx.Exec("UPDATE accounts SET balance = balance - ? WHERE id = ?", 100, 1)
if err != nil {
    tx.Rollback()
    vibez.spill("Failed to update account 1:", err)
    return
}

result, err = tx.Exec("UPDATE accounts SET balance = balance + ? WHERE id = ?", 100, 2)
if err != nil {
    tx.Rollback()
    vibez.spill("Failed to update account 2:", err)
    return
}

if err := tx.Commit(); err != nil {
    vibez.spill("Failed to commit transaction:", err)
    return
}

// Using query builders
select := sql_slay.NewSelectBuilder("users")
    .Columns("id", "name", "email")
    .Where("age > ?", 18)
    .OrderBy("name ASC")
    .Limit(10)

users, err := select.All(db)
if err != nil {
    vibez.spill("Query failed:", err)
    return
}

// Migrations
migrator := sql_slay.NewMigrator(db)
migrator.AddMigration(sql_slay.Migration{
    Version:     1,
    Description: "Create users table",
    Up:          "CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(255), age INT);",
    Down:        "DROP TABLE users;",
})

if err := migrator.MigrateUp(); err != nil {
    vibez.spill("Migration failed:", err)
    return
}
```

## Implementation Guidelines
1. Support all major SQL databases (MySQL, PostgreSQL, SQLite, SQL Server) with consistent behavior
2. Provide intelligent connection pooling with configurable limits
3. Implement proper resource cleanup to prevent connection leaks
4. Support both raw SQL and builder patterns for query construction
5. Implement context-aware methods for cancellation and timeouts
6. Ensure thread-safety for concurrent database operations
7. Provide clear error messages with specific database error codes