/// Process communication and IPC integration for CURSED
/// 
/// This module provides high-level process communication functionality,
/// integrating with the IPC system for inter-process data exchange.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, communication_error, timeout_error, invalid_arguments
};
use crate::stdlib::process::core::{Process, ProcessConfig};

/// Process communication channels
#[derive(Debug, Clone)]
pub struct ProcessChannels {
    /// Named pipes for bidirectional communication
    pub pipes: Vec<String>,
    /// Shared memory regions
    pub shared_memory: Vec<String>,
    /// Message queues
    pub message_queues: Vec<String>,
    /// Communication configuration
    pub config: CommunicationConfig,
}

impl ProcessChannels {
    /// Create new process channels
    pub fn new() -> Self {
        Self {
            pipes: Vec::new(),
            shared_memory: Vec::new(),
            message_queues: Vec::new(),
            config: CommunicationConfig::default(),
        }
    }

    /// Add a named pipe
    pub fn add_pipe(&mut self, pipe_name: String) {
        self.pipes.push(pipe_name);
    }

    /// Add shared memory
    pub fn add_shared_memory(&mut self, memory_name: String) {
        self.shared_memory.push(memory_name);
    }

    /// Add message queue
    pub fn add_message_queue(&mut self, queue_name: String) {
        self.message_queues.push(queue_name);
    }

    /// Get total channel count
    pub fn total_channels(&self) -> usize {
        self.pipes.len() + self.shared_memory.len() + self.message_queues.len()
    }
}

/// Process communication configuration
#[derive(Debug, Clone)]
pub struct CommunicationConfig {
    /// Communication timeout
    pub timeout: Duration,
    /// Buffer size for communication
    pub buffer_size: usize,
    /// Enable compression for large messages
    pub enable_compression: bool,
    /// Enable encryption for sensitive data
    pub enable_encryption: bool,
    /// Maximum message size
    pub max_message_size: usize,
    /// IPC type preference
    pub ipc_type: IpcType,
}

impl Default for CommunicationConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            buffer_size: 8192,
            enable_compression: false,
            enable_encryption: false,
            max_message_size: 1024 * 1024, // 1MB
            ipc_type: IpcType::Pipe,
        }
    }
}

/// IPC type preferences
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcType {
    /// Named pipes (good for streaming)
    Pipe,
    /// Shared memory (best for large data)
    SharedMemory,
    /// Message queues (good for discrete messages)
    MessageQueue,
    /// Automatic selection based on use case
    Auto,
}

/// Process communication handle
pub struct ProcessCommunication {
    /// Process being communicated with
    pub process_id: u32,
    /// Communication channels
    pub channels: ProcessChannels,
    /// Communication statistics
    pub stats: Arc<Mutex<CommunicationStats>>,
}

impl ProcessCommunication {
    /// Create new process communication
    pub fn new(process_id: u32, channels: ProcessChannels) -> Self {
        Self {
            process_id,
            channels,
            stats: Arc::new(Mutex::new(CommunicationStats::new())),
        }
    }

    /// Send data to the process
    pub fn send_data(&self, data: &[u8]) -> ProcessResult<usize> {
        if data.len() > self.channels.config.max_message_size {
            return Err(invalid_arguments(
                "send_data",
                "data",
                &format!("Data size {} exceeds maximum {}", data.len(), self.channels.config.max_message_size)
            ));
        }

        // Select best IPC method based on configuration and data size
        let ipc_method = self.select_ipc_method(data.len());
        
        let bytes_sent = match ipc_method {
            IpcType::Pipe => self.send_via_pipe(data)?,
            IpcType::SharedMemory => self.send_via_shared_memory(data)?,
            IpcType::MessageQueue => self.send_via_message_queue(data)?,
            IpcType::Auto => unreachable!(), // Should be resolved by select_ipc_method
        };

        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.bytes_sent += bytes_sent as u64;
            stats.messages_sent += 1;
        }

        Ok(bytes_sent)
    }

    /// Receive data from the process
    pub fn receive_data(&self, buffer: &mut [u8]) -> ProcessResult<usize> {
        // Try each communication method
        let mut bytes_received = 0;

        // Try pipes first (most common)
        if !self.channels.pipes.is_empty() {
            bytes_received = self.receive_via_pipe(buffer).unwrap_or(0);
        }

        // Try shared memory if no data from pipes
        if bytes_received == 0 && !self.channels.shared_memory.is_empty() {
            bytes_received = self.receive_via_shared_memory(buffer).unwrap_or(0);
        }

        // Try message queues if no data from other methods
        if bytes_received == 0 && !self.channels.message_queues.is_empty() {
            bytes_received = self.receive_via_message_queue(buffer).unwrap_or(0);
        }

        // Update statistics
        if bytes_received > 0 {
            if let Ok(mut stats) = self.stats.lock() {
                stats.bytes_received += bytes_received as u64;
                stats.messages_received += 1;
            }
        }

        Ok(bytes_received)
    }

    /// Send and receive data (request-response pattern)
    pub fn exchange_data(&self, request: &[u8], response: &mut [u8]) -> ProcessResult<usize> {
        self.send_data(request)?;
        std::thread::sleep(Duration::from_millis(10)); // Small delay for processing
        self.receive_data(response)
    }

    /// Get communication statistics
    pub fn get_statistics(&self) -> CommunicationStats {
        self.stats.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| CommunicationStats::new())
    }

    /// Close all communication channels
    pub fn close(&self) -> ProcessResult<()> {
        // Close pipes
        for pipe_name in &self.channels.pipes {
            // In a real implementation, this would close the actual pipe
            eprintln!("Closing pipe: {}", pipe_name);
        }

        // Close shared memory
        for memory_name in &self.channels.shared_memory {
            eprintln!("Closing shared memory: {}", memory_name);
        }

        // Close message queues
        for queue_name in &self.channels.message_queues {
            eprintln!("Closing message queue: {}", queue_name);
        }

        Ok(())
    }

    fn select_ipc_method(&self, data_size: usize) -> IpcType {
        match self.channels.config.ipc_type {
            IpcType::Auto => {
                // Select based on data size and available channels
                if data_size > 64 * 1024 && !self.channels.shared_memory.is_empty() {
                    IpcType::SharedMemory
                } else if data_size < 1024 && !self.channels.message_queues.is_empty() {
                    IpcType::MessageQueue
                } else if !self.channels.pipes.is_empty() {
                    IpcType::Pipe
                } else if !self.channels.shared_memory.is_empty() {
                    IpcType::SharedMemory
                } else if !self.channels.message_queues.is_empty() {
                    IpcType::MessageQueue
                } else {
                    IpcType::Pipe // Fallback
                }
            }
            other => other,
        }
    }

    fn send_via_pipe(&self, data: &[u8]) -> ProcessResult<usize> {
        if self.channels.pipes.is_empty() {
            return Err(communication_error("send_pipe", "No pipes available"));
        }

        // In a real implementation, this would use the IPC pipe system
        // For now, we'll simulate the operation
        eprintln!("Sending {} bytes via pipe: {}", data.len(), self.channels.pipes[0]);
        Ok(data.len())
    }

    fn send_via_shared_memory(&self, data: &[u8]) -> ProcessResult<usize> {
        if self.channels.shared_memory.is_empty() {
            return Err(communication_error("send_shared_memory", "No shared memory available"));
        }

        // In a real implementation, this would use the IPC shared memory system
        eprintln!("Sending {} bytes via shared memory: {}", data.len(), self.channels.shared_memory[0]);
        Ok(data.len())
    }

    fn send_via_message_queue(&self, data: &[u8]) -> ProcessResult<usize> {
        if self.channels.message_queues.is_empty() {
            return Err(communication_error("send_message_queue", "No message queues available"));
        }

        // In a real implementation, this would use the IPC message queue system
        eprintln!("Sending {} bytes via message queue: {}", data.len(), self.channels.message_queues[0]);
        Ok(data.len())
    }

    fn receive_via_pipe(&self, buffer: &mut [u8]) -> ProcessResult<usize> {
        if self.channels.pipes.is_empty() {
            return Ok(0);
        }

        // Simulate receiving data
        let data = b"Hello from pipe";
        let bytes_to_copy = data.len().min(buffer.len());
        buffer[..bytes_to_copy].copy_from_slice(&data[..bytes_to_copy]);
        Ok(bytes_to_copy)
    }

    fn receive_via_shared_memory(&self, buffer: &mut [u8]) -> ProcessResult<usize> {
        if self.channels.shared_memory.is_empty() {
            return Ok(0);
        }

        // Simulate receiving data
        let data = b"Hello from shared memory";
        let bytes_to_copy = data.len().min(buffer.len());
        buffer[..bytes_to_copy].copy_from_slice(&data[..bytes_to_copy]);
        Ok(bytes_to_copy)
    }

    fn receive_via_message_queue(&self, buffer: &mut [u8]) -> ProcessResult<usize> {
        if self.channels.message_queues.is_empty() {
            return Ok(0);
        }

        // Simulate receiving data
        let data = b"Hello from message queue";
        let bytes_to_copy = data.len().min(buffer.len());
        buffer[..bytes_to_copy].copy_from_slice(&data[..bytes_to_copy]);
        Ok(bytes_to_copy)
    }
}

/// Communication statistics
#[derive(Debug, Clone)]
pub struct CommunicationStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub errors: u64,
    pub timeouts: u64,
}

impl CommunicationStats {
    pub fn new() -> Self {
        Self {
            bytes_sent: 0,
            bytes_received: 0,
            messages_sent: 0,
            messages_received: 0,
            errors: 0,
            timeouts: 0,
        }
    }

    pub fn record_error(&mut self) {
        self.errors += 1;
    }

    pub fn record_timeout(&mut self) {
        self.timeouts += 1;
    }
}

/// Named pipe wrapper for process communication
pub struct NamedPipe {
    pub name: String,
    pub mode: PipeMode,
}

impl NamedPipe {
    pub fn new(name: String, mode: PipeMode) -> Self {
        Self { name, mode }
    }
}

/// Pipe access mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipeMode {
    Read,
    Write,
    ReadWrite,
}

/// Shared memory wrapper for process communication
pub struct SharedMemory {
    pub name: String,
    pub size: usize,
}

impl SharedMemory {
    pub fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }
}

/// Message queue wrapper for process communication
pub struct MessageQueue {
    pub name: String,
    pub max_messages: usize,
}

impl MessageQueue {
    pub fn new(name: String, max_messages: usize) -> Self {
        Self { name, max_messages }
    }
}

/// High-level function to create process communication
pub fn create_process_communication(
    process_id: u32,
    config: CommunicationConfig,
) -> ProcessResult<ProcessCommunication> {
    let mut channels = ProcessChannels::new();
    channels.config = config;

    // Create default communication channels based on configuration
    match channels.config.ipc_type {
        IpcType::Pipe => {
            channels.add_pipe(format!("process_pipe_{}", process_id));
        }
        IpcType::SharedMemory => {
            channels.add_shared_memory(format!("process_mem_{}", process_id));
        }
        IpcType::MessageQueue => {
            channels.add_message_queue(format!("process_queue_{}", process_id));
        }
        IpcType::Auto => {
            // Create all types for automatic selection
            channels.add_pipe(format!("process_pipe_{}", process_id));
            channels.add_shared_memory(format!("process_mem_{}", process_id));
            channels.add_message_queue(format!("process_queue_{}", process_id));
        }
    }

    Ok(ProcessCommunication::new(process_id, channels))
}

/// Create a named pipe for process communication
pub fn create_pipe(name: &str, mode: PipeMode) -> ProcessResult<NamedPipe> {
    Ok(NamedPipe::new(name.to_string(), mode))
}

/// Execute a process with communication setup
pub fn execute_with_communication(
    config: ProcessConfig,
    comm_config: CommunicationConfig,
) -> ProcessResult<(Process, ProcessCommunication)> {
    // Spawn the process
    let process = crate::stdlib::process::spawn_process(config)?;
    
    // Create communication channels
    let communication = create_process_communication(process.id(), comm_config)?;
    
    Ok((process, communication))
}

/// Send data and receive response
pub fn send_and_receive(
    comm: &ProcessCommunication,
    request: &[u8],
    timeout: Duration,
) -> ProcessResult<Vec<u8>> {
    let mut response = vec![0u8; comm.channels.config.buffer_size];
    
    // Send request
    comm.send_data(request)?;
    
    // Wait for response with timeout
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        let bytes_received = comm.receive_data(&mut response)?;
        if bytes_received > 0 {
            response.truncate(bytes_received);
            return Ok(response);
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    
    Err(timeout_error("send_and_receive", timeout, "No response received"))
}

/// Create a daemon process with communication
pub fn create_daemon(
    config: ProcessConfig,
    comm_config: CommunicationConfig,
) -> ProcessResult<ProcessCommunication> {
    // Create daemon-specific configuration
    let mut daemon_config = config;
    
    #[cfg(unix)]
    {
        // On Unix, set up daemon properties
        daemon_config = daemon_config.detached();
    }
    
    // Spawn the daemon process
    let process = crate::stdlib::process::spawn_process(daemon_config)?;
    
    // Create communication for the daemon
    create_process_communication(process.id(), comm_config)
}

/// Monitor process output through communication channels
pub fn monitor_process_output(
    comm: &ProcessCommunication,
    callback: impl Fn(&[u8]) -> bool,
) -> ProcessResult<()> {
    let mut buffer = vec![0u8; comm.channels.config.buffer_size];
    
    loop {
        let bytes_received = comm.receive_data(&mut buffer)?;
        if bytes_received > 0 {
            if !callback(&buffer[..bytes_received]) {
                break; // Callback requested stop
            }
        } else {
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_channels() {
        let mut channels = ProcessChannels::new();
        assert_eq!(channels.total_channels(), 0);

        channels.add_pipe("test_pipe".to_string());
        channels.add_shared_memory("test_memory".to_string());
        channels.add_message_queue("test_queue".to_string());

        assert_eq!(channels.total_channels(), 3);
        assert_eq!(channels.pipes.len(), 1);
        assert_eq!(channels.shared_memory.len(), 1);
        assert_eq!(channels.message_queues.len(), 1);
    }

    #[test]
    fn test_communication_config() {
        let config = CommunicationConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.buffer_size, 8192);
        assert!(!config.enable_compression);
        assert!(!config.enable_encryption);
        assert_eq!(config.max_message_size, 1024 * 1024);
        assert_eq!(config.ipc_type, IpcType::Pipe);
    }

    #[test]
    fn test_process_communication_creation() {
        let channels = ProcessChannels::new();
        let comm = ProcessCommunication::new(1234, channels);
        
        assert_eq!(comm.process_id, 1234);
        assert_eq!(comm.channels.total_channels(), 0);
        
        let stats = comm.get_statistics();
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.messages_sent, 0);
    }

    #[test]
    fn test_create_process_communication() {
        let config = CommunicationConfig::default();
        let comm = create_process_communication(5678, config).unwrap();
        
        assert_eq!(comm.process_id, 5678);
        assert_eq!(comm.channels.pipes.len(), 1);
        assert_eq!(comm.channels.pipes[0], "process_pipe_5678");
    }

    #[test]
    fn test_auto_ipc_type() {
        let mut config = CommunicationConfig::default();
        config.ipc_type = IpcType::Auto;
        
        let comm = create_process_communication(9999, config).unwrap();
        
        // Should have all types available for auto selection
        assert!(!comm.channels.pipes.is_empty());
        assert!(!comm.channels.shared_memory.is_empty());
        assert!(!comm.channels.message_queues.is_empty());
    }

    #[test]
    fn test_named_pipe() {
        let pipe = NamedPipe::new("test_pipe".to_string(), PipeMode::ReadWrite);
        assert_eq!(pipe.name, "test_pipe");
        assert_eq!(pipe.mode, PipeMode::ReadWrite);
    }

    #[test]
    fn test_shared_memory() {
        let shm = SharedMemory::new("test_memory".to_string(), 4096);
        assert_eq!(shm.name, "test_memory");
        assert_eq!(shm.size, 4096);
    }

    #[test]
    fn test_message_queue() {
        let mq = MessageQueue::new("test_queue".to_string(), 100);
        assert_eq!(mq.name, "test_queue");
        assert_eq!(mq.max_messages, 100);
    }

    #[test]
    fn test_communication_stats() {
        let mut stats = CommunicationStats::new();
        assert_eq!(stats.errors, 0);
        assert_eq!(stats.timeouts, 0);

        stats.record_error();
        stats.record_timeout();

        assert_eq!(stats.errors, 1);
        assert_eq!(stats.timeouts, 1);
    }

    #[test]
    fn test_create_pipe() {
        let pipe = create_pipe("test", PipeMode::Read).unwrap();
        assert_eq!(pipe.name, "test");
        assert_eq!(pipe.mode, PipeMode::Read);
    }

    #[test]
    fn test_pipe_mode_equality() {
        assert_eq!(PipeMode::Read, PipeMode::Read);
        assert_ne!(PipeMode::Read, PipeMode::Write);
        assert_ne!(PipeMode::Write, PipeMode::ReadWrite);
    }

    #[test]
    fn test_ipc_type_equality() {
        assert_eq!(IpcType::Pipe, IpcType::Pipe);
        assert_ne!(IpcType::Pipe, IpcType::SharedMemory);
        assert_ne!(IpcType::Auto, IpcType::MessageQueue);
    }
}
