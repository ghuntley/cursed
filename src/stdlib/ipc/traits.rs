/// IPC traits and interfaces for CURSED
/// 
/// This module defines the common traits and interfaces used throughout the IPC system,
/// providing a unified API for different communication mechanisms.

use std::io::{Read, Write};
use std::time::Duration;
use crate::stdlib::ipc::{IpcResult, IpcError};

/// Core trait for IPC channels
pub trait IpcChannel {
    /// Get the channel identifier
    fn id(&self) -> &str;
    
    /// Check if the channel is open/connected
    fn is_open(&self) -> bool;
    
    /// Close the channel
    fn close(&mut self) -> IpcResult<()>;
    
    /// Get channel statistics
    fn get_statistics(&self) -> IpcResult<ChannelStatistics>;
    
    /// Set timeout for operations
    fn set_timeout(&mut self, operation: &str, timeout: Duration) -> IpcResult<()>;
    
    /// Get timeout for operations
    fn get_timeout(&self, operation: &str) -> Option<Duration>;
    
    /// Check if channel supports operation
    fn supports_operation(&self, operation: &str) -> bool;
}

/// Trait for reading from IPC channels
pub trait IpcReader: IpcChannel {
    /// Read data into buffer, returns number of bytes read
    fn read_data(&mut self, buffer: &mut [u8]) -> IpcResult<usize>;
    
    /// Read exact amount of data
    fn read_exact(&mut self, buffer: &mut [u8]) -> IpcResult<()> {
        let mut total_read = 0;
        while total_read < buffer.len() {
            let bytes_read = self.read_data(&mut buffer[total_read..])?;
            if bytes_read == 0 {
                return Err(IpcError::CommunicationError {
                    operation: "read_exact".to_string(),
                    error_type: "unexpected_eof".to_string(),
                    message: "Unexpected end of stream".to_string(),
                    resource_id: Some(self.id().to_string()),
                });
            }
            total_read += bytes_read;
        }
        Ok(())
    }
    
    /// Read data with timeout
    fn read_timeout(&mut self, buffer: &mut [u8], timeout: Duration) -> IpcResult<usize>;
    
    /// Read until delimiter or buffer is full
    fn read_until(&mut self, delimiter: u8, buffer: &mut Vec<u8>) -> IpcResult<usize>;
    
    /// Read all available data
    fn read_all(&mut self, buffer: &mut Vec<u8>) -> IpcResult<usize>;
    
    /// Peek at data without consuming it
    fn peek(&self, buffer: &mut [u8]) -> IpcResult<usize>;
    
    /// Check if data is available for reading
    fn has_data_available(&self) -> IpcResult<bool>;
    
    /// Get number of bytes available for reading
    fn bytes_available(&self) -> IpcResult<usize>;
}

/// Trait for writing to IPC channels
pub trait IpcWriter: IpcChannel {
    /// Write data to channel, returns number of bytes written
    fn write_data(&mut self, data: &[u8]) -> IpcResult<usize>;
    
    /// Write all data to channel
    fn write_all(&mut self, data: &[u8]) -> IpcResult<()> {
        let mut total_written = 0;
        while total_written < data.len() {
            let bytes_written = self.write_data(&data[total_written..])?;
            if bytes_written == 0 {
                return Err(IpcError::CommunicationError {
                    operation: "write_all".to_string(),
                    error_type: "write_failed".to_string(),
                    message: "Unable to write data".to_string(),
                    resource_id: Some(self.id().to_string()),
                });
            }
            total_written += bytes_written;
        }
        Ok(())
    }
    
    /// Write data with timeout
    fn write_timeout(&mut self, data: &[u8], timeout: Duration) -> IpcResult<usize>;
    
    /// Flush any buffered data
    fn flush(&mut self) -> IpcResult<()>;
    
    /// Check if channel can accept more data
    fn can_write(&self) -> IpcResult<bool>;
    
    /// Get available write buffer space
    fn write_buffer_space(&self) -> IpcResult<usize>;
    
    /// Write formatted data
    fn write_formatted(&mut self, format: &str, args: &[&dyn ToString]) -> IpcResult<usize> {
        // Simple implementation - could be enhanced with proper formatting
        let mut formatted = format.to_string();
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            formatted = formatted.replace(&placeholder, &arg.to_string());
        }
        self.write_all(formatted.as_bytes())?;
        Ok(formatted.len())
    }
}

/// Trait for bidirectional IPC channels
pub trait IpcBidirectional: IpcReader + IpcWriter {
    /// Exchange data (write then read)
    fn exchange(&mut self, request: &[u8], response: &mut [u8]) -> IpcResult<usize> {
        self.write_all(request)?;
        self.flush()?;
        self.read_data(response)
    }
    
    /// Exchange data with timeout
    fn exchange_timeout(&mut self, request: &[u8], response: &mut [u8], timeout: Duration) -> IpcResult<usize> {
        self.write_timeout(request, timeout)?;
        self.flush()?;
        self.read_timeout(response, timeout)
    }
    
    /// Shutdown one direction of communication
    fn shutdown_direction(&mut self, direction: ShutdownDirection) -> IpcResult<()>;
}

/// Direction for shutdown operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownDirection {
    Read,
    Write,
    Both,
}

/// Trait for synchronizable IPC resources
pub trait Synchronizable {
    /// Wait for resource to become available
    fn wait(&self) -> IpcResult<()>;
    
    /// Wait with timeout
    fn wait_timeout(&self, timeout: Duration) -> IpcResult<bool>;
    
    /// Try to acquire resource without blocking
    fn try_wait(&self) -> IpcResult<bool>;
    
    /// Signal the resource
    fn signal(&self) -> IpcResult<()>;
    
    /// Signal all waiting processes
    fn signal_all(&self) -> IpcResult<()>;
    
    /// Get number of waiting processes
    fn waiting_count(&self) -> IpcResult<usize>;
}

/// Trait for lockable IPC resources
pub trait Lockable {
    /// Lock the resource
    fn lock(&self) -> IpcResult<LockGuard>;
    
    /// Try to lock without blocking
    fn try_lock(&self) -> IpcResult<Option<LockGuard>>;
    
    /// Lock with timeout
    fn lock_timeout(&self, timeout: Duration) -> IpcResult<Option<LockGuard>>;
    
    /// Check if resource is currently locked
    fn is_locked(&self) -> IpcResult<bool>;
    
    /// Get lock holder information
    fn lock_info(&self) -> IpcResult<LockInfo>;
}

/// Lock guard for automatic unlocking
pub struct LockGuard {
    resource_id: String,
    lock_type: LockType,
}

impl LockGuard {
    pub fn new(resource_id: String, lock_type: LockType) -> Self {
        Self { resource_id, lock_type }
    }
    
    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }
    
    pub fn lock_type(&self) -> LockType {
        self.lock_type
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        // Automatically unlock when guard is dropped
        // Implementation would need access to the actual lock mechanism
    }
}

/// Types of locks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockType {
    Exclusive,
    Shared,
    Upgradeable,
}

/// Lock information
#[derive(Debug, Clone)]
pub struct LockInfo {
    pub lock_type: LockType,
    pub holder_pid: Option<u32>,
    pub acquired_at: std::time::SystemTime,
    pub lock_count: u32,
}

/// Trait for waitable IPC resources
pub trait Waitable {
    /// Wait for specific condition
    fn wait_for_condition(&self, condition: &str) -> IpcResult<()>;
    
    /// Wait for condition with timeout
    fn wait_for_condition_timeout(&self, condition: &str, timeout: Duration) -> IpcResult<bool>;
    
    /// Check if condition is currently met
    fn check_condition(&self, condition: &str) -> IpcResult<bool>;
    
    /// List available conditions
    fn available_conditions(&self) -> Vec<String>;
}

/// Trait for signalable IPC resources
pub trait Signalable {
    /// Send signal to resource
    fn send_signal(&self, signal: &str) -> IpcResult<()>;
    
    /// Send signal with data
    fn send_signal_with_data(&self, signal: &str, data: &[u8]) -> IpcResult<()>;
    
    /// Register signal handler
    fn register_signal_handler(&mut self, signal: &str, handler: Box<dyn SignalHandler>) -> IpcResult<()>;
    
    /// Unregister signal handler
    fn unregister_signal_handler(&mut self, signal: &str) -> IpcResult<()>;
    
    /// List supported signals
    fn supported_signals(&self) -> Vec<String>;
}

/// Signal handler trait
pub trait SignalHandler: Send + Sync {
    /// Handle received signal
    fn handle_signal(&self, signal: &str, data: &[u8]) -> IpcResult<()>;
}

/// Trait for serializable data
pub trait Serializable {
    /// Serialize object to bytes
    fn serialize(&self) -> IpcResult<Vec<u8>>;
    
    /// Get serialized size estimate
    fn serialized_size(&self) -> usize;
    
    /// Serialize to writer
    fn serialize_to<W: Write>(&self, writer: &mut W) -> IpcResult<()> {
        let data = self.serialize()?;
        writer.write_all(&data)
            .map_err(|e| IpcError::SerializationError {
                operation: "serialize_to".to_string(),
                data_type: std::any::type_name::<Self>().to_string(),
                message: e.to_string(),
                position: None,
            })?;
        Ok(())
    }
}

/// Trait for deserializable data
pub trait Deserializable: Sized {
    /// Deserialize object from bytes
    fn deserialize(data: &[u8]) -> IpcResult<Self>;
    
    /// Deserialize from reader
    fn deserialize_from<R: Read>(reader: &mut R) -> IpcResult<Self> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)
            .map_err(|e| IpcError::SerializationError {
                operation: "deserialize_from".to_string(),
                data_type: std::any::type_name::<Self>().to_string(),
                message: e.to_string(),
                position: None,
            })?;
        Self::deserialize(&buffer)
    }
    
    /// Try to deserialize with partial data
    fn try_deserialize_partial(data: &[u8]) -> IpcResult<(Self, usize)>;
}

/// Trait for IPC resources with lifecycle management
pub trait IpcResource {
    /// Initialize the resource
    fn initialize(&mut self) -> IpcResult<()>;
    
    /// Cleanup the resource
    fn cleanup(&mut self) -> IpcResult<()>;
    
    /// Check if resource is valid/healthy
    fn is_healthy(&self) -> IpcResult<bool>;
    
    /// Get resource information
    fn get_info(&self) -> IpcResult<ResourceInfo>;
    
    /// Reset resource to initial state
    fn reset(&mut self) -> IpcResult<()>;
    
    /// Get resource dependencies
    fn dependencies(&self) -> Vec<String>;
}

/// Resource information
#[derive(Debug, Clone)]
pub struct ResourceInfo {
    pub id: String,
    pub resource_type: String,
    pub status: ResourceStatus,
    pub created_at: std::time::SystemTime,
    pub last_accessed: Option<std::time::SystemTime>,
    pub access_count: u64,
    pub size: Option<usize>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Resource status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceStatus {
    Initializing,
    Ready,
    Active,
    Idle,
    Error,
    Cleanup,
    Destroyed,
}

/// Channel statistics
#[derive(Debug, Clone)]
pub struct ChannelStatistics {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub read_operations: u64,
    pub write_operations: u64,
    pub errors: u64,
    pub last_activity: Option<std::time::SystemTime>,
    pub average_latency: Duration,
    pub peak_throughput: f64,
}

impl ChannelStatistics {
    pub fn new() -> Self {
        Self {
            bytes_read: 0,
            bytes_written: 0,
            read_operations: 0,
            write_operations: 0,
            errors: 0,
            last_activity: None,
            average_latency: Duration::from_nanos(0),
            peak_throughput: 0.0,
        }
    }
    
    pub fn record_read(&mut self, bytes: usize, latency: Duration) {
        self.bytes_read += bytes as u64;
        self.read_operations += 1;
        self.last_activity = Some(std::time::SystemTime::now());
        self.update_latency(latency);
    }
    
    pub fn record_write(&mut self, bytes: usize, latency: Duration) {
        self.bytes_written += bytes as u64;
        self.write_operations += 1;
        self.last_activity = Some(std::time::SystemTime::now());
        self.update_latency(latency);
    }
    
    pub fn record_error(&mut self) {
        self.errors += 1;
        self.last_activity = Some(std::time::SystemTime::now());
    }
    
    fn update_latency(&mut self, latency: Duration) {
        let total_ops = self.read_operations + self.write_operations;
        if total_ops > 1 {
            let current_avg = self.average_latency.as_nanos() as u64;
            let new_latency = latency.as_nanos() as u64;
            let updated_avg = (current_avg * (total_ops - 1) + new_latency) / total_ops;
            self.average_latency = Duration::from_nanos(updated_avg);
        } else {
            self.average_latency = latency;
        }
    }
}

/// Utility trait for converting between different data types in IPC
pub trait IpcConvert<T> {
    /// Convert to IPC-compatible type
    fn to_ipc(&self) -> IpcResult<T>;
    
    /// Convert from IPC-compatible type
    fn from_ipc(value: T) -> IpcResult<Self>
    where
        Self: Sized;
}

/// Trait for IPC message formatting
pub trait MessageFormat {
    /// Get message format identifier
    fn format_id(&self) -> &str;
    
    /// Encode message with this format
    fn encode(&self, data: &[u8]) -> IpcResult<Vec<u8>>;
    
    /// Decode message with this format
    fn decode(&self, data: &[u8]) -> IpcResult<Vec<u8>>;
    
    /// Get format metadata
    fn metadata(&self) -> std::collections::HashMap<String, String>;
}

/// Trait for connection pooling
pub trait ConnectionPool<T> {
    /// Get connection from pool
    fn get_connection(&self) -> IpcResult<T>;
    
    /// Return connection to pool
    fn return_connection(&self, connection: T) -> IpcResult<()>;
    
    /// Get pool statistics
    fn pool_stats(&self) -> PoolStatistics;
    
    /// Resize pool
    fn resize(&self, new_size: usize) -> IpcResult<()>;
    
    /// Close all connections
    fn close_all(&self) -> IpcResult<()>;
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStatistics {
    pub total_connections: usize,
    pub active_connections: usize,
    pub idle_connections: usize,
    pub failed_connections: usize,
    pub pool_hits: u64,
    pub pool_misses: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_statistics() {
        let mut stats = ChannelStatistics::new();
        assert_eq!(stats.bytes_read, 0);
        assert_eq!(stats.read_operations, 0);
        
        stats.record_read(1024, Duration::from_millis(10));
        assert_eq!(stats.bytes_read, 1024);
        assert_eq!(stats.read_operations, 1);
        assert!(stats.last_activity.is_some());
        
        stats.record_write(512, Duration::from_millis(5));
        assert_eq!(stats.bytes_written, 512);
        assert_eq!(stats.write_operations, 1);
    }

    #[test]
    fn test_lock_guard() {
        let guard = LockGuard::new("test_resource".to_string(), LockType::Exclusive);
        assert_eq!(guard.resource_id(), "test_resource");
        assert_eq!(guard.lock_type(), LockType::Exclusive);
    }

    #[test]
    fn test_resource_status() {
        let status = ResourceStatus::Ready;
        assert_eq!(status, ResourceStatus::Ready);
        assert_ne!(status, ResourceStatus::Error);
    }

    #[test]
    fn test_shutdown_direction() {
        let direction = ShutdownDirection::Both;
        assert_eq!(direction, ShutdownDirection::Both);
        assert_ne!(direction, ShutdownDirection::Read);
    }

    #[test]
    fn test_lock_info() {
        let info = LockInfo {
            lock_type: LockType::Shared,
            holder_pid: Some(1234),
            acquired_at: std::time::SystemTime::now(),
            lock_count: 1,
        };
        
        assert_eq!(info.lock_type, LockType::Shared);
        assert_eq!(info.holder_pid, Some(1234));
        assert_eq!(info.lock_count, 1);
    }

    #[test]
    fn test_pool_statistics() {
        let stats = PoolStatistics {
            total_connections: 10,
            active_connections: 5,
            idle_connections: 5,
            failed_connections: 0,
            pool_hits: 100,
            pool_misses: 10,
        };
        
        assert_eq!(stats.total_connections, 10);
        assert_eq!(stats.active_connections, 5);
        assert_eq!(stats.pool_hits, 100);
    }
}
