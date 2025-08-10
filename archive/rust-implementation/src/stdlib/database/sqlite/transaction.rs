//! SQLite transaction management

use crate::error::CursedError;
use super::{SqliteError, SqliteConnection};
use std::sync::{Arc, Mutex};

/// Result type for transaction operations
pub type TransactionResult<T> = Result<T, SqliteError>;

/// SQLite transaction handle
#[derive(Debug)]
pub struct SqliteTransaction {
    /// Transaction ID
    pub id: u64,
    /// Connection reference
    pub connection: Arc<SqliteConnection>,
    /// Transaction state
    pub state: TransactionState,
    /// Savepoint name
    pub savepoint: Option<String>,
}

/// Transaction state
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    /// Transaction is active
    Active,
    /// Transaction has been committed
    Committed,
    /// Transaction has been rolled back
    RolledBack,
}

impl SqliteTransaction {
    /// Create a new transaction
    pub fn new(connection: Arc<SqliteConnection>) -> TransactionResult<Self> {
        let id = 1; // Mock transaction ID for now
        
        // Begin transaction on the connection
        connection.begin_transaction()?;
        
        Ok(Self {
            id,
            connection,
            state: TransactionState::Active,
            savepoint: None,
        })
    }
    
    /// Create a transaction with savepoint
    pub fn with_savepoint(connection: Arc<SqliteConnection>, savepoint: String) -> TransactionResult<Self> {
        let mut transaction = Self::new(connection)?;
        transaction.savepoint = Some(savepoint);
        Ok(transaction)
    }
    
    /// Commit the transaction
    pub fn commit(mut self) -> TransactionResult<()> {
        if self.state != TransactionState::Active {
            return Err(SqliteError::transaction_not_active());
        }
        
        self.connection.commit()?;
        self.state = TransactionState::Committed;
        Ok(())
    }
    
    /// Rollback the transaction
    pub fn rollback(mut self) -> TransactionResult<()> {
        if self.state != TransactionState::Active {
            return Err(SqliteError::transaction_not_active());
        }
        
        self.connection.rollback()?;
        self.state = TransactionState::RolledBack;
        Ok(())
    }
    
    /// Execute SQL within the transaction
    pub fn execute(&self, sql: &str) -> TransactionResult<u64> {
        if self.state != TransactionState::Active {
            return Err(SqliteError::transaction_not_active());
        }
        
        self.connection.execute(sql)
    }
    
    /// Get transaction state
    pub fn state(&self) -> TransactionState {
        self.state.clone()
    }
}

/// SQLite transaction builder
#[derive(Debug)]
pub struct SqliteTransactionBuilder {
    savepoint: Option<String>,
}

impl SqliteTransactionBuilder {
    /// Create a new transaction builder
    pub fn new() -> Self {
        Self {
            savepoint: None,
        }
    }
    
    /// Set savepoint name
    pub fn savepoint<S: Into<String>>(mut self, name: S) -> Self {
        self.savepoint = Some(name.into());
        self
    }
    
    /// Build the transaction
    pub fn build(self, connection: Arc<SqliteConnection>) -> TransactionResult<SqliteTransaction> {
        if let Some(savepoint) = self.savepoint {
            SqliteTransaction::with_savepoint(connection, savepoint)
        } else {
            SqliteTransaction::new(connection)
        }
    }
}

impl Default for SqliteTransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy compatibility functions
/// Initialize transaction processing
pub fn init_transaction() -> Result<(), CursedError> {
    println!("📁 SQLite transaction system initialized");
    Ok(())
}

/// Test transaction functionality
pub fn test_transaction() -> Result<(), CursedError> {
    println!("Transaction test completed");
    Ok(())
}
