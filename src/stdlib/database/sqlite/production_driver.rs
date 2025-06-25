/// Production-ready SQLite driver implementation for CURSED
/// 
/// This module provides a complete, production-ready SQLite driver that replaces
/// all placeholder implementations with real functionality. It includes:
/// - Full connection management with pooling
/// - Complete prepared statement support  
/// - Transaction management with savepoints
/// - Type-safe parameter binding and result processing
/// - Comprehensive error handling
/// - Thread safety and concurrent access
/// - Performance optimizations and monitoring

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::time::{SystemTime, Duration, Instant};
use std::thread;
use rusqlite::{Connection, OpenFlags, Statement, Transaction, Savepoint, types::Value as SqliteValue, params};
use super::{SqliteError, SqliteResult, SqliteConfig};
use super::super::{
    DriverStmt, DriverTx, SqlIsolationLevel
// };
use crate::error::CursedError;
use super::super::driver::{QueryResult, ExecuteResult, ConnectionMetadata};

/// Production SQLite connection with full functionality
#[derive(Debug)]
pub struct ProductionSqliteConnection {
    /// The underlying rusqlite connection
    /// Connection configuration
    /// Unique connection identifier
    /// When this connection was created
    /// Connection statistics
    /// Prepared statement cache
    /// Connection pool if this is part of a pool
    /// Whether this connection is in a transaction
    /// Transaction savepoint stack
/// Connection statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct ConnectionStats {
    /// Total queries executed
    /// Total statements prepared
    /// Total transactions committed
    /// Total transactions rolled back
    /// Total time spent executing queries
    /// Last activity timestamp
    /// Connection errors encountered
    /// Cache hits for prepared statements
    /// Cache misses for prepared statements
/// Cached prepared statement
#[derive(Debug)]
struct CachedStatement {
    /// The query string
    /// When this was cached
    /// How many times it's been used
    /// Last time it was used
/// Information about connection pooling
#[derive(Debug, Clone)]
struct PoolInfo {
impl ProductionSqliteConnection {
    /// Create new production SQLite connection
    pub fn new(config: SqliteConfig) -> SqliteResult<Self> {
        let flags = Self::build_open_flags(&config);
        
        let connection = Connection::open_with_flags(&config.database_path, flags)
            .map_err(|e| SqliteError::connection(&format!("Failed to open SQLite database '{}': {}", config.database_path, e)))?;
        
        let conn = Self {

        // Initialize connection with configuration
        conn.initialize_connection()?;
        
        Ok(conn)
    /// Create connection with pool information
    pub fn new_pooled(config: SqliteConfig, pool_id: String, connection_index: usize) -> SqliteResult<Self> {
        let mut conn = Self::new(config)?;
        conn.pool_info = Some(PoolInfo {
            max_idle_time: Duration::from_secs(300), // 5 minutes default
        });
        Ok(conn)
    /// Build SQLite open flags from configuration
    fn build_open_flags(config: &SqliteConfig) -> OpenFlags {
        let mut flags = OpenFlags::SQLITE_OPEN_URI;
        
        if config.read_only {
            flags |= OpenFlags::SQLITE_OPEN_READ_ONLY;
        } else {
            flags |= OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE;
        // Thread safety configuration
        if config.enable_shared_cache {
            flags |= OpenFlags::SQLITE_OPEN_SHARED_CACHE;
        } else {
            flags |= OpenFlags::SQLITE_OPEN_PRIVATE_CACHE;
        // Use full mutex for thread safety
        flags |= OpenFlags::SQLITE_OPEN_FULL_MUTEX;
        
        flags
    /// Initialize connection with PRAGMA statements and configuration
    fn initialize_connection(&self) -> SqliteResult<()> {
        let initialization_sql = self.config.initialization_sql();
        
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            for statement in initialization_sql {
                if statement.trim().is_empty() {
                    continue;
                conn.execute(&statement, [])
                    .map_err(|e| SqliteError::execution(&format!("Failed to execute initialization SQL '{}': {}", statement, e)))?;
            // Set additional performance and safety pragmas
            self.set_performance_pragmas(conn)?;
        Ok(())
    /// Set performance and safety PRAGMA statements
    fn set_performance_pragmas(&self, conn: &Connection) -> SqliteResult<()> {
        let pragmas = vec![
            // Enable foreign key constraints
            // Optimize for better performance
            // Set reasonable timeouts
            // Enable automatic indexing
            // Set memory limits
            "PRAGMA cache_size = -16384", // 16MB cache
        ];

        for pragma in pragmas {
            conn.execute(pragma, [])
                .map_err(|e| SqliteError::execution(&format!("Failed to set pragma '{}': {}", pragma, e)))?;
        Ok(())
    /// Update connection statistics
    fn update_stats<F>(&self, updater: F) -> SqliteResult<()>
    where
    {
        if let Ok(mut stats) = self.stats.lock() {
            updater(&mut stats);
            stats.last_activity = SystemTime::now();
        }
        Ok(())
    /// Get cached statement or prepare new one
    fn get_or_prepare_statement(&self, query: &str) -> SqliteResult<()> {
        let mut cache = self.statement_cache.lock().unwrap();
        
        if cache.contains_key(query) {
            // Update cache statistics
            if let Some(cached) = cache.get_mut(query) {
                cached.use_count += 1;
                cached.last_used = Instant::now();
            self.update_stats(|stats| stats.cache_hits += 1)?;
        } else {
            // Prepare new statement and cache it
            cache.insert(query.to_string(), CachedStatement {
            });
            
            self.update_stats(|stats| {
                stats.statements_prepared += 1;
                stats.cache_misses += 1;
            })?;
        Ok(())
    /// Clean up old cached statements
    fn cleanup_statement_cache(&self) -> SqliteResult<()> {
        let mut cache = self.statement_cache.lock().unwrap();
        let now = Instant::now();
        let max_age = Duration::from_secs(300); // 5 minutes
        
        cache.retain(|_, cached| {
            now.duration_since(cached.last_used) < max_age
        });
        
        Ok(())
    /// Execute query with timing and error handling
    fn execute_with_timing<F, R>(&self, operation: F) -> crate::error::Result<()>
    where
    {
        let start = Instant::now();
        let result = operation();
        let elapsed = start.elapsed();
        
        match &result {
            Ok(_) => {
                let _ = self.update_stats(|stats| {
                    stats.queries_executed += 1;
                    stats.total_query_time += elapsed;
                });
            }
            Err(_) => {
                let _ = self.update_stats(|stats| {
                    stats.error_count += 1;
                });
            }
        }
        
        result
    /// Get connection statistics
    pub fn get_stats(&self) -> SqliteResult<ConnectionStats> {
        let stats = self.stats.lock().unwrap();
        Ok(stats.clone())
    /// Get connection ID
    pub fn connection_id(&self) -> &str {
        &self.connection_id
    /// Check if connection is pooled
    pub fn is_pooled(&self) -> bool {
        self.pool_info.is_some()
    /// Get pool information
    pub fn pool_info(&self) -> Option<&PoolInfo> {
        self.pool_info.as_ref()
    /// Set busy timeout for the connection
    pub fn set_busy_timeout(&self, timeout: Duration) -> SqliteResult<()> {
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            conn.busy_timeout(timeout)
                .map_err(|e| SqliteError::execution(&format!("Failed to set busy timeout: {}", e)))?;
        }
        Ok(())
    /// Get database file size
    pub fn get_database_size(&self) -> SqliteResult<u64> {
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            let mut stmt = conn.prepare("PRAGMA page_count")
                .map_err(|e| SqliteError::execution(&format!("Failed to prepare page count query: {}", e)))?;
            
            let page_count: i64 = stmt.query_row([], |row| row.get(0))
                .map_err(|e| SqliteError::execution(&format!("Failed to get page count: {}", e)))?;
            
            let mut stmt = conn.prepare("PRAGMA page_size")
                .map_err(|e| SqliteError::execution(&format!("Failed to prepare page size query: {}", e)))?;
            
            let page_size: i64 = stmt.query_row([], |row| row.get(0))
                .map_err(|e| SqliteError::execution(&format!("Failed to get page size: {}", e)))?;
            
            Ok((page_count * page_size) as u64)
        } else {
            Err(SqliteError::connection("Connection is not available"))
        }
    }

    /// Vacuum the database
    pub fn vacuum(&self) -> SqliteResult<()> {
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            conn.execute("VACUUM", [])
                .map_err(|e| SqliteError::execution(&format!("Failed to vacuum database: {}", e)))?;
        }
        Ok(())
    /// Analyze the database for optimization
    pub fn analyze(&self) -> SqliteResult<()> {
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            conn.execute("ANALYZE", [])
                .map_err(|e| SqliteError::execution(&format!("Failed to analyze database: {}", e)))?;
        }
        Ok(())
    }
}

impl DriverConn for ProductionSqliteConnection {
    fn prepare(&self, query: &str) -> crate::error::Result<()> {
        self.execute_with_timing(|| {
            self.get_or_prepare_statement(query)
                .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &e.to_string()))?;
            
            let stmt = ProductionSqliteStatement::new(
                self.stats.clone()
            )?;
            
            Ok(Box::new(stmt) as Box<dyn DriverStmt>)
        })
    fn query(&self, query: &str, args: &[SqlValue]) -> crate::error::Result<()> {
        self.execute_with_timing(|| {
            let handle = self.connection.lock().unwrap();
            if let Some(ref conn) = *handle {
                let mut stmt = conn.prepare(query)
                    .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &format!("Failed to prepare query: {}", e)))?;
                
                // Get column information
                let column_names: Vec<String> = stmt.column_names().into_iter().map(|s| s.to_string()).collect();
                let column_types: Vec<String> = (0..stmt.column_count())
                    .map(|i| stmt.column_type(i).map(|t| format!("{:?}", t)).unwrap_or_else(|| "Unknown".to_string()))
                    .collect();
                
                // Convert arguments
                let params = convert_args_to_rusqlite_params(args)?;
                
                // Execute query
                let mut rows = stmt.query(rusqlite::params_from_iter(params.iter()))
                    .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &format!("Failed to execute query: {}", e)))?;
                
                let mut result_rows = Vec::new();
                
                while let Some(row) = rows.next()
                    .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &format!("Failed to fetch row: {}", e)))? {
                    
                    let mut values = Vec::new();
                    for i in 0..row.as_ref().column_count() {
                        let value = convert_rusqlite_value_to_sql_value(&row, i)?;
                        values.push(value);
                    }
                    result_rows.push(values);
                Ok(QueryResult {
                })
            } else {
                Err(DatabaseError::new(
                    "Connection is not available"
                ))
            }
        })
    fn execute(&self, query: &str, args: &[SqlValue]) -> crate::error::Result<()> {
        self.execute_with_timing(|| {
            let handle = self.connection.lock().unwrap();
            if let Some(ref conn) = *handle {
                let mut stmt = conn.prepare(query)
                    .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &format!("Failed to prepare statement: {}", e)))?;
                
                let params = convert_args_to_rusqlite_params(args)?;
                
                let changes = stmt.execute(rusqlite::params_from_iter(params.iter()))
                    .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &format!("Failed to execute statement: {}", e)))?;
                
                let last_insert_id = conn.last_insert_rowid();
                
                Ok(ExecuteResult {
                })
            } else {
                Err(DatabaseError::new(
                    "Connection is not available"
                ))
            }
        })
    fn begin_transaction(&self, opts: TxOptions) -> crate::error::Result<()> {
        let mut in_tx = self.in_transaction.lock().unwrap();
        if *in_tx {
            // Already in transaction, create savepoint
            let savepoint_name = format!("sp_{}", uuid::Uuid::new_v4().simple());
            let mut savepoints = self.savepoint_stack.lock().unwrap();
            savepoints.push(savepoint_name.clone());
            
            let tx = ProductionSqliteTransaction::new_savepoint(
                self.savepoint_stack.clone()
            )?;
            
            return Ok(Box::new(tx));
        *in_tx = true;
        
        let tx = ProductionSqliteTransaction::new(
            self.in_transaction.clone()
        )?;
        
        Ok(Box::new(tx))
    fn ping(&self) -> crate::error::Result<()> {
        self.execute_with_timing(|| {
            let handle = self.connection.lock().unwrap();
            if let Some(ref conn) = *handle {
                conn.execute("SELECT 1", [])
                    .map_err(|e| DatabaseError::new(DatabaseErrorKind::ConnectionError, &format!("Ping failed: {}", e)))?;
                Ok(())
            } else {
                Err(DatabaseError::new(
                    "Connection is not available"
                ))
            }
        })
    fn close(&self) -> crate::error::Result<()> {
        // Clean up cached statements
        let _ = self.cleanup_statement_cache();
        
        // Close connection
        let mut handle = self.connection.lock().unwrap();
        if let Some(conn) = handle.take() {
            drop(conn);
        }
        Ok(())
    fn is_alive(&self) -> bool {
        self.ping().is_ok()
    fn metadata(&self) -> ConnectionMetadata {
        let mut additional_info = HashMap::new();
        additional_info.insert("driver_name".to_string(), "ProductionSQLite".to_string());
        additional_info.insert("driver_version".to_string(), "1.0.0".to_string());
        additional_info.insert("connection_id".to_string(), self.connection_id.clone());
        additional_info.insert("is_read_only".to_string(), self.config.read_only.to_string());
        additional_info.insert("is_pooled".to_string(), self.is_pooled().to_string());
        
        if let Ok(stats) = self.get_stats() {
            additional_info.insert("queries_executed".to_string(), stats.queries_executed.to_string());
            additional_info.insert("statements_prepared".to_string(), stats.statements_prepared.to_string());
            additional_info.insert("cache_hit_ratio".to_string(), {
                let total = stats.cache_hits + stats.cache_misses;
                if total > 0 {
                    format!("{:.2}%", (stats.cache_hits as f64 / total as f64) * 100.0)
                } else {
                    "0.00%".to_string()
                }
            });
        ConnectionMetadata {
        }
    }

    fn clone(&self) -> Box<dyn DriverConn> {
        // For SQLite, create a new connection with the same configuration
        let new_conn = ProductionSqliteConnection::new(self.config.clone())
            .expect("Failed to clone SQLite connection");
        Box::new(new_conn)
    }
}

/// Production SQLite statement with full functionality
#[derive(Debug)]
pub struct ProductionSqliteStatement {
    /// The connection handle
    /// The SQL query
    /// Statistics for monitoring
    /// Statement ID for tracking
    /// Parameter count (cached)
impl ProductionSqliteStatement {
    pub fn new(
        stats: Arc<Mutex<ConnectionStats>>
    ) -> crate::error::Result<()> {
        Ok(Self {
        })
    /// Get parameter count by parsing the query
    fn get_parameter_count(&mut self) -> crate::error::Result<()> {
        if let Some(count) = self.parameter_count {
            return Ok(count);
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            let stmt = conn.prepare(&self.query)
                .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &format!("Failed to prepare statement for parameter count: {}", e)))?;
            
            let count = stmt.parameter_count();
            self.parameter_count = Some(count);
            Ok(count)
        } else {
            Err(DatabaseError::new(
                "Connection is not available"
            ))
        }
    }
impl DriverStmt for ProductionSqliteStatement {
    fn execute(&self, args: &[SqlValue]) -> crate::error::Result<()> {
        let start = Instant::now();
        
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            let mut stmt = conn.prepare(&self.query)
                .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &format!("Failed to prepare statement: {}", e)))?;
            
            let params = convert_args_to_rusqlite_params(args)?;
            
            let changes = stmt.execute(rusqlite::params_from_iter(params.iter()))
                .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &format!("Failed to execute statement: {}", e)))?;
            
            let last_insert_id = conn.last_insert_rowid();
            
            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.queries_executed += 1;
                stats.total_query_time += start.elapsed();
                stats.last_activity = SystemTime::now();
            Ok(ExecuteResult {
            })
        } else {
            Err(DatabaseError::new(
                "Connection is not available"
            ))
        }
    }

    fn query(&self, args: &[SqlValue]) -> crate::error::Result<()> {
        let start = Instant::now();
        
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            let mut stmt = conn.prepare(&self.query)
                .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &format!("Failed to prepare statement: {}", e)))?;
            
            // Get column information
            let column_names: Vec<String> = stmt.column_names().into_iter().map(|s| s.to_string()).collect();
            let column_types: Vec<String> = (0..stmt.column_count())
                .map(|i| stmt.column_type(i).map(|t| format!("{:?}", t)).unwrap_or_else(|| "Unknown".to_string()))
                .collect();
            
            let params = convert_args_to_rusqlite_params(args)?;
            
            let mut rows = stmt.query(rusqlite::params_from_iter(params.iter()))
                .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &format!("Failed to execute query: {}", e)))?;
            
            let mut result_rows = Vec::new();
            
            while let Some(row) = rows.next()
                .map_err(|e| DatabaseError::new(DatabaseErrorKind::QueryError, &format!("Failed to fetch row: {}", e)))? {
                
                let mut values = Vec::new();
                for i in 0..row.as_ref().column_count() {
                    let value = convert_rusqlite_value_to_sql_value(&row, i)?;
                    values.push(value);
                }
                result_rows.push(values);
            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.queries_executed += 1;
                stats.total_query_time += start.elapsed();
                stats.last_activity = SystemTime::now();
            Ok(QueryResult {
            })
        } else {
            Err(DatabaseError::new(
                "Connection is not available"
            ))
        }
    }

    fn close(&self) -> crate::error::Result<()> {
        // Statement cleanup is handled automatically by rusqlite
        Ok(())
    fn query_string(&self) -> &str {
        &self.query
    fn parameter_count(&self) -> usize {
        // Try to get cached parameter count, fall back to 0 if error
        if let Ok(mut stmt) = &mut { self.clone() } {
            stmt.get_parameter_count().unwrap_or(0)
        } else {
            0
        }
    }

    fn clone(&self) -> Box<dyn DriverStmt> {
        Box::new(ProductionSqliteStatement {
        })
    }
}

/// Production SQLite transaction with savepoint support
#[derive(Debug)]
pub struct ProductionSqliteTransaction {
    /// The connection handle
    /// Transaction options
    /// Statistics for monitoring
    /// Whether this is the main transaction or a savepoint
    /// Savepoint name if this is a savepoint
    /// Transaction state
    /// Savepoint stack
    /// Transaction ID
    /// When transaction started
impl ProductionSqliteTransaction {
    /// Create new main transaction
    pub fn new(
        in_transaction: Arc<Mutex<bool>>
    ) -> crate::error::Result<()> {
        // Begin transaction
        {
            let handle = connection.lock().unwrap();
            if let Some(ref conn) = *handle {
                let isolation_sql = match options.isolation {
                
                conn.execute(isolation_sql, [])
                    .map_err(|e| DatabaseError::new(DatabaseErrorKind::TransactionError, &format!("Failed to begin transaction: {}", e)))?;
            }
        }
        
        Ok(Self {
        })
    /// Create new savepoint transaction
    pub fn new_savepoint(
        savepoint_stack: Arc<Mutex<Vec<String>>>
    ) -> crate::error::Result<()> {
        // Create savepoint
        {
            let handle = connection.lock().unwrap();
            if let Some(ref conn) = *handle {
                conn.execute(&format!("SAVEPOINT {}", savepoint_name), [])
                    .map_err(|e| DatabaseError::new(DatabaseErrorKind::TransactionError, &format!("Failed to create savepoint: {}", e)))?;
            }
        }
        
        Ok(Self {
        })
    }
}

impl DriverTx for ProductionSqliteTransaction {
    fn commit(&self) -> crate::error::Result<()> {
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            if self.is_savepoint {
                if let Some(ref savepoint_name) = self.savepoint_name {
                    conn.execute(&format!("RELEASE SAVEPOINT {}", savepoint_name), [])
                        .map_err(|e| DatabaseError::new(DatabaseErrorKind::TransactionError, &format!("Failed to commit savepoint: {}", e)))?;
                    
                    // Remove from savepoint stack
                    if let Some(ref stack) = self.savepoint_stack {
                        let mut sp_stack = stack.lock().unwrap();
                        sp_stack.retain(|sp| sp != savepoint_name);
                    }
                }
            } else {
                conn.execute("COMMIT", [])
                    .map_err(|e| DatabaseError::new(DatabaseErrorKind::TransactionError, &format!("Failed to commit transaction: {}", e)))?;
                
                // Mark transaction as complete
                if let Some(ref in_tx) = self.in_transaction {
                    let mut tx_state = in_tx.lock().unwrap();
                    *tx_state = false;
                }
            }
            
            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.transactions_committed += 1;
                stats.last_activity = SystemTime::now();
            }
        }
        Ok(())
    fn rollback(&self) -> crate::error::Result<()> {
        let handle = self.connection.lock().unwrap();
        if let Some(ref conn) = *handle {
            if self.is_savepoint {
                if let Some(ref savepoint_name) = self.savepoint_name {
                    conn.execute(&format!("ROLLBACK TO SAVEPOINT {}", savepoint_name), [])
                        .map_err(|e| DatabaseError::new(DatabaseErrorKind::TransactionError, &format!("Failed to rollback savepoint: {}", e)))?;
                    
                    // Remove from savepoint stack
                    if let Some(ref stack) = self.savepoint_stack {
                        let mut sp_stack = stack.lock().unwrap();
                        sp_stack.retain(|sp| sp != savepoint_name);
                    }
                }
            } else {
                conn.execute("ROLLBACK", [])
                    .map_err(|e| DatabaseError::new(DatabaseErrorKind::TransactionError, &format!("Failed to rollback transaction: {}", e)))?;
                
                // Mark transaction as complete
                if let Some(ref in_tx) = self.in_transaction {
                    let mut tx_state = in_tx.lock().unwrap();
                    *tx_state = false;
                }
            }
            
            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.transactions_rolled_back += 1;
                stats.last_activity = SystemTime::now();
            }
        }
        Ok(())
    fn prepare(&self, query: &str) -> crate::error::Result<()> {
        let stmt = ProductionSqliteStatement::new(
            self.stats.clone()
        )?;
        Ok(Box::new(stmt))
    fn query(&self, query: &str, args: &[SqlValue]) -> crate::error::Result<()> {
        let stmt = self.prepare(query)?;
        stmt.query(args)
    fn execute(&self, query: &str, args: &[SqlValue]) -> crate::error::Result<()> {
        let stmt = self.prepare(query)?;
        stmt.execute(args)
    fn options(&self) -> &TxOptions {
        &self.options
    fn is_active(&self) -> bool {
        if self.is_savepoint {
            if let Some(ref stack) = self.savepoint_stack {
                if let Some(ref name) = self.savepoint_name {
                    let sp_stack = stack.lock().unwrap();
                    return sp_stack.contains(name);
                }
            }
        } else if let Some(ref in_tx) = self.in_transaction {
            let tx_state = in_tx.lock().unwrap();
            return *tx_state;
        }
        false
    fn clone(&self) -> Box<dyn DriverTx> {
        // Transactions cannot be truly cloned, return a new one with same options
        Box::new(ProductionSqliteTransaction {
        })
    }
}

/// Convert CURSED SqlValue arguments to rusqlite parameters
fn convert_args_to_rusqlite_params(args: &[SqlValue]) -> crate::error::Result<()> {
    let mut params = Vec::new();
    
    for arg in args {
        match arg {
            SqlValue::Timestamp(ts) => {
                // Convert SystemTime to ISO string representation
                let timestamp_str = format!("{:?}", ts);
                params.push(Box::new(timestamp_str) as Box<dyn rusqlite::ToSql>);
            SqlValue::Json(j) => {
                // Convert JSON to string representation
                let json_str = j.to_string();
                params.push(Box::new(json_str) as Box<dyn rusqlite::ToSql>);
            _ => return Err(DatabaseError::new(
                &format!("Unsupported SqlValue type for SQLite: {:?}", arg)
        }
    }
    
    Ok(params)
/// Convert rusqlite value to CURSED SqlValue
fn convert_rusqlite_value_to_sql_value(row: &rusqlite::Row, index: usize) -> crate::error::Result<()> {
    let value: SqliteValue = row.get(index)
        .map_err(|e| DatabaseError::new(DatabaseErrorKind::ConversionError, &format!("Failed to get column {}: {}", index, e)))?;
    
    match value {
    }
}

