use crate::error::Error;
/// Transport traits for IPC communication
/// 
/// This module defines the core traits that all transport implementations
/// must implement to provide consistent, thread-safe, and efficient
/// inter-process communication.

use std::io::{Read, Write};
use std::time::Duration;
use crate::stdlib::ipc::{IpcResult, IpcError};

/// Core transport trait for bidirectional communication
pub trait Transport: Send + Sync + std::fmt::Debug {
    type Connection: TransportConnection;
    type Listener: TransportListener<Connection = Self::Connection>;
    
    /// Create a new connection to the specified address
    fn connect(&self, address: &str) -> IpcResult<Self::Connection>;
    
    /// Create a listener bound to the specified address
    fn bind(&self, address: &str) -> IpcResult<Self::Listener>;
    
    /// Check if the transport is available on this platform
    fn is_available() -> bool;
    
    /// Get transport name for debugging and logging
    fn name(&self) -> &'static str;
}

/// Transport connection trait for active connections
pub trait TransportConnection: Send + Sync + std::fmt::Debug {
    /// Read data from the connection
    fn read(&mut self, buffer: &mut [u8]) -> IpcResult<usize>;
    
    /// Write data to the connection
    fn write(&mut self, data: &[u8]) -> IpcResult<usize>;
    
    /// Flush pending writes
    fn flush(&mut self) -> IpcResult<()>;
    
    /// Close the connection
    fn close(&mut self) -> IpcResult<()>;
    
    /// Check if the connection is still active
    fn is_active(&self) -> bool;
    
    /// Get the remote address if available
    fn remote_address(&self) -> Option<String>;
    
    /// Set read timeout
    fn set_read_timeout(&mut self, timeout: Option<Duration>) -> IpcResult<()>;
    
    /// Set write timeout
    fn set_write_timeout(&mut self, timeout: Option<Duration>) -> IpcResult<()>;
    
    /// Clone the connection for use in multiple threads
    fn try_clone(&self) -> IpcResult<Box<dyn TransportConnection>>;
}

/// Transport listener trait for accepting connections
pub trait TransportListener: Send + Sync + std::fmt::Debug {
    type Connection: TransportConnection;
    
    /// Accept a new connection
    fn accept(&mut self) -> IpcResult<Self::Connection>;
    
    /// Set the listener to non-blocking mode
    fn set_nonblocking(&mut self, nonblocking: bool) -> IpcResult<()>;
    
    /// Get the local address the listener is bound to
    fn local_address(&self) -> Option<String>;
    
    /// Close the listener
    fn close(&mut self) -> IpcResult<()>;
}

/// Stream-based transport for reliable, ordered communication
pub trait StreamTransport: Transport {
    /// Maximum message size for this transport
    fn max_message_size(&self) -> usize;
    
    /// Whether the transport preserves message boundaries
    fn preserves_message_boundaries(&self) -> bool {
        false // Stream transports typically don't preserve boundaries
    }
}

/// Datagram-based transport for unreliable, unordered communication
pub trait DatagramTransport: Transport {
    /// Send a datagram to the specified address
    fn send_to(&self, data: &[u8], address: &str) -> IpcResult<usize>;
    
    /// Receive a datagram and return the data and sender address
    fn recv_from(&self, buffer: &mut [u8]) -> IpcResult<(usize, String)>;
    
    /// Maximum datagram size for this transport
    fn max_datagram_size(&self) -> usize;
}

/// Serializable data for transport
pub trait Serializable {
    /// Serialize the data to bytes
    fn serialize(&self) -> IpcResult<Vec<u8>>;
}

/// Deserializable data from transport
pub trait Deserializable: Sized {
    /// Deserialize the data from bytes
    fn deserialize(data: &[u8]) -> IpcResult<Self>;
}

/// High-level message transport combining serialization with transport
pub trait MessageTransport<T>: Transport 
where 
    T: Serializable + Deserializable + Send + Sync,
{
    /// Send a serialized message
    fn send_message(&self, address: &str, message: &T) -> IpcResult<()> {
        let data = message.serialize()?;
        let mut connection = self.connect(address)?;
        connection.write(&data)?;
        connection.flush()?;
        Ok(())
    }
    
    /// Receive and deserialize a message
    fn receive_message(&self, listener: &mut Self::Listener) -> IpcResult<T> {
        let mut connection = listener.accept()?;
        let mut buffer = vec![0u8; 65536]; // 64KB buffer
        let bytes_read = connection.read(&mut buffer)?;
        buffer.truncate(bytes_read);
        T::deserialize(&buffer)
    }
}

/// Implement MessageTransport for any Transport
impl<Trans, T> MessageTransport<T> for Trans 
where 
    Trans: Transport,
    T: Serializable + Deserializable + Send + Sync,
{
}

/// Transport configuration trait
pub trait TransportConfig: Clone + std::fmt::Debug {
    /// Validate the configuration
    fn validate(&self) -> IpcResult<()>;
    
    /// Get default configuration
    fn default() -> Self;
    
    /// Get timeout settings
    fn timeout(&self) -> Duration;
    
    /// Get buffer size settings
    fn buffer_size(&self) -> usize;
}

/// Transport statistics trait
pub trait TransportStatistics: Clone + std::fmt::Debug {
    /// Get the number of bytes sent
    fn bytes_sent(&self) -> u64;
    
    /// Get the number of bytes received
    fn bytes_received(&self) -> u64;
    
    /// Get the number of messages sent
    fn messages_sent(&self) -> u64;
    
    /// Get the number of messages received
    fn messages_received(&self) -> u64;
    
    /// Get the number of connection errors
    fn connection_errors(&self) -> u64;
    
    /// Get the average latency
    fn average_latency(&self) -> Duration;
    
    /// Reset statistics
    fn reset(&mut self);
}

/// Transport pool trait for connection management
pub trait TransportPool<T: TransportConnection>: Send + Sync + std::fmt::Debug {
    /// Get a connection from the pool
    fn get_connection(&self, address: &str) -> IpcResult<T>;
    
    /// Return a connection to the pool
    fn return_connection(&self, connection: T) -> IpcResult<()>;
    
    /// Get the current pool size
    fn pool_size(&self) -> usize;
    
    /// Get the number of active connections
    fn active_connections(&self) -> usize;
    
    /// Close all connections in the pool
    fn close_all(&mut self) -> IpcResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_definitions() {
        // This test ensures that our traits compile and have the expected signatures
        // We can't instantiate them directly, but we can verify the trait definitions
        
        fn assert_transport<T: Transport>() {}
        fn assert_connection<T: TransportConnection>() {}
        fn assert_listener<T: TransportListener>() {}
        fn assert_stream<T: StreamTransport>() {}
        fn assert_datagram<T: DatagramTransport>() {}
        
        // These functions will only compile if the traits are properly defined
    }
    
    #[test]
    fn test_serializable_implementable() {
        // Test that we can implement Serializable
        #[derive(Debug)]
        struct TestMessage {
            data: String,
        }
        
        impl Serializable for TestMessage {
            fn serialize(&self) -> IpcResult<Vec<u8>> {
                Ok(self.data.as_bytes().to_vec())
            }
        }
        
        impl Deserializable for TestMessage {
            fn deserialize(data: &[u8]) -> IpcResult<Self> {
                let data_str = String::from_utf8(data.to_vec())
                    .map_err(|e| IpcError::SerializationError { 
                        message: format!("UTF-8 error: {}", e) 
                    })?;
                Ok(Self { data: data_str })
            }
        }
        
        let message = TestMessage {
            data: "test".to_string(),
        };
        
        let serialized = message.serialize().unwrap();
        let deserialized = TestMessage::deserialize(&serialized).unwrap();
        assert_eq!(message.data, deserialized.data);
    }
}
