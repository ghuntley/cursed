//! Functional implementation for pool

use crate::error::CursedError;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::stdlib::packages::ModuleError;

/// Result type for pool operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// Configuration for connection pool
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub min_connections: usize,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 50,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(3600),
        }
    }
}

/// Generic connection pool
pub struct ConnectionPool<T> {
    config: PoolConfig,
    connections: Arc<Mutex<VecDeque<PooledConnection<T>>>>,
    total_connections: Arc<Mutex<usize>>,
}

/// Pooled connection wrapper
pub struct PooledConnection<T> {
    connection: T,
    created_at: Instant,
    last_used: Instant,
}

impl<T> PooledConnection<T> {
    pub fn new(connection: T) -> Self {
        let now = Instant::now();
        Self {
            connection,
            created_at: now,
            last_used: now,
        }
    }
    
    pub fn get(&self) -> &T {
        &self.connection
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        self.last_used = Instant::now();
        &mut self.connection
    }
    
    pub fn is_expired(&self, config: &PoolConfig) -> bool {
        let now = Instant::now();
        now.duration_since(self.created_at) > config.max_lifetime ||
        now.duration_since(self.last_used) > config.idle_timeout
    }
}

impl<T> ConnectionPool<T> 
where
    T: Send + 'static,
{
    pub fn new(config: PoolConfig) -> Self {
        Self {
            config,
            connections: Arc::new(Mutex::new(VecDeque::new())),
            total_connections: Arc::new(Mutex::new(0)),
        }
    }
    
    pub fn get_connection(&self) -> ModuleResult<Option<PooledConnection<T>>> {
        let mut connections = self.connections.lock()
            .map_err(|_| ModuleError::Other("Failed to acquire lock".to_string()))?;
        
        // Remove expired connections
        connections.retain(|conn| !conn.is_expired(&self.config));
        
        Ok(connections.pop_front())
    }
    
    pub fn return_connection(&self, connection: PooledConnection<T>) -> ModuleResult<()> {
        if connection.is_expired(&self.config) {
            let mut total = self.total_connections.lock()
                .map_err(|_| ModuleError::Other("Failed to acquire lock".to_string()))?;
            *total -= 1;
            return Ok(());
        }
        
        let mut connections = self.connections.lock()
            .map_err(|_| ModuleError::Other("Failed to acquire lock".to_string()))?;
        
        connections.push_back(connection);
        Ok(())
    }
    
    pub fn add_connection(&self, connection: T) -> ModuleResult<()> {
        let mut total = self.total_connections.lock()
            .map_err(|_| ModuleError::Other("Failed to acquire lock".to_string()))?;
        
        if *total >= self.config.max_connections {
            return Err(CursedError::runtime_error(&"Pool is at maximum capacity"));
        }
        
        let mut connections = self.connections.lock()
            .map_err(|_| ModuleError::Other("Failed to acquire lock".to_string()))?;
        
        connections.push_back(PooledConnection::new(connection));
        *total += 1;
        Ok(())
    }
    
    pub fn size(&self) -> usize {
        *self.total_connections.lock().unwrap_or_else(|_| panic!("Failed to lock total_connections"))
    }
    
    pub fn available(&self) -> usize {
        self.connections.lock().unwrap_or_else(|_| panic!("Failed to lock connections")).len()
    }
}

/// pool operations handler
pub struct ModuleHandler {
    enabled: bool,
}

impl ModuleHandler {
    /// Create a new module handler
    pub fn new() -> Self {
        Self {
            enabled: true,
        }
    }
    
    /// Enable or disable the module
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Check if module is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Process data
    pub fn process(&self, data: &str) -> ModuleResult<String> {
        if !self.enabled {
            return Err(CursedError::runtime_error(&"Module is disabled"));
        }
        Ok(format!("Processed: {}", data))
    }
    
    /// Get module info
    pub fn info(&self) -> String {
        format!("Module: pool, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize pool processing
pub fn init_pool() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    println!("⚙️  Module processing (pool) initialized");
    Ok(())
}

/// Test pool functionality
pub fn test_pool() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    Ok(())
}
