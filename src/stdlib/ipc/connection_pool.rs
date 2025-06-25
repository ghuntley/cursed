use crate::error::CursedError;
/// Connection pooling and resource management for IPC
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::time::{Duration, Instant};
use std::thread;

// use crate::stdlib::ipc::error::{IpcError, IpcResult, out_of_resources, timeout_error, not_found};

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct ConnectionPoolConfig {
    pub max_connections: usize,
    pub min_connections: usize,
    pub max_idle_time: Duration,
    pub connection_timeout: Duration,
    pub validation_interval: Duration,
    pub retry_attempts: u32,
    pub retry_delay: Duration,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            min_connections: 5,
            max_idle_time: Duration::from_secs(300), // 5 minutes
            connection_timeout: Duration::from_secs(30),
            validation_interval: Duration::from_secs(60),
            retry_attempts: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
}

/// IPC connection types
#[derive(Debug, Clone, PartialEq)]
pub enum IpcConnectionType {
    NamedPipe(String),
    UnixSocket(String),
    MessageQueue(String),
    SharedMemory(String),
    TcpSocket(String, u16),
}

/// IPC connection wrapper
pub trait IpcConnection: Send + Sync {
    fn connection_type(&self) -> IpcConnectionType;
    fn is_valid(&self) -> bool;
    fn send_data(&mut self, data: &[u8]) -> IpcResult<usize>;
    fn recv_data(&mut self, buffer: &mut [u8]) -> IpcResult<usize>;
    fn close(&mut self) -> IpcResult<()>;
    fn last_used(&self) -> Instant;
    fn update_last_used(&mut self);
}

/// Connection pool entry
struct PoolEntry {
    connection: Box<dyn IpcConnection>,
    created_at: Instant,
    last_used: Instant,
    use_count: usize,
}

/// Connection pool for IPC resources
pub struct IpcConnectionPool {
    config: ConnectionPoolConfig,
    available_connections: Mutex<VecDeque<PoolEntry>>,
    active_connections: RwLock<HashMap<usize, PoolEntry>>,
    next_connection_id: Mutex<usize>,
    stats: RwLock<PoolStatistics>,
    shutdown: Arc<std::sync::atomic::AtomicBool>,
    cleanup_condvar: Condvar,
    cleanup_mutex: Mutex<bool>,
}

/// Pool statistics
#[derive(Debug, Clone, Default)]
pub struct PoolStatistics {
    pub total_connections_created: usize,
    pub total_connections_destroyed: usize,
    pub active_connections: usize,
    pub available_connections: usize,
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub average_wait_time: Duration,
    pub peak_connections: usize,
}

impl IpcConnectionPool {
    /// Create a new connection pool
    pub fn new(config: ConnectionPoolConfig) -> Arc<Self> {
        let pool = Arc::new(Self {
            config,
            available_connections: Mutex::new(VecDeque::new()),
            active_connections: RwLock::new(HashMap::new()),
            next_connection_id: Mutex::new(0),
            stats: RwLock::new(PoolStatistics::default()),
            shutdown: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            cleanup_condvar: Condvar::new(),
            cleanup_mutex: Mutex::new(false),
        });

        // Start cleanup thread
        let pool_clone = Arc::clone(&pool);
        thread::spawn(move || {
            pool_clone.cleanup_thread();
        });

        pool
    }

    /// Get a connection from the pool
    pub fn get_connection<F>(&self, factory: F) -> IpcResult<PooledConnection>
    where
        F: FnOnce() -> IpcResult<Box<dyn IpcConnection>>,
    {
        let start_time = Instant::now();
        
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_requests += 1;
        }

        // Try to get an available connection
        if let Some(entry) = self.get_available_connection()? {
            let connection_id = self.register_active_connection(entry)?;
            
            {
                let mut stats = self.stats.write().unwrap();
                stats.successful_requests += 1;
                stats.average_wait_time = 
                    (stats.average_wait_time + start_time.elapsed()) / 2;
            }
            
            return Ok(PooledConnection::new(Arc::clone(self as &Arc<Self>), connection_id));
        }

        // Check if we can create a new connection
        let active_count = self.active_connections.read().unwrap().len();
        if active_count >= self.config.max_connections {
            let mut stats = self.stats.write().unwrap();
            stats.failed_requests += 1;
            return Err(out_of_resources("Connection pool exhausted"));
        }

        // Create a new connection
        let connection = factory()?;
        let entry = PoolEntry {
            connection,
            created_at: Instant::now(),
            last_used: Instant::now(),
            use_count: 0,
        };

        let connection_id = self.register_active_connection(entry)?;
        
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_connections_created += 1;
            stats.successful_requests += 1;
            stats.average_wait_time = 
                (stats.average_wait_time + start_time.elapsed()) / 2;
            stats.peak_connections = stats.peak_connections.max(active_count + 1);
        }

        Ok(PooledConnection::new(Arc::clone(self as &Arc<Self>), connection_id))
    }

    /// Return a connection to the pool
    pub fn return_connection(&self, connection_id: usize) -> IpcResult<()> {
        let mut active_connections = self.active_connections.write().unwrap();
        
        if let Some(mut entry) = active_connections.remove(&connection_id) {
            entry.last_used = Instant::now();
            entry.use_count += 1;
            
            // Check if connection is still valid
            if entry.connection.is_valid() {
                // Return to available pool
                let mut available = self.available_connections.lock().unwrap();
                available.push_back(entry);
                
                {
                    let mut stats = self.stats.write().unwrap();
                    stats.active_connections = active_connections.len();
                    stats.available_connections = available.len();
                }
            } else {
                // Connection is invalid, destroy it
                let mut stats = self.stats.write().unwrap();
                stats.total_connections_destroyed += 1;
                stats.active_connections = active_connections.len();
            }
        }

        Ok(())
    }

    /// Get pool statistics
    pub fn statistics(&self) -> PoolStatistics {
        self.stats.read().unwrap().clone()
    }

    /// Shutdown the pool
    pub fn shutdown(&self) -> IpcResult<()> {
        self.shutdown.store(true, std::sync::atomic::Ordering::SeqCst);
        
        // Wake up cleanup thread
        let _guard = self.cleanup_mutex.lock().unwrap();
        self.cleanup_condvar.notify_all();
        
        // Close all connections
        let mut available = self.available_connections.lock().unwrap();
        while let Some(mut entry) = available.pop_front() {
            let _ = entry.connection.close();
        }
        
        let mut active = self.active_connections.write().unwrap();
        for (_, mut entry) in active.drain() {
            let _ = entry.connection.close();
        }

        Ok(())
    }

    fn get_available_connection(&self) -> IpcResult<Option<PoolEntry>> {
        let mut available = self.available_connections.lock().unwrap();
        
        // Find a valid connection
        while let Some(entry) = available.pop_front() {
            if entry.connection.is_valid() && 
               entry.last_used.elapsed() < self.config.max_idle_time {
                return Ok(Some(entry));
            } else {
                // Connection is invalid or too old, destroy it
                let mut stats = self.stats.write().unwrap();
                stats.total_connections_destroyed += 1;
            }
        }
        
        Ok(None)
    }

    fn register_active_connection(&self, entry: PoolEntry) -> IpcResult<usize> {
        let mut active_connections = self.active_connections.write().unwrap();
        let mut connection_id = self.next_connection_id.lock().unwrap();
        
        let id = *connection_id;
        *connection_id += 1;
        
        active_connections.insert(id, entry);
        
        {
            let mut stats = self.stats.write().unwrap();
            stats.active_connections = active_connections.len();
        }
        
        Ok(id)
    }

    fn cleanup_thread(&self) {
        while !self.shutdown.load(std::sync::atomic::Ordering::SeqCst) {
            self.cleanup_idle_connections();
            
            // Wait for cleanup interval or shutdown signal
            let guard = self.cleanup_mutex.lock().unwrap();
            let _result = self.cleanup_condvar.wait_timeout(
                guard, 
                self.config.validation_interval
            ).unwrap();
        }
    }

    fn cleanup_idle_connections(&self) {
        let mut available = self.available_connections.lock().unwrap();
        let mut to_remove = Vec::new();
        
        for (index, entry) in available.iter().enumerate() {
            if entry.last_used.elapsed() > self.config.max_idle_time ||
               !entry.connection.is_valid() {
                to_remove.push(index);
            }
        }
        
        // Remove in reverse order to maintain indices
        for &index in to_remove.iter().rev() {
            if let Some(mut entry) = available.remove(index) {
                let _ = entry.connection.close();
                let mut stats = self.stats.write().unwrap();
                stats.total_connections_destroyed += 1;
                stats.available_connections = available.len();
            }
        }
        
        // Ensure minimum connections
        let current_total = available.len() + 
                           self.active_connections.read().unwrap().len();
        
        if current_total < self.config.min_connections {
            // Would need connection factory here to create minimum connections
            // This is a simplified version
        }
    }
}

/// Pooled connection wrapper
pub struct PooledConnection {
    pool: Arc<IpcConnectionPool>,
    connection_id: Option<usize>,
}

impl PooledConnection {
    fn new(pool: Arc<IpcConnectionPool>, connection_id: usize) -> Self {
        Self {
            pool,
            connection_id: Some(connection_id),
        }
    }

    /// Get the underlying connection for operations
    pub fn with_connection<F, R>(&mut self, f: F) -> IpcResult<R>
    where
        F: FnOnce(&mut dyn IpcConnection) -> IpcResult<R>,
    {
        let connection_id = self.connection_id
            .ok_or_else(|| not_found("Connection already returned to pool"))?;
        
        let mut active_connections = self.pool.active_connections.write().unwrap();
        let entry = active_connections.get_mut(&connection_id)
            .ok_or_else(|| not_found("Connection not found in pool"))?;
        
        entry.connection.update_last_used();
        f(entry.connection.as_mut())
    }

    /// Send data through the connection
    pub fn send_data(&mut self, data: &[u8]) -> IpcResult<usize> {
        self.with_connection(|conn| conn.send_data(data))
    }

    /// Receive data from the connection
    pub fn recv_data(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        self.with_connection(|conn| conn.recv_data(buffer))
    }

    /// Check if connection is valid
    pub fn is_valid(&self) -> bool {
        if let Some(connection_id) = self.connection_id {
            let active_connections = self.pool.active_connections.read().unwrap();
            if let Some(entry) = active_connections.get(&connection_id) {
                return entry.connection.is_valid();
            }
        }
        false
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(connection_id) = self.connection_id.take() {
            let _ = self.pool.return_connection(connection_id);
        }
    }
}

/// Connection factory trait
pub trait ConnectionFactory: Send + Sync {
    fn create_connection(&self, conn_type: IpcConnectionType) -> IpcResult<Box<dyn IpcConnection>>;
    fn connection_type(&self) -> IpcConnectionType;
}

/// Pool manager for multiple connection pools
pub struct IpcPoolManager {
    pools: RwLock<HashMap<String, Arc<IpcConnectionPool>>>,
    factories: RwLock<HashMap<String, Box<dyn ConnectionFactory>>>,
    default_config: ConnectionPoolConfig,
}

impl IpcPoolManager {
    /// Create a new pool manager
    pub fn new(default_config: ConnectionPoolConfig) -> Self {
        Self {
            pools: RwLock::new(HashMap::new()),
            factories: RwLock::new(HashMap::new()),
            default_config,
        }
    }

    /// Register a connection factory
    pub fn register_factory(&self, name: String, factory: Box<dyn ConnectionFactory>) -> IpcResult<()> {
        let mut factories = self.factories.write().unwrap();
        factories.insert(name, factory);
        Ok(())
    }

    /// Get or create a connection pool
    pub fn get_pool(&self, name: &str) -> IpcResult<Arc<IpcConnectionPool>> {
        {
            let pools = self.pools.read().unwrap();
            if let Some(pool) = pools.get(name) {
                return Ok(Arc::clone(pool));
            }
        }

        // Create new pool
        let pool = IpcConnectionPool::new(self.default_config.clone());
        
        {
            let mut pools = self.pools.write().unwrap();
            pools.insert(name.to_string(), Arc::clone(&pool));
        }

        Ok(pool)
    }

    /// Get a connection from a named pool
    pub fn get_connection(&self, pool_name: &str, connection_type: IpcConnectionType) -> IpcResult<PooledConnection> {
        let pool = self.get_pool(pool_name)?;
        
        // Find appropriate factory
        let factories = self.factories.read().unwrap();
        let factory = factories.values()
            .find(|f| f.connection_type() == connection_type)
            .ok_or_else(|| not_found("No factory found for connection type"))?;

        pool.get_connection(|| factory.create_connection(connection_type.clone()))
    }

    /// Shutdown all pools
    pub fn shutdown_all(&self) -> IpcResult<()> {
        let pools = self.pools.read().unwrap();
        for pool in pools.values() {
            pool.shutdown()?;
        }
        Ok(())
    }

    /// Get statistics for all pools
    pub fn get_all_statistics(&self) -> HashMap<String, PoolStatistics> {
        let pools = self.pools.read().unwrap();
        pools.iter()
            .map(|(name, pool)| (name.clone(), pool.statistics()))
            .collect()
    }
}

// Global pool manager instance
static mut GLOBAL_POOL_MANAGER: Option<IpcPoolManager> = None;
static INIT_POOL_MANAGER: std::sync::Once = std::sync::Once::new();

/// Initialize the global IPC pool manager
pub fn initialize_pool_manager(config: ConnectionPoolConfig) -> IpcResult<()> {
    unsafe {
        INIT_POOL_MANAGER.call_once(|| {
            GLOBAL_POOL_MANAGER = Some(IpcPoolManager::new(config));
        });
    }
    Ok(())
}

/// Get the global IPC pool manager
pub fn get_pool_manager() -> IpcResult<&'static IpcPoolManager> {
    unsafe {
        GLOBAL_POOL_MANAGER.as_ref()
            .ok_or_else(|| not_found("Pool manager not initialized"))
    }
}

/// Cleanup the global pool manager
pub fn cleanup_pool_manager() -> IpcResult<()> {
    if let Ok(manager) = get_pool_manager() {
        manager.shutdown_all()?;
    }
    Ok(())
}
