/// IPC resource traits for CURSED
/// 
/// This module provides common traits for IPC resources

// use crate::stdlib::ipc::error::{IpcError, IpcResult};
use crate::error::CursedError;
use std::time::Duration;

/// Trait for IPC resources that can be created, opened, and closed
pub trait IpcResource {
    /// The type of configuration for this resource
    type Config;
    
    /// Create a new resource with the given configuration
    fn create(name: &str, config: Self::Config) -> IpcResult<Self>
    where
        Self: Sized;
    
    /// Open an existing resource
    fn open(name: &str) -> IpcResult<Self>
    where
        Self: Sized;
    
    /// Close the resource
    fn close(&mut self) -> IpcResult<()>;
    
    /// Get the name of the resource
    fn name(&self) -> &str;
    
    /// Check if the resource is open
    fn is_open(&self) -> bool;
}

/// Trait for IPC resources that support reading
pub trait IpcReadable {
    /// Read data from the resource
    fn read(&mut self, buffer: &mut [u8]) -> IpcResult<usize>;
    
    /// Read data with a timeout
    fn read_timeout(&mut self, buffer: &mut [u8], timeout: Duration) -> IpcResult<usize>;
    
    /// Check if data is available for reading
    fn readable(&self) -> IpcResult<bool>;
}

/// Trait for IPC resources that support writing
pub trait IpcWritable {
    /// Write data to the resource
    fn write(&mut self, data: &[u8]) -> IpcResult<usize>;
    
    /// Write data with a timeout
    fn write_timeout(&mut self, data: &[u8], timeout: Duration) -> IpcResult<usize>;
    
    /// Flush any buffered data
    fn flush(&mut self) -> IpcResult<()>;
    
    /// Check if the resource is ready for writing
    fn writable(&self) -> IpcResult<bool>;
}

/// Trait for IPC resources that support synchronization
pub trait IpcSynchronizable {
    /// Wait for a signal
    fn wait(&mut self) -> IpcResult<()>;
    
    /// Wait for a signal with timeout
    fn wait_timeout(&mut self, timeout: Duration) -> IpcResult<bool>;
    
    /// Send a signal
    fn signal(&mut self) -> IpcResult<()>;
    
    /// Try to acquire without blocking
    fn try_wait(&mut self) -> IpcResult<bool>;
}

/// Trait for IPC resources that support messaging
pub trait IpcMessaging {
    /// The type of message this resource handles
    type Message;
    
    /// Send a message
    fn send(&mut self, message: Self::Message) -> IpcResult<()>;
    
    /// Receive a message
    fn receive(&mut self) -> IpcResult<Self::Message>;
    
    /// Try to receive a message without blocking
    fn try_receive(&mut self) -> IpcResult<Option<Self::Message>>;
    
    /// Receive a message with timeout
    fn receive_timeout(&mut self, timeout: Duration) -> IpcResult<Option<Self::Message>>;
}

/// Trait for IPC resources that support cleanup
pub trait IpcCleanup {
    /// Cleanup the resource from the system
    fn cleanup(&mut self) -> IpcResult<()>;
    
    /// Check if cleanup is needed
    fn needs_cleanup(&self) -> bool;
}

/// Trait for IPC resources that provide statistics
pub trait IpcStats {
    /// Get statistics about the resource
    fn stats(&self) -> IpcResult<IpcResourceStats>;
}

/// Statistics for IPC resources
#[derive(Debug, Clone)]
pub struct IpcResourceStats {
    /// Number of bytes read
    pub bytes_read: u64,
    /// Number of bytes written
    pub bytes_written: u64,
    /// Number of messages sent
    pub messages_sent: u64,
    /// Number of messages received
    pub messages_received: u64,
    /// Number of connections
    pub connections: u64,
    /// Creation time
    pub created_at: std::time::SystemTime,
    /// Last accessed time
    pub last_accessed: std::time::SystemTime,
}

impl Default for IpcResourceStats {
    fn default() -> Self {
        let now = std::time::SystemTime::now();
        Self {
            bytes_read: 0,
            bytes_written: 0,
            messages_sent: 0,
            messages_received: 0,
            connections: 0,
            created_at: now,
            last_accessed: now,
        }
    }
}

