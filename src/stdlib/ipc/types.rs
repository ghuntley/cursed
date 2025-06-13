/// Core IPC types and data structures for CURSED
/// 
/// This module defines the fundamental types used throughout the IPC system,
/// including handles, permissions, configurations, and resource identifiers.

use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, SystemTime};

/// Process identifier
pub type ProcessId = u32;

/// IPC handle type identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IpcHandleType {
    SharedMemory,
    NamedPipe,
    MessageQueue,
    Semaphore,
    DomainSocket,
    RpcConnection,
}

impl fmt::Display for IpcHandleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcHandleType::SharedMemory => write!(f, "SharedMemory"),
            IpcHandleType::NamedPipe => write!(f, "NamedPipe"),
            IpcHandleType::MessageQueue => write!(f, "MessageQueue"),
            IpcHandleType::Semaphore => write!(f, "Semaphore"),
            IpcHandleType::DomainSocket => write!(f, "DomainSocket"),
            IpcHandleType::RpcConnection => write!(f, "RpcConnection"),
        }
    }
}

/// IPC handle - universal identifier for IPC resources
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IpcHandle {
    /// Unique identifier for the resource
    pub id: String,
    /// Type of IPC resource
    pub handle_type: IpcHandleType,
    /// Creation timestamp
    pub created_at: SystemTime,
    /// Process ID that created this handle
    pub creator_pid: ProcessId,
    /// Optional metadata
    pub metadata: HashMap<String, String>,
}

impl IpcHandle {
    /// Create a new IPC handle
    pub fn new(id: String, handle_type: IpcHandleType) -> Self {
        Self {
            id,
            handle_type,
            created_at: SystemTime::now(),
            creator_pid: std::process::id(),
            metadata: HashMap::new(),
        }
    }

    /// Create handle with metadata
    pub fn with_metadata(id: String, handle_type: IpcHandleType, metadata: HashMap<String, String>) -> Self {
        Self {
            id,
            handle_type,
            created_at: SystemTime::now(),
            creator_pid: std::process::id(),
            metadata,
        }
    }

    /// Get age of the handle
    pub fn age(&self) -> Duration {
        self.created_at.elapsed().unwrap_or(Duration::from_secs(0))
    }

    /// Set metadata value
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Check if handle is owned by current process
    pub fn is_owned_by_current_process(&self) -> bool {
        self.creator_pid == std::process::id()
    }
}

impl fmt::Display for IpcHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.handle_type, self.id)
    }
}

/// IPC permissions system
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcPermissions {
    /// Read permission
    pub read: bool,
    /// Write permission
    pub write: bool,
    /// Execute permission (for some resource types)
    pub execute: bool,
    /// Delete permission
    pub delete: bool,
    /// Admin/control permission
    pub admin: bool,
    /// Owner user ID (Unix)
    pub owner_uid: Option<u32>,
    /// Owner group ID (Unix)
    pub owner_gid: Option<u32>,
    /// Permission mask
    pub mask: u32,
}

impl IpcPermissions {
    /// Create read-only permissions
    pub fn read_only() -> Self {
        Self {
            read: true,
            write: false,
            execute: false,
            delete: false,
            admin: false,
            owner_uid: None,
            owner_gid: None,
            mask: 0o444,
        }
    }

    /// Create write-only permissions
    pub fn write_only() -> Self {
        Self {
            read: false,
            write: true,
            execute: false,
            delete: false,
            admin: false,
            owner_uid: None,
            owner_gid: None,
            mask: 0o222,
        }
    }

    /// Create read-write permissions
    pub fn read_write() -> Self {
        Self {
            read: true,
            write: true,
            execute: false,
            delete: false,
            admin: false,
            owner_uid: None,
            owner_gid: None,
            mask: 0o666,
        }
    }

    /// Create full permissions
    pub fn full() -> Self {
        Self {
            read: true,
            write: true,
            execute: true,
            delete: true,
            admin: true,
            owner_uid: None,
            owner_gid: None,
            mask: 0o777,
        }
    }

    /// Create permissions from octal mode
    pub fn from_octal(mode: u32) -> Self {
        Self {
            read: (mode & 0o444) != 0,
            write: (mode & 0o222) != 0,
            execute: (mode & 0o111) != 0,
            delete: (mode & 0o200) != 0, // Write permission implies delete
            admin: (mode & 0o700) == 0o700, // Full owner permissions
            owner_uid: None,
            owner_gid: None,
            mask: mode,
        }
    }

    /// Convert to octal mode
    pub fn to_octal(&self) -> u32 {
        self.mask
    }

    /// Check if can read
    pub fn can_read(&self) -> bool {
        self.read
    }

    /// Check if can write
    pub fn can_write(&self) -> bool {
        self.write
    }

    /// Check if can execute
    pub fn can_execute(&self) -> bool {
        self.execute
    }

    /// Check if can delete
    pub fn can_delete(&self) -> bool {
        self.delete
    }

    /// Check if has admin rights
    pub fn can_admin(&self) -> bool {
        self.admin
    }

    /// Set owner information
    pub fn with_owner(mut self, uid: u32, gid: u32) -> Self {
        self.owner_uid = Some(uid);
        self.owner_gid = Some(gid);
        self
    }

    /// Check if permissions allow operation
    pub fn allows(&self, operation: &str) -> bool {
        match operation {
            "read" => self.read,
            "write" => self.write,
            "execute" => self.execute,
            "delete" => self.delete,
            "admin" => self.admin,
            _ => false,
        }
    }
}

/// IPC access mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcMode {
    /// Read-only access
    ReadOnly,
    /// Write-only access
    WriteOnly,
    /// Read-write access
    ReadWrite,
    /// Append mode (write-only, append to end)
    Append,
    /// Create and write (fail if exists)
    CreateNew,
    /// Create or truncate and write
    CreateOrTruncate,
}

impl IpcMode {
    /// Check if mode allows reading
    pub fn can_read(&self) -> bool {
        matches!(self, IpcMode::ReadOnly | IpcMode::ReadWrite)
    }

    /// Check if mode allows writing
    pub fn can_write(&self) -> bool {
        matches!(
            self,
            IpcMode::WriteOnly | IpcMode::ReadWrite | IpcMode::Append | 
            IpcMode::CreateNew | IpcMode::CreateOrTruncate
        )
    }

    /// Check if mode creates new resource
    pub fn creates_new(&self) -> bool {
        matches!(self, IpcMode::CreateNew | IpcMode::CreateOrTruncate)
    }
}

/// Resource identifiers for different IPC types
pub type SharedMemoryId = String;
pub type MessageQueueId = String;
pub type SemaphoreId = String;
pub type PipeId = String;

/// IPC timeout configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcTimeout {
    /// Connect timeout
    pub connect: Option<Duration>,
    /// Read timeout
    pub read: Option<Duration>,
    /// Write timeout
    pub write: Option<Duration>,
    /// Operation timeout
    pub operation: Option<Duration>,
    /// Idle timeout
    pub idle: Option<Duration>,
}

impl IpcTimeout {
    /// Create timeout with all operations having the same duration
    pub fn uniform(duration: Duration) -> Self {
        Self {
            connect: Some(duration),
            read: Some(duration),
            write: Some(duration),
            operation: Some(duration),
            idle: Some(duration),
        }
    }

    /// Create timeout with no limits
    pub fn infinite() -> Self {
        Self {
            connect: None,
            read: None,
            write: None,
            operation: None,
            idle: None,
        }
    }

    /// Create default timeout configuration
    pub fn default() -> Self {
        Self {
            connect: Some(Duration::from_secs(30)),
            read: Some(Duration::from_secs(60)),
            write: Some(Duration::from_secs(60)),
            operation: Some(Duration::from_secs(120)),
            idle: Some(Duration::from_secs(300)),
        }
    }

    /// Set connect timeout
    pub fn with_connect(mut self, timeout: Duration) -> Self {
        self.connect = Some(timeout);
        self
    }

    /// Set read timeout
    pub fn with_read(mut self, timeout: Duration) -> Self {
        self.read = Some(timeout);
        self
    }

    /// Set write timeout
    pub fn with_write(mut self, timeout: Duration) -> Self {
        self.write = Some(timeout);
        self
    }

    /// Set operation timeout
    pub fn with_operation(mut self, timeout: Duration) -> Self {
        self.operation = Some(timeout);
        self
    }

    /// Set idle timeout
    pub fn with_idle(mut self, timeout: Duration) -> Self {
        self.idle = Some(timeout);
        self
    }
}

/// IPC configuration container
#[derive(Debug, Clone)]
pub struct IpcConfig {
    /// Resource name/identifier
    pub name: String,
    /// Access mode
    pub mode: IpcMode,
    /// Permissions
    pub permissions: IpcPermissions,
    /// Timeout configuration
    pub timeout: IpcTimeout,
    /// Buffer sizes
    pub buffer_sizes: HashMap<String, usize>,
    /// Feature flags
    pub features: HashMap<String, bool>,
    /// Custom properties
    pub properties: HashMap<String, String>,
}

impl IpcConfig {
    /// Create new configuration
    pub fn new(name: String) -> Self {
        Self {
            name,
            mode: IpcMode::ReadWrite,
            permissions: IpcPermissions::read_write(),
            timeout: IpcTimeout::default(),
            buffer_sizes: HashMap::new(),
            features: HashMap::new(),
            properties: HashMap::new(),
        }
    }

    /// Set access mode
    pub fn with_mode(mut self, mode: IpcMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set permissions
    pub fn with_permissions(mut self, permissions: IpcPermissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// Set timeout configuration
    pub fn with_timeout(mut self, timeout: IpcTimeout) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set buffer size
    pub fn with_buffer_size(mut self, buffer_type: &str, size: usize) -> Self {
        self.buffer_sizes.insert(buffer_type.to_string(), size);
        self
    }

    /// Enable feature
    pub fn with_feature(mut self, feature: &str, enabled: bool) -> Self {
        self.features.insert(feature.to_string(), enabled);
        self
    }

    /// Set property
    pub fn with_property(mut self, key: &str, value: &str) -> Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }

    /// Get buffer size
    pub fn get_buffer_size(&self, buffer_type: &str) -> usize {
        self.buffer_sizes.get(buffer_type).copied().unwrap_or(8192)
    }

    /// Check if feature is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.features.get(feature).copied().unwrap_or(false)
    }

    /// Get property value
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}

/// Resource limits for IPC operations
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum shared memory size
    pub max_shared_memory_size: Option<usize>,
    /// Maximum number of shared memory regions
    pub max_shared_memory_regions: Option<usize>,
    /// Maximum message queue size
    pub max_message_queue_size: Option<usize>,
    /// Maximum number of message queues
    pub max_message_queues: Option<usize>,
    /// Maximum number of pipes
    pub max_pipes: Option<usize>,
    /// Maximum number of semaphores
    pub max_semaphores: Option<usize>,
    /// Maximum number of open file descriptors
    pub max_file_descriptors: Option<usize>,
    /// Maximum memory usage
    pub max_memory_usage: Option<usize>,
    /// Maximum number of concurrent connections
    pub max_connections: Option<usize>,
}

impl ResourceLimits {
    /// Create unlimited configuration
    pub fn unlimited() -> Self {
        Self {
            max_shared_memory_size: None,
            max_shared_memory_regions: None,
            max_message_queue_size: None,
            max_message_queues: None,
            max_pipes: None,
            max_semaphores: None,
            max_file_descriptors: None,
            max_memory_usage: None,
            max_connections: None,
        }
    }

    /// Create conservative limits
    pub fn conservative() -> Self {
        Self {
            max_shared_memory_size: Some(64 * 1024 * 1024), // 64MB
            max_shared_memory_regions: Some(100),
            max_message_queue_size: Some(1024 * 1024), // 1MB
            max_message_queues: Some(50),
            max_pipes: Some(100),
            max_semaphores: Some(100),
            max_file_descriptors: Some(1024),
            max_memory_usage: Some(128 * 1024 * 1024), // 128MB
            max_connections: Some(100),
        }
    }

    /// Create production limits
    pub fn production() -> Self {
        Self {
            max_shared_memory_size: Some(512 * 1024 * 1024), // 512MB
            max_shared_memory_regions: Some(1000),
            max_message_queue_size: Some(16 * 1024 * 1024), // 16MB
            max_message_queues: Some(500),
            max_pipes: Some(1000),
            max_semaphores: Some(1000),
            max_file_descriptors: Some(65536),
            max_memory_usage: Some(2 * 1024 * 1024 * 1024), // 2GB
            max_connections: Some(10000),
        }
    }

    /// Check if value is within limit
    pub fn check_limit(&self, resource: &str, current: usize) -> bool {
        match resource {
            "shared_memory_size" => {
                self.max_shared_memory_size.map_or(true, |limit| current <= limit)
            }
            "shared_memory_regions" => {
                self.max_shared_memory_regions.map_or(true, |limit| current <= limit)
            }
            "message_queue_size" => {
                self.max_message_queue_size.map_or(true, |limit| current <= limit)
            }
            "message_queues" => {
                self.max_message_queues.map_or(true, |limit| current <= limit)
            }
            "pipes" => {
                self.max_pipes.map_or(true, |limit| current <= limit)
            }
            "semaphores" => {
                self.max_semaphores.map_or(true, |limit| current <= limit)
            }
            "file_descriptors" => {
                self.max_file_descriptors.map_or(true, |limit| current <= limit)
            }
            "memory_usage" => {
                self.max_memory_usage.map_or(true, |limit| current <= limit)
            }
            "connections" => {
                self.max_connections.map_or(true, |limit| current <= limit)
            }
            _ => true,
        }
    }

    /// Get limit value for resource
    pub fn get_limit(&self, resource: &str) -> Option<usize> {
        match resource {
            "shared_memory_size" => self.max_shared_memory_size,
            "shared_memory_regions" => self.max_shared_memory_regions,
            "message_queue_size" => self.max_message_queue_size,
            "message_queues" => self.max_message_queues,
            "pipes" => self.max_pipes,
            "semaphores" => self.max_semaphores,
            "file_descriptors" => self.max_file_descriptors,
            "memory_usage" => self.max_memory_usage,
            "connections" => self.max_connections,
            _ => None,
        }
    }
}

/// IPC statistics container
#[derive(Debug, Clone)]
pub struct IpcStatistics {
    /// Number of active shared memory regions
    pub active_shared_memory_regions: usize,
    /// Number of active pipes
    pub active_pipes: usize,
    /// Number of active message queues
    pub active_message_queues: usize,
    /// Number of active semaphores
    pub active_semaphores: usize,
    /// Number of active sockets
    pub active_sockets: usize,
    /// Number of active RPC connections
    pub active_rpc_connections: usize,
    /// Total memory usage by IPC subsystems
    pub total_memory_usage: usize,
    /// Number of security violations detected
    pub security_violations: u64,
    /// Resource contention statistics
    pub resource_contention_stats: ResourceContentionStats,
    /// Performance metrics
    pub performance_metrics: IpcPerformanceMetrics,
}

/// Resource contention statistics
#[derive(Debug, Clone)]
pub struct ResourceContentionStats {
    /// Number of semaphore waits
    pub semaphore_waits: u64,
    /// Number of pipe blocks
    pub pipe_blocks: u64,
    /// Number of queue full events
    pub queue_full_events: u64,
    /// Number of memory allocation failures
    pub memory_allocation_failures: u64,
    /// Average wait time in nanoseconds
    pub average_wait_time_nanos: u64,
}

/// Performance metrics for IPC operations
#[derive(Debug, Clone)]
pub struct IpcPerformanceMetrics {
    /// Message throughput (messages per second)
    pub message_throughput: f64,
    /// Memory transfer rate (bytes per second)
    pub memory_transfer_rate: f64,
    /// Average pipe operation latency (nanoseconds)
    pub pipe_latency_nanos: u64,
    /// RPC call rate (calls per second)
    pub rpc_call_rate: f64,
    /// Average signal handling time (nanoseconds)
    pub signal_handling_time: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_handle_creation() {
        let handle = IpcHandle::new("test_shm".to_string(), IpcHandleType::SharedMemory);
        assert_eq!(handle.id, "test_shm");
        assert_eq!(handle.handle_type, IpcHandleType::SharedMemory);
        assert_eq!(handle.creator_pid, std::process::id());
    }

    #[test]
    fn test_ipc_permissions() {
        let perms = IpcPermissions::read_write();
        assert!(perms.can_read());
        assert!(perms.can_write());
        assert!(!perms.can_execute());

        let full_perms = IpcPermissions::full();
        assert!(full_perms.can_read());
        assert!(full_perms.can_write());
        assert!(full_perms.can_execute());
        assert!(full_perms.can_delete());
        assert!(full_perms.can_admin());
    }

    #[test]
    fn test_ipc_permissions_octal() {
        let perms = IpcPermissions::from_octal(0o755);
        assert!(perms.can_read());
        assert!(perms.can_write());
        assert!(perms.can_execute());
        assert_eq!(perms.to_octal(), 0o755);
    }

    #[test]
    fn test_ipc_mode() {
        let mode = IpcMode::ReadWrite;
        assert!(mode.can_read());
        assert!(mode.can_write());
        assert!(!mode.creates_new());

        let create_mode = IpcMode::CreateNew;
        assert!(!create_mode.can_read());
        assert!(create_mode.can_write());
        assert!(create_mode.creates_new());
    }

    #[test]
    fn test_ipc_timeout() {
        let timeout = IpcTimeout::uniform(Duration::from_secs(30));
        assert_eq!(timeout.connect, Some(Duration::from_secs(30)));
        assert_eq!(timeout.read, Some(Duration::from_secs(30)));
        assert_eq!(timeout.write, Some(Duration::from_secs(30)));

        let infinite = IpcTimeout::infinite();
        assert_eq!(infinite.connect, None);
        assert_eq!(infinite.read, None);
    }

    #[test]
    fn test_ipc_config() {
        let config = IpcConfig::new("test_resource".to_string())
            .with_mode(IpcMode::ReadOnly)
            .with_buffer_size("input", 4096)
            .with_feature("compression", true)
            .with_property("version", "1.0");

        assert_eq!(config.name, "test_resource");
        assert_eq!(config.mode, IpcMode::ReadOnly);
        assert_eq!(config.get_buffer_size("input"), 4096);
        assert_eq!(config.get_buffer_size("unknown"), 8192); // default
        assert!(config.is_feature_enabled("compression"));
        assert!(!config.is_feature_enabled("encryption")); // default
        assert_eq!(config.get_property("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_resource_limits() {
        let limits = ResourceLimits::conservative();
        assert!(limits.check_limit("shared_memory_size", 32 * 1024 * 1024));
        assert!(!limits.check_limit("shared_memory_size", 128 * 1024 * 1024));
        
        let unlimited = ResourceLimits::unlimited();
        assert!(unlimited.check_limit("shared_memory_size", usize::MAX));
    }

    #[test]
    fn test_handle_metadata() {
        let mut handle = IpcHandle::new("test".to_string(), IpcHandleType::MessageQueue);
        handle.set_metadata("owner".to_string(), "test_process".to_string());
        
        assert_eq!(handle.get_metadata("owner"), Some(&"test_process".to_string()));
        assert_eq!(handle.get_metadata("nonexistent"), None);
    }

    #[test]
    fn test_handle_display() {
        let handle = IpcHandle::new("test_pipe".to_string(), IpcHandleType::NamedPipe);
        let display = format!("{}", handle);
        assert_eq!(display, "NamedPipe:test_pipe");
    }

    #[test]
    fn test_permissions_allows() {
        let perms = IpcPermissions::read_write();
        assert!(perms.allows("read"));
        assert!(perms.allows("write"));
        assert!(!perms.allows("execute"));
        assert!(!perms.allows("admin"));
        assert!(!perms.allows("unknown"));
    }
}
