/// PostgreSQL Transaction Implementation
/// 
/// Provides comprehensive transaction management for PostgreSQL including
/// commit, rollback, savepoints, and proper isolation level handling.

use std::sync::Arc;
// use tokio_postgres::Transaction; // Disabled - tokio causes E0753 errors
use super::connection::Transaction;
// use crate::stdlib::database::{
    DriverTx, DriverStmt, SqlValue, SqlIsolationLevel, DatabaseError,
    TxOptions, IsolationLevel,
    driver::{QueryResult, ExecuteResult}
};
use crate::error::CursedError;
use super::error::{PostgresError, PostgresErrorKind, PostgresResult};
use super::types::{map_postgres_value, prepare_parameters, extract_column_info};
use super::connection::ConnectionStats;

/// PostgreSQL transaction wrapper
#[derive(Debug)]
pub struct PostgresTransaction<'a> {
    /// Underlying tokio-postgres transaction
    transaction: Option<Transaction<'a>>,
    /// Transaction state tracking
    state: Arc<std::sync::Mutex<TransactionState>>,
    /// Connection transaction flag
    in_transaction: Arc<std::sync::Mutex<bool>>,
    /// Connection statistics reference
    connection_stats: Arc<std::sync::Mutex<ConnectionStats>>,
    /// Transaction statistics
    stats: TransactionStats,
    /// Savepoint counter for nested transactions
    savepoint_counter: u32,
}

/// Transaction state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionState {
    Active,
    Committed,
    RolledBack,
    Failed,
}

/// Transaction execution statistics
#[derive(Debug, Clone, Default)]
pub struct TransactionStats {
    pub statements_executed: u64,
    pub rows_affected: u64,
    pub rows_returned: u64,
    pub savepoints_created: u32,
    pub savepoints_released: u32,
    pub savepoints_rolled_back: u32,
    pub started_at: std::time::SystemTime,
    pub duration: Option<std::time::Duration>,
}

impl<'a> PostgresTransaction<'a> {
    /// Create new transaction wrapper
    pub fn new(
        transaction: Transaction<'a>,
        in_transaction: Arc<std::sync::Mutex<bool>>,
        connection_stats: Arc<std::sync::Mutex<ConnectionStats>>,
    ) -> Self {
        Self {
            transaction: Some(transaction),
            state: Arc::new(std::sync::Mutex::new(TransactionState::Active)),
            in_transaction,
            connection_stats,
            stats: TransactionStats {
                started_at: std::time::SystemTime::now(),
                ..Default::default()
            },
            savepoint_counter: 0,
        }
    }

    /// Execute query within transaction
    pub async fn query(&mut self, sql: &str, args: &[SqlValue]) -> PostgresResult<QueryResult> {
        let transaction = self.get_active_transaction()?;
        
        let params = prepare_parameters(args)?;
        let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = 
            params.iter().map(|p| p.as_ref()).collect();
        
        let rows = transaction
            .query(sql, &param_refs)
            .await
            .map_err(|e| {
                self.set_state(TransactionState::Failed);
                PostgresError::from(e)
            })?;

        // Convert rows to CURSED format
        let mut result_rows = Vec::new();
        let mut columns = Vec::new();
        
        if !rows.is_empty() {
            columns = extract_column_info(&rows[0]);
            
            for row in &rows {
                let mut row_values = Vec::new();
                for (i, column) in row.columns().iter().enumerate() {
                    let value = map_postgres_value(column.type_(), row, i)?;
                    row_values.push(value);
                }
                result_rows.push(row_values);
            }
        }

        self.stats.statements_executed += 1;
        self.stats.rows_returned += result_rows.len() as u64;

        Ok(QueryResult {
            rows: result_rows,
            columns: columns.into_iter().map(|c| c.name).collect(),
            rows_affected: 0, // Not available for SELECT queries
        })
    }

    /// Execute statement within transaction
    pub async fn execute(&mut self, sql: &str, args: &[SqlValue]) -> PostgresResult<ExecuteResult> {
        let transaction = self.get_active_transaction()?;
        
        let params = prepare_parameters(args)?;
        let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = 
            params.iter().map(|p| p.as_ref()).collect();
        
        let rows_affected = transaction
            .execute(sql, &param_refs)
            .await
            .map_err(|e| {
                self.set_state(TransactionState::Failed);
                PostgresError::from(e)
            })?;

        self.stats.statements_executed += 1;
        self.stats.rows_affected += rows_affected;

        Ok(ExecuteResult {
            rows_affected: rows_affected as i64,
            last_insert_id: None, // PostgreSQL uses RETURNING clause
        })
    }

    /// Commit the transaction
    pub async fn commit(mut self) -> PostgresResult<()> {
        if let Some(transaction) = self.transaction.take() {
            transaction.commit().await.map_err(|e| {
                self.set_state(TransactionState::Failed);
                PostgresError::from(e)
            })?;
            
            self.set_state(TransactionState::Committed);
            self.finalize_transaction();
            Ok(())
        } else {
            Err(PostgresError::new(
                PostgresErrorKind::TransactionError,
                "Transaction already completed",
            ))
        }
    }

    /// Rollback the transaction
    pub async fn rollback(mut self) -> PostgresResult<()> {
        if let Some(transaction) = self.transaction.take() {
            transaction.rollback().await.map_err(|e| {
                self.set_state(TransactionState::Failed);
                PostgresError::from(e)
            })?;
            
            self.set_state(TransactionState::RolledBack);
            self.finalize_transaction();
            Ok(())
        } else {
            Err(PostgresError::new(
                PostgresErrorKind::TransactionError,
                "Transaction already completed",
            ))
        }
    }

    /// Create savepoint for nested transactions
    pub async fn savepoint(&mut self, name: &str) -> PostgresResult<String> {
        let transaction = self.get_active_transaction()?;
        
        self.savepoint_counter += 1;
        let savepoint_name = if name.is_empty() {
            format!("sp_{}", self.savepoint_counter)
        } else {
            format!("{}_{}", name, self.savepoint_counter)
        };
        
        let sql = format!("SAVEPOINT {}", savepoint_name);
        transaction.execute(&sql, &[]).await.map_err(PostgresError::from)?;
        
        self.stats.savepoints_created += 1;
        Ok(savepoint_name)
    }

    /// Release savepoint
    pub async fn release_savepoint(&mut self, name: &str) -> PostgresResult<()> {
        let transaction = self.get_active_transaction()?;
        
        let sql = format!("RELEASE SAVEPOINT {}", name);
        transaction.execute(&sql, &[]).await.map_err(PostgresError::from)?;
        
        self.stats.savepoints_released += 1;
        Ok(())
    }

    /// Rollback to savepoint
    pub async fn rollback_to_savepoint(&mut self, name: &str) -> PostgresResult<()> {
        let transaction = self.get_active_transaction()?;
        
        let sql = format!("ROLLBACK TO SAVEPOINT {}", name);
        transaction.execute(&sql, &[]).await.map_err(PostgresError::from)?;
        
        self.stats.savepoints_rolled_back += 1;
        Ok(())
    }

    /// Set transaction isolation level
    pub async fn set_isolation_level(&mut self, level: SqlIsolationLevel) -> PostgresResult<()> {
        let transaction = self.get_active_transaction()?;
        
        let sql = format!("SET TRANSACTION ISOLATION LEVEL {}", level);
        transaction.execute(&sql, &[]).await.map_err(PostgresError::from)?;
        
        Ok(())
    }

    /// Set transaction read-only mode
    pub async fn set_read_only(&mut self, read_only: bool) -> PostgresResult<()> {
        let transaction = self.get_active_transaction()?;
        
        let sql = if read_only {
            "SET TRANSACTION READ ONLY"
        } else {
            "SET TRANSACTION READ WRITE"
        };
        
        transaction.execute(sql, &[]).await.map_err(PostgresError::from)?;
        Ok(())
    }

    /// Get transaction state
    pub fn get_state(&self) -> TransactionState {
        self.state.lock().unwrap().clone()
    }

    /// Get transaction statistics
    pub fn get_stats(&self) -> TransactionStats {
        let mut stats = self.stats.clone();
        if matches!(self.get_state(), TransactionState::Committed | TransactionState::RolledBack) {
            stats.duration = Some(
                std::time::SystemTime::now()
                    .duration_since(stats.started_at)
                    .unwrap_or(std::time::Duration::ZERO)
            );
        }
        stats
    }

    /// Check if transaction is active
    pub fn is_active(&self) -> bool {
        matches!(self.get_state(), TransactionState::Active)
    }

    /// Get reference to active transaction
    fn get_active_transaction(&mut self) -> PostgresResult<&mut Transaction<'a>> {
        if !self.is_active() {
            return Err(PostgresError::new(
                PostgresErrorKind::TransactionError,
                "Transaction is not active",
            ));
        }
        
        self.transaction.as_mut().ok_or_else(|| {
            PostgresError::new(
                PostgresErrorKind::TransactionError,
                "Transaction handle is not available",
            )
        })
    }

    /// Set transaction state
    fn set_state(&self, state: TransactionState) {
        *self.state.lock().unwrap() = state;
    }

    /// Finalize transaction (update connection state)
    fn finalize_transaction(&mut self) {
        *self.in_transaction.lock().unwrap() = false;
        
        self.stats.duration = Some(
            std::time::SystemTime::now()
                .duration_since(self.stats.started_at)
                .unwrap_or(std::time::Duration::ZERO)
        );
    }
}

impl<'a> DriverTx for PostgresTransaction<'a> {
    fn query(&self, sql: &str, args: &[SqlValue]) -> crate::error::Result<()> {
        // For async execution in sync context, we need a runtime handle
        // This is a limitation of the current sync API design
//         Err(crate::stdlib::database::DatabaseError::new(
//             crate::stdlib::database::DatabaseErrorKind::NotSupported,
            "Transaction queries require async context. Use async transaction methods instead.",
        ))
    }

    fn execute(&self, sql: &str, args: &[SqlValue]) -> crate::error::Result<()> {
        // For async execution in sync context, we need a runtime handle
        // This is a limitation of the current sync API design
//         Err(crate::stdlib::database::DatabaseError::new(
//             crate::stdlib::database::DatabaseErrorKind::NotSupported,
            "Transaction execution requires async context. Use async transaction methods instead.",
        ))
    }

    fn commit(&self) -> crate::error::Result<()> {
        // Cannot commit async transaction in sync context
//         Err(crate::stdlib::database::DatabaseError::new(
//             crate::stdlib::database::DatabaseErrorKind::NotSupported,
            "Transaction commit requires async context. Use async transaction methods instead.",
        ))
    }

    fn rollback(&self) -> crate::error::Result<()> {
        // Cannot rollback async transaction in sync context
//         Err(crate::stdlib::database::DatabaseError::new(
//             crate::stdlib::database::DatabaseErrorKind::NotSupported,
            "Transaction rollback requires async context. Use async transaction methods instead.",
        ))
    }

    fn prepare(&self, query: &str) -> crate::error::Result<()> {
        // For async operations in sync context, return not supported error
//         Err(crate::stdlib::database::DatabaseError::new(
//             crate::stdlib::database::DatabaseErrorKind::NotSupported,
            "Transaction prepare requires async context. Use async transaction methods instead.",
        ))
    }

    fn options(&self) -> &TxOptions {
        // Return default options for sync context
        &TxOptions {
            isolation_level: Some(IsolationLevel::ReadCommitted),
            read_only: false,
        }
    }

    fn clone(&self) -> Box<dyn DriverTx> {
        // PostgreSQL transactions cannot be cloned in a meaningful way
        // Return an error transaction that will fail operations
        Box::new(PostgresTransaction {
            transaction: None,
            state: Arc::clone(&self.state),
            in_transaction: Arc::clone(&self.in_transaction),
            connection_stats: Arc::clone(&self.connection_stats),
            stats: self.stats.clone(),
            savepoint_counter: self.savepoint_counter,
        })
    }

    fn is_active(&self) -> bool {
        self.is_active()
    }
}

impl<'a> Drop for PostgresTransaction<'a> {
    fn drop(&mut self) {
        // If transaction is still active when dropped, it should be rolled back
        if self.is_active() && self.transaction.is_some() {
            log::warn!("PostgreSQL transaction dropped without explicit commit/rollback - will be rolled back");
            self.set_state(TransactionState::RolledBack);
            self.finalize_transaction();
        }
    }
}

impl std::fmt::Display for TransactionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionState::Active => write!(f, "Active"),
            TransactionState::Committed => write!(f, "Committed"),
            TransactionState::RolledBack => write!(f, "Rolled Back"),
            TransactionState::Failed => write!(f, "Failed"),
        }
    }
}

impl std::fmt::Display for TransactionStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Transaction Statistics:")?;
        writeln!(f, "  Statements Executed: {}", self.statements_executed)?;
        writeln!(f, "  Rows Affected: {}", self.rows_affected)?;
        writeln!(f, "  Rows Returned: {}", self.rows_returned)?;
        writeln!(f, "  Savepoints Created: {}", self.savepoints_created)?;
        writeln!(f, "  Savepoints Released: {}", self.savepoints_released)?;
        writeln!(f, "  Savepoints Rolled Back: {}", self.savepoints_rolled_back)?;
        writeln!(f, "  Started At: {:?}", self.started_at)?;
        
        if let Some(duration) = self.duration {
            writeln!(f, "  Duration: {:?}", duration)?;
        }
        
        Ok(())
    }
}

