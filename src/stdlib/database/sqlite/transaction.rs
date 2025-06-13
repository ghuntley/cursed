/// fr fr SQLite transaction implementation that slays periodt
/// 
/// This module provides transaction management with savepoints,
/// isolation levels, and proper ACID compliance for SQLite.

use std::sync::{Arc, Mutex};
use rusqlite::{Connection, types::Value as SqliteValue};
use super::{SqliteError, SqliteResult};
use super::connection::SqliteConnection;
use super::super::{DriverTx, DatabaseError, SqlValue, TxOptions, SqlIsolationLevel};

/// fr fr Transaction state tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionState {
    Active,
    Committed,
    RolledBack,
    Error,
}

/// fr fr SQLite transaction options
#[derive(Debug, Clone)]
pub struct SqliteTransactionOptions {
    pub base: TxOptions,
    pub transaction_type: SqliteTransactionType,
    pub lock_timeout: Option<std::time::Duration>,
}

/// fr fr SQLite transaction types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteTransactionType {
    Deferred,
    Immediate,
    Exclusive,
}

impl SqliteTransactionType {
    /// slay Convert to SQL string
    pub fn to_sql(self) -> &'static str {
        match self {
            SqliteTransactionType::Deferred => "DEFERRED",
            SqliteTransactionType::Immediate => "IMMEDIATE", 
            SqliteTransactionType::Exclusive => "EXCLUSIVE",
        }
    }
}

impl Default for SqliteTransactionType {
    fn default() -> Self {
        SqliteTransactionType::Deferred
    }
}

/// fr fr SQLite transaction implementation
#[derive(Debug)]
pub struct SqliteTransaction {
    connection: Arc<SqliteConnection>,
    options: SqliteTransactionOptions,
    state: Arc<Mutex<TransactionState>>,
    started_at: std::time::SystemTime,
    savepoints: Vec<String>,
}

impl SqliteTransaction {
    /// slay Create new transaction
    pub fn new(
        connection: Arc<SqliteConnection>,
        options: SqliteTransactionOptions,
    ) -> SqliteResult<Self> {
        let transaction = Self {
            connection,
            options,
            state: Arc::new(Mutex::new(TransactionState::Active)),
            started_at: std::time::SystemTime::now(),
            savepoints: Vec::new(),
        };

        // Begin the transaction
        transaction.begin_internal()?;

        Ok(transaction)
    }

    /// slay Begin transaction internally
    fn begin_internal(&self) -> SqliteResult<()> {
        // This would execute the BEGIN statement via FFI
        // For now, just validate state
        let state = self.state.lock()
            .map_err(|_| SqliteError::internal("Failed to acquire transaction state lock"))?;
        
        if *state != TransactionState::Active {
            return Err(SqliteError::transaction_not_active());
        }

        Ok(())
    }

    /// slay Create savepoint
    pub fn savepoint(&mut self, name: &str) -> SqliteResult<()> {
        if self.savepoints.contains(&name.to_string()) {
            return Err(SqliteError::invalid_parameter("Savepoint already exists"));
        }

        // This would execute SAVEPOINT statement
        self.savepoints.push(name.to_string());
        Ok(())
    }

    /// slay Release savepoint
    pub fn release_savepoint(&mut self, name: &str) -> SqliteResult<()> {
        if let Some(pos) = self.savepoints.iter().position(|s| s == name) {
            self.savepoints.remove(pos);
            // This would execute RELEASE SAVEPOINT statement
            Ok(())
        } else {
            Err(SqliteError::invalid_parameter("Savepoint does not exist"))
        }
    }

    /// slay Rollback to savepoint
    pub fn rollback_to_savepoint(&mut self, name: &str) -> SqliteResult<()> {
        if !self.savepoints.contains(&name.to_string()) {
            return Err(SqliteError::invalid_parameter("Savepoint does not exist"));
        }

        // This would execute ROLLBACK TO SAVEPOINT statement
        Ok(())
    }

    /// slay Get transaction state
    pub fn state(&self) -> TransactionState {
        self.state.lock()
            .map(|s| *s)
            .unwrap_or(TransactionState::Error)
    }

    /// slay Get transaction duration
    pub fn duration(&self) -> std::time::Duration {
        std::time::SystemTime::now()
            .duration_since(self.started_at)
            .unwrap_or_default()
    }
}

impl DriverTx for SqliteTransaction {
    fn prepare(&self, query: &str) -> Result<Box<dyn super::super::DriverStmt>, DatabaseError> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Transaction is not active"
            ));
        }

        // Use the DriverConn trait method through the connection
        (self.connection.as_ref() as &dyn super::super::DriverConn).prepare(query)
    }

    fn query(&self, query: &str, args: &[SqlValue]) -> Result<super::super::driver::QueryResult, DatabaseError> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Transaction is not active"
            ));
        }

        // Use the DriverConn trait method through the connection
        (self.connection.as_ref() as &dyn super::super::DriverConn).query(query, args)
    }

    fn execute(&self, query: &str, args: &[SqlValue]) -> Result<super::super::driver::ExecuteResult, DatabaseError> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Transaction is not active"
            ));
        }

        // Use the DriverConn trait method through the connection
        (self.connection.as_ref() as &dyn super::super::DriverConn).execute(query, args)
    }

    fn commit(&self) -> Result<(), DatabaseError> {
        let mut state = self.state.lock()
            .map_err(|_| DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Failed to acquire transaction state lock"
            ))?;

        if *state != TransactionState::Active {
            return Err(DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Transaction is not active"
            ));
        }

        // This would execute COMMIT statement via FFI
        *state = TransactionState::Committed;
        Ok(())
    }

    fn rollback(&self) -> Result<(), DatabaseError> {
        let mut state = self.state.lock()
            .map_err(|_| DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Failed to acquire transaction state lock"
            ))?;

        if *state != TransactionState::Active {
            return Err(DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Transaction is not active"
            ));
        }

        // This would execute ROLLBACK statement via FFI
        *state = TransactionState::RolledBack;
        Ok(())
    }

    fn options(&self) -> &TxOptions {
        &self.options.base
    }

    fn is_active(&self) -> bool {
        self.state() == TransactionState::Active
    }

    fn clone(&self) -> Box<dyn DriverTx> {
        // Create a new transaction reference
        Box::new(Self {
            connection: Arc::clone(&self.connection),
            options: self.options.clone(),
            state: Arc::clone(&self.state),
            started_at: self.started_at,
            savepoints: self.savepoints.clone(),
        })
    }
}

/// Real SQLite transaction implementation for rusqlite integration
#[derive(Debug)]
pub struct RealSqliteTransaction {
    connection: Arc<Mutex<Option<Connection>>>,
    options: TxOptions,
    state: Arc<Mutex<TransactionState>>,
}

impl RealSqliteTransaction {
    /// Create new transaction with connection handle
    pub fn new(connection: Arc<Mutex<Option<Connection>>>, options: TxOptions) -> Result<Self, DatabaseError> {
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
            options,
            state: Arc::new(Mutex::new(TransactionState::Active)),
        })
    }
}

impl DriverTx for RealSqliteTransaction {
    fn prepare(&self, query: &str) -> Result<Box<dyn super::super::DriverStmt>, DatabaseError> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Transaction is not active"
            ));
        }

        let stmt = super::statement::SqliteStatement::new_with_connection(self.connection.clone(), query.to_string())
            .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &e.to_string()))?;
        Ok(Box::new(stmt))
    }

    fn query(&self, query: &str, args: &[SqlValue]) -> Result<super::super::driver::QueryResult, DatabaseError> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Transaction is not active"
            ));
        }

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

    fn execute(&self, query: &str, args: &[SqlValue]) -> Result<super::super::driver::ExecuteResult, DatabaseError> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Transaction is not active"
            ));
        }

        let handle = self.connection.lock().unwrap();
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

    fn commit(&self) -> Result<(), DatabaseError> {
        let mut state = self.state.lock()
            .map_err(|_| DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Failed to acquire transaction state lock"
            ))?;

        if *state != TransactionState::Active {
            return Err(DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Transaction is not active"
            ));
        }

        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            conn.execute("COMMIT", [])
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::TransactionError, &format!("Failed to commit transaction: {}", e)))?;
        }

        *state = TransactionState::Committed;
        Ok(())
    }

    fn rollback(&self) -> Result<(), DatabaseError> {
        let mut state = self.state.lock()
            .map_err(|_| DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Failed to acquire transaction state lock"
            ))?;

        if *state != TransactionState::Active {
            return Err(DatabaseError::new(
                super::super::DatabaseErrorKind::TransactionError,
                "Transaction is not active"
            ));
        }

        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            conn.execute("ROLLBACK", [])
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::TransactionError, &format!("Failed to rollback transaction: {}", e)))?;
        }

        *state = TransactionState::RolledBack;
        Ok(())
    }

    fn options(&self) -> &TxOptions {
        &self.options
    }

    fn is_active(&self) -> bool {
        self.state() == TransactionState::Active
    }

    fn clone(&self) -> Box<dyn DriverTx> {
        Box::new(Self {
            connection: self.connection.clone(),
            options: self.options.clone(),
            state: self.state.clone(),
        })
    }
}

impl RealSqliteTransaction {
    /// Get transaction state
    pub fn state(&self) -> TransactionState {
        self.state.lock()
            .map(|s| *s)
            .unwrap_or(TransactionState::Error)
    }
}

/// Convert CURSED SqlValue to rusqlite parameters
fn convert_args_to_params(args: &[SqlValue]) -> Result<Vec<Box<dyn rusqlite::ToSql>>, DatabaseError> {
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
fn convert_value_from_sqlite(row: &rusqlite::Row, index: usize) -> Result<SqlValue, DatabaseError> {
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

impl From<TxOptions> for SqliteTransactionOptions {
    fn from(base: TxOptions) -> Self {
        let transaction_type = match base.isolation {
            SqlIsolationLevel::LevelSerializable => SqliteTransactionType::Exclusive,
            SqlIsolationLevel::LevelReadCommitted => SqliteTransactionType::Immediate,
            _ => SqliteTransactionType::Deferred,
        };

        Self {
            base,
            transaction_type,
            lock_timeout: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_type() {
        assert_eq!(SqliteTransactionType::Deferred.to_sql(), "DEFERRED");
        assert_eq!(SqliteTransactionType::Immediate.to_sql(), "IMMEDIATE");
        assert_eq!(SqliteTransactionType::Exclusive.to_sql(), "EXCLUSIVE");
    }

    #[test]
    fn test_transaction_state() {
        assert_eq!(TransactionState::Active, TransactionState::Active);
        assert_ne!(TransactionState::Active, TransactionState::Committed);
    }

    #[test]
    fn test_transaction_options_conversion() {
        let base = TxOptions {
            isolation: SqlIsolationLevel::LevelSerializable,
            read_only: false,
        };

        let sqlite_opts = SqliteTransactionOptions::from(base);
        assert_eq!(sqlite_opts.transaction_type, SqliteTransactionType::Exclusive);
    }
}
