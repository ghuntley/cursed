/// fr fr SQLite driver implementation - the lightweight champion periodt

use crate::runtime::value::Value;
// Placeholder imports disabled
    db_core::{
        ExecuteResult, TransactionIsolation
    // types::ParameterDirection  // Explicit import to resolve E0659
// };
use crate::error::CursedError;
// Placeholder imports disabled
    SqlTransactionIsolation, SqlConnectionInfo, SqlBatch, SqlTransaction
// };

// Placeholder imports disabled
    DatabaseResult as DbResult, DatabaseError, ErrorKind, ConnectionError, QueryError, TransactionError
// };
use async_trait::async_trait;
use rusqlite::{Connection, Statement, Row, ToSql, types::Value as SqliteValue, Transaction as SqliteTransaction};
use std::sync::{Arc, Mutex};
use std::path::Path;

/// fr fr SQLite driver
#[derive(Debug)]
pub struct SqliteDriver {
/// fr fr SQLite connection
#[derive(Debug)]
pub struct SqliteConnection {
/// fr fr SQLite error
#[derive(Debug)]
pub struct SqliteError {
/// fr fr SQLite result set implementation
#[derive(Debug)]
pub struct SqliteResultSet {
//     rows: Vec<crate::stdlib::packages::db_core::Row>,
//     metadata: crate::stdlib::packages::db_core::ResultMetadata,
/// fr fr SQLite prepared statement implementation
#[derive(Debug)]
pub struct SqlitePreparedStatement {
/// fr fr SQLite transaction implementation
#[derive(Debug)]
pub struct SqliteTransactionImpl {
impl SqliteDriver {
    pub fn new() -> Self {
        Self {
        }
    }
impl SqliteConnection {
    /// slay Create a new SQLite connection
    pub fn new(database_path: &str) -> DbResult<Self> {
        let conn = Connection::open(database_path)
            .map_err(|e| DatabaseError::connection(
                &format!("Failed to open SQLite database: {}", e)
            ))?;

        // Enable foreign key constraints by default
        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| DatabaseError::driver(&format!("Failed to enable foreign keys: {}", e)))?;

        Ok(Self {
        })
    /// slay Create an in-memory SQLite connection
    pub fn new_in_memory() -> DbResult<Self> {
        Self::new(":memory:")
    /// slay Convert SqlValue to rusqlite ToSql
    fn sql_value_to_sqlite(value: &SqlValue) -> Box<dyn ToSql> {
        match value {
        }
    }

    /// slay Convert rusqlite Value to SqlValue
    fn sqlite_value_to_sql(value: SqliteValue) -> SqlValue {
        match value {
        }
    }
#[async_trait]
// impl crate::stdlib::packages::db_core::DatabaseDriver for SqliteDriver {
    async fn connect(&self, config: ConnectionConfig) -> DbResult<Box<dyn DatabaseConnection>> {
        let database_path = config.connection_string
            .strip_prefix("sqlite://")
            .unwrap_or(&config.connection_string);
        
        let conn = SqliteConnection::new(database_path)?;
        Ok(Box::new(conn))
//     fn driver_info(&self) -> crate::stdlib::packages::db_core::DriverInfo {
//         crate::stdlib::packages::db_core::DriverInfo::new(
            "CURSED"
        )
    fn supports_feature(&self, _feature: DriverFeature) -> bool {
        true
    fn sql_dialect(&self) -> SqlDialect {
        SqlDialect::SQLite
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
    fn sql_dialect(&self) -> Box<dyn SqlDialectTrait> {
//         Box::new(crate::stdlib::packages::db_sql::SqliteDialect::new())
//     fn supported_types(&self) -> Vec<crate::stdlib::packages::db_sql::SqlType> {
        vec![
//             crate::stdlib::packages::db_sql::SqlType::Integer,
//             crate::stdlib::packages::db_sql::SqlType::Text,
//             crate::stdlib::packages::db_sql::SqlType::Boolean,
//             crate::stdlib::packages::db_sql::SqlType::Blob,
        ]
    fn supports_sql_feature(&self, feature: SqlFeature) -> bool {
        // SQLite has some limitations
        match feature {
            _ => false, // Many advanced features not supported
        }
    }

    fn configuration_options(&self) -> Vec<ConfigurationOption> {
        Vec::from([])
    fn validate_sql(&self, _sql: &str) -> DbResult<()> {
        Ok(())
    fn performance_info(&self) -> DriverPerformanceInfo {
        DriverPerformanceInfo {
            max_connections: Some(1), // SQLite is single-threaded
        }
    }

    fn limitations(&self) -> DriverLimitations {
        DriverLimitations {
            unsupported_features: vec![
        }
    }
#[async_trait]
impl DatabaseConnection for SqliteConnection {
    async fn query(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
                &format!("Failed to prepare query: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
//                 crate::stdlib::packages::types::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
                }
            })
            .collect();

        let rows = stmt.query_map(&sqlite_params[..], |row| {
            let mut values = Vec::new();
            for i in 0..row.as_ref().column_count() {
                let value = row.get::<usize, SqliteValue>(i)?;
//                 let column_value = crate::stdlib::packages::db_core::ColumnValue {
                    data: match &value {
                    column_type: match value {
//                         SqliteValue::Null => crate::stdlib::packages::db_core::ColumnType::Null,
//                         SqliteValue::Integer(_) => crate::stdlib::packages::db_core::ColumnType::BigInt,
//                         SqliteValue::Real(_) => crate::stdlib::packages::db_core::ColumnType::Double,
//                         SqliteValue::Text(_) => crate::stdlib::packages::db_core::ColumnType::Text,
//                         SqliteValue::Blob(_) => crate::stdlib::packages::db_core::ColumnType::Blob,
                values.push(column_value);
//             Ok(crate::stdlib::packages::db_core::Row {
//                 metadata: crate::stdlib::packages::db_core::RowMetadata {
            })
        })
        .map_err(|e| DatabaseError::query(
            &format!("Query execution failed: {}", e)
        ))?;

        let mut result_rows = Vec::new();
        for row_result in rows {
            let row = row_result.map_err(|e| DatabaseError::query(
                &format!("Row processing failed: {}", e)
            ))?;
            result_rows.push(row);
        let columns = if let Some(first_row) = result_rows.first() {
            first_row.values.iter().enumerate().map(|(i, val)| {
//                 crate::stdlib::packages::db_core::Column {
                }
            }).collect()
        } else {
            Vec::new()

        let result_set = SqliteResultSet {
//             metadata: crate::stdlib::packages::db_core::ResultMetadata {

        Ok(Box::new(result_set))
    async fn execute(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<ExecuteResult> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
                &format!("Failed to prepare statement: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
//                 crate::stdlib::packages::types::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
            })
            .collect();

        let affected_rows = stmt.execute(&sqlite_params[..])
            .map_err(|e| DatabaseError::query(
                &format!("Statement execution failed: {}", e)
            ))?;

        Ok(ExecuteResult {
        })
    async fn prepare(&mut self, sql: &str) -> DbResult<Box<dyn PreparedStatement>> {
        let stmt = SqlitePreparedStatement {

        Ok(Box::new(stmt))
//     async fn begin_transaction(&mut self, _options: Option<crate::stdlib::packages::db_core::TransactionOptions>) -> DbResult<Box<dyn DatabaseTransaction>> {
        if self.in_transaction {
            return Err(DatabaseError::transaction(
                "Transaction already active"
            ));
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute("BEGIN TRANSACTION", [])
            .map_err(|e| DatabaseError::transaction(
                &format!("Failed to begin transaction: {}", e)
            ))?;

        self.in_transaction = true;

        let transaction = SqliteTransactionImpl {

        Ok(Box::new(transaction))
    async fn ping(&mut self) -> DbResult<()> {
        // SQLite is always "connected" as it's a file-based database
        // We can test with a simple query
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute("SELECT 1", [])
            .map_err(|e| DatabaseError::connection(
                &format!("Ping failed: {}", e)
            ))?;

        Ok(())
    async fn close(self: Box<Self>) -> DbResult<()> {
        // SQLite connections close automatically when dropped
        Ok(())
//     fn connection_info(&self) -> crate::stdlib::packages::db_core::traits::ConnectionInfo {
//         crate::stdlib::packages::db_core::traits::ConnectionInfo {
//             transaction_isolation: crate::stdlib::packages::db_core::traits::TransactionIsolation::Serializable,
        }
    }
#[async_trait]
impl SqlConnection for SqliteConnection {
    async fn sql_query(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlResultSet> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
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
            &format!("Query execution failed: {}", e)
        ))?;

        let mut result_rows = Vec::new();
        for row_result in rows {
            let row_values = row_result.map_err(|e| DatabaseError::query(
                &format!("Row processing failed: {}", e)
            ))?;
            result_rows.push(row_values);
        // Get column names from the statement
        let column_names: Vec<String> = stmt.column_names().iter()
            .map(|name| name.to_string())
            .collect();

        let result_set = SqlResultSet {

        Ok(result_set)
    async fn sql_execute(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlExecuteResult> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
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
                &format!("Statement execution failed: {}", e)
            ))?;

        Ok(SqlExecuteResult {
        })
    async fn sql_prepare(&mut self, sql: &str) -> DbResult<Box<dyn PreparedStatement>> {
        let stmt = SqlitePreparedStatement {

        Ok(Box::new(stmt))
    async fn sql_begin_transaction(&mut self, isolation: Option<SqlTransactionIsolation>) -> DbResult<Box<dyn SqlTransaction>> {
        if self.in_transaction {
            return Err(DatabaseError::transaction(
                "Transaction already active"
            ));
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        // Set isolation level if specified (SQLite uses PRAGMA for this)
        if let Some(isolation_level) = isolation {
            let pragma_sql = match isolation_level {
            
            conn.execute(pragma_sql, [])
                .map_err(|e| DatabaseError::transaction(
                    &format!("Failed to set isolation level: {}", e)
                ))?;
        conn.execute("BEGIN TRANSACTION", [])
            .map_err(|e| DatabaseError::transaction(
                &format!("Failed to begin transaction: {}", e)
            ))?;

        self.in_transaction = true;

        let transaction = SqliteTransactionImpl {

        Ok(Box::new(transaction))
    async fn sql_batch(&mut self, statements: &[SqlBatch]) -> DbResult<Vec<SqlExecuteResult>> {
        let mut results = Vec::new();
        
        for batch_stmt in statements {
            let result = self.sql_execute(&batch_stmt.sql, &batch_stmt.parameters).await?;
            results.push(result);
        Ok(results)
    fn sql_connection_info(&self) -> SqlConnectionInfo {
        SqlConnectionInfo {
        }
    }

    async fn set_sql_variable(&mut self, name: &str, value: &SqlValue) -> DbResult<()> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        // SQLite uses PRAGMA statements instead of variables
        // Convert SqlValue to string for PRAGMA usage
        let value_str = match value {
            SqlValue::Text(s) => format!("'{}'", s.replace("'", "''")), // Escape single quotes

        let pragma_sql = format!("PRAGMA {} = {}", name, value_str);
        
        conn.execute(&pragma_sql, [])
            .map_err(|e| DatabaseError::driver(&format!("Failed to set PRAGMA {}: {}", name, e)))?;

        Ok(())
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
        // If no rows returned, return NULL
        Ok(SqlValue::Null)
    }
}

// Implement ResultSet trait for SqliteResultSet
impl ResultSet for SqliteResultSet {
//     fn next(&mut self) -> DbResult<Option<crate::stdlib::packages::db_core::Row>> {
        if self.current_index < self.rows.len() {
            let row = self.rows[self.current_index].clone();
            self.current_index += 1;
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

//     fn collect(&mut self) -> DbResult<Vec<crate::stdlib::packages::db_core::Row>> {
        let mut result = Vec::new();
        while let Some(row) = self.next()? {
            result.push(row);
        }
        Ok(result)
//     fn columns(&self) -> &[crate::stdlib::packages::db_core::Column] {
        &self.metadata.columns
    fn has_next(&self) -> bool {
        self.current_index < self.rows.len()
//     fn metadata(&self) -> &crate::stdlib::packages::db_core::ResultMetadata {
        &self.metadata
    fn row_count(&self) -> Option<usize> {
        Some(self.rows.len())
    fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

// Implement PreparedStatement trait for SqlitePreparedStatement
#[async_trait]
impl PreparedStatement for SqlitePreparedStatement {
//     async fn execute(&mut self, parameters: &[Parameter]) -> DbResult<crate::stdlib::packages::db_core::ExecuteResult> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(&self.sql)
            .map_err(|e| DatabaseError::query(
                &format!("Failed to prepare statement: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
//                 crate::stdlib::packages::types::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
            })
            .collect();

        let affected_rows = stmt.execute(&sqlite_params[..])
            .map_err(|e| DatabaseError::query(
                &format!("Statement execution failed: {}", e)
            ))?;

//         Ok(crate::stdlib::packages::db_core::ExecuteResult {
        })
    async fn query(&mut self, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(&self.sql)
            .map_err(|e| DatabaseError::query(
                &format!("Failed to prepare query: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
//                 crate::stdlib::packages::types::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
            })
            .collect();

        let rows = stmt.query_map(&sqlite_params[..], |row| {
            let mut values = Vec::new();
            for i in 0..row.as_ref().column_count() {
                let value = row.get::<usize, SqliteValue>(i)?;
//                 let column_value = crate::stdlib::packages::db_core::ColumnValue {
                    data: match &value {
                    column_type: match value {
//                         SqliteValue::Null => crate::stdlib::packages::db_core::ColumnType::Null,
//                         SqliteValue::Integer(_) => crate::stdlib::packages::db_core::ColumnType::BigInt,
//                         SqliteValue::Real(_) => crate::stdlib::packages::db_core::ColumnType::Double,
//                         SqliteValue::Text(_) => crate::stdlib::packages::db_core::ColumnType::Text,
//                         SqliteValue::Blob(_) => crate::stdlib::packages::db_core::ColumnType::Blob,
                values.push(column_value);
//             Ok(crate::stdlib::packages::db_core::Row {
//                 metadata: crate::stdlib::packages::db_core::RowMetadata {
            })
        })
        .map_err(|e| DatabaseError::query(
            &format!("Query execution failed: {}", e)
        ))?;

        let mut result_rows = Vec::new();
        for row_result in rows {
            let row = row_result.map_err(|e| DatabaseError::query(
                &format!("Row processing failed: {}", e)
            ))?;
            result_rows.push(row);
        let columns = if let Some(first_row) = result_rows.first() {
            first_row.values.iter().enumerate().map(|(i, val)| {
//                 crate::stdlib::packages::db_core::Column {
                }
            }).collect()
        } else {
            Vec::new()

        let result_set = SqliteResultSet {
//             metadata: crate::stdlib::packages::db_core::ResultMetadata {

        Ok(Box::new(result_set))
    fn parameter_count(&self) -> usize {
        // SQLite doesn't expose parameter count directly, we'll estimate
        self.sql.matches('?').count()
    fn sql(&self) -> &str {
        &self.sql
    /// slay Get parameter metadata
//     fn parameter_metadata(&self) -> &[crate::stdlib::packages::db_core::ParameterMetadata] {
        // Placeholder implementation - would need to extract from SQLite statement
        &[]
    /// slay Get result set metadata
//     fn result_metadata(&self) -> &crate::stdlib::packages::db_core::ResultMetadata {
        // Placeholder implementation - would need to extract from SQLite statement
//         static EMPTY_METADATA: std::sync::LazyLock<crate::stdlib::packages::db_core::ResultMetadata> = 
//             std::sync::LazyLock::new(|| crate::stdlib::packages::db_core::ResultMetadata {
//                 result_type: crate::stdlib::packages::db_core::result::ResultType::ForwardOnly,
            });
        &EMPTY_METADATA
    /// slay Close the prepared statement
    async fn close(self: Box<Self>) -> DbResult<()> {
        // SQLite prepared statements are automatically cleaned up
        Ok(())
    }
}

// Implement DatabaseTransaction trait for SqliteTransactionImpl
#[async_trait]
impl DatabaseTransaction for SqliteTransactionImpl {
    async fn commit(mut self: Box<Self>) -> DbResult<()> {
        if !self.active {
            return Err(DatabaseError::transaction(
                "Transaction already completed"
            ));
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute("COMMIT", [])
            .map_err(|e| DatabaseError::transaction(
                &format!("Failed to commit transaction: {}", e)
            ))?;

        self.active = false;
        Ok(())
    async fn rollback(mut self: Box<Self>) -> DbResult<()> {
        if !self.active {
            return Err(DatabaseError::transaction(
                "Transaction already completed"
            ));
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute("ROLLBACK", [])
            .map_err(|e| DatabaseError::transaction(
                &format!("Failed to rollback transaction: {}", e)
            ))?;

        self.active = false;
        Ok(())
//     async fn savepoint(&mut self, name: &str) -> DbResult<crate::stdlib::packages::db_core::SavePoint> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute(&format!("SAVEPOINT {}", name), [])
            .map_err(|e| DatabaseError::transaction(
                &format!("Failed to create savepoint: {}", e)
            ))?;

//         Ok(crate::stdlib::packages::db_core::SavePoint {
        })
//     async fn rollback_to_savepoint(&mut self, savepoint: &crate::stdlib::packages::db_core::SavePoint) -> DbResult<()> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        conn.execute(&format!("ROLLBACK TO SAVEPOINT {}", savepoint.name), [])
            .map_err(|e| DatabaseError::transaction(
                &format!("Failed to rollback to savepoint: {}", e)
            ))?;

        Ok(())
    async fn query(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        // Reuse the connection query implementation
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
                &format!("Failed to prepare query: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
//                 crate::stdlib::packages::types::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
            })
            .collect();

        let rows = stmt.query_map(&sqlite_params[..], |row| {
            let mut values = Vec::new();
            for i in 0..row.as_ref().column_count() {
                let value = row.get::<usize, SqliteValue>(i)?;
//                 let column_value = crate::stdlib::packages::db_core::ColumnValue {
                    data: match &value {
                    column_type: match value {
//                         SqliteValue::Null => crate::stdlib::packages::db_core::ColumnType::Null,
//                         SqliteValue::Integer(_) => crate::stdlib::packages::db_core::ColumnType::BigInt,
//                         SqliteValue::Real(_) => crate::stdlib::packages::db_core::ColumnType::Double,
//                         SqliteValue::Text(_) => crate::stdlib::packages::db_core::ColumnType::Text,
//                         SqliteValue::Blob(_) => crate::stdlib::packages::db_core::ColumnType::Blob,
                values.push(column_value);
//             Ok(crate::stdlib::packages::db_core::Row {
//                 metadata: crate::stdlib::packages::db_core::RowMetadata {
            })
        })
        .map_err(|e| DatabaseError::query(
            &format!("Query execution failed: {}", e)
        ))?;

        let mut result_rows = Vec::new();
        for row_result in rows {
            let row = row_result.map_err(|e| DatabaseError::query(
                &format!("Row processing failed: {}", e)
            ))?;
            result_rows.push(row);
        let columns = if let Some(first_row) = result_rows.first() {
            first_row.values.iter().enumerate().map(|(i, val)| {
//                 crate::stdlib::packages::db_core::Column {
                }
            }).collect()
        } else {
            Vec::new()

        let result_set = SqliteResultSet {
//             metadata: crate::stdlib::packages::db_core::ResultMetadata {

        Ok(Box::new(result_set))
//     async fn execute(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<crate::stdlib::packages::db_core::ExecuteResult> {
        let conn = self.connection.lock()
            .map_err(|_| DatabaseError::driver("Failed to acquire connection lock"))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DatabaseError::query(
                &format!("Failed to prepare statement: {}", e)
            ))?;

        // Convert parameters to rusqlite format
        let sqlite_params: Vec<&dyn ToSql> = parameters.iter()
            .map(|p| match p.direction {
//                 crate::stdlib::packages::types::ParameterDirection::In => {
                    Box::leak(Box::new(p.value.clone())) as &dyn ToSql
            })
            .collect();

        let affected_rows = stmt.execute(&sqlite_params[..])
            .map_err(|e| DatabaseError::query(
                &format!("Statement execution failed: {}", e)
            ))?;

//         Ok(crate::stdlib::packages::db_core::ExecuteResult {
        })
//     fn state(&self) -> crate::stdlib::packages::db_core::traits::TransactionState {
        if self.active {
//             crate::stdlib::packages::db_core::traits::TransactionState::Active
        } else {
//             crate::stdlib::packages::db_core::traits::TransactionState::Committed
        }
    }
// impl std::fmt::Display for SqliteError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "SQLite CursedError: {}", self.message)
//     }
// }

// impl std::error::CursedError for SqliteError {}
// 