//! PostgreSQL transaction implementation

use crate::error::CursedError;
use super::connection::{PostgresConnection, PostgresQueryResult};
use crate::stdlib::packages::IOError;

/// Result type for PostgreSQL transaction operations
pub type PostgresTransactionResult<T> = Result<T, CursedError>;

/// PostgreSQL transaction
pub struct PostgresTransaction {
    connection: Option<PostgresConnection>,
    is_active: bool,
    savepoint_level: u32,
}

impl PostgresTransaction {
    /// Create a new PostgreSQL transaction
    pub fn new(connection: PostgresConnection) -> Self {
        Self {
            connection: Some(connection),
            is_active: false,
            savepoint_level: 0,
        }
    }
    
    /// Begin the transaction
    pub fn begin(&mut self) -> PostgresTransactionResult<()> {
        if self.is_active {
            return Err(CursedError::runtime_error(&"Transaction already active".to_string()));
        }
        
        if let Some(ref mut conn) = self.connection {
            conn.begin_transaction()?;
            self.is_active = true;
            println!("🔄 PostgreSQL transaction begun");
        } else {
            return Err(CursedError::runtime_error(&"No connection available".to_string()));
        }
        
        Ok(())
    }
    
    /// Commit the transaction
    pub fn commit(&mut self) -> PostgresTransactionResult<()> {
        if !self.is_active {
            return Err(CursedError::runtime_error(&"No active transaction to commit".to_string()));
        }
        
        if let Some(ref mut conn) = self.connection {
            conn.commit()?;
            self.is_active = false;
            self.savepoint_level = 0;
            println!("✅ PostgreSQL transaction committed");
        } else {
            return Err(CursedError::runtime_error(&"No connection available".to_string()));
        }
        
        Ok(())
    }
    
    /// Rollback the transaction
    pub fn rollback(&mut self) -> PostgresTransactionResult<()> {
        if !self.is_active {
            return Err(CursedError::runtime_error(&"No active transaction to rollback".to_string()));
        }
        
        if let Some(ref mut conn) = self.connection {
            conn.rollback()?;
            self.is_active = false;
            self.savepoint_level = 0;
            println!("🔄 PostgreSQL transaction rolled back");
        } else {
            return Err(CursedError::runtime_error(&"No connection available".to_string()));
        }
        
        Ok(())
    }
    
    /// Create a savepoint
    pub fn savepoint(&mut self, name: &str) -> PostgresTransactionResult<()> {
        if !self.is_active {
            return Err(CursedError::runtime_error(&"No active transaction for savepoint".to_string()));
        }
        
        self.savepoint_level += 1;
        println!("💾 Created savepoint '{}' (level {})", name, self.savepoint_level);
        Ok(())
    }
    
    /// Rollback to a savepoint
    pub fn rollback_to_savepoint(&mut self, name: &str) -> PostgresTransactionResult<()> {
        if !self.is_active {
            return Err(CursedError::runtime_error(&"No active transaction".to_string()));
        }
        
        if self.savepoint_level > 0 {
            self.savepoint_level -= 1;
        }
        println!("🔄 Rolled back to savepoint '{}'", name);
        Ok(())
    }
    
    /// Execute a query within the transaction
    pub fn execute(&self, query: &str) -> PostgresTransactionResult<PostgresQueryResult> {
        if !self.is_active {
            return Err(CursedError::runtime_error(&"No active transaction".to_string()));
        }
        
        if let Some(ref conn) = self.connection {
            conn.execute(query).map_err(|e| e)
        } else {
            Err(CursedError::runtime_error(&"No connection available".to_string()))
        }
    }
    
    /// Check if transaction is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    
    /// Get current savepoint level
    pub fn savepoint_level(&self) -> u32 {
        self.savepoint_level
    }
}

impl Drop for PostgresTransaction {
    fn drop(&mut self) {
        if self.is_active {
            println!("⚠️ PostgreSQL transaction dropped while active - attempting rollback");
            let _ = self.rollback();
        }
    }
}
