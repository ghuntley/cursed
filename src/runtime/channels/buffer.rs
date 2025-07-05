//! Channel buffering strategies and implementations
//!
//! Provides different buffer types for channels:
//! - Unbuffered (synchronous) channels
//! - Fixed-size ring buffers
//! - Dynamically growing buffers
//! - Memory-efficient sparse buffers

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr::NonNull;
use std::mem::{self, MaybeUninit};
use std::alloc::{self, Layout};

use crate::runtime::channels::ChannelError;

/// Buffer type for channels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    /// Unbuffered channel (synchronous)
    Unbuffered,
    /// Fixed-size ring buffer
    Fixed(usize),
    /// Dynamically growing buffer
    Dynamic { initial: usize, max: Option<usize> },
    /// Memory-efficient sparse buffer
    Sparse(usize),
}

/// Channel buffer trait
pub trait ChannelBuffer<T>: Send + Sync {
    /// Try to push a value into the buffer
    fn try_push(&self, value: T) -> Result<(), (T, ChannelError)>;
    
    /// Try to pop a value from the buffer
    fn try_pop(&self) -> Result<Option<T>, ChannelError>;
    
    /// Get the current length of the buffer
    fn len(&self) -> usize;
    
    /// Get the capacity of the buffer
    fn capacity(&self) -> usize;
    
    /// Check if the buffer is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Check if the buffer is full
    fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }
    
    /// Close the buffer
    fn close(&self);
    
    /// Check if the buffer is closed
    fn is_closed(&self) -> bool;
}

/// Unbuffered channel implementation (synchronous)
pub struct UnbufferedChannel<T: Send> {
    /// Pending value from sender
    pending_value: Mutex<Option<T>>,
    /// Sender waiting condition
    sender_waiting: Condvar,
    /// Receiver waiting condition
    receiver_waiting: Condvar,
    /// Number of waiting senders
    pending_senders: AtomicUsize,
    /// Number of waiting receivers
    pending_receivers: AtomicUsize,
    /// Channel closed flag
    closed: Mutex<bool>,
}

impl<T: Send> UnbufferedChannel<T> {
    pub fn new() -> Self {
        Self {
            pending_value: Mutex::new(None),
            sender_waiting: Condvar::new(),
            receiver_waiting: Condvar::new(),
            pending_senders: AtomicUsize::new(0),
            pending_receivers: AtomicUsize::new(0),
            closed: Mutex::new(false),
        }
    }
    
    /// Send a value (blocking)
    pub fn send(&self, value: T) -> Result<(), (T, ChannelError)> {
        let mut pending = self.pending_value.lock().unwrap();
        
        // Check if closed
        if *self.closed.lock().unwrap() {
            return Err((value, ChannelError::Closed));
        }
        
        // Wait for a receiver
        self.pending_senders.fetch_add(1, Ordering::SeqCst);
        
        while pending.is_some() && !*self.closed.lock().unwrap() {
            pending = self.sender_waiting.wait(pending).unwrap();
        }
        
        self.pending_senders.fetch_sub(1, Ordering::SeqCst);
        
        if *self.closed.lock().unwrap() {
            return Err((value, ChannelError::Closed));
        }
        
        // Place value and notify receiver
        *pending = Some(value);
        self.receiver_waiting.notify_one();
        
        Ok(())
    }
    
    /// Receive a value (blocking)
    pub fn recv(&self) -> Result<T, ChannelError> {
        let mut pending = self.pending_value.lock().unwrap();
        
        self.pending_receivers.fetch_add(1, Ordering::SeqCst);
        
        // Wait for a value
        while pending.is_none() && !*self.closed.lock().unwrap() {
            pending = self.receiver_waiting.wait(pending).unwrap();
        }
        
        self.pending_receivers.fetch_sub(1, Ordering::SeqCst);
        
        if let Some(value) = pending.take() {
            // Notify sender that value was taken
            self.sender_waiting.notify_one();
            Ok(value)
        } else {
            Err(ChannelError::Closed)
        }
    }
}

impl<T: Send> ChannelBuffer<T> for UnbufferedChannel<T> {
    fn try_push(&self, value: T) -> Result<(), (T, ChannelError)> {
        let mut pending = self.pending_value.lock().unwrap();
        
        if *self.closed.lock().unwrap() {
            return Err((value, ChannelError::Closed));
        }
        
        // For unbuffered channels, we can only push if there's no pending value
        if pending.is_some() {
            return Err((value, ChannelError::WouldBlock));
        }
        
        *pending = Some(value);
        self.receiver_waiting.notify_one();
        Ok(())
    }
    
    fn try_pop(&self) -> Result<Option<T>, ChannelError> {
        let mut pending = self.pending_value.lock().unwrap();
        
        if let Some(value) = pending.take() {
            self.sender_waiting.notify_one();
            Ok(Some(value))
        } else if *self.closed.lock().unwrap() {
            Err(ChannelError::Closed)
        } else {
            Ok(None)
        }
    }
    
    fn len(&self) -> usize {
        if self.pending_value.lock().unwrap().is_some() {
            1
        } else {
            0
        }
    }
    
    fn capacity(&self) -> usize {
        0 // Unbuffered
    }
    
    fn close(&self) {
        *self.closed.lock().unwrap() = true;
        self.sender_waiting.notify_all();
        self.receiver_waiting.notify_all();
    }
    
    fn is_closed(&self) -> bool {
        *self.closed.lock().unwrap()
    }
}

/// Ring buffer implementation for fixed-size channels
pub struct RingBuffer<T: Send> {
    /// Buffer data
    buffer: Mutex<RingBufferData<T>>,
    /// Not empty condition
    not_empty: Condvar,
    /// Not full condition
    not_full: Condvar,
    /// Channel closed flag
    closed: Mutex<bool>,
}

struct RingBufferData<T: Send> {
    /// Underlying storage
    data: Vec<MaybeUninit<T>>,
    /// Write position
    write_pos: usize,
    /// Read position
    read_pos: usize,
    /// Current length
    len: usize,
    /// Buffer capacity
    capacity: usize,
}

impl<T: Send> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        // Initialize with uninitialized memory
        for _ in 0..capacity {
            data.push(MaybeUninit::uninit());
        }
        
        Self {
            buffer: Mutex::new(RingBufferData {
                data,
                write_pos: 0,
                read_pos: 0,
                len: 0,
                capacity,
            }),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
            closed: Mutex::new(false),
        }
    }
    
    /// Send a value (blocking)
    pub fn send(&self, value: T) -> Result<(), (T, ChannelError)> {
        let mut buffer = self.buffer.lock().unwrap();
        
        // Wait for space
        while buffer.len == buffer.capacity && !*self.closed.lock().unwrap() {
            buffer = self.not_full.wait(buffer).unwrap();
        }
        
        if *self.closed.lock().unwrap() {
            return Err((value, ChannelError::Closed));
        }
        
        // Write value
        let write_pos = buffer.write_pos;
        let capacity = buffer.capacity;
        buffer.data[write_pos] = MaybeUninit::new(value);
        buffer.write_pos = (write_pos + 1) % capacity;
        buffer.len += 1;
        
        // Notify receivers
        self.not_empty.notify_one();
        
        Ok(())
    }
    
    /// Receive a value (blocking)
    pub fn recv(&self) -> Result<T, ChannelError> {
        let mut buffer = self.buffer.lock().unwrap();
        
        // Wait for data
        while buffer.len == 0 && !*self.closed.lock().unwrap() {
            buffer = self.not_empty.wait(buffer).unwrap();
        }
        
        if buffer.len == 0 {
            return Err(ChannelError::Closed);
        }
        
        // Read value
        let value = unsafe {
            std::ptr::read(buffer.data[buffer.read_pos].as_ptr())
        };
        buffer.read_pos = (buffer.read_pos + 1) % buffer.capacity;
        buffer.len -= 1;
        
        // Notify senders
        self.not_full.notify_one();
        
        Ok(value)
    }
}

impl<T: Send> ChannelBuffer<T> for RingBuffer<T> {
    fn try_push(&self, value: T) -> Result<(), (T, ChannelError)> {
        let mut buffer = self.buffer.lock().unwrap();
        
        if *self.closed.lock().unwrap() {
            return Err((value, ChannelError::Closed));
        }
        
        if buffer.len == buffer.capacity {
            return Err((value, ChannelError::BufferFull));
        }
        
        // Write value
        let write_pos = buffer.write_pos;
        let capacity = buffer.capacity;
        buffer.data[write_pos] = MaybeUninit::new(value);
        buffer.write_pos = (write_pos + 1) % capacity;
        buffer.len += 1;
        
        // Notify receivers
        self.not_empty.notify_one();
        
        Ok(())
    }
    
    fn try_pop(&self) -> Result<Option<T>, ChannelError> {
        let mut buffer = self.buffer.lock().unwrap();
        
        if buffer.len == 0 {
            if *self.closed.lock().unwrap() {
                return Err(ChannelError::Closed);
            } else {
                return Ok(None);
            }
        }
        
        // Read value
        let value = unsafe {
            std::ptr::read(buffer.data[buffer.read_pos].as_ptr())
        };
        buffer.read_pos = (buffer.read_pos + 1) % buffer.capacity;
        buffer.len -= 1;
        
        // Notify senders
        self.not_full.notify_one();
        
        Ok(Some(value))
    }
    
    fn len(&self) -> usize {
        self.buffer.lock().unwrap().len
    }
    
    fn capacity(&self) -> usize {
        self.buffer.lock().unwrap().capacity
    }
    
    fn close(&self) {
        *self.closed.lock().unwrap() = true;
        self.not_empty.notify_all();
        self.not_full.notify_all();
    }
    
    fn is_closed(&self) -> bool {
        *self.closed.lock().unwrap()
    }
}

/// Dynamic buffer that can grow as needed
pub struct DynamicBuffer<T: Send> {
    /// Buffer data
    buffer: Mutex<VecDeque<T>>,
    /// Not empty condition
    not_empty: Condvar,
    /// Maximum capacity
    max_capacity: Option<usize>,
    /// Channel closed flag
    closed: Mutex<bool>,
}

impl<T: Send> DynamicBuffer<T> {
    pub fn new(initial_capacity: usize, max_capacity: Option<usize>) -> Self {
        Self {
            buffer: Mutex::new(VecDeque::with_capacity(initial_capacity)),
            not_empty: Condvar::new(),
            max_capacity,
            closed: Mutex::new(false),
        }
    }
}

impl<T: Send> ChannelBuffer<T> for DynamicBuffer<T> {
    fn try_push(&self, value: T) -> Result<(), (T, ChannelError)> {
        let mut buffer = self.buffer.lock().unwrap();
        
        if *self.closed.lock().unwrap() {
            return Err((value, ChannelError::Closed));
        }
        
        // Check capacity limit
        if let Some(max_cap) = self.max_capacity {
            if buffer.len() >= max_cap {
                return Err((value, ChannelError::BufferFull));
            }
        }
        
        buffer.push_back(value);
        self.not_empty.notify_one();
        
        Ok(())
    }
    
    fn try_pop(&self) -> Result<Option<T>, ChannelError> {
        let mut buffer = self.buffer.lock().unwrap();
        
        if let Some(value) = buffer.pop_front() {
            Ok(Some(value))
        } else if *self.closed.lock().unwrap() {
            Err(ChannelError::Closed)
        } else {
            Ok(None)
        }
    }
    
    fn len(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }
    
    fn capacity(&self) -> usize {
        self.max_capacity.unwrap_or(usize::MAX)
    }
    
    fn close(&self) {
        *self.closed.lock().unwrap() = true;
        self.not_empty.notify_all();
    }
    
    fn is_closed(&self) -> bool {
        *self.closed.lock().unwrap()
    }
}

/// Create a buffer based on the buffer type
pub fn create_buffer<T: Send + 'static>(buffer_type: BufferType) -> Box<dyn ChannelBuffer<T>> {
    match buffer_type {
        BufferType::Unbuffered => Box::new(UnbufferedChannel::new()),
        BufferType::Fixed(capacity) => Box::new(RingBuffer::new(capacity)),
        BufferType::Dynamic { initial, max } => Box::new(DynamicBuffer::new(initial, max)),
        BufferType::Sparse(capacity) => {
            // For now, use dynamic buffer for sparse
            Box::new(DynamicBuffer::new(capacity, Some(capacity)))
        }
    }
}

// Drop implementation for RingBuffer to properly clean up uninitialized memory
impl<T: Send> Drop for RingBufferData<T> {
    fn drop(&mut self) {
        // Drop any initialized values
        let mut read_pos = self.read_pos;
        for _ in 0..self.len {
            unsafe {
                std::ptr::drop_in_place(self.data[read_pos].as_mut_ptr());
            }
            read_pos = (read_pos + 1) % self.capacity;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_unbuffered_channel() {
        let channel = UnbufferedChannel::new();
        
        // Should be able to send one value to unbuffered channel
        assert!(channel.try_push(42).is_ok());
        
        // Should be able to receive the value
        assert_eq!(channel.try_pop().unwrap(), Some(42));
        
        // Should not be able to receive when empty
        assert!(channel.try_pop().unwrap().is_none());
    }

    #[test]
    fn test_ring_buffer() {
        let buffer = RingBuffer::new(3);
        
        // Should be able to send up to capacity
        assert!(buffer.try_push(1).is_ok());
        assert!(buffer.try_push(2).is_ok());
        assert!(buffer.try_push(3).is_ok());
        
        // Should be full now
        assert!(buffer.try_push(4).is_err());
        
        // Should be able to receive
        assert_eq!(buffer.try_pop().unwrap().unwrap(), 1);
        assert_eq!(buffer.try_pop().unwrap().unwrap(), 2);
        assert_eq!(buffer.try_pop().unwrap().unwrap(), 3);
        
        // Should be empty now
        assert!(buffer.try_pop().unwrap().is_none());
    }

    #[test]
    fn test_dynamic_buffer() {
        let buffer = DynamicBuffer::new(2, Some(5));
        
        // Should be able to send multiple values
        for i in 0..5 {
            assert!(buffer.try_push(i).is_ok());
        }
        
        // Should be at capacity
        assert!(buffer.try_push(5).is_err());
        
        // Should be able to receive all values
        for i in 0..5 {
            assert_eq!(buffer.try_pop().unwrap().unwrap(), i);
        }
    }
}
