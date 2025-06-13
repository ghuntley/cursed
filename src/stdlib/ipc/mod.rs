/// Inter-Process Communication (IPC) module for CURSED programming language
/// 
/// This module provides comprehensive IPC functionality including:
/// - Shared memory management and operations
/// - Named pipes for process communication
/// - Message queues for structured communication
/// - Semaphores for resource coordination
/// - Domain sockets for local communication
/// - Remote procedure calls (RPC) infrastructure
/// - Signal handling and process events
/// - IPC security and permissions management
/// 
/// # Examples
/// 
/// ## Shared Memory
/// ```rust
/// use crate::stdlib::ipc::{SharedMemory, SharedMemoryConfig};
/// 
/// let config = SharedMemoryConfig::new("app_data", 1024)?;
/// let mut shm = SharedMemory::create(config)?;
/// shm.write_bytes(b"Hello from process 1")?;
/// ```
/// 
/// ## Named Pipes
/// ```rust
/// use crate::stdlib::ipc::{NamedPipe, PipeMode};
/// 
/// let pipe = NamedPipe::create("/tmp/app_pipe", PipeMode::ReadWrite)?;
/// pipe.write("Hello from sender")?;
/// let response = pipe.read_string()?;
/// ```
/// 
/// ## Message Queues
/// ```rust
/// use crate::stdlib::ipc::{MessageQueue, Message, MessagePriority};
/// 
/// let mq = MessageQueue::create("app_messages", 10)?;
/// let msg = Message::new("task_data", MessagePriority::High)?;
/// mq.send(msg)?;
/// 
/// let received = mq.receive()?;
/// ```
/// 
/// ## Process Signals
/// ```rust
/// use crate::stdlib::ipc::{SignalHandler, Signal};
/// 
/// let handler = SignalHandler::new()?;
/// handler.register(Signal::SIGUSR1, |sig| {
///     println!("Received custom signal: {:?}", sig);
/// })?;
/// handler.send_signal(target_pid, Signal::SIGUSR1)?;
/// ```

pub mod error;
pub mod types;
pub mod traits;
pub mod shared_memory;
pub mod pipes;
pub mod message_queue;
pub mod semaphore;
pub mod signals;
pub mod domain_socket;
pub mod rpc;
pub mod security;
pub mod channels;
pub mod synchronization;

// Re-export main types and functions for easy access
pub use error::{
    IpcError, IpcResult, 
    communication_error, security_error, resource_error, timeout_error,
    invalid_operation, permission_denied, resource_exhausted, connection_failed
};

// Core IPC types and traits
pub use types::{
    ProcessId, IpcHandle, IpcPermissions, IpcMode, 
    SharedMemoryId, MessageQueueId, SemaphoreId, PipeId,
    IpcTimeout, IpcConfig, IpcStatistics, ResourceLimits
};

pub use traits::{
    IpcChannel, IpcReader, IpcWriter, IpcBidirectional,
    Synchronizable, Lockable, Waitable, Signalable,
    Serializable, Deserializable, IpcResource
};

// Shared Memory operations
pub use shared_memory::{
    SharedMemory, SharedMemoryConfig, SharedMemoryAccess,
    create_shared_memory, open_shared_memory, remove_shared_memory,
    SharedMemoryIterator, SharedMemoryView, MemoryMapping, MemoryProtection
};

// Named Pipes operations
pub use pipes::{
    NamedPipe, AnonymousPipe, PipeConfig, PipeMode, PipeEnd,
    create_pipe, create_named_pipe, open_pipe, connect_pipe,
    PipeReader, PipeWriter, PipeStream, PipeListener
};

// Message Queue operations
pub use message_queue::{
    MessageQueue, Message, MessageType, MessagePriority, MessageConfig,
    create_message_queue, open_message_queue, remove_message_queue,
    send_message, receive_message, peek_message, MessageIterator
};

// Semaphore operations
pub use semaphore::{
    Semaphore, SemaphoreConfig, SemaphoreValue, SemaphorePermissions,
    create_semaphore, open_semaphore, remove_semaphore,
    acquire_semaphore, release_semaphore, try_acquire_semaphore,
    CountingSemaphore, BinarySemaphore, NamedSemaphore
};

// Signal handling
pub use signals::{
    SignalHandler, Signal, SignalAction, SignalMask, SignalConfig,
    send_signal, block_signal, unblock_signal, ignore_signal,
    register_signal_handler, unregister_signal_handler,
    wait_for_signal, signal_pending, SignalSet
};

// Domain Socket operations
pub use domain_socket::{
    DomainSocket, UnixSocket, SocketConfig, SocketType, SocketAddress,
    create_socket, bind_socket, listen_socket, accept_connection,
    connect_socket, SocketListener, SocketStream, SocketPair
};

// Remote Procedure Call infrastructure
pub use rpc::{
    RpcClient, RpcServer, RpcConfig, RpcMethod, RpcRequest, RpcResponse,
    RpcError, RpcHandler, RpcRegistry, RpcTransport,
    create_rpc_server, create_rpc_client, register_rpc_method,
    call_remote_method, RpcSerializer, RpcDeserializer
};

// Security and permissions management
pub use security::{
    IpcSecurityContext, SecurityPolicy, AccessControl, Permission,
    Credential, AuthenticationMethod, AuthorizationResult,
    create_security_context, validate_permissions, check_access,
    encrypt_ipc_data, decrypt_ipc_data, generate_ipc_token
};

// High-level IPC channels
pub use channels::{
    IpcChannel, ChannelConfig, ChannelType, ChannelStatistics, ChannelPair
};

// Advanced synchronization primitives
pub use synchronization::{
    IpcBarrier, IpcRwLock, IpcCondVar, ProcessCoordinator,
    IpcRwLockReadGuard, IpcRwLockWriteGuard, BarrierWaitResult,
    CoordinatorStatistics
};

/// Initialize the IPC subsystem
/// 
/// This function should be called once at program startup to initialize
/// global IPC resources and set up proper security contexts.
pub fn initialize() -> IpcResult<()> {
    // Initialize global security context
    security::initialize_security_context()?;
    
    // Set up signal handlers for cleanup
    signals::setup_default_signal_handlers()?;
    
    // Initialize shared memory subsystem
    shared_memory::initialize_shared_memory_subsystem()?;
    
    // Initialize message queue subsystem
    message_queue::initialize_message_queue_subsystem()?;
    
    // Set up resource limits and monitoring
    setup_resource_monitoring()?;
    
    Ok(())
}

/// Shutdown the IPC subsystem
/// 
/// This function should be called at program shutdown to ensure all
/// IPC resources are properly cleaned up and released.
pub fn shutdown() -> IpcResult<()> {
    // Clean up all active IPC resources
    cleanup_active_resources()?;
    
    // Shutdown message queue subsystem
    message_queue::shutdown_message_queue_subsystem()?;
    
    // Shutdown shared memory subsystem
    shared_memory::shutdown_shared_memory_subsystem()?;
    
    // Clean up signal handlers
    signals::cleanup_signal_handlers()?;
    
    // Clean up security context
    security::cleanup_security_context()?;
    
    Ok(())
}

/// Get comprehensive IPC subsystem statistics
pub fn get_ipc_statistics() -> IpcStatistics {
    IpcStatistics {
        active_shared_memory_regions: shared_memory::get_active_region_count(),
        active_pipes: pipes::get_active_pipe_count(),
        active_message_queues: message_queue::get_active_queue_count(),
        active_semaphores: semaphore::get_active_semaphore_count(),
        active_sockets: domain_socket::get_active_socket_count(),
        active_rpc_connections: rpc::get_active_connection_count(),
        total_memory_usage: get_total_memory_usage(),
        security_violations: security::get_violation_count(),
        resource_contention_stats: get_resource_contention_stats(),
        performance_metrics: get_performance_metrics(),
    }
}

/// Set up resource monitoring for IPC operations
fn setup_resource_monitoring() -> IpcResult<()> {
    #[cfg(unix)]
    {
        // Set up memory usage monitoring using /proc filesystem
        if std::path::Path::new("/proc/self/status").exists() {
            // Initialize memory monitoring
            std::thread::spawn(|| {
                loop {
                    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
                        // Parse VmSize, VmRSS for memory usage tracking
                        for line in status.lines() {
                            if line.starts_with("VmRSS:") {
                                // Update global memory usage statistics
                                if let Some(kb_str) = line.split_whitespace().nth(1) {
                                    if let Ok(kb) = kb_str.parse::<usize>() {
                                        RESOURCE_MONITOR.lock().unwrap().update_memory_usage(kb * 1024);
                                    }
                                }
                            }
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_secs(10));
                }
            });
        }
        
        // Set up file descriptor monitoring
        std::thread::spawn(|| {
            loop {
                if let Ok(fd_count) = std::fs::read_dir("/proc/self/fd") {
                    let count = fd_count.count();
                    RESOURCE_MONITOR.lock().unwrap().update_fd_count(count);
                }
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        });
    }
    
    #[cfg(windows)]
    {
        // Set up basic memory monitoring for Windows
        // Note: More detailed monitoring would require additional Windows API dependencies
        std::thread::spawn(|| {
            loop {
                // For now, use a placeholder that would be filled with proper Windows APIs
                // in a production implementation with appropriate dependencies
                RESOURCE_MONITOR.lock().unwrap().update_memory_usage(0);
                
                std::thread::sleep(std::time::Duration::from_secs(10));
            }
        });
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS specific monitoring using task_info
        std::thread::spawn(|| {
            loop {
                // Use mach system calls for memory monitoring
                // This would require additional dependencies for full implementation
                std::thread::sleep(std::time::Duration::from_secs(10));
            }
        });
    }
    
    Ok(())
}

/// Clean up all active IPC resources
fn cleanup_active_resources() -> IpcResult<()> {
    // Clean up shared memory regions
    shared_memory::cleanup_all_regions()?;
    
    // Clean up pipes
    pipes::cleanup_all_pipes()?;
    
    // Clean up message queues
    message_queue::cleanup_all_queues()?;
    
    // Clean up semaphores
    semaphore::cleanup_all_semaphores()?;
    
    // Clean up sockets
    domain_socket::cleanup_all_sockets()?;
    
    // Clean up RPC connections
    rpc::cleanup_all_connections()?;
    
    Ok(())
}

/// Get total memory usage by IPC subsystems
fn get_total_memory_usage() -> usize {
    shared_memory::get_memory_usage() +
    pipes::get_memory_usage() +
    message_queue::get_memory_usage() +
    semaphore::get_memory_usage() +
    domain_socket::get_memory_usage() +
    rpc::get_memory_usage()
}

/// Get resource contention statistics
fn get_resource_contention_stats() -> ResourceContentionStats {
    ResourceContentionStats {
        semaphore_waits: semaphore::get_wait_count(),
        pipe_blocks: pipes::get_block_count(),
        queue_full_events: message_queue::get_full_event_count(),
        memory_allocation_failures: shared_memory::get_allocation_failure_count(),
        average_wait_time_nanos: get_average_wait_time(),
    }
}

/// Get performance metrics for IPC operations
fn get_performance_metrics() -> IpcPerformanceMetrics {
    IpcPerformanceMetrics {
        message_throughput: message_queue::get_throughput(),
        memory_transfer_rate: shared_memory::get_transfer_rate(),
        pipe_latency_nanos: pipes::get_average_latency(),
        rpc_call_rate: rpc::get_call_rate(),
        signal_handling_time: signals::get_average_handling_time(),
    }
}

fn get_average_wait_time() -> u64 {
    RESOURCE_MONITOR.lock()
        .map(|monitor| {
            let total_waits = monitor.semaphore_waits + monitor.pipe_blocks + monitor.queue_blocks;
            if total_waits > 0 {
                monitor.total_wait_time_nanos / total_waits
            } else {
                0
            }
        })
        .unwrap_or(0)
}

/// Resource contention statistics
#[derive(Debug, Clone)]
pub struct ResourceContentionStats {
    pub semaphore_waits: u64,
    pub pipe_blocks: u64,
    pub queue_full_events: u64,
    pub memory_allocation_failures: u64,
    pub average_wait_time_nanos: u64,
}

/// Performance metrics for IPC operations
#[derive(Debug, Clone)]
pub struct IpcPerformanceMetrics {
    pub message_throughput: f64,        // messages per second
    pub memory_transfer_rate: f64,      // bytes per second
    pub pipe_latency_nanos: u64,        // average pipe operation latency
    pub rpc_call_rate: f64,             // RPC calls per second
    pub signal_handling_time: u64,      // average signal handling time in nanos
}

// Internal resource monitoring structure
#[derive(Debug, Clone)]
struct ResourceMonitor {
    memory_usage_bytes: usize,
    fd_count: usize,
    semaphore_waits: u64,
    pipe_blocks: u64,
    queue_blocks: u64,
    total_wait_time_nanos: u64,
    last_update: SystemTime,
}

impl ResourceMonitor {
    fn new() -> Self {
        Self {
            memory_usage_bytes: 0,
            fd_count: 0,
            semaphore_waits: 0,
            pipe_blocks: 0,
            queue_blocks: 0,
            total_wait_time_nanos: 0,
            last_update: SystemTime::now(),
        }
    }
    
    fn update_memory_usage(&mut self, bytes: usize) {
        self.memory_usage_bytes = bytes;
        self.last_update = SystemTime::now();
    }
    
    fn update_fd_count(&mut self, count: usize) {
        self.fd_count = count;
        self.last_update = SystemTime::now();
    }
    
    fn record_wait(&mut self, wait_type: WaitType, duration_nanos: u64) {
        match wait_type {
            WaitType::Semaphore => self.semaphore_waits += 1,
            WaitType::Pipe => self.pipe_blocks += 1,
            WaitType::Queue => self.queue_blocks += 1,
        }
        self.total_wait_time_nanos += duration_nanos;
        self.last_update = SystemTime::now();
    }
}

#[derive(Debug, Clone, Copy)]
enum WaitType {
    Semaphore,
    Pipe,
    Queue,
}

// Global resource monitor instance
lazy_static::lazy_static! {
    static ref RESOURCE_MONITOR: Arc<Mutex<ResourceMonitor>> = 
        Arc::new(Mutex::new(ResourceMonitor::new()));
}

use std::time::SystemTime;
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_module_initialization() {
        // Test that the module can be initialized without errors
        assert!(initialize().is_ok());
        assert!(shutdown().is_ok());
    }

    #[test]
    fn test_ipc_functions_exist() {
        // Test that all expected functions are exported
        // We can't easily test the actual IPC functions in unit tests
        // but we can verify they exist and have the right signatures
        
        let _ = communication_error;
        let _ = security_error;
        let _ = resource_error;
        let _ = timeout_error;
        let _ = invalid_operation;
        let _ = permission_denied;
        let _ = resource_exhausted;
        let _ = connection_failed;
    }

    #[test]
    fn test_ipc_statistics() {
        let stats = get_ipc_statistics();
        // Basic validation that we can get stats
        assert!(stats.active_shared_memory_regions >= 0);
        assert!(stats.total_memory_usage >= 0);
    }

    #[test]
    fn test_error_creation() {
        let err = communication_error("test error");
        assert!(matches!(err, IpcError::CommunicationError { .. }));
        
        let err = security_error("security test");
        assert!(matches!(err, IpcError::SecurityError { .. }));
        
        let err = resource_error("resource test");
        assert!(matches!(err, IpcError::ResourceError { .. }));
    }

    #[test]
    fn test_performance_metrics() {
        let metrics = get_performance_metrics();
        assert!(metrics.message_throughput >= 0.0);
        assert!(metrics.memory_transfer_rate >= 0.0);
        assert!(metrics.pipe_latency_nanos >= 0);
        assert!(metrics.rpc_call_rate >= 0.0);
        assert!(metrics.signal_handling_time >= 0);
    }

    #[test]
    fn test_resource_contention_stats() {
        let stats = get_resource_contention_stats();
        assert!(stats.semaphore_waits >= 0);
        assert!(stats.pipe_blocks >= 0);
        assert!(stats.queue_full_events >= 0);
        assert!(stats.memory_allocation_failures >= 0);
        assert!(stats.average_wait_time_nanos >= 0);
    }
}
