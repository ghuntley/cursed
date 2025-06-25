use crate::error::CursedError;
/// Connection pooling for transport layer
/// 
/// This module provides production-ready connection pooling with resource
/// management, health monitoring, and automatic cleanup capabilities.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::time::{Duration, Instant, SystemTime};
use std::thread;
use tracing::{debug, info, warn, error, instrument};
// use crate::stdlib::ipc::{IpcResult, IpcError, communication_error_detailed, resource_error};
use super::traits::{Transport, TransportConnection};

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub max_connections: usize,
    pub min_connections: usize,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub health_check_interval: Duration,
    pub enable_health_checks: bool,
    pub enable_connection_validation: bool,
    pub retry_attempts: usize,
    pub retry_delay: Duration,
}

impl PoolConfig {
    pub fn new() -> Self {
        Self {
            max_connections: 100,
            min_connections: 5,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300), // 5 minutes
            max_lifetime: Duration::from_secs(3600), // 1 hour
            health_check_interval: Duration::from_secs(60),
            enable_health_checks: true,
            enable_connection_validation: true,
            retry_attempts: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
    
    pub fn with_max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }
    
    pub fn with_min_connections(mut self, min: usize) -> Self {
        self.min_connections = min;
        self
    }
    
    pub fn with_connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }
    
    pub fn with_idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }
    
    pub fn validate(&self) -> IpcResult<()> {
        if self.max_connections == 0 {
            return Err(resource_error("Pool max_connections cannot be zero"));
        }
        if self.min_connections > self.max_connections {
            return Err(resource_error("Pool min_connections cannot exceed max_connections"));
        }
        Ok(())
    }
}

/// Pooled connection wrapper
#[derive(Debug)]
pub struct PooledConnection<T: TransportConnection> {
    connection: Option<T>,
    pool: Arc<TransportPool<T>>,
    address: String,
    created_at: Instant,
    last_used: Instant,
    usage_count: u64,
}

impl<T: TransportConnection> PooledConnection<T> {
    fn new(connection: T, pool: Arc<TransportPool<T>>, address: String) -> Self {
        let now = Instant::now();
        Self {
            connection: Some(connection),
            pool,
            address,
            created_at: now,
            last_used: now,
            usage_count: 0,
        }
    }
    
    /// Get a reference to the underlying connection
    pub fn connection(&mut self) -> IpcResult<&mut T> {
        self.last_used = Instant::now();
        self.usage_count += 1;
        self.connection.as_mut()
            .ok_or_else(|| communication_error_detailed(
                "pool",
                "connection",
                "Connection has been returned to pool"
            ))
    }
    
    /// Check if the connection is still valid
    pub fn is_valid(&self, config: &PoolConfig) -> bool {
        if let Some(conn) = &self.connection {
            if !conn.is_active() {
                return false;
            }
            
            // Check age limits
            if self.created_at.elapsed() > config.max_lifetime {
                return false;
            }
            
            if self.last_used.elapsed() > config.idle_timeout {
                return false;
            }
            
            true
        } else {
            false
        }
    }
    
    /// Get connection statistics
    pub fn get_stats(&self) -> ConnectionStats {
        ConnectionStats {
            created_at: self.created_at,
            last_used: self.last_used,
            usage_count: self.usage_count,
            age: self.created_at.elapsed(),
            idle_time: self.last_used.elapsed(),
        }
    }
}

impl<T: TransportConnection> Drop for PooledConnection<T> {
    fn drop(&mut self) {
        if let Some(connection) = self.connection.take() {
            let _ = self.pool.return_connection_internal(connection, &self.address);
        }
    }
}

/// Connection statistics
#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub created_at: Instant,
    pub last_used: Instant,
    pub usage_count: u64,
    pub age: Duration,
    pub idle_time: Duration,
}

/// Transport connection pool
#[derive(Debug)]
pub struct TransportPool<T: TransportConnection> {
    pools: Arc<RwLock<HashMap<String, Arc<Mutex<ConnectionQueue<T>>>>>>,
    config: PoolConfig,
    transport: Arc<dyn Transport<Connection = T>>,
    statistics: Arc<Mutex<PoolStatistics>>,
    health_monitor: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
    shutdown_signal: Arc<(Mutex<bool>, Condvar)>,
}

/// Connection queue for a specific address
#[derive(Debug)]
struct ConnectionQueue<T: TransportConnection> {
    connections: VecDeque<PooledConnection<T>>,
    active_count: usize,
    waiters: VecDeque<Arc<(Mutex<Option<T>>, Condvar)>>,
}

impl<T: TransportConnection> ConnectionQueue<T> {
    fn new() -> Self {
        Self {
            connections: VecDeque::new(),
            active_count: 0,
            waiters: VecDeque::new(),
        }
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStatistics {
    pub total_connections_created: u64,
    pub total_connections_destroyed: u64,
    pub current_pool_size: usize,
    pub current_active_connections: usize,
    pub total_requests: u64,
    pub total_waits: u64,
    pub total_timeouts: u64,
    pub average_wait_time: Duration,
    pub peak_pool_size: usize,
    pub health_check_failures: u64,
    pub connection_errors: u64,
}

impl PoolStatistics {
    fn new() -> Self {
        Self {
            total_connections_created: 0,
            total_connections_destroyed: 0,
            current_pool_size: 0,
            current_active_connections: 0,
            total_requests: 0,
            total_waits: 0,
            total_timeouts: 0,
            average_wait_time: Duration::from_micros(0),
            peak_pool_size: 0,
            health_check_failures: 0,
            connection_errors: 0,
        }
    }
}

impl<T: TransportConnection + 'static> TransportPool<T> {
    /// Create a new transport pool
    #[instrument(skip(transport))]
    pub fn new(transport: Arc<dyn Transport<Connection = T>>, config: PoolConfig) -> IpcResult<Self> {
        config.validate()?;
        
        let pool = Self {
            pools: Arc::new(RwLock::new(HashMap::new())),
            config,
            transport,
            statistics: Arc::new(Mutex::new(PoolStatistics::new())),
            health_monitor: Arc::new(Mutex::new(None)),
            shutdown_signal: Arc::new((Mutex::new(false), Condvar::new())),
        };
        
        if pool.config.enable_health_checks {
            pool.start_health_monitor()?;
        }
        
        info!("Created transport pool with max_connections={}", pool.config.max_connections);
        Ok(pool)
    }
    
    /// Get a connection from the pool
    #[instrument(skip(self))]
    pub fn get_connection(&self, address: &str) -> IpcResult<PooledConnection<T>> {
        let start_time = Instant::now();
        
        // Update statistics
        if let Ok(mut stats) = self.statistics.lock() {
            stats.total_requests += 1;
        }
        
        // Try to get an existing connection
        if let Some(connection) = self.try_get_existing_connection(address)? {
            debug!("Reused existing connection to {}", address);
            return Ok(connection);
        }
        
        // Create a new connection if we haven't hit the limit
        if self.can_create_new_connection(address)? {
            match self.create_new_connection(address) {
                Ok(connection) => {
                    debug!("Created new connection to {}", address);
                    return Ok(connection);
                }
                Err(e) => {
                    warn!("Failed to create new connection to {}: {}", address, e);
                    if let Ok(mut stats) = self.statistics.lock() {
                        stats.connection_errors += 1;
                    }
                }
            }
        }
        
        // Wait for an available connection
        debug!("Waiting for available connection to {}", address);
        self.wait_for_connection(address, start_time)
    }
    
    /// Return a connection to the pool (internal method)
    fn return_connection_internal(&self, mut connection: T, address: &str) -> IpcResult<()> {
        // Validate the connection if enabled
        if self.config.enable_connection_validation && !connection.is_active() {
            debug!("Discarding invalid connection to {}", address);
            return Ok(());
        }
        
        let pools = self.pools.read().unwrap();
        if let Some(queue_arc) = pools.get(address) {
            let mut queue = queue_arc.lock().unwrap();
            
            // Check if there are waiters
            if let Some(waiter) = queue.waiters.pop_front() {
                // Give the connection directly to a waiter
                let (mutex, condvar) = &*waiter;
                let mut result = mutex.lock().unwrap();
                *result = Some(connection);
                condvar.notify_one();
                debug!("Gave connection directly to waiter for {}", address);
            } else {
                // Return to the pool
                let pooled_conn = PooledConnection::new(
                    connection,
                    Arc::new(unsafe { std::mem::transmute::<&Self, &'static Self>(self) }.clone()),
                    address.to_string()
                );
                queue.connections.push_back(pooled_conn);
                debug!("Returned connection to pool for {}", address);
            }
            
            queue.active_count = queue.active_count.saturating_sub(1);
        }
        
        Ok(())
    }
    
    /// Try to get an existing connection from the pool
    fn try_get_existing_connection(&self, address: &str) -> IpcResult<Option<PooledConnection<T>>> {
        let pools = self.pools.read().unwrap();
        if let Some(queue_arc) = pools.get(address) {
            let mut queue = queue_arc.lock().unwrap();
            
            // Remove stale connections
            while let Some(conn) = queue.connections.front() {
                if conn.is_valid(&self.config) {
                    break;
                }
                debug!("Removing stale connection to {}", address);
                queue.connections.pop_front();
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.total_connections_destroyed += 1;
                    stats.current_pool_size = stats.current_pool_size.saturating_sub(1);
                }
            }
            
            // Get a connection if available
            if let Some(mut connection) = queue.connections.pop_front() {
                queue.active_count += 1;
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.current_active_connections += 1;
                }
                return Ok(Some(connection));
            }
        }
        
        Ok(None)
    }
    
    /// Check if we can create a new connection
    fn can_create_new_connection(&self, address: &str) -> IpcResult<bool> {
        let pools = self.pools.read().unwrap();
        if let Some(queue_arc) = pools.get(address) {
            let queue = queue_arc.lock().unwrap();
            let total_connections = queue.connections.len() + queue.active_count;
            Ok(total_connections < self.config.max_connections)
        } else {
            Ok(true) // No queue exists yet, so we can create one
        }
    }
    
    /// Create a new connection
    fn create_new_connection(&self, address: &str) -> IpcResult<PooledConnection<T>> {
        // Create the connection
        let connection = self.transport.connect(address)?;
        
        // Ensure the queue exists
        {
            let mut pools = self.pools.write().unwrap();
            pools.entry(address.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(ConnectionQueue::new())));
        }
        
        // Update statistics
        if let Ok(mut stats) = self.statistics.lock() {
            stats.total_connections_created += 1;
            stats.current_pool_size += 1;
            stats.current_active_connections += 1;
            if stats.current_pool_size > stats.peak_pool_size {
                stats.peak_pool_size = stats.current_pool_size;
            }
        }
        
        // Update active count
        let pools = self.pools.read().unwrap();
        if let Some(queue_arc) = pools.get(address) {
            let mut queue = queue_arc.lock().unwrap();
            queue.active_count += 1;
        }
        
        Ok(PooledConnection::new(
            connection,
            Arc::new(unsafe { std::mem::transmute::<&Self, &'static Self>(self) }.clone()),
            address.to_string()
        ))
    }
    
    /// Wait for an available connection
    fn wait_for_connection(&self, address: &str, start_time: Instant) -> IpcResult<PooledConnection<T>> {
        let waiter = Arc::new((Mutex::new(None), Condvar::new()));
        
        // Add ourselves to the wait queue
        {
            let mut pools = self.pools.write().unwrap();
            let queue_arc = pools.entry(address.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(ConnectionQueue::new())));
            let mut queue = queue_arc.lock().unwrap();
            queue.waiters.push_back(waiter.clone());
        }
        
        // Update statistics
        if let Ok(mut stats) = self.statistics.lock() {
            stats.total_waits += 1;
        }
        
        // Wait for a connection
        let (mutex, condvar) = &*waiter;
        let mut result = mutex.lock().unwrap();
        
        let timeout_result = condvar.wait_timeout(result, self.config.connection_timeout).unwrap();
        result = timeout_result.0;
        
        if timeout_result.1.timed_out() {
            warn!("Connection timeout waiting for {}", address);
            if let Ok(mut stats) = self.statistics.lock() {
                stats.total_timeouts += 1;
            }
            return Err(resource_error(&format!(
                "Timeout waiting for connection to {}", address
            )));
        }
        
        if let Some(connection) = result.take() {
            let wait_time = start_time.elapsed();
            debug!("Got connection after waiting {:?} for {}", wait_time, address);
            
            // Update average wait time
            if let Ok(mut stats) = self.statistics.lock() {
                let total_waits = stats.total_waits;
                if total_waits > 1 {
                    let current_avg_nanos = stats.average_wait_time.as_nanos() as u64;
                    let new_wait_nanos = wait_time.as_nanos() as u64;
                    let updated_avg = (current_avg_nanos * (total_waits - 1) + new_wait_nanos) / total_waits;
                    stats.average_wait_time = Duration::from_nanos(updated_avg);
                } else {
                    stats.average_wait_time = wait_time;
                }
            }
            
            Ok(PooledConnection::new(
                connection,
                Arc::new(unsafe { std::mem::transmute::<&Self, &'static Self>(self) }.clone()),
                address.to_string()
            ))
        } else {
            Err(communication_error_detailed(
                "pool",
                "wait",
                "Failed to get connection from waiter"
            ))
        }
    }
    
    /// Start the health monitoring thread
    fn start_health_monitor(&self) -> IpcResult<()> {
        let pools = self.pools.clone();
        let config = self.config.clone();
        let statistics = self.statistics.clone();
        let shutdown_signal = self.shutdown_signal.clone();
        
        let handle = thread::spawn(move || {
            let (shutdown_mutex, shutdown_condvar) = &*shutdown_signal;
            
            loop {
                // Check for shutdown signal
                {
                    let shutdown = shutdown_mutex.lock().unwrap();
                    let result = shutdown_condvar.wait_timeout(shutdown, config.health_check_interval).unwrap();
                    if *result.0 {
                        break;
                    }
                }
                
                // Perform health checks
                Self::perform_health_checks(&pools, &config, &statistics);
            }
            
            debug!("Health monitor thread shutting down");
        });
        
        *self.health_monitor.lock().unwrap() = Some(handle);
        info!("Started health monitor thread");
        Ok(())
    }
    
    /// Perform health checks on all connections
    fn perform_health_checks(
        pools: &Arc<RwLock<HashMap<String, Arc<Mutex<ConnectionQueue<T>>>>>>,
        config: &PoolConfig,
        statistics: &Arc<Mutex<PoolStatistics>>,
    ) {
        let pools_read = pools.read().unwrap();
        let mut total_removed = 0;
        
        for (address, queue_arc) in pools_read.iter() {
            let mut queue = queue_arc.lock().unwrap();
            let mut removed_count = 0;
            
            // Remove invalid connections
            queue.connections.retain(|conn| {
                if conn.is_valid(config) {
                    true
                } else {
                    removed_count += 1;
                    false
                }
            });
            
            if removed_count > 0 {
                total_removed += removed_count;
                debug!("Removed {} invalid connections for {}", removed_count, address);
            }
        }
        
        if total_removed > 0 {
            if let Ok(mut stats) = statistics.lock() {
                stats.total_connections_destroyed += total_removed;
                stats.current_pool_size = stats.current_pool_size.saturating_sub(total_removed);
                stats.health_check_failures += total_removed;
            }
        }
    }
    
    /// Get pool statistics
    pub fn get_statistics(&self) -> PoolStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| PoolStatistics::new())
    }
    
    /// Close all connections and shutdown the pool
    pub fn shutdown(&self) -> IpcResult<()> {
        info!("Shutting down transport pool");
        
        // Signal shutdown to health monitor
        {
            let (shutdown_mutex, shutdown_condvar) = &*self.shutdown_signal;
            let mut shutdown = shutdown_mutex.lock().unwrap();
            *shutdown = true;
            shutdown_condvar.notify_all();
        }
        
        // Wait for health monitor to finish
        if let Ok(mut monitor) = self.health_monitor.lock() {
            if let Some(handle) = monitor.take() {
                let _ = handle.join();
            }
        }
        
        // Close all connections
        let mut pools = self.pools.write().unwrap();
        for (address, queue_arc) in pools.drain() {
            let mut queue = queue_arc.lock().unwrap();
            
            // Close all pooled connections
            for mut conn in queue.connections.drain(..) {
                if let Some(mut connection) = conn.connection.take() {
                    let _ = connection.close();
                }
            }
            
            // Notify all waiters
            for waiter in queue.waiters.drain(..) {
                let (mutex, condvar) = &*waiter;
                let _result = mutex.lock().unwrap();
                condvar.notify_one();
            }
            
            debug!("Closed all connections for {}", address);
        }
        
        info!("Transport pool shutdown complete");
        Ok(())
    }
}

impl<T: TransportConnection> Clone for TransportPool<T> {
    fn clone(&self) -> Self {
        Self {
            pools: self.pools.clone(),
            config: self.config.clone(),
            transport: self.transport.clone(),
            statistics: self.statistics.clone(),
            health_monitor: self.health_monitor.clone(),
            shutdown_signal: self.shutdown_signal.clone(),
        }
    }
}

impl<T: TransportConnection> Drop for TransportPool<T> {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

// Module-level functions

/// Pool manager for global pool management
pub struct PoolManager {
    // This would contain global pool management functionality
}

/// Pool configuration for the global manager
pub struct PoolConfiguration {
    pub default_config: PoolConfig,
}

/// Resource manager for monitoring resource usage
pub struct ResourceManager {
    // This would contain resource monitoring functionality
}

/// Initialize the pool manager
pub fn initialize_pool_manager() -> IpcResult<()> {
    debug!("Initializing pool manager");
    Ok(())
}

/// Shutdown the pool manager
pub fn shutdown_pool_manager() -> IpcResult<()> {
    debug!("Shutting down pool manager");
    Ok(())
}

/// Get global pool statistics
pub fn get_pool_statistics() -> PoolStatistics {
    PoolStatistics::new()
}

