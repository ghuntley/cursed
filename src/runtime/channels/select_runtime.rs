//! Runtime integration for select/ready statements
//!
//! This module provides the C-compatible runtime functions that are called
//! by the LLVM codegen for select statement execution.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::ffi::c_void;
use std::any::Any;

use crate::runtime::channels::{ChannelError, ChannelResult, SendResult, ReceiveResult};
use crate::runtime::channels::select::{Select, SelectResult, SelectOperation};
use crate::runtime::channels::buffer::ChannelBuffer;
use crate::runtime::channels::channel::Channel;
use crate::runtime::goroutine::GoroutineId;

/// Select context for runtime execution
pub struct SelectContext {
    /// Internal select builder
    select: Select,
    /// Channel registry for the select operation
    channels: HashMap<usize, Arc<dyn Any + Send + Sync>>,
    /// Case operations
    cases: Vec<(usize, SelectOperation, Option<Box<dyn Any + Send>>)>,
    /// Next channel ID
    next_channel_id: AtomicUsize,
}

impl SelectContext {
    pub fn new(num_cases: usize) -> Self {
        Self {
            select: Select::new(),
            channels: HashMap::with_capacity(num_cases),
            cases: Vec::with_capacity(num_cases),
            next_channel_id: AtomicUsize::new(0),
        }
    }

    pub fn add_case(&mut self, channel_ptr: *mut c_void, operation_type: i32, value_ptr: *mut c_void) -> Result<i32, ChannelError> {
        let channel_id = self.next_channel_id.fetch_add(1, Ordering::SeqCst);
        
        // Convert channel pointer to our channel type
        // This is a simplified implementation - in a real system we'd need proper type checking
        let channel = unsafe {
            let channel_raw = channel_ptr as *mut Channel<i64>;
            if channel_raw.is_null() {
                return Err(ChannelError::NoSenders);
            }
            Arc::from_raw(channel_raw)
        };

        self.channels.insert(channel_id, channel.clone());

        match operation_type {
            0 => {
                // Receive operation
                let operation = SelectOperation::Receive {
                    channel_id,
                    case_index: self.cases.len(),
                };
                self.cases.push((channel_id, operation, None));
                
                // Add to select builder - use public buffer method
                self.select.receive(channel_id, channel.get_buffer());
            }
            1 => {
                // Send operation
                let value = if value_ptr.is_null() {
                    return Err(ChannelError::NoSenders);
                } else {
                    unsafe {
                        let value_raw = value_ptr as *mut i64;
                        *value_raw
                    }
                };
                
                let operation = SelectOperation::Send {
                    channel_id,
                    case_index: self.cases.len(),
                };
                self.cases.push((channel_id, operation, Some(Box::new(value))));
                
                // Add to select builder - use public buffer method
                self.select.send(channel_id, channel.get_buffer(), value);
            }
            -1 => {
                // Default case
                let operation = SelectOperation::Default {
                    case_index: self.cases.len(),
                };
                self.cases.push((channel_id, operation, None));
                
                // Add to select builder
                self.select.default_case();
            }
            _ => return Err(ChannelError::NoSenders),
        }

        Ok(self.cases.len() as i32 - 1)
    }

    pub fn execute(&mut self, has_default: bool) -> Result<i32, ChannelError> {
        match self.select.execute()? {
            SelectResult::SendCompleted(case_index) => Ok(case_index as i32),
            SelectResult::ReceiveCompleted(case_index, _value) => Ok(case_index as i32),
            SelectResult::DefaultExecuted => Ok(-1),
            SelectResult::Timeout => Ok(-2),
            SelectResult::AllClosed => Ok(-3),
        }
    }
}

/// Global select context registry
static SELECT_CONTEXTS: std::sync::LazyLock<Mutex<HashMap<usize, SelectContext>>> = std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));
static NEXT_SELECT_ID: AtomicUsize = AtomicUsize::new(1);

/// C-compatible runtime functions for LLVM codegen

/// Prepare a new select context
#[no_mangle]
pub extern "C" fn cursed_select_prepare(num_cases: i32) -> *mut c_void {
    let select_id = NEXT_SELECT_ID.fetch_add(1, Ordering::SeqCst);
    let context = SelectContext::new(num_cases as usize);
    
    if let Ok(mut contexts) = SELECT_CONTEXTS.lock() {
        contexts.insert(select_id, context);
        select_id as *mut c_void
    } else {
        std::ptr::null_mut()
    }
}

/// Add a case to the select context
#[no_mangle]
pub extern "C" fn cursed_select_add_case(
    select_ctx: *mut c_void,
    channel_ptr: *mut c_void,
    operation_type: i32,
    value_ptr: *mut c_void,
) -> i32 {
    let select_id = select_ctx as usize;
    
    if let Ok(mut contexts) = SELECT_CONTEXTS.lock() {
        if let Some(context) = contexts.get_mut(&select_id) {
            match context.add_case(channel_ptr, operation_type, value_ptr) {
                Ok(case_index) => case_index,
                Err(_) => -1,
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

/// Execute the select operation
#[no_mangle]
pub extern "C" fn cursed_select_execute(select_ctx: *mut c_void, has_default: bool) -> i32 {
    let select_id = select_ctx as usize;
    
    if let Ok(mut contexts) = SELECT_CONTEXTS.lock() {
        if let Some(context) = contexts.get_mut(&select_id) {
            match context.execute(has_default) {
                Ok(case_index) => case_index,
                Err(_) => -1,
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

/// Cleanup the select context
#[no_mangle]
pub extern "C" fn cursed_select_cleanup(select_ctx: *mut c_void) {
    let select_id = select_ctx as usize;
    
    if let Ok(mut contexts) = SELECT_CONTEXTS.lock() {
        contexts.remove(&select_id);
    }
}

/// Get the result value from a receive operation
#[no_mangle]
pub extern "C" fn cursed_select_get_receive_value(select_ctx: *mut c_void, case_index: i32) -> *mut c_void {
    let select_id = select_ctx as usize;
    
    if let Ok(contexts) = SELECT_CONTEXTS.lock() {
        if let Some(context) = contexts.get(&select_id) {
            // This is a simplified implementation
            // In a real system, we'd need to store the received values
            std::ptr::null_mut()
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    }
}

/// Create a channel at runtime
#[no_mangle]
pub extern "C" fn cursed_channel_create(buffer_size: i32) -> *mut c_void {
    let channel = if buffer_size == 0 {
        Channel::<i64>::new()
    } else {
        Channel::<i64>::with_capacity(buffer_size as usize)
    };
    
    Box::into_raw(Box::new(channel)) as *mut c_void
}

/// Send a value to a channel
#[no_mangle]
pub extern "C" fn cursed_channel_send(channel_ptr: *mut c_void, value: i64) -> i32 {
    if channel_ptr.is_null() {
        return -1;
    }
    
    let channel = unsafe { &*(channel_ptr as *const Channel<i64>) };
    match channel.send(value) {
        SendResult::Sent => 0,
        SendResult::Closed(_) => -1,
        SendResult::WouldBlock(_) => -2,
    }
}

/// Receive a value from a channel
#[no_mangle]
pub extern "C" fn cursed_channel_receive(channel_ptr: *mut c_void, value_out: *mut i64) -> i32 {
    if channel_ptr.is_null() || value_out.is_null() {
        return -1;
    }
    
    let channel = unsafe { &*(channel_ptr as *const Channel<i64>) };
    match channel.recv() {
        ReceiveResult::Received(value) => {
            unsafe {
                *value_out = value;
            }
            0
        }
        ReceiveResult::Closed => -1,
        ReceiveResult::WouldBlock => -2,
    }
}

/// Try to receive a value from a channel (non-blocking)
#[no_mangle]
pub extern "C" fn cursed_channel_try_receive(channel_ptr: *mut c_void, value_out: *mut i64) -> i32 {
    if channel_ptr.is_null() || value_out.is_null() {
        return -1;
    }
    
    let channel = unsafe { &*(channel_ptr as *const Channel<i64>) };
    match channel.try_recv() {
        ReceiveResult::Received(value) => {
            unsafe {
                *value_out = value;
            }
            0
        }
        ReceiveResult::Closed => -1,
        ReceiveResult::WouldBlock => -2,
    }
}

/// Close a channel
#[no_mangle]
pub extern "C" fn cursed_channel_close(channel_ptr: *mut c_void) {
    if channel_ptr.is_null() {
        return;
    }
    
    let channel = unsafe { &*(channel_ptr as *const Channel<i64>) };
    channel.close();
}

/// Destroy a channel
#[no_mangle]
pub extern "C" fn cursed_channel_destroy(channel_ptr: *mut c_void) {
    if channel_ptr.is_null() {
        return;
    }
    
    unsafe {
        let _ = Box::from_raw(channel_ptr as *mut Channel<i64>);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_select_prepare() {
        let ctx = cursed_select_prepare(2);
        assert!(!ctx.is_null());
        
        cursed_select_cleanup(ctx);
    }

    #[test]
    fn test_channel_create() {
        let channel = cursed_channel_create(0);
        assert!(!channel.is_null());
        
        cursed_channel_destroy(channel);
    }

    #[test]
    fn test_channel_send_receive() {
        let channel = cursed_channel_create(1);
        assert!(!channel.is_null());
        
        let result = cursed_channel_send(channel, 42);
        assert_eq!(result, 0);
        
        let mut value = 0;
        let result = cursed_channel_receive(channel, &mut value);
        assert_eq!(result, 0);
        assert_eq!(value, 42);
        
        cursed_channel_destroy(channel);
    }
}
