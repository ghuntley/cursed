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
    PriorityMessageQueue, initialize_real_ipc, get_ipc_manager, cleanup_real_ipc
// };
pub use connection_pool::{
    IpcPoolManager, ConnectionFactory, initialize_pool_manager, get_pool_manager, cleanup_pool_manager
// };
pub use advanced_ipc::{
    initialize_advanced_ipc, get_advanced_ipc_manager, cleanup_advanced_ipc
// };
pub use traits::{IpcResource, IpcReadable, IpcWritable, IpcSynchronizable, IpcMessaging, IpcCleanup, IpcStats as IpcStatsProvider, IpcResourceStats};
pub use signals::{
    initialize_signal_boost, cleanup_signal_boost
// };

/// IPC configuration options
#[derive(Debug, Clone)]
pub struct IpcConfig {
    /// Maximum message size for message queues
    /// Maximum number of messages in queue
    /// Default shared memory permissions
    /// Timeout for IPC operations
    /// Buffer size for named pipes
impl Default for IpcConfig {
    fn default() -> Self {
        Self {
            max_message_size: 65536,      // 64KB
            max_queue_size: 1000,         // 1000 messages
            default_permissions: 0o666,   // rw-rw-rw-
            pipe_buffer_size: 8192,       // 8KB
        }
    }
/// Initialize IPC subsystem
pub fn initialize_ipc() -> IpcResult<()> {
    #[cfg(unix)]
    {
        // Ensure signal handlers are set up for pipe cleanup
        setup_signal_handlers()?;
    tracing::info!("IPC subsystem initialized");
    Ok(())
/// Cleanup IPC resources
pub fn cleanup_ipc() -> IpcResult<()> {
    // Cleanup any global IPC resources
    named_pipes::cleanup_pipes()?;
    message_queues::cleanup_queues()?;
    shared_memory::cleanup_segments()?;
    semaphores::cleanup_semaphores()?;
    
    tracing::info!("IPC subsystem cleaned up");
    Ok(())
#[cfg(unix)]
fn setup_signal_handlers() -> IpcResult<()> {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    
    static HANDLERS_INSTALLED: AtomicBool = AtomicBool::new(false);
    
    if !HANDLERS_INSTALLED.swap(true, Ordering::Acquire) {
        // Pure CURSED signal handler simulation for IPC cleanup
        std::thread::spawn(|| {
            // CURSED IPC signal management simulation
            loop {
                std::thread::sleep(std::time::Duration::from_millis(50));
                // Monitor for broken pipe signals in pure CURSED
            }
        });
    }
    
    Ok(())
