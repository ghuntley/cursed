/// PostgreSQL transaction implementation with savepoints and isolation levels
/// 
/// This module provides comprehensive transaction support for PostgreSQL including
/// savepoints, nested transactions, isolation levels, and proper rollback handling.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use super::super::{
    DriverTx, DriverStmt, DatabaseError, SqlValue, QueryResult, ExecuteResult, TxOptions, SqlIsolationLevel
};
use super::{
    PostgreSQLError, PostgreSQLConfig, PostgreSQLStatement
};
use super::ffi::SafePGconn;

/// fr fr PostgreSQL transaction implementation
#[derive(Debug)]
pub struct PostgreSQLTransaction {
    /// fr fr Connection handle
    conn: Arc<Mutex<SafePGconn>>,
    /// fr fr Transaction options
    options: TxOptions,
    /// fr fr Configuration
    config: PostgreSQLConfig,
    /// fr fr Transaction state
    state: TransactionState,
    /// fr fr Savepoints stack
    savepoints: Vec<String>,
    /// fr fr Transaction metadata
    metadata: TransactionMetadata,
}

/// fr fr Transaction state tracking
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    /// Transaction is active
    Active,
    /// Transaction has been committed
    Committed,
    /// Transaction has been rolled back
    RolledBack,
    /// Transaction is in error state
    Error(String),
}

/// fr fr Transaction metadata for monitoring
#[derive(Debug, Clone)]
pub struct TransactionMetadata {
    /// fr fr Transaction start time
    pub started_at: std::time::SystemTime,
    /// fr fr Transaction ID (if available)
    pub transaction_id: Option<String>,
    /// fr fr Statements executed in this transaction
    pub statements_executed: u64,
    /// fr fr Savepoints created
    pub savepoints_created: u64,
    /// fr fr Total duration (when completed)
    pub duration: Option<std::time::Duration>,
    /// fr fr Whether transaction is read-only
    pub read_only: bool,
    /// fr fr Isolation level
    pub isolation_level: SqlIsolationLevel,
}

impl Default for TransactionMetadata {
    fn default() -> Self {
        Self {
            started_at: std::time::SystemTime::now(),
            transaction_id: None,
            statements_executed: 0,
            savepoints_created: 0,
            duration: None,
            read_only: false,
            isolation_level: SqlIsolationLevel::LevelReadCommitted,
        }
    }
}

impl PostgreSQLTransaction {
    /// slay Begin a new transaction
    pub fn begin(
        conn: Arc<Mutex<SafePGconn>>,
        options: TxOptions,
        config: &PostgreSQLConfig,
    ) -> Result<Self, PostgreSQLError> {
        let mut metadata = TransactionMetadata::default();
        metadata.isolation_level = options.isolation_level;
        metadata.read_only = options.read_only;
        
        let mut tx = Self {
            conn,
            options: options.clone(),
            config: config.clone(),
            state: TransactionState::Active,
            savepoints: Vec::new(),
            metadata,
        };
        
        tx.start_transaction()?;
        Ok(tx)
    }
    
    /// slay Start the actual PostgreSQL transaction
    fn start_transaction(&mut self) -> Result<(), PostgreSQLError> {
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::transaction_error("Failed to acquire connection lock")
        })?;
        
        // Build BEGIN statement with options
        let mut begin_sql = String::from("BEGIN");
        
        // Set isolation level
        let isolation_sql = match self.options.isolation_level {
            SqlIsolationLevel::LevelReadUncommitted => " ISOLATION LEVEL READ UNCOMMITTED",
            SqlIsolationLevel::LevelReadCommitted => " ISOLATION LEVEL READ COMMITTED",
            SqlIsolationLevel::LevelRepeatableRead => " ISOLATION LEVEL REPEATABLE READ",
            SqlIsolationLevel::LevelSerializable => " ISOLATION LEVEL SERIALIZABLE",
            _ => "", // Use default
        };
        begin_sql.push_str(isolation_sql);
        
        // Set read-only mode
        if self.options.read_only {
            begin_sql.push_str(" READ ONLY");
        } else {
            begin_sql.push_str(" READ WRITE");
        }
        
        // Execute BEGIN statement
        conn.exec(&begin_sql)
            .map_err(|e| PostgreSQLError::transaction_error(&format!("Failed to begin transaction: {}", e)))?;
        
        // Get transaction ID if possible
        if let Ok(result) = conn.exec("SELECT txid_current()") {
            if result.ntuples() > 0 {
                if let Some(txid_bytes) = result.get_value(0, 0) {
                    let txid = String::from_utf8_lossy(&txid_bytes);
                    self.metadata.transaction_id = Some(txid.to_string());
                }
            }
        }
        
        Ok(())
    }
    
    /// slay Create a savepoint
    pub fn savepoint(&mut self, name: &str) -> Result<(), PostgreSQLError> {
        if self.state != TransactionState::Active {
            return Err(PostgreSQLError::transaction_error("Transaction is not active"));
        }
        
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::transaction_error("Failed to acquire connection lock")
        })?;
        
        let savepoint_sql = format!("SAVEPOINT {}", name);
        conn.exec(&savepoint_sql)
            .map_err(|e| PostgreSQLError::transaction_error(&format!("Failed to create savepoint: {}", e)))?;
        
        self.savepoints.push(name.to_string());
        self.metadata.savepoints_created += 1;
        
        Ok(())
    }
    
    /// slay Release a savepoint
    pub fn release_savepoint(&mut self, name: &str) -> Result<(), PostgreSQLError> {
        if self.state != TransactionState::Active {
            return Err(PostgreSQLError::transaction_error("Transaction is not active"));
        }
        
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::transaction_error("Failed to acquire connection lock")
        })?;
        
        let release_sql = format!("RELEASE SAVEPOINT {}", name);
        conn.exec(&release_sql)
            .map_err(|e| PostgreSQLError::transaction_error(&format!("Failed to release savepoint: {}", e)))?;
        
        // Remove from savepoints stack
        if let Some(pos) = self.savepoints.iter().position(|sp| sp == name) {
            self.savepoints.remove(pos);
        }
        
        Ok(())
    }
    
    /// slay Rollback to a savepoint
    pub fn rollback_to_savepoint(&mut self, name: &str) -> Result<(), PostgreSQLError> {
        if self.state != TransactionState::Active {
            return Err(PostgreSQLError::transaction_error("Transaction is not active"));
        }
        
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::transaction_error("Failed to acquire connection lock")
        })?;
        
        let rollback_sql = format!("ROLLBACK TO SAVEPOINT {}", name);
        conn.exec(&rollback_sql)
            .map_err(|e| PostgreSQLError::transaction_error(&format!("Failed to rollback to savepoint: {}", e)))?;
        
        // Remove all savepoints after this one
        if let Some(pos) = self.savepoints.iter().position(|sp| sp == name) {
            self.savepoints.truncate(pos + 1);
        }
        
        Ok(())
    }
    
    /// slay Execute query within transaction
    fn execute_in_transaction(&self, query: &str, args: &[SqlValue], return_rows: bool) -> Result<super::ffi::SafePGresult, PostgreSQLError> {
        if self.state != TransactionState::Active {
            return Err(PostgreSQLError::transaction_error("Transaction is not active"));
        }
        
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::transaction_error("Failed to acquire connection lock")
        })?;
        
        // For simplicity, execute as simple query
        // In production, you'd use proper parameter binding
        let final_query = if args.is_empty() {
            query.to_string()
        } else {
            // Simple parameter substitution (not safe for production)
            let mut result = query.to_string();
            for (i, arg) in args.iter().enumerate() {
                let placeholder = format!("${}", i + 1);
                let value_str = self.sql_value_to_string(arg)?;
                result = result.replace(&placeholder, &value_str);
            }
            result
        };
        
        conn.exec(&final_query)
            .map_err(|e| PostgreSQLError::query_error(&format!("Query failed in transaction: {}", e)))
    }
    
    /// slay Convert SqlValue to SQL string (simplified)
    fn sql_value_to_string(&self, value: &SqlValue) -> Result<String, PostgreSQLError> {
        match value {
            SqlValue::Null => Ok("NULL".to_string()),
            SqlValue::Boolean(b) => Ok(if *b { "TRUE" } else { "FALSE" }.to_string()),
            SqlValue::Integer(i) => Ok(i.to_string()),
            SqlValue::Float(f) => Ok(f.to_string()),
            SqlValue::String(s) => {
                let escaped = s.replace("'", "''");
                Ok(format!("'{}'", escaped))
            }
            SqlValue::Bytes(b) => {
                let hex = b.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
                Ok(format!("'\\x{}'", hex))
            }
            SqlValue::Json(j) => {
                let json_str = j.to_string().replace("'", "''");
                Ok(format!("'{}'::jsonb", json_str))
            }
            SqlValue::Timestamp(t) => {
                let duration = t.duration_since(std::time::UNIX_EPOCH)
                    .map_err(|_| PostgreSQLError::query_error("Invalid timestamp"))?;
                Ok(format!("to_timestamp({})", duration.as_secs()))
            }
        }
    }
    
    /// slay Convert result to QueryResult
    fn convert_to_query_result(&self, pg_result: super::ffi::SafePGresult) -> Result<QueryResult, PostgreSQLError> {
        let num_fields = pg_result.nfields();
        let num_tuples = pg_result.ntuples();
        
        let mut column_names = Vec::with_capacity(num_fields as usize);
        let mut column_types = Vec::with_capacity(num_fields as usize);
        
        for col in 0..num_fields {
            column_names.push(pg_result.field_name(col));
            let type_oid = pg_result.field_type(col);
            let pg_type = super::types::PostgreSQLType::from_oid(type_oid);
            column_types.push(pg_type.sql_name());
        }
        
        let mut rows = Vec::with_capacity(num_tuples as usize);
        
        for row in 0..num_tuples {
            let mut row_values = Vec::with_capacity(num_fields as usize);
            
            for col in 0..num_fields {
                if let Some(value_bytes) = pg_result.get_value(row, col) {
                    let type_oid = pg_result.field_type(col);
                    let pg_type = super::types::PostgreSQLType::from_oid(type_oid);
                    
                    match super::types::PostgreSQLValue::from_pg_bytes(&value_bytes, pg_type) {
                        Ok(pg_value) => row_values.push(pg_value.value),
                        Err(_) => {
                            let text = String::from_utf8_lossy(&value_bytes);
                            row_values.push(SqlValue::String(text.to_string()));
                        }
                    }
                } else {
                    row_values.push(SqlValue::Null);
                }
            }
            
            rows.push(row_values);
        }
        
        Ok(QueryResult::new(column_names, column_types, rows))
    }
    
    /// slay Get transaction metadata
    pub fn metadata(&self) -> &TransactionMetadata {
        &self.metadata
    }
    
    /// slay Get current savepoints
    pub fn savepoints(&self) -> &[String] {
        &self.savepoints
    }
    
    /// slay Check if transaction has savepoints
    pub fn has_savepoints(&self) -> bool {
        !self.savepoints.is_empty()
    }
    
    /// slay Set transaction as read-only
    pub fn set_read_only(&mut self, read_only: bool) -> Result<(), PostgreSQLError> {
        if self.state != TransactionState::Active {
            return Err(PostgreSQLError::transaction_error("Transaction is not active"));
        }
        
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::transaction_error("Failed to acquire connection lock")
        })?;
        
        let sql = if read_only {
            "SET TRANSACTION READ ONLY"
        } else {
            "SET TRANSACTION READ WRITE"
        };
        
        conn.exec(sql)
            .map_err(|e| PostgreSQLError::transaction_error(&format!("Failed to set transaction mode: {}", e)))?;
        
        self.metadata.read_only = read_only;
        Ok(())
    }
    
    /// slay Set transaction isolation level
    pub fn set_isolation_level(&mut self, level: SqlIsolationLevel) -> Result<(), PostgreSQLError> {
        if self.state != TransactionState::Active {
            return Err(PostgreSQLError::transaction_error("Transaction is not active"));
        }
        
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::transaction_error("Failed to acquire connection lock")
        })?;
        
        let level_sql = match level {
            SqlIsolationLevel::LevelReadUncommitted => "SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED",
            SqlIsolationLevel::LevelReadCommitted => "SET TRANSACTION ISOLATION LEVEL READ COMMITTED",
            SqlIsolationLevel::LevelRepeatableRead => "SET TRANSACTION ISOLATION LEVEL REPEATABLE READ",
            SqlIsolationLevel::LevelSerializable => "SET TRANSACTION ISOLATION LEVEL SERIALIZABLE",
            _ => return Err(PostgreSQLError::transaction_error("Unsupported isolation level")),
        };
        
        conn.exec(level_sql)
            .map_err(|e| PostgreSQLError::transaction_error(&format!("Failed to set isolation level: {}", e)))?;
        
        self.metadata.isolation_level = level;
        Ok(())
    }
}

impl DriverTx for PostgreSQLTransaction {
    /// slay Prepare a statement within this transaction
    fn prepare(&self, query: &str) -> Result<Box<dyn DriverStmt>, DatabaseError> {
        if self.state != TransactionState::Active {
            return Err(DatabaseError::transaction_error("Transaction is not active"));
        }
        
        PostgreSQLStatement::new(self.conn.clone(), query, &self.config)
            .map(|stmt| Box::new(stmt) as Box<dyn DriverStmt>)
            .map_err(|e| e.into())
    }
    
    /// slay Execute query that returns rows within this transaction
    fn query(&self, query: &str, args: &[SqlValue]) -> Result<QueryResult, DatabaseError> {
        let pg_result = self.execute_in_transaction(query, args, true)?;
        self.convert_to_query_result(pg_result)
            .map_err(|e| e.into())
    }
    
    /// slay Execute query that doesn't return rows within this transaction
    fn execute(&self, query: &str, args: &[SqlValue]) -> Result<ExecuteResult, DatabaseError> {
        let pg_result = self.execute_in_transaction(query, args, false)?;
        
        let affected_rows = pg_result.affected_rows();
        let last_insert_id = None; // PostgreSQL doesn't have a universal last insert ID
        
        Ok(ExecuteResult::new(last_insert_id, affected_rows))
    }
    
    /// slay Commit this transaction
    fn commit(&self) -> Result<(), DatabaseError> {
        if self.state != TransactionState::Active {
            return Err(DatabaseError::transaction_error("Transaction is not active"));
        }
        
        let conn = self.conn.lock().map_err(|_| {
            DatabaseError::transaction_error("Failed to acquire connection lock")
        })?;
        
        conn.exec("COMMIT")
            .map_err(|e| DatabaseError::transaction_error(&format!("Failed to commit transaction: {}", e)))?;
        
        // Update state and metadata
        let mut_self = unsafe { &mut *(self as *const _ as *mut Self) };
        mut_self.state = TransactionState::Committed;
        mut_self.metadata.duration = Some(
            std::time::SystemTime::now()
                .duration_since(mut_self.metadata.started_at)
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
        );
        
        Ok(())
    }
    
    /// slay Rollback this transaction
    fn rollback(&self) -> Result<(), DatabaseError> {
        if self.state != TransactionState::Active {
            return Err(DatabaseError::transaction_error("Transaction is not active"));
        }
        
        let conn = self.conn.lock().map_err(|_| {
            DatabaseError::transaction_error("Failed to acquire connection lock")
        })?;
        
        conn.exec("ROLLBACK")
            .map_err(|e| DatabaseError::transaction_error(&format!("Failed to rollback transaction: {}", e)))?;
        
        // Update state and metadata
        let mut_self = unsafe { &mut *(self as *const _ as *mut Self) };
        mut_self.state = TransactionState::RolledBack;
        mut_self.metadata.duration = Some(
            std::time::SystemTime::now()
                .duration_since(mut_self.metadata.started_at)
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
        );
        
        Ok(())
    }
    
    /// slay Get transaction options
    fn options(&self) -> &TxOptions {
        &self.options
    }
    
    /// slay Check if transaction is still active
    fn is_active(&self) -> bool {
        self.state == TransactionState::Active
    }
    
    /// slay Clone this transaction (not really possible, return error)
    fn clone(&self) -> Box<dyn DriverTx> {
        // Transactions cannot be cloned, return a dummy transaction that will fail
        Box::new(PostgreSQLTransaction {
            conn: self.conn.clone(),
            options: self.options.clone(),
            config: self.config.clone(),
            state: TransactionState::Error("Cannot clone transaction".to_string()),
            savepoints: vec![],
            metadata: TransactionMetadata::default(),
        })
    }
}

/// fr fr Transaction manager for coordinating multiple transactions
#[derive(Debug)]
pub struct TransactionManager {
    /// fr fr Active transactions by ID
    transactions: HashMap<String, Arc<Mutex<PostgreSQLTransaction>>>,
    /// fr fr Transaction counter for generating IDs
    counter: std::sync::atomic::AtomicU64,
}

impl TransactionManager {
    /// slay Create a new transaction manager
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            counter: std::sync::atomic::AtomicU64::new(0),
        }
    }
    
    /// slay Start a new managed transaction
    pub fn begin_transaction(
        &mut self,
        conn: Arc<Mutex<SafePGconn>>,
        options: TxOptions,
        config: &PostgreSQLConfig,
    ) -> Result<String, PostgreSQLError> {
        let tx = PostgreSQLTransaction::begin(conn, options, config)?;
        let tx_id = self.generate_tx_id();
        
        self.transactions.insert(tx_id.clone(), Arc::new(Mutex::new(tx)));
        Ok(tx_id)
    }
    
    /// slay Get managed transaction
    pub fn get_transaction(&self, tx_id: &str) -> Option<Arc<Mutex<PostgreSQLTransaction>>> {
        self.transactions.get(tx_id).cloned()
    }
    
    /// slay Commit managed transaction
    pub fn commit_transaction(&mut self, tx_id: &str) -> Result<(), PostgreSQLError> {
        if let Some(tx_arc) = self.transactions.remove(tx_id) {
            let tx = tx_arc.lock().map_err(|_| {
                PostgreSQLError::transaction_error("Failed to acquire transaction lock")
            })?;
            
            tx.commit().map_err(|e| PostgreSQLError::transaction_error(&e.to_string()))
        } else {
            Err(PostgreSQLError::transaction_error("Transaction not found"))
        }
    }
    
    /// slay Rollback managed transaction
    pub fn rollback_transaction(&mut self, tx_id: &str) -> Result<(), PostgreSQLError> {
        if let Some(tx_arc) = self.transactions.remove(tx_id) {
            let tx = tx_arc.lock().map_err(|_| {
                PostgreSQLError::transaction_error("Failed to acquire transaction lock")
            })?;
            
            tx.rollback().map_err(|e| PostgreSQLError::transaction_error(&e.to_string()))
        } else {
            Err(PostgreSQLError::transaction_error("Transaction not found"))
        }
    }
    
    /// slay Generate unique transaction ID
    fn generate_tx_id(&self) -> String {
        let counter = self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0));
        
        format!("tx_{}_{}", now.as_nanos(), counter)
    }
    
    /// slay Get all active transaction IDs
    pub fn active_transactions(&self) -> Vec<String> {
        self.transactions.keys().cloned().collect()
    }
    
    /// slay Clean up completed transactions
    pub fn cleanup(&mut self) {
        self.transactions.retain(|_, tx_arc| {
            if let Ok(tx) = tx_arc.lock() {
                tx.is_active()
            } else {
                false
            }
        });
    }
}

impl Default for TransactionManager {
    fn default() -> Self {
        Self::new()
    }
}
