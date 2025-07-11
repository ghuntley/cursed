//! Simplified Go-style channel implementation for CURSED
//!
//! This provides a working channel system with basic functionality:
//! - Unbuffered and buffered channels
//! - Send and receive operations
//! - Channel closing and proper cleanup
//! - Integration with goroutine scheduler

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::fmt;

use crate::runtime::channels::{SendResult, ReceiveResult};
use crate::runtime::goroutine::GoroutineId;

/// Simple channel implementation
pub struct SimpleChannel<T> {
    /// Channel ID
    id: usize,
    /// Internal buffer
    buffer: Arc<Mutex<VecDeque<T>>>,
    /// Channel closed flag
    closed: Arc<AtomicBool>,
    /// Buffer capacity (0 = unbuffered)
    capacity: usize,
    /// Condition variable for senders
    sender_notify: Arc<Condvar>,
    /// Condition variable for receivers  
    receiver_notify: Arc<Condvar>,
    /// Number of active senders
    sender_count: Arc<AtomicUsize>,
    /// Number of active receivers
    receiver_count: Arc<AtomicUsize>,
}

impl<T> SimpleChannel<T> {
    /// Create a new unbuffered channel
    pub fn new() -> Self {
        Self::with_capacity(0)
    }
    
    /// Create a new buffered channel
    pub fn with_capacity(capacity: usize) -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
        
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            closed: Arc::new(AtomicBool::new(false)),
            capacity,
            sender_notify: Arc::new(Condvar::new()),
            receiver_notify: Arc::new(Condvar::new()),
            sender_count: Arc::new(AtomicUsize::new(0)),
            receiver_count: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    /// Send a value (blocking)
    pub fn send(&self, value: T) -> SendResult<T> {
        if self.is_closed() {
            return SendResult::Closed(value);
        }
        
        let mut buffer = self.buffer.lock().unwrap();
        
        // For unbuffered channels, wait for receiver
        if self.capacity == 0 {
            // Wait for receiver to be available
            while self.receiver_count.load(Ordering::SeqCst) == 0 && !self.is_closed() {
                buffer = self.sender_notify.wait(buffer).unwrap();
            }
            
            if self.is_closed() {
                return SendResult::Closed(value);
            }
            
            buffer.push_back(value);
            self.receiver_notify.notify_one();
            return SendResult::Sent;
        }
        
        // For buffered channels, wait for space
        while buffer.len() >= self.capacity && !self.is_closed() {
            buffer = self.sender_notify.wait(buffer).unwrap();
        }
        
        if self.is_closed() {
            return SendResult::Closed(value);
        }
        
        buffer.push_back(value);
        self.receiver_notify.notify_one();
        SendResult::Sent
    }
    
    /// Try to send a value (non-blocking)
    pub fn try_send(&self, value: T) -> SendResult<T> {
        if self.is_closed() {
            return SendResult::Closed(value);
        }
        
        let mut buffer = self.buffer.lock().unwrap();
        
        // For unbuffered channels, need receiver
        if self.capacity == 0 {
            if self.receiver_count.load(Ordering::SeqCst) == 0 {
                return SendResult::WouldBlock(value);
            }
            buffer.push_back(value);
            self.receiver_notify.notify_one();
            return SendResult::Sent;
        }
        
        // For buffered channels, check capacity
        if buffer.len() >= self.capacity {
            return SendResult::WouldBlock(value);
        }
        
        buffer.push_back(value);
        self.receiver_notify.notify_one();
        SendResult::Sent
    }
    
    /// Send with timeout
    pub fn send_timeout(&self, value: T, timeout: Duration) -> SendResult<T> {
        let start = Instant::now();
        let mut result = self.try_send(value);
        
        while let SendResult::WouldBlock(v) = result {
            if start.elapsed() >= timeout {
                return SendResult::WouldBlock(v);
            }
            
            std::thread::sleep(Duration::from_millis(1));
            result = self.try_send(v);
        }
        
        result
    }
    
    /// Receive a value (blocking)
    pub fn recv(&self) -> ReceiveResult<T> {
        let mut buffer = self.buffer.lock().unwrap();
        
        // Wait for data or channel close
        while buffer.is_empty() && !self.is_closed() {
            buffer = self.receiver_notify.wait(buffer).unwrap();
        }
        
        if let Some(value) = buffer.pop_front() {
            self.sender_notify.notify_one();
            return ReceiveResult::Received(value);
        }
        
        if self.is_closed() {
            ReceiveResult::Closed
        } else {
            // Shouldn't happen, but handle gracefully
            ReceiveResult::WouldBlock
        }
    }
    
    /// Try to receive a value (non-blocking)
    pub fn try_recv(&self) -> ReceiveResult<T> {
        let mut buffer = self.buffer.lock().unwrap();
        
        if let Some(value) = buffer.pop_front() {
            self.sender_notify.notify_one();
            ReceiveResult::Received(value)
        } else if self.is_closed() {
            ReceiveResult::Closed
        } else {
            ReceiveResult::WouldBlock
        }
    }
    
    /// Receive with timeout
    pub fn recv_timeout(&self, timeout: Duration) -> ReceiveResult<T> {
        let start = Instant::now();
        let mut result = self.try_recv();
        
        while let ReceiveResult::WouldBlock = result {
            if start.elapsed() >= timeout {
                return ReceiveResult::WouldBlock;
            }
            
            std::thread::sleep(Duration::from_millis(1));
            result = self.try_recv();
        }
        
        result
    }
    
    /// Close the channel
    pub fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        self.sender_notify.notify_all();
        self.receiver_notify.notify_all();
    }
    
    /// Check if channel is closed
    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }
    
    /// Get channel ID
    pub fn id(&self) -> usize {
        self.id
    }
    
    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get current length
    pub fn len(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.buffer.lock().unwrap().is_empty()
    }
    
    /// Send a value with goroutine context (for scheduler integration)
    pub fn send_with_goroutine(&self, value: T, _goroutine_id: Option<GoroutineId>) -> SendResult<T> {
        // For now, just use regular send - future integration point for scheduler
        self.send(value)
    }
    
    /// Receive a value with goroutine context (for scheduler integration)
    pub fn recv_with_goroutine(&self, _goroutine_id: Option<GoroutineId>) -> ReceiveResult<T> {
        // For now, just use regular recv - future integration point for scheduler
        self.recv()
    }
    
    /// Create a sender handle
    pub fn sender(&self) -> SimpleChannelSender<T> {
        self.sender_count.fetch_add(1, Ordering::SeqCst);
        SimpleChannelSender {
            channel: self.clone(),
        }
    }
    
    /// Create a receiver handle
    pub fn receiver(&self) -> SimpleChannelReceiver<T> {
        self.receiver_count.fetch_add(1, Ordering::SeqCst);
        SimpleChannelReceiver {
            channel: self.clone(),
        }
    }
}

impl<T> Clone for SimpleChannel<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            buffer: self.buffer.clone(),
            closed: self.closed.clone(),
            capacity: self.capacity,
            sender_notify: self.sender_notify.clone(),
            receiver_notify: self.receiver_notify.clone(),
            sender_count: self.sender_count.clone(),
            receiver_count: self.receiver_count.clone(),
        }
    }
}

/// Simple channel sender
pub struct SimpleChannelSender<T> {
    channel: SimpleChannel<T>,
}

impl<T> SimpleChannelSender<T> {
    /// Send a value
    pub fn send(&self, value: T) -> SendResult<T> {
        self.channel.send(value)
    }
    
    /// Try to send a value (non-blocking)
    pub fn try_send(&self, value: T) -> SendResult<T> {
        self.channel.try_send(value)
    }
    
    /// Send with timeout
    pub fn send_timeout(&self, value: T, timeout: Duration) -> SendResult<T> {
        self.channel.send_timeout(value, timeout)
    }
    
    /// Close the channel
    pub fn close(&self) {
        self.channel.close();
    }
    
    /// Check if closed
    pub fn is_closed(&self) -> bool {
        self.channel.is_closed()
    }
    
    /// Get channel ID
    pub fn channel_id(&self) -> usize {
        self.channel.id()
    }
}

impl<T> Clone for SimpleChannelSender<T> {
    fn clone(&self) -> Self {
        self.channel.sender_count.fetch_add(1, Ordering::SeqCst);
        Self {
            channel: self.channel.clone(),
        }
    }
}

impl<T> Drop for SimpleChannelSender<T> {
    fn drop(&mut self) {
        let count = self.channel.sender_count.fetch_sub(1, Ordering::SeqCst);
        // If this was the last sender, close the channel
        if count == 1 {
            self.channel.close();
        }
    }
}

/// Simple channel receiver
pub struct SimpleChannelReceiver<T> {
    channel: SimpleChannel<T>,
}

impl<T> SimpleChannelReceiver<T> {
    /// Receive a value
    pub fn recv(&self) -> ReceiveResult<T> {
        self.channel.recv()
    }
    
    /// Try to receive a value (non-blocking)
    pub fn try_recv(&self) -> ReceiveResult<T> {
        self.channel.try_recv()
    }
    
    /// Receive with timeout
    pub fn recv_timeout(&self, timeout: Duration) -> ReceiveResult<T> {
        self.channel.recv_timeout(timeout)
    }
    
    /// Check if closed
    pub fn is_closed(&self) -> bool {
        self.channel.is_closed()
    }
    
    /// Get channel ID
    pub fn channel_id(&self) -> usize {
        self.channel.id()
    }
    
    /// Create an iterator
    pub fn into_iter(self) -> SimpleChannelIterator<T> {
        SimpleChannelIterator { receiver: self }
    }
}

impl<T> Clone for SimpleChannelReceiver<T> {
    fn clone(&self) -> Self {
        self.channel.receiver_count.fetch_add(1, Ordering::SeqCst);
        Self {
            channel: self.channel.clone(),
        }
    }
}

impl<T> Drop for SimpleChannelReceiver<T> {
    fn drop(&mut self) {
        self.channel.receiver_count.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Iterator for channel range operations
pub struct SimpleChannelIterator<T> {
    receiver: SimpleChannelReceiver<T>,
}

impl<T> Iterator for SimpleChannelIterator<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.receiver.recv() {
            ReceiveResult::Received(value) => Some(value),
            ReceiveResult::Closed | ReceiveResult::WouldBlock => None,
        }
    }
}

/// Create a simple channel pair
pub fn simple_channel<T>() -> (SimpleChannelSender<T>, SimpleChannelReceiver<T>) {
    let channel = SimpleChannel::new();
    let sender = channel.sender();
    let receiver = channel.receiver();
    (sender, receiver)
}

/// Create a simple buffered channel pair
pub fn simple_buffered_channel<T>(capacity: usize) -> (SimpleChannelSender<T>, SimpleChannelReceiver<T>) {
    let channel = SimpleChannel::with_capacity(capacity);
    let sender = channel.sender();
    let receiver = channel.receiver();
    (sender, receiver)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_unbuffered_channel() {
        let (sender, receiver) = simple_channel::<i32>();
        
        // Spawn a thread to send
        let sender_handle = thread::spawn(move || {
            assert!(matches!(sender.send(42), SendResult::Sent));
        });
        
        // Receive in main thread
        match receiver.recv() {
            ReceiveResult::Received(value) => assert_eq!(value, 42),
            ReceiveResult::Closed => {
                eprintln!("Channel closed unexpectedly");
                assert!(false, "Channel should not be closed")
            },
            ReceiveResult::WouldBlock => {
                eprintln!("Receive would block unexpectedly");
                assert!(false, "Receive should not block on unbuffered channel")
            },
        }
        
        sender_handle.join().unwrap();
    }

    #[test]
    fn test_buffered_channel() {
        let (sender, receiver) = simple_buffered_channel::<i32>(3);
        
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
        let (sender, receiver) = simple_channel::<i32>();
        
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
        let (sender, receiver) = simple_buffered_channel::<i32>(5);
        
        // Send some values
        for i in 1..=5 {
            assert!(matches!(sender.send(i), SendResult::Sent));
        }
        sender.close();
        
        // Collect using iterator
        let values: Vec<i32> = receiver.into_iter().collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5]);
    }
}

impl<T> fmt::Debug for SimpleChannel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleChannel")
            .field("id", &self.id)
            .field("capacity", &self.capacity)
            .field("closed", &self.closed.load(Ordering::SeqCst))
            .field("sender_count", &self.sender_count.load(Ordering::SeqCst))
            .field("receiver_count", &self.receiver_count.load(Ordering::SeqCst))
            .finish()
    }
}
