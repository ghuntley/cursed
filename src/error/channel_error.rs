//! Specialized error types for channel operations
//!
//! This module provides comprehensive error handling for channel operations
//! including closed channel errors, timeout errors, and operation failures.

use std::fmt;
use crate::error::Error;

/// Specialized error types for channel operations
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelError {
    /// Channel is closed and cannot accept new values
    ChannelClosed {
        operation: String,
        channel_type: String,
    },
    
    /// Operation would block (for non-blocking operations)
    WouldBlock {
        operation: String,
        reason: String,
    },
    
    /// Timeout occurred during operation
    Timeout {
        operation: String,
        timeout_ms: u64,
    },
    
    /// Channel buffer is full
    BufferFull {
        capacity: usize,
        current_size: usize,
    },
    
    /// Channel buffer is empty
    BufferEmpty,
    
    /// Invalid channel state
    InvalidState {
        expected: String,
        actual: String,
    },
    
    /// Type mismatch for channel operations
    TypeMismatch {
        expected: String,
        actual: String,
    },
    
    /// Panic occurred during channel operation
    PanicOccurred {
        operation: String,
        details: String,
    },
    
    /// Null pointer provided to FFI function
    NullPointer {
        function: String,
        parameter: String,
    },
}

impl ChannelError {
    /// Create a channel closed error
    pub fn closed(operation: &str, channel_type: &str) -> Self {
        ChannelError::ChannelClosed {
            operation: operation.to_string(),
            channel_type: channel_type.to_string(),
        }
    }
    
    /// Create a would block error
    pub fn would_block(operation: &str, reason: &str) -> Self {
        ChannelError::WouldBlock {
            operation: operation.to_string(),
            reason: reason.to_string(),
        }
    }
    
    /// Create a timeout error
    pub fn timeout(operation: &str, timeout_ms: u64) -> Self {
        ChannelError::Timeout {
            operation: operation.to_string(),
            timeout_ms,
        }
    }
    
    /// Create a buffer full error
    pub fn buffer_full(capacity: usize, current_size: usize) -> Self {
        ChannelError::BufferFull {
            capacity,
            current_size,
        }
    }
    
    /// Create a buffer empty error
    pub fn buffer_empty() -> Self {
        ChannelError::BufferEmpty
    }
    
    /// Create an invalid state error
    pub fn invalid_state(expected: &str, actual: &str) -> Self {
        ChannelError::InvalidState {
            expected: expected.to_string(),
            actual: actual.to_string(),
        }
    }
    
    /// Create a type mismatch error
    pub fn type_mismatch(expected: &str, actual: &str) -> Self {
        ChannelError::TypeMismatch {
            expected: expected.to_string(),
            actual: actual.to_string(),
        }
    }
    
    /// Create a panic occurred error
    pub fn panic_occurred(operation: &str, details: &str) -> Self {
        ChannelError::PanicOccurred {
            operation: operation.to_string(),
            details: details.to_string(),
        }
    }
    
    /// Create a null pointer error
    pub fn null_pointer(function: &str, parameter: &str) -> Self {
        ChannelError::NullPointer {
            function: function.to_string(),
            parameter: parameter.to_string(),
        }
    }
    
    /// Get the error code for FFI functions
    pub fn error_code(&self) -> i32 {
        match self {
            ChannelError::ChannelClosed { .. } => -2,
            ChannelError::WouldBlock { .. } => 1,
            ChannelError::Timeout { .. } => 2,
            ChannelError::BufferFull { .. } => 1, // Same as would block
            ChannelError::BufferEmpty => 1, // Same as would block
            ChannelError::InvalidState { .. } => -3,
            ChannelError::TypeMismatch { .. } => -4,
            ChannelError::PanicOccurred { .. } => -5,
            ChannelError::NullPointer { .. } => -1,
        }
    }
    
    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        match self {
            ChannelError::WouldBlock { .. } => true,
            ChannelError::Timeout { .. } => true,
            ChannelError::BufferFull { .. } => true,
            ChannelError::BufferEmpty => true,
            _ => false,
        }
    }
    
    /// Check if this error indicates the channel is closed
    pub fn is_closed_error(&self) -> bool {
        matches!(self, ChannelError::ChannelClosed { .. })
    }
}

impl fmt::Display for ChannelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChannelError::ChannelClosed { operation, channel_type } => {
                write!(f, "Cannot perform '{}' on closed channel of type '{}'", operation, channel_type)
            },
            ChannelError::WouldBlock { operation, reason } => {
                write!(f, "Operation '{}' would block: {}", operation, reason)
            },
            ChannelError::Timeout { operation, timeout_ms } => {
                write!(f, "Operation '{}' timed out after {}ms", operation, timeout_ms)
            },
            ChannelError::BufferFull { capacity, current_size } => {
                write!(f, "Channel buffer full: {}/{} (would block)", current_size, capacity)
            },
            ChannelError::BufferEmpty => {
                write!(f, "Channel buffer empty (would block)")
            },
            ChannelError::InvalidState { expected, actual } => {
                write!(f, "Invalid channel state: expected '{}', got '{}'", expected, actual)
            },
            ChannelError::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch: expected '{}', got '{}'", expected, actual)
            },
            ChannelError::PanicOccurred { operation, details } => {
                write!(f, "Panic during '{}': {}", operation, details)
            },
            ChannelError::NullPointer { function, parameter } => {
                write!(f, "Null pointer passed to '{}' for parameter '{}'", function, parameter)
            },
        }
    }
}

impl std::error::Error for ChannelError {}

/// Convert ChannelError to the main Error type
impl From<ChannelError> for Error {
    fn from(channel_error: ChannelError) -> Self {
        Error::Runtime(format!("Channel error: {}", channel_error))
    }
}

/// Helper functions for common channel error scenarios
impl ChannelError {
    /// Error for sending to a closed channel
    pub fn send_to_closed(channel_type: &str) -> Self {
        Self::closed("send", channel_type)
    }
    
    /// Error for receiving from a closed empty channel  
    pub fn receive_from_closed_empty(channel_type: &str) -> Self {
        Self::closed("receive", channel_type)
    }
    
    /// Error for send operation that would block
    pub fn send_would_block(reason: &str) -> Self {
        Self::would_block("send", reason)
    }
    
    /// Error for receive operation that would block
    pub fn receive_would_block(reason: &str) -> Self {
        Self::would_block("receive", reason)
    }
    
    /// Error for close operation timeout
    pub fn close_timeout(timeout_ms: u64) -> Self {
        Self::timeout("close", timeout_ms)
    }
    
    /// Error for send operation timeout
    pub fn send_timeout(timeout_ms: u64) -> Self {
        Self::timeout("send", timeout_ms)
    }
    
    /// Error for receive operation timeout
    pub fn receive_timeout(timeout_ms: u64) -> Self {
        Self::timeout("receive", timeout_ms)
    }
}

/// Result type for channel operations
pub type ChannelResult<T> = Result<T, ChannelError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_error_creation() {
        let closed_error = ChannelError::closed("send", "normie");
        assert!(closed_error.is_closed_error());
        assert!(!closed_error.is_recoverable());
        assert_eq!(closed_error.error_code(), -2);
        
        let block_error = ChannelError::would_block("receive", "buffer empty");
        assert!(!block_error.is_closed_error());
        assert!(block_error.is_recoverable());
        assert_eq!(block_error.error_code(), 1);
    }

    #[test]
    fn test_error_display() {
        let error = ChannelError::closed("send", "normie");
        let display = format!("{}", error);
        assert!(display.contains("Cannot perform 'send'"));
        assert!(display.contains("closed channel"));
        assert!(display.contains("normie"));
    }

    #[test]
    fn test_error_conversion() {
        let channel_error = ChannelError::buffer_full(10, 10);
        let general_error: Error = channel_error.into();
        
        let error_msg = format!("{}", general_error);
        assert!(error_msg.contains("Channel error"));
        assert!(error_msg.contains("buffer full"));
    }

    #[test]
    fn test_helper_functions() {
        let send_closed = ChannelError::send_to_closed("string");
        assert!(send_closed.is_closed_error());
        
        let recv_block = ChannelError::receive_would_block("empty buffer");
        assert!(recv_block.is_recoverable());
        
        let timeout = ChannelError::close_timeout(5000);
        assert_eq!(timeout.error_code(), 2);
    }

    #[test]
    fn test_null_pointer_error() {
        let null_error = ChannelError::null_pointer("cursed_send_to_channel", "channel_ptr");
        assert_eq!(null_error.error_code(), -1);
        assert!(!null_error.is_recoverable());
        
        let display = format!("{}", null_error);
        assert!(display.contains("Null pointer"));
        assert!(display.contains("cursed_send_to_channel"));
        assert!(display.contains("channel_ptr"));
    }

    #[test]
    fn test_panic_error() {
        let panic_error = ChannelError::panic_occurred("send", "mutex poisoned");
        assert_eq!(panic_error.error_code(), -5);
        assert!(!panic_error.is_recoverable());
    }
}
