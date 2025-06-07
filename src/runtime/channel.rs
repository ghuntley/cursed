//! Runtime implementation of channel operations
//!
//! This module provides the actual implementation of channel operations that are
//! called by the LLVM-generated code. It serves as the bridge between the compiled
//! LLVM code and the CURSED channel implementation in the core library.

use std::cell::RefCell;
use std::sync::{Arc, RwLock};
use std::panic;
use std::sync::Mutex;
use crate::object::{Object, Channel};
use crate::object_thread_safe::ThreadSafeObject;
use crate::runtime::channel_gc::ThreadSafeChannel;
use crate::core::channel::{create_channel, send_to_channel, receive_from_channel};
use crate::memory::{GarbageCollector, ThreadSafeGc};
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
    
    // Determine if we should use thread-safe channels
    let use_thread_safe = cfg!(feature = "concurrent_gc");
    
    if use_thread_safe {
        // Create a thread-safe channel
        let channel = ThreadSafeObject::new_channel(element_type, capacity as usize);
        
        // Create a new heap-allocated box to hold the channel
        let boxed = Box::new(channel);
        
        // Convert to a raw pointer that can be passed via FFI
        let raw_ptr = Box::into_raw(boxed) as *mut c_void;
        tracing::debug!(ptr = ?raw_ptr, "Thread-safe channel created");
        raw_ptr
    } else {
        // Create the regular channel
        let channel = create_channel(element_type, Some(capacity as usize));
        
        // Convert to a raw pointer that can be passed via FFI
        match channel {
            Object::Channel(channel_ref) => {
                // We're returning a raw pointer to the RefCell<Channel>
                // The caller will need to handle this as a Channel*
                let raw_ptr = Arc::into_raw(channel_ref) as *mut c_void;
                tracing::debug!(ptr = ?raw_ptr, "Channel created");
                raw_ptr
            },
            _ => {
                tracing::error!("Failed to create channel");
                std::ptr::null_mut()
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
    
    // Safety: Ensure pointers are valid
    if channel_ptr.is_null() || value_ptr.is_null() {
        tracing::error!("Null pointer in send_to_channel");
        return;
    }
    
    // Determine if we should use thread-safe channels
    let use_thread_safe = cfg!(feature = "concurrent_gc");
    
    if use_thread_safe {
        // Get a reference to the global GC
        let gc = crate::memory::get_global_gc();
        
        // Convert the raw channel pointer back to a ThreadSafeObject
        let thread_safe_channel = unsafe { Box::from_raw(channel_ptr as *mut ThreadSafeObject) };
        
        // Get a clone to keep it alive after this function returns
        let channel_clone = thread_safe_channel.clone();
        
        // Convert from raw pointer back to Object
        // This is a simplified approach - in reality, we'd need to know the type
        // Assuming the value_ptr points to an i64 for this example
        let value_int = unsafe { *(value_ptr as *const i64) };
        
        // Create a thread-safe value object
        let value_obj = ThreadSafeObject::Integer(value_int);
        
        // Allocate the value in the GC to get a ThreadSafeGc<ThreadSafeObject>
        let value_gc = gc.allocate_thread_safe(value_obj);
        
        // Send the value to the channel
        match channel_clone.channel_send(value_gc) {
            Ok(_) => {
                tracing::debug!("Value sent to thread-safe channel successfully");
            },
            Err(e) => {
                tracing::error!(error = ?e, "Failed to send to thread-safe channel");
            }
        }
        
        // Put the box back so we don't drop it
        let _ = Box::into_raw(thread_safe_channel);
        } else {
        // Convert the raw channel pointer back to an Arc<RwLock<Channel>>
        let channel_ref = unsafe { Arc::from_raw(channel_ptr as *const RwLock<Channel>) };
        
        // Clone the Arc to keep it alive after this function returns
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
        
        // Instead of forgetting the Arc, clone it to maintain proper ownership
        let _ = Arc::into_raw(channel_ref.clone());
    }
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
    
    // Determine if we should use thread-safe channels
    let use_thread_safe = cfg!(feature = "concurrent_gc");
    
    if use_thread_safe {
        // Get a reference to the global GC
        let gc = crate::memory::get_global_gc();
        
        // Convert the raw channel pointer back to a ThreadSafeObject
        let thread_safe_channel = unsafe { Box::from_raw(channel_ptr as *mut ThreadSafeObject) };
        
        // Get a clone to keep it alive after this function returns
        let channel_clone = thread_safe_channel.clone();
        
        // Receive from the channel
        match channel_clone.channel_receive(&gc) {
            Ok(value_gc) => {
                // Unwrap the value from the ThreadSafeGc
                if let Some(value) = value_gc.inner() {
                    unsafe {
                        // Store the result in the provided memory location based on the type
                        match value {
                            ThreadSafeObject::Integer(i) => {
                                *(result_ptr as *mut i64) = *i;
                            },
                            ThreadSafeObject::Float(f) => {
                                *(result_ptr as *mut f64) = *f;
                            },
                            ThreadSafeObject::Boolean(b) => {
                                *(result_ptr as *mut bool) = *b;
                            },
                            ThreadSafeObject::String(s) => {
                                // String handling would need special care
                                tracing::warn!("String handling in thread-safe FFI is not fully implemented");
                            },
                            _ => {
                                tracing::warn!("Unhandled type in thread-safe FFI receive");
                            }
                        }
                    }
                    tracing::debug!("Value received from thread-safe channel successfully");
                } else {
                    tracing::error!("Failed to unwrap received value from thread-safe channel");
                }
            },
            Err(e) => {
                tracing::error!(error = ?e, "Failed to receive from thread-safe channel");
            }
        }
        
        // Put the box back so we don't drop it
        let _ = Box::into_raw(thread_safe_channel);
    } else {
        // Convert the raw channel pointer back to an Arc<RwLock<Channel>>
        let channel_ref = unsafe { Arc::from_raw(channel_ptr as *const RwLock<Channel>) };
        
        // Clone the Arc to keep it alive after this function returns
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
        
        // Instead of forgetting the Arc, clone it to maintain proper ownership
        let _ = Arc::into_raw(channel_ref.clone());
    }
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
    
    // Determine if we should use thread-safe channels
    let use_thread_safe = cfg!(feature = "concurrent_gc");
    
    if use_thread_safe {
        // Convert the raw channel pointer back to a ThreadSafeObject
        let thread_safe_channel = unsafe { Box::from_raw(channel_ptr as *mut ThreadSafeObject) };
        
        // Get a clone to keep it alive after this function returns
        let channel_clone = thread_safe_channel.clone();
        
        // Close the channel
        match channel_clone.channel_close() {
            Ok(_) => {
                tracing::debug!("Thread-safe channel closed successfully");
            },
            Err(e) => {
                tracing::error!(error = ?e, "Failed to close thread-safe channel");
            }
        }
        
        // Put the box back so we don't drop it
        let _ = Box::into_raw(thread_safe_channel);
    } else {
        // Convert the raw channel pointer back to an Arc<RwLock<Channel>>
        let channel_ref = unsafe { Arc::from_raw(channel_ptr as *const RwLock<Channel>) };
        
        // Close the channel
        let mut channel = channel_ref.write().unwrap();
        channel.close();
        
        // Instead of forgetting the Arc, clone it to maintain proper ownership
        let _ = Arc::into_raw(channel_ref.clone());
    }
}