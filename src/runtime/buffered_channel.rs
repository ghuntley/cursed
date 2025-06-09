//! Comprehensive buffered channel implementation for CURSED
//!
//! This module provides an efficient buffered channel implementation using
//! a circular buffer for optimal performance in concurrent scenarios.

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

use crate::object::Object;
use crate::error::Error;

/// A buffered channel that uses a circular buffer for efficient operations
#[derive(Debug)]
pub struct BufferedChannel {
    /// Element type for type safety
    element_type: String,
    /// The circular buffer storing channel values
    buffer: VecDeque<Object>,
    /// Maximum capacity of the buffer
    capacity: usize,
    /// Whether the channel has been closed
    closed: bool,
    /// Number of goroutines waiting to send
    send_waiters: usize,
    /// Number of goroutines waiting to receive
    recv_waiters: usize,
}

/// Thread-safe wrapper for buffered channels
#[derive(Debug)]
pub struct ThreadSafeBufferedChannel {
    /// The channel data protected by a mutex
    inner: Arc<Mutex<BufferedChannel>>,
    /// Condition variable for send operations
    send_cond: Arc<Condvar>,
    /// Condition variable for receive operations
    recv_cond: Arc<Condvar>,
}

impl BufferedChannel {
    /// Create a new buffered channel
    #[tracing::instrument(fields(element_type = ?element_type, capacity = capacity), level = "debug")]
    pub fn new(element_type: String, capacity: usize) -> Self {
        debug!(element_type = ?element_type, capacity = capacity, "Creating buffered channel");
        
        Self {
            element_type,
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            closed: false,
            send_waiters: 0,
            recv_waiters: 0,
        }
    }

    /// Check if the channel is full
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }

    /// Check if the channel is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Check if the channel is closed
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Get current length of the buffer
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Get the capacity of the channel
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get available space in the buffer
    pub fn available_space(&self) -> usize {
        self.capacity.saturating_sub(self.buffer.len())
    }

    /// Try to send a value without blocking
    #[tracing::instrument(skip(self, value), fields(buffer_len = self.buffer.len(), capacity = self.capacity, closed = self.closed), level = "debug")]
    pub fn try_send(&mut self, value: Object) -> Result<bool, Error> {
        if self.closed {
            return Err(Error::Runtime("cannot send on closed channel".to_string()));
        }

        if self.is_full() {
            debug!("Channel buffer is full, would block");
            return Ok(false); // Would block
        }

        self.buffer.push_back(value);
        debug!(new_len = self.buffer.len(), "Value added to buffer");
        Ok(true)
    }

    /// Try to receive a value without blocking
    #[tracing::instrument(skip(self), fields(buffer_len = self.buffer.len(), closed = self.closed), level = "debug")]
    pub fn try_receive(&mut self) -> Result<Option<Object>, Error> {
        if let Some(value) = self.buffer.pop_front() {
            debug!(new_len = self.buffer.len(), "Value received from buffer");
            return Ok(Some(value));
        }

        if self.closed {
            debug!("Attempted to receive from closed empty channel");
            return Err(Error::Runtime("receive from closed channel".to_string()));
        }

        debug!("Channel buffer is empty, would block");
        Ok(None) // Would block
    }

    /// Close the channel
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn close(&mut self) {
        if !self.closed {
            self.closed = true;
            debug!("Channel closed");
        }
    }

    /// Get the element type
    pub fn element_type(&self) -> &str {
        &self.element_type
    }
}

impl ThreadSafeBufferedChannel {
    /// Create a new thread-safe buffered channel
    pub fn new(element_type: String, capacity: usize) -> Self {
        let inner = Arc::new(Mutex::new(BufferedChannel::new(element_type, capacity)));
        let send_cond = Arc::new(Condvar::new());
        let recv_cond = Arc::new(Condvar::new());

        Self {
            inner,
            send_cond,
            recv_cond,
        }
    }

    /// Send a value to the channel (blocking)
    #[tracing::instrument(skip(self, value), level = "debug")]
    pub fn send(&self, value: Object) -> Result<(), Error> {
        let mut channel = self.inner.lock().unwrap();
        
        // Increment waiter count
        channel.send_waiters += 1;

        // Wait until we can send or the channel is closed
        loop {
            if channel.closed {
                channel.send_waiters -= 1;
                return Err(Error::Runtime("cannot send on closed channel".to_string()));
            }

            if !channel.is_full() {
                channel.buffer.push_back(value);
                channel.send_waiters -= 1;
                debug!(buffer_len = channel.buffer.len(), "Value sent successfully");
                
                // Notify waiting receivers
                self.recv_cond.notify_one();
                return Ok(());
            }

            // Wait for space to become available
            debug!("Waiting for buffer space");
            channel = self.send_cond.wait(channel).unwrap();
        }
    }

    /// Try to send a value without blocking
    pub fn try_send(&self, value: Object) -> Result<bool, Error> {
        let mut channel = self.inner.lock().unwrap();
        let result = channel.try_send(value)?;
        
        if result {
            // Notify waiting receivers
            self.recv_cond.notify_one();
        }
        
        Ok(result)
    }

    /// Send a value with a timeout
    pub fn send_timeout(&self, value: Object, timeout: Duration) -> Result<bool, Error> {
        let start = Instant::now();
        let mut channel = self.inner.lock().unwrap();
        
        channel.send_waiters += 1;

        loop {
            if channel.closed {
                channel.send_waiters -= 1;
                return Err(Error::Runtime("cannot send on closed channel".to_string()));
            }

            if !channel.is_full() {
                channel.buffer.push_back(value);
                channel.send_waiters -= 1;
                self.recv_cond.notify_one();
                return Ok(true);
            }

            let elapsed = start.elapsed();
            if elapsed >= timeout {
                channel.send_waiters -= 1;
                return Ok(false); // Timeout
            }

            let remaining = timeout - elapsed;
            let (guard, timeout_result) = self.send_cond.wait_timeout(channel, remaining).unwrap();
            channel = guard;

            if timeout_result.timed_out() {
                channel.send_waiters -= 1;
                return Ok(false);
            }
        }
    }

    /// Receive a value from the channel (blocking)
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn receive(&self) -> Result<Object, Error> {
        let mut channel = self.inner.lock().unwrap();
        
        channel.recv_waiters += 1;

        loop {
            if let Some(value) = channel.buffer.pop_front() {
                channel.recv_waiters -= 1;
                debug!(buffer_len = channel.buffer.len(), "Value received successfully");
                
                // Notify waiting senders
                self.send_cond.notify_one();
                return Ok(value);
            }

            if channel.closed {
                channel.recv_waiters -= 1;
                return Err(Error::Runtime("receive from closed channel".to_string()));
            }

            // Wait for a value to become available
            debug!("Waiting for value");
            channel = self.recv_cond.wait(channel).unwrap();
        }
    }

    /// Try to receive a value without blocking
    pub fn try_receive(&self) -> Result<Option<Object>, Error> {
        let mut channel = self.inner.lock().unwrap();
        let result = channel.try_receive()?;
        
        if result.is_some() {
            // Notify waiting senders
            self.send_cond.notify_one();
        }
        
        Ok(result)
    }

    /// Receive a value with a timeout
    pub fn receive_timeout(&self, timeout: Duration) -> Result<Option<Object>, Error> {
        let start = Instant::now();
        let mut channel = self.inner.lock().unwrap();
        
        channel.recv_waiters += 1;

        loop {
            if let Some(value) = channel.buffer.pop_front() {
                channel.recv_waiters -= 1;
                self.send_cond.notify_one();
                return Ok(Some(value));
            }

            if channel.closed {
                channel.recv_waiters -= 1;
                return Err(Error::Runtime("receive from closed channel".to_string()));
            }

            let elapsed = start.elapsed();
            if elapsed >= timeout {
                channel.recv_waiters -= 1;
                return Ok(None); // Timeout
            }

            let remaining = timeout - elapsed;
            let (guard, timeout_result) = self.recv_cond.wait_timeout(channel, remaining).unwrap();
            channel = guard;

            if timeout_result.timed_out() {
                channel.recv_waiters -= 1;
                return Ok(None);
            }
        }
    }

    /// Close the channel
    pub fn close(&self) -> Result<(), Error> {
        let mut channel = self.inner.lock().unwrap();
        channel.close();
        
        // Wake up all waiting senders and receivers
        self.send_cond.notify_all();
        self.recv_cond.notify_all();
        
        Ok(())
    }

    /// Get channel statistics
    pub fn stats(&self) -> ChannelStats {
        let channel = self.inner.lock().unwrap();
        ChannelStats {
            element_type: channel.element_type.clone(),
            capacity: channel.capacity,
            current_length: channel.len(),
            is_closed: channel.closed,
            send_waiters: channel.send_waiters,
            recv_waiters: channel.recv_waiters,
            available_space: channel.available_space(),
        }
    }

    /// Check if the channel is closed
    pub fn is_closed(&self) -> bool {
        let channel = self.inner.lock().unwrap();
        channel.closed
    }

    /// Get the capacity
    pub fn capacity(&self) -> usize {
        let channel = self.inner.lock().unwrap();
        channel.capacity
    }

    /// Get current length
    pub fn len(&self) -> usize {
        let channel = self.inner.lock().unwrap();
        channel.len()
    }
}

/// Statistics for a channel
#[derive(Debug, Clone)]
pub struct ChannelStats {
    pub element_type: String,
    pub capacity: usize,
    pub current_length: usize,
    pub is_closed: bool,
    pub send_waiters: usize,
    pub recv_waiters: usize,
    pub available_space: usize,
}

impl Clone for ThreadSafeBufferedChannel {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            send_cond: Arc::clone(&self.send_cond),
            recv_cond: Arc::clone(&self.recv_cond),
        }
    }
}

// Conversion utilities
impl From<BufferedChannel> for Object {
    fn from(channel: BufferedChannel) -> Self {
        Object::Channel(Arc::new(RwLock::new(crate::object::Channel {
            element_type: channel.element_type,
            buffer: channel.buffer.into(),
            buffer_size: channel.capacity,
            closed: channel.closed,
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_buffered_channel_creation() {
        let channel = BufferedChannel::new("int".to_string(), 10);
        assert_eq!(channel.capacity(), 10);
        assert_eq!(channel.len(), 0);
        assert!(!channel.is_closed());
        assert!(channel.is_empty());
        assert!(!channel.is_full());
    }

    #[test]
    fn test_try_send_and_receive() {
        let mut channel = BufferedChannel::new("int".to_string(), 2);
        
        // Send first value
        assert!(channel.try_send(Object::Integer(1)).unwrap());
        assert_eq!(channel.len(), 1);
        
        // Send second value
        assert!(channel.try_send(Object::Integer(2)).unwrap());
        assert_eq!(channel.len(), 2);
        assert!(channel.is_full());
        
        // Try to send third value (should fail)
        assert!(!channel.try_send(Object::Integer(3)).unwrap());
        
        // Receive values
        assert_eq!(channel.try_receive().unwrap().unwrap(), Object::Integer(1));
        assert_eq!(channel.try_receive().unwrap().unwrap(), Object::Integer(2));
        assert!(channel.try_receive().unwrap().is_none());
    }

    #[test]
    fn test_thread_safe_channel() {
        let channel = ThreadSafeBufferedChannel::new("int".to_string(), 3);
        let sent_count = Arc::new(AtomicUsize::new(0));
        let received_count = Arc::new(AtomicUsize::new(0));

        let sender_channel = channel.clone();
        let sender_count = Arc::clone(&sent_count);
        
        let receiver_channel = channel.clone();
        let receiver_count = Arc::clone(&received_count);

        // Sender thread
        let sender = thread::spawn(move || {
            for i in 0..10 {
                sender_channel.send(Object::Integer(i)).unwrap();
                sender_count.fetch_add(1, Ordering::SeqCst);
            }
            sender_channel.close().unwrap();
        });

        // Receiver thread
        let receiver = thread::spawn(move || {
            loop {
                match receiver_channel.try_receive() {
                    Ok(Some(_)) => {
                        receiver_count.fetch_add(1, Ordering::SeqCst);
                    }
                    Ok(None) => {
                        // Would block, try again
                        thread::sleep(Duration::from_millis(1));
                    }
                    Err(_) => {
                        // Channel closed
                        break;
                    }
                }
            }
        });

        sender.join().unwrap();
        receiver.join().unwrap();

        assert_eq!(sent_count.load(Ordering::SeqCst), 10);
        assert_eq!(received_count.load(Ordering::SeqCst), 10);
        assert!(channel.is_closed());
    }

    #[test]
    fn test_blocking_operations() {
        let channel = ThreadSafeBufferedChannel::new("int".to_string(), 1);
        
        // Test that send blocks when buffer is full
        channel.send(Object::Integer(1)).unwrap();
        
        let channel_clone = channel.clone();
        let handle = thread::spawn(move || {
            // This should not block since we'll receive the first value
            thread::sleep(Duration::from_millis(10));
            channel_clone.receive().unwrap()
        });
        
        let received = handle.join().unwrap();
        assert_eq!(received, Object::Integer(1));
    }

    #[test]
    fn test_channel_stats() {
        let channel = ThreadSafeBufferedChannel::new("string".to_string(), 5);
        
        let stats = channel.stats();
        assert_eq!(stats.element_type, "string");
        assert_eq!(stats.capacity, 5);
        assert_eq!(stats.current_length, 0);
        assert!(!stats.is_closed);
        assert_eq!(stats.available_space, 5);
        
        channel.send(Object::String("test".to_string())).unwrap();
        
        let stats = channel.stats();
        assert_eq!(stats.current_length, 1);
        assert_eq!(stats.available_space, 4);
    }
}
