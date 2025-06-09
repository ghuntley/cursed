//! Enhanced channel closing semantics for CURSED language
//!
//! This module provides comprehensive channel closing functionality including:
//! - Thread-safe close operations with proper synchronization
//! - Multiple close protection (prevents panic on duplicate closes)
//! - Proper error handling for operations on closed channels
//! - Memory cleanup and resource management

use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use tracing::{debug, error, info, trace, warn, instrument};

use crate::error::Error;
use crate::object::{Object, Channel};
use crate::object_thread_safe::ThreadSafeObject;
use crate::runtime::channel_gc::ThreadSafeChannel;
use crate::memory::{GarbageCollector, ThreadSafeGc};

/// Channel state management for closing operations
#[derive(Debug)]
pub struct ChannelCloseState {
    /// Atomic flag indicating if the channel is closed
    closed: AtomicBool,
    /// Condition variable for coordinating close operations
    close_signal: Condvar,
    /// Mutex for synchronizing close operations
    close_mutex: Mutex<()>,
    /// Number of pending operations
    pending_operations: AtomicUsize,
}

impl ChannelCloseState {
    /// Create a new channel close state
    pub fn new() -> Self {
        Self {
            closed: AtomicBool::new(false),
            close_signal: Condvar::new(),
            close_mutex: Mutex::new(()),
            pending_operations: AtomicUsize::new(0),
        }
    }

    /// Check if the channel is closed
    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }

    /// Mark the channel as closed, returns true if this call actually closed it
    pub fn close(&self) -> bool {
        // Use compare_and_swap to ensure we only close once
        !self.closed.swap(true, Ordering::AcqRel)
    }

    /// Wait for all pending operations to complete before closing
    pub fn close_gracefully(&self, timeout: Duration) -> Result<bool, Error> {
        let _guard = self.close_mutex.lock().unwrap();
        
        // Mark as closed
        let was_open = self.close();
        
        if was_open {
            // Wait for pending operations to complete
            let start = std::time::Instant::now();
            while self.pending_operations.load(Ordering::Acquire) > 0 {
                if start.elapsed() > timeout {
                    warn!("Timeout waiting for pending channel operations to complete");
                    break;
                }
                std::thread::sleep(Duration::from_millis(1));
            }
            
            // Notify all waiting threads
            self.close_signal.notify_all();
            debug!("Channel closed gracefully");
        }
        
        Ok(was_open)
    }

    /// Increment pending operation counter
    pub fn start_operation(&self) -> Result<(), Error> {
        if self.is_closed() {
            return Err(Error::Runtime("Operation on closed channel".to_string()));
        }
        
        self.pending_operations.fetch_add(1, Ordering::AcqRel);
        
        // Double-check after incrementing
        if self.is_closed() {
            self.pending_operations.fetch_sub(1, Ordering::AcqRel);
            return Err(Error::Runtime("Operation on closed channel".to_string()));
        }
        
        Ok(())
    }

    /// Decrement pending operation counter
    pub fn end_operation(&self) {
        self.pending_operations.fetch_sub(1, Ordering::AcqRel);
    }
}

use std::sync::atomic::AtomicUsize;

/// Enhanced channel implementation with proper closing semantics
#[derive(Debug)]
pub struct EnhancedChannel {
    /// The underlying channel
    inner: Arc<Mutex<Channel>>,
    /// Close state management
    close_state: Arc<ChannelCloseState>,
    /// Element type for type safety
    element_type: String,
    /// Channel capacity
    capacity: usize,
}

impl EnhancedChannel {
    /// Create a new enhanced channel
    #[instrument(fields(element_type = ?element_type, capacity = capacity), level = "debug")]
    pub fn new(element_type: String, capacity: usize) -> Self {
        debug!("Creating enhanced channel with closing semantics");
        
        Self {
            inner: Arc::new(Mutex::new(Channel::new(element_type.clone(), capacity))),
            close_state: Arc::new(ChannelCloseState::new()),
            element_type,
            capacity,
        }
    }

    /// Send a value to the channel with proper close checking
    #[instrument(skip(self, value), fields(element_type = ?self.element_type, capacity = self.capacity), level = "debug")]
    pub fn send(&self, value: Object) -> Result<(), Error> {
        // Start operation tracking
        self.close_state.start_operation()?;
        
        let result = {
            let mut channel = self.inner.lock().unwrap();
            
            // Double-check closed state under lock
            if self.close_state.is_closed() {
                return Err(Error::Runtime("Cannot send on closed channel".to_string()));
            }
            
            channel.send(value)
        };
        
        // End operation tracking
        self.close_state.end_operation();
        
        match result {
            Ok(()) => {
                debug!("Value sent successfully");
                Ok(())
            },
            Err(e) => {
                if self.close_state.is_closed() {
                    error!("Send failed: channel is closed");
                    Err(Error::Runtime("Cannot send on closed channel".to_string()))
                } else {
                    error!(error = ?e, "Send failed");
                    Err(e)
                }
            }
        }
    }

    /// Try to send a value to the channel without blocking
    pub fn try_send(&self, value: Object) -> Result<bool, Error> {
        // Start operation tracking
        self.close_state.start_operation()?;
        
        let result = {
            let mut channel = self.inner.lock().unwrap();
            
            // Double-check closed state under lock
            if self.close_state.is_closed() {
                return Err(Error::Runtime("Cannot send on closed channel".to_string()));
            }
            
            channel.try_send(value)
        };
        
        // End operation tracking
        self.close_state.end_operation();
        
        result
    }

    /// Receive a value from the channel
    #[instrument(skip(self), fields(element_type = ?self.element_type, capacity = self.capacity), level = "debug")]
    pub fn receive(&self) -> Result<(Object, bool), Error> {
        // Start operation tracking - receiving is allowed on closed channels
        // until the buffer is empty
        if self.close_state.is_closed() {
            let channel = self.inner.lock().unwrap();
            if channel.is_empty() {
                // Closed channel with empty buffer: return zero value and closed flag
                let zero_value = self.get_zero_value();
                debug!("Receiving from closed empty channel, returning zero value");
                return Ok((zero_value, true)); // (value, closed)
            }
        } else {
            self.close_state.start_operation()?;
        }
        
        let result = {
            let mut channel = self.inner.lock().unwrap();
            channel.receive()
        };
        
        // End operation tracking if we started it
        if !self.close_state.is_closed() {
            self.close_state.end_operation();
        }
        
        match result {
            Ok(value) => {
                debug!("Value received successfully");
                Ok((value, false)) // (value, not_closed)
            },
            Err(_) => {
                if self.close_state.is_closed() {
                    // Closed channel: return zero value and closed flag
                    let zero_value = self.get_zero_value();
                    debug!("Receiving from closed channel, returning zero value");
                    Ok((zero_value, true)) // (value, closed)
                } else {
                    // Channel not closed, but no value available (would block)
                    Err(Error::Runtime("Receive would block".to_string()))
                }
            }
        }
    }

    /// Try to receive a value from the channel without blocking
    pub fn try_receive(&self) -> Result<Option<(Object, bool)>, Error> {
        let mut channel = self.inner.lock().unwrap();
        
        if channel.is_empty() {
            if self.close_state.is_closed() {
                // Closed and empty: return zero value with closed flag
                let zero_value = self.get_zero_value();
                Ok(Some((zero_value, true)))
            } else {
                // Not closed but empty: would block
                Ok(None)
            }
        } else {
            // Has values: receive normally
            match channel.try_receive() {
                Ok(Some(value)) => Ok(Some((value, false))),
                Ok(None) => Ok(None),
                Err(e) => Err(e),
            }
        }
    }

    /// Close the channel with proper semantics
    #[instrument(skip(self), fields(element_type = ?self.element_type), level = "debug")]
    pub fn close(&self) -> Result<(), Error> {
        let was_open = self.close_state.close();
        
        if was_open {
            // Update the inner channel state
            let mut channel = self.inner.lock().unwrap();
            channel.close();
            debug!("Channel closed successfully");
        } else {
            debug!("Channel was already closed");
        }
        
        Ok(())
    }

    /// Close the channel gracefully with timeout
    pub fn close_gracefully(&self, timeout: Duration) -> Result<(), Error> {
        self.close_state.close_gracefully(timeout)?;
        
        // Update the inner channel state
        let mut channel = self.inner.lock().unwrap();
        channel.close();
        
        Ok(())
    }

    /// Check if the channel is closed
    pub fn is_closed(&self) -> bool {
        self.close_state.is_closed()
    }

    /// Get the zero value for this channel's element type
    fn get_zero_value(&self) -> Object {
        match self.element_type.as_str() {
            "byte" | "normie" | "thicc" => Object::Integer(0),
            "flote" => Object::Float(0.0),
            "bool" => Object::Boolean(false),
            "string" => Object::String(String::new()),
            _ => Object::Null,
        }
    }

    /// Get channel information
    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.inner.lock().unwrap().is_empty()
    }

    pub fn element_type(&self) -> &str {
        &self.element_type
    }
}

/// Enhanced thread-safe channel with closing semantics
#[derive(Debug)]
pub struct EnhancedThreadSafeChannel {
    /// The underlying thread-safe channel
    inner: Arc<ThreadSafeChannel>,
    /// Close state management
    close_state: Arc<ChannelCloseState>,
}

impl EnhancedThreadSafeChannel {
    /// Create a new enhanced thread-safe channel
    #[instrument(fields(element_type = ?element_type, capacity = capacity), level = "debug")]
    pub fn new(element_type: String, capacity: usize) -> Self {
        debug!("Creating enhanced thread-safe channel with closing semantics");
        
        Self {
            inner: Arc::new(ThreadSafeChannel::new(element_type, capacity)),
            close_state: Arc::new(ChannelCloseState::new()),
        }
    }

    /// Send a value to the channel with proper close checking
    pub fn send(&self, value: Object) -> Result<(), Error> {
        self.close_state.start_operation()?;
        
        let result = self.inner.send(value);
        self.close_state.end_operation();
        
        result
    }

    /// Try to send a value without blocking
    pub fn try_send(&self, value: Object) -> Result<bool, Error> {
        self.close_state.start_operation()?;
        
        let result = self.inner.try_send(value);
        self.close_state.end_operation();
        
        result
    }

    /// Receive a value from the channel
    pub fn receive(&self) -> Result<(Object, bool), Error> {
        match self.inner.receive() {
            Ok(value) => Ok((value, false)),
            Err(_) => {
                if self.close_state.is_closed() {
                    // Return zero value for closed channel
                    let zero_value = self.get_zero_value();
                    Ok((zero_value, true))
                } else {
                    Err(Error::Runtime("Receive would block".to_string()))
                }
            }
        }
    }

    /// Try to receive a value without blocking
    pub fn try_receive(&self) -> Result<Option<(Object, bool)>, Error> {
        match self.inner.try_receive() {
            Ok(Some(value)) => Ok(Some((value, false))),
            Ok(None) => {
                if self.close_state.is_closed() {
                    let zero_value = self.get_zero_value();
                    Ok(Some((zero_value, true)))
                } else {
                    Ok(None)
                }
            },
            Err(e) => Err(e),
        }
    }

    /// Close the channel
    pub fn close(&self) -> Result<(), Error> {
        let was_open = self.close_state.close();
        
        if was_open {
            self.inner.close();
            debug!("Thread-safe channel closed successfully");
        }
        
        Ok(())
    }

    /// Close gracefully with timeout
    pub fn close_gracefully(&self, timeout: Duration) -> Result<(), Error> {
        self.close_state.close_gracefully(timeout)?;
        self.inner.close();
        Ok(())
    }

    /// Check if closed
    pub fn is_closed(&self) -> bool {
        self.close_state.is_closed()
    }

    /// Get zero value for the element type
    fn get_zero_value(&self) -> Object {
        match self.inner.element_type() {
            "byte" | "normie" | "thicc" => Object::Integer(0),
            "flote" => Object::Float(0.0),
            "bool" => Object::Boolean(false),
            "string" => Object::String(String::new()),
            _ => Object::Null,
        }
    }

    /// Get channel information
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn element_type(&self) -> &str {
        self.inner.element_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_channel_close_semantics() {
        let channel = EnhancedChannel::new("normie".to_string(), 2);
        
        // Send some values
        assert!(channel.send(Object::Integer(1)).is_ok());
        assert!(channel.send(Object::Integer(2)).is_ok());
        
        // Close the channel
        assert!(channel.close().is_ok());
        assert!(channel.is_closed());
        
        // Sending should fail
        assert!(channel.send(Object::Integer(3)).is_err());
        
        // But receiving should work until buffer is empty
        let (val1, closed1) = channel.receive().unwrap();
        assert_eq!(val1, Object::Integer(1));
        assert!(!closed1);
        
        let (val2, closed2) = channel.receive().unwrap();
        assert_eq!(val2, Object::Integer(2));
        assert!(!closed2);
        
        // Now buffer is empty, should get zero value with closed flag
        let (zero_val, closed3) = channel.receive().unwrap();
        assert_eq!(zero_val, Object::Integer(0));
        assert!(closed3);
    }

    #[test]
    fn test_multiple_close_protection() {
        let channel = EnhancedChannel::new("normie".to_string(), 1);
        
        // First close should succeed
        assert!(channel.close().is_ok());
        assert!(channel.is_closed());
        
        // Subsequent closes should not panic
        assert!(channel.close().is_ok());
        assert!(channel.close().is_ok());
    }

    #[test]
    fn test_thread_safe_channel_close() {
        let channel = EnhancedThreadSafeChannel::new("normie".to_string(), 1);
        
        assert!(channel.send(Object::Integer(42)).is_ok());
        assert!(channel.close().is_ok());
        
        // Send should fail
        assert!(channel.send(Object::Integer(43)).is_err());
        
        // Receive should work
        let (val, closed) = channel.receive().unwrap();
        assert_eq!(val, Object::Integer(42));
        assert!(!closed);
        
        // Second receive should return zero value
        let (zero_val, closed2) = channel.receive().unwrap();
        assert_eq!(zero_val, Object::Integer(0));
        assert!(closed2);
    }
}
