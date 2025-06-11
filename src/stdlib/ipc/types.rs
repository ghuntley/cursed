/// Basic types and data structures for IPC operations
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::fmt;

/// Process identifier type
pub type ProcessId = u32;

/// Generic IPC handle identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IpcHandle {
    pub id: String,
    pub handle_type: IpcHandleType,
    pub created_at: SystemTime,
}

impl IpcHandle {
    pub fn new(id: String, handle_type: IpcHandleType) -> Self {
        Self {
            id,
            handle_type,
            created_at: SystemTime::now(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.id.is_empty()
    }

    pub fn age(&self) -> Duration {
        SystemTime::now()
            .duration_since(self.created_at)
            .unwrap_or(Duration::from_secs(0))
    }
}

/// Types of IPC handles
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IpcHandleType {
    SharedMemory,
    MessageQueue,
    Semaphore,
    NamedPipe,
    AnonymousPipe,
    DomainSocket,
    Signal,
    RpcConnection,
}

impl fmt::Display for IpcHandleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcHandleType::SharedMemory => write!(f, "SharedMemory"),
            IpcHandleType::MessageQueue => write!(f, "MessageQueue"),
            IpcHandleType::Semaphore => write!(f, "Semaphore"),
            IpcHandleType::NamedPipe => write!(f, "NamedPipe"),
            IpcHandleType::AnonymousPipe => write!(f, "AnonymousPipe"),
            IpcHandleType::DomainSocket => write!(f, "DomainSocket"),
            IpcHandleType::Signal => write!(f, "Signal"),
            IpcHandleType::RpcConnection => write!(f, "RpcConnection"),
        }
    }
}

/// IPC resource permissions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub owner: Option<String>,
    pub group: Option<String>,
    pub others_read: bool,
    pub others_write: bool,
    pub others_execute: bool,
}

impl IpcPermissions {
    pub fn read_only() -> Self {
        Self {
            read: true,
            write: false,
            execute: false,
            owner: None,
            group: None,
            others_read: false,
            others_write: false,
            others_execute: false,
        }
    }

    pub fn read_write() -> Self {
        Self {
            read: true,
            write: true,
            execute: false,
            owner: None,
            group: None,
            others_read: false,
            others_write: false,
            others_execute: false,
        }
    }

    pub fn full_access() -> Self {
        Self {
            read: true,
            write: true,
            execute: true,
            owner: None,
            group: None,
            others_read: true,
            others_write: true,
            others_execute: true,
        }
    }

    pub fn can_read(&self) -> bool {
        self.read
    }

    pub fn can_write(&self) -> bool {
        self.write
    }

    pub fn can_execute(&self) -> bool {
        self.execute
    }

    pub fn to_octal(&self) -> u32 {
        let mut mode = 0u32;
        
        // Owner permissions
        if self.read { mode |= 0o400; }
        if self.write { mode |= 0o200; }
        if self.execute { mode |= 0o100; }
        
        // Others permissions
        if self.others_read { mode |= 0o004; }
        if self.others_write { mode |= 0o002; }
        if self.others_execute { mode |= 0o001; }
        
        mode
    }
}

/// IPC access mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcMode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    Execute,
    Create,
    CreateExclusive,
}

impl fmt::Display for IpcMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcMode::ReadOnly => write!(f, "ReadOnly"),
            IpcMode::WriteOnly => write!(f, "WriteOnly"),
            IpcMode::ReadWrite => write!(f, "ReadWrite"),
            IpcMode::Execute => write!(f, "Execute"),
            IpcMode::Create => write!(f, "Create"),
            IpcMode::CreateExclusive => write!(f, "CreateExclusive"),
        }
    }
}

/// Shared memory region identifier
pub type SharedMemoryId = String;

/// Message queue identifier
pub type MessageQueueId = String;

/// Semaphore identifier
pub type SemaphoreId = String;

/// Named pipe identifier
pub type PipeId = String;

/// Timeout configuration for IPC operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcTimeout {
    pub connect: Option<Duration>,
    pub read: Option<Duration>,
    pub write: Option<Duration>,
    pub total: Option<Duration>,
}

impl IpcTimeout {
    pub fn new() -> Self {
        Self {
            connect: None,
            read: None,
            write: None,
            total: None,
        }
    }

    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect = Some(timeout);
        self
    }

    pub fn with_read_timeout(mut self, timeout: Duration) -> Self {
        self.read = Some(timeout);
        self
    }

    pub fn with_write_timeout(mut self, timeout: Duration) -> Self {
        self.write = Some(timeout);
        self
    }

    pub fn with_total_timeout(mut self, timeout: Duration) -> Self {
        self.total = Some(timeout);
        self
    }

    pub fn infinite() -> Self {
        Self::new()
    }

    pub fn immediate() -> Self {
        Self {
            connect: Some(Duration::from_secs(0)),
            read: Some(Duration::from_secs(0)),
            write: Some(Duration::from_secs(0)),
            total: Some(Duration::from_secs(0)),
        }
    }
}

impl Default for IpcTimeout {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for IPC operations
#[derive(Debug, Clone)]
pub struct IpcConfig {
    pub permissions: IpcPermissions,
    pub timeout: IpcTimeout,
    pub buffer_size: usize,
    pub max_connections: Option<usize>,
    pub enable_compression: bool,
    pub enable_encryption: bool,
    pub retry_count: u32,
    pub backoff_strategy: BackoffStrategy,
    pub metadata: HashMap<String, String>,
}

impl IpcConfig {
    pub fn new() -> Self {
        Self {
            permissions: IpcPermissions::read_write(),
            timeout: IpcTimeout::default(),
            buffer_size: 8192,
            max_connections: None,
            enable_compression: false,
            enable_encryption: false,
            retry_count: 3,
            backoff_strategy: BackoffStrategy::Exponential,
            metadata: HashMap::new(),
        }
    }

    pub fn with_permissions(mut self, permissions: IpcPermissions) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_timeout(mut self, timeout: IpcTimeout) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn with_encryption(mut self, enabled: bool) -> Self {
        self.enable_encryption = enabled;
        self
    }

    pub fn with_compression(mut self, enabled: bool) -> Self {
        self.enable_compression = enabled;
        self
    }

    pub fn with_retry_count(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }

    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl Default for IpcConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Backoff strategy for retries
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackoffStrategy {
    None,
    Linear,
    Exponential,
    Custom(Vec<Duration>),
}

impl BackoffStrategy {
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        match self {
            BackoffStrategy::None => Duration::from_millis(0),
            BackoffStrategy::Linear => Duration::from_millis(100 * attempt as u64),
            BackoffStrategy::Exponential => {
                let base_ms = 100u64;
                let max_ms = 30000u64; // 30 seconds max
                let delay_ms = base_ms * (2u64.pow(attempt.min(10)));
                Duration::from_millis(delay_ms.min(max_ms))
            }
            BackoffStrategy::Custom(delays) => {
                delays.get(attempt as usize)
                    .copied()
                    .unwrap_or_else(|| delays.last().copied().unwrap_or(Duration::from_secs(1)))
            }
        }
    }
}

/// Resource limits for IPC operations
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory_usage: usize,
    pub max_file_descriptors: usize,
    pub max_connections: usize,
    pub max_message_size: usize,
    pub max_queue_size: usize,
    pub max_shared_memory_regions: usize,
    pub max_semaphores: usize,
    pub max_pipes: usize,
}

impl ResourceLimits {
    pub fn default_limits() -> Self {
        Self {
            max_memory_usage: 1024 * 1024 * 100,  // 100 MB
            max_file_descriptors: 1024,
            max_connections: 100,
            max_message_size: 1024 * 64,           // 64 KB
            max_queue_size: 1000,
            max_shared_memory_regions: 50,
            max_semaphores: 100,
            max_pipes: 50,
        }
    }

    pub fn unlimited() -> Self {
        Self {
            max_memory_usage: usize::MAX,
            max_file_descriptors: usize::MAX,
            max_connections: usize::MAX,
            max_message_size: usize::MAX,
            max_queue_size: usize::MAX,
            max_shared_memory_regions: usize::MAX,
            max_semaphores: usize::MAX,
            max_pipes: usize::MAX,
        }
    }

    pub fn strict_limits() -> Self {
        Self {
            max_memory_usage: 1024 * 1024 * 10,   // 10 MB
            max_file_descriptors: 100,
            max_connections: 10,
            max_message_size: 1024 * 4,           // 4 KB
            max_queue_size: 100,
            max_shared_memory_regions: 10,
            max_semaphores: 20,
            max_pipes: 10,
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self::default_limits()
    }
}

/// Comprehensive IPC statistics
#[derive(Debug, Clone)]
pub struct IpcStatistics {
    pub active_shared_memory_regions: usize,
    pub active_pipes: usize,
    pub active_message_queues: usize,
    pub active_semaphores: usize,
    pub active_sockets: usize,
    pub active_rpc_connections: usize,
    pub total_memory_usage: usize,
    pub security_violations: u64,
    pub resource_contention_stats: crate::stdlib::ipc::ResourceContentionStats,
    pub performance_metrics: crate::stdlib::ipc::IpcPerformanceMetrics,
}

impl IpcStatistics {
    pub fn new() -> Self {
        Self {
            active_shared_memory_regions: 0,
            active_pipes: 0,
            active_message_queues: 0,
            active_semaphores: 0,
            active_sockets: 0,
            active_rpc_connections: 0,
            total_memory_usage: 0,
            security_violations: 0,
            resource_contention_stats: crate::stdlib::ipc::ResourceContentionStats {
                semaphore_waits: 0,
                pipe_blocks: 0,
                queue_full_events: 0,
                memory_allocation_failures: 0,
                average_wait_time_nanos: 0,
            },
            performance_metrics: crate::stdlib::ipc::IpcPerformanceMetrics {
                message_throughput: 0.0,
                memory_transfer_rate: 0.0,
                pipe_latency_nanos: 0,
                rpc_call_rate: 0.0,
                signal_handling_time: 0,
            },
        }
    }

    pub fn total_active_resources(&self) -> usize {
        self.active_shared_memory_regions +
        self.active_pipes +
        self.active_message_queues +
        self.active_semaphores +
        self.active_sockets +
        self.active_rpc_connections
    }

    pub fn is_healthy(&self) -> bool {
        // Define health criteria
        self.total_active_resources() < 1000 &&
        self.total_memory_usage < 100 * 1024 * 1024 && // < 100MB
        self.security_violations < 10
    }
}

impl Default for IpcStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// Data buffer for IPC operations
#[derive(Debug, Clone)]
pub struct IpcBuffer {
    data: Vec<u8>,
    capacity: usize,
    position: usize,
}

impl IpcBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
            position: 0,
        }
    }

    pub fn with_data(data: Vec<u8>) -> Self {
        let capacity = data.len();
        Self {
            data,
            capacity,
            position: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn remaining(&self) -> usize {
        self.capacity.saturating_sub(self.data.len())
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.remaining() < data.len() {
            return Err("Buffer overflow");
        }
        
        self.data.extend_from_slice(data);
        Ok(data.len())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let available = self.data.len() - self.position;
        let to_read = buf.len().min(available);
        
        if to_read > 0 {
            buf[..to_read].copy_from_slice(&self.data[self.position..self.position + to_read]);
            self.position += to_read;
        }
        
        to_read
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.position = 0;
    }

    pub fn reset_position(&mut self) {
        self.position = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_handle_creation() {
        let handle = IpcHandle::new("test_handle".to_string(), IpcHandleType::SharedMemory);
        assert!(handle.is_valid());
        assert_eq!(handle.id, "test_handle");
        assert_eq!(handle.handle_type, IpcHandleType::SharedMemory);
    }

    #[test]
    fn test_ipc_permissions() {
        let perms = IpcPermissions::read_write();
        assert!(perms.can_read());
        assert!(perms.can_write());
        assert!(!perms.can_execute());

        let full_perms = IpcPermissions::full_access();
        assert!(full_perms.can_read());
        assert!(full_perms.can_write());
        assert!(full_perms.can_execute());

        // Test octal conversion
        let octal = full_perms.to_octal();
        assert!(octal > 0);
    }

    #[test]
    fn test_ipc_timeout() {
        let timeout = IpcTimeout::new()
            .with_connect_timeout(Duration::from_secs(5))
            .with_read_timeout(Duration::from_secs(10));
        
        assert_eq!(timeout.connect, Some(Duration::from_secs(5)));
        assert_eq!(timeout.read, Some(Duration::from_secs(10)));
        assert_eq!(timeout.write, None);
    }

    #[test]
    fn test_ipc_config() {
        let config = IpcConfig::new()
            .with_buffer_size(16384)
            .with_encryption(true)
            .with_retry_count(5)
            .add_metadata("version".to_string(), "1.0".to_string());

        assert_eq!(config.buffer_size, 16384);
        assert!(config.enable_encryption);
        assert_eq!(config.retry_count, 5);
        assert_eq!(config.metadata.get("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_backoff_strategy() {
        let exponential = BackoffStrategy::Exponential;
        let delay1 = exponential.calculate_delay(1);
        let delay2 = exponential.calculate_delay(2);
        assert!(delay2 > delay1);

        let linear = BackoffStrategy::Linear;
        let linear_delay = linear.calculate_delay(3);
        assert_eq!(linear_delay, Duration::from_millis(300));

        let custom = BackoffStrategy::Custom(vec![
            Duration::from_millis(100),
            Duration::from_millis(500),
            Duration::from_millis(1000),
        ]);
        assert_eq!(custom.calculate_delay(1), Duration::from_millis(500));
    }

    #[test]
    fn test_resource_limits() {
        let limits = ResourceLimits::default_limits();
        assert!(limits.max_memory_usage > 0);
        assert!(limits.max_connections > 0);

        let strict = ResourceLimits::strict_limits();
        assert!(strict.max_memory_usage < limits.max_memory_usage);
        assert!(strict.max_connections < limits.max_connections);
    }

    #[test]
    fn test_ipc_statistics() {
        let stats = IpcStatistics::new();
        assert_eq!(stats.total_active_resources(), 0);
        assert!(stats.is_healthy());

        let mut unhealthy_stats = stats.clone();
        unhealthy_stats.security_violations = 100;
        assert!(!unhealthy_stats.is_healthy());
    }

    #[test]
    fn test_ipc_buffer() {
        let mut buffer = IpcBuffer::new(1024);
        assert_eq!(buffer.capacity(), 1024);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());

        let data = b"hello world";
        let written = buffer.write(data).unwrap();
        assert_eq!(written, data.len());
        assert_eq!(buffer.len(), data.len());

        let mut read_buf = [0u8; 5];
        let read_count = buffer.read(&mut read_buf);
        assert_eq!(read_count, 5);
        assert_eq!(&read_buf, b"hello");
    }

    #[test]
    fn test_handle_type_display() {
        assert_eq!(IpcHandleType::SharedMemory.to_string(), "SharedMemory");
        assert_eq!(IpcHandleType::MessageQueue.to_string(), "MessageQueue");
        assert_eq!(IpcHandleType::NamedPipe.to_string(), "NamedPipe");
    }

    #[test]
    fn test_ipc_mode_display() {
        assert_eq!(IpcMode::ReadOnly.to_string(), "ReadOnly");
        assert_eq!(IpcMode::ReadWrite.to_string(), "ReadWrite");
        assert_eq!(IpcMode::CreateExclusive.to_string(), "CreateExclusive");
    }
}
