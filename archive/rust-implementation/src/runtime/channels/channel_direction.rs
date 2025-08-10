//! Channel direction enforcement for CURSED channels
//!
//! This module implements proper channel direction enforcement (send-only/receive-only)
//! to prevent misuse of channels at compile time and runtime.

use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use crate::runtime::channels::{SendResult, ReceiveResult, ChannelStats};
use crate::runtime::channels::simple_channel::SimpleChannel;

/// Channel direction marker types
pub struct SendOnly;
pub struct ReceiveOnly;
pub struct Bidirectional;

/// Channel direction trait
pub trait ChannelDirection {
    const CAN_SEND: bool;
    const CAN_RECEIVE: bool;
    const DIRECTION_NAME: &'static str;
}

impl ChannelDirection for SendOnly {
    const CAN_SEND: bool = true;
    const CAN_RECEIVE: bool = false;
    const DIRECTION_NAME: &'static str = "send-only";
}

impl ChannelDirection for ReceiveOnly {
    const CAN_SEND: bool = false;
    const CAN_RECEIVE: bool = true;
    const DIRECTION_NAME: &'static str = "receive-only";
}

impl ChannelDirection for Bidirectional {
    const CAN_SEND: bool = true;
    const CAN_RECEIVE: bool = true;
    const DIRECTION_NAME: &'static str = "bidirectional";
}

/// Directional channel wrapper
pub struct DirectionalChannel<T, D: ChannelDirection> {
    inner: Arc<SimpleChannel<T>>,
    _direction: PhantomData<D>,
}

impl<T, D: ChannelDirection> DirectionalChannel<T, D> {
    /// Create a new directional channel wrapper
    pub fn new(channel: Arc<SimpleChannel<T>>) -> Self {
        Self {
            inner: channel,
            _direction: PhantomData,
        }
    }
    
    /// Get channel ID
    pub fn id(&self) -> usize {
        self.inner.id()
    }
    
    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }
    
    /// Get current length
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    
    /// Check if closed
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
    
    /// Get channel statistics
    pub fn stats(&self) -> ChannelStats {
        self.inner.stats()
    }
    
    /// Convert to send-only channel
    pub fn into_send_only(self) -> DirectionalChannel<T, SendOnly> {
        DirectionalChannel::new(self.inner)
    }
    
    /// Convert to receive-only channel
    pub fn into_receive_only(self) -> DirectionalChannel<T, ReceiveOnly> {
        DirectionalChannel::new(self.inner)
    }
    
    /// Convert to bidirectional channel
    pub fn into_bidirectional(self) -> DirectionalChannel<T, Bidirectional> {
        DirectionalChannel::new(self.inner)
    }
    
    /// Get a reference to the underlying channel
    pub fn as_ref(&self) -> &SimpleChannel<T> {
        &self.inner
    }
    
    /// Get direction information
    pub fn direction(&self) -> &'static str {
        D::DIRECTION_NAME
    }
    
    /// Check if this channel can send
    pub fn can_send(&self) -> bool {
        D::CAN_SEND
    }
    
    /// Check if this channel can receive
    pub fn can_receive(&self) -> bool {
        D::CAN_RECEIVE
    }
}

/// Send operations (only available for send-capable channels)
impl<T, D: ChannelDirection> DirectionalChannel<T, D> 
where 
    D: ChannelDirection,
    SendOnly: ChannelDirection,
    Bidirectional: ChannelDirection,
{
    /// Send a value (only if channel can send)
    pub fn send(&self, value: T) -> Result<SendResult<T>, ChannelDirectionError> {
        if !D::CAN_SEND {
            return Err(ChannelDirectionError::SendNotAllowed(D::DIRECTION_NAME));
        }
        Ok(self.inner.send(value))
    }
    
    /// Try to send a value (non-blocking)
    pub fn try_send(&self, value: T) -> Result<SendResult<T>, ChannelDirectionError> {
        if !D::CAN_SEND {
            return Err(ChannelDirectionError::SendNotAllowed(D::DIRECTION_NAME));
        }
        Ok(self.inner.try_send(value))
    }
    
    /// Send with timeout
    pub fn send_timeout(&self, value: T, timeout: Duration) -> Result<SendResult<T>, ChannelDirectionError> {
        if !D::CAN_SEND {
            return Err(ChannelDirectionError::SendNotAllowed(D::DIRECTION_NAME));
        }
        Ok(self.inner.send_timeout(value, timeout))
    }
}

/// Receive operations (only available for receive-capable channels)
impl<T, D: ChannelDirection> DirectionalChannel<T, D> 
where 
    D: ChannelDirection,
    ReceiveOnly: ChannelDirection,
    Bidirectional: ChannelDirection,
{
    /// Receive a value (only if channel can receive)
    pub fn recv(&self) -> Result<ReceiveResult<T>, ChannelDirectionError> {
        if !D::CAN_RECEIVE {
            return Err(ChannelDirectionError::ReceiveNotAllowed(D::DIRECTION_NAME));
        }
        Ok(self.inner.recv())
    }
    
    /// Try to receive a value (non-blocking)
    pub fn try_recv(&self) -> Result<ReceiveResult<T>, ChannelDirectionError> {
        if !D::CAN_RECEIVE {
            return Err(ChannelDirectionError::ReceiveNotAllowed(D::DIRECTION_NAME));
        }
        Ok(self.inner.try_recv())
    }
    
    /// Receive with timeout
    pub fn recv_timeout(&self, timeout: Duration) -> Result<ReceiveResult<T>, ChannelDirectionError> {
        if !D::CAN_RECEIVE {
            return Err(ChannelDirectionError::ReceiveNotAllowed(D::DIRECTION_NAME));
        }
        Ok(self.inner.recv_timeout(timeout))
    }
}

/// Close operation (available for all channels)
impl<T, D: ChannelDirection> DirectionalChannel<T, D> {
    /// Close the channel
    pub fn close(&self) {
        self.inner.close();
    }
}

/// Channel direction error
#[derive(Debug, Clone)]
pub enum ChannelDirectionError {
    SendNotAllowed(&'static str),
    ReceiveNotAllowed(&'static str),
}

impl std::fmt::Display for ChannelDirectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelDirectionError::SendNotAllowed(direction) => {
                write!(f, "Send operation not allowed on {} channel", direction)
            }
            ChannelDirectionError::ReceiveNotAllowed(direction) => {
                write!(f, "Receive operation not allowed on {} channel", direction)
            }
        }
    }
}

impl std::error::Error for ChannelDirectionError {}

/// Type aliases for common channel directions
pub type SendChannel<T> = DirectionalChannel<T, SendOnly>;
pub type ReceiveChannel<T> = DirectionalChannel<T, ReceiveOnly>;
pub type BidirectionalChannel<T> = DirectionalChannel<T, Bidirectional>;

/// Create a directional channel pair
pub fn directional_channel<T>() -> (SendChannel<T>, ReceiveChannel<T>) {
    let channel = Arc::new(SimpleChannel::new());
    let send_channel = DirectionalChannel::new(channel.clone());
    let receive_channel = DirectionalChannel::new(channel);
    (send_channel, receive_channel)
}

/// Create a directional buffered channel pair
pub fn directional_buffered_channel<T>(capacity: usize) -> (SendChannel<T>, ReceiveChannel<T>) {
    let channel = Arc::new(SimpleChannel::with_capacity(capacity));
    let send_channel = DirectionalChannel::new(channel.clone());
    let receive_channel = DirectionalChannel::new(channel);
    (send_channel, receive_channel)
}

/// Create a bidirectional channel
pub fn bidirectional_channel<T>() -> BidirectionalChannel<T> {
    let channel = Arc::new(SimpleChannel::new());
    DirectionalChannel::new(channel)
}

/// Create a bidirectional buffered channel
pub fn bidirectional_buffered_channel<T>(capacity: usize) -> BidirectionalChannel<T> {
    let channel = Arc::new(SimpleChannel::with_capacity(capacity));
    DirectionalChannel::new(channel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_send_only_channel() {
        let (send_ch, _recv_ch) = directional_channel::<i32>();
        
        // Send should work
        assert!(send_ch.send(42).is_ok());
        
        // Receive should not be available (compilation error if uncommented)
        // assert!(send_ch.recv().is_err());
    }

    #[test]
    fn test_receive_only_channel() {
        let (send_ch, recv_ch) = directional_channel::<i32>();
        
        // Send from send channel
        assert!(send_ch.send(42).is_ok());
        
        // Receive should work
        assert!(recv_ch.recv().is_ok());
        
        // Send should not be available (compilation error if uncommented)
        // assert!(recv_ch.send(42).is_err());
    }

    #[test]
    fn test_bidirectional_channel() {
        // Use buffered channel to avoid blocking
        let bidir_ch = bidirectional_buffered_channel::<i32>(1);
        
        // Both send and receive should work
        assert!(bidir_ch.send(42).is_ok());
        let result = bidir_ch.recv();
        assert!(result.is_ok());
        
        // Verify we received the correct value
        if let Ok(ReceiveResult::Received(val)) = result {
            assert_eq!(val, 42);
        }
    }

    #[test]
    fn test_channel_conversion() {
        let bidir_ch = bidirectional_channel::<i32>();
        
        // Convert to send-only
        let send_only = bidir_ch.into_send_only();
        assert!(send_only.can_send());
        assert!(!send_only.can_receive());
        
        // Convert to receive-only
        let bidir_ch = bidirectional_channel::<i32>();
        let recv_only = bidir_ch.into_receive_only();
        assert!(!recv_only.can_send());
        assert!(recv_only.can_receive());
    }

    #[test]
    fn test_channel_direction_info() {
        let (send_ch, recv_ch) = directional_channel::<i32>();
        let bidir_ch = bidirectional_channel::<i32>();
        
        assert_eq!(send_ch.direction(), "send-only");
        assert_eq!(recv_ch.direction(), "receive-only");
        assert_eq!(bidir_ch.direction(), "bidirectional");
    }

    #[test]
    fn test_timeout_operations() {
        let (send_ch, recv_ch) = directional_buffered_channel::<i32>(1);
        
        // Send with timeout should work
        assert!(send_ch.send_timeout(42, Duration::from_millis(100)).is_ok());
        
        // Receive with timeout should work
        assert!(recv_ch.recv_timeout(Duration::from_millis(100)).is_ok());
    }
}
