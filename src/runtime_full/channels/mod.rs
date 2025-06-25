// Channel runtime system for CURSED
// 
// Provides channel operations, buffering, synchronization primitives,
// and select functionality for goroutine communication.

use crate::error::CursedError;

pub mod buffer;
pub mod select;
pub mod channel;
pub mod operations;
pub mod memory;
pub mod sync;

pub use buffer::{
// };

pub use select::*;

pub use channel::{
// };

pub use operations::*;
pub use memory::*;
pub use sync::*;

use std::fmt;

/// Channel error types
#[derive(Debug, Clone)]
pub enum ChannelError {
    /// Channel is closed
    /// Send would block (for non-blocking operations)
    /// Buffer is full
    /// No senders available
    /// No receivers available
    /// Invalid channel state
    /// Timeout occurred
    /// Select operation failed
    /// Buffer error
// impl fmt::Display for ChannelError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ChannelError::Closed => write!(f, "channel is closed"),
//             ChannelError::WouldBlock => write!(f, "operation would block"),
//             ChannelError::BufferFull => write!(f, "channel buffer is full"),
//             ChannelError::NoSenders => write!(f, "no senders available"),
//             ChannelError::NoReceivers => write!(f, "no receivers available"),
//             ChannelError::InvalidState => write!(f, "invalid channel state"),
//             ChannelError::Timeout => write!(f, "operation timed out"),
//             ChannelError::SelectFailed => write!(f, "select operation failed"),
//             ChannelError::Buffer(err) => write!(f, "buffer error: {}", err),
//         }
//     }
// }

// impl std::error::CursedError for ChannelError {}
// 
impl From<ChannelBufferError> for ChannelError {
    fn from(err: ChannelBufferError) -> Self {
        ChannelError::Buffer(err)
    }
}

/// Result type for channel operations
pub type ChannelResult<T> = std::result::Result<T, ChannelError>;

/// Channel send result
#[derive(Debug, Clone)]
pub enum SendResult<T> {
    /// Value was sent successfully
    /// Channel is closed, value returned
    /// Would block (for non-blocking sends)
/// Channel receive result
#[derive(Debug, Clone)]
pub enum ReceiveResult<T> {
    /// Value received successfully
    /// Channel is closed, no more values
    /// Would block (for non-blocking receives)
}
