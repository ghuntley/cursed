/// IPC (Inter-Process Communication) module for CURSED
/// 
/// This module provides comprehensive IPC mechanisms including:
/// - Named pipes (FIFOs)
/// - Message queues
/// - Shared memory
/// - Semaphores
/// - Memory-mapped files
/// - Unix domain sockets

pub mod named_pipes;
pub mod message_queues;
pub mod shared_memory;
pub mod semaphores;
pub mod unix_sockets;
pub mod error;
pub mod traits;
pub mod process_coordination;
pub mod real_ipc;
pub mod connection_pool;
pub mod advanced_ipc;
pub mod signals;

pub use error::{IpcError, IpcResult};
pub use named_pipes::{NamedPipe, NamedPipeServer, NamedPipeClient};
pub use message_queues::{MessageQueue, Message, MessageQueueConfig};
pub use shared_memory::{SharedMemory, SharedMemorySegment, SharedMemoryConfig};
pub use semaphores::{Semaphore, NamedSemaphore, SemaphoreValue};
pub use unix_sockets::{UnixSocket, UnixSocketType, UnixSocketServer, UnixSocketClient};
pub use process_coordination::{IpcProcessRegistry, ProcessAwareIpcManager, ProcessIpcBinding};
pub use real_ipc::{
    RealIpcManager, IpcConnection, IpcMessage as RealIpcMessage, MessagePriority, IpcStats,
    PriorityMessageQueue, initialize_real_ipc, get_ipc_manager, cleanup_real_ipc
};
pub use connection_pool::{
    IpcConnectionPool, IpcConnectionType, PooledConnection, ConnectionPoolConfig,
    IpcPoolManager, ConnectionFactory, initialize_pool_manager, get_pool_manager, cleanup_pool_manager
};
pub use advanced_ipc::{
    AdvancedIpcManager, AdvancedIpcConfig, AdvancedSharedMemory, AdvancedMessageQueue,
    AdvancedNamedPipe, AdvancedUnixSocket, IpcMessage as AdvancedIpcMessage, 
    MessagePriority as AdvancedMessagePriority, MessageType,
    initialize_advanced_ipc, get_advanced_ipc_manager, cleanup_advanced_ipc
};
pub use traits::{IpcResource, IpcReadable, IpcWritable, IpcSynchronizable, IpcMessaging, IpcCleanup, IpcStats as IpcStatsProvider, IpcResourceStats};
pub use signals::{
    SignalBoost, BoostSignal, SignalHandler, GracefulShutdown, SignalMultiplexer,
    SignalAction, VibeChecker, NotifyHandle, ShutdownOptions, ShutdownStatus,
    initialize_signal_boost, cleanup_signal_boost
};

/// IPC configuration options
#[derive(Debug, Clone)]
pub struct IpcConfig {
    /// Maximum message size for message queues
    pub max_message_size: usize,
    /// Maximum number of messages in queue
    pub max_queue_size: usize,
    /// Default shared memory permissions
    pub default_permissions: u32,
    /// Timeout for IPC operations
    pub default_timeout: std::time::Duration,
    /// Buffer size for named pipes
    pub pipe_buffer_size: usize,
}

impl Default for IpcConfig {
    fn default() -> Self {
        Self {
            max_message_size: 65536,      // 64KB
            max_queue_size: 1000,         // 1000 messages
            default_permissions: 0o666,   // rw-rw-rw-
            default_timeout: std::time::Duration::from_secs(30),
            pipe_buffer_size: 8192,       // 8KB
        }
    }
}

/// Initialize IPC subsystem
pub fn initialize_ipc() -> IpcResult<()> {
    #[cfg(unix)]
    {
        // Ensure signal handlers are set up for pipe cleanup
        setup_signal_handlers()?;
    }
    
    tracing::info!("IPC subsystem initialized");
    Ok(())
}

/// Cleanup IPC resources
pub fn cleanup_ipc() -> IpcResult<()> {
    // Cleanup any global IPC resources
    named_pipes::cleanup_pipes()?;
    message_queues::cleanup_queues()?;
    shared_memory::cleanup_segments()?;
    semaphores::cleanup_semaphores()?;
    
    tracing::info!("IPC subsystem cleaned up");
    Ok(())
}

#[cfg(unix)]
fn setup_signal_handlers() -> IpcResult<()> {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    
    static HANDLERS_INSTALLED: AtomicBool = AtomicBool::new(false);
    
    if !HANDLERS_INSTALLED.swap(true, Ordering::Acquire) {
        // Install signal handlers for cleanup
        unsafe {
            libc::signal(libc::SIGPIPE, libc::SIG_IGN);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::stdlib::process::real_ipc::IpcMessage;

    #[test]
    fn test_ipc_config_default() {
        let config = IpcConfig::default();
        assert_eq!(config.max_message_size, 65536);
        assert_eq!(config.max_queue_size, 1000);
        assert_eq!(config.default_permissions, 0o666);
    }

    #[test]
    fn test_ipc_initialization() {
        assert!(initialize_ipc().is_ok());
        assert!(cleanup_ipc().is_ok());
    }
}
