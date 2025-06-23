/// fr fr SQLite prepared statement implementation that slays periodt
/// 
/// This module provides prepared statement functionality with parameter binding
/// and result set management for SQLite connections.

use std::sync::{Arc, Mutex};
use rusqlite::{Connection, crate::types::Value as SqliteValue};
use super::{SqliteError, SqliteResult, SqliteType, SqliteColumnInfo};
use super::ffi::{SqliteFFI, SqliteStmtHandle};
use super::super::{DriverStmt, DatabaseError, SqlValue};

/// fr fr Statement information and metadata
#[derive(Debug, Clone)]
pub struct StatementInfo {
    pub sql: String,
    pub parameter_count: i32,
    pub column_count: i32,
    pub prepared_at: std::time::SystemTime,
    pub execution_count: u64,
}

/// fr fr Parameter information
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub index: i32,
    pub name: Option<String>,
    pub data_type: Option<SqliteType>,
}

/// fr fr SQLite prepared statement
#[derive(Debug)]
pub struct SqliteStatement {
    handle: Arc<Mutex<Option<SqliteStmtHandle>>>,
    connection: Arc<Mutex<Option<Connection>>>,
    info: StatementInfo,
    parameters: Vec<ParameterInfo>,
    columns: Vec<SqliteColumnInfo>,
}

impl SqliteStatement {
    /// slay Create new prepared statement
    pub fn new(handle: SqliteStmtHandle, sql: String) -> SqliteResult<Self> {
        let parameter_count = handle.parameter_count();
        let column_count = handle.column_count();
        
        let info = StatementInfo {
            sql: sql.clone(),
            parameter_count,
            column_count,
            prepared_at: std::time::SystemTime::now(),
            execution_count: 0,
        };

        // Build parameter info
        let parameters = (1..=parameter_count)
            .map(|i| ParameterInfo {
                index: i,
                name: None, // Would get from SQLite if available
                data_type: None,
            })
            .collect();

        // Build column info
        let columns = (0..column_count)
            .map(|i| {
                let name = SqliteFFI::column_name(&handle, i)
                    .unwrap_or_else(|_| format!("column_{}", i));
                
                SqliteColumnInfo::new(
                    name,
                    SqliteType::Null, // Would be determined at runtime
                    i as usize,
                    "UNKNOWN".to_string(),
                )
            })
            .collect();

        Ok(Self {
            handle: Arc::new(Mutex::new(Some(handle))),
            connection: Arc::new(Mutex::new(None)), // This constructor doesn't have connection access
            info,
            parameters,
            columns,
        })
    }

    /// slay Create new prepared statement with connection
    pub fn new_with_connection(connection: Arc<Mutex<Option<Connection>>>, sql: String) -> SqliteResult<Self> {
        let info = StatementInfo {
            sql: sql.clone(),
            parameter_count: 0, // Will be determined when statement is prepared
            column_count: 0,    // Will be determined when statement is prepared
            prepared_at: std::time::SystemTime::now(),
            execution_count: 0,
        };

        let parameters = Vec::new(); // Will be determined when statement is prepared
        let columns = Vec::new();    // Will be determined when statement is prepared

        Ok(Self {
            handle: Arc::new(Mutex::new(None)), // No handle yet
            connection,
            info,
            parameters,
            columns,
        })
    }

    /// slay Get statement information
    pub fn info(&self) -> &StatementInfo {
        &self.info
    }

    /// slay Get parameter information
    pub fn parameters(&self) -> &[ParameterInfo] {
        &self.parameters
    }

    /// slay Get column information
    pub fn columns(&self) -> &[SqliteColumnInfo] {
        &self.columns
    }
}

impl DriverStmt for SqliteStatement {
    fn query(&self, args: &[SqlValue]) -> Result<(), Error> {
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
            }
            
            Ok(super::super::driver::QueryResult {
                column_names: columns,
                column_types: vec![], // Would need to extract actual types
                rows: result_rows,
                error: None,
            })
        } else {
            Err(DatabaseError::new(
                super::super::DatabaseErrorKind::ConnectionError,
                "Connection is not available"
            ))
        }
    }

    fn execute(&self, args: &[SqlValue]) -> Result<(), Error> {
        let connection = self.connection.lock().unwrap();
        if let Some(ref conn) = *connection {
            let mut stmt = conn.prepare(&self.info.sql)
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &format!("Failed to prepare statement: {}", e)))?;
            
            let params = convert_args_to_params(args)?;
            
            let changes = stmt.execute(rusqlite::params_from_iter(params.iter()))
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &format!("Failed to execute statement: {}", e)))?;
            
            let last_insert_id = conn.last_insert_rowid();
            
            Ok(super::super::driver::ExecuteResult {
                rows_affected: changes as i64,
                last_insert_id: Some(last_insert_id as i64),
            })
        } else {
            Err(DatabaseError::new(
                super::super::DatabaseErrorKind::ConnectionError,
                "Connection is not available"
            ))
        }
    }

    fn close(&self) -> Result<(), Error> {
        let mut handle = self.handle.lock()
            .map_err(|_| DatabaseError::new(
                super::super::DatabaseErrorKind::ConnectionError,
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
    }

    fn parameter_count(&self) -> usize {
        self.info.parameter_count as usize
    }

    fn clone(&self) -> Box<dyn DriverStmt> {
        // For now, return a copy with closed handle
        Box::new(Self {
            handle: Arc::new(Mutex::new(None)),
            connection: self.connection.clone(),
            info: self.info.clone(),
            parameters: self.parameters.clone(),
            columns: self.columns.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statement_info() {
        let info = StatementInfo {
            sql: "SELECT * FROM users WHERE id = ?".to_string(),
            parameter_count: 1,
            column_count: 3,
            prepared_at: std::time::SystemTime::now(),
            execution_count: 0,
        };

        assert_eq!(info.parameter_count, 1);
        assert_eq!(info.column_count, 3);
        assert!(info.sql.contains("SELECT"));
    }

    #[test]
    fn test_parameter_info() {
        let param = ParameterInfo {
            index: 1,
            name: Some("id".to_string()),
            data_type: Some(SqliteType::Integer),
        };

        assert_eq!(param.index, 1);
        assert_eq!(param.name, Some("id".to_string()));
        assert_eq!(param.data_type, Some(SqliteType::Integer));
    }
}

/// Convert CURSED SqlValue to rusqlite parameters
fn convert_args_to_params(args: &[SqlValue]) -> Result<(), Error> {
    let mut params = Vec::new();
    
    for arg in args {
        match arg {
            SqlValue::Null => params.push(Box::new(rusqlite::crate::types::Null) as Box<dyn rusqlite::ToSql>),
            SqlValue::Boolean(b) => params.push(Box::new(*b) as Box<dyn rusqlite::ToSql>),
            SqlValue::Integer(i) => params.push(Box::new(*i) as Box<dyn rusqlite::ToSql>),
            SqlValue::Float(f) => params.push(Box::new(*f) as Box<dyn rusqlite::ToSql>),
            SqlValue::String(s) => params.push(Box::new(s.clone()) as Box<dyn rusqlite::ToSql>),
            SqlValue::Bytes(b) => params.push(Box::new(b.clone()) as Box<dyn rusqlite::ToSql>),
            _ => return Err(DatabaseError::new(
                super::super::DatabaseErrorKind::ConversionError,
                &format!("Unsupported SqlValue type: {:?}", arg)
            )),
        }
    }
    
    Ok(params)
}

/// Convert rusqlite value to CURSED SqlValue
fn convert_value_from_sqlite(row: &rusqlite::Row, index: usize) -> Result<(), Error> {
    let value: SqliteValue = row.get(index)
        .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::ConversionError, &format!("Failed to get column {}: {}", index, e)))?;
    
    match value {
        SqliteValue::Null => Ok(SqlValue::Null),
        SqliteValue::Integer(i) => Ok(SqlValue::Integer(i)),
        SqliteValue::Real(f) => Ok(SqlValue::Float(f)),
        SqliteValue::Text(s) => Ok(SqlValue::String(s)),
        SqliteValue::Blob(b) => Ok(SqlValue::Bytes(b)),
    }
}
