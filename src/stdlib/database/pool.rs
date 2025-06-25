use crate::error::CursedError;
/// fr fr Connection pool implementation for SQLSlay
/// 
/// This module provides intelligent connection pooling with configurable limits,
/// lifecycle management, and performance monitoring.
/// 
/// Why connection pooling is critical for database performance:
/// - Creating connections is expensive (network handshake, authentication)
/// - Reusing connections reduces latency and improves throughput
/// - Pool limits prevent resource exhaustion under high load
/// - Connection validation ensures reliability in long-running applications
/// - Proper cleanup prevents connection leaks and resource waste

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::time::{Duration, Instant, SystemTime};
use std::thread;
use super::{
    driver::get_driver, VibeContext
// };

/// fr fr Configuration for connection pool behavior
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// fr fr Maximum number of open connections
    /// fr fr Maximum number of idle connections to keep
    /// fr fr Maximum lifetime of a connection
    /// fr fr Maximum idle time before closing connection
    /// fr fr Timeout when acquiring connection from pool
    /// fr fr How often to check for expired connections
    /// fr fr Enable connection validation before use
    /// fr fr Retry count for failed connection creation
    /// fr fr Delay between connection retry attempts
impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            connection_max_lifetime: Duration::from_secs(3600), // 1 hour
            connection_max_idle_time: Duration::from_secs(600),  // 10 minutes
            cleanup_interval: Duration::from_secs(60),           // 1 minute
        }
    }
/// fr fr Statistics about connection pool performance
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// fr fr Current number of open connections
    /// fr fr Current number of idle connections
    /// fr fr Current number of connections in use
    /// fr fr Total connections created since pool start
    /// fr fr Total connections closed
    /// fr fr Total connection acquisition attempts
    /// fr fr Total successful acquisitions
    /// fr fr Total failed acquisitions
    /// fr fr Total time waited for connections
    /// fr fr Current number of threads waiting for connections
    /// fr fr Maximum wait time for a connection
    /// fr fr Total connections closed due to max lifetime
    /// fr fr Total connections closed due to max idle time
    /// fr fr Total failed connection validations
/// fr fr Internal connection wrapper with metadata
#[derive(Debug)]
struct PooledConnection {
    /// fr fr The actual database connection
    /// fr fr When this connection was created
    /// fr fr When this connection was last used
    /// fr fr Whether this connection is currently in use
    /// fr fr Unique identifier for this connection
impl PooledConnection {
    /// slay Create a new pooled connection
    fn new(connection: Box<dyn DriverConn>, id: u64) -> Self {
        let now = Instant::now();
        Self {
        }
    }

    /// slay Check if this connection has exceeded its lifetime
    fn is_expired(&self, max_lifetime: Duration) -> bool {
        self.created_at.elapsed() > max_lifetime
    /// slay Check if this connection has been idle too long
    fn is_idle_too_long(&self, max_idle_time: Duration) -> bool {
        !self.in_use && self.last_used_at.elapsed() > max_idle_time
    /// slay Mark this connection as in use
    fn mark_in_use(&mut self) {
        self.in_use = true;
        self.last_used_at = Instant::now();
    /// slay Mark this connection as available
    fn mark_available(&mut self) {
        self.in_use = false;
        self.last_used_at = Instant::now();
    /// slay Validate that this connection is still working
    fn validate(&self) -> bool {
        self.connection.is_alive()
    }
}

/// fr fr Internal pool state
#[derive(Debug)]
struct PoolState {
    /// fr fr Available connections
    /// fr fr All connections (idle + in-use)
    /// fr fr Next connection ID
    /// fr fr Pool statistics
    /// fr fr Whether the pool is shutting down
impl PoolState {
    fn new() -> Self {
        Self {
        }
    }
/// fr fr Main connection pool implementation
#[derive(Debug)]
pub struct ConnectionPool {
    /// fr fr Driver for creating new connections
    /// fr fr Data source name for connections
    /// fr fr Pool configuration
    /// fr fr Internal pool state
    /// fr fr Condition variable for waiting threads
    /// fr fr Background cleanup thread handle
impl ConnectionPool {
    /// slay Create a new connection pool
    pub fn new(driver_name: &str, data_source_name: &str) -> crate::error::Result<()> {
        let driver = get_driver(driver_name)?;
        let config = PoolConfig::default();
        
        let pool = Self::with_config(driver, data_source_name.to_string(), config)?;
        Ok(pool)
    /// slay Create a connection pool with custom configuration
    pub fn with_config(
        config: PoolConfig
    ) -> crate::error::Result<()> {
        let state = Arc::new(Mutex::new(PoolState::new()));
        let available_signal = Arc::new(Condvar::new());
        
        let mut pool = Self {

        // Start background cleanup thread
        pool.start_cleanup_thread();
        
        Ok(pool)
    /// slay Acquire a connection from the pool
    pub fn acquire_connection(&self, timeout: Option<Duration>) -> crate::error::Result<()> {
        let timeout = timeout.unwrap_or(self.config.connection_timeout);
        let start_time = Instant::now();
        
        loop {
            // Try to get an existing connection
            if let Some(conn) = self.try_acquire_existing_connection()? {
                return Ok(conn);
            // Try to create a new connection
            if let Some(conn) = self.try_create_new_connection()? {
                return Ok(conn);
            // Check timeout
            if start_time.elapsed() >= timeout {
                self.update_stats(|stats| {
                    stats.total_acquire_failures += 1;
                });
                return Err(DatabaseError::timeout_error("Connection acquisition timeout"));
            // Wait for a connection to become available
            self.wait_for_available_connection(timeout)?;
        }
    }

    /// slay Try to acquire an existing idle connection
    fn try_acquire_existing_connection(&self) -> crate::error::Result<()> {
        let mut state = self.state.lock().map_err(|_| {
            DatabaseError::pool_error("Failed to acquire pool state lock")
        })?;

        if state.shutting_down {
            return Err(DatabaseError::pool_error("Connection pool is shutting down"));
        // Look for an available connection
        while let Some(mut pooled_conn) = state.idle_connections.pop_front() {
            // Validate connection if enabled
            if self.config.validate_connections && !pooled_conn.validate() {
                state.stats.validation_failures += 1;
                self.remove_connection_from_state(&mut state, pooled_conn.id);
                continue;
            // Check if connection is expired
            if pooled_conn.is_expired(self.config.connection_max_lifetime) {
                state.stats.lifetime_closures += 1;
                self.remove_connection_from_state(&mut state, pooled_conn.id);
                continue;
            // Mark connection as in use
            pooled_conn.mark_in_use();
            
            // Update statistics
            state.stats.total_acquire_success += 1;
            state.stats.in_use_connections += 1;
            state.stats.idle_connections -= 1;

            // Return cloned connection
            return Ok(Some(pooled_conn.connection.clone()));
        Ok(None)
    /// slay Try to create a new connection if under limits
    fn try_create_new_connection(&self) -> crate::error::Result<()> {
        let mut state = self.state.lock().map_err(|_| {
            DatabaseError::pool_error("Failed to acquire pool state lock")
        })?;

        if state.shutting_down {
            return Err(DatabaseError::pool_error("Connection pool is shutting down"));
        // Check if we can create more connections
        if state.all_connections.len() >= self.config.max_open_connections {
            return Ok(None);
        // Create new connection with retries
        let connection = self.create_connection_with_retry()?;
        let conn_id = state.next_connection_id;
        state.next_connection_id += 1;

        // Create pooled connection
        let mut pooled_conn = PooledConnection::new(connection, conn_id);
        pooled_conn.mark_in_use();

        // Update statistics
        state.stats.total_connections_created += 1;
        state.stats.total_acquire_success += 1;
        state.stats.open_connections += 1;
        state.stats.in_use_connections += 1;

        // Add to pool state
        let result_conn = pooled_conn.connection.clone();
        state.all_connections.push(pooled_conn);

        Ok(Some(result_conn))
    /// slay Create a connection with retry logic
    fn create_connection_with_retry(&self) -> crate::error::Result<()> {
        let mut last_error = None;
        
        for attempt in 0..=self.config.connection_retry_count {
            match self.driver.open(&self.data_source_name) {
                Err(err) => {
                    last_error = Some(err);
                    if attempt < self.config.connection_retry_count {
                        thread::sleep(self.config.connection_retry_delay);
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            DatabaseError::connection_error("Failed to create connection after retries")
        }))
    /// slay Wait for a connection to become available
    fn wait_for_available_connection(&self, timeout: Duration) -> crate::error::Result<()> {
        let mut state = self.state.lock().map_err(|_| {
            DatabaseError::pool_error("Failed to acquire pool state lock")
        })?;

        state.stats.current_wait_count += 1;
        let wait_start = Instant::now();

        let result = self.available_signal.wait_timeout(state, timeout).map_err(|_| {
            DatabaseError::pool_error("Failed to wait for available connection")
        })?;

        let wait_duration = wait_start.elapsed();
        let mut state = result.0;
        
        state.stats.current_wait_count -= 1;
        state.stats.total_wait_duration += wait_duration;
        
        if wait_duration > state.stats.max_wait_duration {
            state.stats.max_wait_duration = wait_duration;
        if result.1.timed_out() {
            Err(DatabaseError::timeout_error("Timed out waiting for available connection"))
        } else {
            Ok(())
        }
    }

    /// slay Return a connection to the pool
    pub fn return_connection(&self, conn: Box<dyn DriverConn>) -> crate::error::Result<()> {
        let mut state = self.state.lock().map_err(|_| {
            DatabaseError::pool_error("Failed to acquire pool state lock")
        })?;

        if state.shutting_down {
            return Ok(()); // Just drop the connection
        // Find the connection in our tracking - collect info first to avoid borrow conflicts
        let connection_info = state.all_connections.iter_mut()
            .find(|pc| std::ptr::eq(pc.connection.as_ref(), conn.as_ref()))
            .map(|pc| {
                pc.mark_available();
                 pc.is_idle_too_long(self.config.connection_max_idle_time))
            });

        if let Some((conn_id, is_expired, is_idle_too_long)) = connection_info {
            // Check if we should keep this connection
            if state.idle_connections.len() >= self.config.max_idle_connections ||
               is_expired || is_idle_too_long {
                
                // Remove connection
                self.remove_connection_from_state(&mut state, conn_id);
            } else {
                // Return to idle pool - in a real implementation this would work differently
                // For now, we'll just update stats
                state.stats.idle_connections += 1;
            state.stats.in_use_connections -= 1;
            
            // Signal waiting threads
            self.available_signal.notify_one();
        Ok(())
    /// slay Remove a connection from internal tracking
    fn remove_connection_from_state(&self, state: &mut PoolState, conn_id: u64) {
        state.all_connections.retain(|pc| pc.id != conn_id);
        state.idle_connections.retain(|pc| pc.id != conn_id);
        state.stats.open_connections -= 1;
        state.stats.total_connections_closed += 1;
    /// slay Start background cleanup thread
    fn start_cleanup_thread(&mut self) {
        let state = Arc::clone(&self.state);
        let config = self.config.clone();
        let signal = Arc::clone(&self.available_signal);

        let handle = thread::spawn(move || {
            loop {
                thread::sleep(config.cleanup_interval);
                
                let mut pool_state = match state.lock() {

                if pool_state.shutting_down {
                    break;
                // Clean up expired connections
                let mut to_remove = Vec::new();
                
                for pooled_conn in &pool_state.all_connections {
                    if !pooled_conn.in_use {
                        if pooled_conn.is_expired(config.connection_max_lifetime) {
                            to_remove.push((pooled_conn.id, "lifetime"));
                        } else if pooled_conn.is_idle_too_long(config.connection_max_idle_time) {
                            to_remove.push((pooled_conn.id, "idle"));
                        }
                    }
                for (conn_id, reason) in to_remove {
                    pool_state.all_connections.retain(|pc| pc.id != conn_id);
                    pool_state.idle_connections.retain(|pc| pc.id != conn_id);
                    pool_state.stats.open_connections -= 1;
                    pool_state.stats.total_connections_closed += 1;
                    
                    match reason {
                        _ => {}
                    }
                drop(pool_state);
                signal.notify_all();
            }
        });

        self.cleanup_handle = Some(handle);
    /// slay Close the connection pool
    pub fn close(&self) -> crate::error::Result<()> {
        let mut state = self.state.lock().map_err(|_| {
            DatabaseError::pool_error("Failed to acquire pool state lock")
        })?;

        state.shutting_down = true;
        
        // Close all connections
        for pooled_conn in &state.all_connections {
            let _ = pooled_conn.connection.close();
        state.all_connections.clear();
        state.idle_connections.clear();
        state.stats.open_connections = 0;
        
        // Wake up waiting threads
        self.available_signal.notify_all();
        
        Ok(())
    /// slay Get current pool statistics
    pub fn stats(&self) -> crate::error::Result<()> {
        let state = self.state.lock().map_err(|_| {
            DatabaseError::pool_error("Failed to acquire pool state lock")
        })?;

        Ok(state.stats.clone())
    /// slay Set maximum open connections
    pub fn set_max_open_connections(&self, max: usize) {
        // In a real implementation, this would update the config
        // and possibly close excess connections
    /// slay Set maximum idle connections
    pub fn set_max_idle_connections(&self, max: usize) {
        // In a real implementation, this would update the config
        // and possibly close excess idle connections
    /// slay Set maximum connection lifetime
    pub fn set_max_lifetime(&self, duration: Duration) {
        // In a real implementation, this would update the config
    /// slay Set maximum idle time
    pub fn set_max_idle_time(&self, duration: Duration) {
        // In a real implementation, this would update the config
    /// slay Get the underlying driver
    pub fn get_driver(&self) -> Box<dyn Driver> {
        self.driver.clone_driver()
    /// slay Update statistics with a closure
    fn update_stats<F>(&self, f: F) 
    where 
    {
        if let Ok(mut state) = self.state.lock() {
            f(&mut state.stats);
        }
    }
impl Drop for ConnectionPool {
    fn drop(&mut self) {
        let _ = self.close();
        
        // Wait for cleanup thread to finish
        if let Some(handle) = self.cleanup_handle.take() {
            let _ = handle.join();
        }
    }
/// fr fr Connection wrapper that automatically returns to pool when dropped
pub struct PooledConnectionWrapper {
    /// fr fr The actual connection
    /// fr fr Reference to the pool for returning connection
impl PooledConnectionWrapper {
    /// slay Create a new pooled connection wrapper
    pub fn new(connection: Box<dyn DriverConn>, pool: Arc<ConnectionPool>) -> Self {
        Self {
        }
    }

    /// slay Get reference to the underlying connection
    pub fn connection(&self) -> Option<&dyn DriverConn> {
        self.connection.as_ref().map(|c| c.as_ref())
    }
}

impl Drop for PooledConnectionWrapper {
    fn drop(&mut self) {
        if let Some(conn) = self.connection.take() {
            let _ = self.pool.return_connection(conn);
        }
    }
/// fr fr Builder for creating connection pools with custom configuration
#[derive(Debug)]
pub struct ConnectionPoolBuilder {
    /// fr fr Pool configuration being built
impl ConnectionPoolBuilder {
    /// slay Create a new pool builder
    pub fn new() -> Self {
        Self {
        }
    }

    /// slay Set maximum open connections
    pub fn max_open_connections(mut self, max: usize) -> Self {
        self.config.max_open_connections = max;
        self
    /// slay Set maximum idle connections
    pub fn max_idle_connections(mut self, max: usize) -> Self {
        self.config.max_idle_connections = max;
        self
    /// slay Set connection maximum lifetime
    pub fn connection_max_lifetime(mut self, duration: Duration) -> Self {
        self.config.connection_max_lifetime = duration;
        self
    /// slay Set connection maximum idle time
    pub fn connection_max_idle_time(mut self, duration: Duration) -> Self {
        self.config.connection_max_idle_time = duration;
        self
    /// slay Set connection timeout
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.config.connection_timeout = timeout;
        self
    /// slay Enable or disable connection validation
    pub fn validate_connections(mut self, validate: bool) -> Self {
        self.config.validate_connections = validate;
        self
    /// slay Set connection retry parameters
    pub fn connection_retries(mut self, count: usize, delay: Duration) -> Self {
        self.config.connection_retry_count = count;
        self.config.connection_retry_delay = delay;
        self
    /// slay Build the connection pool
    pub fn build(self, driver: Box<dyn Driver>, data_source_name: String) -> crate::error::Result<()> {
        ConnectionPool::with_config(driver, data_source_name, self.config)
    }
}

impl Default for ConnectionPoolBuilder {
    fn default() -> Self {
        Self::new()
    }
}
