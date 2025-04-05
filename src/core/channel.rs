//! Core channel operations for CURSED language runtime

use crate::object::{Object, Channel};
use crate::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

/// Create a new channel with the specified element type and optional capacity
pub fn create_channel(element_type: String, capacity: Option<usize>) -> Object {
    let buffer_size = capacity.unwrap_or(0); // Default to unbuffered channel (capacity 0)
    let channel = Channel::new(element_type, buffer_size);
    Object::Channel(Rc::new(RefCell::new(channel)))
}

/// Send a value to a channel
pub fn send_to_channel(channel_obj: &Object, value: Object) -> Result<Object, Error> {
    match channel_obj {
        Object::Channel(channel_ref) => {
            let mut channel = channel_ref.borrow_mut();
            match channel.send(value.clone()) {
                Ok(_) => Ok(Object::Null), // Send operation returns nothing
                Err(e) => Err(e),
            }
        },
        _ => Err(Error::Runtime(format!("Expected channel object, got {}", channel_obj.type_name())))
    }
}

/// Try to send a value to a channel without blocking
pub fn try_send_to_channel(channel_obj: &Object, value: Object) -> Result<Object, Error> {
    match channel_obj {
        Object::Channel(channel_ref) => {
            let mut channel = channel_ref.borrow_mut();
            match channel.try_send(value.clone()) {
                Ok(sent) => Ok(Object::Boolean(sent)), // Returns boolean indicating success
                Err(e) => Err(e),
            }
        },
        _ => Err(Error::Runtime(format!("Expected channel object, got {}", channel_obj.type_name())))
    }
}

/// Receive a value from a channel
pub fn receive_from_channel(channel_obj: &Object) -> Result<Object, Error> {
    match channel_obj {
        Object::Channel(channel_ref) => {
            let mut channel = channel_ref.borrow_mut();
            match channel.receive() {
                Ok(value) => Ok(value),
                Err(e) => Err(e),
            }
        },
        _ => Err(Error::Runtime(format!("Expected channel object, got {}", channel_obj.type_name())))
    }
}

/// Try to receive a value from a channel without blocking
pub fn try_receive_from_channel(channel_obj: &Object) -> Result<Object, Error> {
    match channel_obj {
        Object::Channel(channel_ref) => {
            let mut channel = channel_ref.borrow_mut();
            match channel.try_receive() {
                Ok(Some(value)) => Ok(Object::Array(vec![Object::Boolean(true), value])),
                Ok(None) => Ok(Object::Array(vec![Object::Boolean(false)])),
                Err(e) => Err(e),
            }
        },
        _ => Err(Error::Runtime(format!("Expected channel object, got {}", channel_obj.type_name())))
    }
}

/// Close a channel
pub fn close_channel(channel_obj: &Object) -> Result<Object, Error> {
    match channel_obj {
        Object::Channel(channel_ref) => {
            let mut channel = channel_ref.borrow_mut();
            channel.close();
            Ok(Object::Null)
        },
        _ => Err(Error::Runtime(format!("Expected channel object, got {}", channel_obj.type_name())))
    }
}