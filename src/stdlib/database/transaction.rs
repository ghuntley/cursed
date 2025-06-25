/// fr fr Transaction management for SQLSlay
/// 
/// This module provides transaction coordination and management capabilities.

use std::sync::{Arc, Mutex};
use super::{DatabaseError, DatabaseErrorKind, TxOptions, DriverTx};
use crate::error::CursedError;

/// fr fr Transaction manager for coordinating database transactions
#[derive(Debug)]
pub struct TransactionManager {
    /// fr fr Active transaction
    active_tx: Arc<Mutex<Option<Box<dyn DriverTx>>>>,
}

impl TransactionManager {
    /// slay Create a new transaction manager
    pub fn new() -> Self {
        Self {
            active_tx: Arc::new(Mutex::new(None)),
        }
    }

    /// slay Begin a new transaction
    pub fn begin(&self, tx: Box<dyn DriverTx>) -> crate::error::Result<()> {
        let mut active = self.active_tx.lock().map_err(|_| {
            DatabaseError::transaction_error("Failed to acquire transaction lock")
        })?;

        if active.is_some() {
            return Err(DatabaseError::transaction_error("Transaction already active"));
        }

        *active = Some(tx);
        Ok(())
    }

    /// slay Commit the active transaction
    pub fn commit(&self) -> crate::error::Result<()> {
        let mut active = self.active_tx.lock().map_err(|_| {
            DatabaseError::transaction_error("Failed to acquire transaction lock")
        })?;

        if let Some(tx) = active.take() {
            tx.commit()
        } else {
            Err(DatabaseError::transaction_error("No active transaction"))
        }
    }

    /// slay Rollback the active transaction
    pub fn rollback(&self) -> crate::error::Result<()> {
        let mut active = self.active_tx.lock().map_err(|_| {
            DatabaseError::transaction_error("Failed to acquire transaction lock")
        })?;

        if let Some(tx) = active.take() {
            tx.rollback()
        } else {
            Err(DatabaseError::transaction_error("No active transaction"))
        }
    }
}

/// fr fr Transaction wrapper type (re-exported from core)
pub use super::core::Tx;
