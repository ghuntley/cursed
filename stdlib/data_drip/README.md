# Data Drip Module

The `data_drip` module provides a comprehensive database interface for SQL operations in the CURSED language. It offers connection pooling, transaction management, and SQL injection protection.

## Features

### Database Connection Management
- **OpenDB**: Establish database connections
- **Close**: Close database connections
- **Ping**: Test database connectivity
- **SetMaxOpenConns**: Configure connection pool size
- **SetMaxIdleConns**: Configure idle connection limits
- **SetConnMaxLifetime**: Set connection lifetime limits

### Query Execution
- **Query**: Execute SELECT statements that return rows
- **Exec**: Execute INSERT, UPDATE, DELETE statements
- **QueryRow**: Execute queries that return a single row
- **Prepare**: Create prepared statements for efficient execution

### Transaction Management
- **Begin**: Start database transactions
- **Commit**: Commit transactions
- **Rollback**: Rollback transactions

### Result Processing
- **NextRow**: Iterate through query results
- **ScanRow**: Extract data from result rows
- **CloseRows**: Close result sets
- **GetColumns**: Get column information

### Enhanced Features
- **Query Builder**: Fluent interface for building SQL queries
- **Connection Pool Stats**: Monitor connection pool performance
- **SQL Injection Protection**: Automatic parameter sanitization
- **Async Operations**: Non-blocking database operations

## Usage Examples

```cursed
fr fr Database connection
sus db tea = OpenDB("postgres", "user=admin dbname=myapp")
sus pingResult tea = Ping(db)

fr fr Simple query
sus rows tea = Query(db, "SELECT id, name FROM users WHERE age > ?", "18")

fr fr Transaction example
sus tx tea = Begin(db)
sus result tea = Exec(tx, "INSERT INTO users (name, email) VALUES (?, ?)", "Alice,alice@example.com")
sus commitResult tea = Commit(tx)

fr fr Query builder
sus builder tea = NewQueryBuilder()
sus query tea = SelectFrom(builder, "users")
query = WhereClause(query, "active", "true")
query = OrderBy(query, "created_at", "DESC")
query = LimitRows(query, 10)

fr fr Connection pool monitoring
sus stats tea = PoolStats(db)
sus isActive lit = IsConnected(db)

fr fr Cleanup
sus closeResult tea = Close(db)
```

## Database Drivers

The module supports multiple database drivers:
- **PostgreSQL**: Use "postgres" driver name
- **MySQL**: Use "mysql" driver name  
- **SQLite**: Use "sqlite3" driver name
- **SQL Server**: Use "sqlserver" driver name

## Connection Strings

Format connection strings according to your database:
- PostgreSQL: `"user=username password=secret dbname=mydb sslmode=disable"`
- MySQL: `"username:password@tcp(localhost:3306)/dbname"`
- SQLite: `"./database.db"`

## Security Features

- Automatic parameter sanitization to prevent SQL injection
- Connection string validation
- Query validation and analysis
- Secure credential handling

## Implementation Notes

This is a pure CURSED implementation providing:
- Comprehensive database abstraction layer
- Production-ready connection pooling
- Enterprise-grade transaction support
- Cross-database compatibility
- Security-first design approach

The module serves as a foundation for robust database-driven applications while maintaining the CURSED language's unique syntax and philosophy.
