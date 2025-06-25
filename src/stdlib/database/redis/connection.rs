/// Redis connection management and pooling
/// 
/// Provides connection pooling, health checking, and connection lifecycle management
/// for Redis operations with proper error handling and monitoring.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, warn};
use tokio::time::timeout;

use super::{DatabaseError, RedisConfig};
use crate::error::CursedError;

/// Redis connection pool
#[derive(Debug)]
pub struct RedisConnectionPool {
    config: RedisConfig,
    connections: Arc<Mutex<VecDeque<RedisConnection>>>,
    stats: Arc<Mutex<ConnectionPoolStats>>,
}

/// Individual Redis connection
#[derive(Debug)]
pub struct RedisConnection {
    id: u64,
    created_at: Instant,
    last_used: Instant,
    is_healthy: bool,
    config: RedisConfig,
}

/// Connection pool statistics
#[derive(Debug, Default)]
pub struct ConnectionPoolStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub idle_connections: usize,
    pub connections_created: u64,
    pub connections_destroyed: u64,
    pub connection_errors: u64,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
}

impl RedisConnectionPool {
    /// Create new connection pool
    #[instrument]
    pub async fn new(config: &RedisConfig) -> crate::error::Result<()> {
        info!("Creating Redis connection pool");
        
        let pool = Self {
            config: config.clone(),
            connections: Arc::new(Mutex::new(VecDeque::new())),
            stats: Arc::new(Mutex::new(ConnectionPoolStats::default())),
        };
        
        // Pre-create initial connections
        for _ in 0..2 {
            let connection = pool.create_connection().await?;
            pool.connections.lock().unwrap().push_back(connection);
        }
        
        debug!("Redis connection pool created successfully");
        Ok(pool)
    }
    
    /// Get connection from pool
    #[instrument(skip(self))]
    pub async fn get_connection(&self) -> crate::error::Result<()> {
        debug!("Getting connection from pool");
        
        // Try to get existing connection
        if let Some(mut connection) = self.connections.lock().unwrap().pop_front() {
            // Check if connection is still healthy
            if connection.is_healthy() && !connection.is_expired() {
                connection.last_used = Instant::now();
                
                // Update stats
                let mut stats = self.stats.lock().unwrap();
                stats.active_connections += 1;
                stats.idle_connections = stats.idle_connections.saturating_sub(1);
                
                return Ok(connection);
            } else {
                // Connection is unhealthy or expired, create new one
                warn!("Connection is unhealthy or expired, creating new one");
            }
        }
        
        // Create new connection if none available or healthy
        let connection = self.create_connection().await?;
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.active_connections += 1;
        stats.total_requests += 1;
        
        Ok(connection)
    }
    
    /// Return connection to pool
    #[instrument(skip(self, connection))]
    pub async fn return_connection(&self, connection: RedisConnection) -> crate::error::Result<()> {
        debug!(connection_id = connection.id, "Returning connection to pool");
        
        // Check if connection is still healthy
        if connection.is_healthy() {
            self.connections.lock().unwrap().push_back(connection);
            
            // Update stats
            let mut stats = self.stats.lock().unwrap();
            stats.active_connections = stats.active_connections.saturating_sub(1);
            stats.idle_connections += 1;
        } else {
            warn!(connection_id = connection.id, "Connection unhealthy, not returning to pool");
            
            // Update stats
            let mut stats = self.stats.lock().unwrap();
            stats.active_connections = stats.active_connections.saturating_sub(1);
            stats.connection_errors += 1;
        }
        
        Ok(())
    }
    
    /// Create new connection
    #[instrument(skip(self))]
    async fn create_connection(&self) -> crate::error::Result<()> {
        debug!("Creating new Redis connection");
        
        let connection = RedisConnection::new(&self.config).await?;
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.connections_created += 1;
        stats.total_connections += 1;
        
        info!(connection_id = connection.id, "New Redis connection created");
        Ok(connection)
    }
    
    /// Get pool statistics
    pub fn get_stats(&self) -> ConnectionPoolStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Health check for all connections
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> crate::error::Result<()> {
        debug!("Performing pool health check");
        
        let mut connections = self.connections.lock().unwrap();
        let mut healthy_connections = VecDeque::new();
        let mut unhealthy_count = 0;
        
        for connection in connections.drain(..) {
            if connection.health_check().await.is_ok() {
                healthy_connections.push_back(connection);
            } else {
                unhealthy_count += 1;
            }
        }
        
        *connections = healthy_connections;
        
        if unhealthy_count > 0 {
            warn!(unhealthy_count = unhealthy_count, "Removed unhealthy connections from pool");
        }
        
        debug!("Pool health check completed");
        Ok(())
    }
    
    /// Close all connections in pool
    #[instrument(skip(self))]
    pub async fn close(&self) -> crate::error::Result<()> {
        info!("Closing Redis connection pool");
        
        let mut connections = self.connections.lock().unwrap();
        
        for connection in connections.drain(..) {
            if let Err(e) = connection.close().await {
                error!(error = ?e, "CursedError closing connection");
            }
        }
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.total_connections = 0;
        stats.active_connections = 0;
        stats.idle_connections = 0;
        
        info!("Redis connection pool closed");
        Ok(())
    }
}

impl RedisConnection {
    /// Create new Redis connection
    #[instrument]
    pub async fn new(config: &RedisConfig) -> crate::error::Result<()> {
        let id = rand::random::<u64>();
        debug!(connection_id = id, "Creating new Redis connection");
        
        // Simulate connection establishment with timeout
        let connection_result = timeout(config.timeout, async {
            // Placeholder for actual Redis connection logic
            tokio::time::sleep(Duration::from_millis(10)).await;
            Ok(())
        }).await;
        
        match connection_result {
            Ok(Ok(())) => {
                info!(connection_id = id, "Redis connection established successfully");
                Ok(Self {
                    id,
                    created_at: Instant::now(),
                    last_used: Instant::now(),
                    is_healthy: true,
                    config: config.clone(),
                })
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Err(DatabaseError::Connection("Connection timeout".to_string()).into()),
        }
    }
    
    /// Check if connection is healthy
    pub fn is_healthy(&self) -> bool {
        self.is_healthy
    }
    
    /// Check if connection is expired
    pub fn is_expired(&self) -> bool {
        let idle_time = self.last_used.elapsed();
        idle_time > Duration::from_secs(300) // 5 minutes
    }
    
    /// Perform health check on connection
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> crate::error::Result<()> {
        debug!(connection_id = self.id, "Performing connection health check");
        
        // Simulate PING command
        let ping_result = timeout(Duration::from_secs(1), async {
            tokio::time::sleep(Duration::from_millis(1)).await;
            Ok(())
        }).await;
        
        match ping_result {
            Ok(Ok(())) => {
                debug!(connection_id = self.id, "Connection health check passed");
                Ok(())
            }
            Ok(Err(e)) => {
                error!(connection_id = self.id, error = ?e, "Connection health check failed");
                Err(e)
            }
            Err(_) => {
                error!(connection_id = self.id, "Connection health check timed out");
                Err(DatabaseError::Connection("Health check timeout".to_string()).into())
            }
        }
    }
    
    /// Execute Redis command on this connection
    #[instrument(skip(self))]
    pub async fn execute_command(&mut self, command: &str, args: &[&str]) -> crate::error::Result<()> {
        debug!(connection_id = self.id, command = command, "Executing Redis command");
        
        // Update last used time
        self.last_used = Instant::now();
        
        // Simulate command execution
        let result = timeout(self.config.command_timeout, async {
            tokio::time::sleep(Duration::from_millis(1)).await;
            Ok(format!("OK:{}", command))
        }).await;
        
        match result {
            Ok(Ok(response)) => {
                debug!(connection_id = self.id, command = command, "Command executed successfully");
                Ok(response)
            }
            Ok(Err(e)) => {
                error!(connection_id = self.id, command = command, error = ?e, "Command execution failed");
                self.is_healthy = false;
                Err(e)
            }
            Err(_) => {
                error!(connection_id = self.id, command = command, "Command execution timed out");
                self.is_healthy = false;
                Err(DatabaseError::Query("Command timeout".to_string()).into())
            }
        }
    }
    
    /// Close the connection
    #[instrument(skip(self))]
    pub async fn close(self) -> crate::error::Result<()> {
        info!(connection_id = self.id, "Closing Redis connection");
        
        // Simulate connection cleanup
        tokio::time::sleep(Duration::from_millis(1)).await;
        
        debug!(connection_id = self.id, "Redis connection closed");
        Ok(())
    }
    
    /// Get connection ID
    pub fn id(&self) -> u64 {
        self.id
    }
    
    /// Get connection age
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }
    
    /// Get time since last use
    pub fn idle_time(&self) -> Duration {
        self.last_used.elapsed()
    }
}

impl Clone for ConnectionPoolStats {
    fn clone(&self) -> Self {
        Self {
            total_connections: self.total_connections,
            active_connections: self.active_connections,
            idle_connections: self.idle_connections,
            connections_created: self.connections_created,
            connections_destroyed: self.connections_destroyed,
            connection_errors: self.connection_errors,
            total_requests: self.total_requests,
            successful_requests: self.successful_requests,
            failed_requests: self.failed_requests,
        }
    }
}
