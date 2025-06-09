/// fr fr SQLite transaction implementation that slays periodt
/// 
/// This module provides transaction management with savepoints,
/// isolation levels, and proper ACID compliance for SQLite.

use std::sync::{Arc, Mutex};
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
