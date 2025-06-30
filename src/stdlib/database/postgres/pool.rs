//! PostgreSQL connection pool implementation

use crate::error::CursedError;
use super::connection::PostgresConnection;
use std::collections::{VecDeque, HashMap};
use std::sync::{Arc, Mutex};

/// Result type for PostgreSQL pool operations
pub type PostgresPoolResult<T> = Result<T, CursedError>;

/// PostgreSQL connection pool
pub struct PostgresPool {
    config: PostgresPoolConfig,
    connections: Arc<Mutex<Vec<PostgresConnection>>>,
    available: Arc<Mutex<VecDeque<usize>>>,
    stats: Arc<Mutex<PostgresPoolStats>>,
}

/// PostgreSQL pool configuration
#[derive(Debug, Clone)]
pub struct PostgresPoolConfig {
    pub min_connections: u32,
    pub max_connections: u32,
    pub connection_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub connection_string: String,
}

/// PostgreSQL pool statistics
#[derive(Debug, Clone)]
pub struct PostgresPoolStats {
    pub total_connections: u32,
    pub active_connections: u32,
    pub idle_connections: u32,
    pub connections_created: u64,
    pub connections_closed: u64,
    pub connection_errors: u64,
}

impl PostgresPool {
    /// Create a new PostgreSQL connection pool
    pub fn new(config: PostgresPoolConfig) -> PostgresPoolResult<Self> {
        let pool = Self {
            config: config.clone(),
            connections: Arc::new(Mutex::new(Vec::new())),
            available: Arc::new(Mutex::new(VecDeque::new())),
            stats: Arc::new(Mutex::new(PostgresPoolStats::new())),
        };
        
        println!("🏊 Created PostgreSQL pool with min: {}, max: {}", 
                config.min_connections, config.max_connections);
        Ok(pool)
    }
    
    /// Initialize the pool with minimum connections
    pub fn initialize(&self) -> PostgresPoolResult<()> {
        let mut connections = self.connections.lock().unwrap();
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        for i in 0..self.config.min_connections {
            let mut conn = PostgresConnection::new(self.config.connection_string.clone());
            conn.connect()?;
            connections.push(conn);
            available.push_back(i as usize);
            stats.connections_created += 1;
            stats.total_connections += 1;
            stats.idle_connections += 1;
        }
        
        println!("🏊 Initialized PostgreSQL pool with {} connections", self.config.min_connections);
        Ok(())
    }
    
    /// Get a connection from the pool
    pub fn get_connection(&self) -> PostgresPoolResult<PostgresPooledConnection> {
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if let Some(index) = available.pop_front() {
            stats.active_connections += 1;
            stats.idle_connections -= 1;
            println!("🔗 Retrieved connection {} from pool", index);
            Ok(PostgresPooledConnection::new(index, Arc::clone(&self.available)))
        } else {
            // Try to create a new connection if under max limit
            let connections = self.connections.lock().unwrap();
            if connections.len() < self.config.max_connections as usize {
                drop(connections);
                self.create_new_connection()
            } else {
                Err(CursedError::runtime_error("Pool exhausted - no connections available"))
            }
        }
    }
    
    /// Create a new connection and add to pool
    fn create_new_connection(&self) -> PostgresPoolResult<PostgresPooledConnection> {
        let mut connections = self.connections.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        let index = connections.len();
        let mut conn = PostgresConnection::new(self.config.connection_string.clone());
        conn.connect()?;
        connections.push(conn);
        
        stats.connections_created += 1;
        stats.total_connections += 1;
        stats.active_connections += 1;
        
        println!("🔗 Created new connection {} for pool", index);
        Ok(PostgresPooledConnection::new(index, Arc::clone(&self.available)))
    }
    
    /// Get pool statistics
    pub fn stats(&self) -> PostgresPoolStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Close all connections in the pool
    pub fn close(&self) -> PostgresPoolResult<()> {
        let mut connections = self.connections.lock().unwrap();
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        for conn in connections.iter_mut() {
            let _ = conn.disconnect();
        }
        
        let closed_count = connections.len();
        connections.clear();
        available.clear();
        stats.connections_closed += closed_count as u64;
        stats.total_connections = 0;
        stats.active_connections = 0;
        stats.idle_connections = 0;
        
        println!("🏊 Closed PostgreSQL pool with {} connections", closed_count);
        Ok(())
    }
}

/// A pooled PostgreSQL connection
pub struct PostgresPooledConnection {
    index: usize,
    available_queue: Arc<Mutex<VecDeque<usize>>>,
    returned: bool,
}

impl PostgresPooledConnection {
    fn new(index: usize, available_queue: Arc<Mutex<VecDeque<usize>>>) -> Self {
        Self {
            index,
            available_queue,
            returned: false,
        }
    }
    
    /// Get the connection index
    pub fn index(&self) -> usize {
        self.index
    }
    
    /// Execute a query on this connection
    pub fn execute(&self, query: &str) -> PostgresPoolResult<super::connection::PostgresQueryResult> {
        // Stub implementation - would use actual connection
        println!("🔍 Executing query on pooled connection {}: {}", self.index, query);
        Ok(super::connection::PostgresQueryResult::new(1, Vec::new()))
    }
}

impl Drop for PostgresPooledConnection {
    fn drop(&mut self) {
        if !self.returned {
            let mut available = self.available_queue.lock().unwrap();
            available.push_back(self.index);
            self.returned = true;
            println!("🔄 Returned connection {} to pool", self.index);
        }
    }
}

impl Default for PostgresPoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 20,
            connection_timeout: 30,
            idle_timeout: 600,
            max_lifetime: 1800,
            connection_string: "postgresql://localhost:5432/postgres".to_string(),
        }
    }
}

impl PostgresPoolStats {
    pub fn new() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            idle_connections: 0,
            connections_created: 0,
            connections_closed: 0,
            connection_errors: 0,
        }
    }
}
