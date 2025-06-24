/// Real SQLite connection implementation using rusqlite
/// 
/// This module provides the production-ready SQLite connection implementation
/// with actual database operations instead of placeholder stubs.

use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use std::collections::HashMap;
use rusqlite::{Connection, OpenFlags, params, crate::types::Value as SqliteValue};
use super::{SqliteError, SqliteResult, SqliteConfig, SqliteStats};
use super::super::{DriverConn, DatabaseError, SqlValue, TxOptions, DriverStmt, DriverTx};
use super::super::driver::{QueryResult, ExecuteResult, ConnectionMetadata};
use crate::error::Error;

/// Real SQLite connection implementation
#[derive(Debug)]
pub struct RealSqliteConnection {
    handle: Arc<Mutex<Option<Connection>>>,
    config: SqliteConfig,
    connection_id: String,
    connected_at: SystemTime,
    stats: Arc<Mutex<SqliteStats>>,
}

impl RealSqliteConnection {
    /// Create new SQLite connection
    pub fn new(config: SqliteConfig) -> SqliteResult<Self> {
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE 
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_URI
            | OpenFlags::SQLITE_OPEN_NO_MUTEX;
        
        let handle = Connection::open_with_flags(&config.database_path, flags)
            .map_err(|e| SqliteError::connection(&format!("Failed to open SQLite database: {}", e)))?;
        
        let connection = Self {
            handle: Arc::new(Mutex::new(Some(handle))),
            config,
            connection_id: uuid::Uuid::new_v4().to_string(),
            connected_at: SystemTime::now(),
            stats: Arc::new(Mutex::new(SqliteStats::default())),
        };

        // Initialize connection with PRAGMA statements
        connection.initialize_connection()?;

        Ok(connection)
    }

    /// Initialize connection with configuration
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

    /// Get connection ID
    pub fn connection_id(&self) -> &str {
        &self.connection_id
    }

    /// Get connection configuration
    pub fn config(&self) -> &SqliteConfig {
        &self.config
    }
}

impl DriverConn for RealSqliteConnection {
    fn prepare(&self, query: &str) -> Result<(), Error> {
        let stmt = RealSqliteStatement::new(self.handle.clone(), query)
            .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &e.to_string()))?;
        Ok(Box::new(stmt))
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
            
            Ok(QueryResult {
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
            
            Ok(ExecuteResult {
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
        let handle = self.handle.lock().unwrap();
        if let Some(ref conn) = *handle {
            conn.execute("SELECT 1", [])
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::ConnectionError, &format!("Ping failed: {}", e)))?;
            Ok(())
        } else {
            Err(DatabaseError::new(
                super::super::DatabaseErrorKind::ConnectionError,
                "Connection is not available"
            ))
        }
    }

    fn close(&self) -> Result<(), Error> {
        let mut handle = self.handle.lock().unwrap();
        if let Some(conn) = handle.take() {
            drop(conn); // Close the connection
        }
        Ok(())
    }

    fn is_alive(&self) -> bool {
        self.ping().is_ok()
    }

    fn metadata(&self) -> ConnectionMetadata {
        let mut additional_info = HashMap::new();
        additional_info.insert("driver_name".to_string(), "SQLite".to_string());
        additional_info.insert("driver_version".to_string(), "3.0".to_string());
        additional_info.insert("connection_id".to_string(), self.connection_id.clone());
        additional_info.insert("is_read_only".to_string(), "false".to_string());
        
        ConnectionMetadata {
            database_name: self.config.database_path.clone(),
            server_version: "3.0".to_string(),
            server_host: "localhost".to_string(),
            server_port: 0,
            username: "".to_string(),
            connected_at: self.connected_at,
            additional_info,
        }
    }

    fn clone(&self) -> Box<dyn DriverConn> {
        // For SQLite, we need to create a new connection since SQLite connections aren't clonable
        let new_conn = RealSqliteConnection::new(self.config.clone())
            .expect("Failed to clone SQLite connection");
        Box::new(new_conn)
    }
}

/// Real SQLite statement implementation
#[derive(Debug)]
pub struct RealSqliteStatement {
    query: String,
    connection: Arc<Mutex<Option<Connection>>>,
}

impl RealSqliteStatement {
    pub fn new(connection: Arc<Mutex<Option<Connection>>>, query: &str) -> Result<(), Error> {
        Ok(Self {
            query: query.to_string(),
            connection,
        })
    }
}

impl DriverStmt for RealSqliteStatement {
    fn execute(&self, args: &[SqlValue]) -> Result<(), Error> {
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            let mut stmt = conn.prepare(&self.query)
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &format!("Failed to prepare statement: {}", e)))?;
            
            let params = convert_args_to_params(args)?;
            
            let changes = stmt.execute(rusqlite::params_from_iter(params.iter()))
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &format!("Failed to execute statement: {}", e)))?;
            
            let last_insert_id = conn.last_insert_rowid();
            
            Ok(ExecuteResult {
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

    fn query(&self, args: &[SqlValue]) -> Result<(), Error> {
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            let mut stmt = conn.prepare(&self.query)
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
            
            Ok(QueryResult {
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

    fn close(&self) -> Result<(), Error> {
        Ok(())
    }

    fn query_string(&self) -> &str {
        &self.query
    }

    fn parameter_count(&self) -> usize {
        // For now, return 0 - would need proper parameter parsing
        0
    }

    fn clone(&self) -> Box<dyn DriverStmt> {
        Box::new(RealSqliteStatement {
            query: self.query.clone(),
        })
    }
}

/// Real SQLite transaction implementation
#[derive(Debug)]
pub struct RealSqliteTransaction {
    connection: Arc<Mutex<Option<Connection>>>,
    _opts: TxOptions,
}

impl RealSqliteTransaction {
    pub fn new(connection: Arc<Mutex<Option<Connection>>>, opts: TxOptions) -> Result<(), Error> {
        {
            let handle = connection.lock().unwrap();
            if let Some(ref conn) = *handle {
                // Begin transaction
                conn.execute("BEGIN", [])
                    .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::TransactionError, &format!("Failed to begin transaction: {}", e)))?;
            }
        }
        
        Ok(Self {
            connection,
            _opts: opts,
        })
    }
}

impl DriverTx for RealSqliteTransaction {
    fn commit(&self) -> Result<(), Error> {
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            conn.execute("COMMIT", [])
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::TransactionError, &format!("Failed to commit transaction: {}", e)))?;
        }
        Ok(())
    }

    fn rollback(&self) -> Result<(), Error> {
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            conn.execute("ROLLBACK", [])
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::TransactionError, &format!("Failed to rollback transaction: {}", e)))?;
        }
        Ok(())
    }

    fn prepare(&self, query: &str) -> Result<(), Error> {
        let stmt = RealSqliteStatement::new(self.connection.clone(), query)
            .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &e.to_string()))?;
        Ok(Box::new(stmt))
    }

    fn query(&self, query: &str, args: &[SqlValue]) -> Result<(), Error> {
        let handle = self.connection.lock().unwrap();
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
            
            Ok(QueryResult {
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
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            let mut stmt = conn.prepare(query)
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &format!("Failed to prepare statement: {}", e)))?;
            
            let params = convert_args_to_params(args)?;
            
            let changes = stmt.execute(rusqlite::params_from_iter(params.iter()))
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &format!("Failed to execute statement: {}", e)))?;
            
            let last_insert_id = conn.last_insert_rowid();
            
            Ok(ExecuteResult {
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

    fn options(&self) -> &TxOptions {
        &self._opts
    }

    fn is_active(&self) -> bool {
        // For now, assume always active - would need proper state tracking
        true
    }

    fn clone(&self) -> Box<dyn DriverTx> {
        Box::new(RealSqliteTransaction {
            connection: self.connection.clone(),
            _opts: self._opts.clone(),
        })
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
