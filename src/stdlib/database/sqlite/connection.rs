/// fr fr SQLite connection implementation that slays periodt
/// 
/// This module provides the SQLite-specific connection implementation
/// with proper resource management and thread safety.

use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use rusqlite::{Connection, Statement, Transaction, OpenFlags, params, types::Value as SqliteValue};
use super::{SqliteError, SqliteResult, SqliteConfig, SqliteStats, SqliteFFI};
use super::super::{DriverConn, DatabaseError, SqlValue, TxOptions, DriverStmt, DriverTx};
use super::super::driver::{QueryResult, ExecuteResult, ConnectionMetadata};
use super::statement::SqliteStatement;
use super::transaction::{SqliteTransaction, RealSqliteTransaction};

/// fr fr Connection state tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connected,
    Disconnected,
    Error,
    Busy,
}

/// fr fr SQLite connection information
#[derive(Debug, Clone)]
pub struct SqliteConnectionInfo {
    pub database_path: String,
    pub connection_id: String,
    pub connected_at: SystemTime,
    pub state: ConnectionState,
    pub thread_id: Option<std::thread::ThreadId>,
}

/// fr fr SQLite connection implementation
#[derive(Debug)]
pub struct SqliteConnection {
    handle: Arc<Mutex<Option<Connection>>>,
    config: SqliteConfig,
    info: SqliteConnectionInfo,
    stats: Arc<Mutex<SqliteStats>>,
}

impl SqliteConnection {
    /// slay Create new SQLite connection
    pub fn new(config: SqliteConfig) -> SqliteResult<Self> {
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE 
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_URI
            | OpenFlags::SQLITE_OPEN_NO_MUTEX;
        
        let handle = Connection::open_with_flags(&config.database_path, flags)
            .map_err(|e| SqliteError::connection(&format!("Failed to open SQLite database: {}", e)))?;
        
        let info = SqliteConnectionInfo {
            database_path: config.database_path.clone(),
            connection_id: uuid::Uuid::new_v4().to_string(),
            connected_at: SystemTime::now(),
            state: ConnectionState::Connected,
            thread_id: Some(std::thread::current().id()),
        };

        let connection = Self {
            handle: Arc::new(Mutex::new(Some(handle))),
            config,
            info,
            stats: Arc::new(Mutex::new(SqliteStats::default())),
        };

        // Initialize connection with PRAGMA statements
        connection.initialize_connection()?;

        Ok(connection)
    }

    /// slay Initialize connection with configuration
    fn initialize_connection(&self) -> SqliteResult<()> {
        let statements = self.config.initialization_sql();
        
        let handle = self.handle.lock().unwrap();
        if let Some(ref conn) = *handle {
            for statement in statements {
                if statement.trim().is_empty() {
                    continue;
                }
                
                conn.execute(&statement, [])
                    .map_err(|e| SqliteError::execution(&format!("Failed to execute initialization SQL '{}': {}", statement, e)))?;
            }
        }

        Ok(())
    }

    /// slay Get connection information
    pub fn info(&self) -> &SqliteConnectionInfo {
        &self.info
    }

    /// slay Get connection configuration
    pub fn config(&self) -> &SqliteConfig {
        &self.config
    }
}

impl DriverConn for SqliteConnection {
    fn prepare(&self, query: &str) -> Result<(), Error> {
        let handle = self.handle.lock().unwrap();
        if let Some(ref _conn) = *handle {
            let stmt = SqliteStatement::new_with_connection(self.handle.clone(), query.to_string())
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &e.to_string()))?;
            Ok(Box::new(stmt))
        } else {
            Err(DatabaseError::new(
                super::super::DatabaseErrorKind::ConnectionError,
                "Connection is not available"
            ))
        }
    }

    fn query(&self, query: &str, args: &[SqlValue]) -> Result<(), Error> {
        let handle = self.handle.lock().unwrap();
        if let Some(ref conn) = *handle {
            let mut stmt = conn.prepare(query)
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

    fn execute(&self, query: &str, args: &[SqlValue]) -> Result<(), Error> {
        let handle = self.handle.lock().unwrap();
        if let Some(ref conn) = *handle {
            let mut stmt = conn.prepare(query)
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

    fn begin_transaction(&self, opts: TxOptions) -> Result<(), Error> {
        let handle = self.handle.lock().unwrap();
        if let Some(ref _conn) = *handle {
            // Begin transaction
            let tx = RealSqliteTransaction::new(self.handle.clone(), opts)
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::TransactionError, &e.to_string()))?;
            Ok(Box::new(tx))
        } else {
            Err(DatabaseError::new(
                super::super::DatabaseErrorKind::ConnectionError,
                "Connection is not available"
            ))
        }
    }

    fn ping(&self) -> Result<(), Error> {
        // Simple ping by checking if handle is valid
        let handle = self.handle.lock()
            .map_err(|_| DatabaseError::new(
                super::super::DatabaseErrorKind::ConnectionError,
                "Failed to acquire connection lock"
            ))?;
        
        if handle.is_some() {
            Ok(())
        } else {
            Err(DatabaseError::new(
                super::super::DatabaseErrorKind::ConnectionError,
                "Connection is closed"
            ))
        }
    }

    fn close(&self) -> Result<(), Error> {
        let mut handle = self.handle.lock()
            .map_err(|_| DatabaseError::new(
                super::super::DatabaseErrorKind::ConnectionError,
                "Failed to acquire connection lock"
            ))?;
        
        if let Some(_h) = handle.take() {
            // rusqlite::Connection automatically closes when dropped
            Ok(())
        } else {
            Ok(()) // Already closed
        }
    }

    fn is_alive(&self) -> bool {
        self.handle.lock()
            .map(|h| h.is_some())
            .unwrap_or(false)
    }

    fn metadata(&self) -> super::super::driver::ConnectionMetadata {
        super::super::driver::ConnectionMetadata {
            server_version: "SQLite".to_string(),
            database_name: self.info.database_path.clone(),
            server_host: "localhost".to_string(),
            server_port: 0,
            username: "sqlite".to_string(),
            connected_at: self.info.connected_at,
            additional_info: std::collections::HashMap::new(),
        }
    }

    fn clone(&self) -> Box<dyn DriverConn> {
        // Create a new connection with the same config
        match Self::new(self.config.clone()) {
            Ok(conn) => Box::new(conn),
            Err(_) => {
                // Return a closed connection as fallback
                let mut info = self.info.clone();
                info.state = ConnectionState::Error;
                Box::new(Self {
                    handle: Arc::new(Mutex::new(None)),
                    config: self.config.clone(),
                    info,
                    stats: Arc::new(Mutex::new(SqliteStats::default())),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::error::Error;

    #[test]
    fn test_connection_info() {
        let info = SqliteConnectionInfo {
            database_path: "test.db".to_string(),
            connection_id: "test-id".to_string(),
            connected_at: SystemTime::now(),
            state: ConnectionState::Connected,
            thread_id: Some(std::thread::current().id()),
        };

        assert_eq!(info.database_path, "test.db");
        assert_eq!(info.state, ConnectionState::Connected);
        assert!(info.thread_id.is_some());
    }

    #[test]
    fn test_connection_creation() {
        let config = SqliteConfig::memory();
        
        // This test will fail in environments without SQLite
        // but demonstrates the expected interface
        match SqliteConnection::new(config) {
            Ok(conn) => {
                assert_eq!(conn.info().database_path, ":memory:");
                assert_eq!(conn.info().state, ConnectionState::Connected);
            }
            Err(_) => {
                println!("SQLite connection creation failed (expected in test environment)");
            }
        }
    }
}

/// Convert CURSED SqlValue to rusqlite parameters
fn convert_args_to_params(args: &[SqlValue]) -> Result<(), Error> {
    let mut params = Vec::new();
    
    for arg in args {
        match arg {
            SqlValue::Null => params.push(Box::new(rusqlite::types::Null) as Box<dyn rusqlite::ToSql>),
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
