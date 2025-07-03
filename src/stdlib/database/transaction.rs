//! Database transaction management

use crate::error::CursedError;
use super::driver::{DriverConn, DriverTx};
use crate::stdlib::packages::IOError;

/// Result type for transaction operations
pub type TransactionResult<T> = Result<T, CursedError>;

/// Database transaction wrapper
pub struct Tx {
    transaction: Option<Box<dyn DriverTx>>,
    is_active: bool,
}

/// Transaction manager
pub struct TransactionManager {
    connection: Option<Box<dyn DriverConn>>,
    current_transaction: Option<Tx>,
}

impl Tx {
    /// Create a new transaction
    pub fn new(transaction: Box<dyn DriverTx>) -> Self {
        Self {
            transaction: Some(transaction),
            is_active: true,
        }
    }
    
    /// Commit the transaction
    pub fn commit(mut self) -> TransactionResult<()> {
        if let Some(ref tx) = self.transaction {
            tx.commit()?;
            self.is_active = false;
            println!("✅ Transaction committed");
        }
        Ok(())
    }
    
    /// Rollback the transaction
    pub fn rollback(mut self) -> TransactionResult<()> {
        if let Some(ref tx) = self.transaction {
            tx.rollback()?;
            self.is_active = false;
            println!("🔄 Transaction rolled back");
        }
        Ok(())
    }
    
    /// Execute a query within the transaction  
    pub fn execute(&self, query: &str) -> TransactionResult<Box<dyn super::driver::DatabaseResult>> {
        if let Some(ref tx) = self.transaction {
            tx.execute(query).map_err(|e| e)
        } else {
            Err(CursedError::runtime_error(&"Transaction not available"))
        }
    }
    
    /// Check if transaction is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

impl TransactionManager {
    /// Create a new transaction manager
    pub fn new() -> Self {
        Self {
            connection: None,
            current_transaction: None,
        }
    }
    
    /// Set the database connection
    pub fn with_connection(mut self, connection: Box<dyn DriverConn>) -> Self {
        self.connection = Some(connection);
        self
    }
    
    /// Begin a new transaction
    pub fn begin(&mut self) -> TransactionResult<&mut Tx> {
        if self.current_transaction.is_some() {
            return Err(CursedError::runtime_error(&"Transaction already active"));
        }
        
        if let Some(ref conn) = self.connection {
            let tx = conn.begin()?;
            self.current_transaction = Some(Tx::new(tx));
            println!("🔄 Transaction begun");
            Ok(self.current_transaction.as_mut().unwrap())
        } else {
            Err(CursedError::runtime_error(&"No connection available"))
        }
    }
    
    /// Get the current transaction
    pub fn current(&mut self) -> Option<&mut Tx> {
        self.current_transaction.as_mut()
    }
    
    /// Check if there's an active transaction
    pub fn has_active_transaction(&self) -> bool {
        self.current_transaction.as_ref().map_or(false, |tx| tx.is_active())
    }
}

impl Default for TransactionManager {
    fn default() -> Self {
        Self::new()
    }
}
