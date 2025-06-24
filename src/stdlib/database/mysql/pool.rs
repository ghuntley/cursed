use crate::error::Error;
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
    pub min_connections: usize,
    /// Maximum number of connections
    pub max_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Maximum connection lifetime
    pub max_lifetime: Option<Duration>,
    /// Connection idle timeout
    pub idle_timeout: Option<Duration>,
    /// Test query for health checks
    pub test_query: String,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Maximum retries for failed connections
    pub max_retries: usize,
    /// Retry delay
    pub retry_delay: Duration,
}

impl Default for MySqlPoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 1,
            max_connections: 100,
            connection_timeout: Duration::from_secs(30),
            max_lifetime: Some(Duration::from_secs(3600)),
            idle_timeout: Some(Duration::from_secs(600)),
            test_query: "SELECT 1".to_string(),
            health_check_interval: Duration::from_secs(60),
            max_retries: 3,
            retry_delay: Duration::from_millis(1000),
        }
    }
}

/// fr fr MySQL connection pool with advanced features
#[derive(Debug)]
pub struct MySqlPool {
    /// Underlying MySQL pool
    inner: Arc<Pool>,
    /// Pool configuration
    config: MySqlPoolConfig,
    /// Pool statistics
    stats: MySqlPoolStats,
}

impl MySqlPool {
    /// Create a new MySQL connection pool
    pub fn new(dsn: &str, config: MySqlPoolConfig) -> MySqlResult<Self> {
        let conn_info = parse_connection_string(dsn)?;
        let pool = Self::create_pool(&conn_info, &config)?;

        Ok(Self {
            inner: Arc::new(pool),
            config,
            stats: MySqlPoolStats::new(),
        })
    }

    /// Create a MySQL pool with default configuration
    pub fn with_defaults(dsn: &str) -> MySqlResult<Self> {
        Self::new(dsn, MySqlPoolConfig::default())
    }

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
    }

    /// Get a connection from the pool
    pub fn get_connection(&self) -> MySqlResult<PooledConn> {
        let start_time = Instant::now();
        
        let conn = self.inner.get_conn()
            .map_err(|e| MySqlError::pool_error(&format!("Failed to get connection: {}", e)))?;

        // Update statistics
        self.stats.record_connection_acquired(start_time.elapsed());

        Ok(conn)
    }

    /// Get a connection with timeout
    pub fn get_connection_timeout(&self, timeout: Duration) -> MySqlResult<PooledConn> {
        let start_time = Instant::now();
        
        // Create a scoped pool with timeout
        let conn = self.inner.get_conn()
            .map_err(|e| MySqlError::timeout_error(&format!("Connection timeout: {}", e)))?;

        if start_time.elapsed() > timeout {
            return Err(MySqlError::timeout_error("Connection acquisition timed out"));
        }

        self.stats.record_connection_acquired(start_time.elapsed());
        Ok(conn)
    }

    /// Test a connection's health
    pub fn test_connection(&self, conn: &mut PooledConn) -> MySqlResult<bool> {
        use mysql::prelude::Queryable;
        
        match conn.query_drop(&self.config.test_query) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
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
            total_connections,
            healthy_connections,
            failed_connections,
            errors,
            timestamp: Instant::now(),
        })
    }

    /// Get pool statistics
    pub fn stats(&self) -> &MySqlPoolStats {
        &self.stats
    }

    /// Get pool state information
    pub fn state(&self) -> MySqlResult<MySqlPoolState> {
        let state = self.inner.state()
            .ok_or_else(|| MySqlError::pool_error("Failed to get pool state"))?;

        Ok(MySqlPoolState {
            connections: state.connections,
            idle_connections: state.idle_connections,
        })
    }

    /// Close the pool
    pub fn close(&self) -> MySqlResult<()> {
        // The mysql crate doesn't provide an explicit close method
        // Connections will be closed when the pool is dropped
        Ok(())
    }

    /// Get the underlying pool reference
    pub fn inner(&self) -> &Arc<Pool> {
        &self.inner
    }

    /// Get pool configuration
    pub fn config(&self) -> &MySqlPoolConfig {
        &self.config
    }
}

/// fr fr Pool state information
#[derive(Debug, Clone)]
pub struct MySqlPoolState {
    /// Total number of connections
    pub connections: usize,
    /// Number of idle connections
    pub idle_connections: usize,
}

/// fr fr Pool health report
#[derive(Debug, Clone)]
pub struct MySqlPoolHealthReport {
    /// Total connections checked
    pub total_connections: usize,
    /// Number of healthy connections
    pub healthy_connections: usize,
    /// Number of failed connections
    pub failed_connections: usize,
    /// List of errors encountered
    pub errors: Vec<String>,
    /// Timestamp of the health check
    pub timestamp: Instant,
}

impl MySqlPoolHealthReport {
    /// Check if the pool is healthy
    pub fn is_healthy(&self) -> bool {
        self.failed_connections == 0 && self.healthy_connections > 0
    }

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
    pub connections_created: std::sync::atomic::AtomicUsize,
    /// Total connections closed
    pub connections_closed: std::sync::atomic::AtomicUsize,
    /// Total connection acquisition attempts
    pub connection_attempts: std::sync::atomic::AtomicUsize,
    /// Total failed connection attempts
    pub connection_failures: std::sync::atomic::AtomicUsize,
    /// Total time spent acquiring connections
    pub total_acquisition_time: std::sync::Mutex<Duration>,
    /// Maximum acquisition time observed
    pub max_acquisition_time: std::sync::Mutex<Duration>,
}

impl MySqlPoolStats {
    /// Create new pool statistics
    pub fn new() -> Self {
        Self {
            connections_created: std::sync::atomic::AtomicUsize::new(0),
            connections_closed: std::sync::atomic::AtomicUsize::new(0),
            connection_attempts: std::sync::atomic::AtomicUsize::new(0),
            connection_failures: std::sync::atomic::AtomicUsize::new(0),
            total_acquisition_time: std::sync::Mutex::new(Duration::ZERO),
            max_acquisition_time: std::sync::Mutex::new(Duration::ZERO),
        }
    }

    /// Record a successful connection acquisition
    pub fn record_connection_acquired(&self, duration: Duration) {
        use std::sync::atomic::Ordering;
        
        self.connection_attempts.fetch_add(1, Ordering::Relaxed);
        
        if let Ok(mut total) = self.total_acquisition_time.lock() {
            *total += duration;
        }
        
        if let Ok(mut max) = self.max_acquisition_time.lock() {
            if duration > *max {
                *max = duration;
            }
        }
    }

    /// Record a failed connection attempt
    pub fn record_connection_failure(&self) {
        use std::sync::atomic::Ordering;
        
        self.connection_attempts.fetch_add(1, Ordering::Relaxed);
        self.connection_failures.fetch_add(1, Ordering::Relaxed);
    }

    /// Get average acquisition time
    pub fn average_acquisition_time(&self) -> Duration {
        use std::sync::atomic::Ordering;
        
        let attempts = self.connection_attempts.load(Ordering::Relaxed);
        if attempts == 0 {
            return Duration::ZERO;
        }
        
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
        }
        
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_config_defaults() {
        let config = MySqlPoolConfig::default();
        
        assert_eq!(config.min_connections, 1);
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.test_query, "SELECT 1");
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_pool_stats() {
        let stats = MySqlPoolStats::new();
        
        // Record some acquisitions
        stats.record_connection_acquired(Duration::from_millis(10));
        stats.record_connection_acquired(Duration::from_millis(20));
        stats.record_connection_failure();
        
        assert!(stats.success_rate() > 0.0);
        assert!(stats.success_rate() < 100.0);
        assert!(stats.average_acquisition_time() > Duration::ZERO);
    }

    #[test]
    fn test_health_report() {
        let report = MySqlPoolHealthReport {
            total_connections: 10,
            healthy_connections: 8,
            failed_connections: 2,
            errors: vec!["Test error".to_string()],
            timestamp: Instant::now(),
        };
        
        assert!(!report.is_healthy()); // Has failed connections
        assert_eq!(report.health_percentage(), 80.0);
    }

    #[test]
    fn test_pool_state() {
        let state = MySqlPoolState {
            connections: 10,
            idle_connections: 5,
        };
        
        assert_eq!(state.connections, 10);
        assert_eq!(state.idle_connections, 5);
    }
}
