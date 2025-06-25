/// Transport layer for IPC communication
/// 
/// This module provides production-ready transport implementations for various
/// IPC mechanisms including Unix domain sockets, named pipes, and TCP sockets.
/// 
/// # Design Philosophy
/// 
/// The transport layer abstracts the underlying communication mechanism while
/// providing:
/// - Connection pooling and resource management  
/// - Bidirectional communication with proper error handling
/// - Thread-safe operations with comprehensive synchronization
/// - Performance monitoring and statistics
/// - Cross-platform compatibility where possible
/// - Integration with existing IPC infrastructure

pub mod unix_socket;
pub mod pool;
pub mod traits;
pub mod rpc_transport;

// Re-export main types
pub use unix_socket::{
    UnixSocketTransport, UnixSocketConfig, UnixSocketPool, 
    ConnectionPool, PoolConfig, PoolStatistics
};
pub use pool::{
    TransportPool, PooledConnection, PoolManager,
    PoolConfiguration, ResourceManager
};
pub use traits::{
    Transport, TransportConnection, TransportListener,
    Serializable, Deserializable, StreamTransport, DatagramTransport
};
pub use rpc_transport::{
    UnixSocketRpcTransport, create_unix_rpc_client, create_unix_rpc_server
};

/// Initialize transport subsystem
// pub fn initialize() -> crate::stdlib::ipc::IpcResult<()> {
    unix_socket::initialize_unix_transport()?;
    pool::initialize_pool_manager()?;
    Ok(())
}

/// Shutdown transport subsystem  
// pub fn shutdown() -> crate::stdlib::ipc::IpcResult<()> {
    pool::shutdown_pool_manager()?;
    unix_socket::cleanup_unix_transport()?;
    Ok(())
}

/// Get transport subsystem statistics
pub fn get_transport_statistics() -> TransportStatistics {
    TransportStatistics {
        active_connections: unix_socket::get_active_connection_count(),
        total_bytes_transferred: unix_socket::get_total_bytes_transferred(),
        pool_statistics: pool::get_pool_statistics(),
        error_count: unix_socket::get_error_count(),
        performance_metrics: unix_socket::get_performance_metrics(),
    }
}

/// Transport subsystem statistics
#[derive(Debug, Clone)]
pub struct TransportStatistics {
    pub active_connections: usize,
    pub total_bytes_transferred: u64,
    pub pool_statistics: pool::PoolStatistics,
    pub error_count: u64,
    pub performance_metrics: unix_socket::PerformanceMetrics,
}
