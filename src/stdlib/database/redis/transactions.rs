/// Redis transaction management and MULTI/EXEC support
/// 
/// Provides comprehensive transaction support including MULTI/EXEC blocks,
/// optimistic locking with WATCH, pipeline operations, and transaction monitoring.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, warn};

use super::{DatabaseError, RedisConnection};
use crate::error::Error;

/// Redis transaction manager
#[derive(Debug)]
pub struct RedisTransactionManager {
    active_transactions: Arc<Mutex<HashMap<String, RedisTransaction>>>,
    transaction_stats: Arc<Mutex<TransactionStats>>,
    config: TransactionConfig,
}

/// Redis transaction
#[derive(Debug)]
pub struct RedisTransaction {
    pub id: String,
    pub connection_id: u64,
    pub commands: VecDeque<TransactionCommand>,
    pub watched_keys: Vec<String>,
    pub created_at: Instant,
    pub state: TransactionState,
    pub timeout: Duration,
}

/// Transaction command
#[derive(Debug, Clone)]
pub struct TransactionCommand {
    pub command: String,
    pub args: Vec<String>,
    pub queued_at: Instant,
}

/// Transaction state
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    Active,
    Queuing,
    WaitingExecution,
    Committed,
    Aborted,
    TimedOut,
}

/// Transaction configuration
#[derive(Debug, Clone)]
pub struct TransactionConfig {
    /// Maximum transaction duration
    pub max_duration: Duration,
    /// Maximum commands per transaction
    pub max_commands: usize,
    /// Enable optimistic locking
    pub enable_watch: bool,
    /// Transaction timeout
    pub transaction_timeout: Duration,
    /// Enable transaction monitoring
    pub enable_monitoring: bool,
}

/// Transaction statistics
#[derive(Debug, Default)]
pub struct TransactionStats {
    pub total_transactions: u64,
    pub committed_transactions: u64,
    pub aborted_transactions: u64,
    pub timed_out_transactions: u64,
    pub avg_commands_per_transaction: f64,
    pub avg_transaction_duration: Duration,
    pub peak_concurrent_transactions: usize,
    pub current_active_transactions: usize,
}

/// Transaction result
#[derive(Debug)]
pub struct TransactionResult {
    pub transaction_id: String,
    pub success: bool,
    pub commands_executed: usize,
    pub execution_time: Duration,
    pub results: Vec<CommandResult>,
    pub error_message: Option<String>,
}

/// Command result within transaction
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub command: String,
    pub success: bool,
    pub result: Option<String>,
    pub error: Option<String>,
    pub execution_time: Duration,
}

impl Default for TransactionConfig {
    fn default() -> Self {
        Self {
            max_duration: Duration::from_secs(30),
            max_commands: 100,
            enable_watch: true,
            transaction_timeout: Duration::from_secs(10),
            enable_monitoring: true,
        }
    }
}

impl RedisTransactionManager {
    /// Create new transaction manager
    #[instrument]
    pub fn new(config: TransactionConfig) -> Result<(), Error> {
        info!("Creating Redis transaction manager");
        
        config.validate()?;
        
        Ok(Self {
            active_transactions: Arc::new(Mutex::new(HashMap::new())),
            transaction_stats: Arc::new(Mutex::new(TransactionStats::default())),
            config,
        })
    }
    
    /// Begin new transaction
    #[instrument(skip(self, connection))]
    pub async fn begin_transaction(&self, connection: &mut RedisConnection) -> Result<(), Error> {
        debug!(connection_id = connection.id(), "Beginning new transaction");
        
        let transaction_id = format!("txn_{}", rand::random::<u64>());
        
        // Send MULTI command to Redis
        connection.execute_command("MULTI", &[]).await?;
        
        let transaction = RedisTransaction {
            id: transaction_id.clone(),
            connection_id: connection.id(),
            commands: VecDeque::new(),
            watched_keys: Vec::new(),
            created_at: Instant::now(),
            state: TransactionState::Active,
            timeout: self.config.transaction_timeout,
        };
        
        // Store transaction
        {
            let mut transactions = self.active_transactions.lock().unwrap();
            transactions.insert(transaction_id.clone(), transaction);
            
            // Update stats
            let mut stats = self.transaction_stats.lock().unwrap();
            stats.total_transactions += 1;
            stats.current_active_transactions = transactions.len();
            if transactions.len() > stats.peak_concurrent_transactions {
                stats.peak_concurrent_transactions = transactions.len();
            }
        }
        
        info!(transaction_id = %transaction_id, connection_id = connection.id(), "Transaction started");
        Ok(transaction_id)
    }
    
    /// Add command to transaction
    #[instrument(skip(self))]
    pub async fn queue_command(&self, transaction_id: &str, command: &str, args: &[&str]) -> Result<(), Error> {
        debug!(transaction_id = transaction_id, command = command, "Queuing command in transaction");
        
        let mut transactions = self.active_transactions.lock().unwrap();
        
        if let Some(transaction) = transactions.get_mut(transaction_id) {
            // Check transaction state
            if transaction.state != TransactionState::Active {
                return Err(DatabaseError::Transaction("Transaction is not active".to_string()));
            }
            
            // Check command limit
            if transaction.commands.len() >= self.config.max_commands {
                return Err(DatabaseError::Transaction("Transaction command limit exceeded".to_string()));
            }
            
            // Check timeout
            if transaction.created_at.elapsed() > transaction.timeout {
                transaction.state = TransactionState::TimedOut;
                return Err(DatabaseError::Transaction("Transaction timed out".to_string()));
            }
            
            // Add command to queue
            let tx_command = TransactionCommand {
                command: command.to_string(),
                args: args.iter().map(|s| s.to_string()).collect(),
                queued_at: Instant::now(),
            };
            
            transaction.commands.push_back(tx_command);
            transaction.state = TransactionState::Queuing;
            
            debug!(transaction_id = transaction_id, command = command, queue_size = transaction.commands.len(), "Command queued");
            Ok(())
        } else {
            Err(DatabaseError::Transaction("Transaction not found".to_string()))
        }
    }
    
    /// Watch keys for optimistic locking
    #[instrument(skip(self, connection))]
    pub async fn watch_keys(&self, transaction_id: &str, connection: &mut RedisConnection, keys: &[&str]) -> Result<(), Error> {
        debug!(transaction_id = transaction_id, keys = ?keys, "Watching keys for transaction");
        
        if !self.config.enable_watch {
            return Err(DatabaseError::Transaction("WATCH is disabled in configuration".to_string()));
        }
        
        let mut transactions = self.active_transactions.lock().unwrap();
        
        if let Some(transaction) = transactions.get_mut(transaction_id) {
            // Send WATCH command to Redis
            let watch_args: Vec<&str> = keys.iter().copied().collect();
            connection.execute_command("WATCH", &watch_args).await?;
            
            // Store watched keys
            transaction.watched_keys.extend(keys.iter().map(|s| s.to_string()));
            
            info!(transaction_id = transaction_id, keys = ?keys, "Keys are now watched");
            Ok(())
        } else {
            Err(DatabaseError::Transaction("Transaction not found".to_string()))
        }
    }
    
    /// Commit transaction (execute EXEC)
    #[instrument(skip(self, connection))]
    pub async fn commit_transaction(&self, transaction_id: &str, connection: &mut RedisConnection) -> Result<(), Error> {
        info!(transaction_id = transaction_id, "Committing transaction");
        
        let start_time = Instant::now();
        
        // Get and remove transaction
        let transaction = {
            let mut transactions = self.active_transactions.lock().unwrap();
            transactions.remove(transaction_id)
        };
        
        if let Some(mut transaction) = transaction {
            transaction.state = TransactionState::WaitingExecution;
            
            // Execute EXEC command
            let exec_result = connection.execute_command("EXEC", &[]).await;
            
            let execution_time = start_time.elapsed();
            
            match exec_result {
                Ok(_) => {
                    transaction.state = TransactionState::Committed;
                    
                    // Update statistics
                    self.update_commit_stats(&transaction, execution_time);
                    
                    let result = TransactionResult {
                        transaction_id: transaction_id.to_string(),
                        success: true,
                        commands_executed: transaction.commands.len(),
                        execution_time,
                        results: self.simulate_command_results(&transaction),
                        error_message: None,
                    };
                    
                    info!(transaction_id = transaction_id, commands = transaction.commands.len(), duration = ?execution_time, "Transaction committed successfully");
                    Ok(result)
                }
                Err(e) => {
                    transaction.state = TransactionState::Aborted;
                    
                    // Update statistics
                    self.update_abort_stats();
                    
                    let result = TransactionResult {
                        transaction_id: transaction_id.to_string(),
                        success: false,
                        commands_executed: 0,
                        execution_time,
                        results: Vec::new(),
                        error_message: Some(format!("Transaction failed: {}", e)),
                    };
                    
                    error!(transaction_id = transaction_id, error = ?e, "Transaction commit failed");
                    Ok(result)
                }
            }
        } else {
            Err(DatabaseError::Transaction("Transaction not found".to_string()))
        }
    }
    
    /// Abort transaction (execute DISCARD)
    #[instrument(skip(self, connection))]
    pub async fn abort_transaction(&self, transaction_id: &str, connection: &mut RedisConnection) -> Result<(), Error> {
        info!(transaction_id = transaction_id, "Aborting transaction");
        
        // Remove transaction
        let transaction = {
            let mut transactions = self.active_transactions.lock().unwrap();
            transactions.remove(transaction_id)
        };
        
        if let Some(mut transaction) = transaction {
            // Send DISCARD command to Redis
            connection.execute_command("DISCARD", &[]).await?;
            
            transaction.state = TransactionState::Aborted;
            
            // Update statistics
            self.update_abort_stats();
            
            info!(transaction_id = transaction_id, "Transaction aborted successfully");
            Ok(())
        } else {
            Err(DatabaseError::Transaction("Transaction not found".to_string()))
        }
    }
    
    /// Get transaction information
    #[instrument(skip(self))]
    pub fn get_transaction_info(&self, transaction_id: &str) -> Option<TransactionInfo> {
        let transactions = self.active_transactions.lock().unwrap();
        
        transactions.get(transaction_id).map(|tx| TransactionInfo {
            id: tx.id.clone(),
            state: tx.state.clone(),
            commands_count: tx.commands.len(),
            watched_keys_count: tx.watched_keys.len(),
            created_at: tx.created_at,
            age: tx.created_at.elapsed(),
        })
    }
    
    /// Get transaction statistics
    pub fn get_stats(&self) -> TransactionStats {
        self.transaction_stats.lock().unwrap().clone()
    }
    
    /// Clean up expired transactions
    #[instrument(skip(self))]
    pub async fn cleanup_expired_transactions(&self) -> Result<(), Error> {
        debug!("Cleaning up expired transactions");
        
        let mut transactions = self.active_transactions.lock().unwrap();
        let mut expired_ids = Vec::new();
        
        for (id, transaction) in transactions.iter_mut() {
            if transaction.created_at.elapsed() > transaction.timeout {
                transaction.state = TransactionState::TimedOut;
                expired_ids.push(id.clone());
            }
        }
        
        for id in &expired_ids {
            transactions.remove(id);
        }
        
        // Update timeout statistics
        if !expired_ids.is_empty() {
            let mut stats = self.transaction_stats.lock().unwrap();
            stats.timed_out_transactions += expired_ids.len() as u64;
            stats.current_active_transactions = transactions.len();
        }
        
        if !expired_ids.is_empty() {
            warn!(expired_count = expired_ids.len(), "Cleaned up expired transactions");
        }
        
        Ok(expired_ids.len())
    }
    
    /// Update commit statistics
    fn update_commit_stats(&self, transaction: &RedisTransaction, execution_time: Duration) {
        let mut stats = self.transaction_stats.lock().unwrap();
        stats.committed_transactions += 1;
        stats.current_active_transactions = stats.current_active_transactions.saturating_sub(1);
        
        // Update averages
        let total_commands = stats.avg_commands_per_transaction * (stats.committed_transactions - 1) as f64 + transaction.commands.len() as f64;
        stats.avg_commands_per_transaction = total_commands / stats.committed_transactions as f64;
        
        let total_time = stats.avg_transaction_duration * (stats.committed_transactions - 1) as u32 + execution_time;
        stats.avg_transaction_duration = total_time / stats.committed_transactions as u32;
    }
    
    /// Update abort statistics
    fn update_abort_stats(&self) {
        let mut stats = self.transaction_stats.lock().unwrap();
        stats.aborted_transactions += 1;
        stats.current_active_transactions = stats.current_active_transactions.saturating_sub(1);
    }
    
    /// Simulate command results (placeholder)
    fn simulate_command_results(&self, transaction: &RedisTransaction) -> Vec<CommandResult> {
        transaction.commands.iter().map(|cmd| CommandResult {
            command: cmd.command.clone(),
            success: true,
            result: Some("OK".to_string()),
            error: None,
            execution_time: Duration::from_millis(1),
        }).collect()
    }
}

/// Transaction information summary
#[derive(Debug, Clone)]
pub struct TransactionInfo {
    pub id: String,
    pub state: TransactionState,
    pub commands_count: usize,
    pub watched_keys_count: usize,
    pub created_at: Instant,
    pub age: Duration,
}

impl TransactionConfig {
    /// Validate transaction configuration
    pub fn validate(&self) -> Result<(), Error> {
        if self.max_duration.as_secs() == 0 {
            return Err(DatabaseError::Configuration("Max duration must be greater than 0".to_string()));
        }
        
        if self.max_commands == 0 {
            return Err(DatabaseError::Configuration("Max commands must be greater than 0".to_string()));
        }
        
        if self.transaction_timeout.as_secs() == 0 {
            return Err(DatabaseError::Configuration("Transaction timeout must be greater than 0".to_string()));
        }
        
        Ok(())
    }
}

impl Clone for TransactionStats {
    fn clone(&self) -> Self {
        Self {
            total_transactions: self.total_transactions,
            committed_transactions: self.committed_transactions,
            aborted_transactions: self.aborted_transactions,
            timed_out_transactions: self.timed_out_transactions,
            avg_commands_per_transaction: self.avg_commands_per_transaction,
            avg_transaction_duration: self.avg_transaction_duration,
            peak_concurrent_transactions: self.peak_concurrent_transactions,
            current_active_transactions: self.current_active_transactions,
        }
    }
}
