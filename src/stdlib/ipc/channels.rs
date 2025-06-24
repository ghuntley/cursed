use crate::error::Error;
/// High-level IPC channels for CURSED
/// 
/// This module provides unified channel abstractions that can work with different
/// underlying IPC mechanisms like pipes, sockets, and message queues.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::time::{Duration, Instant};
use std::thread;
use std::io::{Read, Write};

use crate::stdlib::ipc::{
    IpcResult, IpcError, IpcTimeout, ProcessId,
    NamedPipe, MessageQueue, Message, MessagePriority,
    timeout_error, communication_error, resource_exhausted
};

/// High-level IPC channel that can use different underlying mechanisms
#[derive(Debug)]
pub struct IpcChannel {
    inner: ChannelInner,
    config: ChannelConfig,
    statistics: Arc<Mutex<ChannelStatistics>>,
}

#[derive(Debug)]
enum ChannelInner {
    Pipe(NamedPipe),
    MessageQueue(MessageQueue),
    InMemory(Arc<Mutex<InMemoryChannel>>),
}

/// Configuration for IPC channels
#[derive(Debug, Clone)]
pub struct ChannelConfig {
    /// Channel name/identifier
    pub name: String,
    /// Maximum capacity (for buffered channels)
    pub capacity: Option<usize>,
    /// Default timeout for operations
    pub default_timeout: Duration,
    /// Whether to create the channel if it doesn't exist
    pub create_if_missing: bool,
    /// Channel type preference
    pub channel_type: ChannelType,
}

/// Channel type preferences
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelType {
    /// Prefer named pipes (fastest, local only)
    Pipe,
    /// Prefer message queues (structured, persistent)
    MessageQueue,
    /// Use in-memory channels (testing, single process)
    InMemory,
    /// Automatically select best option
    Auto,
}

/// In-memory channel for testing and single-process scenarios
#[derive(Debug)]
struct InMemoryChannel {
    queue: VecDeque<Vec<u8>>,
    capacity: Option<usize>,
    readers_waiting: usize,
    writers_waiting: usize,
    closed: bool,
}

/// Channel statistics
#[derive(Debug, Clone)]
pub struct ChannelStatistics {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub send_timeouts: u64,
    pub receive_timeouts: u64,
    pub errors: u64,
    pub average_send_time_micros: u64,
    pub average_receive_time_micros: u64,
}

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
                    queue: VecDeque::new(),
                    capacity: config.capacity,
                    readers_waiting: 0,
                    writers_waiting: 0,
                    closed: false,
                };
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
                        queue: VecDeque::new(),
                        capacity: config.capacity,
                        readers_waiting: 0,
                        writers_waiting: 0,
                        closed: false,
                    };
                    ChannelInner::InMemory(Arc::new(Mutex::new(mem_channel)))
                }
            }
        };

        Ok(Self {
            inner,
            config,
            statistics: Arc::new(Mutex::new(ChannelStatistics::new())),
        })
    }

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
        };

        Ok(Self {
            inner,
            config,
            statistics: Arc::new(Mutex::new(ChannelStatistics::new())),
        })
    }

    /// Send data through the channel
    pub fn send(&self, data: &[u8]) -> IpcResult<()> {
        self.send_with_timeout(data, self.config.default_timeout)
    }

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
        };

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
        }

        result
    }

    /// Receive data from the channel
    pub fn receive(&self) -> IpcResult<Vec<u8>> {
        self.receive_with_timeout(self.config.default_timeout)
    }

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
        };

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
        }

        result
    }

    /// Try to send data without blocking
    pub fn try_send(&self, data: &[u8]) -> IpcResult<bool> {
        match self.send_with_timeout(data, Duration::from_millis(0)) {
            Ok(()) => Ok(true),
            Err(IpcError::TimeoutError { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Try to receive data without blocking
    pub fn try_receive(&self) -> IpcResult<Option<Vec<u8>>> {
        match self.receive_with_timeout(Duration::from_millis(0)) {
            Ok(data) => Ok(Some(data)),
            Err(IpcError::TimeoutError { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Close the channel
    pub fn close(&self) -> IpcResult<()> {
        match &self.inner {
            ChannelInner::Pipe(pipe) => pipe.close(),
            ChannelInner::MessageQueue(mq) => mq.close(),
            ChannelInner::InMemory(mem_channel) => {
                let mut channel = mem_channel.lock().unwrap();
                channel.closed = true;
                Ok(())
            }
        }
    }

    /// Get channel statistics
    pub fn get_statistics(&self) -> ChannelStatistics {
        self.statistics.lock().unwrap().clone()
    }

    /// Check if the channel is connected/available
    pub fn is_connected(&self) -> bool {
        match &self.inner {
            ChannelInner::Pipe(pipe) => pipe.is_connected(),
            ChannelInner::MessageQueue(mq) => mq.is_available(),
            ChannelInner::InMemory(mem_channel) => {
                !mem_channel.lock().unwrap().closed
            }
        }
    }

    /// Get the actual channel type being used
    pub fn get_channel_type(&self) -> ChannelType {
        match &self.inner {
            ChannelInner::Pipe(_) => ChannelType::Pipe,
            ChannelInner::MessageQueue(_) => ChannelType::MessageQueue,
            ChannelInner::InMemory(_) => ChannelType::InMemory,
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
                }
                
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
    }
    
    fn receive_in_memory(&self, mem_channel: &Arc<Mutex<InMemoryChannel>>, timeout: Duration) -> IpcResult<Vec<u8>> {
        let start_time = Instant::now();
        
        loop {
            {
                let mut channel = mem_channel.lock().unwrap();
                
                if let Some(data) = channel.queue.pop_front() {
                    return Ok(data);
                }
                
                if channel.closed {
                    return Err(communication_error("Channel is closed"));
                }
                
                if start_time.elapsed() >= timeout {
                    return Err(timeout_error("Receive timeout"));
                }
                
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
            name: name.to_string(),
            capacity: None,
            default_timeout: Duration::from_secs(30),
            create_if_missing: true,
            channel_type: ChannelType::Auto,
        }
    }

    /// Set the channel capacity
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self
    }

    /// Set the default timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Set whether to create the channel if missing
    pub fn create_if_missing(mut self, create: bool) -> Self {
        self.create_if_missing = create;
        self
    }

    /// Set the preferred channel type
    pub fn channel_type(mut self, channel_type: ChannelType) -> Self {
        self.channel_type = channel_type;
        self
    }
}

impl ChannelStatistics {
    fn new() -> Self {
        Self {
            messages_sent: 0,
            messages_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
            send_timeouts: 0,
            receive_timeouts: 0,
            errors: 0,
            average_send_time_micros: 0,
            average_receive_time_micros: 0,
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
}

/// Channel pair for bidirectional communication
#[derive(Debug)]
pub struct ChannelPair {
    pub sender: IpcChannel,
    pub receiver: IpcChannel,
}

impl ChannelPair {
    /// Create a new channel pair
    pub fn create(base_name: &str, config: ChannelConfig) -> IpcResult<Self> {
        let sender_config = ChannelConfig {
            name: format!("{}_send", base_name),
            ..config.clone()
        };
        
        let receiver_config = ChannelConfig {
            name: format!("{}_recv", base_name),
            ..config
        };

        let sender = IpcChannel::create(sender_config)?;
        let receiver = IpcChannel::create(receiver_config)?;

        Ok(Self { sender, receiver })
    }

    /// Send data through the sender channel
    pub fn send(&self, data: &[u8]) -> IpcResult<()> {
        self.sender.send(data)
    }

    /// Receive data from the receiver channel
    pub fn receive(&self) -> IpcResult<Vec<u8>> {
        self.receiver.receive()
    }

    /// Close both channels
    pub fn close(&self) -> IpcResult<()> {
        self.sender.close()?;
        self.receiver.close()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
use crate::stdlib::process::real_ipc::IpcChannel;

    #[test]
    fn test_in_memory_channel_basic() {
        let config = ChannelConfig::new("test_channel")
            .channel_type(ChannelType::InMemory)
            .timeout(Duration::from_millis(100));

        let channel = IpcChannel::create(config).unwrap();
        
        // Test send and receive
        let test_data = b"Hello, World!";
        channel.send(test_data).unwrap();
        
        let received = channel.receive().unwrap();
        assert_eq!(received, test_data);
    }

    #[test]
    fn test_in_memory_channel_timeout() {
        let config = ChannelConfig::new("test_timeout")
            .channel_type(ChannelType::InMemory)
            .timeout(Duration::from_millis(10));

        let channel = IpcChannel::create(config).unwrap();
        
        // Try to receive from empty channel should timeout
        let result = channel.receive();
        assert!(result.is_err());
    }

    #[test]
    fn test_in_memory_channel_capacity() {
        let config = ChannelConfig::new("test_capacity")
            .channel_type(ChannelType::InMemory)
            .capacity(2)
            .timeout(Duration::from_millis(10));

        let channel = IpcChannel::create(config).unwrap();
        
        // Fill capacity
        channel.send(b"message1").unwrap();
        channel.send(b"message2").unwrap();
        
        // Third message should timeout
        let result = channel.send(b"message3");
        assert!(result.is_err());
    }

    #[test]
    fn test_channel_statistics() {
        let config = ChannelConfig::new("test_stats")
            .channel_type(ChannelType::InMemory);

        let channel = IpcChannel::create(config).unwrap();
        
        let test_data = b"test data";
        channel.send(test_data).unwrap();
        let _received = channel.receive().unwrap();
        
        let stats = channel.get_statistics();
        assert_eq!(stats.messages_sent, 1);
        assert_eq!(stats.messages_received, 1);
        assert_eq!(stats.bytes_sent, test_data.len() as u64);
        assert_eq!(stats.bytes_received, test_data.len() as u64);
    }

    #[test]
    fn test_channel_pair() {
        let config = ChannelConfig::new("test_pair")
            .channel_type(ChannelType::InMemory);

        let pair = ChannelPair::create("test_pair", config).unwrap();
        
        let test_data = b"paired communication";
        pair.send(test_data).unwrap();
        let received = pair.receive().unwrap();
        assert_eq!(received, test_data);
    }

    #[test]
    fn test_try_operations() {
        let config = ChannelConfig::new("test_try")
            .channel_type(ChannelType::InMemory);

        let channel = IpcChannel::create(config).unwrap();
        
        // Try receive on empty channel should return None
        let result = channel.try_receive().unwrap();
        assert!(result.is_none());
        
        // Send and try receive should work
        let test_data = b"try operations";
        assert!(channel.try_send(test_data).unwrap());
        
        let received = channel.try_receive().unwrap();
        assert!(received.is_some());
        assert_eq!(received.unwrap(), test_data);
    }

    #[test]
    fn test_channel_close() {
        let config = ChannelConfig::new("test_close")
            .channel_type(ChannelType::InMemory);

        let channel = IpcChannel::create(config).unwrap();
        
        assert!(channel.is_connected());
        channel.close().unwrap();
        
        // Operations after close should fail
        let result = channel.send(b"after close");
        assert!(result.is_err());
    }
}
