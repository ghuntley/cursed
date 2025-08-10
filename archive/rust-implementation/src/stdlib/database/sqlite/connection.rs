//! SQLite connection management and pooling

use crate::error::CursedError;
use super::{SqliteConfig, SqliteStats, SqliteError};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::stdlib::packages::IOError;

/// Result type for connection operations
pub type ConnectionResult<T> = Result<T, SqliteError>;

/// SQLite connection handle
#[derive(Debug)]
pub struct SqliteConnection {
    /// Connection ID
    pub id: u64,
    /// Database path
    pub path: String,
    /// Connection configuration
    pub config: SqliteConfig,
    /// Connection statistics
    pub stats: Arc<Mutex<SqliteStats>>,
    /// Is connection active
    pub active: bool,
    /// Prepared statement cache
    pub statement_cache: Arc<Mutex<HashMap<String, u64>>>,
}

impl SqliteConnection {
    /// Create a new SQLite connection
    pub fn new(path: &str, config: SqliteConfig) -> ConnectionResult<Self> {
        Ok(Self {
            id: 1, // Mock ID for now
            path: path.to_string(),
            config,
            stats: Arc::new(Mutex::new(SqliteStats::default())),
            active: true,
            statement_cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    
    /// Execute a SQL query
    pub fn execute(&self, sql: &str) -> ConnectionResult<u64> {
        if !self.active {
            return Err(SqliteError::connection_closed());
        }
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_queries += 1;
        }
        
        // Mock execution - return affected rows
        Ok(1)
    }
    
    /// Prepare a SQL statement
    pub fn prepare(&self, sql: &str) -> ConnectionResult<u64> {
        if !self.active {
            return Err(SqliteError::connection_closed());
        }
        
        let statement_id = 1; // Mock statement ID for now
        
        // Cache the prepared statement
        if let Ok(mut cache) = self.statement_cache.lock() {
            cache.insert(sql.to_string(), statement_id);
        }
        
        Ok(statement_id)
    }
    
    /// Begin a transaction
    pub fn begin_transaction(&self) -> ConnectionResult<()> {
        self.execute("BEGIN")?;
        Ok(())
    }
    
    /// Commit a transaction
    pub fn commit(&self) -> ConnectionResult<()> {
        self.execute("COMMIT")?;
        Ok(())
    }
    
    /// Rollback a transaction
    pub fn rollback(&self) -> ConnectionResult<()> {
        self.execute("ROLLBACK")?;
        Ok(())
    }
    
    /// Close the connection
    pub fn close(&mut self) -> ConnectionResult<()> {
        self.active = false;
        Ok(())
    }
    
    /// Get connection statistics
    pub fn stats(&self) -> SqliteStats {
        self.stats.lock().unwrap_or_else(|poisoned| {
            poisoned.into_inner()
        }).clone()
    }
}

/// SQLite connection builder for configuration
#[derive(Debug, Clone)]
pub struct SqliteConnectionBuilder {
    config: SqliteConfig,
}

impl SqliteConnectionBuilder {
    /// Create a new connection builder
    pub fn new() -> Self {
        Self {
            config: SqliteConfig::default(),
        }
    }
    
    /// Set database path
    pub fn path<P: AsRef<str>>(mut self, path: P) -> Self {
        self.config.database_path = path.as_ref().to_string();
        self
    }
    
    /// Enable WAL mode
    pub fn enable_wal(mut self, enable: bool) -> Self {
        self.config.enable_wal = enable;
        self
    }
    
    /// Set cache size
    pub fn cache_size(mut self, size: i32) -> Self {
        self.config.cache_size = size;
        self
    }
    
    /// Set connection timeout
    pub fn timeout_seconds(mut self, timeout: u32) -> Self {
        self.config.timeout_seconds = timeout;
        self
    }
    
    /// Enable foreign keys
    pub fn foreign_keys(mut self, enable: bool) -> Self {
        self.config.foreign_keys = enable;
        self
    }
    
    /// Build the connection
    pub fn build(self) -> ConnectionResult<SqliteConnection> {
        let path = self.config.database_path.clone();
        SqliteConnection::new(&path, self.config)
    }
}

impl Default for SqliteConnectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy compatibility functions
/// Initialize connection processing
pub fn init_connection() -> Result<(), CursedError> {
    println!("📁 SQLite connection system initialized");
    Ok(())
}

/// Test connection functionality
pub fn test_connection() -> Result<(), CursedError> {
    let config = SqliteConfig::default();
    let connection = SqliteConnection::new(":memory:", config);
    match connection {
        Ok(_) => Ok(()),
        Err(e) => Err(CursedError::runtime_error(&format!("Connection test failed: {}", e))),
    }
}
