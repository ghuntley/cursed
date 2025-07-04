# SQLSlay (database/sql package)

## Overview
SQLSlay provides a generic collab around SQL (or SQL-like) databases with a focus on elegance and simplicity. It's inspired by Go's database/sql package but with modern enhancements and a "slaying it" approach to database access.

## Core Types

### `DB`
Represents a database connection pool.

```
be_like DB squad {}

fr fr Consquador
slay Open(driverName, dataSourceName tea) (*DB, tea)

fr fr Methods
slay (db *DB) Begin() (*Tx, tea)
slay (db *DB) BeginTx(ctx VibeContext, opts *TxOptions) (*Tx, tea)
slay (db *DB) Close() tea
slay (db *DB) Conn(ctx VibeContext) (*Conn, tea)
slay (db *DB) Driver() driver.Driver
slay (db *DB) Exec(query tea, args ...interface{}) (Result, tea)
slay (db *DB) ExecContext(ctx VibeContext, query tea, args ...interface{}) (Result, tea)
slay (db *DB) Ping() tea
slay (db *DB) PingContext(ctx VibeContext) tea
slay (db *DB) Prepare(query tea) (*Stmt, tea)
slay (db *DB) PrepareContext(ctx VibeContext, query tea) (*Stmt, tea)
slay (db *DB) Query(query tea, args ...interface{}) (*Rows, tea)
slay (db *DB) QueryContext(ctx VibeContext, query tea, args ...interface{}) (*Rows, tea)
slay (db *DB) QueryRow(query tea, args ...interface{}) *Row
slay (db *DB) QueryRowContext(ctx VibeContext, query tea, args ...interface{}) *Row
slay (db *DB) SetConnMaxIdleTime(d time.Duration)
slay (db *DB) SetConnMaxLifetime(d time.Duration)
slay (db *DB) SetMaxIdleConns(n normie)
slay (db *DB) SetMaxOpenConns(n normie)
slay (db *DB) Stats() DBStats

fr fr Enhanced Methods
slay (db *DB) SlayQuery(query tea, args ...interface{}) *SlayRows
slay (db *DB) SlayExec(query tea, args ...interface{}) (SlayResult, tea)
slay (db *DB) MapQuery(query tea, args ...interface{}) ([]map[tea]interface{}, tea)
slay (db *DB) StructQuery(query tea, dest interface{}, args ...interface{}) tea
slay (db *DB) BatchExec(queries []tea) ([]SlayResult, tea)
```

### `Conn`
Represents a single database connection rather than a pool.

```
be_like Conn squad {}

fr fr Methods
slay (c *Conn) BeginTx(ctx VibeContext, opts *TxOptions) (*Tx, tea)
slay (c *Conn) Close() tea
slay (c *Conn) ExecContext(ctx VibeContext, query tea, args ...interface{}) (Result, tea)
slay (c *Conn) PingContext(ctx VibeContext) tea
slay (c *Conn) PrepareContext(ctx VibeContext, query tea) (*Stmt, tea)
slay (c *Conn) QueryContext(ctx VibeContext, query tea, args ...interface{}) (*Rows, tea)
slay (c *Conn) QueryRowContext(ctx VibeContext, query tea, args ...interface{}) *Row
slay (c *Conn) Raw(f func(driverConn interface{}) tea) tea
```

### `Tx`
Represents a database transaction.

```
be_like Tx squad {}

fr fr Methods
slay (tx *Tx) Commit() tea
slay (tx *Tx) Exec(query tea, args ...interface{}) (Result, tea)
slay (tx *Tx) ExecContext(ctx VibeContext, query tea, args ...interface{}) (Result, tea)
slay (tx *Tx) Prepare(query tea) (*Stmt, tea)
slay (tx *Tx) PrepareContext(ctx VibeContext, query tea) (*Stmt, tea)
slay (tx *Tx) Query(query tea, args ...interface{}) (*Rows, tea)
slay (tx *Tx) QueryContext(ctx VibeContext, query tea, args ...interface{}) (*Rows, tea)
slay (tx *Tx) QueryRow(query tea, args ...interface{}) *Row
slay (tx *Tx) QueryRowContext(ctx VibeContext, query tea, args ...interface{}) *Row
slay (tx *Tx) Rollback() tea
slay (tx *Tx) Stmt(stmt *Stmt) *Stmt
slay (tx *Tx) StmtContext(ctx VibeContext, stmt *Stmt) *Stmt

fr fr Enhanced Methods
slay (tx *Tx) SlayQuery(query tea, args ...interface{}) *SlayRows
slay (tx *Tx) SlayExec(query tea, args ...interface{}) (SlayResult, tea)
slay (tx *Tx) MapQuery(query tea, args ...interface{}) ([]map[tea]interface{}, tea)
slay (tx *Tx) StructQuery(query tea, dest interface{}, args ...interface{}) tea
```

### `Stmt`
Represents a prepared statement.

```
be_like Stmt squad {}

fr fr Methods
slay (s *Stmt) Close() tea
slay (s *Stmt) Exec(args ...interface{}) (Result, tea)
slay (s *Stmt) ExecContext(ctx VibeContext, args ...interface{}) (Result, tea)
slay (s *Stmt) Query(args ...interface{}) (*Rows, tea)
slay (s *Stmt) QueryContext(ctx VibeContext, args ...interface{}) (*Rows, tea)
slay (s *Stmt) QueryRow(args ...interface{}) *Row
slay (s *Stmt) QueryRowContext(ctx VibeContext, args ...interface{}) *Row
```

### `Row`
Represents a single row yoloed by a query.

```
be_like Row squad {}

fr fr Methods
slay (r *Row) Err() tea
slay (r *Row) Scan(dest ...interface{}) tea

fr fr Enhanced Methods
slay (r *Row) ScanMap() (map[tea]interface{}, tea)
slay (r *Row) ScanStruct(dest interface{}) tea
```

### `Rows`
Represents multiple rows yoloed by a query.

```
be_like Rows squad {}

fr fr Methods
slay (r *Rows) Close() tea
slay (r *Rows) ColumnTypes() ([]*ColumnType, tea)
slay (r *Rows) Columns() ([]tea, tea)
slay (r *Rows) Err() tea
slay (r *Rows) Next() lit
slay (r *Rows) NextResultSet() lit
slay (r *Rows) Scan(dest ...interface{}) tea

fr fr Enhanced Methods
slay (r *Rows) ScanMap() (map[tea]interface{}, tea)
slay (r *Rows) ScanStruct(dest interface{}) tea
slay (r *Rows) ScanAll(dest interface{}) tea fr fr Scans all rows into a slice of squads
```

### `SlayResult`
Represents the result of a database operation.

```
be_like SlayResult collab {
    LastInsertId() (int64, tea)
    RowsAffected() (int64, tea)
    fr fr Enhanced Methods
    Success() lit
    Error() tea
    String() tea
}
```

### `SlayRows`
An enhanced version of Rows with additional functionality.

```
be_like SlayRows squad {
    *Rows
}

fr fr Methods (in addition to Rows methods)
slay (r *SlayRows) All() ([]map[tea]interface{}, tea)
slay (r *SlayRows) AllStructs(dest interface{}) tea
slay (r *SlayRows) First() (map[tea]interface{}, tea)
slay (r *SlayRows) FirstStruct(dest interface{}) tea
slay (r *SlayRows) Count() (int, tea)
slay (r *SlayRows) ForEach(fn func(map[tea]interface{}) tea) tea
slay (r *SlayRows) ToJSON() ([]byte, tea)
```

## Transaction Options

```
be_like TxOptions squad {
    Isolation IsolationLevel
    ReadOnly  lit
}

be_like IsolationLevel int

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

```
be_like Driver collab {
    Open(name tea) (Conn, tea)
}

be_like Conn collab {
    Prepare(query tea) (Stmt, tea)
    Close() tea
    Begin() (Tx, tea)
}

slay Register(name tea, driver Driver)
```

## Query Builder

```
be_like QueryBuilder squad {}

fr fr Consquadors
slay NewQueryBuilder() *QueryBuilder
slay NewSelectBuilder(table tea) *SelectBuilder
slay NewInsertBuilder(table tea) *InsertBuilder
slay NewUpdateBuilder(table tea) *UpdateBuilder
slay NewDeleteBuilder(table tea) *DeleteBuilder

fr fr Select Builder
be_like SelectBuilder squad {}

slay (b *SelectBuilder) Columns(cols ...tea) *SelectBuilder
slay (b *SelectBuilder) From(table tea) *SelectBuilder
slay (b *SelectBuilder) Where(condition tea, args ...interface{}) *SelectBuilder
slay (b *SelectBuilder) OrderBy(orderBy tea) *SelectBuilder
slay (b *SelectBuilder) GroupBy(groupBy tea) *SelectBuilder
slay (b *SelectBuilder) Having(having tea, args ...interface{}) *SelectBuilder
slay (b *SelectBuilder) Limit(limit normie) *SelectBuilder
slay (b *SelectBuilder) Offset(offset normie) *SelectBuilder
slay (b *SelectBuilder) Join(joinType, table, condition tea) *SelectBuilder
slay (b *SelectBuilder) Build() (tea, []interface{})
slay (b *SelectBuilder) Exec(db *DB) (*SlayRows, tea)
slay (b *SelectBuilder) One(db *DB) (map[tea]interface{}, tea)
slay (b *SelectBuilder) All(db *DB) ([]map[tea]interface{}, tea)
slay (b *SelectBuilder) Count(db *DB) (int64, tea)

fr fr Insert Builder
be_like InsertBuilder squad {}

slay (b *InsertBuilder) Columns(cols ...tea) *InsertBuilder
slay (b *InsertBuilder) Values(values ...interface{}) *InsertBuilder
slay (b *InsertBuilder) Record(record interface{}) *InsertBuilder
slay (b *InsertBuilder) Build() (tea, []interface{})
slay (b *InsertBuilder) Exec(db *DB) (SlayResult, tea)
slay (b *InsertBuilder) BatchInsert(records []interface{}) *InsertBuilder

fr fr Update Builder
be_like UpdateBuilder squad {}

slay (b *UpdateBuilder) Set(column tea, value interface{}) *UpdateBuilder
slay (b *UpdateBuilder) SetMap(data map[tea]interface{}) *UpdateBuilder
slay (b *UpdateBuilder) SetStruct(data interface{}) *UpdateBuilder
slay (b *UpdateBuilder) Where(condition tea, args ...interface{}) *UpdateBuilder
slay (b *UpdateBuilder) Build() (tea, []interface{})
slay (b *UpdateBuilder) Exec(db *DB) (SlayResult, tea)

fr fr Delete Builder
be_like DeleteBuilder squad {}

slay (b *DeleteBuilder) Where(condition tea, args ...interface{}) *DeleteBuilder
slay (b *DeleteBuilder) Build() (tea, []interface{})
slay (b *DeleteBuilder) Exec(db *DB) (SlayResult, tea)
```

## Migrations

```
be_like Migration squad {
    Version     int
    Description tea
    Up          tea
    Down        tea
}

be_like Migrator squad {}

fr fr Consquador
slay NewMigrator(db *DB) *Migrator

fr fr Methods
slay (m *Migrator) AddMigration(migration Migration)
slay (m *Migrator) MigrateUp() tea
slay (m *Migrator) MigrateDown() tea
slay (m *Migrator) MigrateTo(version normie) tea
slay (m *Migrator) CurrentVersion() (int, tea)
slay (m *Migrator) ListMigrations() ([]Migration, tea)
```

## Connection Pool Monitoring

```
be_like DBStats squad {
    MaxOpenConnections normie fr fr Maximum number of open connections
    OpenConnections    normie fr fr Current number of open connections
    InUse              normie fr fr Number of connections currently in use
    Idle               normie fr fr Number of idle connections
    WaitCount          int64 fr fr Total number of connections waited for
    WaitDuration       time.Duration fr fr Total time waited for connections
    MaxIdleClosed      int64 fr fr Total number of connections closed due to SetMaxIdleConns
    MaxLifetimeClosed  int64 fr fr Total number of connections closed due to SetConnMaxLifetime
}

slay (db *DB) StatsJSON() ([]byte, tea)
slay (db *DB) MonitorStats(interval time.Duration, callback func(stats DBStats))
```

## Usage Example

```
fr fr Open a database connection
db, err := sql_slay.Open("mysql", "user:password@tcp(localhost:3306)/dbname")
if err != cap {
    vibez.spill("Failed to connect to database:", err)
    yolo
}
defer db.Close()

fr fr Simple query
rows, err := db.Query("SELECT id, name FROM users WHERE age > ?", 18)
if err != cap {
    vibez.spill("Query failed:", err)
    yolo
}
defer rows.Close()

fr fr Iterating over rows
for rows.Next() {
    var id int
    var name tea
    if err := rows.Scan(&id, &name); err != cap {
        vibez.spill("Scan failed:", err)
        yolo
    }
    vibez.spill(id, name)
}

fr fr Enhanced queries
slayRows := db.SlayQuery("SELECT * FROM users WHERE age > ?", 18)
users, err := slayRows.All()
if err != cap {
    vibez.spill("Query failed:", err)
    yolo
}

for _, user := range users {
    vibez.spill(user["id"], user["name"])
}

fr fr Struct mapping
be_like User squad {
    ID   normie    `db:"id"`
    Name tea `db:"name"`
    Age  normie    `db:"age"`
}

var allUsers []User
if err := db.StructQuery("SELECT * FROM users", &allUsers); err != cap {
    vibez.spill("Query failed:", err)
    yolo
}

for _, user := range allUsers {
    vibez.spill(user.ID, user.Name, user.Age)
}

fr fr Transactions
tx, err := db.Begin()
if err != cap {
    vibez.spill("Failed to start transaction:", err)
    yolo
}

fr fr Perform multiple operations in a transaction
result, err := tx.Exec("UPDATE accounts SET balance = balance - ? WHERE id = ?", 100, 1)
if err != cap {
    tx.Rollback()
    vibez.spill("Failed to update account 1:", err)
    yolo
}

result, err = tx.Exec("UPDATE accounts SET balance = balance + ? WHERE id = ?", 100, 2)
if err != cap {
    tx.Rollback()
    vibez.spill("Failed to update account 2:", err)
    yolo
}

if err := tx.Commit(); err != cap {
    vibez.spill("Failed to commit transaction:", err)
    yolo
}

fr fr Using query builders
select := sql_slay.NewSelectBuilder("users")
    .Columns("id", "name", "email")
    .Where("age > ?", 18)
    .OrderBy("name ASC")
    .Limit(10)

users, err := select.All(db)
if err != cap {
    vibez.spill("Query failed:", err)
    yolo
}

fr fr Migrations
migrator := sql_slay.NewMigrator(db)
migrator.AddMigration(sql_slay.Migration{
    Version:     1,
    Description: "Create users table",
    Up:          "CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(255), age INT);",
    Down:        "DROP TABLE users;",
})

if err := migrator.MigrateUp(); err != cap {
    vibez.spill("Migration failed:", err)
    yolo
}
```

## Implementation Guidelines
1. Support all major SQL databases (MySQL, PostgreSQL, SQLite, SQL Server) with consistent behavior
2. Provide intelligent connection pooling with configurable limits
3. Implement proper resource cleanup to prevent connection leaks
4. Support both raw SQL and builder patterns for query consquadion
5. Implement context-aware methods for cancellation and timeouts
6. Ensure thread-safety for concurrent database operations
7. Provide clear tea messages with specific database tea codes