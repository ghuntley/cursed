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
pub mod channel;
#[allow(dead_code)]
pub mod operations;
#[allow(dead_code)]
pub mod memory;
#[allow(dead_code)]
pub mod sync;

// Re-export the simple implementation as the main interface
pub use simple_channel::{
    SimpleChannel,
    SimpleChannelSender,
    SimpleChannelReceiver,
    simple_channel,
    simple_buffered_channel,
};

// Main channel API using simple implementation
pub type Channel<T> = SimpleChannel<T>;
pub type ChannelSender<T> = SimpleChannelSender<T>;
pub type ChannelReceiver<T> = SimpleChannelReceiver<T>;
pub fn channel<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    simple_channel()
}
pub fn buffered_channel<T>(capacity: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    simple_buffered_channel(capacity)
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

// Legacy compatibility wrapper - keeping old interface for backward compatibility
// The new full-featured implementation is in the channel module

/// Legacy channel wrapper - redirects to new implementation
#[deprecated(note = "Use the new Channel type from channel module")]
pub struct LegacyChannel<T> {
    inner: channel::Channel<T>,
}

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

impl<T: Send + 'static> Default for LegacyChannel<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy channel sender handle - redirects to new implementation
#[deprecated(note = "Use ChannelSender from channel module")]
pub struct LegacyChannelSender<T> {
    inner: channel::ChannelSender<T>,
}

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
pub struct LegacyChannelReceiver<T> {
    inner: channel::ChannelReceiver<T>,
}

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
pub fn legacy_channel<T: Send + 'static>() -> (LegacyChannelSender<T>, LegacyChannelReceiver<T>) {
    let (sender, receiver) = channel::channel();
    (LegacyChannelSender::new(sender), LegacyChannelReceiver::new(receiver))
}

/// Legacy buffered channel creation function - redirects to new implementation
#[deprecated(note = "Use channel::buffered_channel() instead")]
pub fn legacy_buffered_channel<T: Send + 'static>(capacity: usize) -> (LegacyChannelSender<T>, LegacyChannelReceiver<T>) {
    let (sender, receiver) = channel::buffered_channel(capacity);
    (LegacyChannelSender::new(sender), LegacyChannelReceiver::new(receiver))
}
