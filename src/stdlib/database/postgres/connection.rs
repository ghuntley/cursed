/// PostgreSQL connection implementation for CURSED database operations
/// 
/// This module provides the main PostgreSQL connection implementation with
/// connection management, query execution, and transaction support.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use super::super::{
    DriverConn, DatabaseError, DatabaseErrorKind, SqlValue, TxOptions,
    QueryResult, ExecuteResult, ConnectionMetadata
};
use super::{
    PostgreSQLStatement, PostgreSQLTransaction, PostgreSQLError,
    PostgreSQLConfig, PostgreSQLType, PostgreSQLValue
};
use super::ffi::{SafePGconn, SafePGresult, ExecStatusType};
use super::types::type_utils;

/// fr fr PostgreSQL connection implementation
#[derive(Debug)]
pub struct PostgreSQLConnection {
    /// fr fr Raw PostgreSQL connection
    conn: Arc<Mutex<SafePGconn>>,
    /// fr fr Connection configuration
    config: PostgreSQLConfig,
    /// fr fr Connection metadata
    metadata: ConnectionMetadata,
    /// fr fr Type cache for custom types
    type_cache: Arc<Mutex<HashMap<u32, PostgreSQLType>>>,
    /// fr fr Prepared statement cache
    stmt_cache: Arc<Mutex<HashMap<String, String>>>,
    /// fr fr Connection statistics
    stats: Arc<Mutex<ConnectionStats>>,
}

/// fr fr Connection statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct ConnectionStats {
    /// fr fr Total queries executed
    pub queries_executed: u64,
    /// fr fr Total statements prepared
    pub statements_prepared: u64,
    /// fr fr Total transactions started
    pub transactions_started: u64,
    /// fr fr Connection created time
    pub created_at: std::time::SystemTime,
    /// fr fr Last activity time
    pub last_activity: std::time::SystemTime,
    /// fr fr Total errors encountered
    pub errors: u64,
}

impl PostgreSQLConnection {
    /// slay Create a new PostgreSQL connection
    pub fn new(conn_str: &str) -> Result<Self, PostgreSQLError> {
        let config = super::config::ConnectionString::parse(conn_str)?.to_config()?;
        Self::from_config(config)
    }
    
    /// slay Create connection from configuration
    pub fn from_config(config: PostgreSQLConfig) -> Result<Self, PostgreSQLError> {
        let conn_str = config.to_connection_string();
        let conn = SafePGconn::connect(&conn_str)
            .map_err(|e| PostgreSQLError::connection_error(&e))?;
        
        let db_info = conn.database_info();
        let metadata = ConnectionMetadata {
            server_version: Self::get_server_version(&conn)?,
            database_name: db_info.database,
            server_host: db_info.host,
            server_port: db_info.port,
            username: db_info.user,
            connected_at: std::time::SystemTime::now(),
            additional_info: HashMap::new(),
        };
        
        let mut stats = ConnectionStats::default();
        stats.created_at = std::time::SystemTime::now();
        stats.last_activity = stats.created_at;
        
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            config,
            metadata,
            type_cache: Arc::new(Mutex::new(HashMap::new())),
            stmt_cache: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(stats)),
        })
    }
    
    /// slay Get PostgreSQL server version
    fn get_server_version(conn: &SafePGconn) -> Result<String, PostgreSQLError> {
        let result = conn.exec("SELECT version()")
            .map_err(|e| PostgreSQLError::query_error(&e))?;
        
        if result.ntuples() > 0 {
            if let Some(version_bytes) = result.get_value(0, 0) {
                let version = String::from_utf8(version_bytes)
                    .map_err(|_| PostgreSQLError::query_error("Invalid server version encoding"))?;
                Ok(version)
            } else {
                Ok("Unknown".to_string())
            }
        } else {
            Ok("Unknown".to_string())
        }
    }
    
    /// slay Execute query and convert result
    fn execute_query_internal(&self, query: &str, params: &[SqlValue]) -> Result<SafePGresult, PostgreSQLError> {
        let conn = self.conn.lock().map_err(|_| PostgreSQLError::connection_error("Failed to acquire connection lock"))?;
        
        self.update_activity();
        
        if params.is_empty() {
            // Simple query without parameters
            conn.exec(query).map_err(|e| PostgreSQLError::query_error(&e))
        } else {
            // Parameterized query - use prepared statement
            let stmt_name = format!("stmt_{}", self.generate_stmt_name(query));
            
            // Prepare statement if not cached
            {
                let mut cache = self.stmt_cache.lock().map_err(|_| PostgreSQLError::query_error("Failed to acquire cache lock"))?;
                if !cache.contains_key(query) {
                    let prepare_result = conn.exec(&format!("PREPARE {} AS {}", stmt_name, query))
                        .map_err(|e| PostgreSQLError::query_error(&e))?;
                    
                    cache.insert(query.to_string(), stmt_name.clone());
                    
                    let mut stats = self.stats.lock().map_err(|_| PostgreSQLError::query_error("Failed to acquire stats lock"))?;
                    stats.statements_prepared += 1;
                }
            }
            
            // Convert parameters to PostgreSQL format
            let pg_params = self.convert_params_to_pg(params)?;
            
            // Execute prepared statement
            let param_strs: Vec<String> = pg_params.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect();
            let exec_query = format!("EXECUTE {} ({})", stmt_name, param_strs.join(", "));
            
            // For now, use simple execution - in production, you'd use PQexecPrepared
            conn.exec(&exec_query).map_err(|e| PostgreSQLError::query_error(&e))
        }
    }
    
    /// slay Convert SqlValue parameters to PostgreSQL format
    fn convert_params_to_pg(&self, params: &[SqlValue]) -> Result<Vec<PostgreSQLValue>, PostgreSQLError> {
        params.iter()
            .map(|param| Ok(type_utils::infer_pg_value(param.clone())))
            .collect()
    }
    
    /// slay Generate statement name from query
    fn generate_stmt_name(&self, query: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        query.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    /// slay Convert PostgreSQL result to QueryResult
    fn convert_result_to_query_result(&self, pg_result: SafePGresult) -> Result<QueryResult, PostgreSQLError> {
        let num_fields = pg_result.nfields();
        let num_tuples = pg_result.ntuples();
        
        // Get column information
        let mut column_names = Vec::with_capacity(num_fields as usize);
        let mut column_types = Vec::with_capacity(num_fields as usize);
        
        for col in 0..num_fields {
            column_names.push(pg_result.field_name(col));
            
            let type_oid = pg_result.field_type(col);
            let pg_type = self.get_type_from_oid(type_oid);
            column_types.push(pg_type.sql_name());
        }
        
        // Get row data
        let mut rows = Vec::with_capacity(num_tuples as usize);
        
        for row in 0..num_tuples {
            let mut row_values = Vec::with_capacity(num_fields as usize);
            
            for col in 0..num_fields {
                if let Some(value_bytes) = pg_result.get_value(row, col) {
                    let type_oid = pg_result.field_type(col);
                    let pg_type = self.get_type_from_oid(type_oid);
                    
                    match PostgreSQLValue::from_pg_bytes(&value_bytes, pg_type) {
                        Ok(pg_value) => row_values.push(pg_value.value),
                        Err(_) => {
                            // Fallback to string representation
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
    
    /// slay Get PostgreSQL type from OID
    fn get_type_from_oid(&self, oid: u32) -> PostgreSQLType {
        // Check cache first
        if let Ok(cache) = self.type_cache.lock() {
            if let Some(pg_type) = cache.get(&oid) {
                return pg_type.clone();
            }
        }
        
        // Get type from built-in types
        let pg_type = PostgreSQLType::from_oid(oid);
        
        // Cache the type
        if let Ok(mut cache) = self.type_cache.lock() {
            cache.insert(oid, pg_type.clone());
        }
        
        pg_type
    }
    
    /// slay Update last activity time
    fn update_activity(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.last_activity = std::time::SystemTime::now();
        }
    }
    
    /// slay Increment query counter
    fn increment_queries(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.queries_executed += 1;
        }
    }
    
    /// slay Increment error counter
    fn increment_errors(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.errors += 1;
        }
    }
    
    /// slay Get connection statistics
    pub fn stats(&self) -> Result<ConnectionStats, PostgreSQLError> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| PostgreSQLError::connection_error("Failed to acquire stats lock"))
    }
    
    /// slay Execute raw SQL command
    pub fn execute_raw(&self, sql: &str) -> Result<SafePGresult, PostgreSQLError> {
        let conn = self.conn.lock().map_err(|_| PostgreSQLError::connection_error("Failed to acquire connection lock"))?;
        
        self.update_activity();
        self.increment_queries();
        
        conn.exec(sql).map_err(|e| {
            self.increment_errors();
            PostgreSQLError::query_error(&e)
        })
    }
    
    /// slay Check if connection is in transaction
    pub fn in_transaction(&self) -> Result<bool, PostgreSQLError> {
        let result = self.execute_raw("SELECT current_setting('transaction_isolation')")?;
        // This is a simplified check - in reality, you'd check transaction status
        Ok(result.ntuples() > 0)
    }
    
    /// slay Set connection parameter
    pub fn set_parameter(&self, name: &str, value: &str) -> Result<(), PostgreSQLError> {
        let sql = format!("SET {} = '{}'", name, value);
        self.execute_raw(&sql)?;
        Ok(())
    }
    
    /// slay Get connection parameter
    pub fn get_parameter(&self, name: &str) -> Result<String, PostgreSQLError> {
        let sql = format!("SELECT current_setting('{}')", name);
        let result = self.execute_raw(&sql)?;
        
        if result.ntuples() > 0 {
            if let Some(value_bytes) = result.get_value(0, 0) {
                String::from_utf8(value_bytes)
                    .map_err(|_| PostgreSQLError::query_error("Invalid parameter encoding"))
            } else {
                Ok(String::new())
            }
        } else {
            Err(PostgreSQLError::query_error("Parameter not found"))
        }
    }
    
    /// slay Clear prepared statement cache
    pub fn clear_statement_cache(&self) -> Result<(), PostgreSQLError> {
        let mut cache = self.stmt_cache.lock().map_err(|_| PostgreSQLError::connection_error("Failed to acquire cache lock"))?;
        
        // Deallocate all prepared statements
        for (_, stmt_name) in cache.iter() {
            let deallocate_sql = format!("DEALLOCATE {}", stmt_name);
            let _ = self.execute_raw(&deallocate_sql); // Ignore errors
        }
        
        cache.clear();
        Ok(())
    }
}

impl DriverConn for PostgreSQLConnection {
    /// slay Prepare a statement
    fn prepare(&self, query: &str) -> Result<Box<dyn super::super::DriverStmt>, DatabaseError> {
        PostgreSQLStatement::new(self.conn.clone(), query, &self.config)
            .map(|stmt| Box::new(stmt) as Box<dyn super::super::DriverStmt>)
            .map_err(|e| e.into())
    }
    
    /// slay Execute query that returns rows
    fn query(&self, query: &str, args: &[SqlValue]) -> Result<QueryResult, DatabaseError> {
        let result = self.execute_query_internal(query, args)
            .map_err(|e| { self.increment_errors(); e })?;
        
        self.increment_queries();
        
        self.convert_result_to_query_result(result)
            .map_err(|e| e.into())
    }
    
    /// slay Execute query that doesn't return rows
    fn execute(&self, query: &str, args: &[SqlValue]) -> Result<ExecuteResult, DatabaseError> {
        let result = self.execute_query_internal(query, args)
            .map_err(|e| { self.increment_errors(); e })?;
        
        self.increment_queries();
        
        let affected_rows = result.affected_rows();
        let last_insert_id = None; // PostgreSQL doesn't have a universal last insert ID
        
        Ok(ExecuteResult::new(last_insert_id, affected_rows))
    }
    
    /// slay Begin a transaction
    fn begin_transaction(&self, opts: TxOptions) -> Result<Box<dyn super::super::DriverTx>, DatabaseError> {
        PostgreSQLTransaction::begin(self.conn.clone(), opts, &self.config)
            .map(|tx| {
                if let Ok(mut stats) = self.stats.lock() {
                    stats.transactions_started += 1;
                }
                Box::new(tx) as Box<dyn super::super::DriverTx>
            })
            .map_err(|e| e.into())
    }
    
    /// slay Ping the database
    fn ping(&self) -> Result<(), DatabaseError> {
        self.execute_raw("SELECT 1")
            .map(|_| ())
            .map_err(|e| e.into())
    }
    
    /// slay Close the connection
    fn close(&self) -> Result<(), DatabaseError> {
        // Clear prepared statement cache
        let _ = self.clear_statement_cache();
        
        // The connection will be automatically closed when dropped
        Ok(())
    }
    
    /// slay Check if connection is alive
    fn is_alive(&self) -> bool {
        if let Ok(conn) = self.conn.lock() {
            conn.is_valid()
        } else {
            false
        }
    }
    
    /// slay Get connection metadata
    fn metadata(&self) -> ConnectionMetadata {
        self.metadata.clone()
    }
    
    /// slay Clone the connection (creates a new connection with same config)
    fn clone(&self) -> Box<dyn DriverConn> {
        match Self::from_config(self.config.clone()) {
            Ok(new_conn) => Box::new(new_conn),
            Err(_) => {
                // Return a broken connection that will fail on use
                let broken_config = PostgreSQLConfig::default();
                Box::new(Self {
                    conn: Arc::new(Mutex::new(SafePGconn::connect("").unwrap())),
                    config: broken_config,
                    metadata: ConnectionMetadata::default(),
                    type_cache: Arc::new(Mutex::new(HashMap::new())),
                    stmt_cache: Arc::new(Mutex::new(HashMap::new())),
                    stats: Arc::new(Mutex::new(ConnectionStats::default())),
                })
            }
        }
    }
}

/// fr fr Connection pool-specific optimizations for PostgreSQL
impl PostgreSQLConnection {
    /// slay Reset connection state for pool reuse
    pub fn reset_for_pool_reuse(&self) -> Result<(), PostgreSQLError> {
        // Reset to default state
        self.execute_raw("RESET ALL")?;
        
        // Clear temporary tables
        self.execute_raw("DISCARD TEMP")?;
        
        // Clear prepared statements
        self.execute_raw("DISCARD PLANS")?;
        
        // Clear sequences
        self.execute_raw("DISCARD SEQUENCES")?;
        
        // Clear all
        self.execute_raw("DISCARD ALL")?;
        
        // Clear our internal caches
        self.clear_statement_cache()?;
        
        if let Ok(mut cache) = self.type_cache.lock() {
            cache.clear();
        }
        
        Ok(())
    }
    
    /// slay Check if connection can be reused in pool
    pub fn is_reusable(&self) -> bool {
        // Check if connection is valid and not in a transaction
        if !self.is_alive() {
            return false;
        }
        
        // Check transaction status
        match self.in_transaction() {
            Ok(in_tx) => !in_tx,
            Err(_) => false,
        }
    }
    
    /// slay Validate connection health for pool
    pub fn validate_health(&self) -> Result<(), PostgreSQLError> {
        // Quick health check
        self.execute_raw("SELECT 1")?;
        Ok(())
    }
}

/// fr fr PostgreSQL-specific connection extensions
impl PostgreSQLConnection {
    /// slay Get PostgreSQL server information
    pub fn server_info(&self) -> Result<HashMap<String, String>, PostgreSQLError> {
        let mut info = HashMap::new();
        
        // Get version
        let version_result = self.execute_raw("SELECT version()")?;
        if version_result.ntuples() > 0 {
            if let Some(version_bytes) = version_result.get_value(0, 0) {
                let version = String::from_utf8_lossy(&version_bytes);
                info.insert("version".to_string(), version.to_string());
            }
        }
        
        // Get encoding
        let encoding_result = self.execute_raw("SHOW client_encoding")?;
        if encoding_result.ntuples() > 0 {
            if let Some(encoding_bytes) = encoding_result.get_value(0, 0) {
                let encoding = String::from_utf8_lossy(&encoding_bytes);
                info.insert("client_encoding".to_string(), encoding.to_string());
            }
        }
        
        // Get timezone
        let tz_result = self.execute_raw("SHOW timezone")?;
        if tz_result.ntuples() > 0 {
            if let Some(tz_bytes) = tz_result.get_value(0, 0) {
                let timezone = String::from_utf8_lossy(&tz_bytes);
                info.insert("timezone".to_string(), timezone.to_string());
            }
        }
        
        Ok(info)
    }
    
    /// slay Listen for PostgreSQL notifications
    pub fn listen(&self, channel: &str) -> Result<(), PostgreSQLError> {
        let sql = format!("LISTEN {}", channel);
        self.execute_raw(&sql)?;
        Ok(())
    }
    
    /// slay Stop listening for PostgreSQL notifications
    pub fn unlisten(&self, channel: &str) -> Result<(), PostgreSQLError> {
        let sql = format!("UNLISTEN {}", channel);
        self.execute_raw(&sql)?;
        Ok(())
    }
    
    /// slay Send PostgreSQL notification
    pub fn notify(&self, channel: &str, payload: Option<&str>) -> Result<(), PostgreSQLError> {
        let sql = if let Some(payload) = payload {
            format!("NOTIFY {}, '{}'", channel, payload)
        } else {
            format!("NOTIFY {}", channel)
        };
        self.execute_raw(&sql)?;
        Ok(())
    }
}
