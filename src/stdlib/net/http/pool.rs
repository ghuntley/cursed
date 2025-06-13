/// Connection pool for HTTP client

use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::stdlib::net::socket::TcpSocket;
use crate::stdlib::net::error::{NetError, NetResult};

/// Connection pool for reusing TCP connections
#[derive(Debug)]
pub struct ConnectionPool {
    connections: HashMap<String, PooledConnection>,
    config: PoolConfig,
}

#[derive(Debug)]
struct PooledConnection {
    socket: TcpSocket,
    last_used: Instant,
    in_use: bool,
}

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub max_connections: usize,
    pub max_idle_time: Duration,
    pub connection_timeout: Duration,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            max_idle_time: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(30),
        }
    }
}

/// Pool statistics
#[derive(Debug)]
pub struct PoolStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub idle_connections: usize,
}

impl ConnectionPool {
    pub fn new() -> Self {
        Self::with_config(PoolConfig::default())
    }
    
    pub fn with_config(config: PoolConfig) -> Self {
        Self {
            connections: HashMap::new(),
            config,
        }
    }
    
    pub fn get_connection(&mut self, host: &str, port: u16) -> NetResult<TcpSocket> {
        let key = format!("{}:{}", host, port);
        
        // Try to reuse existing connection
        if let Some(pooled) = self.connections.get_mut(&key) {
            if !pooled.in_use && !self.is_expired(&pooled) {
                pooled.in_use = true;
                pooled.last_used = Instant::now();
                // In a real implementation, we'd return a reference or handle
                return TcpSocket::connect(&key);
            }
        }
        
        // Create new connection
        let socket = TcpSocket::connect_timeout(&key, self.config.connection_timeout)?;
        
        self.connections.insert(key, PooledConnection {
            socket: TcpSocket::new(), // Placeholder
            last_used: Instant::now(),
            in_use: true,
        });
        
        Ok(socket)
    }
    
    pub fn return_connection(&mut self, host: &str, port: u16) {
        let key = format!("{}:{}", host, port);
        if let Some(pooled) = self.connections.get_mut(&key) {
            pooled.in_use = false;
            pooled.last_used = Instant::now();
        }
    }
    
    pub fn cleanup_expired(&mut self) {
        self.connections.retain(|_, pooled| {
            !self.is_expired(pooled) || pooled.in_use
        });
    }
    
    pub fn stats(&self) -> PoolStats {
        let total = self.connections.len();
        let active = self.connections.values().filter(|c| c.in_use).count();
        let idle = total - active;
        
        PoolStats {
            total_connections: total,
            active_connections: active,
            idle_connections: idle,
        }
    }
    
    fn is_expired(&self, pooled: &PooledConnection) -> bool {
        pooled.last_used.elapsed() > self.config.max_idle_time
    }
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}
