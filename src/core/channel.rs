//! Communication channels for concurrent CURSED programs
//!
//! This module implements channel-based communication for CURSED, similar to
//! Go's channels. Channels provide a type-safe way for goroutines to communicate
//! and synchronize their execution.
//!
//! The module offers both buffered and unbuffered channels:
//! - Unbuffered channels synchronize goroutines (send blocks until receive happens)
//! - Buffered channels provide asynchronous communication up to buffer capacity
//!
//! Channels are represented by the `dm` keyword in CURSED code.

use crate::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

/// Creates a new channel for communication between goroutines
///
/// This function creates either a buffered or unbuffered channel of the specified
/// element type. The buffer size parameter determines the channel behavior:
/// - If None or 0: Creates an unbuffered channel (synchronous communication)
/// - If Some(n) where n > 0: Creates a buffered channel with capacity n
///
/// # Arguments
///
/// * `element_type` - The type name of elements that can be sent on this channel
/// * `buffer_size` - Optional buffer capacity; None or 0 creates an unbuffered channel
///
/// # Returns
///
/// A new Channel object
pub fn create_channel(element_type: String, buffer_size: Option<usize>) -> Object {
    let capacity = buffer_size.unwrap_or(0);
    Object::new_channel(element_type, capacity)
}

/// Send a value on a channel
pub fn send_to_channel(channel: Object, value: Object) -> Result<Object, String> {
    match channel {
        Object::Channel(channel_ref) => {
            let mut channel = channel_ref.borrow_mut();
            match channel.send(value) {
                Ok(_) => Ok(Object::Null),
                Err(e) => Err(e.to_string()),
            }
        }
        _ => Err(format!(
            "Cannot send to non-channel type: {}",
            channel.type_name()
        )),
    }
}

/// Receive a value from a channel
pub fn receive_from_channel(channel: Object) -> Result<Object, String> {
    match channel {
        Object::Channel(channel_ref) => {
            let mut channel = channel_ref.borrow_mut();
            match channel.receive() {
                Ok(value) => Ok(value),
                Err(e) => Err(e.to_string()),
            }
        }
        _ => Err(format!(
            "Cannot receive from non-channel type: {}",
            channel.type_name()
        )),
    }
}

/// Try to send a value on a channel without blocking
pub fn try_send_to_channel(channel: Object, value: Object) -> Result<Object, String> {
    match channel {
        Object::Channel(channel_ref) => {
            let mut channel = channel_ref.borrow_mut();
            match channel.try_send(value) {
                Ok(true) => Ok(Object::Boolean(true)),   // Successfully sent
                Ok(false) => Ok(Object::Boolean(false)), // Would block
                Err(e) => Err(e.to_string()),
            }
        }
        _ => Err(format!(
            "Cannot send to non-channel type: {}",
            channel.type_name()
        )),
    }
}

/// Try to receive a value from a channel without blocking
pub fn try_receive_from_channel(channel: Object) -> Result<Object, String> {
    match channel {
        Object::Channel(channel_ref) => {
            let mut channel = channel_ref.borrow_mut();
            match channel.try_receive() {
                Ok(Some(value)) => Ok(value), // Successfully received
                Ok(None) => Ok(Object::Null), // Would block
                Err(e) => Err(e.to_string()),
            }
        }
        _ => Err(format!(
            "Cannot receive from non-channel type: {}",
            channel.type_name()
        )),
    }
}

/// Close a channel
pub fn close_channel(channel: Object) -> Result<Object, String> {
    match channel {
        Object::Channel(channel_ref) => {
            let mut channel = channel_ref.borrow_mut();
            channel.close();
            Ok(Object::Null)
        }
        _ => Err(format!(
            "Cannot close non-channel type: {}",
            channel.type_name()
        )),
    }
}
