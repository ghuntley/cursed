//! Database connection pool implementation

use crate::error::CursedError;
use super::driver::{Driver, DriverConn};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::stdlib::packages::IOError;

/// Result type for connection pool operations
pub type PoolResult<T> = Result<T, CursedError>;

/// Generic database connection pool
pub struct ConnectionPool {
    config: PoolConfig,
    connections: Arc<Mutex<Vec<PooledConnection>>>,
    available: Arc<Mutex<VecDeque<usize>>>,
    stats: Arc<Mutex<PoolStats>>,
    driver: Arc<dyn Driver>,
}

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub min_connections: u32,
    pub max_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub validation_query: Option<String>,
    pub test_on_borrow: bool,
    pub test_on_return: bool,
    pub test_while_idle: bool,
}

/// Connection pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total_connections: u32,
    pub active_connections: u32,
    pub idle_connections: u32,
    pub connections_created: u64,
    pub connections_closed: u64,
    pub connection_requests: u64,
    pub connection_timeouts: u64,
    pub validation_failures: u64,
}

/// Pooled database connection wrapper
pub struct PooledConnection {
    connection: Option<Box<dyn DriverConn>>,
    created_at: Instant,
    last_used: Instant,
    is_valid: bool,
    usage_count: u64,
}

/// Handle to a borrowed connection from the pool
pub struct PooledConnectionHandle {
    index: usize,
    pool: Arc<ConnectionPool>,
    returned: bool,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(driver: Arc<dyn Driver>, config: PoolConfig) -> PoolResult<Self> {
        let pool = Self {
            config,
            connections: Arc::new(Mutex::new(Vec::new())),
            available: Arc::new(Mutex::new(VecDeque::new())),
            stats: Arc::new(Mutex::new(PoolStats::new())),
            driver,
        };
        
        println!("🏊 Created connection pool for driver: {}", pool.driver.name());
        Ok(pool)
    }
    
    /// Initialize the pool with minimum connections
    pub fn initialize(&self) -> PoolResult<()> {
        let mut connections = self.connections.lock().unwrap();
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        for i in 0..self.config.min_connections {
            let conn = self.driver.connect()?;
            let pooled_conn = PooledConnection::new(conn);
            connections.push(pooled_conn);
            available.push_back(i as usize);
            
            stats.connections_created += 1;
            stats.total_connections += 1;
            stats.idle_connections += 1;
        }
        
        println!("🏊 Initialized pool with {} connections", self.config.min_connections);
        Ok(())
    }
    
    /// Get a connection from the pool
    pub fn get_connection(&self) -> PoolResult<PooledConnectionHandle> {
        let mut stats = self.stats.lock().unwrap();
        stats.connection_requests += 1;
        drop(stats);
        
        let start_time = Instant::now();
        
        loop {
            // Try to get an available connection
            if let Some(index) = self.try_get_available_connection()? {
                return Ok(PooledConnectionHandle::new(index, Arc::new(self.clone())));
            }
            
            // Try to create a new connection if under max limit
            if let Some(index) = self.try_create_connection()? {
                return Ok(PooledConnectionHandle::new(index, Arc::new(self.clone())));
            }
            
            // Check for timeout
            if start_time.elapsed() >= self.config.connection_timeout {
                let mut stats = self.stats.lock().unwrap();
                stats.connection_timeouts += 1;
                return Err(CursedError::runtime_error(&"Connection timeout".to_string()));
            }
            
            // Wait a bit before trying again
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    
    /// Try to get an available connection
    fn try_get_available_connection(&self) -> PoolResult<Option<usize>> {
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if let Some(index) = available.pop_front() {
            stats.active_connections += 1;
            stats.idle_connections -= 1;
            
            // Validate connection if configured
            if self.config.test_on_borrow {
                if !self.validate_connection(index)? {
                    self.remove_connection(index)?;
                    return Ok(None);
                }
            }
            
            println!("🔗 Retrieved connection {} from pool", index);
            return Ok(Some(index));
        }
        
        Ok(None)
    }
    
    /// Try to create a new connection
    fn try_create_connection(&self) -> PoolResult<Option<usize>> {
        let mut connections = self.connections.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if connections.len() < self.config.max_connections as usize {
            let conn = self.driver.connect()?;
            let pooled_conn = PooledConnection::new(conn);
            let index = connections.len();
            connections.push(pooled_conn);
            
            stats.connections_created += 1;
            stats.total_connections += 1;
            stats.active_connections += 1;
            
            println!("🔗 Created new connection {} for pool", index);
            return Ok(Some(index));
        }
        
        Ok(None)
    }
    
    /// Return a connection to the pool
    fn return_connection(&self, index: usize) -> PoolResult<()> {
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        // Validate connection if configured
        if self.config.test_on_return {
            if !self.validate_connection(index)? {
                self.remove_connection(index)?;
                return Ok(());
            }
        }
        
        // Update connection usage
        let mut connections = self.connections.lock().unwrap();
        if let Some(conn) = connections.get_mut(index) {
            conn.last_used = Instant::now();
            conn.usage_count += 1;
        }
        drop(connections);
        
        available.push_back(index);
        stats.active_connections -= 1;
        stats.idle_connections += 1;
        
        println!("🔄 Returned connection {} to pool", index);
        Ok(())
    }
    
    /// Validate a connection
    fn validate_connection(&self, index: usize) -> PoolResult<bool> {
        let connections = self.connections.lock().unwrap();
        if let Some(conn) = connections.get(index) {
            if let Some(ref validation_query) = self.config.validation_query {
                if let Some(ref db_conn) = conn.connection {
                    match db_conn.execute(validation_query) {
                        Ok(_) => return Ok(true),
                        Err(_) => {
                            let mut stats = self.stats.lock().unwrap();
                            stats.validation_failures += 1;
                            return Ok(false);
                        }
                    }
                }
            }
            return Ok(conn.is_valid);
        }
        Ok(false)
    }
    
    /// Remove a connection from the pool
    fn remove_connection(&self, index: usize) -> PoolResult<()> {
        let mut connections = self.connections.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if let Some(mut conn) = connections.get_mut(index) {
            if let Some(ref db_conn) = conn.connection {
                let _ = db_conn.close();
            }
            conn.connection = None;
            conn.is_valid = false;
            
            stats.connections_closed += 1;
            stats.total_connections -= 1;
        }
        
        println!("🗑️ Removed invalid connection {} from pool", index);
        Ok(())
    }
    
    /// Cleanup idle connections
    pub fn cleanup_idle_connections(&self) -> PoolResult<()> {
        let mut connections = self.connections.lock().unwrap();
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        let now = Instant::now();
        let mut to_remove = Vec::new();
        
        for &index in available.iter() {
            if let Some(conn) = connections.get(index) {
                if now.duration_since(conn.last_used) > self.config.idle_timeout ||
                   now.duration_since(conn.created_at) > self.config.max_lifetime {
                    to_remove.push(index);
                }
            }
        }
        
        for index in &to_remove {
            if let Some(pos) = available.iter().position(|&x| x == *index) {
                available.remove(pos);
            }
            
            if let Some(mut conn) = connections.get_mut(*index) {
                if let Some(ref db_conn) = conn.connection {
                    let _ = db_conn.close();
                }
                conn.connection = None;
                conn.is_valid = false;
                
                stats.connections_closed += 1;
                stats.total_connections -= 1;
                stats.idle_connections -= 1;
            }
        }
        
        if !to_remove.is_empty() {
            println!("🧹 Cleaned up {} idle connections", to_remove.len());
        }
        
        Ok(())
    }
    
    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Close all connections in the pool
    pub fn close(&self) -> PoolResult<()> {
        let mut connections = self.connections.lock().unwrap();
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        for conn in connections.iter_mut() {
            if let Some(ref db_conn) = conn.connection {
                let _ = db_conn.close();
            }
        }
        
        let closed_count = connections.len();
        connections.clear();
        available.clear();
        
        stats.connections_closed += closed_count as u64;
        stats.total_connections = 0;
        stats.active_connections = 0;
        stats.idle_connections = 0;
        
        println!("🏊 Closed connection pool with {} connections", closed_count);
        Ok(())
    }
}

// Clone implementation is needed for Arc wrapping
impl Clone for ConnectionPool {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            connections: Arc::clone(&self.connections),
            available: Arc::clone(&self.available),
            stats: Arc::clone(&self.stats),
            driver: Arc::clone(&self.driver),
        }
    }
}

impl PooledConnection {
    fn new(connection: Box<dyn DriverConn>) -> Self {
        let now = Instant::now();
        Self {
            connection: Some(connection),
            created_at: now,
            last_used: now,
            is_valid: true,
            usage_count: 0,
        }
    }
}

impl PooledConnectionHandle {
    fn new(index: usize, pool: Arc<ConnectionPool>) -> Self {
        Self {
            index,
            pool,
            returned: false,
        }
    }
    
    /// Get the connection index
    pub fn index(&self) -> usize {
        self.index
    }
    
    /// Execute a query on this connection
    pub fn execute(&self, query: &str) -> PoolResult<super::driver::DriverResult<Box<dyn super::driver::DatabaseResult>>> {
        println!("🔍 Executing query on pooled connection {}: {}", self.index, query);
        // In a real implementation, this would use the actual connection
        Err(CursedError::runtime_error(&"Not implemented".to_string()))
    }
}

impl Drop for PooledConnectionHandle {
    fn drop(&mut self) {
        if !self.returned {
            let _ = self.pool.return_connection(self.index);
            self.returned = true;
        }
    }
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 20,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600), // 10 minutes
            max_lifetime: Duration::from_secs(1800), // 30 minutes
            validation_query: Some("SELECT 1".to_string()),
            test_on_borrow: true,
            test_on_return: false,
            test_while_idle: true,
        }
    }
}

impl PoolStats {
    pub fn new() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            idle_connections: 0,
            connections_created: 0,
            connections_closed: 0,
            connection_requests: 0,
            connection_timeouts: 0,
            validation_failures: 0,
        }
    }
    
    /// Get connection utilization percentage
    pub fn utilization(&self) -> f64 {
        if self.total_connections == 0 {
            0.0
        } else {
            (self.active_connections as f64 / self.total_connections as f64) * 100.0
        }
    }
    
    /// Check if pool is healthy
    pub fn is_healthy(&self) -> bool {
        self.connection_timeouts < 10 && self.validation_failures < 5
    }
}
