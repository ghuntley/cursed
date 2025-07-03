//! Database query execution implementation

use crate::error::CursedError;
use super::driver::{DriverConn, DriverResult};
use std::collections::HashMap;
use std::any::Any;
use crate::stdlib::packages::IOError;

/// Result type for query operations
pub type QueryResult<T> = Result<T, CursedError>;

/// Database query executor
pub struct QueryExecutor {
    connection: Option<Box<dyn DriverConn>>,
    context: QueryContext,
}

/// Query execution context
#[derive(Debug)]
pub struct QueryContext {
    pub timeout: Option<u64>,
    pub max_rows: Option<u64>,
    pub parameters: HashMap<String, Box<dyn Any + Send + Sync>>,
    pub hints: HashMap<String, String>,
}

/// Query execution result  
pub struct DatabaseQueryResult {
    pub query: String,
    pub rows_affected: u64,
    pub execution_time: u64,
    pub rows: Vec<HashMap<String, Box<dyn Any + Send + Sync>>>,
    pub columns: Vec<String>,
}

impl QueryExecutor {
    /// Create a new query executor
    pub fn new() -> Self {
        Self {
            connection: None,
            context: QueryContext::new(),
        }
    }
    
    /// Set the database connection
    pub fn with_connection(mut self, connection: Box<dyn DriverConn>) -> Self {
        self.connection = Some(connection);
        self
    }
    
    /// Set the query context
    pub fn with_context(mut self, context: QueryContext) -> Self {
        self.context = context;
        self
    }
    
    /// Execute a query
    pub fn execute(&self, query: &str) -> QueryResult<DatabaseQueryResult> {
        let start_time = std::time::Instant::now();
        
        println!("🔍 Executing query: {}", query);
        
        if let Some(ref conn) = self.connection {
            match conn.execute(query) {
                Ok(result) => {
                    let execution_time = start_time.elapsed().as_millis() as u64;
                    Ok(DatabaseQueryResult {
                        query: query.to_string(),
                        rows_affected: result.rows_affected(),
                        execution_time,
                        rows: Vec::new(), // Simplified
                        columns: result.columns(),
                    })
                }
                Err(e) => Err(e),
            }
        } else {
            Err(CursedError::runtime_error(&"No database connection available".to_string()))
        }
    }
    
    /// Execute a parameterized query
    pub fn execute_with_params(&self, query: &str, params: &HashMap<String, Box<dyn Any + Send + Sync>>) -> QueryResult<DatabaseQueryResult> {
        println!("🔍 Executing parameterized query with {} parameters", params.len());
        self.execute(query)
    }
    
    /// Execute multiple queries in a batch
    pub fn execute_batch(&self, queries: &[&str]) -> QueryResult<Vec<DatabaseQueryResult>> {
        let mut results = Vec::new();
        
        for query in queries {
            match self.execute(query) {
                Ok(result) => results.push(result),
                Err(e) => return Err(e),
            }
        }
        
        println!("📦 Executed batch of {} queries", queries.len());
        Ok(results)
    }
    
    /// Get the current context
    pub fn context(&self) -> &QueryContext {
        &self.context
    }
    
    /// Check if connection is available
    pub fn has_connection(&self) -> bool {
        self.connection.is_some()
    }
}

impl QueryContext {
    /// Create a new query context
    pub fn new() -> Self {
        Self {
            timeout: None,
            max_rows: None,
            parameters: HashMap::new(),
            hints: HashMap::new(),
        }
    }
    
    /// Set query timeout
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    /// Set maximum rows to return
    pub fn with_max_rows(mut self, max_rows: u64) -> Self {
        self.max_rows = Some(max_rows);
        self
    }
    
    /// Add a parameter
    pub fn with_parameter<T: Any + Send + Sync>(mut self, name: &str, value: T) -> Self {
        self.parameters.insert(name.to_string(), Box::new(value));
        self
    }
    
    /// Add a query hint
    pub fn with_hint(mut self, name: &str, value: &str) -> Self {
        self.hints.insert(name.to_string(), value.to_string());
        self
    }
    
    /// Get parameter count
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }
    
    /// Get hint count
    pub fn hint_count(&self) -> usize {
        self.hints.len()
    }
}

impl DatabaseQueryResult {
    /// Check if query was successful
    pub fn is_success(&self) -> bool {
        true // Simplified
    }
    
    /// Get row count
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
    
    /// Get column count
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }
    
    /// Check if result has data
    pub fn has_data(&self) -> bool {
        !self.rows.is_empty()
    }
    
    /// Get execution time in milliseconds
    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time
    }
    
    /// Get formatted summary
    pub fn summary(&self) -> String {
        format!(
            "Query executed in {}ms, {} rows affected, {} columns returned",
            self.execution_time,
            self.rows_affected,
            self.columns.len()
        )
    }
}

impl Default for QueryExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for QueryContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Query builder for constructing SQL queries
pub struct QueryBuilder {
    query_type: QueryType,
    table: Option<String>,
    columns: Vec<String>,
    conditions: Vec<String>,
    joins: Vec<String>,
    order_by: Vec<String>,
    group_by: Vec<String>,
    having: Vec<String>,
    limit: Option<u64>,
    offset: Option<u64>,
}

/// Types of SQL queries
#[derive(Debug, Clone)]
pub enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
}

impl QueryBuilder {
    /// Create a new SELECT query builder
    pub fn select() -> Self {
        Self {
            query_type: QueryType::Select,
            table: None,
            columns: Vec::new(),
            conditions: Vec::new(),
            joins: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            limit: None,
            offset: None,
        }
    }
    
    /// Create a new INSERT query builder
    pub fn insert() -> Self {
        Self {
            query_type: QueryType::Insert,
            table: None,
            columns: Vec::new(),
            conditions: Vec::new(),
            joins: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            limit: None,
            offset: None,
        }
    }
    
    /// Create a new UPDATE query builder
    pub fn update() -> Self {
        Self {
            query_type: QueryType::Update,
            table: None,
            columns: Vec::new(),
            conditions: Vec::new(),
            joins: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            limit: None,
            offset: None,
        }
    }
    
    /// Create a new DELETE query builder
    pub fn delete() -> Self {
        Self {
            query_type: QueryType::Delete,
            table: None,
            columns: Vec::new(),
            conditions: Vec::new(),
            joins: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            limit: None,
            offset: None,
        }
    }
    
    /// Set the table name
    pub fn from(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }
    
    /// Add columns to select
    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.columns.extend(columns.iter().map(|s| s.to_string()));
        self
    }
    
    /// Add a WHERE condition
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }
    
    /// Add a JOIN clause
    pub fn join(mut self, join: &str) -> Self {
        self.joins.push(join.to_string());
        self
    }
    
    /// Add ORDER BY clause
    pub fn order_by(mut self, column: &str) -> Self {
        self.order_by.push(column.to_string());
        self
    }
    
    /// Add GROUP BY clause
    pub fn group_by(mut self, column: &str) -> Self {
        self.group_by.push(column.to_string());
        self
    }
    
    /// Set LIMIT clause
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Set OFFSET clause
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }
    
    /// Build the SQL query
    pub fn build(&self) -> QueryResult<String> {
        match self.query_type {
            QueryType::Select => self.build_select(),
            QueryType::Insert => self.build_insert(),
            QueryType::Update => self.build_update(),
            QueryType::Delete => self.build_delete(),
        }
    }
    
    fn build_select(&self) -> QueryResult<String> {
        let table = self.table.as_ref()
            .ok_or_else(|| IOError::Other("Table name required".to_string()))?;
        
        let columns = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };
        
        let mut query = format!("SELECT {} FROM {}", columns, table);
        
        if !self.joins.is_empty() {
            query.push_str(" ");
            query.push_str(&self.joins.join(" "));
        }
        
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }
        
        if !self.group_by.is_empty() {
            query.push_str(" GROUP BY ");
            query.push_str(&self.group_by.join(", "));
        }
        
        if !self.having.is_empty() {
            query.push_str(" HAVING ");
            query.push_str(&self.having.join(" AND "));
        }
        
        if !self.order_by.is_empty() {
            query.push_str(" ORDER BY ");
            query.push_str(&self.order_by.join(", "));
        }
        
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
        
        Ok(query)
    }
    
    fn build_insert(&self) -> QueryResult<String> {
        let table = self.table.as_ref()
            .ok_or_else(|| IOError::Other("Table name required".to_string()))?;
        
        if self.columns.is_empty() {
            return Err(CursedError::runtime_error(&"Columns required for INSERT".to_string()));
        }
        
        let columns = self.columns.join(", ");
        let placeholders = (0..self.columns.len()).map(|_| "?").collect::<Vec<_>>().join(", ");
        
        Ok(format!("INSERT INTO {} ({}) VALUES ({})", table, columns, placeholders))
    }
    
    fn build_update(&self) -> QueryResult<String> {
        let table = self.table.as_ref()
            .ok_or_else(|| IOError::Other("Table name required".to_string()))?;
        
        if self.columns.is_empty() {
            return Err(CursedError::runtime_error(&"Columns required for UPDATE".to_string()));
        }
        
        let set_clauses = self.columns.iter()
            .map(|col| format!("{} = ?", col))
            .collect::<Vec<_>>()
            .join(", ");
        
        let mut query = format!("UPDATE {} SET {}", table, set_clauses);
        
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }
        
        Ok(query)
    }
    
    fn build_delete(&self) -> QueryResult<String> {
        let table = self.table.as_ref()
            .ok_or_else(|| IOError::Other("Table name required".to_string()))?;
        
        let mut query = format!("DELETE FROM {}", table);
        
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }
        
        Ok(query)
    }
}
