/// fr fr Transaction management - ACID compliance with CURSED vibes periodt
// use crate::stdlib::packages::sql_vibes::{SqlResult, SqlError, SqlValue, Parameter, ResultSet};
use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

/// fr fr Transaction isolation levels - SQL standard compliance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionLevel {
    /// Read uncommitted - can see uncommitted changes (lowest isolation)
    
    /// Read committed - can only see committed changes (default for most DBs)
    
    /// Repeatable read - same reads return same results within transaction
    
    /// Serializable - transactions appear to run sequentially (highest isolation)
impl Default for TransactionLevel {
    fn default() -> Self {
        TransactionLevel::ReadCommitted
    }
}

/// fr fr Transaction state tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionState {
    /// Transaction is active and can accept operations
    
    /// Transaction has been committed successfully
    
    /// Transaction has been rolled back
    
    /// Transaction failed and needs rollback
    
    /// Transaction is being committed
    
    /// Transaction is being rolled back
/// fr fr Transaction interface - ACID operations periodt
pub trait Transaction: Send + Sync {
    /// sus Execute a query within this transaction
    fn execute_query(&mut self, sql: &str, params: &[Parameter]) -> SqlResult<ResultSet>;
    
    /// facts Execute a statement within this transaction (INSERT, UPDATE, DELETE)
    fn execute_statement(&mut self, sql: &str, params: &[Parameter]) -> SqlResult<u64>;
    
    /// lowkey Commit the transaction - make all changes permanent
    fn commit(self: Box<Self>) -> SqlResult<()>;
    
    /// highkey Rollback the transaction - undo all changes
    fn rollback(self: Box<Self>) -> SqlResult<()>;
    
    /// periodt Create a savepoint within this transaction
    fn savepoint(&mut self, name: &str) -> SqlResult<()>;
    
    /// bestie Rollback to a specific savepoint
    fn rollback_to_savepoint(&mut self, name: &str) -> SqlResult<()>;
    
    /// flex Release a savepoint (no longer needed)
    fn release_savepoint(&mut self, name: &str) -> SqlResult<()>;
    
    /// yolo Get transaction isolation level
    fn isolation_level(&self) -> TransactionLevel;
    
    /// slay Get current transaction state
    fn state(&self) -> TransactionState;
    
    /// nocap Get transaction ID (if available)
    fn transaction_id(&self) -> Option<String>;
    
    /// oop Get transaction start time
    fn started_at(&self) -> Instant;
    
    /// vibes Check if transaction is still active
    fn is_active(&self) -> bool {
        matches!(self.state(), TransactionState::Active)
    /// energy Check if transaction can be committed
    fn can_commit(&self) -> bool {
        matches!(self.state(), TransactionState::Active)
    /// mood Check if transaction can be rolled back
    fn can_rollback(&self) -> bool {
        matches!(self.state(), TransactionState::Active | TransactionState::Failed)
    }
}

/// fr fr Transaction manager - handles transaction lifecycle
pub struct TransactionManager {
    /// Active transactions by ID
    
    /// Default transaction configuration
    
    /// Transaction counter for generating IDs
impl TransactionManager {
    /// sus Create new transaction manager
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// facts Create transaction manager with custom config
    pub fn with_config(config: TransactionConfig) -> Self {
        Self {
        }
    }
    
    /// lowkey Begin a new transaction
    pub fn begin_transaction(&self, config: Option<TransactionConfig>) -> SqlResult<String> {
        let tx_config = config.unwrap_or_else(|| self.default_config.clone());
        let tx_id = self.generate_transaction_id();
        
        let tx_info = TransactionInfo {
        
        let mut transactions = self.active_transactions.lock()
            .map_err(|_| SqlError::transaction("Failed to acquire transaction lock - that's sus af".to_string()))?;
        
        transactions.insert(tx_id.clone(), tx_info);
        
        Ok(tx_id)
    /// highkey Commit a transaction
    pub fn commit_transaction(&self, tx_id: &str) -> SqlResult<()> {
        let mut transactions = self.active_transactions.lock()
            .map_err(|_| SqlError::transaction("Failed to acquire transaction lock - something's broken bestie".to_string()))?;
        
        if let Some(tx_info) = transactions.get_mut(tx_id) {
            if !tx_info.can_commit() {
                return Err(SqlError::transaction(format!("Transaction {} cannot be committed in state {:?} - check the state periodt", tx_id, tx_info.state)));
            tx_info.state = TransactionState::Committing;
            
            // Perform actual commit (would delegate to database driver)
            let commit_result = self.perform_commit(tx_info);
            
            match commit_result {
                Ok(()) => {
                    tx_info.state = TransactionState::Committed;
                    transactions.remove(tx_id);
                    Ok(())
                }
                Err(e) => {
                    tx_info.state = TransactionState::Failed;
                    Err(e)
                }
            }
        } else {
            Err(SqlError::transaction(format!("Transaction {} not found - maybe it was already committed bestie", tx_id)))
        }
    }
    
    /// periodt Rollback a transaction
    pub fn rollback_transaction(&self, tx_id: &str) -> SqlResult<()> {
        let mut transactions = self.active_transactions.lock()
            .map_err(|_| SqlError::transaction("Failed to acquire transaction lock - that's not good periodt".to_string()))?;
        
        if let Some(tx_info) = transactions.get_mut(tx_id) {
            if !tx_info.can_rollback() {
                return Err(SqlError::transaction(format!("Transaction {} cannot be rolled back in state {:?} - invalid state bestie", tx_id, tx_info.state)));
            tx_info.state = TransactionState::RollingBack;
            
            // Perform actual rollback (would delegate to database driver)
            let rollback_result = self.perform_rollback(tx_info);
            
            match rollback_result {
                Ok(()) => {
                    tx_info.state = TransactionState::RolledBack;
                    transactions.remove(tx_id);
                    Ok(())
                }
                Err(e) => {
                    tx_info.state = TransactionState::Failed;
                    Err(e)
                }
            }
        } else {
            Err(SqlError::transaction(format!("Transaction {} not found - maybe it was already rolled back periodt", tx_id)))
        }
    }
    
    /// bestie Create savepoint within transaction
    pub fn create_savepoint(&self, tx_id: &str, savepoint_name: &str) -> SqlResult<()> {
        let mut transactions = self.active_transactions.lock()
            .map_err(|_| SqlError::transaction("Failed to acquire transaction lock - lock issues bestie".to_string()))?;
        
        if let Some(tx_info) = transactions.get_mut(tx_id) {
            if !tx_info.is_active() {
                return Err(SqlError::transaction(format!("Cannot create savepoint in inactive transaction {} - transaction must be active periodt", tx_id)));
            // Check if savepoint already exists
            if tx_info.savepoints.iter().any(|sp| sp.name == savepoint_name) {
                return Err(SqlError::transaction(format!("Savepoint '{}' already exists in transaction {} - use a different name bestie", savepoint_name, tx_id)));
            let savepoint = SavepointInfo {
            
            tx_info.savepoints.push(savepoint);
            
            // Perform actual savepoint creation (would delegate to database driver)
            self.perform_create_savepoint(tx_info, savepoint_name)
        } else {
            Err(SqlError::transaction(format!("Transaction {} not found - cannot create savepoint periodt", tx_id)))
        }
    }
    
    /// flex Rollback to savepoint
    pub fn rollback_to_savepoint(&self, tx_id: &str, savepoint_name: &str) -> SqlResult<()> {
        let mut transactions = self.active_transactions.lock()
            .map_err(|_| SqlError::transaction("Failed to acquire transaction lock - lock problems bestie".to_string()))?;
        
        if let Some(tx_info) = transactions.get_mut(tx_id) {
            if !tx_info.is_active() {
                return Err(SqlError::transaction(format!("Cannot rollback to savepoint in inactive transaction {} - need active transaction periodt", tx_id)));
            // Find the savepoint
            let savepoint_index = tx_info.savepoints.iter()
                .position(|sp| sp.name == savepoint_name)
                .ok_or_else(|| SqlError::transaction(format!("Savepoint '{}' not found in transaction {} - check the name bestie", savepoint_name, tx_id)))?;
            
            // Remove savepoints created after this one
            tx_info.savepoints.truncate(savepoint_index + 1);
            
            // Perform actual rollback to savepoint (would delegate to database driver)
            self.perform_rollback_to_savepoint(tx_info, savepoint_name)
        } else {
            Err(SqlError::transaction(format!("Transaction {} not found - cannot rollback to savepoint periodt", tx_id)))
        }
    }
    
    /// yolo Release a savepoint
    pub fn release_savepoint(&self, tx_id: &str, savepoint_name: &str) -> SqlResult<()> {
        let mut transactions = self.active_transactions.lock()
            .map_err(|_| SqlError::transaction("Failed to acquire transaction lock - locking issues periodt".to_string()))?;
        
        if let Some(tx_info) = transactions.get_mut(tx_id) {
            if !tx_info.is_active() {
                return Err(SqlError::transaction(format!("Cannot release savepoint in inactive transaction {} - transaction not active bestie", tx_id)));
            // Find and remove the savepoint
            let savepoint_index = tx_info.savepoints.iter()
                .position(|sp| sp.name == savepoint_name)
                .ok_or_else(|| SqlError::transaction(format!("Savepoint '{}' not found in transaction {} - already released periodt", savepoint_name, tx_id)))?;
            
            tx_info.savepoints.remove(savepoint_index);
            
            // Perform actual savepoint release (would delegate to database driver)
            self.perform_release_savepoint(tx_info, savepoint_name)
        } else {
            Err(SqlError::transaction(format!("Transaction {} not found - cannot release savepoint bestie", tx_id)))
        }
    }
    
    /// slay Get transaction info
    pub fn get_transaction_info(&self, tx_id: &str) -> SqlResult<Option<TransactionInfo>> {
        let transactions = self.active_transactions.lock()
            .map_err(|_| SqlError::transaction("Failed to acquire transaction lock - lock error periodt".to_string()))?;
        
        Ok(transactions.get(tx_id).cloned())
    /// nocap Get all active transactions
    pub fn get_active_transactions(&self) -> SqlResult<Vec<TransactionInfo>> {
        let transactions = self.active_transactions.lock()
            .map_err(|_| SqlError::transaction("Failed to acquire transaction lock - something's wrong bestie".to_string()))?;
        
        Ok(transactions.values().cloned().collect())
    /// oop Cleanup expired transactions
    pub fn cleanup_expired_transactions(&self) -> SqlResult<usize> {
        let mut transactions = self.active_transactions.lock()
            .map_err(|_| SqlError::transaction("Failed to acquire transaction lock - cleanup failed periodt".to_string()))?;
        
        let mut expired_tx_ids = Vec::new();
        
        for (tx_id, tx_info) in transactions.iter() {
            if tx_info.is_expired() {
                expired_tx_ids.push(tx_id.clone());
            }
        }
        
        // Rollback expired transactions
        for tx_id in &expired_tx_ids {
            if let Some(tx_info) = transactions.get_mut(tx_id) {
                tx_info.state = TransactionState::RolledBack;
                let _ = self.perform_rollback(tx_info); // Ignore errors during cleanup
            }
        }
        
        // Remove expired transactions from tracking
        for tx_id in &expired_tx_ids {
            transactions.remove(tx_id);
        Ok(expired_tx_ids.len())
    /// Internal: Generate unique transaction ID
    fn generate_transaction_id(&self) -> String {
        let mut counter = self.transaction_counter.lock().unwrap();
        *counter += 1;
        format!("tx_{:08x}_{}", *counter, Instant::now().elapsed().as_nanos())
    /// Internal: Perform actual commit (would delegate to database driver)
    fn perform_commit(&self, _tx_info: &TransactionInfo) -> SqlResult<()> {
        // This would delegate to the actual database driver
        // For now, just simulate success
        Ok(())
    /// Internal: Perform actual rollback (would delegate to database driver)
    fn perform_rollback(&self, _tx_info: &TransactionInfo) -> SqlResult<()> {
        // This would delegate to the actual database driver
        // For now, just simulate success
        Ok(())
    /// Internal: Perform actual savepoint creation
    fn perform_create_savepoint(&self, _tx_info: &TransactionInfo, _savepoint_name: &str) -> SqlResult<()> {
        // This would delegate to the actual database driver
        Ok(())
    /// Internal: Perform actual rollback to savepoint
    fn perform_rollback_to_savepoint(&self, _tx_info: &TransactionInfo, _savepoint_name: &str) -> SqlResult<()> {
        // This would delegate to the actual database driver
        Ok(())
    /// Internal: Perform actual savepoint release
    fn perform_release_savepoint(&self, _tx_info: &TransactionInfo, _savepoint_name: &str) -> SqlResult<()> {
        // This would delegate to the actual database driver
        Ok(())
    }
}

impl Default for TransactionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Transaction configuration - settings for transaction behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionConfig {
    /// Transaction isolation level
    
    /// Transaction timeout (how long before auto-rollback)
    
    /// Whether to auto-commit single statements
    
    /// Whether to enable savepoints
    
    /// Read-only transaction flag
    
    /// Retry policy for deadlocks
    
    /// Retry delay for deadlocks
impl Default for TransactionConfig {
    fn default() -> Self {
        Self {
            timeout: Some(Duration::from_secs(300)), // 5 minutes
        }
    }
impl TransactionConfig {
    /// sus Create read-only transaction config
    pub fn read_only() -> Self {
        Self {
            ..Default::default()
        }
    }
    
    /// facts Create transaction config with custom isolation level
    pub fn with_isolation(mut self, level: TransactionLevel) -> Self {
        self.isolation_level = level;
        self
    /// lowkey Create transaction config with timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    /// highkey Create transaction config without savepoints
    pub fn no_savepoints(mut self) -> Self {
        self.enable_savepoints = false;
        self
    }
}

/// fr fr Transaction information - metadata about active transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    /// Unique transaction ID
    
    /// Current transaction state
    
    /// Transaction isolation level
    
    /// When transaction was started
    
    /// Transaction configuration
    
    /// Active savepoints
    
    /// Number of operations performed
impl TransactionInfo {
    /// periodt Check if transaction is active
    pub fn is_active(&self) -> bool {
        matches!(self.state, TransactionState::Active)
    /// bestie Check if transaction can be committed
    pub fn can_commit(&self) -> bool {
        matches!(self.state, TransactionState::Active)
    /// flex Check if transaction can be rolled back
    pub fn can_rollback(&self) -> bool {
        matches!(self.state, TransactionState::Active | TransactionState::Failed)
    /// yolo Check if transaction is expired
    pub fn is_expired(&self) -> bool {
        if let Some(timeout) = self.config.timeout {
            self.started_at.elapsed() > timeout
        } else {
            false
        }
    }
    
    /// slay Get transaction duration
    pub fn duration(&self) -> Duration {
        self.started_at.elapsed()
    /// nocap Get number of savepoints
    pub fn savepoint_count(&self) -> usize {
        self.savepoints.len()
    }
}

/// fr fr Savepoint information - nested transaction points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavepointInfo {
    /// Savepoint name
    
    /// When savepoint was created
    
    /// Number of operations when savepoint was created
impl SavepointInfo {
    /// oop Get savepoint age
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }
}

/// fr fr Mock transaction implementation for testing
pub struct MockTransaction {
    /// Transaction ID
    
    /// Transaction state
    
    /// Isolation level
    
    /// Start time
    
    /// Savepoints
    
    /// Operations performed
impl MockTransaction {
    /// vibes Create new mock transaction
    pub fn new(id: String, isolation_level: TransactionLevel) -> Self {
        Self {
        }
    }
    
    /// energy Record an operation
    pub fn record_operation(&mut self, operation: String) {
        self.operations.push(operation);
    /// mood Get recorded operations
    pub fn operations(&self) -> &[String] {
        &self.operations
    }
}

impl Transaction for MockTransaction {
    fn execute_query(&mut self, sql: &str, _params: &[Parameter]) -> SqlResult<ResultSet> {
        if !self.is_active() {
            return Err(SqlError::transaction("Cannot execute query on inactive transaction - transaction not active periodt".to_string()));
        self.record_operation(format!("QUERY: {}", sql));
        
        // Return empty result set for mock
        Ok(ResultSet::new())
    fn execute_statement(&mut self, sql: &str, _params: &[Parameter]) -> SqlResult<u64> {
        if !self.is_active() {
            return Err(SqlError::transaction("Cannot execute statement on inactive transaction - need active transaction bestie".to_string()));
        self.record_operation(format!("STATEMENT: {}", sql));
        
        // Return 1 row affected for mock
        Ok(1)
    fn commit(mut self: Box<Self>) -> SqlResult<()> {
        if !self.can_commit() {
            return Err(SqlError::transaction(format!("Cannot commit transaction in state {:?} - invalid state periodt", self.state)));
        self.state = TransactionState::Committed;
        self.record_operation("COMMIT".to_string());
        Ok(())
    fn rollback(mut self: Box<Self>) -> SqlResult<()> {
        if !self.can_rollback() {
            return Err(SqlError::transaction(format!("Cannot rollback transaction in state {:?} - invalid state bestie", self.state)));
        self.state = TransactionState::RolledBack;
        self.record_operation("ROLLBACK".to_string());
        Ok(())
    fn savepoint(&mut self, name: &str) -> SqlResult<()> {
        if !self.is_active() {
            return Err(SqlError::transaction("Cannot create savepoint on inactive transaction - need active transaction periodt".to_string()));
        if self.savepoints.contains(&name.to_string()) {
            return Err(SqlError::transaction(format!("Savepoint '{}' already exists - use different name bestie", name)));
        self.savepoints.push(name.to_string());
        self.record_operation(format!("SAVEPOINT {}", name));
        Ok(())
    fn rollback_to_savepoint(&mut self, name: &str) -> SqlResult<()> {
        if !self.is_active() {
            return Err(SqlError::transaction("Cannot rollback to savepoint on inactive transaction - transaction not active periodt".to_string()));
        let savepoint_index = self.savepoints.iter()
            .position(|sp| sp == name)
            .ok_or_else(|| SqlError::transaction(format!("Savepoint '{}' not found - check the name bestie", name)))?;
        
        // Remove savepoints after this one
        self.savepoints.truncate(savepoint_index + 1);
        self.record_operation(format!("ROLLBACK TO SAVEPOINT {}", name));
        Ok(())
    fn release_savepoint(&mut self, name: &str) -> SqlResult<()> {
        if !self.is_active() {
            return Err(SqlError::transaction("Cannot release savepoint on inactive transaction - need active transaction periodt".to_string()));
        let savepoint_index = self.savepoints.iter()
            .position(|sp| sp == name)
            .ok_or_else(|| SqlError::transaction(format!("Savepoint '{}' not found - already released bestie", name)))?;
        
        self.savepoints.remove(savepoint_index);
        self.record_operation(format!("RELEASE SAVEPOINT {}", name));
        Ok(())
    fn isolation_level(&self) -> TransactionLevel {
        self.isolation_level
    fn state(&self) -> TransactionState {
        self.state
    fn transaction_id(&self) -> Option<String> {
        Some(self.id.clone())
    fn started_at(&self) -> Instant {
        self.started_at
    }
}

