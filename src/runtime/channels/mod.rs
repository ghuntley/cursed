// Channel System for CURSED Runtime
//
// This module provides Go-style channels for communication between goroutines:
// - Unbuffered (synchronous) channels
// - Buffered (asynchronous) channels
// - Channel operations (send, receive, select)
// - Channel closing and error handling
// - Memory management for channel buffers

// Simple working channel implementation (main interface)
pub mod simple_channel;

// Advanced channel implementation modules (work in progress)
#[allow(dead_code)]
pub mod buffer;
#[allow(dead_code)]
pub mod select;
#[allow(dead_code)]
pub mod select_runtime;
#[allow(dead_code)]
pub mod channel;
#[allow(dead_code)]
pub mod operations;
#[allow(dead_code)]
pub mod memory;
#[allow(dead_code)]
pub mod sync;
// pub mod advanced_channel;
// pub mod enhanced_select;
// pub mod simple_advanced_channel;

// Enhanced select for simple channels
pub mod enhanced_select_simple;

// Production-grade channel implementation
pub mod production_channel;

// Channel direction enforcement
pub mod channel_direction;

// Enhanced select with timeout patterns
pub mod select_timeout;

// Channel lifecycle management
pub mod lifecycle;

// Timeout manager for reliable timeout handling
pub mod timeout_manager;

// Channel lifecycle tests
#[cfg(test)]
pub mod lifecycle_test;

// Re-export the simple implementation as the main interface
pub use simple_channel::{
    SimpleChannel,
    SimpleChannelSender,
    SimpleChannelReceiver,
    simple_channel,
    simple_buffered_channel,
};

// Re-export enhanced select
pub use enhanced_select_simple::{
    SimpleSelect,
    SelectResult,
    SelectCase,
    MixedSelectBuilder,
    select_receive,
    select_send,
};

// Re-export production channel
pub use production_channel::{
    ProductionChannel,
    ProductionChannelSender,
    ProductionChannelReceiver,
    ChannelConfig,
    ChannelPriority,
    PriorityMessage,
    ProductionChannelStats,
    production_channel,
    production_channel_default,
    production_channel_with_capacity,
    production_channel_unbounded,
};

// Re-export lifecycle management
pub use lifecycle::{
    ChannelLifecycleManager,
    ChannelLifecycleStats,
    ChannelResourceLimits,
    ChannelEvent,
    ChannelInfo,
    get_global_channel_manager,
    init_global_channel_manager,
};

// Main channel API using simple implementation with lifecycle management
pub type Channel<T> = SimpleChannel<T>;
pub type ChannelSender<T> = SimpleChannelSender<T>;
pub type ChannelReceiver<T> = SimpleChannelReceiver<T>;

pub fn channel<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    let (sender, receiver) = simple_channel();
    
    // Register with lifecycle manager
    let manager = lifecycle::get_global_channel_manager();
    let type_name = std::any::type_name::<T>().to_string();
    
    if let Ok(id) = manager.register_channel(type_name, 0) {
        // Store channel ID for later cleanup (would need to modify SimpleChannel)
        // For now, we'll just track the creation
        let _ = id;
    }
    
    (sender, receiver)
}

pub fn buffered_channel<T>(capacity: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    let (sender, receiver) = simple_buffered_channel(capacity);
    
    // Register with lifecycle manager
    let manager = lifecycle::get_global_channel_manager();
    let type_name = std::any::type_name::<T>().to_string();
    
    if let Ok(id) = manager.register_channel(type_name, capacity) {
        // Store channel ID for later cleanup (would need to modify SimpleChannel)
        // For now, we'll just track the creation
        let _ = id;
    }
    
    (sender, receiver)
}

/// Create a channel with lifecycle management
pub fn managed_channel<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    channel()
}

/// Create a buffered channel with lifecycle management
pub fn managed_buffered_channel<T>(capacity: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    buffered_channel(capacity)
}

// CURSED syntax support
pub type Dm<T> = Channel<T>;
pub fn dm<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    channel()
}
pub fn dm_buffered<T>(capacity: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    buffered_channel(capacity)
}

use std::fmt;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

/// Channel error types
#[derive(Debug, Clone)]
pub enum ChannelError {
    /// Channel is closed
    Closed,
    /// Send would block (for non-blocking operations)
    WouldBlock,
    /// Buffer is full
    BufferFull,
    /// No senders available
    NoSenders,
    /// No receivers available
    NoReceivers,
    /// Operation timeout
    Timeout,
    /// Channel not found
    ChannelNotFound,
    /// Invalid buffer size
    InvalidBufferSize(usize),
    /// Memory allocation error
    AllocationError(String),
}

impl fmt::Display for ChannelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChannelError::Closed => write!(f, "Channel is closed"),
            ChannelError::WouldBlock => write!(f, "Operation would block"),
            ChannelError::BufferFull => write!(f, "Channel buffer is full"),
            ChannelError::NoSenders => write!(f, "No senders available"),
            ChannelError::NoReceivers => write!(f, "No receivers available"),
            ChannelError::Timeout => write!(f, "Operation timed out"),
            ChannelError::ChannelNotFound => write!(f, "Channel not found"),
            ChannelError::InvalidBufferSize(size) => write!(f, "Invalid buffer size: {}", size),
            ChannelError::AllocationError(msg) => write!(f, "Memory allocation error: {}", msg),
        }
    }
}

impl std::error::Error for ChannelError {}

/// Result type for channel operations
pub type ChannelResult<T> = Result<T, ChannelError>;

/// Result of a send operation
#[derive(Debug, Clone)]
pub enum SendResult<T> {
    /// Value was sent successfully
    Sent,
    /// Channel is closed, value returned
    Closed(T),
    /// Send would block (non-blocking mode)
    WouldBlock(T),
}

impl<T> SendResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            SendResult::Sent => {
                // This is not a valid unwrap for Sent variant - return a default or handle differently
                // For now, we'll provide a better error message
                unreachable!("SendResult::Sent has no value to unwrap - use is_ok() to check success")
            },
            SendResult::Closed(t) => t,
            SendResult::WouldBlock(t) => t,
        }
    }
    
    pub fn unwrap_value(self) -> Option<T> {
        match self {
            SendResult::Sent => None,
            SendResult::Closed(t) => Some(t),
            SendResult::WouldBlock(t) => Some(t),
        }
    }
    
    pub fn is_ok(&self) -> bool {
        matches!(self, SendResult::Sent)
    }
    
    pub fn is_err(&self) -> bool {
        matches!(self, SendResult::Closed(_) | SendResult::WouldBlock(_))
    }
}

/// Result of a receive operation
#[derive(Debug, Clone)]
pub enum ReceiveResult<T> {
    /// Value was received successfully
    Received(T),
    /// Channel is closed, no more values
    Closed,
    /// Receive would block (non-blocking mode)
    WouldBlock,
}

/// Channel statistics
#[derive(Debug, Clone)]
pub struct ChannelStats {
    /// Channel ID
    pub id: usize,
    /// Channel capacity
    pub capacity: usize,
    /// Current number of buffered messages
    pub current_length: usize,
    /// Number of active senders
    pub sender_count: usize,
    /// Number of active receivers
    pub receiver_count: usize,
    /// Whether channel is closed
    pub is_closed: bool,
    /// Total messages sent
    pub total_sent: u64,
    /// Total messages received
    pub total_received: u64,
    /// Messages dropped due to overflow
    pub messages_dropped: u64,
}

impl<T> ReceiveResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            ReceiveResult::Received(t) => t,
            ReceiveResult::Closed => {
                unreachable!("ReceiveResult::Closed has no value to unwrap - use is_ok() to check success")
            },
            ReceiveResult::WouldBlock => {
                unreachable!("ReceiveResult::WouldBlock has no value to unwrap - use is_ok() to check success")
            },
        }
    }
    
    pub fn unwrap_or_default(self) -> Option<T> {
        match self {
            ReceiveResult::Received(t) => Some(t),
            ReceiveResult::Closed => None,
            ReceiveResult::WouldBlock => None,
        }
    }
    
    pub fn is_ok(&self) -> bool {
        matches!(self, ReceiveResult::Received(_))
    }
}

// Legacy compatibility wrapper - keeping old interface for backward compatibility
// The new full-featured implementation is in the channel module

/// Legacy channel wrapper - redirects to new implementation
#[deprecated(note = "Use the new Channel type from channel module")]
pub struct LegacyChannel<T: Send + 'static> {
    #[allow(deprecated)]
    inner: channel::Channel<T>,
}

#[allow(deprecated)]
impl<T: Send + 'static> LegacyChannel<T> {
    /// Create a new unbuffered channel
    pub fn new() -> Self {
        Self {
            inner: channel::Channel::new(),
        }
    }

    /// Create a new buffered channel with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: channel::Channel::with_capacity(capacity),
        }
    }

    /// Send a value into the channel
    pub fn send(&self, value: T) -> SendResult<T> {
        self.inner.send(value)
    }

    /// Try to receive a value from the channel (non-blocking)
    pub fn try_recv(&self) -> ReceiveResult<T> {
        self.inner.try_recv()
    }

    /// Receive a value from the channel (blocking)
    pub fn recv(&self) -> ReceiveResult<T> {
        self.inner.recv()
    }

    /// Close the channel
    pub fn close(&self) {
        self.inner.close();
    }

    /// Check if channel is closed
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// Get buffer size
    pub fn buffer_size(&self) -> usize {
        self.inner.capacity()
    }
}

#[allow(deprecated)]
impl<T: Send + 'static> Default for LegacyChannel<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy channel sender handle - redirects to new implementation
#[deprecated(note = "Use ChannelSender from channel module")]
pub struct LegacyChannelSender<T: Send + 'static> {
    #[allow(deprecated)]
    inner: channel::ChannelSender<T>,
}

#[allow(deprecated)]
impl<T: Send + 'static> LegacyChannelSender<T> {
    pub fn new(inner: channel::ChannelSender<T>) -> Self {
        Self { inner }
    }

    pub fn send(&self, value: T) -> SendResult<T> {
        self.inner.send(value)
    }
}

/// Legacy channel receiver handle - redirects to new implementation
#[deprecated(note = "Use ChannelReceiver from channel module")]
pub struct LegacyChannelReceiver<T: Send + 'static> {
    #[allow(deprecated)]
    inner: channel::ChannelReceiver<T>,
}

#[allow(deprecated)]
impl<T: Send + 'static> LegacyChannelReceiver<T> {
    pub fn new(inner: channel::ChannelReceiver<T>) -> Self {
        Self { inner }
    }

    pub fn recv(&self) -> ReceiveResult<T> {
        self.inner.recv()
    }

    pub fn try_recv(&self) -> ReceiveResult<T> {
        self.inner.try_recv()
    }
}

/// Legacy channel creation function - redirects to new implementation
#[deprecated(note = "Use channel::channel() instead")]
#[allow(deprecated)]
pub fn legacy_channel<T: Send + 'static>() -> (LegacyChannelSender<T>, LegacyChannelReceiver<T>) {
    let (sender, receiver) = channel::channel();
    (LegacyChannelSender::new(sender), LegacyChannelReceiver::new(receiver))
}

/// Legacy buffered channel creation function - redirects to new implementation
#[deprecated(note = "Use channel::buffered_channel() instead")]
#[allow(deprecated)]
pub fn legacy_buffered_channel<T: Send + 'static>(capacity: usize) -> (LegacyChannelSender<T>, LegacyChannelReceiver<T>) {
    let (sender, receiver) = channel::buffered_channel(capacity);
    (LegacyChannelSender::new(sender), LegacyChannelReceiver::new(receiver))
}
