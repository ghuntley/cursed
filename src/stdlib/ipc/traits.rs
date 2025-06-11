/// Core traits and interfaces for IPC operations
use std::time::Duration;
use crate::stdlib::ipc::error::{IpcResult, IpcError};
use crate::stdlib::ipc::types::{IpcHandle, IpcPermissions, IpcMode, ProcessId};

/// Core trait for all IPC communication channels
pub trait IpcChannel {
    /// Get the channel's unique handle
    fn handle(&self) -> &IpcHandle;
    
    /// Check if the channel is currently open and ready for operations
    fn is_open(&self) -> bool;
    
    /// Close the channel and release resources
    fn close(&mut self) -> IpcResult<()>;
    
    /// Get the current permissions for this channel
    fn permissions(&self) -> &IpcPermissions;
    
    /// Set new permissions (if supported and authorized)
    fn set_permissions(&mut self, permissions: IpcPermissions) -> IpcResult<()>;
    
    /// Get the channel's current mode
    fn mode(&self) -> IpcMode;
    
    /// Get statistics about this channel's usage
    fn statistics(&self) -> ChannelStatistics;
}

/// Statistics for individual IPC channels
#[derive(Debug, Clone)]
pub struct ChannelStatistics {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub read_operations: u64,
    pub write_operations: u64,
    pub errors_count: u64,
    pub last_activity: Option<std::time::SystemTime>,
    pub creation_time: std::time::SystemTime,
}

impl ChannelStatistics {
    pub fn new() -> Self {
        Self {
            bytes_read: 0,
            bytes_written: 0,
            read_operations: 0,
            write_operations: 0,
            errors_count: 0,
            last_activity: None,
            creation_time: std::time::SystemTime::now(),
        }
    }

    pub fn record_read(&mut self, bytes: usize) {
        self.bytes_read += bytes as u64;
        self.read_operations += 1;
        self.last_activity = Some(std::time::SystemTime::now());
    }

    pub fn record_write(&mut self, bytes: usize) {
        self.bytes_written += bytes as u64;
        self.write_operations += 1;
        self.last_activity = Some(std::time::SystemTime::now());
    }

    pub fn record_error(&mut self) {
        self.errors_count += 1;
        self.last_activity = Some(std::time::SystemTime::now());
    }

    pub fn total_operations(&self) -> u64 {
        self.read_operations + self.write_operations
    }

    pub fn total_bytes(&self) -> u64 {
        self.bytes_read + self.bytes_written
    }
}

impl Default for ChannelStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for readable IPC channels
pub trait IpcReader {
    /// Read data into the provided buffer
    /// Returns the number of bytes actually read
    fn read(&mut self, buf: &mut [u8]) -> IpcResult<usize>;
    
    /// Read all available data into a vector
    fn read_all(&mut self) -> IpcResult<Vec<u8>>;
    
    /// Read a specific number of bytes
    fn read_exact(&mut self, buf: &mut [u8]) -> IpcResult<()>;
    
    /// Read data with a timeout
    fn read_timeout(&mut self, buf: &mut [u8], timeout: Duration) -> IpcResult<usize>;
    
    /// Check if data is available for reading without blocking
    fn has_data(&self) -> IpcResult<bool>;
    
    /// Get the number of bytes available for reading
    fn available(&self) -> IpcResult<usize>;
}

/// Trait for writable IPC channels
pub trait IpcWriter {
    /// Write data to the channel
    /// Returns the number of bytes actually written
    fn write(&mut self, data: &[u8]) -> IpcResult<usize>;
    
    /// Write all data, retrying if necessary
    fn write_all(&mut self, data: &[u8]) -> IpcResult<()>;
    
    /// Write data with a timeout
    fn write_timeout(&mut self, data: &[u8], timeout: Duration) -> IpcResult<usize>;
    
    /// Flush any buffered data
    fn flush(&mut self) -> IpcResult<()>;
    
    /// Check if the channel is ready for writing without blocking
    fn can_write(&self) -> IpcResult<bool>;
    
    /// Get the amount of buffer space available for writing
    fn write_capacity(&self) -> IpcResult<usize>;
}

/// Trait for bidirectional IPC channels
pub trait IpcBidirectional: IpcReader + IpcWriter {
    /// Perform a synchronous request-response operation
    fn exchange(&mut self, request: &[u8], response: &mut [u8]) -> IpcResult<usize>;
    
    /// Send data and read response with timeout
    fn exchange_timeout(&mut self, request: &[u8], response: &mut [u8], timeout: Duration) -> IpcResult<usize>;
}

/// Trait for synchronizable IPC resources
pub trait Synchronizable {
    /// Wait for the resource to become available
    fn wait(&self) -> IpcResult<()>;
    
    /// Wait with timeout
    fn wait_timeout(&self, timeout: Duration) -> IpcResult<bool>;
    
    /// Try to acquire without blocking
    fn try_wait(&self) -> IpcResult<bool>;
    
    /// Signal or notify waiting processes
    fn signal(&self) -> IpcResult<()>;
    
    /// Broadcast signal to all waiting processes
    fn broadcast(&self) -> IpcResult<()>;
}

/// Trait for lockable IPC resources
pub trait Lockable {
    /// Acquire exclusive lock
    fn lock(&self) -> IpcResult<()>;
    
    /// Try to acquire lock without blocking
    fn try_lock(&self) -> IpcResult<bool>;
    
    /// Acquire lock with timeout
    fn lock_timeout(&self, timeout: Duration) -> IpcResult<bool>;
    
    /// Release the lock
    fn unlock(&self) -> IpcResult<()>;
    
    /// Check if currently locked
    fn is_locked(&self) -> bool;
    
    /// Get the process ID that currently holds the lock
    fn lock_owner(&self) -> Option<ProcessId>;
}

/// Trait for waitable IPC resources
pub trait Waitable {
    /// Wait for condition to be met
    fn wait_for_condition<F>(&self, condition: F) -> IpcResult<()>
    where
        F: Fn() -> bool;
    
    /// Wait for condition with timeout
    fn wait_for_condition_timeout<F>(&self, condition: F, timeout: Duration) -> IpcResult<bool>
    where
        F: Fn() -> bool;
    
    /// Wait for specific event
    fn wait_for_event(&self, event_type: EventType) -> IpcResult<()>;
    
    /// Wait for event with timeout
    fn wait_for_event_timeout(&self, event_type: EventType, timeout: Duration) -> IpcResult<bool>;
}

/// Types of events that can be waited for
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    DataAvailable,
    SpaceAvailable,
    ConnectionEstablished,
    ConnectionClosed,
    ErrorOccurred,
    Custom(String),
}

/// Trait for signalable IPC resources
pub trait Signalable {
    /// Send signal to specific process
    fn send_signal(&self, target: ProcessId, signal: Signal) -> IpcResult<()>;
    
    /// Send signal to group of processes
    fn send_signal_group(&self, targets: &[ProcessId], signal: Signal) -> IpcResult<()>;
    
    /// Register signal handler
    fn register_handler<F>(&mut self, signal: Signal, handler: F) -> IpcResult<()>
    where
        F: Fn(Signal) + Send + 'static;
    
    /// Unregister signal handler
    fn unregister_handler(&mut self, signal: Signal) -> IpcResult<()>;
    
    /// Block specific signal
    fn block_signal(&self, signal: Signal) -> IpcResult<()>;
    
    /// Unblock specific signal
    fn unblock_signal(&self, signal: Signal) -> IpcResult<()>;
}

/// Signal types for IPC communication
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Signal {
    /// User-defined signal 1
    SIGUSR1,
    /// User-defined signal 2
    SIGUSR2,
    /// Termination signal
    SIGTERM,
    /// Interrupt signal
    SIGINT,
    /// Quit signal
    SIGQUIT,
    /// Kill signal (cannot be caught or ignored)
    SIGKILL,
    /// Stop signal
    SIGSTOP,
    /// Continue signal
    SIGCONT,
    /// Custom signal with identifier
    Custom(String),
}

/// Trait for serializable data in IPC operations
pub trait Serializable {
    /// Serialize object to bytes
    fn serialize(&self) -> IpcResult<Vec<u8>>;
    
    /// Get the serialized size without actually serializing
    fn serialized_size(&self) -> usize;
    
    /// Serialize with specific format
    fn serialize_with_format(&self, format: SerializationFormat) -> IpcResult<Vec<u8>>;
}

/// Trait for deserializable data in IPC operations
pub trait Deserializable: Sized {
    /// Deserialize from bytes
    fn deserialize(data: &[u8]) -> IpcResult<Self>;
    
    /// Deserialize with specific format
    fn deserialize_with_format(data: &[u8], format: SerializationFormat) -> IpcResult<Self>;
    
    /// Partial deserialize (for streaming)
    fn partial_deserialize(data: &[u8]) -> IpcResult<(Self, usize)>;
}

/// Serialization formats supported by IPC
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationFormat {
    Binary,
    Json,
    MessagePack,
    Protobuf,
    Custom(String),
}

/// Trait for IPC resources that need cleanup
pub trait IpcResource {
    /// Get resource information
    fn resource_info(&self) -> ResourceInfo;
    
    /// Check if resource is still valid
    fn is_valid(&self) -> bool;
    
    /// Cleanup and release resource
    fn cleanup(&mut self) -> IpcResult<()>;
    
    /// Get resource usage statistics
    fn usage_stats(&self) -> ResourceUsageStats;
    
    /// Set resource limits
    fn set_limits(&mut self, limits: ResourceLimits) -> IpcResult<()>;
}

/// Information about an IPC resource
#[derive(Debug, Clone)]
pub struct ResourceInfo {
    pub resource_type: String,
    pub id: String,
    pub owner: Option<ProcessId>,
    pub created_at: std::time::SystemTime,
    pub size: Option<usize>,
    pub permissions: IpcPermissions,
}

/// Resource usage statistics
#[derive(Debug, Clone, Default)]
pub struct ResourceUsageStats {
    pub memory_usage: usize,
    pub file_descriptors: usize,
    pub active_connections: usize,
    pub total_operations: u64,
    pub error_count: u64,
    pub last_access: Option<std::time::SystemTime>,
}

impl ResourceUsageStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_operation(&mut self) {
        self.total_operations += 1;
        self.last_access = Some(std::time::SystemTime::now());
    }

    pub fn record_error(&mut self) {
        self.error_count += 1;
        self.last_access = Some(std::time::SystemTime::now());
    }

    pub fn error_rate(&self) -> f64 {
        if self.total_operations == 0 {
            0.0
        } else {
            self.error_count as f64 / self.total_operations as f64
        }
    }
}

/// Resource limits for IPC operations
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory: Option<usize>,
    pub max_file_descriptors: Option<usize>,
    pub max_connections: Option<usize>,
    pub max_operations_per_second: Option<u64>,
    pub max_errors_per_minute: Option<u64>,
}

impl ResourceLimits {
    pub fn unlimited() -> Self {
        Self {
            max_memory: None,
            max_file_descriptors: None,
            max_connections: None,
            max_operations_per_second: None,
            max_errors_per_minute: None,
        }
    }

    pub fn default_limits() -> Self {
        Self {
            max_memory: Some(100 * 1024 * 1024), // 100 MB
            max_file_descriptors: Some(1024),
            max_connections: Some(100),
            max_operations_per_second: Some(1000),
            max_errors_per_minute: Some(60),
        }
    }

    pub fn check_memory(&self, current: usize) -> bool {
        self.max_memory.map_or(true, |limit| current <= limit)
    }

    pub fn check_file_descriptors(&self, current: usize) -> bool {
        self.max_file_descriptors.map_or(true, |limit| current <= limit)
    }

    pub fn check_connections(&self, current: usize) -> bool {
        self.max_connections.map_or(true, |limit| current <= limit)
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self::default_limits()
    }
}

/// Trait for connection-oriented IPC channels
pub trait IpcConnection: IpcChannel + IpcBidirectional {
    /// Connect to remote endpoint
    fn connect(&mut self, address: &str) -> IpcResult<()>;
    
    /// Connect with timeout
    fn connect_timeout(&mut self, address: &str, timeout: Duration) -> IpcResult<()>;
    
    /// Disconnect from remote endpoint
    fn disconnect(&mut self) -> IpcResult<()>;
    
    /// Check if connected
    fn is_connected(&self) -> bool;
    
    /// Get remote endpoint address
    fn remote_address(&self) -> Option<String>;
    
    /// Get local endpoint address
    fn local_address(&self) -> Option<String>;
    
    /// Get connection state
    fn connection_state(&self) -> ConnectionState;
}

/// Connection states for IPC connections
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

/// Trait for listening/server IPC channels
pub trait IpcListener {
    /// Start listening for connections
    fn listen(&mut self, address: &str) -> IpcResult<()>;
    
    /// Accept incoming connection
    fn accept(&mut self) -> IpcResult<Box<dyn IpcConnection>>;
    
    /// Accept with timeout
    fn accept_timeout(&mut self, timeout: Duration) -> IpcResult<Option<Box<dyn IpcConnection>>>;
    
    /// Stop listening
    fn stop_listening(&mut self) -> IpcResult<()>;
    
    /// Check if currently listening
    fn is_listening(&self) -> bool;
    
    /// Get the address being listened on
    fn listening_address(&self) -> Option<String>;
    
    /// Get pending connection count
    fn pending_connections(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_statistics() {
        let mut stats = ChannelStatistics::new();
        assert_eq!(stats.total_operations(), 0);
        assert_eq!(stats.total_bytes(), 0);

        stats.record_read(100);
        stats.record_write(50);
        
        assert_eq!(stats.total_operations(), 2);
        assert_eq!(stats.total_bytes(), 150);
        assert_eq!(stats.bytes_read, 100);
        assert_eq!(stats.bytes_written, 50);
    }

    #[test]
    fn test_resource_usage_stats() {
        let mut stats = ResourceUsageStats::new();
        assert_eq!(stats.error_rate(), 0.0);

        stats.record_operation();
        stats.record_operation();
        stats.record_error();

        assert_eq!(stats.total_operations, 2);
        assert_eq!(stats.error_count, 1);
        assert_eq!(stats.error_rate(), 0.5);
    }

    #[test]
    fn test_resource_limits() {
        let limits = ResourceLimits::default_limits();
        assert!(limits.check_memory(1024));
        assert!(!limits.check_memory(200 * 1024 * 1024));

        let unlimited = ResourceLimits::unlimited();
        assert!(unlimited.check_memory(usize::MAX));
        assert!(unlimited.check_connections(usize::MAX));
    }

    #[test]
    fn test_signal_types() {
        assert_eq!(Signal::SIGUSR1, Signal::SIGUSR1);
        assert_ne!(Signal::SIGUSR1, Signal::SIGUSR2);
        
        let custom = Signal::Custom("my_signal".to_string());
        assert!(matches!(custom, Signal::Custom(_)));
    }

    #[test]
    fn test_event_types() {
        assert_eq!(EventType::DataAvailable, EventType::DataAvailable);
        assert_ne!(EventType::DataAvailable, EventType::SpaceAvailable);
        
        let custom = EventType::Custom("my_event".to_string());
        assert!(matches!(custom, EventType::Custom(_)));
    }

    #[test]
    fn test_serialization_formats() {
        assert_eq!(SerializationFormat::Binary, SerializationFormat::Binary);
        assert_ne!(SerializationFormat::Binary, SerializationFormat::Json);
        
        let custom = SerializationFormat::Custom("protobuf".to_string());
        assert!(matches!(custom, SerializationFormat::Custom(_)));
    }

    #[test]
    fn test_connection_state() {
        let state = ConnectionState::Connected;
        assert_eq!(state, ConnectionState::Connected);
        assert_ne!(state, ConnectionState::Disconnected);
        
        let error_state = ConnectionState::Error("timeout".to_string());
        assert!(matches!(error_state, ConnectionState::Error(_)));
    }
}
