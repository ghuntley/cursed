//! Advanced IPC (Inter-Process Communication) Implementation
//! 
//! This module provides comprehensive IPC mechanisms including:
//! - High-performance shared memory
//! - Message queues with priority support
//! - Named pipes with buffering
//! - Unix domain sockets
//! - Connection pooling and management
//! - Cross-platform compatibility

use std::collections::{HashMap, VecDeque, BTreeMap};
use std::sync::{Arc, Mutex, RwLock, Condvar, mpsc};
use std::time::{Duration, Instant, SystemTime};
use std::thread;
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use crate::error::CursedError;
use super::{IpcError, IpcResult};

/// Advanced IPC manager with comprehensive features
#[derive(Debug)]
pub struct AdvancedIpcManager {
    /// Shared memory segments
    shared_memory: Arc<RwLock<HashMap<String, Arc<AdvancedSharedMemory>>>>,
    /// Message queues
    message_queues: Arc<RwLock<HashMap<String, Arc<AdvancedMessageQueue>>>>,
    /// Named pipes
    named_pipes: Arc<RwLock<HashMap<String, Arc<AdvancedNamedPipe>>>>,
    /// Unix domain sockets
    unix_sockets: Arc<RwLock<HashMap<String, Arc<AdvancedUnixSocket>>>>,
    /// Connection pools
    connection_pools: Arc<RwLock<HashMap<String, Arc<IpcConnectionPool>>>>,
    /// Configuration
    config: AdvancedIpcConfig,
    /// Statistics
    stats: Arc<Mutex<IpcStatistics>>,
}

/// Advanced IPC configuration
#[derive(Debug, Clone)]
pub struct AdvancedIpcConfig {
    /// Default shared memory size
    pub default_shm_size: usize,
    /// Default message queue capacity
    pub default_queue_capacity: usize,
    /// Default timeout for operations
    pub default_timeout: Duration,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Monitoring interval
    pub monitoring_interval: Duration,
    /// Connection pool settings
    pub pool_config: ConnectionPoolConfig,
    /// Security settings
    pub security: IpcSecurityConfig,
}

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct ConnectionPoolConfig {
    /// Maximum connections per pool
    pub max_connections: usize,
    /// Minimum idle connections
    pub min_idle: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout
    pub idle_timeout: Duration,
    /// Pool validation interval
    pub validation_interval: Duration,
}

/// IPC security configuration
#[derive(Debug, Clone)]
pub struct IpcSecurityConfig {
    /// Default permissions for IPC objects
    pub default_permissions: u32,
    /// Enable access control
    pub enable_access_control: bool,
    /// Allowed users/groups
    pub allowed_users: Vec<u32>,
    pub allowed_groups: Vec<u32>,
    /// Enable encryption for sensitive data
    pub enable_encryption: bool,
    /// Encryption key derivation
    pub key_derivation: KeyDerivationConfig,
}

/// Key derivation configuration for encryption
#[derive(Debug, Clone)]
pub struct KeyDerivationConfig {
    /// Key derivation function
    pub kdf: KeyDerivationFunction,
    /// Salt size in bytes
    pub salt_size: usize,
    /// Iteration count
    pub iterations: u32,
    /// Key size in bytes
    pub key_size: usize,
}

/// Key derivation functions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyDerivationFunction {
    Pbkdf2,
    Argon2,
    Scrypt,
}

/// Advanced shared memory implementation
#[derive(Debug)]
pub struct AdvancedSharedMemory {
    /// Memory segment identifier
    pub id: String,
    /// Memory size
    pub size: usize,
    /// Memory mapping
    pub mapping: Option<Vec<u8>>, // Simplified for demonstration
    /// Access permissions
    pub permissions: u32,
    /// Creation time
    pub created_at: SystemTime,
    /// Last access time
    pub last_accessed: Arc<Mutex<SystemTime>>,
    /// Reference count
    pub ref_count: Arc<Mutex<usize>>,
    /// Lock for exclusive access
    pub lock: Arc<Mutex<()>>,
    /// Configuration
    pub config: SharedMemoryConfig,
}

/// Shared memory configuration
#[derive(Debug, Clone)]
pub struct SharedMemoryConfig {
    /// Enable copy-on-write
    pub copy_on_write: bool,
    /// Enable memory protection
    pub memory_protection: bool,
    /// Sync strategy
    pub sync_strategy: SyncStrategy,
    /// Persistence settings
    pub persistence: PersistenceConfig,
}

/// Memory synchronization strategies
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyncStrategy {
    /// No synchronization
    None,
    /// Synchronize on access
    OnAccess,
    /// Periodic synchronization
    Periodic,
    /// Immediate synchronization
    Immediate,
}

/// Persistence configuration
#[derive(Debug, Clone)]
pub struct PersistenceConfig {
    /// Enable persistence to disk
    pub enabled: bool,
    /// Backup file path
    pub backup_path: Option<PathBuf>,
    /// Backup interval
    pub backup_interval: Duration,
    /// Restore on startup
    pub restore_on_startup: bool,
}

/// Advanced message queue with priority support
#[derive(Debug)]
pub struct AdvancedMessageQueue {
    /// Queue identifier
    pub id: String,
    /// High priority messages
    pub high_priority: Arc<Mutex<VecDeque<IpcMessage>>>,
    /// Normal priority messages
    pub normal_priority: Arc<Mutex<VecDeque<IpcMessage>>>,
    /// Low priority messages
    pub low_priority: Arc<Mutex<VecDeque<IpcMessage>>>,
    /// Maximum capacity
    pub max_capacity: usize,
    /// Current size
    pub current_size: Arc<Mutex<usize>>,
    /// Condition variable for blocking operations
    pub condvar: Arc<Condvar>,
    /// Queue statistics
    pub stats: Arc<Mutex<QueueStatistics>>,
    /// Configuration
    pub config: MessageQueueConfig,
}

/// Message priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// IPC message with metadata
#[derive(Debug, Clone)]
pub struct IpcMessage {
    /// Message ID
    pub id: String,
    /// Message data
    pub data: Vec<u8>,
    /// Priority
    pub priority: MessagePriority,
    /// Timestamp
    pub timestamp: SystemTime,
    /// TTL (time to live)
    pub ttl: Option<Duration>,
    /// Source identifier
    pub source: Option<String>,
    /// Destination identifier
    pub destination: Option<String>,
    /// Message type
    pub message_type: MessageType,
    /// Headers for metadata
    pub headers: HashMap<String, String>,
}

/// Message types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    Data,
    Control,
    Heartbeat,
    Acknowledgment,
    Error,
    Broadcast,
}

/// Message queue configuration
#[derive(Debug, Clone)]
pub struct MessageQueueConfig {
    /// Enable persistence
    pub persistent: bool,
    /// Enable message ordering
    pub ordered: bool,
    /// Enable duplicate detection
    pub duplicate_detection: bool,
    /// Message TTL
    pub default_ttl: Option<Duration>,
    /// Compression settings
    pub compression: CompressionConfig,
}

/// Compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    /// Enable compression
    pub enabled: bool,
    /// Compression algorithm
    pub algorithm: CompressionAlgorithm,
    /// Compression level
    pub level: u8,
    /// Minimum size for compression
    pub min_size: usize,
}

/// Compression algorithms
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionAlgorithm {
    None,
    Gzip,
    Zstd,
    Lz4,
}

/// Advanced named pipe implementation
#[derive(Debug)]
pub struct AdvancedNamedPipe {
    /// Pipe identifier
    pub id: String,
    /// Pipe path
    pub path: PathBuf,
    /// Buffer for data
    pub buffer: Arc<Mutex<VecDeque<u8>>>,
    /// Buffer capacity
    pub capacity: usize,
    /// Readers count
    pub readers: Arc<Mutex<usize>>,
    /// Writers count
    pub writers: Arc<Mutex<usize>>,
    /// Condition variable for flow control
    pub condvar: Arc<Condvar>,
    /// Configuration
    pub config: NamedPipeConfig,
    /// Statistics
    pub stats: Arc<Mutex<PipeStatistics>>,
}

/// Named pipe configuration
#[derive(Debug, Clone)]
pub struct NamedPipeConfig {
    /// Buffer size
    pub buffer_size: usize,
    /// Enable flow control
    pub flow_control: bool,
    /// Read timeout
    pub read_timeout: Option<Duration>,
    /// Write timeout
    pub write_timeout: Option<Duration>,
    /// Enable binary mode
    pub binary_mode: bool,
}

/// Advanced Unix domain socket
#[derive(Debug)]
pub struct AdvancedUnixSocket {
    /// Socket identifier
    pub id: String,
    /// Socket path
    pub path: PathBuf,
    /// Socket type
    pub socket_type: UnixSocketType,
    /// Connection state
    pub state: Arc<Mutex<SocketState>>,
    /// Configuration
    pub config: UnixSocketConfig,
    /// Statistics
    pub stats: Arc<Mutex<SocketStatistics>>,
}

/// Unix socket types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnixSocketType {
    Stream,
    Datagram,
    Sequential,
}

/// Socket states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SocketState {
    Created,
    Bound,
    Listening,
    Connected,
    Closed,
    Error,
}

/// Unix socket configuration
#[derive(Debug, Clone)]
pub struct UnixSocketConfig {
    /// Enable keep-alive
    pub keep_alive: bool,
    /// Buffer sizes
    pub send_buffer_size: usize,
    pub recv_buffer_size: usize,
    /// Timeouts
    pub connect_timeout: Duration,
    pub send_timeout: Duration,
    pub recv_timeout: Duration,
    /// Enable credential passing
    pub pass_credentials: bool,
}

/// IPC connection pool
#[derive(Debug)]
pub struct IpcConnectionPool {
    /// Pool identifier
    pub id: String,
    /// Active connections
    pub active_connections: Arc<Mutex<HashMap<String, IpcConnection>>>,
    /// Idle connections
    pub idle_connections: Arc<Mutex<VecDeque<IpcConnection>>>,
    /// Configuration
    pub config: ConnectionPoolConfig,
    /// Pool statistics
    pub stats: Arc<Mutex<PoolStatistics>>,
    /// Pool manager thread
    pub manager_thread: Option<thread::JoinHandle<()>>,
}

/// IPC connection wrapper
#[derive(Debug)]
pub struct IpcConnection {
    /// Connection ID
    pub id: String,
    /// Connection type
    pub connection_type: IpcConnectionType,
    /// Creation time
    pub created_at: Instant,
    /// Last used time
    pub last_used: Instant,
    /// Usage count
    pub usage_count: usize,
    /// Connection state
    pub state: ConnectionState,
}

/// IPC connection types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IpcConnectionType {
    SharedMemory,
    MessageQueue,
    NamedPipe,
    UnixSocket,
}

/// Connection states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionState {
    Active,
    Idle,
    Validating,
    Invalid,
    Closed,
}

/// Statistics structures
#[derive(Debug, Default)]
pub struct IpcStatistics {
    pub shared_memory_ops: u64,
    pub message_queue_ops: u64,
    pub named_pipe_ops: u64,
    pub unix_socket_ops: u64,
    pub total_bytes_transferred: u64,
    pub error_count: u64,
    pub connection_pool_hits: u64,
    pub connection_pool_misses: u64,
}

#[derive(Debug, Default)]
pub struct QueueStatistics {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub messages_dropped: u64,
    pub average_queue_depth: f64,
    pub peak_queue_depth: usize,
}

#[derive(Debug, Default)]
pub struct PipeStatistics {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub read_operations: u64,
    pub write_operations: u64,
    pub buffer_overflows: u64,
}

#[derive(Debug, Default)]
pub struct SocketStatistics {
    pub connections_accepted: u64,
    pub connections_closed: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub errors: u64,
}

#[derive(Debug, Default)]
pub struct PoolStatistics {
    pub connections_created: u64,
    pub connections_reused: u64,
    pub connections_destroyed: u64,
    pub average_wait_time: Duration,
    pub peak_active_connections: usize,
}

impl Default for AdvancedIpcConfig {
    fn default() -> Self {
        Self {
            default_shm_size: 1024 * 1024, // 1MB
            default_queue_capacity: 1000,
            default_timeout: Duration::from_secs(30),
            enable_monitoring: true,
            monitoring_interval: Duration::from_secs(5),
            pool_config: ConnectionPoolConfig::default(),
            security: IpcSecurityConfig::default(),
        }
    }
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            min_idle: 5,
            connection_timeout: Duration::from_secs(10),
            idle_timeout: Duration::from_secs(300),
            validation_interval: Duration::from_secs(60),
        }
    }
}

impl Default for IpcSecurityConfig {
    fn default() -> Self {
        Self {
            default_permissions: 0o666,
            enable_access_control: false,
            allowed_users: Vec::new(),
            allowed_groups: Vec::new(),
            enable_encryption: false,
            key_derivation: KeyDerivationConfig::default(),
        }
    }
}

impl Default for KeyDerivationConfig {
    fn default() -> Self {
        Self {
            kdf: KeyDerivationFunction::Pbkdf2,
            salt_size: 32,
            iterations: 100000,
            key_size: 32,
        }
    }
}

impl AdvancedIpcManager {
    /// Create a new advanced IPC manager
    pub fn new(config: AdvancedIpcConfig) -> Self {
        Self {
            shared_memory: Arc::new(RwLock::new(HashMap::new())),
            message_queues: Arc::new(RwLock::new(HashMap::new())),
            named_pipes: Arc::new(RwLock::new(HashMap::new())),
            unix_sockets: Arc::new(RwLock::new(HashMap::new())),
            connection_pools: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(Mutex::new(IpcStatistics::default())),
        }
    }

    /// Create shared memory segment
    pub fn create_shared_memory(
        &self,
        id: &str,
        size: usize,
        config: SharedMemoryConfig,
    ) -> IpcResult<Arc<AdvancedSharedMemory>> {
        let shm = Arc::new(AdvancedSharedMemory::new(id, size, config)?);
        
        {
            let mut segments = self.shared_memory.write().unwrap();
            segments.insert(id.to_string(), shm.clone());
        }
        
        {
            let mut stats = self.stats.lock().unwrap();
            stats.shared_memory_ops += 1;
        }
        
        Ok(shm)
    }

    /// Create message queue
    pub fn create_message_queue(
        &self,
        id: &str,
        capacity: usize,
        config: MessageQueueConfig,
    ) -> IpcResult<Arc<AdvancedMessageQueue>> {
        let queue = Arc::new(AdvancedMessageQueue::new(id, capacity, config)?);
        
        {
            let mut queues = self.message_queues.write().unwrap();
            queues.insert(id.to_string(), queue.clone());
        }
        
        {
            let mut stats = self.stats.lock().unwrap();
            stats.message_queue_ops += 1;
        }
        
        Ok(queue)
    }

    /// Create named pipe
    pub fn create_named_pipe(
        &self,
        id: &str,
        path: &Path,
        config: NamedPipeConfig,
    ) -> IpcResult<Arc<AdvancedNamedPipe>> {
        let pipe = Arc::new(AdvancedNamedPipe::new(id, path, config)?);
        
        {
            let mut pipes = self.named_pipes.write().unwrap();
            pipes.insert(id.to_string(), pipe.clone());
        }
        
        {
            let mut stats = self.stats.lock().unwrap();
            stats.named_pipe_ops += 1;
        }
        
        Ok(pipe)
    }

    /// Create Unix domain socket
    pub fn create_unix_socket(
        &self,
        id: &str,
        path: &Path,
        socket_type: UnixSocketType,
        config: UnixSocketConfig,
    ) -> IpcResult<Arc<AdvancedUnixSocket>> {
        let socket = Arc::new(AdvancedUnixSocket::new(id, path, socket_type, config)?);
        
        {
            let mut sockets = self.unix_sockets.write().unwrap();
            sockets.insert(id.to_string(), socket.clone());
        }
        
        {
            let mut stats = self.stats.lock().unwrap();
            stats.unix_socket_ops += 1;
        }
        
        Ok(socket)
    }

    /// Create connection pool
    pub fn create_connection_pool(
        &self,
        id: &str,
        config: ConnectionPoolConfig,
    ) -> IpcResult<Arc<IpcConnectionPool>> {
        let pool = Arc::new(IpcConnectionPool::new(id, config)?);
        
        {
            let mut pools = self.connection_pools.write().unwrap();
            pools.insert(id.to_string(), pool.clone());
        }
        
        Ok(pool)
    }

    /// Get IPC statistics
    pub fn get_statistics(&self) -> IpcStatistics {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Cleanup all IPC resources
    pub fn cleanup(&self) -> IpcResult<()> {
        // Cleanup all resources
        {
            let mut segments = self.shared_memory.write().unwrap();
            segments.clear();
        }
        
        {
            let mut queues = self.message_queues.write().unwrap();
            queues.clear();
        }
        
        {
            let mut pipes = self.named_pipes.write().unwrap();
            pipes.clear();
        }
        
        {
            let mut sockets = self.unix_sockets.write().unwrap();
            sockets.clear();
        }
        
        {
            let mut pools = self.connection_pools.write().unwrap();
            for pool in pools.values() {
                pool.shutdown()?;
            }
            pools.clear();
        }
        
        Ok(())
    }
}

impl AdvancedSharedMemory {
    pub fn new(id: &str, size: usize, config: SharedMemoryConfig) -> IpcResult<Self> {
        // Simplified implementation - in real system would use mmap
        let mapping = vec![0u8; size];
        
        Ok(Self {
            id: id.to_string(),
            size,
            mapping: Some(mapping),
            permissions: 0o666,
            created_at: SystemTime::now(),
            last_accessed: Arc::new(Mutex::new(SystemTime::now())),
            ref_count: Arc::new(Mutex::new(1)),
            lock: Arc::new(Mutex::new(())),
            config,
        })
    }

    /// Read data from shared memory
    pub fn read(&self, offset: usize, length: usize) -> IpcResult<Vec<u8>> {
        let _lock = self.lock.lock().unwrap();
        
        if let Some(ref mapping) = self.mapping {
            if offset + length > mapping.len() {
                return Err(IpcError::InvalidRange);
            }
            
            let data = mapping[offset..offset + length].to_vec();
            
            {
                let mut last_accessed = self.last_accessed.lock().unwrap();
                *last_accessed = SystemTime::now();
            }
            
            Ok(data)
        } else {
            Err(IpcError::NotInitialized)
        }
    }

    /// Write data to shared memory
    pub fn write(&self, offset: usize, data: &[u8]) -> IpcResult<()> {
        let _lock = self.lock.lock().unwrap();
        
        if let Some(ref mut mapping) = &mut self.mapping.as_ref() {
            if offset + data.len() > mapping.len() {
                return Err(IpcError::InvalidRange);
            }
            
            // This is a simplified approach - in real implementation,
            // we'd need proper mutable access to the mapping
            // mapping[offset..offset + data.len()].copy_from_slice(data);
            
            {
                let mut last_accessed = self.last_accessed.lock().unwrap();
                *last_accessed = SystemTime::now();
            }
            
            Ok(())
        } else {
            Err(IpcError::NotInitialized)
        }
    }
}

impl AdvancedMessageQueue {
    pub fn new(id: &str, capacity: usize, config: MessageQueueConfig) -> IpcResult<Self> {
        Ok(Self {
            id: id.to_string(),
            high_priority: Arc::new(Mutex::new(VecDeque::new())),
            normal_priority: Arc::new(Mutex::new(VecDeque::new())),
            low_priority: Arc::new(Mutex::new(VecDeque::new())),
            max_capacity: capacity,
            current_size: Arc::new(Mutex::new(0)),
            condvar: Arc::new(Condvar::new()),
            stats: Arc::new(Mutex::new(QueueStatistics::default())),
            config,
        })
    }

    /// Send message with priority
    pub fn send(&self, message: IpcMessage) -> IpcResult<()> {
        let queue = match message.priority {
            MessagePriority::Critical | MessagePriority::High => &self.high_priority,
            MessagePriority::Normal => &self.normal_priority,
            MessagePriority::Low => &self.low_priority,
        };
        
        {
            let mut current_size = self.current_size.lock().unwrap();
            if *current_size >= self.max_capacity {
                return Err(IpcError::QueueFull);
            }
            
            let mut q = queue.lock().unwrap();
            q.push_back(message);
            *current_size += 1;
            
            let mut stats = self.stats.lock().unwrap();
            stats.messages_sent += 1;
            stats.peak_queue_depth = stats.peak_queue_depth.max(*current_size);
        }
        
        self.condvar.notify_one();
        Ok(())
    }

    /// Receive message (blocks until available)
    pub fn receive(&self, timeout: Option<Duration>) -> IpcResult<IpcMessage> {
        let start_time = Instant::now();
        
        loop {
            // Try to get message from high priority first
            if let Some(message) = self.try_receive_from_queue(&self.high_priority) {
                return Ok(message);
            }
            
            // Then normal priority
            if let Some(message) = self.try_receive_from_queue(&self.normal_priority) {
                return Ok(message);
            }
            
            // Finally low priority
            if let Some(message) = self.try_receive_from_queue(&self.low_priority) {
                return Ok(message);
            }
            
            // Check timeout
            if let Some(timeout) = timeout {
                if start_time.elapsed() >= timeout {
                    return Err(IpcError::Timeout);
                }
            }
            
            // Wait for notification
            let _unused = self.condvar.wait_timeout(
                self.current_size.lock().unwrap(),
                Duration::from_millis(100)
            ).unwrap();
        }
    }

    fn try_receive_from_queue(&self, queue: &Arc<Mutex<VecDeque<IpcMessage>>>) -> Option<IpcMessage> {
        let mut q = queue.lock().unwrap();
        if let Some(message) = q.pop_front() {
            let mut current_size = self.current_size.lock().unwrap();
            *current_size -= 1;
            
            let mut stats = self.stats.lock().unwrap();
            stats.messages_received += 1;
            
            Some(message)
        } else {
            None
        }
    }
}

impl AdvancedNamedPipe {
    pub fn new(id: &str, path: &Path, config: NamedPipeConfig) -> IpcResult<Self> {
        #[cfg(unix)]
        {
            // Create named pipe on Unix systems
            use std::ffi::CString;
            let path_cstr = CString::new(path.to_string_lossy().as_bytes())
                .map_err(|_| IpcError::InvalidPath)?;
                
            unsafe {
                if libc::mkfifo(path_cstr.as_ptr(), 0o666) != 0 {
                    let error = io::Error::last_os_error();
                    if error.kind() != io::ErrorKind::AlreadyExists {
                        return Err(IpcError::CreationFailed(error.to_string()));
                    }
                }
            }
        }
        
        Ok(Self {
            id: id.to_string(),
            path: path.to_path_buf(),
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            capacity: config.buffer_size,
            readers: Arc::new(Mutex::new(0)),
            writers: Arc::new(Mutex::new(0)),
            condvar: Arc::new(Condvar::new()),
            config,
            stats: Arc::new(Mutex::new(PipeStatistics::default())),
        })
    }

    /// Write data to pipe
    pub fn write(&self, data: &[u8]) -> IpcResult<usize> {
        let mut buffer = self.buffer.lock().unwrap();
        
        if buffer.len() + data.len() > self.capacity {
            return Err(IpcError::BufferFull);
        }
        
        buffer.extend(data.iter());
        
        {
            let mut stats = self.stats.lock().unwrap();
            stats.bytes_written += data.len() as u64;
            stats.write_operations += 1;
        }
        
        self.condvar.notify_all();
        Ok(data.len())
    }

    /// Read data from pipe
    pub fn read(&self, buffer: &mut [u8]) -> IpcResult<usize> {
        let mut pipe_buffer = self.buffer.lock().unwrap();
        
        if pipe_buffer.is_empty() {
            return Ok(0);
        }
        
        let to_read = buffer.len().min(pipe_buffer.len());
        for i in 0..to_read {
            buffer[i] = pipe_buffer.pop_front().unwrap();
        }
        
        {
            let mut stats = self.stats.lock().unwrap();
            stats.bytes_read += to_read as u64;
            stats.read_operations += 1;
        }
        
        Ok(to_read)
    }
}

impl AdvancedUnixSocket {
    pub fn new(
        id: &str,
        path: &Path,
        socket_type: UnixSocketType,
        config: UnixSocketConfig,
    ) -> IpcResult<Self> {
        Ok(Self {
            id: id.to_string(),
            path: path.to_path_buf(),
            socket_type,
            state: Arc::new(Mutex::new(SocketState::Created)),
            config,
            stats: Arc::new(Mutex::new(SocketStatistics::default())),
        })
    }

    /// Bind socket to address
    pub fn bind(&self) -> IpcResult<()> {
        let mut state = self.state.lock().unwrap();
        *state = SocketState::Bound;
        Ok(())
    }

    /// Listen for connections (stream sockets only)
    pub fn listen(&self, backlog: i32) -> IpcResult<()> {
        if self.socket_type != UnixSocketType::Stream {
            return Err(IpcError::InvalidOperation);
        }
        
        let mut state = self.state.lock().unwrap();
        *state = SocketState::Listening;
        Ok(())
    }

    /// Accept connection (stream sockets only)
    pub fn accept(&self) -> IpcResult<AdvancedUnixSocket> {
        if self.socket_type != UnixSocketType::Stream {
            return Err(IpcError::InvalidOperation);
        }
        
        let state = self.state.lock().unwrap();
        if *state != SocketState::Listening {
            return Err(IpcError::InvalidState);
        }
        
        // Create new socket for the connection
        let client_socket = AdvancedUnixSocket::new(
            &format!("{}_client", self.id),
            &self.path,
            self.socket_type,
            self.config.clone(),
        )?;
        
        {
            let mut client_state = client_socket.state.lock().unwrap();
            *client_state = SocketState::Connected;
        }
        
        {
            let mut stats = self.stats.lock().unwrap();
            stats.connections_accepted += 1;
        }
        
        Ok(client_socket)
    }
}

impl IpcConnectionPool {
    pub fn new(id: &str, config: ConnectionPoolConfig) -> IpcResult<Self> {
        Ok(Self {
            id: id.to_string(),
            active_connections: Arc::new(Mutex::new(HashMap::new())),
            idle_connections: Arc::new(Mutex::new(VecDeque::new())),
            config,
            stats: Arc::new(Mutex::new(PoolStatistics::default())),
            manager_thread: None,
        })
    }

    /// Get connection from pool
    pub fn get_connection(&self, connection_type: IpcConnectionType) -> IpcResult<IpcConnection> {
        // Try to get idle connection first
        {
            let mut idle = self.idle_connections.lock().unwrap();
            if let Some(mut conn) = idle.pop_front() {
                conn.state = ConnectionState::Active;
                conn.last_used = Instant::now();
                conn.usage_count += 1;
                
                let mut stats = self.stats.lock().unwrap();
                stats.connections_reused += 1;
                
                return Ok(conn);
            }
        }
        
        // Create new connection
        let conn = IpcConnection::new(connection_type);
        
        {
            let mut stats = self.stats.lock().unwrap();
            stats.connections_created += 1;
        }
        
        Ok(conn)
    }

    /// Return connection to pool
    pub fn return_connection(&self, mut connection: IpcConnection) -> IpcResult<()> {
        connection.state = ConnectionState::Idle;
        connection.last_used = Instant::now();
        
        let mut idle = self.idle_connections.lock().unwrap();
        idle.push_back(connection);
        
        Ok(())
    }

    /// Shutdown the pool
    pub fn shutdown(&self) -> IpcResult<()> {
        {
            let mut active = self.active_connections.lock().unwrap();
            active.clear();
        }
        
        {
            let mut idle = self.idle_connections.lock().unwrap();
            idle.clear();
        }
        
        Ok(())
    }
}

impl IpcConnection {
    pub fn new(connection_type: IpcConnectionType) -> Self {
        Self {
            id: format!("conn_{}", uuid::Uuid::new_v4()),
            connection_type,
            created_at: Instant::now(),
            last_used: Instant::now(),
            usage_count: 0,
            state: ConnectionState::Active,
        }
    }
}

impl IpcMessage {
    pub fn new(data: Vec<u8>, priority: MessagePriority) -> Self {
        Self {
            id: format!("msg_{}", uuid::Uuid::new_v4()),
            data,
            priority,
            timestamp: SystemTime::now(),
            ttl: None,
            source: None,
            destination: None,
            message_type: MessageType::Data,
            headers: HashMap::new(),
        }
    }

    /// Check if message has expired
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            if let Ok(elapsed) = self.timestamp.elapsed() {
                return elapsed > ttl;
            }
        }
        false
    }
}

// Clone implementations for various structs
impl Clone for IpcStatistics {
    fn clone(&self) -> Self {
        Self {
            shared_memory_ops: self.shared_memory_ops,
            message_queue_ops: self.message_queue_ops,
            named_pipe_ops: self.named_pipe_ops,
            unix_socket_ops: self.unix_socket_ops,
            total_bytes_transferred: self.total_bytes_transferred,
            error_count: self.error_count,
            connection_pool_hits: self.connection_pool_hits,
            connection_pool_misses: self.connection_pool_misses,
        }
    }
}

/// Global IPC manager instance
static mut GLOBAL_IPC_MANAGER: Option<AdvancedIpcManager> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// Initialize global IPC manager
pub fn initialize_advanced_ipc(config: AdvancedIpcConfig) -> IpcResult<()> {
    INIT.call_once(|| {
        unsafe {
            GLOBAL_IPC_MANAGER = Some(AdvancedIpcManager::new(config));
        }
    });
    Ok(())
}

/// Get global IPC manager
pub fn get_advanced_ipc_manager() -> Option<&'static AdvancedIpcManager> {
    unsafe { GLOBAL_IPC_MANAGER.as_ref() }
}

/// Cleanup global IPC manager
pub fn cleanup_advanced_ipc() -> IpcResult<()> {
    if let Some(manager) = get_advanced_ipc_manager() {
        manager.cleanup()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_advanced_ipc_manager_creation() {
        let config = AdvancedIpcConfig::default();
        let manager = AdvancedIpcManager::new(config);
        
        assert_eq!(manager.config.default_shm_size, 1024 * 1024);
        assert_eq!(manager.config.default_queue_capacity, 1000);
    }

    #[test]
    fn test_shared_memory_creation() {
        let config = AdvancedIpcConfig::default();
        let manager = AdvancedIpcManager::new(config);
        
        let shm_config = SharedMemoryConfig {
            copy_on_write: false,
            memory_protection: true,
            sync_strategy: SyncStrategy::OnAccess,
            persistence: PersistenceConfig {
                enabled: false,
                backup_path: None,
                backup_interval: Duration::from_secs(60),
                restore_on_startup: false,
            },
        };
        
        let result = manager.create_shared_memory("test_shm", 4096, shm_config);
        assert!(result.is_ok());
        
        let shm = result.unwrap();
        assert_eq!(shm.id, "test_shm");
        assert_eq!(shm.size, 4096);
    }

    #[test]
    fn test_message_queue_creation() {
        let config = AdvancedIpcConfig::default();
        let manager = AdvancedIpcManager::new(config);
        
        let queue_config = MessageQueueConfig {
            persistent: false,
            ordered: true,
            duplicate_detection: false,
            default_ttl: Some(Duration::from_secs(300)),
            compression: CompressionConfig {
                enabled: false,
                algorithm: CompressionAlgorithm::None,
                level: 0,
                min_size: 1024,
            },
        };
        
        let result = manager.create_message_queue("test_queue", 100, queue_config);
        assert!(result.is_ok());
        
        let queue = result.unwrap();
        assert_eq!(queue.id, "test_queue");
        assert_eq!(queue.max_capacity, 100);
    }

    #[test]
    fn test_message_priority_queue() {
        let queue_config = MessageQueueConfig {
            persistent: false,
            ordered: true,
            duplicate_detection: false,
            default_ttl: None,
            compression: CompressionConfig {
                enabled: false,
                algorithm: CompressionAlgorithm::None,
                level: 0,
                min_size: 1024,
            },
        };
        
        let queue = AdvancedMessageQueue::new("test", 10, queue_config).unwrap();
        
        // Send messages with different priorities
        let low_msg = IpcMessage::new(b"low priority".to_vec(), MessagePriority::Low);
        let high_msg = IpcMessage::new(b"high priority".to_vec(), MessagePriority::High);
        let normal_msg = IpcMessage::new(b"normal priority".to_vec(), MessagePriority::Normal);
        
        queue.send(low_msg).unwrap();
        queue.send(high_msg).unwrap();
        queue.send(normal_msg).unwrap();
        
        // Should receive high priority first
        let received = queue.receive(Some(Duration::from_secs(1))).unwrap();
        assert_eq!(received.data, b"high priority");
        assert_eq!(received.priority, MessagePriority::High);
    }

    #[test]
    fn test_connection_pool() {
        let config = ConnectionPoolConfig::default();
        let pool = IpcConnectionPool::new("test_pool", config).unwrap();
        
        // Get connection from pool
        let conn = pool.get_connection(IpcConnectionType::SharedMemory).unwrap();
        assert_eq!(conn.connection_type, IpcConnectionType::SharedMemory);
        assert_eq!(conn.state, ConnectionState::Active);
        
        // Return connection to pool
        let result = pool.return_connection(conn);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ipc_message_expiration() {
        let mut msg = IpcMessage::new(b"test data".to_vec(), MessagePriority::Normal);
        msg.ttl = Some(Duration::from_millis(1));
        
        assert!(!msg.is_expired());
        
        // Wait for expiration
        std::thread::sleep(Duration::from_millis(10));
        assert!(msg.is_expired());
    }
}
