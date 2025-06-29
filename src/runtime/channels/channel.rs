//! Full-featured Go-style channel implementation
//!
//! Provides a complete channel system with:
//! - Type-safe channels with buffering strategies
//! - Integration with goroutine scheduler
//! - Select operations and multiplexing
//! - Memory-efficient operations
//! - Proper error handling and recovery

use std::sync::{Arc, Weak};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::marker::PhantomData;
use std::time::Duration;

use crate::runtime::channels::{
    ChannelError, ChannelResult, SendResult, ReceiveResult
};
use crate::runtime::channels::buffer::{ChannelBuffer, BufferType, create_buffer};
use crate::runtime::channels::operations::{
    SendOperation, ReceiveOperation, SendOptions, ReceiveOptions, OperationPriority
};
use crate::runtime::channels::memory::get_global_memory_manager;
use crate::runtime::channels::sync::{get_global_channel_sync, WaitOperationType};
use crate::runtime::goroutine::GoroutineId;

/// Channel identifier type
pub type ChannelId = usize;

/// Global channel ID counter
static NEXT_CHANNEL_ID: AtomicUsize = AtomicUsize::new(1);

/// Channel statistics
#[derive(Debug, Clone)]
pub struct ChannelStats {
    /// Total messages sent
    pub messages_sent: usize,
    /// Total messages received
    pub messages_received: usize,
    /// Messages currently in buffer
    pub messages_buffered: usize,
    /// Total send operations blocked
    pub sends_blocked: usize,
    /// Total receive operations blocked
    pub receives_blocked: usize,
    /// Average message size in bytes
    pub average_message_size: usize,
    /// Channel creation time
    pub created_at: std::time::Instant,
    /// Last activity time
    pub last_activity: Option<std::time::Instant>,
}

impl Default for ChannelStats {
    fn default() -> Self {
        Self {
            messages_sent: 0,
            messages_received: 0,
            messages_buffered: 0,
            sends_blocked: 0,
            receives_blocked: 0,
            average_message_size: 0,
            created_at: std::time::Instant::now(),
            last_activity: None,
        }
    }
}

/// Full-featured channel implementation
pub struct Channel<T: Send + 'static> {
    /// Unique channel identifier
    id: ChannelId,
    /// Channel buffer
    buffer: Arc<dyn ChannelBuffer<T>>,
    /// Channel closed flag
    closed: Arc<AtomicBool>,
    /// Number of active senders
    sender_count: Arc<AtomicUsize>,
    /// Number of active receivers
    receiver_count: Arc<AtomicUsize>,
    /// Channel statistics
    stats: Arc<std::sync::Mutex<ChannelStats>>,
    /// Buffer type information
    buffer_type: BufferType,
    /// Memory manager reference
    memory_manager: Weak<crate::runtime::channels::memory::ChannelMemoryManager>,
}

impl<T: Send + 'static> Channel<T> {
    /// Create a new unbuffered channel
    pub fn new() -> Self {
        Self::with_buffer_type(BufferType::Unbuffered)
    }
    
    /// Create a new buffered channel with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_buffer_type(BufferType::Fixed(capacity))
    }
    
    /// Create a channel with specific buffer type
    pub fn with_buffer_type(buffer_type: BufferType) -> Self {
        let id = NEXT_CHANNEL_ID.fetch_add(1, Ordering::SeqCst);
        let buffer = Arc::from(create_buffer(buffer_type));
        
        Self {
            id,
            buffer,
            closed: Arc::new(AtomicBool::new(false)),
            sender_count: Arc::new(AtomicUsize::new(0)),
            receiver_count: Arc::new(AtomicUsize::new(0)),
            stats: Arc::new(std::sync::Mutex::new(ChannelStats {
                created_at: std::time::Instant::now(),
                ..Default::default()
            })),
            buffer_type,
            memory_manager: Weak::new(),
        }
    }
    
    /// Get channel ID
    pub fn id(&self) -> ChannelId {
        self.id
    }
    
    /// Get buffer type
    pub fn buffer_type(&self) -> BufferType {
        self.buffer_type
    }
    
    /// Check if channel is closed
    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }
    
    /// Get current buffer length
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    
    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
    
    /// Check if buffer is full
    pub fn is_full(&self) -> bool {
        self.buffer.is_full()
    }
    
    /// Send a value (blocking)
    pub fn send(&self, value: T) -> SendResult<T> {
        if self.is_closed() {
            return SendResult::Closed(value);
        }
        
        let options = SendOptions::default();
        let operation = SendOperation::new(value, options);
        let result = operation.execute(&self.buffer);
        
        // Update statistics
        if matches!(result, SendResult::Sent) {
            let mut stats = self.stats.lock().unwrap();
            stats.messages_sent += 1;
            stats.last_activity = Some(std::time::Instant::now());
        }
        
        result
    }
    
    /// Try to send a value (non-blocking)
    pub fn try_send(&self, value: T) -> SendResult<T> {
        if self.is_closed() {
            return SendResult::Closed(value);
        }
        
        let mut options = SendOptions::default();
        options.blocking = false;
        
        let operation = SendOperation::new(value, options);
        let result = operation.execute(&self.buffer);
        
        // Update statistics
        if matches!(result, SendResult::Sent) {
            let mut stats = self.stats.lock().unwrap();
            stats.messages_sent += 1;
            stats.last_activity = Some(std::time::Instant::now());
        }
        
        result
    }
    
    /// Send a value with timeout
    pub fn send_timeout(&self, value: T, timeout: Duration) -> SendResult<T> {
        if self.is_closed() {
            return SendResult::Closed(value);
        }
        
        let mut options = SendOptions::default();
        options.timeout = Some(timeout);
        
        let operation = SendOperation::new(value, options);
        let result = operation.execute(&self.buffer);
        
        // Update statistics
        if matches!(result, SendResult::Sent) {
            let mut stats = self.stats.lock().unwrap();
            stats.messages_sent += 1;
            stats.last_activity = Some(std::time::Instant::now());
        }
        
        result
    }
    
    /// Receive a value (blocking)
    pub fn recv(&self) -> ReceiveResult<T> {
        let options = ReceiveOptions::default();
        let operation = ReceiveOperation::<T>::new(options);
        let result = operation.execute(&self.buffer);
        
        // Update statistics
        if matches!(result, ReceiveResult::Received(_)) {
            let mut stats = self.stats.lock().unwrap();
            stats.messages_received += 1;
            stats.last_activity = Some(std::time::Instant::now());
        }
        
        result
    }
    
    /// Try to receive a value (non-blocking)
    pub fn try_recv(&self) -> ReceiveResult<T> {
        let mut options = ReceiveOptions::default();
        options.blocking = false;
        
        let operation = ReceiveOperation::<T>::new(options);
        let result = operation.execute(&self.buffer);
        
        // Update statistics
        if matches!(result, ReceiveResult::Received(_)) {
            let mut stats = self.stats.lock().unwrap();
            stats.messages_received += 1;
            stats.last_activity = Some(std::time::Instant::now());
        }
        
        result
    }
    
    /// Receive a value with timeout
    pub fn recv_timeout(&self, timeout: Duration) -> ReceiveResult<T> {
        let mut options = ReceiveOptions::default();
        options.timeout = Some(timeout);
        
        let operation = ReceiveOperation::<T>::new(options);
        let result = operation.execute(&self.buffer);
        
        // Update statistics
        if matches!(result, ReceiveResult::Received(_)) {
            let mut stats = self.stats.lock().unwrap();
            stats.messages_received += 1;
            stats.last_activity = Some(std::time::Instant::now());
        }
        
        result
    }
    
    /// Close the channel
    pub fn close(&self) {
        if !self.closed.swap(true, Ordering::AcqRel) {
            // First time closing
            self.buffer.close();
            
            // Clean up any waiting goroutines
            get_global_channel_sync().cleanup_channel(self.id);
            
            // Update statistics
            let mut stats = self.stats.lock().unwrap();
            stats.last_activity = Some(std::time::Instant::now());
        }
    }
    
    /// Get channel statistics
    pub fn stats(&self) -> ChannelStats {
        let mut stats = self.stats.lock().unwrap().clone();
        stats.messages_buffered = self.buffer.len();
        stats
    }
    
    /// Create a sender handle
    pub fn sender(&self) -> ChannelSender<T> {
        self.sender_count.fetch_add(1, Ordering::SeqCst);
        ChannelSender {
            channel: self.clone(),
            _phantom: PhantomData,
        }
    }
    
    /// Create a receiver handle
    pub fn receiver(&self) -> ChannelReceiver<T> {
        self.receiver_count.fetch_add(1, Ordering::SeqCst);
        ChannelReceiver {
            channel: self.clone(),
            _phantom: PhantomData,
        }
    }
    
    /// Get number of active senders
    pub fn sender_count(&self) -> usize {
        self.sender_count.load(Ordering::Acquire)
    }
    
    /// Get number of active receivers
    pub fn receiver_count(&self) -> usize {
        self.receiver_count.load(Ordering::Acquire)
    }
}

impl<T: Send + 'static> Clone for Channel<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            buffer: self.buffer.clone(),
            closed: self.closed.clone(),
            sender_count: self.sender_count.clone(),
            receiver_count: self.receiver_count.clone(),
            stats: self.stats.clone(),
            buffer_type: self.buffer_type,
            memory_manager: self.memory_manager.clone(),
        }
    }
}

impl<T: Send + 'static> Drop for Channel<T> {
    fn drop(&mut self) {
        // Close the channel if it's the last reference
        if Arc::strong_count(&self.closed) == 1 {
            self.close();
        }
    }
}

/// Channel sender handle
pub struct ChannelSender<T: Send + 'static> {
    channel: Channel<T>,
    _phantom: PhantomData<T>,
}

impl<T: Send + 'static> ChannelSender<T> {
    /// Send a value
    pub fn send(&self, value: T) -> SendResult<T> {
        self.channel.send(value)
    }
    
    /// Try to send a value (non-blocking)
    pub fn try_send(&self, value: T) -> SendResult<T> {
        self.channel.try_send(value)
    }
    
    /// Send a value with timeout
    pub fn send_timeout(&self, value: T, timeout: Duration) -> SendResult<T> {
        self.channel.send_timeout(value, timeout)
    }
    
    /// Close the channel
    pub fn close(&self) {
        self.channel.close();
    }
    
    /// Check if channel is closed
    pub fn is_closed(&self) -> bool {
        self.channel.is_closed()
    }
    
    /// Get channel ID
    pub fn channel_id(&self) -> ChannelId {
        self.channel.id()
    }
}

impl<T: Send + 'static> Clone for ChannelSender<T> {
    fn clone(&self) -> Self {
        self.channel.sender_count.fetch_add(1, Ordering::SeqCst);
        Self {
            channel: self.channel.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T: Send + 'static> Drop for ChannelSender<T> {
    fn drop(&mut self) {
        let prev_count = self.channel.sender_count.fetch_sub(1, Ordering::SeqCst);
        
        // If this was the last sender and channel is unbuffered, close it
        if prev_count == 1 && matches!(self.channel.buffer_type, BufferType::Unbuffered) {
            self.channel.close();
        }
    }
}

/// Channel receiver handle
pub struct ChannelReceiver<T: Send + 'static> {
    channel: Channel<T>,
    _phantom: PhantomData<T>,
}

impl<T: Send + 'static> ChannelReceiver<T> {
    /// Receive a value
    pub fn recv(&self) -> ReceiveResult<T> {
        self.channel.recv()
    }
    
    /// Try to receive a value (non-blocking)
    pub fn try_recv(&self) -> ReceiveResult<T> {
        self.channel.try_recv()
    }
    
    /// Receive a value with timeout
    pub fn recv_timeout(&self, timeout: Duration) -> ReceiveResult<T> {
        self.channel.recv_timeout(timeout)
    }
    
    /// Check if channel is closed
    pub fn is_closed(&self) -> bool {
        self.channel.is_closed()
    }
    
    /// Get channel ID
    pub fn channel_id(&self) -> ChannelId {
        self.channel.id()
    }
    
    /// Create an iterator for range-based receiving
    pub fn into_iter(self) -> ChannelIterator<T> {
        ChannelIterator {
            receiver: self,
        }
    }
}

impl<T: Send + 'static> Clone for ChannelReceiver<T> {
    fn clone(&self) -> Self {
        self.channel.receiver_count.fetch_add(1, Ordering::SeqCst);
        Self {
            channel: self.channel.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T: Send> Drop for ChannelReceiver<T> {
    fn drop(&mut self) {
        self.channel.receiver_count.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Iterator for channel range operations
pub struct ChannelIterator<T: Send + 'static> {
    receiver: ChannelReceiver<T>,
}

impl<T: Send + 'static> Iterator for ChannelIterator<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.receiver.recv() {
            ReceiveResult::Received(value) => Some(value),
            ReceiveResult::Closed | ReceiveResult::WouldBlock => None,
        }
    }
}

/// Convenience function to create a channel pair
pub fn channel<T: Send + 'static>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    let ch = Channel::new();
    let sender = ch.sender();
    let receiver = ch.receiver();
    (sender, receiver)
}

/// Convenience function to create a buffered channel pair
pub fn buffered_channel<T: Send + 'static>(capacity: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    let ch = Channel::with_capacity(capacity);
    let sender = ch.sender();
    let receiver = ch.receiver();
    (sender, receiver)
}

/// CURSED syntax support - dm<T> type alias for channels
pub type Dm<T: Send> = Channel<T>;

/// CURSED syntax support - channel creation with dm keyword
pub fn dm<T: Send + 'static>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    channel()
}

/// CURSED syntax support - buffered channel creation
pub fn dm_buffered<T: Send + 'static>(capacity: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    buffered_channel(capacity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_unbuffered_channel() {
        let (sender, receiver) = channel::<i32>();
        
        // Spawn a thread to send
        let sender_handle = thread::spawn(move || {
            assert!(matches!(sender.send(42), SendResult::Sent));
        });
        
        // Receive in main thread
        match receiver.recv() {
            ReceiveResult::Received(value) => assert_eq!(value, 42),
            _ => panic!("Should receive value"),
        }
        
        sender_handle.join().unwrap();
    }

    #[test]
    fn test_buffered_channel() {
        let (sender, receiver) = buffered_channel::<i32>(3);
        
        // Should be able to send without blocking
        assert!(matches!(sender.send(1), SendResult::Sent));
        assert!(matches!(sender.send(2), SendResult::Sent));
        assert!(matches!(sender.send(3), SendResult::Sent));
        
        // Should be full now
        assert!(matches!(sender.try_send(4), SendResult::WouldBlock(_)));
        
        // Should be able to receive
        assert!(matches!(receiver.recv(), ReceiveResult::Received(1)));
        assert!(matches!(receiver.recv(), ReceiveResult::Received(2)));
        assert!(matches!(receiver.recv(), ReceiveResult::Received(3)));
    }

    #[test]
    fn test_channel_close() {
        let (sender, receiver) = channel::<i32>();
        
        assert!(matches!(sender.send(42), SendResult::Sent));
        sender.close();
        
        // Should still be able to receive buffered value
        assert!(matches!(receiver.recv(), ReceiveResult::Received(42)));
        
        // Further receives should indicate closed (use try_recv to avoid hanging)
        assert!(matches!(receiver.try_recv(), ReceiveResult::Closed));
        
        // Sends should fail
        assert!(matches!(sender.send(43), SendResult::Closed(43)));
    }

    #[test]
    fn test_channel_iterator() {
        let (sender, receiver) = buffered_channel::<i32>(5);
        
        // Send some values
        for i in 1..=5 {
            assert!(matches!(sender.send(i), SendResult::Sent));
        }
        sender.close();
        
        // Collect using iterator
        let values: Vec<i32> = receiver.into_iter().collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_channel_stats() {
        let (sender, receiver) = buffered_channel::<i32>(3);
        
        assert!(matches!(sender.send(1), SendResult::Sent));
        assert!(matches!(sender.send(2), SendResult::Sent));
        
        let stats = sender.channel.stats();
        assert_eq!(stats.messages_sent, 2);
        assert_eq!(stats.messages_buffered, 2);
        
        assert!(matches!(receiver.recv(), ReceiveResult::Received(_)));
        
        let stats = receiver.channel.stats();
        assert_eq!(stats.messages_received, 1);
        assert_eq!(stats.messages_buffered, 1);
    }

    #[test]
    fn test_cursed_syntax() {
        let (sender, receiver) = dm::<String>();
        
        assert!(matches!(sender.send("hello".to_string()), SendResult::Sent));
        
        match receiver.recv() {
            ReceiveResult::Received(value) => assert_eq!(value, "hello"),
            _ => panic!("Should receive value"),
        }
    }
}
