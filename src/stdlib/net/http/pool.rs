use crate::error::CursedError;
/// Connection pool for HTTP client

use std::collections::HashMap;
use std::time::{Duration, Instant};
// use crate::stdlib::net::socket::TcpSocket;
// use crate::stdlib::net::error::{NetError, NetResult};

/// Connection pool for reusing TCP connections
#[derive(Debug)]
pub struct ConnectionPool {
#[derive(Debug)]
struct PooledConnection {
/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
impl Default for PoolConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Pool statistics
#[derive(Debug)]
pub struct PoolStats {
impl ConnectionPool {
    pub fn new() -> Self {
        Self::with_config(PoolConfig::default())
    pub fn with_config(config: PoolConfig) -> Self {
        Self {
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
        });
        
        Ok(socket)
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
    pub fn stats(&self) -> PoolStats {
        let total = self.connections.len();
        let active = self.connections.values().filter(|c| c.in_use).count();
        let idle = total - active;
        
        PoolStats {
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
