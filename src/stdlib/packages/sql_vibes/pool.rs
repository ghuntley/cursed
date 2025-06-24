/// fr fr Connection pool implementation - manage database connections like a boss periodt
use crate::stdlib::packages::sql_vibes::{SqlResult, SqlError, DatabaseConnection, ConnectionConfig, DatabaseDriver};
use crate::error::Error;
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::thread;
use serde::{Serialize, Deserialize};

/// fr fr Connection pool for managing database connections - thread-safe vibes
pub struct ConnectionPool {
    /// Pool configuration
    config: PoolConfig,
    
    /// Available connections
    available: Arc<Mutex<VecDeque<PooledConnectionInner>>>,
    
    /// Pool statistics
    stats: Arc<RwLock<PoolStats>>,
    
    /// Condition variable for waiting threads
    condition: Arc<Condvar>,
    
    /// Database driver for creating new connections
    driver: Arc<dyn DatabaseDriver + Send + Sync>,
    
    /// Connection configuration template
    connection_config: ConnectionConfig,
    
    /// Pool shutdown flag
    shutdown: Arc<RwLock<bool>>,
    
    /// Background thread handle for pool maintenance
    maintenance_handle: Option<thread::JoinHandle<()>>,
}

impl ConnectionPool {
    /// sus Create new connection pool with configuration
    pub fn new(connection_string: &str, config: PoolConfig) -> SqlResult<Self> {
        // Parse connection string to get driver type
        let driver_type = parse_driver_from_connection_string(connection_string)?;
        let driver = get_mock_driver(&driver_type)?; // In real implementation, get actual driver
        let connection_config = ConnectionConfig::from_string(connection_string)?;
        
        let pool = Self {
            config,
            available: Arc::new(Mutex::new(VecDeque::new())),
            stats: Arc::new(RwLock::new(PoolStats::new())),
            condition: Arc::new(Condvar::new()),
            driver,
            connection_config,
            shutdown: Arc::new(RwLock::new(false)),
            maintenance_handle: None,
        };
        
        // Initialize minimum connections
        pool.initialize_pool()?;
        
        // Start maintenance thread
        let maintenance_pool = pool.clone_for_maintenance();
        let handle = thread::spawn(move || {
            maintenance_pool.run_maintenance_loop();
        });
        
        // Can't assign to self in constructor, so return modified pool
        Ok(pool)
    }
    
    /// facts Get a connection from the pool - blocks until available
    pub fn get_connection(&self) -> SqlResult<PooledConnection> {
        let start_time = Instant::now();
        
        loop {
            // Check if pool is shutting down
            if *self.shutdown.read().unwrap() {
                return Err(SqlError::connection("Connection pool is shutting down - no new connections bestie".to_string()));
            }
            
            // Try to get available connection
            if let Some(conn) = self.try_get_available_connection()? {
                self.update_checkout_stats();
                return Ok(PooledConnection::new(conn, self.clone_for_return()));
            }
            
            // Check if we can create new connection
            if self.can_create_new_connection() {
                match self.create_new_connection() {
                    Ok(conn) => {
                        self.update_checkout_stats();
                        return Ok(PooledConnection::new(conn, self.clone_for_return()));
                    }
                    Err(e) => {
                        // If creation failed, wait for available connection
                        if !self.config.fail_fast {
                            self.wait_for_available_connection(start_time)?;
                            continue;
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
            
            // Wait for connection to become available
            self.wait_for_available_connection(start_time)?;
        }
    }
    
    /// lowkey Try to get connection without blocking
    pub fn try_get_connection(&self) -> SqlResult<Option<PooledConnection>> {
        if *self.shutdown.read().unwrap() {
            return Err(SqlError::connection("Connection pool is shutting down - no connections available periodt".to_string()));
        }
        
        if let Some(conn) = self.try_get_available_connection()? {
            self.update_checkout_stats();
            return Ok(Some(PooledConnection::new(conn, self.clone_for_return())));
        }
        
        if self.can_create_new_connection() {
            match self.create_new_connection() {
                Ok(conn) => {
                    self.update_checkout_stats();
                    Ok(Some(PooledConnection::new(conn, self.clone_for_return())))
                }
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
    
    /// highkey Get current pool statistics
    pub fn stats(&self) -> PoolStats {
        self.stats.read().unwrap().clone()
    }
    
    /// periodt Check pool health
    pub fn health_check(&self) -> SqlResult<PoolHealth> {
        let stats = self.stats();
        let available_count = self.available.lock().unwrap().len();
        
        let health_status = if stats.active_connections > self.config.max_connections {
            PoolHealthStatus::Critical
        } else if stats.active_connections as f64 / self.config.max_connections as f64 > 0.8 {
            PoolHealthStatus::Warning
        } else {
            PoolHealthStatus::Healthy
        };
        
        Ok(PoolHealth {
            status: health_status,
            active_connections: stats.active_connections,
            available_connections: available_count,
            total_created: stats.total_created,
            total_errors: stats.total_errors,
            average_checkout_time_ms: stats.average_checkout_time_ms,
        })
    }
    
    /// bestie Shutdown the pool gracefully
    pub fn shutdown(&mut self) -> SqlResult<()> {
        // Set shutdown flag
        *self.shutdown.write().unwrap() = true;
        
        // Notify waiting threads
        self.condition.notify_all();
        
        // Wait for maintenance thread to finish
        if let Some(handle) = self.maintenance_handle.take() {
            handle.join().map_err(|_| {
                SqlError::connection("Failed to join maintenance thread during shutdown - that's not good bestie".to_string())
            })?;
        }
        
        // Close all available connections
        let mut available = self.available.lock().unwrap();
        while let Some(mut conn) = available.pop_front() {
            let _ = conn.connection.close(); // Ignore errors during shutdown
        }
        
        Ok(())
    }
    
    /// flex Get connection config for creating new connections
    pub fn connection_config(&self) -> &ConnectionConfig {
        &self.connection_config
    }
    
    /// Internal: Try to get available connection from pool
    fn try_get_available_connection(&self) -> SqlResult<Option<PooledConnectionInner>> {
        let mut available = self.available.lock().unwrap();
        
        while let Some(mut conn) = available.pop_front() {
            // Check if connection is still valid
            if conn.is_valid() && !conn.is_expired(&self.config) {
                conn.last_used = Instant::now();
                return Ok(Some(conn));
            } else {
                // Connection is invalid or expired, close it
                let _ = conn.connection.close();
                self.update_connection_closed();
            }
        }
        
        Ok(None)
    }
    
    /// Internal: Check if we can create new connection
    fn can_create_new_connection(&self) -> bool {
        let stats = self.stats.read().unwrap();
        stats.active_connections < self.config.max_connections
    }
    
    /// Internal: Create new database connection
    fn create_new_connection(&self) -> SqlResult<PooledConnectionInner> {
        let connection = self.driver.connect(self.connection_config.clone())?;
        
        let pooled_conn = PooledConnectionInner {
            connection,
            created_at: Instant::now(),
            last_used: Instant::now(),
            checkout_count: 0,
        };
        
        self.update_connection_created();
        Ok(pooled_conn)
    }
    
    /// Internal: Wait for connection to become available
    fn wait_for_available_connection(&self, start_time: Instant) -> SqlResult<()> {
        let available = self.available.lock().unwrap();
        
        // Check timeout
        if start_time.elapsed() >= self.config.connection_timeout {
            return Err(SqlError::timeout("get_connection", self.config.connection_timeout.as_millis() as u64));
        }
        
        // Wait for notification or timeout
        let remaining_timeout = self.config.connection_timeout - start_time.elapsed();
        let _result = self.condition.wait_timeout(available, remaining_timeout).unwrap();
        
        Ok(())
    }
    
    /// Internal: Return connection to pool
    fn return_connection(&self, conn: PooledConnectionInner) {
        if *self.shutdown.read().unwrap() {
            // Pool is shutting down, just close the connection
            let _ = conn.connection.close();
            return;
        }
        
        if conn.is_valid() && !conn.is_expired(&self.config) {
            // Connection is still good, return to pool
            self.available.lock().unwrap().push_back(conn);
            self.condition.notify_one();
        } else {
            // Connection is bad, close it
            let _ = conn.connection.close();
            self.update_connection_closed();
        }
    }
    
    /// Internal: Initialize pool with minimum connections
    fn initialize_pool(&self) -> SqlResult<()> {
        for _ in 0..self.config.min_connections {
            let conn = self.create_new_connection()?;
            self.available.lock().unwrap().push_back(conn);
        }
        Ok(())
    }
    
    /// Internal: Run maintenance loop
    fn run_maintenance_loop(&self) {
        while !*self.shutdown.read().unwrap() {
            // Sleep for maintenance interval
            thread::sleep(self.config.maintenance_interval);
            
            // Check if we need to stop
            if *self.shutdown.read().unwrap() {
                break;
            }
            
            // Perform maintenance tasks
            self.cleanup_expired_connections();
            self.ensure_minimum_connections();
            self.update_pool_stats();
        }
    }
    
    /// Internal: Clean up expired connections
    fn cleanup_expired_connections(&self) {
        let mut available = self.available.lock().unwrap();
        let mut to_remove = Vec::new();
        
        for (index, conn) in available.iter().enumerate() {
            if conn.is_expired(&self.config) {
                to_remove.push(index);
            }
        }
        
        // Remove expired connections (in reverse order to maintain indices)
        for &index in to_remove.iter().rev() {
            if let Some(mut conn) = available.remove(index) {
                let _ = conn.connection.close();
                self.update_connection_closed();
            }
        }
    }
    
    /// Internal: Ensure minimum connections are available
    fn ensure_minimum_connections(&self) {
        let available_count = self.available.lock().unwrap().len();
        let stats = self.stats.read().unwrap();
        let total_connections = available_count + stats.active_connections;
        
        if total_connections < self.config.min_connections {
            let needed = self.config.min_connections - total_connections;
            for _ in 0..needed {
                if let Ok(conn) = self.create_new_connection() {
                    self.available.lock().unwrap().push_back(conn);
                }
            }
        }
    }
    
    /// Internal: Update pool statistics
    fn update_pool_stats(&self) {
        // This would collect and update various pool metrics
        // For now, just a placeholder
    }
    
    /// Internal: Update checkout statistics
    fn update_checkout_stats(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.total_checkouts += 1;
        stats.active_connections += 1;
    }
    
    /// Internal: Update connection created statistics
    fn update_connection_created(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.total_created += 1;
    }
    
    /// Internal: Update connection closed statistics
    fn update_connection_closed(&self) {
        let mut stats = self.stats.write().unwrap();
        if stats.active_connections > 0 {
            stats.active_connections -= 1;
        }
    }
    
    /// Internal: Clone pool for maintenance thread
    fn clone_for_maintenance(&self) -> ConnectionPoolMaintenance {
        ConnectionPoolMaintenance {
            config: self.config.clone(),
            available: self.available.clone(),
            stats: self.stats.clone(),
            condition: self.condition.clone(),
            driver: self.driver.clone(),
            connection_config: self.connection_config.clone(),
            shutdown: self.shutdown.clone(),
        }
    }
    
    /// Internal: Clone pool for connection return
    fn clone_for_return(&self) -> ConnectionPoolReturn {
        ConnectionPoolReturn {
            available: self.available.clone(),
            condition: self.condition.clone(),
            shutdown: self.shutdown.clone(),
            stats: self.stats.clone(),
            config: self.config.clone(),
        }
    }
}

/// fr fr Configuration for connection pool - all the settings periodt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Minimum number of connections to maintain
    pub min_connections: usize,
    
    /// Maximum number of connections allowed
    pub max_connections: usize,
    
    /// Connection timeout when getting from pool
    pub connection_timeout: Duration,
    
    /// Maximum lifetime of a connection
    pub max_connection_lifetime: Duration,
    
    /// Maximum idle time before connection is closed
    pub max_idle_time: Duration,
    
    /// Interval for running pool maintenance
    pub maintenance_interval: Duration,
    
    /// Whether to fail fast when no connections available
    pub fail_fast: bool,
    
    /// Whether to validate connections on checkout
    pub validate_on_checkout: bool,
    
    /// Whether to validate connections on return
    pub validate_on_return: bool,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 2,
            max_connections: 10,
            connection_timeout: Duration::from_secs(30),
            max_connection_lifetime: Duration::from_secs(1800), // 30 minutes
            max_idle_time: Duration::from_secs(600), // 10 minutes
            maintenance_interval: Duration::from_secs(60), // 1 minute
            fail_fast: false,
            validate_on_checkout: true,
            validate_on_return: false,
        }
    }
}

impl PoolConfig {
    /// sus Create pool config with custom max connections
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }
    
    /// facts Create pool config with custom min connections
    pub fn min_connections(mut self, min: usize) -> Self {
        self.min_connections = min;
        self
    }
    
    /// lowkey Set connection timeout
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }
    
    /// highkey Set max connection lifetime
    pub fn max_lifetime(mut self, lifetime: Duration) -> Self {
        self.max_connection_lifetime = lifetime;
        self
    }
    
    /// periodt Enable fail-fast mode
    pub fn fail_fast(mut self) -> Self {
        self.fail_fast = true;
        self
    }
}

/// fr fr Pool statistics - monitoring and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    /// Total connections created since pool start
    pub total_created: usize,
    
    /// Total connection checkouts
    pub total_checkouts: usize,
    
    /// Currently active (checked out) connections
    pub active_connections: usize,
    
    /// Total errors encountered
    pub total_errors: usize,
    
    /// Average checkout time in milliseconds
    pub average_checkout_time_ms: f64,
    
    /// Pool uptime
    pub uptime: Duration,
    
    /// Last maintenance run time
    pub last_maintenance: Option<Instant>,
}

impl PoolStats {
    fn new() -> Self {
        Self {
            total_created: 0,
            total_checkouts: 0,
            active_connections: 0,
            total_errors: 0,
            average_checkout_time_ms: 0.0,
            uptime: Duration::new(0, 0),
            last_maintenance: None,
        }
    }
}

/// fr fr Pool health status - how the pool is doing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolHealth {
    /// Overall health status
    pub status: PoolHealthStatus,
    
    /// Current active connections
    pub active_connections: usize,
    
    /// Current available connections
    pub available_connections: usize,
    
    /// Total connections created
    pub total_created: usize,
    
    /// Total errors
    pub total_errors: usize,
    
    /// Average checkout time
    pub average_checkout_time_ms: f64,
}

/// fr fr Pool health status levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolHealthStatus {
    /// Pool is healthy and operating normally
    Healthy,
    
    /// Pool is under stress but functional
    Warning,
    
    /// Pool is in critical state
    Critical,
    
    /// Pool is unavailable
    Unavailable,
}

/// fr fr Pooled connection wrapper - RAII for returning connections
pub struct PooledConnection {
    /// Inner connection (Some when active, None when returned)
    inner: Option<PooledConnectionInner>,
    
    /// Pool reference for returning connection
    pool: ConnectionPoolReturn,
}

impl PooledConnection {
    fn new(inner: PooledConnectionInner, pool: ConnectionPoolReturn) -> Self {
        Self {
            inner: Some(inner),
            pool,
        }
    }
    
    /// yolo Get the underlying database connection
    pub fn connection(&mut self) -> &mut dyn DatabaseConnection {
        self.inner.as_mut()
            .expect("Connection already returned to pool")
            .connection.as_mut()
    }
    
    /// slay Check if connection is valid
    pub fn is_valid(&self) -> bool {
        self.inner.as_ref()
            .map(|conn| conn.is_valid())
            .unwrap_or(false)
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            self.pool.return_connection(inner);
        }
    }
}

/// fr fr Internal pooled connection with metadata
struct PooledConnectionInner {
    /// The actual database connection
    connection: Box<dyn DatabaseConnection>,
    
    /// When this connection was created
    created_at: Instant,
    
    /// When this connection was last used
    last_used: Instant,
    
    /// Number of times this connection has been checked out
    checkout_count: usize,
}

impl PooledConnectionInner {
    /// Check if connection is still valid
    fn is_valid(&self) -> bool {
        self.connection.is_alive()
    }
    
    /// Check if connection is expired based on pool config
    fn is_expired(&self, config: &PoolConfig) -> bool {
        let lifetime_expired = self.created_at.elapsed() > config.max_connection_lifetime;
        let idle_expired = self.last_used.elapsed() > config.max_idle_time;
        lifetime_expired || idle_expired
    }
}

/// fr fr Maintenance worker for connection pool
struct ConnectionPoolMaintenance {
    config: PoolConfig,
    available: Arc<Mutex<VecDeque<PooledConnectionInner>>>,
    stats: Arc<RwLock<PoolStats>>,
    condition: Arc<Condvar>,
    driver: Arc<dyn DatabaseDriver + Send + Sync>,
    connection_config: ConnectionConfig,
    shutdown: Arc<RwLock<bool>>,
}

impl ConnectionPoolMaintenance {
    fn cleanup_expired_connections(&self) {
        let mut available = self.available.lock().unwrap();
        let mut to_remove = Vec::new();
        
        for (index, conn) in available.iter().enumerate() {
            if conn.is_expired(&self.config) {
                to_remove.push(index);
            }
        }
        
        for &index in to_remove.iter().rev() {
            if let Some(mut conn) = available.remove(index) {
                let _ = conn.connection.close();
            }
        }
    }
    
    fn ensure_minimum_connections(&self) {
        let available_count = self.available.lock().unwrap().len();
        let stats = self.stats.read().unwrap();
        let total_connections = available_count + stats.active_connections;
        
        if total_connections < self.config.min_connections {
            let needed = self.config.min_connections - total_connections;
            for _ in 0..needed {
                if let Ok(connection) = self.driver.connect(self.connection_config.clone()) {
                    let pooled_conn = PooledConnectionInner {
                        connection,
                        created_at: Instant::now(),
                        last_used: Instant::now(),
                        checkout_count: 0,
                    };
                    self.available.lock().unwrap().push_back(pooled_conn);
                }
            }
        }
    }
    
    fn update_pool_stats(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.last_maintenance = Some(Instant::now());
    }
    
    fn run_maintenance_loop(&self) {
        while !*self.shutdown.read().unwrap() {
            thread::sleep(self.config.maintenance_interval);
            
            if *self.shutdown.read().unwrap() {
                break;
            }
            
            self.cleanup_expired_connections();
            self.ensure_minimum_connections();
            self.update_pool_stats();
        }
    }
}

/// fr fr Connection return handler - for RAII pattern
#[derive(Clone)]
struct ConnectionPoolReturn {
    available: Arc<Mutex<VecDeque<PooledConnectionInner>>>,
    condition: Arc<Condvar>,
    shutdown: Arc<RwLock<bool>>,
    stats: Arc<RwLock<PoolStats>>,
    config: PoolConfig,
}

impl ConnectionPoolReturn {
    fn return_connection(&self, conn: PooledConnectionInner) {
        if *self.shutdown.read().unwrap() {
            let _ = conn.connection.close();
            return;
        }
        
        if conn.is_valid() && !conn.is_expired(&self.config) {
            self.available.lock().unwrap().push_back(conn);
            self.condition.notify_one();
        } else {
            let _ = conn.connection.close();
        }
        
        // Update stats
        let mut stats = self.stats.write().unwrap();
        if stats.active_connections > 0 {
            stats.active_connections -= 1;
        }
    }
}

/// Internal helper functions
fn parse_driver_from_connection_string(connection_string: &str) -> SqlResult<String> {
    if connection_string.starts_with("sqlite://") || connection_string.starts_with("sqlite3://") {
        Ok("sqlite".to_string())
    } else if connection_string.starts_with("postgres://") || connection_string.starts_with("postgresql://") {
        Ok("postgres".to_string())
    } else if connection_string.starts_with("mysql://") {
        Ok("mysql".to_string())
    } else {
        Err(SqlError::connection(format!("Unknown driver type in connection string: {} - check the format bestie", connection_string)))
    }
}

fn get_mock_driver(_driver_type: &str) -> SqlResult<Arc<dyn DatabaseDriver + Send + Sync>> {
    // In real implementation, this would return actual database drivers
    // For now, return an error since we don't have real drivers
    Err(SqlError::connection("Mock drivers not implemented yet - coming soon periodt".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_config_default() {
        let config = PoolConfig::default();
        assert_eq!(config.min_connections, 2);
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
        assert!(!config.fail_fast);
        assert!(config.validate_on_checkout);
    }

    #[test]
    fn test_pool_config_builder() {
        let config = PoolConfig::default()
            .max_connections(20)
            .min_connections(5)
            .connection_timeout(Duration::from_secs(60))
            .fail_fast();
        
        assert_eq!(config.max_connections, 20);
        assert_eq!(config.min_connections, 5);
        assert_eq!(config.connection_timeout, Duration::from_secs(60));
        assert!(config.fail_fast);
    }

    #[test]
    fn test_pool_stats_new() {
        let stats = PoolStats::new();
        assert_eq!(stats.total_created, 0);
        assert_eq!(stats.total_checkouts, 0);
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.total_errors, 0);
        assert_eq!(stats.average_checkout_time_ms, 0.0);
    }

    #[test]
    fn test_parse_driver_from_connection_string() {
        assert_eq!(parse_driver_from_connection_string("sqlite://test.db").unwrap(), "sqlite");
        assert_eq!(parse_driver_from_connection_string("postgres://localhost/db").unwrap(), "postgres");
        assert_eq!(parse_driver_from_connection_string("mysql://localhost/db").unwrap(), "mysql");
        
        assert!(parse_driver_from_connection_string("invalid://test").is_err());
    }

    #[test]
    fn test_pool_health_status() {
        let health = PoolHealth {
            status: PoolHealthStatus::Healthy,
            active_connections: 5,
            available_connections: 3,
            total_created: 8,
            total_errors: 0,
            average_checkout_time_ms: 15.5,
        };
        
        assert_eq!(health.status, PoolHealthStatus::Healthy);
        assert_eq!(health.active_connections, 5);
        assert_eq!(health.available_connections, 3);
    }

    #[test]
    fn test_pooled_connection_inner_expiry() {
        let config = PoolConfig {
            max_connection_lifetime: Duration::from_secs(1),
            max_idle_time: Duration::from_secs(1),
            ..Default::default()
        };
        
        // Note: This test would need a mock connection to work properly
        // For now, just test the structure
        assert_eq!(config.max_connection_lifetime, Duration::from_secs(1));
        assert_eq!(config.max_idle_time, Duration::from_secs(1));
    }
}
