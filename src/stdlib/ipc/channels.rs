use crate::error::CursedError;
/// High-level IPC channels for CURSED
/// 
/// This module provides unified channel abstractions that can work with different
/// underlying IPC mechanisms like pipes, sockets, and message queues.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::time::{Duration, Instant};
use std::thread;
use std::io::{Read, Write};

// Placeholder imports disabled
    timeout_error, communication_error, resource_exhausted
// };

/// High-level IPC channel that can use different underlying mechanisms
#[derive(Debug)]
pub struct IpcChannel {
#[derive(Debug)]
enum ChannelInner {
/// Configuration for IPC channels
#[derive(Debug, Clone)]
pub struct ChannelConfig {
    /// Channel name/identifier
    /// Maximum capacity (for buffered channels)
    /// Default timeout for operations
    /// Whether to create the channel if it doesn't exist
    /// Channel type preference
/// Channel type preferences
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelType {
    /// Prefer named pipes (fastest, local only)
    /// Prefer message queues (structured, persistent)
    /// Use in-memory channels (testing, single process)
    /// Automatically select best option
/// In-memory channel for testing and single-process scenarios
#[derive(Debug)]
struct InMemoryChannel {
/// Channel statistics
#[derive(Debug, Clone)]
pub struct ChannelStatistics {
impl IpcChannel {
    /// Create a new IPC channel
    pub fn create(config: ChannelConfig) -> IpcResult<Self> {
        let inner = match config.channel_type {
            ChannelType::Pipe => {
                let pipe = NamedPipe::create(&config.name)?;
                ChannelInner::Pipe(pipe)
            }
            ChannelType::MessageQueue => {
                let mq = MessageQueue::create(&config.name, config.capacity.unwrap_or(100))?;
                ChannelInner::MessageQueue(mq)
            }
            ChannelType::InMemory => {
                let mem_channel = InMemoryChannel {
                ChannelInner::InMemory(Arc::new(Mutex::new(mem_channel)))
            }
            ChannelType::Auto => {
                // Try to create in order of preference: Pipe -> MessageQueue -> InMemory
                if let Ok(pipe) = NamedPipe::create(&config.name) {
                    ChannelInner::Pipe(pipe)
                } else if let Ok(mq) = MessageQueue::create(&config.name, config.capacity.unwrap_or(100)) {
                    ChannelInner::MessageQueue(mq)
                } else {
                    let mem_channel = InMemoryChannel {
                    ChannelInner::InMemory(Arc::new(Mutex::new(mem_channel)))
                }
            }

        Ok(Self {
        })
    /// Open an existing IPC channel
    pub fn open(config: ChannelConfig) -> IpcResult<Self> {
        let inner = match config.channel_type {
            ChannelType::Pipe => {
                let pipe = NamedPipe::open(&config.name)?;
                ChannelInner::Pipe(pipe)
            }
            ChannelType::MessageQueue => {
                let mq = MessageQueue::open(&config.name)?;
                ChannelInner::MessageQueue(mq)
            }
            ChannelType::InMemory => {
                return Err(communication_error("Cannot open in-memory channel from another process"));
            }
            ChannelType::Auto => {
                // Try to open in order of preference
                if let Ok(pipe) = NamedPipe::open(&config.name) {
                    ChannelInner::Pipe(pipe)
                } else if let Ok(mq) = MessageQueue::open(&config.name) {
                    ChannelInner::MessageQueue(mq)
                } else {
                    return Err(communication_error("No suitable channel found"));
                }
            }

        Ok(Self {
        })
    /// Send data through the channel
    pub fn send(&self, data: &[u8]) -> IpcResult<()> {
        self.send_with_timeout(data, self.config.default_timeout)
    /// Send data with a specific timeout
    pub fn send_with_timeout(&self, data: &[u8], timeout: Duration) -> IpcResult<()> {
        let start_time = Instant::now();
        
        let result = match &self.inner {
            ChannelInner::Pipe(pipe) => {
                // For pipes, we write directly
                let mut pipe_writer = pipe.writer()?;
                pipe_writer.write_all(data).map_err(|e| communication_error(&e.to_string()))?;
                pipe_writer.flush().map_err(|e| communication_error(&e.to_string()))?;
                Ok(())
            }
            ChannelInner::MessageQueue(mq) => {
                let message = Message::new(data, MessagePriority::Normal)?;
                mq.send_with_timeout(message, timeout)
            }
            ChannelInner::InMemory(mem_channel) => {
                self.send_in_memory(mem_channel, data, timeout)
            }

        // Update statistics
        let elapsed = start_time.elapsed();
        let mut stats = self.statistics.lock().unwrap();
        match &result {
            Ok(()) => {
                stats.messages_sent += 1;
                stats.bytes_sent += data.len() as u64;
                stats.update_send_time(elapsed.as_micros() as u64);
            }
            Err(_) => {
                stats.errors += 1;
                if elapsed >= timeout {
                    stats.send_timeouts += 1;
                }
            }
        result
    /// Receive data from the channel
    pub fn receive(&self) -> IpcResult<Vec<u8>> {
        self.receive_with_timeout(self.config.default_timeout)
    /// Receive data with a specific timeout
    pub fn receive_with_timeout(&self, timeout: Duration) -> IpcResult<Vec<u8>> {
        let start_time = Instant::now();
        
        let result = match &self.inner {
            ChannelInner::Pipe(pipe) => {
                let mut pipe_reader = pipe.reader()?;
                let mut buffer = Vec::new();
                
                // For simplicity, read all available data
                // In a production implementation, you might want to read a specific length
                // or have a protocol for message framing
                pipe_reader.read_to_end(&mut buffer)
                    .map_err(|e| communication_error(&e.to_string()))?;
                
                Ok(buffer)
            }
            ChannelInner::MessageQueue(mq) => {
                let message = mq.receive_with_timeout(timeout)?;
                Ok(message.data().to_vec())
            }
            ChannelInner::InMemory(mem_channel) => {
                self.receive_in_memory(mem_channel, timeout)
            }

        // Update statistics
        let elapsed = start_time.elapsed();
        let mut stats = self.statistics.lock().unwrap();
        match &result {
            Ok(data) => {
                stats.messages_received += 1;
                stats.bytes_received += data.len() as u64;
                stats.update_receive_time(elapsed.as_micros() as u64);
            }
            Err(_) => {
                stats.errors += 1;
                if elapsed >= timeout {
                    stats.receive_timeouts += 1;
                }
            }
        result
    /// Try to send data without blocking
    pub fn try_send(&self, data: &[u8]) -> IpcResult<bool> {
        match self.send_with_timeout(data, Duration::from_millis(0)) {
        }
    }

    /// Try to receive data without blocking
    pub fn try_receive(&self) -> IpcResult<Option<Vec<u8>>> {
        match self.receive_with_timeout(Duration::from_millis(0)) {
        }
    }

    /// Close the channel
    pub fn close(&self) -> IpcResult<()> {
        match &self.inner {
            ChannelInner::InMemory(mem_channel) => {
                let mut channel = mem_channel.lock().unwrap();
                channel.closed = true;
                Ok(())
            }
        }
    /// Get channel statistics
    pub fn get_statistics(&self) -> ChannelStatistics {
        self.statistics.lock().unwrap().clone()
    /// Check if the channel is connected/available
    pub fn is_connected(&self) -> bool {
        match &self.inner {
            ChannelInner::InMemory(mem_channel) => {
                !mem_channel.lock().unwrap().closed
            }
        }
    /// Get the actual channel type being used
    pub fn get_channel_type(&self) -> ChannelType {
        match &self.inner {
        }
    }

    // Helper methods for in-memory channels
    
    fn send_in_memory(&self, mem_channel: &Arc<Mutex<InMemoryChannel>>, data: &[u8], timeout: Duration) -> IpcResult<()> {
        let start_time = Instant::now();
        
        loop {
            {
                let mut channel = mem_channel.lock().unwrap();
                
                if channel.closed {
                    return Err(communication_error("Channel is closed"));
                // Check capacity
                if let Some(capacity) = channel.capacity {
                    if channel.queue.len() >= capacity {
                        if start_time.elapsed() >= timeout {
                            return Err(timeout_error("Send timeout"));
                        }
                        channel.writers_waiting += 1;
                        // In a real implementation, you'd use a condition variable here
                        drop(channel);
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                }
                
                // Add message to queue
                channel.queue.push_back(data.to_vec());
                return Ok(());
            }
        }
    fn receive_in_memory(&self, mem_channel: &Arc<Mutex<InMemoryChannel>>, timeout: Duration) -> IpcResult<Vec<u8>> {
        let start_time = Instant::now();
        
        loop {
            {
                let mut channel = mem_channel.lock().unwrap();
                
                if let Some(data) = channel.queue.pop_front() {
                    return Ok(data);
                if channel.closed {
                    return Err(communication_error("Channel is closed"));
                if start_time.elapsed() >= timeout {
                    return Err(timeout_error("Receive timeout"));
                channel.readers_waiting += 1;
                // In a real implementation, you'd use a condition variable here
                drop(channel);
                thread::sleep(Duration::from_millis(1));
            }
        }
    }
}

impl ChannelConfig {
    /// Create a new channel configuration
    pub fn new(name: &str) -> Self {
        Self {
        }
    }

    /// Set the channel capacity
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self
    /// Set the default timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    /// Set whether to create the channel if missing
    pub fn create_if_missing(mut self, create: bool) -> Self {
        self.create_if_missing = create;
        self
    /// Set the preferred channel type
    pub fn channel_type(mut self, channel_type: ChannelType) -> Self {
        self.channel_type = channel_type;
        self
    }
}

impl ChannelStatistics {
    fn new() -> Self {
        Self {
        }
    }

    fn update_send_time(&mut self, time_micros: u64) {
        if self.messages_sent == 1 {
            self.average_send_time_micros = time_micros;
        } else {
            // Simple moving average
            self.average_send_time_micros = 
                (self.average_send_time_micros + time_micros) / 2;
        }
    }

    fn update_receive_time(&mut self, time_micros: u64) {
        if self.messages_received == 1 {
            self.average_receive_time_micros = time_micros;
        } else {
            // Simple moving average
            self.average_receive_time_micros = 
                (self.average_receive_time_micros + time_micros) / 2;
        }
    }
/// Channel pair for bidirectional communication
#[derive(Debug)]
pub struct ChannelPair {
impl ChannelPair {
    /// Create a new channel pair
    pub fn create(base_name: &str, config: ChannelConfig) -> IpcResult<Self> {
        let sender_config = ChannelConfig {
            ..config.clone()
        
        let receiver_config = ChannelConfig {
            ..config

        let sender = IpcChannel::create(sender_config)?;
        let receiver = IpcChannel::create(receiver_config)?;

        Ok(Self { sender, receiver })
    /// Send data through the sender channel
    pub fn send(&self, data: &[u8]) -> IpcResult<()> {
        self.sender.send(data)
    /// Receive data from the receiver channel
    pub fn receive(&self) -> IpcResult<Vec<u8>> {
        self.receiver.receive()
    /// Close both channels
    pub fn close(&self) -> IpcResult<()> {
        self.sender.close()?;
        self.receiver.close()?;
        Ok(())
    }
}

