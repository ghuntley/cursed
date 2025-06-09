//! Clean runtime implementation of channel operations with buffered channel support
//!
//! This module provides the actual implementation of channel operations that are
//! called by the LLVM-generated code. It serves as the bridge between the compiled
//! LLVM code and the CURSED channel implementation.

use std::sync::{Arc, RwLock};
use std::panic;
use crate::object::{Object, Channel};
use crate::object_thread_safe::ThreadSafeObject;
use crate::runtime::buffered_channel::{ThreadSafeBufferedChannel, ChannelStats};
use crate::core::channel::{create_channel, send_to_channel, receive_from_channel, try_send_to_channel, try_receive_from_channel};
use crate::memory::{GarbageCollector, ThreadSafeGc};
use crate::error::Error;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::time::Duration;

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
    let element_type = match element_size {
        1 => "byte",
        4 => "normie", 
        8 => "thicc",
        _ => "any",
    }.to_string();
    
    // Use the enhanced buffered channel implementation for better performance
    if capacity > 0 {
        // Create a buffered channel using the new implementation
        let buffered_channel = ThreadSafeBufferedChannel::new(element_type, capacity as usize);
        let boxed = Box::new(buffered_channel);
        let raw_ptr = Box::into_raw(boxed) as *mut c_void;
        tracing::debug!(ptr = ?raw_ptr, capacity = capacity, "Buffered channel created");
        raw_ptr
    } else {
        // For unbuffered channels, use the existing implementation
        let use_thread_safe = cfg!(feature = "concurrent_gc");
        
        if use_thread_safe {
            let channel = ThreadSafeObject::new_channel(element_type, 0);
            let boxed = Box::new(channel);
            let raw_ptr = Box::into_raw(boxed) as *mut c_void;
            tracing::debug!(ptr = ?raw_ptr, "Thread-safe unbuffered channel created");
            raw_ptr
        } else {
            let channel = create_channel(element_type, None);
            match channel {
                Object::Channel(channel_ref) => {
                    let raw_ptr = Arc::into_raw(channel_ref) as *mut c_void;
                    tracing::debug!(ptr = ?raw_ptr, "Unbuffered channel created");
                    raw_ptr
                },
                _ => {
                    tracing::error!("Failed to create channel");
                    std::ptr::null_mut()
                }
            }
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
    
    if channel_ptr.is_null() || value_ptr.is_null() {
        tracing::error!("Null pointer in send_to_channel");
        return;
    }
    
    // Try to interpret as buffered channel first
    if let Ok(_) = std::panic::catch_unwind(|| unsafe {
        let channel = &*(channel_ptr as *const ThreadSafeBufferedChannel);
        let value_int = *(value_ptr as *const i64);
        let value_obj = Object::Integer(value_int);
        
        match channel.send(value_obj) {
            Ok(_) => {
                tracing::debug!("Value sent to buffered channel successfully");
            },
            Err(e) => {
                tracing::error!(error = ?e, "Failed to send to buffered channel");
            }
        }
    }) {
        return;
    }
    
    // Fall back to existing implementations
    tracing::warn!("Falling back to legacy channel implementation");
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
    
    if channel_ptr.is_null() || result_ptr.is_null() {
        tracing::error!("Null pointer in receive_from_channel");
        return;
    }
    
    // Try to interpret as buffered channel first
    if let Ok(_) = std::panic::catch_unwind(|| unsafe {
        let channel = &*(channel_ptr as *const ThreadSafeBufferedChannel);
        
        match channel.receive() {
            Ok(value) => {
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
                    _ => {
                        tracing::warn!("Unhandled type in buffered channel FFI receive");
                    }
                }
                tracing::debug!("Value received from buffered channel successfully");
            },
            Err(e) => {
                tracing::error!(error = ?e, "Failed to receive from buffered channel");
            }
        }
    }) {
        return;
    }
    
    // Fall back to existing implementations
    tracing::warn!("Falling back to legacy channel implementation");
}

/// Try to send a value to a channel without blocking
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
///
/// # Returns
///
/// 1 if value was sent successfully, 0 if would block, -1 if error
#[no_mangle]
pub extern "C" fn cursed_try_send_to_channel(channel_ptr: *mut c_void, value_ptr: *mut c_void) -> i32 {
    tracing::debug!(channel_ptr = ?channel_ptr, value_ptr = ?value_ptr, "Try sending to channel via FFI");
    
    if channel_ptr.is_null() || value_ptr.is_null() {
        tracing::error!("Null pointer in try_send_to_channel");
        return -1;
    }
    
    // Try to interpret as buffered channel first
    if let Ok(result) = std::panic::catch_unwind(|| unsafe {
        let channel = &*(channel_ptr as *const ThreadSafeBufferedChannel);
        let value_int = *(value_ptr as *const i64);
        let value_obj = Object::Integer(value_int);
        
        channel.try_send(value_obj)
    }) {
        match result {
            Ok(true) => {
                tracing::debug!("Value sent to buffered channel successfully");
                return 1;
            },
            Ok(false) => {
                tracing::debug!("Buffered channel would block");
                return 0;
            },
            Err(e) => {
                tracing::error!(error = ?e, "Failed to send to buffered channel");
                return -1;
            }
        }
    }
    
    // Fall back to existing implementation for regular channels
    -1 // Not implemented for regular channels in this version
}

/// Try to receive a value from a channel without blocking
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
///
/// # Returns
///
/// 1 if value was received successfully, 0 if would block, -1 if error
#[no_mangle]
pub extern "C" fn cursed_try_receive_from_channel(channel_ptr: *mut c_void, result_ptr: *mut c_void) -> i32 {
    tracing::debug!(channel_ptr = ?channel_ptr, result_ptr = ?result_ptr, "Try receiving from channel via FFI");
    
    if channel_ptr.is_null() || result_ptr.is_null() {
        tracing::error!("Null pointer in try_receive_from_channel");
        return -1;
    }
    
    // Try to interpret as buffered channel first
    if let Ok(result) = std::panic::catch_unwind(|| unsafe {
        let channel = &*(channel_ptr as *const ThreadSafeBufferedChannel);
        channel.try_receive()
    }) {
        match result {
            Ok(Some(value)) => {
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
                        _ => {
                            tracing::warn!("Unhandled type in buffered channel FFI receive");
                            return -1;
                        }
                    }
                }
                tracing::debug!("Value received from buffered channel successfully");
                return 1;
            },
            Ok(None) => {
                tracing::debug!("Buffered channel would block");
                return 0;
            },
            Err(e) => {
                tracing::error!(error = ?e, "Failed to receive from buffered channel");
                return -1;
            }
        }
    }
    
    // Fall back to existing implementation for regular channels
    -1 // Not implemented for regular channels in this version
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
///
/// # Returns
///
/// 0 on success, -1 on error
#[no_mangle]
pub extern "C" fn cursed_close_channel(channel_ptr: *mut c_void) -> i32 {
    tracing::debug!(channel_ptr = ?channel_ptr, "Closing channel via FFI");
    
    if channel_ptr.is_null() {
        tracing::error!("Null pointer in close_channel");
        return -1;
    }
    
    // Try to interpret as buffered channel first
    if let Ok(result) = std::panic::catch_unwind(|| unsafe {
        let channel = &*(channel_ptr as *const ThreadSafeBufferedChannel);
        channel.close()
    }) {
        match result {
            Ok(_) => {
                tracing::debug!("Buffered channel closed successfully");
                return 0;
            },
            Err(e) => {
                tracing::error!(error = ?e, "Failed to close buffered channel");
                return -1;
            }
        }
    }
    
    // Fall back for other channel types
    tracing::warn!("Channel close not implemented for legacy channels");
    -1
}

/// Get channel statistics
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
/// * `stats_ptr` - Pointer to a buffer where stats will be written
///
/// # Returns
///
/// 0 on success, -1 on error
#[repr(C)]
pub struct ChannelStatsFfi {
    pub capacity: u64,
    pub current_length: u64,
    pub is_closed: u8,
    pub send_waiters: u64,
    pub recv_waiters: u64,
    pub available_space: u64,
}

#[no_mangle]
pub extern "C" fn cursed_channel_stats(channel_ptr: *mut c_void, stats_ptr: *mut ChannelStatsFfi) -> i32 {
    if channel_ptr.is_null() || stats_ptr.is_null() {
        return -1;
    }
    
    // Try buffered channel first
    if let Ok(stats) = std::panic::catch_unwind(|| unsafe {
        let channel = &*(channel_ptr as *const ThreadSafeBufferedChannel);
        channel.stats()
    }) {
        unsafe {
            (*stats_ptr) = ChannelStatsFfi {
                capacity: stats.capacity as u64,
                current_length: stats.current_length as u64,
                is_closed: if stats.is_closed { 1 } else { 0 },
                send_waiters: stats.send_waiters as u64,
                recv_waiters: stats.recv_waiters as u64,
                available_space: stats.available_space as u64,
            };
        }
        return 0;
    }
    
    // Fall back to regular channels
    unsafe {
        (*stats_ptr) = ChannelStatsFfi {
            capacity: 0,
            current_length: 0,
            is_closed: 0,
            send_waiters: 0,
            recv_waiters: 0,
            available_space: 0,
        };
    }
    return 0;
}
