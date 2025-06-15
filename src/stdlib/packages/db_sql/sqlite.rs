/// fr fr SQLite driver implementation - the lightweight champion periodt

use crate::stdlib::packages::{
    db_core::{
        ConnectionConfig, DatabaseConnection, DriverFeature, SqlDialect,
        Parameter, ResultSet, PreparedStatement, DatabaseTransaction,
        ExecuteResult, TransactionIsolation
    },
    db_sql::{SqlDriver, SqlDialectTrait, SqlFeature, SqlValue, SqlResultSet, SqlExecuteResult}
};
use crate::stdlib::packages::db_sql::drivers::{
    SqlConnection, ConfigurationOption, DriverPerformanceInfo, DriverLimitations,
    SqlTransactionIsolation, SqlConnectionInfo, SqlBatch, SqlTransaction
};
use crate::stdlib::packages::db_core::error::{
    DatabaseResult as DbResult, DatabaseError, ErrorKind, ConnectionError, QueryError, TransactionError
};
use async_trait::async_trait;
use rusqlite::{Connection, Statement, Row, ToSql, types::Value as SqliteValue, Transaction as SqliteTransaction};
use std::sync::{Arc, Mutex};
use std::path::Path;

/// fr fr SQLite driver
#[derive(Debug)]
pub struct SqliteDriver {
    name: String,
    version: String,
}

/// fr fr SQLite connection
pub struct SqliteConnection {
    connection_id: String,
    connection: Arc<Mutex<Connection>>,
    database_path: String,
    in_transaction: bool,
}

/// fr fr SQLite error
#[derive(Debug)]
pub struct SqliteError {
    message: String,
}

/// fr fr SQLite result set implementation
pub struct SqliteResultSet {
    rows: Vec<crate::stdlib::packages::db_core::Row>,
    metadata: crate::stdlib::packages::db_core::ResultMetadata,
    current_index: usize,
}

/// fr fr SQLite prepared statement implementation
pub struct SqlitePreparedStatement {
    connection: Arc<Mutex<Connection>>,
    sql: String,
    statement_id: String,
}

/// fr fr SQLite transaction implementation
pub struct SqliteTransactionImpl {
    connection: Arc<Mutex<Connection>>,
    transaction_id: String,
    active: bool,
}

impl SqliteDriver {
    pub fn new() -> Self {
        Self {
            name: "sqlite".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

impl SqliteConnection {
    /// slay Create a new SQLite connection
    pub fn new(database_path: &str) -> DbResult<Self> {
        let conn = Connection::open(database_path)
            .map_err(|e| DatabaseError::connection(
                ConnectionError::FailedToConnect,
                &format!("Failed to open SQLite database: {}", e)
            ))?;

        // Enable foreign key constraints by default
        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| DatabaseError::driver(&format!("Failed to enable foreign keys: {}", e)))?;

        Ok(Self {
            connection_id: format!("sqlite_{}", uuid::Uuid::new_v4()),
            connection: Arc::new(Mutex::new(conn)),
            database_path: database_path.to_string(),
            in_transaction: false,
        })
    }

    /// slay Create an in-memory SQLite connection
    pub fn new_in_memory() -> DbResult<Self> {
        Self::new(":memory:")
    }

    /// slay Convert SqlValue to rusqlite ToSql
    fn sql_value_to_sqlite(value: &SqlValue) -> Box<dyn ToSql> {
        match value {
            SqlValue::Null => Box::new(rusqlite::types::Null),
            SqlValue::Boolean(b) => Box::new(*b),
            SqlValue::Integer(i) => Box::new(*i),
            SqlValue::Float(f) | SqlValue::Double(f) => Box::new(*f),
            SqlValue::Text(s) => Box::new(s.clone()),
            SqlValue::Binary(data) => Box::new(data.clone()),
            SqlValue::Date(d) => Box::new(d.format("%Y-%m-%d").to_string()),
            SqlValue::Time(t) => Box::new(t.format("%H:%M:%S").to_string()),
            SqlValue::Timestamp(dt) => Box::new(dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            SqlValue::TimestampTz(dt) => Box::new(dt.format("%Y-%m-%d %H:%M:%S%z").to_string()),
            SqlValue::Json(j) => Box::new(j.to_string()),
            SqlValue::Uuid(u) => Box::new(u.to_string()),
            _ => Box::new(value.to_sql()),
        }
    }

    /// slay Convert rusqlite Value to SqlValue
    fn sqlite_value_to_sql(value: SqliteValue) -> SqlValue {
        match value {
            SqliteValue::Null => SqlValue::Null,
            SqliteValue::Integer(i) => SqlValue::Integer(i),
            SqliteValue::Real(f) => SqlValue::Double(f),
            SqliteValue::Text(s) => SqlValue::Text(s),
            SqliteValue::Blob(data) => SqlValue::Binary(data),
        }
    }
}

#[async_trait]
impl crate::stdlib::packages::db_core::DatabaseDriver for SqliteDriver {
    async fn connect(&self, config: ConnectionConfig) -> DbResult<Box<dyn DatabaseConnection>> {
        let database_path = config.connection_string
            .strip_prefix("sqlite://")
            .unwrap_or(&config.connection_string);
        
        let conn = SqliteConnection::new(database_path)?;
        Ok(Box::new(conn))
    }

    fn driver_info(&self) -> crate::stdlib::packages::db_core::DriverInfo {
        crate::stdlib::packages::db_core::DriverInfo::new(
            &self.name,
            &self.version,
            "SQLite database driver",
            "CURSED"
        )
    }

    fn supports_feature(&self, _feature: DriverFeature) -> bool {
        true
    }

    fn sql_dialect(&self) -> SqlDialect {
        SqlDialect::SQLite
    }

    fn validate_connection_string(&self, _connection_string: &str) -> DbResult<()> {
        Ok(())
    }
}

#[async_trait]
impl SqlDriver for SqliteDriver {
    async fn sql_connect(&self, config: ConnectionConfig) -> DbResult<Box<dyn SqlConnection>> {
        let database_path = config.connection_string
            .strip_prefix("sqlite://")
            .unwrap_or(&config.connection_string);
        
        let conn = SqliteConnection::new(database_path)?;
        Ok(Box::new(conn))
    }

    fn sql_dialect(&self) -> Box<dyn SqlDialectTrait> {
        Box::new(crate::stdlib::packages::db_sql::SqliteDialect::new())
    }

    fn supported_types(&self) -> Vec<crate::stdlib::packages::db_sql::SqlType> {
        vec![
            crate::stdlib::packages::db_sql::SqlType::Integer,
            crate::stdlib::packages::db_sql::SqlType::Text,
            crate::stdlib::packages::db_sql::SqlType::Boolean,
            crate::stdlib::packages::db_sql::SqlType::Blob,
        ]
    }

    fn supports_sql_feature(&self, feature: SqlFeature) -> bool {
        // SQLite has some limitations
        match feature {
            SqlFeature::ForeignKeys => true,
            SqlFeature::Triggers => true,
            SqlFeature::Views => true,
            SqlFeature::Indexes => true,
            SqlFeature::WindowFunctions => true,
            SqlFeature::CommonTableExpressions => true,
            SqlFeature::JsonSupport => true,
            _ => false, // Many advanced features not supported
        }
    }

    fn configuration_options(&self) -> Vec<ConfigurationOption> {
        Vec::from([])
    }

    fn validate_sql(&self, _sql: &str) -> DbResult<()> {
        Ok(())
    }

    fn performance_info(&self) -> DriverPerformanceInfo {
        DriverPerformanceInfo {
            connection_time: std::time::Duration::from_millis(10),
            query_overhead: std::time::Duration::from_micros(10),
            max_connections: Some(1), // SQLite is single-threaded
            connection_pooling: false,
            statement_caching: true,
            batch_operations: true,
            streaming_results: false,
        }
    }

    fn limitations(&self) -> DriverLimitations {
        DriverLimitations {
            max_statement_length: Some(1000000),
            max_parameters: Some(999),
            max_identifier_length: Some(255),
            max_string_length: Some(1000000000),
            max_numeric_precision: Some(15),
            max_columns: Some(2000),
            max_rows: None,
            unsupported_features: vec![
                SqlFeature::StoredProcedures,
                SqlFeature::UserDefinedFunctions,
                SqlFeature::MaterializedViews,
                SqlFeature::Partitioning,
            ],
        }
    }
}

#[async_trait]
impl DatabaseConnection for SqliteConnection {
    async fn query(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
                QueryError::SyntaxError,
                &format!("Failed to prepare query: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
                crate::stdlib::packages::db_core::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
                }
                _ => Box::leak(Box::new(rusqlite::types::Null)) as &dyn ToSql,
            })
            .collect();

        let rows = stmt.query_map(&sqlite_params[..], |row| {
            let mut values = Vec::new();
            for i in 0..row.as_ref().column_count() {
                let value = row.get::<usize, SqliteValue>(i)?;
                let column_value = crate::stdlib::packages::db_core::ColumnValue {
                    data: match &value {
                        SqliteValue::Null => None,
                        SqliteValue::Integer(i) => Some(i.to_le_bytes().to_vec()),
                        SqliteValue::Real(f) => Some(f.to_le_bytes().to_vec()),
                        SqliteValue::Text(s) => Some(s.as_bytes().to_vec()),
                        SqliteValue::Blob(data) => Some(data.clone()),
                    },
                    column_type: match value {
                        SqliteValue::Null => crate::stdlib::packages::db_core::ColumnType::Null,
                        SqliteValue::Integer(_) => crate::stdlib::packages::db_core::ColumnType::BigInt,
                        SqliteValue::Real(_) => crate::stdlib::packages::db_core::ColumnType::Double,
                        SqliteValue::Text(_) => crate::stdlib::packages::db_core::ColumnType::Text,
                        SqliteValue::Blob(_) => crate::stdlib::packages::db_core::ColumnType::Blob,
                    },
                    is_null: matches!(value, SqliteValue::Null),
                };
                values.push(column_value);
            }
            
            Ok(crate::stdlib::packages::db_core::Row {
                values,
                metadata: crate::stdlib::packages::db_core::RowMetadata {
                    row_number: 0,
                    is_inserted: false,
                    is_updated: false,
                    is_deleted: false,
                },
            })
        })
        .map_err(|e| DatabaseError::query(
            QueryError::ExecutionFailed,
            &format!("Query execution failed: {}", e)
        ))?;

        let mut result_rows = Vec::new();
        for row_result in rows {
            let row = row_result.map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("Row processing failed: {}", e)
            ))?;
            result_rows.push(row);
        }

        let columns = if let Some(first_row) = result_rows.first() {
            first_row.values.iter().enumerate().map(|(i, val)| {
                crate::stdlib::packages::db_core::Column {
                    name: format!("column_{}", i),
                    column_type: val.column_type.clone(),
                    nullable: true,
                    ordinal: i,
                    table_name: None,
                    schema_name: None,
                    precision: None,
                    scale: None,
                    max_length: None,
                    auto_increment: false,
                    default_value: None,
                }
            }).collect()
        } else {
            Vec::new()
        };

        let result_set = SqliteResultSet {
            rows: result_rows,
            metadata: crate::stdlib::packages::db_core::ResultMetadata {
                columns,
                row_count: None,
                affected_rows: 0,
                last_insert_id: None,
                warnings: Vec::new(),
                query_id: Some(uuid::Uuid::new_v4().to_string()),
                execution_time: std::time::Duration::from_millis(0),
            },
            current_index: 0,
        };

        Ok(Box::new(result_set))
    }

    async fn execute(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<ExecuteResult> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
                QueryError::SyntaxError,
                &format!("Failed to prepare statement: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
                crate::stdlib::packages::db_core::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
                },
                _ => Box::leak(Box::new(rusqlite::types::Null)) as &dyn ToSql,
            })
            .collect();

        let affected_rows = stmt.execute(&sqlite_params[..])
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("Statement execution failed: {}", e)
            ))?;

        Ok(ExecuteResult {
            affected_rows: affected_rows as u64,
            last_insert_id: Some(conn.last_insert_rowid() as u64),
            warnings: Vec::new(),
            query_id: Some(uuid::Uuid::new_v4().to_string()),
            execution_time: std::time::Duration::from_millis(0),
        })
    }

    async fn prepare(&mut self, sql: &str) -> DbResult<Box<dyn PreparedStatement>> {
        let stmt = SqlitePreparedStatement {
            connection: Arc::clone(&self.connection),
            sql: sql.to_string(),
            statement_id: uuid::Uuid::new_v4().to_string(),
        };

        Ok(Box::new(stmt))
    }

    async fn begin_transaction(&mut self, _options: Option<crate::stdlib::packages::db_core::TransactionOptions>) -> DbResult<Box<dyn DatabaseTransaction>> {
        if self.in_transaction {
            return Err(DatabaseError::transaction(
                TransactionError::NotActive,
                "Transaction already active"
            ));
        }

        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute("BEGIN TRANSACTION", [])
            .map_err(|e| DatabaseError::transaction(
                TransactionError::NotActive,
                &format!("Failed to begin transaction: {}", e)
            ))?;

        self.in_transaction = true;

        let transaction = SqliteTransactionImpl {
            connection: Arc::clone(&self.connection),
            transaction_id: uuid::Uuid::new_v4().to_string(),
            active: true,
        };

        Ok(Box::new(transaction))
    }

    async fn ping(&mut self) -> DbResult<()> {
        // SQLite is always "connected" as it's a file-based database
        // We can test with a simple query
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute("SELECT 1", [])
            .map_err(|e| DatabaseError::connection(
                ConnectionError::ConnectionLost,
                &format!("Ping failed: {}", e)
            ))?;

        Ok(())
    }

    async fn close(self: Box<Self>) -> DbResult<()> {
        // SQLite connections close automatically when dropped
        Ok(())
    }

    fn connection_info(&self) -> crate::stdlib::packages::db_core::traits::ConnectionInfo {
        crate::stdlib::packages::db_core::traits::ConnectionInfo {
            database_name: "sqlite_db".to_string(),
            server_version: "3.42.0".to_string(),
            protocol_version: "1.0".to_string(),
            connection_id: self.connection_id.clone(),
            is_read_only: false,
            transaction_isolation: crate::stdlib::packages::db_core::traits::TransactionIsolation::Serializable,
        }
    }
}

#[async_trait]
impl SqlConnection for SqliteConnection {
    async fn sql_query(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlResultSet> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
                QueryError::SyntaxError,
                &format!("Failed to prepare query: {}", e)
            ))?;

        // Convert SqlValue parameters to rusqlite format
        let sqlite_params: Vec<Box<dyn ToSql>> = params.iter()
            .map(|p| Self::sql_value_to_sqlite(p))
            .collect();
        
        let param_refs: Vec<&dyn ToSql> = sqlite_params.iter()
            .map(|p| p.as_ref())
            .collect();

        let rows = stmt.query_map(&param_refs[..], |row| {
            let mut values = Vec::new();
            for i in 0..row.as_ref().column_count() {
                let value = row.get::<usize, SqliteValue>(i)?;
                values.push(Self::sqlite_value_to_sql(value));
            }
            Ok(values)
        })
        .map_err(|e| DatabaseError::query(
            QueryError::ExecutionFailed,
            &format!("Query execution failed: {}", e)
        ))?;

        let mut result_rows = Vec::new();
        for row_result in rows {
            let row_values = row_result.map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("Row processing failed: {}", e)
            ))?;
            result_rows.push(row_values);
        }

        // Get column names from the statement
        let column_names: Vec<String> = stmt.column_names().iter()
            .map(|name| name.to_string())
            .collect();

        let result_set = SqlResultSet {
            rows: result_rows,
            columns: column_names,
            affected_rows: 0,
            row_count: Some(result_rows.len()),
        };

        Ok(result_set)
    }

    async fn sql_execute(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlExecuteResult> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
                QueryError::SyntaxError,
                &format!("Failed to prepare statement: {}", e)
            ))?;

        // Convert SqlValue parameters to rusqlite format
        let sqlite_params: Vec<Box<dyn ToSql>> = params.iter()
            .map(|p| Self::sql_value_to_sqlite(p))
            .collect();
        
        let param_refs: Vec<&dyn ToSql> = sqlite_params.iter()
            .map(|p| p.as_ref())
            .collect();

        let affected_rows = stmt.execute(&param_refs[..])
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("Statement execution failed: {}", e)
            ))?;

        Ok(SqlExecuteResult {
            affected_rows: affected_rows as u64,
            last_insert_id: Some(conn.last_insert_rowid() as u64),
            warnings: Vec::new(),
        })
    }

    async fn sql_prepare(&mut self, sql: &str) -> DbResult<Box<dyn PreparedStatement>> {
        let stmt = SqlitePreparedStatement {
            connection: Arc::clone(&self.connection),
            sql: sql.to_string(),
            statement_id: uuid::Uuid::new_v4().to_string(),
        };

        Ok(Box::new(stmt))
    }

    async fn sql_begin_transaction(&mut self, isolation: Option<SqlTransactionIsolation>) -> DbResult<Box<dyn SqlTransaction>> {
        if self.in_transaction {
            return Err(DatabaseError::transaction(
                TransactionError::NotActive,
                "Transaction already active"
            ));
        }

        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        // Set isolation level if specified (SQLite uses PRAGMA for this)
        if let Some(isolation_level) = isolation {
            let pragma_sql = match isolation_level {
                SqlTransactionIsolation::ReadUncommitted => "PRAGMA read_uncommitted = 1",
                SqlTransactionIsolation::ReadCommitted => "PRAGMA read_uncommitted = 0",
                SqlTransactionIsolation::RepeatableRead => "PRAGMA read_uncommitted = 0",
                SqlTransactionIsolation::Serializable => "PRAGMA read_uncommitted = 0",
            };
            
            conn.execute(pragma_sql, [])
                .map_err(|e| DatabaseError::transaction(
                    TransactionError::NotActive,
                    &format!("Failed to set isolation level: {}", e)
                ))?;
        }

        conn.execute("BEGIN TRANSACTION", [])
            .map_err(|e| DatabaseError::transaction(
                TransactionError::NotActive,
                &format!("Failed to begin transaction: {}", e)
            ))?;

        self.in_transaction = true;

        let transaction = SqliteTransactionImpl {
            connection: Arc::clone(&self.connection),
            transaction_id: uuid::Uuid::new_v4().to_string(),
            active: true,
        };

        Ok(Box::new(transaction))
    }

    async fn sql_batch(&mut self, statements: &[SqlBatch]) -> DbResult<Vec<SqlExecuteResult>> {
        let mut results = Vec::new();
        
        for batch_stmt in statements {
            let result = self.sql_execute(&batch_stmt.sql, &batch_stmt.parameters).await?;
            results.push(result);
        }
        
        Ok(results)
    }

    fn sql_connection_info(&self) -> SqlConnectionInfo {
        SqlConnectionInfo {
            server_version: "3.42.0".to_string(),
            protocol_version: "1.0".to_string(),
            database_name: "main".to_string(),
            schema_name: Some("main".to_string()),
            character_set: "UTF-8".to_string(),
            collation: "BINARY".to_string(),
            time_zone: "UTC".to_string(),
            auto_commit: true,
            read_only: false,
            isolation_level: SqlTransactionIsolation::Serializable,
            capabilities: vec!["transactions".to_string(), "views".to_string(), "triggers".to_string()],
        }
    }

    async fn set_sql_variable(&mut self, name: &str, value: &SqlValue) -> DbResult<()> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        // SQLite uses PRAGMA statements instead of variables
        // Convert SqlValue to string for PRAGMA usage
        let value_str = match value {
            SqlValue::Null => "NULL".to_string(),
            SqlValue::Boolean(b) => if *b { "1".to_string() } else { "0".to_string() },
            SqlValue::Integer(i) => i.to_string(),
            SqlValue::Float(f) | SqlValue::Double(f) => f.to_string(),
            SqlValue::Text(s) => format!("'{}'", s.replace("'", "''")), // Escape single quotes
            SqlValue::Binary(_) => return Err(DatabaseError::driver("Binary values not supported for PRAGMA")),
            _ => value.to_string(),
        };

        let pragma_sql = format!("PRAGMA {} = {}", name, value_str);
        
        conn.execute(&pragma_sql, [])
            .map_err(|e| DatabaseError::driver(&format!("Failed to set PRAGMA {}: {}", name, e)))?;

        Ok(())
    }

    async fn get_sql_variable(&mut self, name: &str) -> DbResult<SqlValue> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        // SQLite uses PRAGMA statements instead of variables
        let pragma_sql = format!("PRAGMA {}", name);
        
        let mut stmt = conn.prepare(&pragma_sql)
            .map_err(|e| DatabaseError::driver(&format!("Failed to prepare PRAGMA {}: {}", name, e)))?;

        let rows = stmt.query_map([], |row| {
            // PRAGMA results typically have a single column
            let value = row.get::<usize, SqliteValue>(0)?;
            Ok(Self::sqlite_value_to_sql(value))
        })
        .map_err(|e| DatabaseError::driver(&format!("Failed to execute PRAGMA {}: {}", name, e)))?;

        // Get the first row result
        for row_result in rows {
            let value = row_result.map_err(|e| DatabaseError::driver(&format!("Failed to read PRAGMA {} result: {}", name, e)))?;
            return Ok(value);
        }

        // If no rows returned, return NULL
        Ok(SqlValue::Null)
    }
}

// Implement ResultSet trait for SqliteResultSet
#[async_trait]
impl ResultSet for SqliteResultSet {
    async fn next(&mut self) -> DbResult<Option<crate::stdlib::packages::db_core::Row>> {
        if self.current_index < self.rows.len() {
            let row = self.rows[self.current_index].clone();
            self.current_index += 1;
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

    async fn seek(&mut self, position: usize) -> DbResult<()> {
        if position <= self.rows.len() {
            self.current_index = position;
            Ok(())
        } else {
            Err(DatabaseError::query(
                QueryError::ResultSetExhausted,
                "Position beyond result set bounds"
            ))
        }
    }

    fn metadata(&self) -> &crate::stdlib::packages::db_core::ResultMetadata {
        &self.metadata
    }

    fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    fn row_count(&self) -> Option<usize> {
        Some(self.rows.len())
    }
}

// Implement PreparedStatement trait for SqlitePreparedStatement
#[async_trait]
impl PreparedStatement for SqlitePreparedStatement {
    async fn execute(&mut self, parameters: &[Parameter]) -> DbResult<crate::stdlib::packages::db_core::ExecuteResult> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(&self.sql)
            .map_err(|e| DatabaseError::query(
                QueryError::SyntaxError,
                &format!("Failed to prepare statement: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
                crate::stdlib::packages::db_core::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
                },
                _ => Box::leak(Box::new(rusqlite::types::Null)) as &dyn ToSql,
            })
            .collect();

        let affected_rows = stmt.execute(&sqlite_params[..])
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("Statement execution failed: {}", e)
            ))?;

        Ok(crate::stdlib::packages::db_core::ExecuteResult {
            affected_rows: affected_rows as u64,
            last_insert_id: Some(conn.last_insert_rowid() as u64),
            warnings: Vec::new(),
            query_id: Some(self.statement_id.clone()),
            execution_time: std::time::Duration::from_millis(0),
        })
    }

    async fn query(&mut self, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(&self.sql)
            .map_err(|e| DatabaseError::query(
                QueryError::SyntaxError,
                &format!("Failed to prepare query: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
                crate::stdlib::packages::db_core::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
                },
                _ => Box::leak(Box::new(rusqlite::types::Null)) as &dyn ToSql,
            })
            .collect();

        let rows = stmt.query_map(&sqlite_params[..], |row| {
            let mut values = Vec::new();
            for i in 0..row.as_ref().column_count() {
                let value = row.get::<usize, SqliteValue>(i)?;
                let column_value = crate::stdlib::packages::db_core::ColumnValue {
                    data: match &value {
                        SqliteValue::Null => None,
                        SqliteValue::Integer(i) => Some(i.to_le_bytes().to_vec()),
                        SqliteValue::Real(f) => Some(f.to_le_bytes().to_vec()),
                        SqliteValue::Text(s) => Some(s.as_bytes().to_vec()),
                        SqliteValue::Blob(data) => Some(data.clone()),
                    },
                    column_type: match value {
                        SqliteValue::Null => crate::stdlib::packages::db_core::ColumnType::Null,
                        SqliteValue::Integer(_) => crate::stdlib::packages::db_core::ColumnType::BigInt,
                        SqliteValue::Real(_) => crate::stdlib::packages::db_core::ColumnType::Double,
                        SqliteValue::Text(_) => crate::stdlib::packages::db_core::ColumnType::Text,
                        SqliteValue::Blob(_) => crate::stdlib::packages::db_core::ColumnType::Blob,
                    },
                    is_null: matches!(value, SqliteValue::Null),
                };
                values.push(column_value);
            }
            
            Ok(crate::stdlib::packages::db_core::Row {
                values,
                metadata: crate::stdlib::packages::db_core::RowMetadata {
                    row_number: 0,
                    is_inserted: false,
                    is_updated: false,
                    is_deleted: false,
                },
            })
        })
        .map_err(|e| DatabaseError::query(
            QueryError::ExecutionFailed,
            &format!("Query execution failed: {}", e)
        ))?;

        let mut result_rows = Vec::new();
        for row_result in rows {
            let row = row_result.map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("Row processing failed: {}", e)
            ))?;
            result_rows.push(row);
        }

        let columns = if let Some(first_row) = result_rows.first() {
            first_row.values.iter().enumerate().map(|(i, val)| {
                crate::stdlib::packages::db_core::Column {
                    name: format!("column_{}", i),
                    column_type: val.column_type.clone(),
                    nullable: true,
                    ordinal: i,
                    table_name: None,
                    schema_name: None,
                    precision: None,
                    scale: None,
                    max_length: None,
                    auto_increment: false,
                    default_value: None,
                }
            }).collect()
        } else {
            Vec::new()
        };

        let result_set = SqliteResultSet {
            rows: result_rows,
            metadata: crate::stdlib::packages::db_core::ResultMetadata {
                columns,
                row_count: None,
                affected_rows: 0,
                last_insert_id: None,
                warnings: Vec::new(),
                query_id: Some(self.statement_id.clone()),
                execution_time: std::time::Duration::from_millis(0),
            },
            current_index: 0,
        };

        Ok(Box::new(result_set))
    }

    fn parameter_count(&self) -> usize {
        // SQLite doesn't expose parameter count directly, we'll estimate
        self.sql.matches('?').count()
    }

    fn sql(&self) -> &str {
        &self.sql
    }
}

// Implement DatabaseTransaction trait for SqliteTransactionImpl
#[async_trait]
impl DatabaseTransaction for SqliteTransactionImpl {
    async fn commit(mut self: Box<Self>) -> DbResult<()> {
        if !self.active {
            return Err(DatabaseError::transaction(
                TransactionError::AlreadyCommitted,
                "Transaction already completed"
            ));
        }

        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute("COMMIT", [])
            .map_err(|e| DatabaseError::transaction(
                TransactionError::ConstraintViolation,
                &format!("Failed to commit transaction: {}", e)
            ))?;

        self.active = false;
        Ok(())
    }

    async fn rollback(mut self: Box<Self>) -> DbResult<()> {
        if !self.active {
            return Err(DatabaseError::transaction(
                TransactionError::AlreadyRolledBack,
                "Transaction already completed"
            ));
        }

        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute("ROLLBACK", [])
            .map_err(|e| DatabaseError::transaction(
                TransactionError::ConstraintViolation,
                &format!("Failed to rollback transaction: {}", e)
            ))?;

        self.active = false;
        Ok(())
    }

    async fn savepoint(&mut self, name: &str) -> DbResult<crate::stdlib::packages::db_core::SavePoint> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute(&format!("SAVEPOINT {}", name), [])
            .map_err(|e| DatabaseError::transaction(
                TransactionError::ConstraintViolation,
                &format!("Failed to create savepoint: {}", e)
            ))?;

        Ok(crate::stdlib::packages::db_core::SavePoint {
            name: name.to_string(),
            transaction_id: self.transaction_id.clone(),
            created_at: std::time::SystemTime::now(),
        })
    }

    async fn rollback_to_savepoint(&mut self, savepoint: &crate::stdlib::packages::db_core::SavePoint) -> DbResult<()> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute(&format!("ROLLBACK TO SAVEPOINT {}", savepoint.name), [])
            .map_err(|e| DatabaseError::transaction(
                TransactionError::SavepointNotFound,
                &format!("Failed to rollback to savepoint: {}", e)
            ))?;

        Ok(())
    }

    async fn query(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        // Reuse the connection query implementation
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
                QueryError::SyntaxError,
                &format!("Failed to prepare query: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
                crate::stdlib::packages::db_core::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
                },
                _ => Box::leak(Box::new(rusqlite::types::Null)) as &dyn ToSql,
            })
            .collect();

        let rows = stmt.query_map(&sqlite_params[..], |row| {
            let mut values = Vec::new();
            for i in 0..row.as_ref().column_count() {
                let value = row.get::<usize, SqliteValue>(i)?;
                let column_value = crate::stdlib::packages::db_core::ColumnValue {
                    data: match &value {
                        SqliteValue::Null => None,
                        SqliteValue::Integer(i) => Some(i.to_le_bytes().to_vec()),
                        SqliteValue::Real(f) => Some(f.to_le_bytes().to_vec()),
                        SqliteValue::Text(s) => Some(s.as_bytes().to_vec()),
                        SqliteValue::Blob(data) => Some(data.clone()),
                    },
                    column_type: match value {
                        SqliteValue::Null => crate::stdlib::packages::db_core::ColumnType::Null,
                        SqliteValue::Integer(_) => crate::stdlib::packages::db_core::ColumnType::BigInt,
                        SqliteValue::Real(_) => crate::stdlib::packages::db_core::ColumnType::Double,
                        SqliteValue::Text(_) => crate::stdlib::packages::db_core::ColumnType::Text,
                        SqliteValue::Blob(_) => crate::stdlib::packages::db_core::ColumnType::Blob,
                    },
                    is_null: matches!(value, SqliteValue::Null),
                };
                values.push(column_value);
            }
            
            Ok(crate::stdlib::packages::db_core::Row {
                values,
                metadata: crate::stdlib::packages::db_core::RowMetadata {
                    row_number: 0,
                    is_inserted: false,
                    is_updated: false,
                    is_deleted: false,
                },
            })
        })
        .map_err(|e| DatabaseError::query(
            QueryError::ExecutionFailed,
            &format!("Query execution failed: {}", e)
        ))?;

        let mut result_rows = Vec::new();
        for row_result in rows {
            let row = row_result.map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("Row processing failed: {}", e)
            ))?;
            result_rows.push(row);
        }

        let columns = if let Some(first_row) = result_rows.first() {
            first_row.values.iter().enumerate().map(|(i, val)| {
                crate::stdlib::packages::db_core::Column {
                    name: format!("column_{}", i),
                    column_type: val.column_type.clone(),
                    nullable: true,
                    ordinal: i,
                    table_name: None,
                    schema_name: None,
                    precision: None,
                    scale: None,
                    max_length: None,
                    auto_increment: false,
                    default_value: None,
                }
            }).collect()
        } else {
            Vec::new()
        };

        let result_set = SqliteResultSet {
            rows: result_rows,
            metadata: crate::stdlib::packages::db_core::ResultMetadata {
                columns,
                row_count: None,
                affected_rows: 0,
                last_insert_id: None,
                warnings: Vec::new(),
                query_id: Some(uuid::Uuid::new_v4().to_string()),
                execution_time: std::time::Duration::from_millis(0),
            },
            current_index: 0,
        };

        Ok(Box::new(result_set))
    }

    async fn execute(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<crate::stdlib::packages::db_core::ExecuteResult> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
                QueryError::SyntaxError,
                &format!("Failed to prepare statement: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
                crate::stdlib::packages::db_core::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
                },
                _ => Box::leak(Box::new(rusqlite::types::Null)) as &dyn ToSql,
            })
            .collect();

        let affected_rows = stmt.execute(&sqlite_params[..])
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("Statement execution failed: {}", e)
            ))?;

        Ok(crate::stdlib::packages::db_core::ExecuteResult {
            affected_rows: affected_rows as u64,
            last_insert_id: Some(conn.last_insert_rowid() as u64),
            warnings: Vec::new(),
            query_id: Some(uuid::Uuid::new_v4().to_string()),
            execution_time: std::time::Duration::from_millis(0),
        })
    }

    fn state(&self) -> crate::stdlib::packages::db_core::TransactionState {
        if self.active {
            crate::stdlib::packages::db_core::TransactionState::Active
        } else {
            crate::stdlib::packages::db_core::TransactionState::Committed
        }
    }
}

impl std::fmt::Display for SqliteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SQLite Error: {}", self.message)
    }
}

impl std::error::Error for SqliteError {}
