/// Basic Inter-Process Communication (IPC) module for CURSED programming language
/// 
/// This module provides essential IPC functionality:
/// - Named pipes (FIFO) for simple process communication
/// - Unix domain sockets for local communication
/// - Basic shared memory for high-performance data sharing
/// - Simple message passing for structured communication
/// 
/// # Examples
/// 
/// ## Named Pipes
/// ```rust
/// use crate::stdlib::ipc::{NamedPipe, PipeConfig};
/// 
/// let pipe = NamedPipe::create("/tmp/my_pipe")?;
/// pipe.write(b"Hello from process 1")?;
/// let data = pipe.read()?;
/// ```
/// 
/// ## Unix Domain Sockets
/// ```rust
/// use crate::stdlib::ipc::{UnixSocket, SocketType};
/// 
/// let socket = UnixSocket::bind("/tmp/my_socket")?;
/// let client = UnixSocket::connect("/tmp/my_socket")?;
/// client.send(b"Hello server")?;
/// ```
/// 
/// ## Shared Memory
/// ```rust
/// use crate::stdlib::ipc::{SharedMemory, MemoryConfig};
/// 
/// let shm = SharedMemory::create("my_memory", 1024)?;
/// shm.write_at(0, b"Shared data")?;
/// let data = shm.read_at(0, 11)?;
/// ```

pub mod error;
pub mod pipes;
pub mod sockets;
pub mod shared_memory;
pub mod message_queue;

// Re-export main types for easy access
pub use error::{IpcError, IpcResult};

// Named pipes
pub use pipes::{
    NamedPipe, PipeConfig, PipeMode, PipeHandle,
    create_named_pipe, open_named_pipe, remove_named_pipe
};

// Unix domain sockets
pub use sockets::{
    UnixSocket, SocketConfig, SocketType, SocketAddress,
    UnixListener, UnixStream, create_socket_pair
};

// Shared memory
pub use shared_memory::{
    SharedMemory, MemoryConfig, MemoryAccess, MemoryView,
    create_shared_memory, open_shared_memory, remove_shared_memory
};

// Message queue
pub use message_queue::{
    MessageQueue, Message, QueueConfig, MessageType,
    create_message_queue, open_message_queue, send_message, receive_message
};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// IPC subsystem statistics
#[derive(Debug, Clone, Default)]
pub struct IpcStatistics {
    pub active_pipes: usize,
    pub active_sockets: usize,
    pub active_shared_memory: usize,
    pub active_message_queues: usize,
    pub total_memory_usage: usize,
    pub total_operations: u64,
    pub failed_operations: u64,
}

/// IPC resource registry for cleanup
static IPC_REGISTRY: Mutex<Option<IpcRegistry>> = Mutex::new(None);

struct IpcRegistry {
    pipes: HashMap<String, PipeHandle>,
    sockets: HashMap<String, SocketAddress>,
    shared_memory: HashMap<String, String>, // name -> path
    message_queues: HashMap<String, String>, // name -> path
    statistics: IpcStatistics,
}

impl IpcRegistry {
    fn new() -> Self {
        Self {
            pipes: HashMap::new(),
            sockets: HashMap::new(),
            shared_memory: HashMap::new(),
            message_queues: HashMap::new(),
            statistics: IpcStatistics::default(),
        }
    }
    
    fn register_pipe(&mut self, name: String, handle: PipeHandle) {
        self.pipes.insert(name, handle);
        self.statistics.active_pipes += 1;
    }
    
    fn unregister_pipe(&mut self, name: &str) {
        if self.pipes.remove(name).is_some() {
            self.statistics.active_pipes = self.statistics.active_pipes.saturating_sub(1);
        }
    }
    
    fn register_socket(&mut self, name: String, addr: SocketAddress) {
        self.sockets.insert(name, addr);
        self.statistics.active_sockets += 1;
    }
    
    fn unregister_socket(&mut self, name: &str) {
        if self.sockets.remove(name).is_some() {
            self.statistics.active_sockets = self.statistics.active_sockets.saturating_sub(1);
        }
    }
    
    fn register_shared_memory(&mut self, name: String, path: String) {
        self.shared_memory.insert(name, path);
        self.statistics.active_shared_memory += 1;
    }
    
    fn unregister_shared_memory(&mut self, name: &str) {
        if self.shared_memory.remove(name).is_some() {
            self.statistics.active_shared_memory = self.statistics.active_shared_memory.saturating_sub(1);
        }
    }
    
    fn register_message_queue(&mut self, name: String, path: String) {
        self.message_queues.insert(name, path);
        self.statistics.active_message_queues += 1;
    }
    
    fn unregister_message_queue(&mut self, name: &str) {
        if self.message_queues.remove(name).is_some() {
            self.statistics.active_message_queues = self.statistics.active_message_queues.saturating_sub(1);
        }
    }
    
    fn increment_operations(&mut self) {
        self.statistics.total_operations += 1;
    }
    
    fn increment_failed_operations(&mut self) {
        self.statistics.failed_operations += 1;
    }
}

/// Initialize the IPC subsystem
pub fn initialize() -> IpcResult<()> {
    let mut registry = IPC_REGISTRY.lock().map_err(|_| IpcError::Internal("Failed to acquire registry lock".to_string()))?;
    if registry.is_none() {
        *registry = Some(IpcRegistry::new());
    }
    Ok(())
}

/// Shutdown the IPC subsystem and cleanup all resources
pub fn shutdown() -> IpcResult<()> {
    let mut registry = IPC_REGISTRY.lock().map_err(|_| IpcError::Internal("Failed to acquire registry lock".to_string()))?;
    if let Some(reg) = registry.take() {
        // Cleanup all registered resources
        for (name, _) in &reg.pipes {
            let _ = remove_named_pipe(name);
        }
        
        for (name, _) in &reg.shared_memory {
            let _ = remove_shared_memory(name);
        }
        
        // Sockets and message queues cleanup handled by their respective modules
    }
    Ok(())
}

/// Get current IPC statistics
pub fn get_statistics() -> IpcResult<IpcStatistics> {
    let registry = IPC_REGISTRY.lock().map_err(|_| IpcError::Internal("Failed to acquire registry lock".to_string()))?;
    match registry.as_ref() {
        Some(reg) => Ok(reg.statistics.clone()),
        None => Ok(IpcStatistics::default()),
    }
}

// Internal helper functions for registry operations
pub(crate) fn register_pipe(name: String, handle: PipeHandle) -> IpcResult<()> {
    let mut registry = IPC_REGISTRY.lock().map_err(|_| IpcError::Internal("Failed to acquire registry lock".to_string()))?;
    if let Some(reg) = registry.as_mut() {
        reg.register_pipe(name, handle);
    }
    Ok(())
}

pub(crate) fn unregister_pipe(name: &str) -> IpcResult<()> {
    let mut registry = IPC_REGISTRY.lock().map_err(|_| IpcError::Internal("Failed to acquire registry lock".to_string()))?;
    if let Some(reg) = registry.as_mut() {
        reg.unregister_pipe(name);
    }
    Ok(())
}

pub(crate) fn register_socket(name: String, addr: SocketAddress) -> IpcResult<()> {
    let mut registry = IPC_REGISTRY.lock().map_err(|_| IpcError::Internal("Failed to acquire registry lock".to_string()))?;
    if let Some(reg) = registry.as_mut() {
        reg.register_socket(name, addr);
    }
    Ok(())
}

pub(crate) fn unregister_socket(name: &str) -> IpcResult<()> {
    let mut registry = IPC_REGISTRY.lock().map_err(|_| IpcError::Internal("Failed to acquire registry lock".to_string()))?;
    if let Some(reg) = registry.as_mut() {
        reg.unregister_socket(name);
    }
    Ok(())
}

pub(crate) fn register_shared_memory(name: String, path: String) -> IpcResult<()> {
    let mut registry = IPC_REGISTRY.lock().map_err(|_| IpcError::Internal("Failed to acquire registry lock".to_string()))?;
    if let Some(reg) = registry.as_mut() {
        reg.register_shared_memory(name, path);
    }
    Ok(())
}

pub(crate) fn unregister_shared_memory(name: &str) -> IpcResult<()> {
    let mut registry = IPC_REGISTRY.lock().map_err(|_| IpcError::Internal("Failed to acquire registry lock".to_string()))?;
    if let Some(reg) = registry.as_mut() {
        reg.unregister_shared_memory(name);
    }
    Ok(())
}

pub(crate) fn register_message_queue(name: String, path: String) -> IpcResult<()> {
    let mut registry = IPC_REGISTRY.lock().map_err(|_| IpcError::Internal("Failed to acquire registry lock".to_string()))?;
    if let Some(reg) = registry.as_mut() {
        reg.register_message_queue(name, path);
    }
    Ok(())
}

pub(crate) fn unregister_message_queue(name: &str) -> IpcResult<()> {
    let mut registry = IPC_REGISTRY.lock().map_err(|_| IpcError::Internal("Failed to acquire registry lock".to_string()))?;
    if let Some(reg) = registry.as_mut() {
        reg.unregister_message_queue(name);
    }
    Ok(())
}

pub(crate) fn increment_operations() {
    if let Ok(mut registry) = IPC_REGISTRY.lock() {
        if let Some(reg) = registry.as_mut() {
            reg.increment_operations();
        }
    }
}

pub(crate) fn increment_failed_operations() {
    if let Ok(mut registry) = IPC_REGISTRY.lock() {
        if let Some(reg) = registry.as_mut() {
            reg.increment_failed_operations();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_initialization() {
        assert!(initialize().is_ok());
        assert!(shutdown().is_ok());
    }

    #[test]
    fn test_ipc_statistics() {
        let _ = initialize();
        let stats = get_statistics().unwrap();
        assert_eq!(stats.active_pipes, 0);
        assert_eq!(stats.active_sockets, 0);
        assert_eq!(stats.active_shared_memory, 0);
        assert_eq!(stats.active_message_queues, 0);
    }

    #[test]
    fn test_registry_operations() {
        let _ = initialize();
        
        // Test pipe registration
        let handle = PipeHandle::new("/tmp/test".to_string());
        assert!(register_pipe("test_pipe".to_string(), handle).is_ok());
        let stats = get_statistics().unwrap();
        assert_eq!(stats.active_pipes, 1);
        
        assert!(unregister_pipe("test_pipe").is_ok());
        let stats = get_statistics().unwrap();
        assert_eq!(stats.active_pipes, 0);
    }
}
