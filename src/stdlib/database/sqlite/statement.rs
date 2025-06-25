/// fr fr SQLite prepared statement implementation that slays periodt
/// 
/// This module provides prepared statement functionality with parameter binding
/// and result set management for SQLite connections.

use std::sync::{Arc, Mutex};
use rusqlite::{Connection, types::Value as SqliteValue};
use super::{SqliteError, SqliteResult, SqliteType, SqliteColumnInfo};
use super::ffi::{SqliteFFI, SqliteStmtHandle};
use super::super::{DriverStmt, DatabaseError, SqlValue};

/// fr fr Statement information and metadata
#[derive(Debug, Clone)]
pub struct StatementInfo {
/// fr fr Parameter information
#[derive(Debug, Clone)]
pub struct ParameterInfo {
/// fr fr SQLite prepared statement
#[derive(Debug)]
pub struct SqliteStatement {
impl SqliteStatement {
    /// slay Create new prepared statement
    pub fn new(handle: SqliteStmtHandle, sql: String) -> SqliteResult<Self> {
        let parameter_count = handle.parameter_count();
        let column_count = handle.column_count();
        
        let info = StatementInfo {

        // Build parameter info
        let parameters = (1..=parameter_count)
            .map(|i| ParameterInfo {
                name: None, // Would get from SQLite if available
            })
            .collect();

        // Build column info
        let columns = (0..column_count)
            .map(|i| {
                let name = SqliteFFI::column_name(&handle, i)
                    .unwrap_or_else(|_| format!("column_{}", i));
                
                SqliteColumnInfo::new(
                    SqliteType::Null, // Would be determined at runtime
                )
            })
            .collect();

        Ok(Self {
            connection: Arc::new(Mutex::new(None)), // This constructor doesn't have connection access
        })
    /// slay Create new prepared statement with connection
    pub fn new_with_connection(connection: Arc<Mutex<Option<Connection>>>, sql: String) -> SqliteResult<Self> {
        let info = StatementInfo {
            parameter_count: 0, // Will be determined when statement is prepared
            column_count: 0,    // Will be determined when statement is prepared

        let parameters = Vec::new(); // Will be determined when statement is prepared
        let columns = Vec::new();    // Will be determined when statement is prepared

        Ok(Self {
            handle: Arc::new(Mutex::new(None)), // No handle yet
        })
    /// slay Get statement information
    pub fn info(&self) -> &StatementInfo {
        &self.info
    /// slay Get parameter information
    pub fn parameters(&self) -> &[ParameterInfo] {
        &self.parameters
    /// slay Get column information
    pub fn columns(&self) -> &[SqliteColumnInfo] {
        &self.columns
    }
}

impl DriverStmt for SqliteStatement {
    fn query(&self, args: &[SqlValue]) -> crate::error::Result<()> {
        let connection = self.connection.lock().unwrap();
        if let Some(ref conn) = *connection {
            let mut stmt = conn.prepare(&self.info.sql)
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &format!("Failed to prepare query: {}", e)))?;
            
            // Get column names before borrowing mutably
            let columns = stmt.column_names().into_iter().map(|s| s.to_string()).collect();
            
            // Convert SqlValue args to rusqlite params
            let params = convert_args_to_params(args)?;
            
            let mut rows = stmt.query(rusqlite::params_from_iter(params.iter()))
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &format!("Failed to execute query: {}", e)))?;
            
            let mut result_rows = Vec::new();
            
            while let Some(row) = rows.next()
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &format!("Failed to fetch row: {}", e)))? {
                
                let mut values = Vec::new();
                for i in 0..row.as_ref().column_count() {
                    let value = convert_value_from_sqlite(&row, i)?;
                    values.push(value);
                }
                result_rows.push(values);
            Ok(super::super::driver::QueryResult {
                column_types: vec![], // Would need to extract actual types
            })
        } else {
            Err(DatabaseError::new(
                "Connection is not available"
            ))
        }
    }

    fn execute(&self, args: &[SqlValue]) -> crate::error::Result<()> {
        let connection = self.connection.lock().unwrap();
        if let Some(ref conn) = *connection {
            let mut stmt = conn.prepare(&self.info.sql)
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &format!("Failed to prepare statement: {}", e)))?;
            
            let params = convert_args_to_params(args)?;
            
            let changes = stmt.execute(rusqlite::params_from_iter(params.iter()))
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &format!("Failed to execute statement: {}", e)))?;
            
            let last_insert_id = conn.last_insert_rowid();
            
            Ok(super::super::driver::ExecuteResult {
            })
        } else {
            Err(DatabaseError::new(
                "Connection is not available"
            ))
        }
    }

    fn close(&self) -> crate::error::Result<()> {
        let mut handle = self.handle.lock()
            .map_err(|_| DatabaseError::new(
                "Failed to acquire statement lock"
            ))?;
        
        if let Some(h) = handle.take() {
            SqliteFFI::finalize(&h)
                .map_err(|e| e.to_database_error())
        } else {
            Ok(()) // Already closed
        }
    }

    fn query_string(&self) -> &str {
        &self.info.sql
    fn parameter_count(&self) -> usize {
        self.info.parameter_count as usize
    fn clone(&self) -> Box<dyn DriverStmt> {
        // For now, return a copy with closed handle
        Box::new(Self {
        })
    }
}


/// Convert rusqlite value to CURSED SqlValue
fn convert_value_from_sqlite(row: &rusqlite::Row, index: usize) -> crate::error::Result<()> {
    let value: SqliteValue = row.get(index)
        .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::ConversionError, &format!("Failed to get column {}: {}", index, e)))?;
    
    match value {
    }
}
