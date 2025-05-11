//! Thread-safe channel implementation with garbage collection support
//!
//! This module provides a thread-safe channel implementation that works
//! with the concurrent garbage collector. It ensures proper tracing of
//! objects stored in channels and prevents memory leaks.

use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;
use std::time::Duration;

use tracing::{debug, error, info, trace, instrument};

use crate::error::Error;
use crate::object::Object;
use crate::memory::{Traceable, Tag, Visitor};

/// A thread-safe channel with garbage collection support
#[derive(Debug)]
pub struct ThreadSafeChannel {
    /// The type of elements in the channel
    element_type: String,
    /// The buffer capacity (0 for unbuffered)
    capacity: usize,
    /// The buffer of elements
    buffer: Arc<Mutex<VecDeque<Object>>>,
    /// Condvar for coordinating sends
    send_signal: Arc<Condvar>,
    /// Condvar for coordinating receives
    recv_signal: Arc<Condvar>,
    /// Whether the channel is closed
    closed: Arc<Mutex<bool>>,
}

impl ThreadSafeChannel {
    /// Create a new thread-safe channel
    #[instrument(fields(element_type = ?element_type, capacity = capacity), level = "debug")]
    pub fn new(element_type: String, capacity: usize) -> Self {
        debug!("Creating new thread-safe channel");
        Self {
            element_type,
            capacity,
            buffer: Arc::new(Mutex::new(VecDeque::with_capacity(capacity.max(1)))),
            send_signal: Arc::new(Condvar::new()),
            recv_signal: Arc::new(Condvar::new()),
            closed: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Send a value to the channel
    #[instrument(skip(self, value), fields(element_type = ?self.element_type, capacity = self.capacity), level = "debug")]
    pub fn send(&self, value: Object) -> Result<(), Error> {
        // First check if the channel is closed
        if *self.closed.lock().unwrap() {
            return Err(Error::Runtime("Cannot send on closed channel".to_string()));
        }
        
        let mut buffer = self.buffer.lock().unwrap();
        
        // For unbuffered channels, wait for a receiver
        if self.capacity == 0 {
            // Wait until the buffer is empty (previous value received)
            // or the channel is closed
            while !buffer.is_empty() && !*self.closed.lock().unwrap() {
                buffer = self.send_signal.wait(buffer).unwrap();
            }
            
            // Check again if the channel was closed while waiting
            if *self.closed.lock().unwrap() {
                return Err(Error::Runtime("Cannot send on closed channel".to_string()));
            }
        } else {
            // For buffered channels, wait until there's space
            while buffer.len() >= self.capacity && !*self.closed.lock().unwrap() {
                buffer = self.send_signal.wait(buffer).unwrap();
            }
            
            // Check again if the channel was closed while waiting
            if *self.closed.lock().unwrap() {
                return Err(Error::Runtime("Cannot send on closed channel".to_string()));
            }
        }
        
        // Add the value to the buffer
        buffer.push_back(value);
        
        // Notify any waiting receivers
        self.recv_signal.notify_one();
        
        Ok(())
    }
    
    /// Try to send a value to the channel without blocking
    pub fn try_send(&self, value: Object) -> Result<bool, Error> {
        // First check if the channel is closed
        if *self.closed.lock().unwrap() {
            return Err(Error::Runtime("Cannot send on closed channel".to_string()));
        }
        
        let mut buffer = self.buffer.lock().unwrap();
        
        // For unbuffered channels, only send if the buffer is empty
        if self.capacity == 0 {
            if !buffer.is_empty() {
                return Ok(false); // Would block
            }
        } else {
            // For buffered channels, only send if there's space
            if buffer.len() >= self.capacity {
                return Ok(false); // Would block
            }
        }
        
        // Add the value to the buffer
        buffer.push_back(value);
        
        // Notify any waiting receivers
        self.recv_signal.notify_one();
        
        Ok(true)
    }
    
    /// Receive a value from the channel
    pub fn receive(&self) -> Result<Object, Error> {
        let mut buffer = self.buffer.lock().unwrap();
        
        // Wait until there's a value to receive or the channel is closed
        while buffer.is_empty() && !*self.closed.lock().unwrap() {
            buffer = self.recv_signal.wait(buffer).unwrap();
        }
        
        // Check if the channel is closed and the buffer is empty
        if buffer.is_empty() && *self.closed.lock().unwrap() {
            return Err(Error::Runtime("Channel closed".to_string()));
        }
        
        // Get the value from the buffer
        let value = buffer.pop_front().unwrap();
        
        // Notify any waiting senders
        self.send_signal.notify_one();
        
        Ok(value)
    }
    
    /// Try to receive a value from the channel without blocking
    pub fn try_receive(&self) -> Result<Option<Object>, Error> {
        let mut buffer = self.buffer.lock().unwrap();
        
        // If the buffer is empty, check if the channel is closed
        if buffer.is_empty() {
            if *self.closed.lock().unwrap() {
                return Err(Error::Runtime("Channel closed".to_string()));
            } else {
                return Ok(None); // Would block
            }
        }
        
        // Get the value from the buffer
        let value = buffer.pop_front().unwrap();
        
        // Notify any waiting senders
        self.send_signal.notify_one();
        
        Ok(Some(value))
    }
    
    /// Close the channel
    pub fn close(&self) {
        // Set the closed flag
        *self.closed.lock().unwrap() = true;
        
        // Notify all waiting goroutines
        self.send_signal.notify_all();
        self.recv_signal.notify_all();
    }
    
    /// Check if the channel is closed
    pub fn is_closed(&self) -> bool {
        *self.closed.lock().unwrap()
    }
    
    /// Get the number of elements in the channel buffer
    pub fn len(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }
    
    /// Check if the channel is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.lock().unwrap().is_empty()
    }
    
    /// Get the capacity of the channel
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get the element type of the channel
    pub fn element_type(&self) -> &str {
        &self.element_type
    }
}

impl Traceable for ThreadSafeChannel {
    /// Trace objects stored in the channel buffer
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Acquire a lock on the buffer
        if let Ok(buffer) = self.buffer.lock() {
            // Trace each object in the buffer
            for obj in buffer.iter() {
                // For each object type that might contain references to other objects
                match obj {
                    Object::Array(elements) => {
                        for elem in elements {
                            visitor.visit_object(elem);
                        }
                    },
                    Object::HashTable(map) => {
                        for (_, value) in map {
                            visitor.visit_object(value);
                        }
                    },
                    Object::Channel(channel) => {
                        // For channels, we need to properly mark the channel object
                        visitor.visit_object(obj);
                    },
                    Object::Reference(ref_obj) => {
                        visitor.visit_object(&*ref_obj.borrow());
                    },
                    Object::Instance { fields, .. } => {
                        for (_, value) in fields {
                            visitor.visit_object(value);
                        }
                    },
                    // Handle other object types that might contain references...
                    _ => {}
                }
            }
        }
    }
    
    fn size(&self) -> usize {
        // Estimate the size of the channel object itself
        let base_size = std::mem::size_of::<Self>();
        
        // Add the size of objects in the buffer
        if let Ok(buffer) = self.buffer.lock() {
            base_size + buffer.iter().map(|obj| obj.size_estimate()).sum::<usize>()
        } else {
            base_size
        }
    }
    
    fn tag(&self) -> Tag {
        Tag::Channel
    }
}

// These impls are required for thread-safe usage
unsafe impl Send for ThreadSafeChannel {}
unsafe impl Sync for ThreadSafeChannel {}