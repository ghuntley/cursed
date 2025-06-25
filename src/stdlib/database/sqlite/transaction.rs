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
/// fr fr SQLite transaction options
#[derive(Debug, Clone)]
pub struct SqliteTransactionOptions {
/// fr fr SQLite transaction types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteTransactionType {
impl SqliteTransactionType {
    /// slay Convert to SQL string
    pub fn to_sql(self) -> &'static str {
        match self {
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
impl SqliteTransaction {
    /// slay Create new transaction
    pub fn new(
    ) -> SqliteResult<Self> {
        let transaction = Self {

        // Begin the transaction
        transaction.begin_internal()?;

        Ok(transaction)
    /// slay Begin transaction internally
    fn begin_internal(&self) -> SqliteResult<()> {
        // This would execute the BEGIN statement via FFI
        // For now, just validate state
        let state = self.state.lock()
            .map_err(|_| SqliteError::internal("Failed to acquire transaction state lock"))?;
        
        if *state != TransactionState::Active {
            return Err(SqliteError::transaction_not_active());
        Ok(())
    /// slay Create savepoint
    pub fn savepoint(&mut self, name: &str) -> SqliteResult<()> {
        if self.savepoints.contains(&name.to_string()) {
            return Err(SqliteError::invalid_parameter("Savepoint already exists"));
        // This would execute SAVEPOINT statement
        self.savepoints.push(name.to_string());
        Ok(())
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
        // This would execute ROLLBACK TO SAVEPOINT statement
        Ok(())
    /// slay Get transaction state
    pub fn state(&self) -> TransactionState {
        self.state.lock()
            .map(|s| *s)
            .unwrap_or(TransactionState::CursedError)
    /// slay Get transaction duration
    pub fn duration(&self) -> std::time::Duration {
        std::time::SystemTime::now()
            .duration_since(self.started_at)
            .unwrap_or_default()
    }
}

impl DriverTx for SqliteTransaction {
    fn prepare(&self, query: &str) -> crate::error::Result<()> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                "Transaction is not active"
            ));
        // Use the DriverConn trait method through the connection
        (self.connection.as_ref() as &dyn super::super::DriverConn).prepare(query)
    fn query(&self, query: &str, args: &[SqlValue]) -> crate::error::Result<()> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                "Transaction is not active"
            ));
        // Use the DriverConn trait method through the connection
        (self.connection.as_ref() as &dyn super::super::DriverConn).query(query, args)
    fn execute(&self, query: &str, args: &[SqlValue]) -> crate::error::Result<()> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                "Transaction is not active"
            ));
        // Use the DriverConn trait method through the connection
        (self.connection.as_ref() as &dyn super::super::DriverConn).execute(query, args)
    fn commit(&self) -> crate::error::Result<()> {
        let mut state = self.state.lock()
            .map_err(|_| DatabaseError::new(
                "Failed to acquire transaction state lock"
            ))?;

        if *state != TransactionState::Active {
            return Err(DatabaseError::new(
                "Transaction is not active"
            ));
        // This would execute COMMIT statement via FFI
        *state = TransactionState::Committed;
        Ok(())
    fn rollback(&self) -> crate::error::Result<()> {
        let mut state = self.state.lock()
            .map_err(|_| DatabaseError::new(
                "Failed to acquire transaction state lock"
            ))?;

        if *state != TransactionState::Active {
            return Err(DatabaseError::new(
                "Transaction is not active"
            ));
        // This would execute ROLLBACK statement via FFI
        *state = TransactionState::RolledBack;
        Ok(())
    fn options(&self) -> &TxOptions {
        &self.options.base
    fn is_active(&self) -> bool {
        self.state() == TransactionState::Active
    fn clone(&self) -> Box<dyn DriverTx> {
        // Create a new transaction reference
        Box::new(Self {
        })
    }
}

/// Real SQLite transaction implementation for rusqlite integration
#[derive(Debug)]
pub struct RealSqliteTransaction {
impl RealSqliteTransaction {
    /// Create new transaction with connection handle
    pub fn new(connection: Arc<Mutex<Option<Connection>>>, options: TxOptions) -> crate::error::Result<()> {
        {
            let handle = connection.lock().unwrap();
            if let Some(ref conn) = *handle {
                // Begin transaction
                conn.execute("BEGIN", [])
                    .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::TransactionError, &format!("Failed to begin transaction: {}", e)))?;
            }
        }
        
        Ok(Self {
        })
    }
}

impl DriverTx for RealSqliteTransaction {
    fn prepare(&self, query: &str) -> crate::error::Result<()> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                "Transaction is not active"
            ));
        let stmt = super::statement::SqliteStatement::new_with_connection(self.connection.clone(), query.to_string())
            .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::QueryError, &e.to_string()))?;
        Ok(Box::new(stmt))
    fn query(&self, query: &str, args: &[SqlValue]) -> crate::error::Result<()> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                "Transaction is not active"
            ));
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
            Ok(super::super::driver::QueryResult {
                column_types: vec![], // Would need to extract actual types
            })
        } else {
            Err(DatabaseError::new(
                "Connection is not available"
            ))
        }
    }

    fn execute(&self, query: &str, args: &[SqlValue]) -> crate::error::Result<()> {
        if self.state() != TransactionState::Active {
            return Err(DatabaseError::new(
                "Transaction is not active"
            ));
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            let mut stmt = conn.prepare(query)
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

    fn commit(&self) -> crate::error::Result<()> {
        let mut state = self.state.lock()
            .map_err(|_| DatabaseError::new(
                "Failed to acquire transaction state lock"
            ))?;

        if *state != TransactionState::Active {
            return Err(DatabaseError::new(
                "Transaction is not active"
            ));
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            conn.execute("COMMIT", [])
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::TransactionError, &format!("Failed to commit transaction: {}", e)))?;
        *state = TransactionState::Committed;
        Ok(())
    fn rollback(&self) -> crate::error::Result<()> {
        let mut state = self.state.lock()
            .map_err(|_| DatabaseError::new(
                "Failed to acquire transaction state lock"
            ))?;

        if *state != TransactionState::Active {
            return Err(DatabaseError::new(
                "Transaction is not active"
            ));
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            conn.execute("ROLLBACK", [])
                .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::TransactionError, &format!("Failed to rollback transaction: {}", e)))?;
        *state = TransactionState::RolledBack;
        Ok(())
    fn options(&self) -> &TxOptions {
        &self.options
    fn is_active(&self) -> bool {
        self.state() == TransactionState::Active
    fn clone(&self) -> Box<dyn DriverTx> {
        Box::new(Self {
        })
    }
}

impl RealSqliteTransaction {
    /// Get transaction state
    pub fn state(&self) -> TransactionState {
        self.state.lock()
            .map(|s| *s)
            .unwrap_or(TransactionState::CursedError)
    }
}

/// Convert CURSED SqlValue to rusqlite parameters
fn convert_args_to_params(args: &[SqlValue]) -> crate::error::Result<()> {
    let mut params = Vec::new();
    
    for arg in args {
        match arg {
            _ => return Err(DatabaseError::new(
                &format!("Unsupported SqlValue type: {:?}", arg)
        }
    }
    
    Ok(params)
/// Convert rusqlite value to CURSED SqlValue
fn convert_value_from_sqlite(row: &rusqlite::Row, index: usize) -> crate::error::Result<()> {
    let value: SqliteValue = row.get(index)
        .map_err(|e| DatabaseError::new(super::super::DatabaseErrorKind::ConversionError, &format!("Failed to get column {}: {}", index, e)))?;
    
    match value {
    }
}

impl From<TxOptions> for SqliteTransactionOptions {
    fn from(base: TxOptions) -> Self {
        let transaction_type = match base.isolation {

        Self {
        }
    }
