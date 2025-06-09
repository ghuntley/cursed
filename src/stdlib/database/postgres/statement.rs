/// PostgreSQL prepared statement implementation for CURSED database operations
/// 
/// This module provides prepared statement support with parameter binding,
/// type inference, and result set handling for PostgreSQL.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use super::super::{
    DriverStmt, DatabaseError, SqlValue, QueryResult, ExecuteResult
};
use super::{
    PostgreSQLError, PostgreSQLConfig, PostgreSQLType, PostgreSQLValue
};
use super::ffi::{SafePGconn, SafePGresult};
use super::types::type_utils;

/// fr fr PostgreSQL prepared statement implementation
#[derive(Debug)]
pub struct PostgreSQLStatement {
    /// fr fr Connection handle
    conn: Arc<Mutex<SafePGconn>>,
    /// fr fr Statement name in PostgreSQL
    stmt_name: String,
    /// fr fr Original query string
    query: String,
    /// fr fr Parameter types
    param_types: Vec<PostgreSQLType>,
    /// fr fr Parameter count
    param_count: usize,
    /// fr fr Configuration
    config: PostgreSQLConfig,
    /// fr fr Statement metadata
    metadata: StatementMetadata,
}

/// fr fr Statement metadata for optimization and caching
#[derive(Debug, Clone)]
pub struct StatementMetadata {
    /// fr fr Statement created time
    pub created_at: std::time::SystemTime,
    /// fr fr Number of times executed
    pub execution_count: u64,
    /// fr fr Total execution time
    pub total_execution_time: std::time::Duration,
    /// fr fr Average execution time
    pub avg_execution_time: std::time::Duration,
    /// fr fr Last execution time
    pub last_executed: Option<std::time::SystemTime>,
    /// fr fr Whether statement is prepared on server
    pub is_prepared: bool,
}

impl Default for StatementMetadata {
    fn default() -> Self {
        Self {
            created_at: std::time::SystemTime::now(),
            execution_count: 0,
            total_execution_time: std::time::Duration::from_secs(0),
            avg_execution_time: std::time::Duration::from_secs(0),
            last_executed: None,
            is_prepared: false,
        }
    }
}

impl PostgreSQLStatement {
    /// slay Create a new prepared statement
    pub fn new(
        conn: Arc<Mutex<SafePGconn>>,
        query: &str,
        config: &PostgreSQLConfig,
    ) -> Result<Self, PostgreSQLError> {
        let stmt_name = Self::generate_statement_name(query);
        let param_count = Self::count_parameters(query);
        let param_types = Self::infer_parameter_types(query, param_count)?;
        
        let mut stmt = Self {
            conn,
            stmt_name,
            query: query.to_string(),
            param_types,
            param_count,
            config: config.clone(),
            metadata: StatementMetadata::default(),
        };
        
        // Prepare the statement on the server
        stmt.prepare_on_server()?;
        
        Ok(stmt)
    }
    
    /// slay Generate unique statement name
    fn generate_statement_name(query: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        query.hash(&mut hasher);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0));
        
        format!("cursed_stmt_{}_{}", hasher.finish(), now.as_nanos())
    }
    
    /// slay Count parameters in query (looking for $1, $2, etc.)
    fn count_parameters(query: &str) -> usize {
        let mut max_param = 0;
        let mut chars = query.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '$' {
                let mut num_str = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                if let Ok(param_num) = num_str.parse::<usize>() {
                    max_param = max_param.max(param_num);
                }
            }
        }
        
        max_param
    }
    
    /// slay Infer parameter types from query context
    fn infer_parameter_types(query: &str, param_count: usize) -> Result<Vec<PostgreSQLType>, PostgreSQLError> {
        let mut types = vec![PostgreSQLType::Text; param_count]; // Default to text
        
        // Simple type inference based on SQL context
        let query_lower = query.to_lowercase();
        
        // Look for numeric contexts
        if query_lower.contains("where id = $") || query_lower.contains("limit $") {
            // Likely integer parameters
            for (i, param_type) in types.iter_mut().enumerate() {
                let param_ref = format!("${}", i + 1);
                if query_lower.contains(&format!("id = {}", param_ref)) ||
                   query_lower.contains(&format!("limit {}", param_ref)) {
                    *param_type = PostgreSQLType::Integer;
                }
            }
        }
        
        // Look for boolean contexts
        if query_lower.contains("where") && (query_lower.contains("true") || query_lower.contains("false")) {
            for (i, param_type) in types.iter_mut().enumerate() {
                let param_ref = format!("${}", i + 1);
                if query_lower.contains(&format!("= {}", param_ref)) &&
                   (query_lower.contains("is_") || query_lower.contains("active") || query_lower.contains("enabled")) {
                    *param_type = PostgreSQLType::Boolean;
                }
            }
        }
        
        // Look for timestamp contexts
        if query_lower.contains("created_at") || query_lower.contains("updated_at") ||
           query_lower.contains("timestamp") || query_lower.contains("datetime") {
            for (i, param_type) in types.iter_mut().enumerate() {
                let param_ref = format!("${}", i + 1);
                if query_lower.contains(&format!("created_at = {}", param_ref)) ||
                   query_lower.contains(&format!("updated_at = {}", param_ref)) {
                    *param_type = PostgreSQLType::Timestamptz;
                }
            }
        }
        
        Ok(types)
    }
    
    /// slay Prepare statement on PostgreSQL server
    fn prepare_on_server(&mut self) -> Result<(), PostgreSQLError> {
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::connection_error("Failed to acquire connection lock")
        })?;
        
        // Build PREPARE statement
        let prepare_sql = format!(
            "PREPARE {} AS {}",
            self.stmt_name,
            self.query
        );
        
        let result = conn.exec(&prepare_sql)
            .map_err(|e| PostgreSQLError::query_error(&format!("Failed to prepare statement: {}", e)))?;
        
        self.metadata.is_prepared = true;
        
        Ok(())
    }
    
    /// slay Execute statement with parameters
    fn execute_with_params(&self, args: &[SqlValue], return_rows: bool) -> Result<SafePGresult, PostgreSQLError> {
        if args.len() != self.param_count {
            return Err(PostgreSQLError::query_error(&format!(
                "Parameter count mismatch: expected {}, got {}",
                self.param_count,
                args.len()
            )));
        }
        
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::connection_error("Failed to acquire connection lock")
        })?;
        
        // Convert parameters to PostgreSQL format
        let pg_params = self.convert_params(args)?;
        
        // Build EXECUTE statement
        let param_placeholders: Vec<String> = (1..=args.len())
            .map(|i| format!("${}", i))
            .collect();
        
        let execute_sql = if args.is_empty() {
            format!("EXECUTE {}", self.stmt_name)
        } else {
            // For simplicity, we'll use text substitution
            // In production, you'd use PQexecPrepared with binary parameters
            let param_values: Result<Vec<String>, PostgreSQLError> = pg_params.iter()
                .map(|param| self.param_to_sql_string(param))
                .collect();
            
            let param_values = param_values?;
            let execute_sql = format!("EXECUTE {} ({})", self.stmt_name, param_values.join(", "));
            execute_sql
        };
        
        let start_time = std::time::Instant::now();
        
        let result = conn.exec(&execute_sql)
            .map_err(|e| PostgreSQLError::query_error(&format!("Failed to execute statement: {}", e)))?;
        
        let execution_time = start_time.elapsed();
        self.update_execution_stats(execution_time);
        
        Ok(result)
    }
    
    /// slay Convert SqlValue parameters to PostgreSQLValue
    fn convert_params(&self, args: &[SqlValue]) -> Result<Vec<PostgreSQLValue>, PostgreSQLError> {
        args.iter()
            .enumerate()
            .map(|(i, arg)| {
                let pg_type = self.param_types.get(i)
                    .unwrap_or(&PostgreSQLType::Text)
                    .clone();
                Ok(PostgreSQLValue::new(arg.clone(), pg_type))
            })
            .collect()
    }
    
    /// slay Convert PostgreSQL parameter to SQL string representation
    fn param_to_sql_string(&self, param: &PostgreSQLValue) -> Result<String, PostgreSQLError> {
        match &param.value {
            SqlValue::Null => Ok("NULL".to_string()),
            SqlValue::Boolean(b) => Ok(if *b { "TRUE".to_string() } else { "FALSE".to_string() }),
            SqlValue::Integer(i) => Ok(i.to_string()),
            SqlValue::Float(f) => Ok(f.to_string()),
            SqlValue::String(s) => {
                // Escape single quotes
                let escaped = s.replace("'", "''");
                Ok(format!("'{}'", escaped))
            }
            SqlValue::Bytes(b) => {
                // Convert to hex format for bytea
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
                
                // Convert to PostgreSQL timestamp format
                let secs = duration.as_secs();
                let timestamp_str = format!("to_timestamp({})", secs);
                Ok(timestamp_str)
            }
        }
    }
    
    /// slay Update execution statistics
    fn update_execution_stats(&self, execution_time: std::time::Duration) {
        // In a real implementation, you'd use atomic operations or proper synchronization
        // This is simplified for the example
    }
    
    /// slay Convert PostgreSQL result to QueryResult
    fn convert_to_query_result(&self, pg_result: SafePGresult) -> Result<QueryResult, PostgreSQLError> {
        let num_fields = pg_result.nfields();
        let num_tuples = pg_result.ntuples();
        
        let mut column_names = Vec::with_capacity(num_fields as usize);
        let mut column_types = Vec::with_capacity(num_fields as usize);
        
        for col in 0..num_fields {
            column_names.push(pg_result.field_name(col));
            let type_oid = pg_result.field_type(col);
            let pg_type = PostgreSQLType::from_oid(type_oid);
            column_types.push(pg_type.sql_name());
        }
        
        let mut rows = Vec::with_capacity(num_tuples as usize);
        
        for row in 0..num_tuples {
            let mut row_values = Vec::with_capacity(num_fields as usize);
            
            for col in 0..num_fields {
                if let Some(value_bytes) = pg_result.get_value(row, col) {
                    let type_oid = pg_result.field_type(col);
                    let pg_type = PostgreSQLType::from_oid(type_oid);
                    
                    match PostgreSQLValue::from_pg_bytes(&value_bytes, pg_type) {
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
    
    /// slay Get statement metadata
    pub fn metadata(&self) -> &StatementMetadata {
        &self.metadata
    }
    
    /// slay Check if statement is valid
    pub fn is_valid(&self) -> bool {
        self.metadata.is_prepared && self.conn.lock().map_or(false, |conn| conn.is_valid())
    }
    
    /// slay Deallocate statement on server
    pub fn deallocate(&self) -> Result<(), PostgreSQLError> {
        if !self.metadata.is_prepared {
            return Ok(()); // Nothing to deallocate
        }
        
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::connection_error("Failed to acquire connection lock")
        })?;
        
        let deallocate_sql = format!("DEALLOCATE {}", self.stmt_name);
        conn.exec(&deallocate_sql)
            .map_err(|e| PostgreSQLError::query_error(&format!("Failed to deallocate statement: {}", e)))?;
        
        Ok(())
    }
}

impl DriverStmt for PostgreSQLStatement {
    /// slay Execute statement that returns rows
    fn query(&self, args: &[SqlValue]) -> Result<QueryResult, DatabaseError> {
        let pg_result = self.execute_with_params(args, true)?;
        self.convert_to_query_result(pg_result)
            .map_err(|e| e.into())
    }
    
    /// slay Execute statement that doesn't return rows
    fn execute(&self, args: &[SqlValue]) -> Result<ExecuteResult, DatabaseError> {
        let pg_result = self.execute_with_params(args, false)?;
        
        let affected_rows = pg_result.affected_rows();
        let last_insert_id = None; // PostgreSQL doesn't have a universal last insert ID
        
        Ok(ExecuteResult::new(last_insert_id, affected_rows))
    }
    
    /// slay Close the statement
    fn close(&self) -> Result<(), DatabaseError> {
        self.deallocate().map_err(|e| e.into())
    }
    
    /// slay Get the original query string
    fn query_string(&self) -> &str {
        &self.query
    }
    
    /// slay Get parameter count
    fn parameter_count(&self) -> usize {
        self.param_count
    }
    
    /// slay Clone the statement
    fn clone(&self) -> Box<dyn DriverStmt> {
        match Self::new(self.conn.clone(), &self.query, &self.config) {
            Ok(new_stmt) => Box::new(new_stmt),
            Err(_) => {
                // Return a broken statement that will fail on use
                Box::new(PostgreSQLStatement {
                    conn: self.conn.clone(),
                    stmt_name: "broken".to_string(),
                    query: self.query.clone(),
                    param_types: vec![],
                    param_count: 0,
                    config: self.config.clone(),
                    metadata: StatementMetadata::default(),
                })
            }
        }
    }
}

impl Drop for PostgreSQLStatement {
    fn drop(&mut self) {
        // Attempt to deallocate the statement when dropped
        let _ = self.deallocate();
    }
}

/// fr fr Statement cache for connection-level statement management
#[derive(Debug)]
pub struct StatementCache {
    /// fr fr Cached statements by query hash
    statements: HashMap<String, Arc<PostgreSQLStatement>>,
    /// fr fr Maximum cache size
    max_size: usize,
    /// fr fr Cache hit statistics
    hits: u64,
    /// fr fr Cache miss statistics
    misses: u64,
}

impl StatementCache {
    /// slay Create a new statement cache
    pub fn new(max_size: usize) -> Self {
        Self {
            statements: HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    
    /// slay Get or create statement
    pub fn get_or_create(
        &mut self,
        query: &str,
        conn: Arc<Mutex<SafePGconn>>,
        config: &PostgreSQLConfig,
    ) -> Result<Arc<PostgreSQLStatement>, PostgreSQLError> {
        let query_hash = self.hash_query(query);
        
        if let Some(stmt) = self.statements.get(&query_hash) {
            if stmt.is_valid() {
                self.hits += 1;
                return Ok(stmt.clone());
            } else {
                // Remove invalid statement
                self.statements.remove(&query_hash);
            }
        }
        
        self.misses += 1;
        
        // Create new statement
        let stmt = Arc::new(PostgreSQLStatement::new(conn, query, config)?);
        
        // Evict old statements if cache is full
        if self.statements.len() >= self.max_size {
            self.evict_oldest();
        }
        
        self.statements.insert(query_hash, stmt.clone());
        Ok(stmt)
    }
    
    /// slay Hash query for cache key
    fn hash_query(&self, query: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        query.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    /// slay Evict oldest statement
    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self.statements.keys().next().cloned() {
            self.statements.remove(&oldest_key);
        }
    }
    
    /// slay Clear cache
    pub fn clear(&mut self) {
        self.statements.clear();
    }
    
    /// slay Get cache statistics
    pub fn stats(&self) -> (u64, u64, f64) {
        let total = self.hits + self.misses;
        let hit_rate = if total > 0 {
            self.hits as f64 / total as f64
        } else {
            0.0
        };
        (self.hits, self.misses, hit_rate)
    }
}

/// fr fr Statement builder for complex queries
#[derive(Debug, Clone)]
pub struct StatementBuilder {
    /// fr fr Base query
    query: String,
    /// fr fr Parameters
    parameters: Vec<SqlValue>,
    /// fr fr Parameter types
    parameter_types: Vec<PostgreSQLType>,
}

impl StatementBuilder {
    /// slay Create a new statement builder
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            parameters: Vec::new(),
            parameter_types: Vec::new(),
        }
    }
    
    /// slay Add parameter
    pub fn param(mut self, value: SqlValue, pg_type: PostgreSQLType) -> Self {
        self.parameters.push(value);
        self.parameter_types.push(pg_type);
        self
    }
    
    /// slay Add typed parameter
    pub fn typed_param<T>(self, value: T) -> Self
    where
        T: Into<SqlValue>,
    {
        let sql_value = value.into();
        let pg_type = type_utils::infer_pg_value(sql_value.clone()).pg_type;
        self.param(sql_value, pg_type)
    }
    
    /// slay Build final query with parameter substitution
    pub fn build(&self) -> (String, Vec<SqlValue>) {
        (self.query.clone(), self.parameters.clone())
    }
    
    /// slay Execute as prepared statement
    pub fn execute_on(
        self,
        conn: Arc<Mutex<SafePGconn>>,
        config: &PostgreSQLConfig,
    ) -> Result<QueryResult, PostgreSQLError> {
        let stmt = PostgreSQLStatement::new(conn, &self.query, config)?;
        stmt.query(&self.parameters).map_err(|e| e.into())
    }
}
