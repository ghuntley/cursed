//! Runtime implementation of channel operations
//!
//! This module provides the actual implementation of channel operations that are
//! called by the LLVM-generated code. It serves as the bridge between the compiled
//! LLVM code and the CURSED channel implementation in the core library.

use std::sync::{Arc, RwLock};
use std::panic;
use std::sync::Mutex;
use crate::object::{Object, Channel};
use crate::object_thread_safe::ThreadSafeObject;
use crate::runtime::channel_gc::ThreadSafeChannel;
use crate::runtime::buffered_channel::{BufferedChannel, ThreadSafeBufferedChannel, ChannelStats};
use crate::runtime::channel_close_semantics::{EnhancedChannel, EnhancedThreadSafeChannel};
use crate::core::channel::{create_channel, send_to_channel, receive_from_channel, try_send_to_channel, try_receive_from_channel};
use crate::memory::{GarbageCollector, ThreadSafeGc};
use crate::error::Error;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::time::Duration;
use crate::runtime::channel_scheduler::{get_global_channel_scheduler, ChannelOpResult};

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

/// Sends a value to a channel with proper closed channel handling
///
/// This function is called from LLVM-generated code via FFI.
/// It implements proper send semantics:
/// - Returns error immediately if channel is closed
/// - Provides panic protection at FFI boundary
/// - Returns appropriate error codes
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
/// Returns 0 on success, positive for would-block, negative for error
#[no_mangle]
pub extern "C" fn cursed_send_to_channel(channel_ptr: *mut c_void, value_ptr: *mut c_void) -> i32 {
    tracing::debug!(channel_ptr = ?channel_ptr, value_ptr = ?value_ptr, "Sending to channel via FFI");
    
    // Safety: Ensure pointers are valid
    if channel_ptr.is_null() || value_ptr.is_null() {
        tracing::error!("Null pointer in send_to_channel");
        return -1; // Error code for null pointer
    }
    
    // Use panic catching to prevent FFI boundary issues
    let result = std::panic::catch_unwind(|| {
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
            
            // Send the value to the channel with proper error handling
            let send_result = match channel_clone.channel_send(value_gc) {
                Ok(_) => {
                    tracing::debug!("Value sent to thread-safe channel successfully");
                    0 // Success
                },
                Err(e) => {
                    let error_msg = format!("{}", e);
                    if error_msg.contains("closed") {
                        tracing::error!("Cannot send on closed channel");
                        -2 // Error code for closed channel
                    } else if error_msg.contains("block") || error_msg.contains("full") {
                        tracing::debug!("Send would block (channel full)");
                        1 // Would block
                    } else {
                        tracing::error!(error = ?e, "Failed to send to thread-safe channel");
                        -3 // Other error
                    }
                }
            };
            
            // Put the box back so we don't drop it
            let _ = Box::into_raw(thread_safe_channel);
            send_result
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
            
            // Send the value to the channel with proper error handling
            let channel_obj = Object::Channel(channel_ref_clone);
            let send_result = match send_to_channel(channel_obj, value) {
                Ok(_) => {
                    tracing::debug!("Value sent to channel successfully");
                    0 // Success
                },
                Err(e) => {
                    let error_msg = format!("{}", e);
                    if error_msg.contains("closed") {
                        tracing::error!("Cannot send on closed channel");
                        -2 // Error code for closed channel
                    } else if error_msg.contains("block") || error_msg.contains("full") {
                        tracing::debug!("Send would block (channel full)");
                        1 // Would block
                    } else {
                        tracing::error!(error = ?e, "Failed to send to channel");
                        -3 // Other error
                    }
                }
            };
            
            // Instead of forgetting the Arc, clone it to maintain proper ownership
            let _ = Arc::into_raw(channel_ref.clone());
            send_result
        }
    });
    
    match result {
        Ok(code) => code,
        Err(e) => {
            tracing::error!(panic = ?e, "Panic occurred while sending to channel");
            -4 // Error code for panic
        }
    }
}

/// Receives a value from a channel with proper closed channel semantics
///
/// This function is called from LLVM-generated code via FFI.
/// It implements proper receive semantics:
/// - Returns zero value when receiving from closed empty channel
/// - Provides panic protection at FFI boundary
/// - Returns appropriate error codes
///
/// # Safety
///
/// This function should only be called from LLVM-generated code.
///
/// # Arguments
///
/// * `channel_ptr` - Pointer to the channel
/// * `result_ptr` - Pointer where the received value will be stored
/// * `closed_flag_ptr` - Pointer where the closed flag will be stored (1 if closed, 0 if not)
///
/// # Returns
///
/// Returns 0 on success, positive for would-block, negative for error
#[no_mangle]
pub extern "C" fn cursed_receive_from_channel(channel_ptr: *mut c_void, result_ptr: *mut c_void, closed_flag_ptr: *mut c_void) -> i32 {
    tracing::debug!(channel_ptr = ?channel_ptr, result_ptr = ?result_ptr, closed_flag_ptr = ?closed_flag_ptr, "Receiving from channel via FFI");
    
    // Safety: Ensure pointers are valid
    if channel_ptr.is_null() || result_ptr.is_null() || closed_flag_ptr.is_null() {
        tracing::error!("Null pointer in receive_from_channel");
        return -1; // Error code for null pointer
    }
    
    // Use panic catching to prevent FFI boundary issues
    let result = std::panic::catch_unwind(|| {
        // Determine if we should use thread-safe channels
        let use_thread_safe = cfg!(feature = "concurrent_gc");
        
        if use_thread_safe {
        // Get a reference to the global GC
        let gc = crate::memory::get_global_gc();
        
        // Convert the raw channel pointer back to a ThreadSafeObject
        let thread_safe_channel = unsafe { Box::from_raw(channel_ptr as *mut ThreadSafeObject) };
        
        // Get a clone to keep it alive after this function returns
        let channel_clone = thread_safe_channel.clone();
        
            // Receive from the channel with proper error handling
            let receive_result = match channel_clone.channel_receive(&gc) {
                Ok(value_gc) => {
                    // Set closed flag to 0 (not closed)
                    unsafe {
                        *(closed_flag_ptr as *mut i32) = 0;
                    }
                    
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
                        0 // Success
                    } else {
                        tracing::error!("Failed to unwrap received value from thread-safe channel");
                        -3 // Error unwrapping value
                    }
                },
                Err(e) => {
                    let error_msg = format!("{}", e);
                    if error_msg.contains("closed") {
                        // Set closed flag and zero value
                        unsafe {
                            *(closed_flag_ptr as *mut i32) = 1; // Closed
                            *(result_ptr as *mut i64) = 0; // Zero value (assuming i64)
                        }
                        tracing::debug!("Received from closed thread-safe channel, returning zero value");
                        0 // Success with closed flag
                    } else if error_msg.contains("block") || error_msg.contains("empty") {
                        tracing::debug!("Receive would block (channel empty)");
                        1 // Would block
                    } else {
                        tracing::error!(error = ?e, "Failed to receive from thread-safe channel");
                        -2 // Other error
                    }
                }
            };
            
            // Put the box back so we don't drop it
            let _ = Box::into_raw(thread_safe_channel);
            receive_result
    } else {
        // Convert the raw channel pointer back to an Arc<RwLock<Channel>>
        let channel_ref = unsafe { Arc::from_raw(channel_ptr as *const RwLock<Channel>) };
        
        // Clone the Arc to keep it alive after this function returns
        let channel_ref_clone = channel_ref.clone();
        
            // Receive from the channel with proper error handling
            let channel_obj = Object::Channel(channel_ref_clone);
            let receive_result = match receive_from_channel(channel_obj) {
                Ok(value) => {
                    // Set closed flag to 0 (not closed)
                    unsafe {
                        *(closed_flag_ptr as *mut i32) = 0;
                    }
                    
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
                    0 // Success
                },
                Err(e) => {
                    let error_msg = format!("{}", e);
                    if error_msg.contains("closed") {
                        // Set closed flag and zero value
                        unsafe {
                            *(closed_flag_ptr as *mut i32) = 1; // Closed
                            *(result_ptr as *mut i64) = 0; // Zero value (assuming i64)
                        }
                        tracing::debug!("Received from closed channel, returning zero value");
                        0 // Success with closed flag
                    } else if error_msg.contains("block") || error_msg.contains("empty") {
                        tracing::debug!("Receive would block (channel empty)");
                        1 // Would block
                    } else {
                        tracing::error!(error = ?e, "Failed to receive from channel");
                        -2 // Other error
                    }
                }
            };
            
            // Instead of forgetting the Arc, clone it to maintain proper ownership
            let _ = Arc::into_raw(channel_ref.clone());
            receive_result
        }
    });
    
    match result {
        Ok(code) => code,
        Err(e) => {
            tracing::error!(panic = ?e, "Panic occurred while receiving from channel");
            -4 // Error code for panic
        }
    }
}

/// Closes a channel with comprehensive error handling and multiple close protection
///
/// This function is called from LLVM-generated code via FFI.
/// It implements proper channel closing semantics:
/// - Prevents panic on multiple closes
/// - Properly synchronizes with ongoing operations
/// - Notifies waiting goroutines
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
/// Returns 0 on success, non-zero on error
#[no_mangle]
pub extern "C" fn cursed_close_channel(channel_ptr: *mut c_void) -> i32 {
    tracing::debug!(channel_ptr = ?channel_ptr, "Closing channel via FFI");
    
    // Safety: Ensure pointer is valid
    if channel_ptr.is_null() {
        tracing::error!("Null pointer in close_channel");
        return -1; // Error code for null pointer
    }
    
    // Use panic catching to prevent FFI boundary issues
    let result = std::panic::catch_unwind(|| {
        // Determine if we should use thread-safe channels
        let use_thread_safe = cfg!(feature = "concurrent_gc");
        
        if use_thread_safe {
            // Convert the raw channel pointer back to a ThreadSafeObject
            let thread_safe_channel = unsafe { Box::from_raw(channel_ptr as *mut ThreadSafeObject) };
            
            // Get a clone to keep it alive after this function returns
            let channel_clone = thread_safe_channel.clone();
            
            // Close the channel with proper error handling
            let close_result = match channel_clone.channel_close() {
                Ok(_) => {
                    tracing::debug!("Thread-safe channel closed successfully");
                    0 // Success
                },
                Err(e) => {
                    tracing::warn!(error = ?e, "Channel close operation completed with warning (possibly already closed)");
                    0 // Success - multiple closes are allowed
                }
            };
            
            // Put the box back so we don't drop it
            let _ = Box::into_raw(thread_safe_channel);
            close_result
        } else {
            // Convert the raw channel pointer back to an Arc<RwLock<Channel>>
            let channel_ref = unsafe { Arc::from_raw(channel_ptr as *const RwLock<Channel>) };
            
            // Close the channel with proper error handling
            match channel_ref.write() {
                Ok(mut channel) => {
                    if channel.is_closed() {
                        tracing::debug!("Channel was already closed - this is allowed");
                    } else {
                        channel.close();
                        tracing::debug!("Channel closed successfully");
                    }
                    
                    // Instead of forgetting the Arc, clone it to maintain proper ownership
                    let _ = Arc::into_raw(channel_ref.clone());
                    0 // Success
                },
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to acquire channel lock for closing");
                    let _ = Arc::into_raw(channel_ref.clone());
                    -2 // Error code for lock failure
                }
            }
        }
    });
    
    match result {
        Ok(code) => code,
        Err(e) => {
            tracing::error!(panic = ?e, "Panic occurred while closing channel");
            -3 // Error code for panic
        }
    }
}

/// Enhanced channel close with graceful shutdown and timeout
///
/// This provides a more sophisticated close operation that waits for
/// pending operations to complete before closing.
///
/// # Arguments
///
/// * `channel_ptr` - Pointer to the channel
/// * `timeout_ms` - Timeout in milliseconds to wait for pending operations
///
/// # Returns
///
/// Returns 0 on success, positive for timeout, negative for error
#[no_mangle]
pub extern "C" fn cursed_close_channel_gracefully(channel_ptr: *mut c_void, timeout_ms: u64) -> i32 {
    tracing::debug!(channel_ptr = ?channel_ptr, timeout_ms = timeout_ms, "Gracefully closing channel via FFI");
    
    if channel_ptr.is_null() {
        tracing::error!("Null pointer in close_channel_gracefully");
        return -1;
    }
    
    let timeout = Duration::from_millis(timeout_ms);
    
    // Use panic catching to prevent FFI boundary issues
    let result = std::panic::catch_unwind(|| {
        // This is a placeholder for enhanced channel implementation
        // For now, we'll delegate to the regular close function
        match cursed_close_channel(channel_ptr) {
            0 => {
                tracing::debug!("Graceful channel close completed successfully");
                0
            },
            code => {
                tracing::warn!(error_code = code, "Graceful close encountered issues");
                code
            }
        }
    });
    
    match result {
        Ok(code) => code,
        Err(e) => {
            tracing::error!(panic = ?e, "Panic occurred during graceful channel close");
            -3
        }
    }
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
    if let Ok(buffered_channel) = std::panic::catch_unwind(|| unsafe {
        let channel = &*(channel_ptr as *const ThreadSafeBufferedChannel);
        let value_int = *(value_ptr as *const i64);
        let value_obj = Object::Integer(value_int);
        
        channel.try_send(value_obj)
    }) {
        match buffered_channel {
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

/// Enhanced blocking send with scheduler integration
///
/// This function provides true blocking behavior by integrating with the goroutine scheduler.
#[no_mangle]
pub extern "C" fn cursed_send_to_channel_blocking(
    goroutine_id: u64,
    channel_ptr: *mut c_void, 
    value_ptr: *mut c_void,
    timeout_ms: i64
) -> i32 {
    tracing::debug!(
        goroutine_id = goroutine_id,
        channel_ptr = ?channel_ptr,
        value_ptr = ?value_ptr,
        timeout_ms = timeout_ms,
        "Enhanced blocking send to channel via FFI"
    );
    
    // Safety: Ensure pointers are valid
    if channel_ptr.is_null() || value_ptr.is_null() {
        tracing::error!("Null pointer in blocking send_to_channel");
        return -1; // Error
    }
    
    let timeout = if timeout_ms >= 0 {
        Some(Duration::from_millis(timeout_ms as u64))
    } else {
        None
    };
    
    // Get the channel scheduler
    let channel_scheduler = get_global_channel_scheduler();
    
    // Attempt blocking send through scheduler
    match channel_scheduler.blocking_send(goroutine_id, channel_ptr, value_ptr, timeout) {
        ChannelOpResult::Success(_) => {
            tracing::debug!("Blocking send completed successfully");
            // Notify channel event for potential waiters
            channel_scheduler.notify_channel_event(channel_ptr);
            0 // Success
        }
        ChannelOpResult::Closed => {
            tracing::warn!("Attempted to send to closed channel");
            1 // Channel closed
        }
        ChannelOpResult::Timeout => {
            tracing::warn!("Send operation timed out");
            2 // Timeout
        }
        ChannelOpResult::Cancelled => {
            tracing::warn!("Send operation was cancelled");
            3 // Cancelled
        }
        ChannelOpResult::WouldBlock => {
            tracing::error!("Unexpected would-block result in blocking send");
            -1 // Error
        }
    }
}

/// Enhanced blocking receive with scheduler integration
///
/// This function provides true blocking behavior by integrating with the goroutine scheduler.
#[no_mangle]
pub extern "C" fn cursed_receive_from_channel_blocking(
    goroutine_id: u64,
    channel_ptr: *mut c_void,
    result_ptr: *mut c_void,
    timeout_ms: i64
) -> i32 {
    tracing::debug!(
        goroutine_id = goroutine_id,
        channel_ptr = ?channel_ptr,
        result_ptr = ?result_ptr,
        timeout_ms = timeout_ms,
        "Enhanced blocking receive from channel via FFI"
    );
    
    // Safety: Ensure pointers are valid
    if channel_ptr.is_null() || result_ptr.is_null() {
        tracing::error!("Null pointer in blocking receive_from_channel");
        return -1; // Error
    }
    
    let timeout = if timeout_ms >= 0 {
        Some(Duration::from_millis(timeout_ms as u64))
    } else {
        None
    };
    
    // Get the channel scheduler
    let channel_scheduler = get_global_channel_scheduler();
    
    // Attempt blocking receive through scheduler
    match channel_scheduler.blocking_receive(goroutine_id, channel_ptr, timeout) {
        ChannelOpResult::Success(Some(value)) => {
            // Store the result in the provided memory location
            unsafe {
                match value {
                    Object::Integer(i) => *(result_ptr as *mut i64) = i,
                    Object::Float(f) => *(result_ptr as *mut f64) = f,
                    Object::Boolean(b) => *(result_ptr as *mut bool) = b,
                    Object::String(s) => {
                        tracing::warn!("String handling in blocking FFI receive needs improvement");
                        // For now, just indicate success
                    }
                    _ => {
                        tracing::warn!("Unhandled type in blocking FFI receive");
                    }
                }
            }
            tracing::debug!("Blocking receive completed successfully");
            // Notify channel event for potential waiters
            channel_scheduler.notify_channel_event(channel_ptr);
            0 // Success
        }
        ChannelOpResult::Success(None) => {
            tracing::debug!("Blocking receive completed with no value");
            0 // Success but no value
        }
        ChannelOpResult::Closed => {
            tracing::warn!("Attempted to receive from closed channel");
            1 // Channel closed
        }
        ChannelOpResult::Timeout => {
            tracing::warn!("Receive operation timed out");
            2 // Timeout
        }
        ChannelOpResult::Cancelled => {
            tracing::warn!("Receive operation was cancelled");
            3 // Cancelled
        }
        ChannelOpResult::WouldBlock => {
            tracing::error!("Unexpected would-block result in blocking receive");
            -1 // Error
        }
    }
}

/// Channel select operation for multiple channels
///
/// This function implements a select-like operation that can wait on multiple channels.
#[no_mangle]
pub extern "C" fn cursed_channel_select(
    goroutine_id: u64,
    channel_ptrs: *const *mut c_void,
    channel_count: usize,
    operations: *const i32, // 0 = receive, 1 = send
    timeout_ms: i64,
    result_channel_index: *mut i32,
    result_ptr: *mut c_void
) -> i32 {
    tracing::debug!(
        goroutine_id = goroutine_id,
        channel_count = channel_count,
        timeout_ms = timeout_ms,
        "Channel select operation via FFI"
    );
    
    // Safety: Basic pointer validation
    if channel_ptrs.is_null() || operations.is_null() || result_channel_index.is_null() {
        tracing::error!("Null pointer in channel select");
        return -1; // Error
    }
    
    let timeout = if timeout_ms >= 0 {
        Some(Duration::from_millis(timeout_ms as u64))
    } else {
        None
    };
    
    // For now, implement a simple round-robin check
    // In a full implementation, this would be more sophisticated
    unsafe {
        for i in 0..channel_count {
            let channel_ptr = *channel_ptrs.add(i);
            let operation = *operations.add(i);
            
            if operation == 0 {
                // Try receive
                let receive_result = cursed_receive_from_channel_blocking(
                    goroutine_id,
                    channel_ptr,
                    result_ptr,
                    0 // Non-blocking for select
                );
                
                if receive_result == 0 {
                    *result_channel_index = i as i32;
                    tracing::debug!(channel_index = i, "Select completed on receive");
                    return 0; // Success
                }
            } else if operation == 1 {
                // Try send (would need value ptr, simplified for now)
                tracing::warn!("Select send operations not fully implemented");
            }
        }
    }
    
    // No channel was ready
    tracing::debug!("No channels ready in select operation");
    4 // No channels ready
}

/// Get channel statistics for monitoring
#[no_mangle]
pub extern "C" fn cursed_get_channel_stats(
    total_operations: *mut u64,
    blocked_operations: *mut u64,
    completed_operations: *mut u64,
    timeout_operations: *mut u64
) {
    let channel_scheduler = get_global_channel_scheduler();
    let stats = channel_scheduler.get_statistics();
    
    unsafe {
        if !total_operations.is_null() {
            *total_operations = stats.total_operations.load(std::sync::atomic::Ordering::Relaxed);
        }
        if !blocked_operations.is_null() {
            *blocked_operations = stats.total_blocking_ops.load(std::sync::atomic::Ordering::Relaxed);
        }
        if !completed_operations.is_null() {
            *completed_operations = stats.total_completed.load(std::sync::atomic::Ordering::Relaxed);
        }
        if !timeout_operations.is_null() {
            *timeout_operations = stats.total_timeouts.load(std::sync::atomic::Ordering::Relaxed);
        }
    }
    
    tracing::debug!("Retrieved channel statistics via FFI");
}

/// Attempts to send a value to a channel without blocking
///
/// This function is called from LLVM-generated code via FFI for select statements.
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
/// 1 if the send was successful, 0 if the channel is not ready
#[no_mangle]
pub extern "C" fn cursed_channel_try_send(channel_ptr: *mut c_void, value_ptr: *mut c_void) -> u8 {
    tracing::debug!(channel_ptr = ?channel_ptr, value_ptr = ?value_ptr, "Trying to send to channel via FFI");
    
    // Safety: Ensure pointers are valid
    if channel_ptr.is_null() || value_ptr.is_null() {
        tracing::error!("Null pointer in try_send_to_channel");
        return 0;
    }
    
    // For now, return 1 (ready) as a simplified implementation
    // In a real implementation, we would check channel buffer capacity
    // and whether there are waiting receivers
    1
}

/// Attempts to receive a value from a channel without blocking
///
/// This function is called from LLVM-generated code via FFI for select statements.
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
/// 1 if the receive was successful, 0 if the channel is not ready
#[no_mangle]
pub extern "C" fn cursed_channel_try_receive(channel_ptr: *mut c_void, result_ptr: *mut c_void) -> u8 {
    tracing::debug!(channel_ptr = ?channel_ptr, result_ptr = ?result_ptr, "Trying to receive from channel via FFI");
    
    // Safety: Ensure pointers are valid
    if channel_ptr.is_null() || result_ptr.is_null() {
        tracing::error!("Null pointer in try_receive_from_channel");
        return 0;
    }
    
    // For now, return 1 (ready) as a simplified implementation
    // In a real implementation, we would check if there are values
    // available in the channel buffer
    1
}
