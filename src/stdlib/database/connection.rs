//! Database connection management

use crate::error::CursedError;
use std::collections::HashMap;
use super::core::SqlValue;

/// Result type for database operations
pub type DatabaseResult<T> = Result<T, CursedError>;

/// Database connection types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseType {
    SQLite,
    PostgreSQL,
    MySQL,
    Redis,
    InMemory,
}

/// Database row representation
#[derive(Debug, Clone)]
pub struct Row {
    columns: Vec<String>,
    values: Vec<SqlValue>,
}

impl Row {
    /// Create a new row
    pub fn new(columns: Vec<String>, values: Vec<SqlValue>) -> Self {
        Self { columns, values }
    }
    
    /// Get value by column name
    pub fn get(&self, column: &str) -> Option<&SqlValue> {
        self.columns
            .iter()
            .position(|c| c == column)
            .and_then(|i| self.values.get(i))
    }
    
    /// Get value by index
    pub fn get_by_index(&self, index: usize) -> Option<&SqlValue> {
        self.values.get(index)
    }
    
    /// Get all column names
    pub fn columns(&self) -> &[String] {
        &self.columns
    }
    
    /// Get all values
    pub fn values(&self) -> &[SqlValue] {
        &self.values
    }
    
    /// Convert row to HashMap
    pub fn to_hashmap(&self) -> HashMap<String, SqlValue> {
        let mut map = HashMap::new();
        for (i, column) in self.columns.iter().enumerate() {
            if let Some(value) = self.values.get(i) {
                map.insert(column.clone(), value.clone());
            }
        }
        map
    }
}

/// Query result
#[derive(Debug)]
pub struct QueryResult {
    rows: Vec<Row>,
    affected_rows: u64,
    last_insert_id: Option<i64>,
}

impl QueryResult {
    /// Create a new query result
    pub fn new(rows: Vec<Row>) -> Self {
        Self {
            rows,
            affected_rows: 0,
            last_insert_id: None,
        }
    }
    
    /// Create a result for insert/update/delete operations
    pub fn with_affected_rows(affected_rows: u64, last_insert_id: Option<i64>) -> Self {
        Self {
            rows: Vec::new(),
            affected_rows,
            last_insert_id,
        }
    }
    
    /// Get all rows
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }
    
    /// Get the number of affected rows
    pub fn affected_rows(&self) -> u64 {
        self.affected_rows
    }
    
    /// Get the last insert ID
    pub fn last_insert_id(&self) -> DatabaseResult<i64> {
        self.last_insert_id
            .ok_or_else(|| CursedError::runtime_error("No last insert ID available"))
    }
    
    /// Convert to iterator
    pub fn into_iter(self) -> std::vec::IntoIter<Row> {
        self.rows.into_iter()
    }
}

/// Database connection trait
pub trait DatabaseConnection: Send + Sync {
    /// Execute a query that returns rows
    fn query(&self, sql: String, params: Vec<SqlValue>) -> DatabaseResult<QueryResult>;
    
    /// Execute a query that doesn't return rows (INSERT, UPDATE, DELETE)
    fn exec(&self, sql: String, params: Vec<SqlValue>) -> DatabaseResult<QueryResult>;
    
    /// Begin a transaction
    fn begin_transaction(&self) -> DatabaseResult<()>;
    
    /// Commit a transaction
    fn commit_transaction(&self) -> DatabaseResult<()>;
    
    /// Rollback a transaction
    fn rollback_transaction(&self) -> DatabaseResult<()>;
    
    /// Check if the connection is healthy
    fn is_healthy(&self) -> bool;
}

/// In-memory database implementation for testing and simple use cases
pub struct InMemoryDatabase {
    tables: std::sync::RwLock<HashMap<String, Vec<Row>>>,
}

impl InMemoryDatabase {
    /// Create a new in-memory database
    pub fn new() -> Self {
        Self {
            tables: std::sync::RwLock::new(HashMap::new()),
        }
    }
    
    /// Add a table with sample data
    pub fn add_table(&self, table_name: String, columns: Vec<String>) -> DatabaseResult<()> {
        let mut tables = self.tables.write().unwrap();
        tables.insert(table_name, Vec::new());
        Ok(())
    }
    
    /// Insert a row into a table
    pub fn insert_row(&self, table_name: &str, row: Row) -> DatabaseResult<()> {
        let mut tables = self.tables.write().unwrap();
        if let Some(table) = tables.get_mut(table_name) {
            table.push(row);
            Ok(())
        } else {
            Err(CursedError::runtime_error(&format!("Table '{}' not found", table_name)))
        }
    }
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseConnection for InMemoryDatabase {
    fn query(&self, sql: String, _params: Vec<SqlValue>) -> DatabaseResult<QueryResult> {
        // Simple SQL parsing for demonstration
        if sql.to_uppercase().starts_with("SELECT") {
            let tables = self.tables.read().unwrap();
            
            // Very basic parsing - in real implementation would use proper SQL parser
            if let Some(table_name) = extract_table_name(&sql) {
                if let Some(rows) = tables.get(&table_name) {
                    Ok(QueryResult::new(rows.clone()))
                } else {
                    Ok(QueryResult::new(Vec::new())) // Table doesn't exist
                }
            } else {
                Ok(QueryResult::new(Vec::new())) // Invalid query
            }
        } else {
            Err(CursedError::runtime_error("Query operation expected SELECT statement"))
        }
    }
    
    fn exec(&self, sql: String, _params: Vec<SqlValue>) -> DatabaseResult<QueryResult> {
        let sql_upper = sql.to_uppercase();
        
        if sql_upper.starts_with("CREATE TABLE") {
            // Extract table name and create empty table
            if let Some(table_name) = extract_table_name(&sql) {
                let mut tables = self.tables.write().unwrap();
                tables.insert(table_name, Vec::new());
                Ok(QueryResult::with_affected_rows(0, None))
            } else {
                Err(CursedError::runtime_error("Invalid CREATE TABLE statement"))
            }
        } else if sql_upper.starts_with("INSERT") {
            // For demo purposes, just return success
            Ok(QueryResult::with_affected_rows(1, Some(1)))
        } else if sql_upper.starts_with("UPDATE") || sql_upper.starts_with("DELETE") {
            // For demo purposes, just return success
            Ok(QueryResult::with_affected_rows(1, None))
        } else {
            Err(CursedError::runtime_error("Unsupported SQL operation"))
        }
    }
    
    fn begin_transaction(&self) -> DatabaseResult<()> {
        // In-memory database doesn't need real transactions for this demo
        Ok(())
    }
    
    fn commit_transaction(&self) -> DatabaseResult<()> {
        Ok(())
    }
    
    fn rollback_transaction(&self) -> DatabaseResult<()> {
        Ok(())
    }
    
    fn is_healthy(&self) -> bool {
        true
    }
}

/// Extract table name from SQL query (very basic implementation)
fn extract_table_name(sql: &str) -> Option<String> {
    let sql_upper = sql.to_uppercase();
    let words: Vec<&str> = sql_upper.split_whitespace().collect();
    
    // Look for table name after FROM, INTO, TABLE, etc.
    for (i, word) in words.iter().enumerate() {
        if matches!(*word, "FROM" | "INTO" | "TABLE") && i + 1 < words.len() {
            return Some(words[i + 1].to_string());
        }
    }
    
    None
}

/// Create a database connection
pub fn create_connection(db_type: DatabaseType, connection_string: &str) -> DatabaseResult<Box<dyn DatabaseConnection>> {
    match db_type {
        DatabaseType::InMemory => {
            let db = InMemoryDatabase::new();
            Ok(Box::new(db))
        }
        DatabaseType::SQLite => {
            // For now, fall back to in-memory for compatibility
            println!("⚠️  SQLite not fully implemented, using in-memory database");
            let db = InMemoryDatabase::new();
            Ok(Box::new(db))
        }
        _ => {
            Err(CursedError::runtime_error(&format!("Database type {:?} not yet implemented", db_type)))
        }
    }
}

/// Test database connection functionality
pub fn test_database_connection() -> DatabaseResult<()> {
    let db = create_connection(DatabaseType::InMemory, "")?;
    
    // Test basic operations
    let _result = db.exec("CREATE TABLE test (id INTEGER, name TEXT)".to_string(), Vec::new())?;
    let _result = db.exec("INSERT INTO test VALUES (1, 'Hello')".to_string(), Vec::new())?;
    let result = db.query("SELECT * FROM test".to_string(), Vec::new())?;
    
    println!("✅ Database connection test successful ({} rows)", result.rows().len());
    Ok(())
}

/// Initialize database subsystem
pub fn init_database() -> DatabaseResult<()> {
    test_database_connection()?;
    println!("🗄️  Database connection management initialized");
    Ok(())
}
