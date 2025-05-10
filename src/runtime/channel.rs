//! Runtime implementation of channel operations
//!
//! This module provides the actual implementation of channel operations that are
//! called by the LLVM-generated code. It serves as the bridge between the compiled
//! LLVM code and the CURSED channel implementation in the core library.

use std::rc::Rc;
use std::cell::RefCell;
use std::panic;
use crate::object::{Object, Channel};
use crate::core::channel::{create_channel, send_to_channel, receive_from_channel};
use crate::error::Error;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;

/// Creates a new channel with the specified element size and capacity
///
/// This function is called from LLVM-generated code via FFI.
///
/// # Safety
///
/// This function should only be called from LLVM-generated code.
///
/// # Arguments
///
/// * `element_size` - Size in bytes of the channel element type
/// * `capacity` - Capacity of the channel buffer (0 for unbuffered)
///
/// # Returns
///
/// A raw pointer to the created channel, as a void pointer
#[no_mangle]
pub extern "C" fn cursed_make_channel(element_size: u64, capacity: u64) -> *mut c_void {
    tracing::debug!(
        element_size = element_size,
        capacity = capacity,
        "Creating channel via FFI"
    );
    
    // Create the appropriate element type name based on size
    // This is a simplification - in reality, we'd need more type information
    let element_type = match element_size {
        1 => "byte",
        4 => "normie",
        8 => "thicc",
        _ => "any",
    }.to_string();
    
    // Create the channel
    let channel = create_channel(element_type, Some(capacity as usize));
    
    // Convert to a raw pointer that can be passed via FFI
    match channel {
        Object::Channel(channel_ref) => {
            // We're returning a raw pointer to the RefCell<Channel>
            // The caller will need to handle this as a Channel*
            let raw_ptr = Rc::into_raw(channel_ref) as *mut c_void;
            tracing::debug!(ptr = ?raw_ptr, "Channel created");
            raw_ptr
        },
        _ => {
            tracing::error!("Failed to create channel");
            std::ptr::null_mut()
        }
    }
}

/// Sends a value to a channel
///
/// This function is called from LLVM-generated code via FFI.
///
/// # Safety
///
/// This function should only be called from LLVM-generated code.
///
/// # Arguments
///
/// * `channel_ptr` - Pointer to the channel
/// * `value_ptr` - Pointer to the value to send
#[no_mangle]
pub extern "C" fn cursed_send_to_channel(channel_ptr: *mut c_void, value_ptr: *mut c_void) {
    tracing::debug!(channel_ptr = ?channel_ptr, value_ptr = ?value_ptr, "Sending to channel via FFI");
    
    // Safety: Ensure pointers are valid
    if channel_ptr.is_null() || value_ptr.is_null() {
        tracing::error!("Null pointer in send_to_channel");
        return;
    }
    
    // Convert the raw channel pointer back to an Rc<RefCell<Channel>>
    let channel_ref = unsafe { Rc::from_raw(channel_ptr as *const RefCell<Channel>) };
    
    // Clone the Rc to keep it alive after this function returns
    let channel_ref_clone = channel_ref.clone();
    
    // Convert from raw pointer back to Object
    // This is a simplified approach - in reality, we'd need to know the type
    // Assuming the value_ptr points to an i64 for this example
    let value = unsafe {
        let value_int = *(value_ptr as *const i64);
        Object::Integer(value_int)
    };
    
    // Send the value to the channel
    let channel_obj = Object::Channel(channel_ref_clone);
    match send_to_channel(channel_obj, value) {
        Ok(_) => {
            tracing::debug!("Value sent to channel successfully");
        },
        Err(e) => {
            tracing::error!(error = ?e, "Failed to send to channel");
        }
    }
    
    // Instead of forgetting the Rc, clone it to maintain proper ownership
    let _ = Rc::into_raw(channel_ref.clone());
}

/// Receives a value from a channel
///
/// This function is called from LLVM-generated code via FFI.
///
/// # Safety
///
/// This function should only be called from LLVM-generated code.
///
/// # Arguments
///
/// * `channel_ptr` - Pointer to the channel
/// * `result_ptr` - Pointer where the received value will be stored
#[no_mangle]
pub extern "C" fn cursed_receive_from_channel(channel_ptr: *mut c_void, result_ptr: *mut c_void) {
    tracing::debug!(channel_ptr = ?channel_ptr, result_ptr = ?result_ptr, "Receiving from channel via FFI");
    
    // Safety: Ensure pointers are valid
    if channel_ptr.is_null() || result_ptr.is_null() {
        tracing::error!("Null pointer in receive_from_channel");
        return;
    }
    
    // Convert the raw channel pointer back to an Rc<RefCell<Channel>>
    let channel_ref = unsafe { Rc::from_raw(channel_ptr as *const RefCell<Channel>) };
    
    // Clone the Rc to keep it alive after this function returns
    let channel_ref_clone = channel_ref.clone();
    
    // Receive from the channel
    let channel_obj = Object::Channel(channel_ref_clone);
    match receive_from_channel(channel_obj) {
        Ok(value) => {
            // Store the result in the provided memory location
            // This is a simplified approach - in reality, we'd need to know the type
            unsafe {
                match value {
                    Object::Integer(i) => {
                        *(result_ptr as *mut i64) = i;
                    },
                    Object::Float(f) => {
                        *(result_ptr as *mut f64) = f;
                    },
                    Object::Boolean(b) => {
                        *(result_ptr as *mut bool) = b;
                    },
                    Object::String(s) => {
                        // String handling would be more complex and require proper memory management
                        tracing::warn!("String handling in FFI is not fully implemented");
                    },
                    _ => {
                        tracing::warn!("Unhandled type in FFI receive");
                    }
                }
            }
            tracing::debug!("Value received from channel successfully");
        },
        Err(e) => {
            tracing::error!(error = ?e, "Failed to receive from channel");
        }
    }
    
    // Instead of forgetting the Rc, clone it to maintain proper ownership
    let _ = Rc::into_raw(channel_ref.clone());
}

/// Closes a channel
///
/// This function is called from LLVM-generated code via FFI.
///
/// # Safety
///
/// This function should only be called from LLVM-generated code.
///
/// # Arguments
///
/// * `channel_ptr` - Pointer to the channel
#[no_mangle]
pub extern "C" fn cursed_close_channel(channel_ptr: *mut c_void) {
    tracing::debug!(channel_ptr = ?channel_ptr, "Closing channel via FFI");
    
    // Safety: Ensure pointer is valid
    if channel_ptr.is_null() {
        tracing::error!("Null pointer in close_channel");
        return;
    }
    
    // Convert the raw channel pointer back to an Rc<RefCell<Channel>>
    let channel_ref = unsafe { Rc::from_raw(channel_ptr as *const RefCell<Channel>) };
    
    // Close the channel
    let mut channel = channel_ref.borrow_mut();
    channel.close();
    
    // Instead of forgetting the Rc, clone it to maintain proper ownership
    let _ = Rc::into_raw(channel_ref.clone());
}