/// PostgreSQL connection pooling with optimizations for CURSED database operations
/// 
/// This module provides advanced connection pooling specifically optimized for PostgreSQL
/// including connection validation, automatic recovery, and PostgreSQL-specific features.

use std::sync::{Arc, Mutex, Condvar};
use std::collections::{VecDeque, HashMap};
use std::time::{Duration, Instant, SystemTime};
use std::thread;
use super::{
    PostgreSQLConnection, PostgreSQLConfig, PostgreSQLError
};
use super::super::{DatabaseError, DatabaseErrorKind};

/// fr fr PostgreSQL-specific pool configuration
#[derive(Debug, Clone)]
pub struct PostgreSQLPoolConfig {
    /// fr fr Minimum number of connections to maintain
    pub min_connections: usize,
    /// fr fr Maximum number of connections allowed
    pub max_connections: usize,
    /// fr fr Maximum time to wait for a connection
    pub connection_timeout: Duration,
    /// fr fr Maximum lifetime of a connection
    pub max_connection_lifetime: Duration,
    /// fr fr Maximum idle time for a connection
    pub max_idle_time: Duration,
    /// fr fr Health check interval
    pub health_check_interval: Duration,
    /// fr fr Connection validation query
    pub validation_query: String,
    /// fr fr Enable connection validation on borrow
    pub validate_on_borrow: bool,
    /// fr fr Enable connection validation on return
    pub validate_on_return: bool,
    /// fr fr Enable periodic health checks
    pub enable_health_checks: bool,
    /// fr fr PostgreSQL-specific: prepare cache size per connection
    pub prepare_cache_size: usize,
    /// fr fr PostgreSQL-specific: enable connection multiplexing
    pub enable_multiplexing: bool,
    /// fr fr PostgreSQL-specific: connection application name prefix
    pub app_name_prefix: String,
}

impl Default for PostgreSQLPoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 20,
            connection_timeout: Duration::from_secs(30),
            max_connection_lifetime: Duration::from_secs(3600), // 1 hour
            max_idle_time: Duration::from_secs(600), // 10 minutes
            health_check_interval: Duration::from_secs(30),
            validation_query: "SELECT 1".to_string(),
            validate_on_borrow: true,
            validate_on_return: false,
            enable_health_checks: true,
            prepare_cache_size: 100,
            enable_multiplexing: false,
            app_name_prefix: "cursed_pool".to_string(),
        }
    }
}

/// fr fr Connection wrapper with metadata for pool management
#[derive(Debug)]
struct PooledConnection {
    /// fr fr The actual connection
    connection: PostgreSQLConnection,
    /// fr fr When this connection was created
    created_at: Instant,
    /// fr fr When this connection was last used
    last_used: Instant,
    /// fr fr Number of times this connection has been borrowed
    borrow_count: u64,
    /// fr fr Whether this connection is currently borrowed
    is_borrowed: bool,
    /// fr fr Connection health status
    is_healthy: bool,
    /// fr fr Connection ID for tracking
    id: String,
}

impl PooledConnection {
    fn new(connection: PostgreSQLConnection) -> Self {
        let now = Instant::now();
        let id = format!("conn_{:x}", now.elapsed().as_nanos());
        
        Self {
            connection,
            created_at: now,
            last_used: now,
            borrow_count: 0,
            is_borrowed: false,
            is_healthy: true,
            id,
        }
    }
    
    fn borrow(&mut self) -> &mut PostgreSQLConnection {
        self.is_borrowed = true;
        self.last_used = Instant::now();
        self.borrow_count += 1;
        &mut self.connection
    }
    
    fn return_to_pool(&mut self) {
        self.is_borrowed = false;
        self.last_used = Instant::now();
    }
    
    fn is_expired(&self, max_lifetime: Duration) -> bool {
        self.created_at.elapsed() > max_lifetime
    }
    
    fn is_idle_too_long(&self, max_idle: Duration) -> bool {
        !self.is_borrowed && self.last_used.elapsed() > max_idle
    }
    
    fn validate(&mut self, validation_query: &str) -> bool {
        match self.connection.query(validation_query, &[]) {
            Ok(_) => {
                self.is_healthy = true;
                true
            }
            Err(_) => {
                self.is_healthy = false;
                false
            }
        }
    }
}

/// fr fr PostgreSQL connection pool implementation
#[derive(Debug)]
pub struct PostgreSQLPool {
    /// fr fr Pool configuration
    config: PostgreSQLPoolConfig,
    /// fr fr Database configuration
    db_config: PostgreSQLConfig,
    /// fr fr Available connections
    available: Arc<Mutex<VecDeque<PooledConnection>>>,
    /// fr fr Borrowed connections
    borrowed: Arc<Mutex<HashMap<String, PooledConnection>>>,
    /// fr fr Condition variable for waiting threads
    available_notify: Arc<Condvar>,
    /// fr fr Pool statistics
    stats: Arc<Mutex<PoolStats>>,
    /// fr fr Health checker handle
    health_checker: Option<thread::JoinHandle<()>>,
    /// fr fr Shutdown flag
    shutdown: Arc<std::sync::atomic::AtomicBool>,
}

/// fr fr Pool statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// fr fr Total connections created
    pub connections_created: u64,
    /// fr fr Total connections destroyed
    pub connections_destroyed: u64,
    /// fr fr Total connection borrows
    pub connections_borrowed: u64,
    /// fr fr Total connection returns
    pub connections_returned: u64,
    /// fr fr Current active connections
    pub active_connections: usize,
    /// fr fr Current idle connections
    pub idle_connections: usize,
    /// fr fr Total failed validations
    pub validation_failures: u64,
    /// fr fr Total health check failures
    pub health_check_failures: u64,
    /// fr fr Average connection wait time
    pub avg_wait_time: Duration,
    /// fr fr Peak connections
    pub peak_connections: usize,
    /// fr fr Pool start time
    pub started_at: SystemTime,
}

impl PostgreSQLPool {
    /// slay Create a new PostgreSQL connection pool
    pub fn new(db_config: PostgreSQLConfig, pool_config: PostgreSQLPoolConfig) -> Result<Self, PostgreSQLError> {
        let available = Arc::new(Mutex::new(VecDeque::new()));
        let borrowed = Arc::new(Mutex::new(HashMap::new()));
        let available_notify = Arc::new(Condvar::new());
        let shutdown = Arc::new(std::sync::atomic::AtomicBool::new(false));
        
        let mut stats = PoolStats::default();
        stats.started_at = SystemTime::now();
        let stats = Arc::new(Mutex::new(stats));
        
        let mut pool = Self {
            config: pool_config.clone(),
            db_config: db_config.clone(),
            available: available.clone(),
            borrowed,
            available_notify: available_notify.clone(),
            stats: stats.clone(),
            health_checker: None,
            shutdown: shutdown.clone(),
        };
        
        // Create initial connections
        pool.initialize_connections()?;
        
        // Start health checker if enabled
        if pool_config.enable_health_checks {
            pool.start_health_checker();
        }
        
        Ok(pool)
    }
    
    /// slay Initialize minimum number of connections
    fn initialize_connections(&mut self) -> Result<(), PostgreSQLError> {
        let mut available = self.available.lock().map_err(|_| {
            PostgreSQLError::new(DatabaseErrorKind::ConnectionError, "Failed to acquire pool lock".to_string())
        })?;
        
        for i in 0..self.config.min_connections {
            let mut db_config = self.db_config.clone();
            db_config.application_name = format!("{}_init_{}", self.config.app_name_prefix, i);
            
            match PostgreSQLConnection::from_config(db_config) {
                Ok(conn) => {
                    let pooled_conn = PooledConnection::new(conn);
                    available.push_back(pooled_conn);
                    
                    if let Ok(mut stats) = self.stats.lock() {
                        stats.connections_created += 1;
                        stats.idle_connections += 1;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to create initial connection {}: {}", i, e);
                    // Continue creating other connections
                }
            }
        }
        
        if available.is_empty() {
            return Err(PostgreSQLError::connection_error("Failed to create any initial connections"));
        }
        
        Ok(())
    }
    
    /// slay Start health checker thread
    fn start_health_checker(&mut self) {
        let available = self.available.clone();
        let borrowed = self.borrowed.clone();
        let stats = self.stats.clone();
        let config = self.config.clone();
        let db_config = self.db_config.clone();
        let shutdown = self.shutdown.clone();
        
        let handle = thread::spawn(move || {
            while !shutdown.load(std::sync::atomic::Ordering::Relaxed) {
                thread::sleep(config.health_check_interval);
                
                // Check available connections
                if let Ok(mut available_conns) = available.lock() {
                    let mut to_remove = Vec::new();
                    
                    for (i, conn) in available_conns.iter_mut().enumerate() {
                        if conn.is_expired(config.max_connection_lifetime) ||
                           conn.is_idle_too_long(config.max_idle_time) ||
                           !conn.validate(&config.validation_query) {
                            to_remove.push(i);
                        }
                    }
                    
                    // Remove expired/invalid connections (in reverse order to maintain indices)
                    for &i in to_remove.iter().rev() {
                        available_conns.remove(i);
                        if let Ok(mut stats) = stats.lock() {
                            stats.connections_destroyed += 1;
                            stats.idle_connections = stats.idle_connections.saturating_sub(1);
                        }
                    }
                    
                    // Ensure minimum connections
                    while available_conns.len() < config.min_connections {
                        let mut new_db_config = db_config.clone();
                        new_db_config.application_name = format!("{}_health", config.app_name_prefix);
                        
                        match PostgreSQLConnection::from_config(new_db_config) {
                            Ok(conn) => {
                                let pooled_conn = PooledConnection::new(conn);
                                available_conns.push_back(pooled_conn);
                                
                                if let Ok(mut stats) = stats.lock() {
                                    stats.connections_created += 1;
                                    stats.idle_connections += 1;
                                }
                            }
                            Err(_) => {
                                if let Ok(mut stats) = stats.lock() {
                                    stats.health_check_failures += 1;
                                }
                                break;
                            }
                        }
                    }
                }
                
                // Update stats
                if let Ok(mut stats) = stats.lock() {
                    let available_count = available.lock().map(|a| a.len()).unwrap_or(0);
                    let borrowed_count = borrowed.lock().map(|b| b.len()).unwrap_or(0);
                    
                    stats.idle_connections = available_count;
                    stats.active_connections = borrowed_count;
                    stats.peak_connections = stats.peak_connections.max(available_count + borrowed_count);
                }
            }
        });
        
        self.health_checker = Some(handle);
    }
    
    /// slay Get a connection from the pool
    pub fn get_connection(&self) -> Result<PooledConnectionWrapper, PostgreSQLError> {
        let start_time = Instant::now();
        
        let mut available = self.available.lock().map_err(|_| {
            PostgreSQLError::connection_error("Failed to acquire pool lock")
        })?;
        
        // Wait for available connection or timeout
        let (mut available, _) = self.available_notify.wait_timeout_while(
            available,
            self.config.connection_timeout,
            |available| available.is_empty() && self.can_create_connection()
        ).map_err(|_| {
            PostgreSQLError::connection_error("Failed to wait for available connection")
        })?;
        
        let mut connection = if let Some(mut pooled_conn) = available.pop_front() {
            // Validate connection if required
            if self.config.validate_on_borrow && !pooled_conn.validate(&self.config.validation_query) {
                if let Ok(mut stats) = self.stats.lock() {
                    stats.validation_failures += 1;
                }
                
                // Try to create a new connection
                return self.create_new_connection();
            }
            
            pooled_conn
        } else if self.can_create_connection() {
            // Create new connection
            let mut db_config = self.db_config.clone();
            db_config.application_name = format!("{}_dynamic", self.config.app_name_prefix);
            
            let conn = PostgreSQLConnection::from_config(db_config)?;
            PooledConnection::new(conn)
        } else {
            return Err(PostgreSQLError::connection_error("Pool exhausted and cannot create new connections"));
        };
        
        // Update statistics
        {
            let wait_time = start_time.elapsed();
            if let Ok(mut stats) = self.stats.lock() {
                stats.connections_borrowed += 1;
                stats.active_connections += 1;
                stats.idle_connections = stats.idle_connections.saturating_sub(1);
                
                // Update average wait time
                let total_borrows = stats.connections_borrowed;
                stats.avg_wait_time = Duration::from_nanos(
                    ((stats.avg_wait_time.as_nanos() * (total_borrows - 1) as u128 + wait_time.as_nanos()) / total_borrows as u128) as u64
                );
            }
        }
        
        let conn_id = connection.id.clone();
        let connection_ref = connection.borrow();
        
        // Move to borrowed connections
        {
            let mut borrowed = self.borrowed.lock().map_err(|_| {
                PostgreSQLError::connection_error("Failed to acquire borrowed connections lock")
            })?;
            borrowed.insert(conn_id.clone(), connection);
        }
        
        Ok(PooledConnectionWrapper {
            pool: self,
            connection_id: conn_id,
        })
    }
    
    /// slay Check if we can create a new connection
    fn can_create_connection(&self) -> bool {
        let available_count = self.available.lock().map(|a| a.len()).unwrap_or(0);
        let borrowed_count = self.borrowed.lock().map(|b| b.len()).unwrap_or(0);
        
        available_count + borrowed_count < self.config.max_connections
    }
    
    /// slay Create a new connection
    fn create_new_connection(&self) -> Result<PooledConnectionWrapper, PostgreSQLError> {
        if !self.can_create_connection() {
            return Err(PostgreSQLError::connection_error("Pool exhausted"));
        }
        
        let mut db_config = self.db_config.clone();
        db_config.application_name = format!("{}_new", self.config.app_name_prefix);
        
        let conn = PostgreSQLConnection::from_config(db_config)?;
        let mut pooled_conn = PooledConnection::new(conn);
        
        if let Ok(mut stats) = self.stats.lock() {
            stats.connections_created += 1;
            stats.connections_borrowed += 1;
            stats.active_connections += 1;
        }
        
        let conn_id = pooled_conn.id.clone();
        pooled_conn.borrow();
        
        {
            let mut borrowed = self.borrowed.lock().map_err(|_| {
                PostgreSQLError::connection_error("Failed to acquire borrowed connections lock")
            })?;
            borrowed.insert(conn_id.clone(), pooled_conn);
        }
        
        Ok(PooledConnectionWrapper {
            pool: self,
            connection_id: conn_id,
        })
    }
    
    /// slay Return a connection to the pool
    fn return_connection(&self, connection_id: String) -> Result<(), PostgreSQLError> {
        let mut borrowed = self.borrowed.lock().map_err(|_| {
            PostgreSQLError::connection_error("Failed to acquire borrowed connections lock")
        })?;
        
        if let Some(mut pooled_conn) = borrowed.remove(&connection_id) {
            // Validate on return if required
            if self.config.validate_on_return && !pooled_conn.validate(&self.config.validation_query) {
                if let Ok(mut stats) = self.stats.lock() {
                    stats.validation_failures += 1;
                    stats.connections_destroyed += 1;
                    stats.active_connections = stats.active_connections.saturating_sub(1);
                }
                return Ok(()); // Don't return invalid connection to pool
            }
            
            // Reset connection state for reuse
            if let Err(_) = pooled_conn.connection.reset_for_pool_reuse() {
                // Connection reset failed, don't return to pool
                if let Ok(mut stats) = self.stats.lock() {
                    stats.connections_destroyed += 1;
                    stats.active_connections = stats.active_connections.saturating_sub(1);
                }
                return Ok(());
            }
            
            pooled_conn.return_to_pool();
            
            // Add back to available connections
            {
                let mut available = self.available.lock().map_err(|_| {
                    PostgreSQLError::connection_error("Failed to acquire available connections lock")
                })?;
                available.push_back(pooled_conn);
                
                if let Ok(mut stats) = self.stats.lock() {
                    stats.connections_returned += 1;
                    stats.active_connections = stats.active_connections.saturating_sub(1);
                    stats.idle_connections += 1;
                }
            }
            
            // Notify waiting threads
            self.available_notify.notify_one();
        }
        
        Ok(())
    }
    
    /// slay Get pool statistics
    pub fn stats(&self) -> Result<PoolStats, PostgreSQLError> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| PostgreSQLError::connection_error("Failed to acquire stats lock"))
    }
    
    /// slay Get current pool size
    pub fn size(&self) -> (usize, usize) {
        let available_count = self.available.lock().map(|a| a.len()).unwrap_or(0);
        let borrowed_count = self.borrowed.lock().map(|b| b.len()).unwrap_or(0);
        (available_count, borrowed_count)
    }
    
    /// slay Close the pool and all connections
    pub fn close(&mut self) -> Result<(), PostgreSQLError> {
        // Set shutdown flag
        self.shutdown.store(true, std::sync::atomic::Ordering::Relaxed);
        
        // Wait for health checker to finish
        if let Some(handle) = self.health_checker.take() {
            let _ = handle.join();
        }
        
        // Close all available connections
        {
            let mut available = self.available.lock().map_err(|_| {
                PostgreSQLError::connection_error("Failed to acquire available connections lock")
            })?;
            
            while let Some(mut conn) = available.pop_front() {
                let _ = conn.connection.close();
            }
        }
        
        // Note: Borrowed connections will be closed when returned to the pool
        
        Ok(())
    }
}

impl Drop for PostgreSQLPool {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// fr fr Wrapper for pooled connections that automatically returns to pool on drop
pub struct PooledConnectionWrapper<'a> {
    pool: &'a PostgreSQLPool,
    connection_id: String,
}

impl<'a> PooledConnectionWrapper<'a> {
    /// slay Get reference to the underlying connection
    pub fn connection(&self) -> Result<&PostgreSQLConnection, PostgreSQLError> {
        let borrowed = self.pool.borrowed.lock().map_err(|_| {
            PostgreSQLError::connection_error("Failed to acquire borrowed connections lock")
        })?;
        
        borrowed.get(&self.connection_id)
            .map(|pooled| &pooled.connection)
            .ok_or_else(|| PostgreSQLError::connection_error("Connection not found in borrowed pool"))
    }
    
    /// slay Get mutable reference to the underlying connection
    pub fn connection_mut(&mut self) -> Result<&mut PostgreSQLConnection, PostgreSQLError> {
        let mut borrowed = self.pool.borrowed.lock().map_err(|_| {
            PostgreSQLError::connection_error("Failed to acquire borrowed connections lock")
        })?;
        
        borrowed.get_mut(&self.connection_id)
            .map(|pooled| &mut pooled.connection)
            .ok_or_else(|| PostgreSQLError::connection_error("Connection not found in borrowed pool"))
    }
}

impl<'a> Drop for PooledConnectionWrapper<'a> {
    fn drop(&mut self) {
        let _ = self.pool.return_connection(self.connection_id.clone());
    }
}

/// fr fr Pool builder for easier configuration
#[derive(Debug)]
pub struct PostgreSQLPoolBuilder {
    db_config: PostgreSQLConfig,
    pool_config: PostgreSQLPoolConfig,
}

impl PostgreSQLPoolBuilder {
    /// slay Create a new pool builder
    pub fn new(db_config: PostgreSQLConfig) -> Self {
        Self {
            db_config,
            pool_config: PostgreSQLPoolConfig::default(),
        }
    }
    
    /// slay Set minimum connections
    pub fn min_connections(mut self, min: usize) -> Self {
        self.pool_config.min_connections = min;
        self
    }
    
    /// slay Set maximum connections
    pub fn max_connections(mut self, max: usize) -> Self {
        self.pool_config.max_connections = max;
        self
    }
    
    /// slay Set connection timeout
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.pool_config.connection_timeout = timeout;
        self
    }
    
    /// slay Set max connection lifetime
    pub fn max_lifetime(mut self, lifetime: Duration) -> Self {
        self.pool_config.max_connection_lifetime = lifetime;
        self
    }
    
    /// slay Set max idle time
    pub fn max_idle_time(mut self, idle_time: Duration) -> Self {
        self.pool_config.max_idle_time = idle_time;
        self
    }
    
    /// slay Enable validation on borrow
    pub fn validate_on_borrow(mut self, validate: bool) -> Self {
        self.pool_config.validate_on_borrow = validate;
        self
    }
    
    /// slay Set application name prefix
    pub fn app_name_prefix(mut self, prefix: String) -> Self {
        self.pool_config.app_name_prefix = prefix;
        self
    }
    
    /// slay Build the pool
    pub fn build(self) -> Result<PostgreSQLPool, PostgreSQLError> {
        PostgreSQLPool::new(self.db_config, self.pool_config)
    }
}
