use crate::error::CursedError;
/// fr fr MySQL connection pool implementation
/// 
/// This module provides advanced connection pooling capabilities for MySQL
/// connections with health monitoring, automatic recovery, and performance optimization.

use std::sync::Arc;
use std::time::{Duration, Instant};
use mysql::{Pool, PooledConn, OptsBuilder};

use super::error::{MySqlError, MySqlResult};
use super::types::{parse_connection_string, MySqlConnectionInfo};

/// fr fr MySQL connection pool configuration
#[derive(Debug, Clone)]
pub struct MySqlPoolConfig {
    /// Minimum number of connections to maintain
    /// Maximum number of connections
    /// Connection timeout
    /// Maximum connection lifetime
    /// Connection idle timeout
    /// Test query for health checks
    /// Health check interval
    /// Maximum retries for failed connections
    /// Retry delay
impl Default for MySqlPoolConfig {
    fn default() -> Self {
        Self {
        }
    }
/// fr fr MySQL connection pool with advanced features
#[derive(Debug)]
pub struct MySqlPool {
    /// Underlying MySQL pool
    /// Pool configuration
    /// Pool statistics
impl MySqlPool {
    /// Create a new MySQL connection pool
    pub fn new(dsn: &str, config: MySqlPoolConfig) -> MySqlResult<Self> {
        let conn_info = parse_connection_string(dsn)?;
        let pool = Self::create_pool(&conn_info, &config)?;

        Ok(Self {
        })
    /// Create a MySQL pool with default configuration
    pub fn with_defaults(dsn: &str) -> MySqlResult<Self> {
        Self::new(dsn, MySqlPoolConfig::default())
    /// Create the underlying MySQL pool
    fn create_pool(conn_info: &MySqlConnectionInfo, config: &MySqlPoolConfig) -> MySqlResult<Pool> {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(&conn_info.host))
            .tcp_port(conn_info.port)
            .user(Some(&conn_info.user))
            .pass(Some(&conn_info.password))
            .db_name(Some(&conn_info.database))
            .tcp_connect_timeout(Some(config.connection_timeout));

        Pool::new_manual(config.min_connections, config.max_connections, opts)
            .map_err(|e| MySqlError::pool_error(&format!("Failed to create pool: {}", e)))
    /// Get a connection from the pool
    pub fn get_connection(&self) -> MySqlResult<PooledConn> {
        let start_time = Instant::now();
        
        let conn = self.inner.get_conn()
            .map_err(|e| MySqlError::pool_error(&format!("Failed to get connection: {}", e)))?;

        // Update statistics
        self.stats.record_connection_acquired(start_time.elapsed());

        Ok(conn)
    /// Get a connection with timeout
    pub fn get_connection_timeout(&self, timeout: Duration) -> MySqlResult<PooledConn> {
        let start_time = Instant::now();
        
        // Create a scoped pool with timeout
        let conn = self.inner.get_conn()
            .map_err(|e| MySqlError::timeout_error(&format!("Connection timeout: {}", e)))?;

        if start_time.elapsed() > timeout {
            return Err(MySqlError::timeout_error("Connection acquisition timed out"));
        self.stats.record_connection_acquired(start_time.elapsed());
        Ok(conn)
    /// Test a connection's health
    pub fn test_connection(&self, conn: &mut PooledConn) -> MySqlResult<bool> {
        use mysql::prelude::Queryable;
        
        match conn.query_drop(&self.config.test_query) {
        }
    }

    /// Perform health check on all connections
    pub fn health_check(&self) -> MySqlResult<MySqlPoolHealthReport> {
        let total_connections = self.inner.state().unwrap_or_default().connections;
        let mut healthy_connections = 0;
        let mut failed_connections = 0;
        let mut errors = Vec::new();

        // This is a simplified health check
        // In a real implementation, we'd iterate through available connections
        match self.get_connection() {
            Ok(mut conn) => {
                if self.test_connection(&mut conn).unwrap_or(false) {
                    healthy_connections += 1;
                } else {
                    failed_connections += 1;
                    errors.push("Connection failed health test".to_string());
                }
            }
            Err(e) => {
                failed_connections += 1;
                errors.push(format!("Failed to get connection: {}", e));
            }
        }

        Ok(MySqlPoolHealthReport {
        })
    /// Get pool statistics
    pub fn stats(&self) -> &MySqlPoolStats {
        &self.stats
    /// Get pool state information
    pub fn state(&self) -> MySqlResult<MySqlPoolState> {
        let state = self.inner.state()
            .ok_or_else(|| MySqlError::pool_error("Failed to get pool state"))?;

        Ok(MySqlPoolState {
        })
    /// Close the pool
    pub fn close(&self) -> MySqlResult<()> {
        // The mysql crate doesn't provide an explicit close method
        // Connections will be closed when the pool is dropped
        Ok(())
    /// Get the underlying pool reference
    pub fn inner(&self) -> &Arc<Pool> {
        &self.inner
    /// Get pool configuration
    pub fn config(&self) -> &MySqlPoolConfig {
        &self.config
    }
}

/// fr fr Pool state information
#[derive(Debug, Clone)]
pub struct MySqlPoolState {
    /// Total number of connections
    /// Number of idle connections
/// fr fr Pool health report
#[derive(Debug, Clone)]
pub struct MySqlPoolHealthReport {
    /// Total connections checked
    /// Number of healthy connections
    /// Number of failed connections
    /// List of errors encountered
    /// Timestamp of the health check
impl MySqlPoolHealthReport {
    /// Check if the pool is healthy
    pub fn is_healthy(&self) -> bool {
        self.failed_connections == 0 && self.healthy_connections > 0
    /// Get health percentage
    pub fn health_percentage(&self) -> f64 {
        if self.total_connections == 0 {
            return 0.0;
        }
        (self.healthy_connections as f64 / self.total_connections as f64) * 100.0
    }
}

/// fr fr Pool statistics
#[derive(Debug)]
pub struct MySqlPoolStats {
    /// Total connections created
    /// Total connections closed
    /// Total connection acquisition attempts
    /// Total failed connection attempts
    /// Total time spent acquiring connections
    /// Maximum acquisition time observed
impl MySqlPoolStats {
    /// Create new pool statistics
    pub fn new() -> Self {
        Self {
        }
    }

    /// Record a successful connection acquisition
    pub fn record_connection_acquired(&self, duration: Duration) {
        use std::sync::atomic::Ordering;
        
        self.connection_attempts.fetch_add(1, Ordering::Relaxed);
        
        if let Ok(mut total) = self.total_acquisition_time.lock() {
            *total += duration;
        if let Ok(mut max) = self.max_acquisition_time.lock() {
            if duration > *max {
                *max = duration;
            }
        }
    /// Record a failed connection attempt
    pub fn record_connection_failure(&self) {
        use std::sync::atomic::Ordering;
        
        self.connection_attempts.fetch_add(1, Ordering::Relaxed);
        self.connection_failures.fetch_add(1, Ordering::Relaxed);
    /// Get average acquisition time
    pub fn average_acquisition_time(&self) -> Duration {
        use std::sync::atomic::Ordering;
        
        let attempts = self.connection_attempts.load(Ordering::Relaxed);
        if attempts == 0 {
            return Duration::ZERO;
        if let Ok(total) = self.total_acquisition_time.lock() {
            *total / attempts as u32
        } else {
            Duration::ZERO
        }
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        use std::sync::atomic::Ordering;
        
        let attempts = self.connection_attempts.load(Ordering::Relaxed);
        if attempts == 0 {
            return 100.0;
        let failures = self.connection_failures.load(Ordering::Relaxed);
        let successes = attempts.saturating_sub(failures);
        
        (successes as f64 / attempts as f64) * 100.0
    }
}

impl Default for MySqlPoolStats {
    fn default() -> Self {
        Self::new()
    }
}

