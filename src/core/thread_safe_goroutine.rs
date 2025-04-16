//! Thread-safe goroutine implementation
//!
//! This module implements the thread-safe goroutine system for CURSED,
//! providing functions to run code concurrently with proper synchronization.
//! It uses the ThreadSafeCallable and ThreadSafeObject types to ensure
//! memory safety across thread boundaries.

use crate::error::Error;
use crate::object_thread_safe::{ThreadSafeCallable, ThreadSafeObject, ThreadSafeValue, convert_to_thread_safe, convert_from_thread_safe};
use crate::object::Object;
use crate::memory::ThreadSafeGc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use tracing::{debug, error, info, warn, instrument};

/// Global state for goroutine management
#[derive(Default)]
pub struct GoroutineState {
    /// Count of active goroutines
    active_count: usize,
    /// Channels for communication between goroutines
    channels: std::collections::HashMap<String, Arc<Mutex<Vec<ThreadSafeValue>>>>,
}

/// Lazy-initialized global goroutine state
lazy_static::lazy_static! {
    static ref GOROUTINE_STATE: Arc<RwLock<GoroutineState>> = Arc::new(RwLock::new(GoroutineState::default()));
}

/// Run a function in a new goroutine (thread)
///
/// This function creates a new thread to execute the given callable with the
/// provided arguments. It updates the global goroutine state to track active
/// goroutines and handles proper cleanup when the goroutine completes.
///
/// # Arguments
///
/// * `func` - The thread-safe callable to execute
/// * `args` - The thread-safe arguments to pass to the callable
///
/// # Returns
///
/// Result<(), Error> - Ok if goroutine was launched successfully, Error otherwise
#[instrument(skip(func, args), fields(args_count = args.len()), level = "info")]
pub fn run_goroutine(
    func: Arc<dyn ThreadSafeCallable>,
    args: Vec<ThreadSafeObject>,
) -> Result<(), Error> {
    debug!("Launching thread-safe goroutine");
    
    // Extract ThreadSafeValue from ThreadSafeObject for the call
    let safe_args: Vec<ThreadSafeValue> = args.iter()
        .map(|arg| arg.get())
        .collect();
    
    // Increment active goroutine count
    {
        let mut state = GOROUTINE_STATE.write().unwrap();
        state.active_count += 1;
        debug!(active_count = state.active_count, "Incremented active goroutine count");
    }
    
    // Create clones for the new thread
    let state_arc = Arc::clone(&GOROUTINE_STATE);
    let func_clone = Arc::clone(&func);
    
    // Spawn a new thread to execute the callable
    thread::spawn(move || {
        info!("Thread-safe goroutine started");
        
        // Call the function with the provided arguments
        let result = func_clone.call(safe_args);
        
        // Log the result
        match &result {
            Ok(_) => debug!("Thread-safe goroutine completed successfully"),
            Err(e) => error!(error = ?e, "Thread-safe goroutine execution failed"),
        }
        
        // Decrement active goroutine count
        let mut state = state_arc.write().unwrap();
        state.active_count -= 1;
        debug!(active_count = state.active_count, "Decremented active goroutine count");
        
        // Return the result (though it's discarded in this thread)
        result
    });

    Ok(())
}

/// Run a simple closure as a goroutine
/// 
/// This is a simpler interface for running closure-based goroutines
/// 
/// # Arguments
/// 
/// * `f` - The closure to execute as a goroutine
/// 
/// # Returns
/// 
/// Result<(), Error> - Ok if goroutine was launched successfully
#[instrument(skip(f), level = "debug")]
pub fn run_goroutine_fn<F>(f: F) -> Result<(), Error>
where 
    F: FnOnce() + Send + 'static 
{
    // Increment active goroutine count
    {
        let mut state = GOROUTINE_STATE.write().unwrap();
        state.active_count += 1;
        debug!(active_count = state.active_count, "Incremented active goroutine count");
    }
    
    // Create clone for the new thread
    let state_arc = Arc::clone(&GOROUTINE_STATE);
    
    // Spawn a new thread to execute the closure
    thread::spawn(move || {
        info!("Closure-based goroutine started");
        
        // Execute the closure
        f();
        
        // Decrement active goroutine count
        let mut state = state_arc.write().unwrap();
        state.active_count -= 1;
        debug!(active_count = state.active_count, "Decremented active goroutine count");
        
        // No result to return
    });
    
    Ok(())
}

/// Get the number of active goroutines
pub fn active_goroutine_count() -> usize {
    let state = GOROUTINE_STATE.read().unwrap();
    state.active_count
}

/// Wait for all goroutines to complete
///
/// This function blocks until all active goroutines have completed
/// or until the timeout is reached.
///
/// # Arguments
///
/// * `timeout_ms` - Maximum time to wait in milliseconds (0 means wait indefinitely)
///
/// # Returns
///
/// Result<(), Error> - Ok if all goroutines completed, Error if timeout reached
#[instrument(level = "debug")]
pub fn wait_all_goroutines(timeout_ms: u64) -> Result<(), Error> {
    debug!(timeout_ms = timeout_ms, "Waiting for all goroutines");
    
    let start_time = std::time::Instant::now();
    
    loop {
        // Check if all goroutines are done
        let count = active_goroutine_count();
        if count == 0 {
            debug!("All goroutines completed");
            return Ok(());
        }
        
        // Check if we've exceeded the timeout
        if timeout_ms > 0 && start_time.elapsed().as_millis() > timeout_ms as u128 {
            return Err(Error::from_str(&format!(
                "Timeout waiting for goroutines to complete, {} still active",
                count
            )));
        }
        
        // Sleep briefly before checking again
        thread::sleep(Duration::from_millis(10));
    }
}

/// Create a new channel for goroutine communication
/// 
/// Channels allow for safe communication between goroutines. This creates
/// a named channel that can be used to send and receive values.
/// 
/// # Arguments
/// 
/// * `name` - The name of the channel to create
/// * `capacity` - Optional buffer capacity (0 for unbuffered)
/// 
/// # Returns
/// 
/// Result<(), Error> - Ok if channel was created, Error if a channel with that name already exists
#[instrument(level = "debug")]
pub fn create_channel(name: &str, capacity: usize) -> Result<(), Error> {
    let mut state = GOROUTINE_STATE.write().unwrap();
    
    if state.channels.contains_key(name) {
        return Err(Error::from_str(&format!("Channel {} already exists", name)));
    }
    
    // Create a new channel with the specified capacity
    // Actual implementation uses a mutex-protected vector as a simple queue
    let channel = Arc::new(Mutex::new(Vec::with_capacity(capacity)));
    state.channels.insert(name.to_string(), channel);
    
    debug!(name = name, capacity = capacity, "Created new channel");
    Ok(())
}

/// Send a value on a channel
/// 
/// This adds a value to the specified channel, allowing other goroutines to receive it.
/// 
/// # Arguments
/// 
/// * `name` - The name of the channel to send on
/// * `value` - The thread-safe value to send
/// 
/// # Returns
/// 
/// Result<(), Error> - Ok if value was sent, Error if the channel doesn't exist
#[instrument(skip(value), level = "debug")]
pub fn send_on_channel(name: &str, value: ThreadSafeValue) -> Result<(), Error> {
    let state = GOROUTINE_STATE.read().unwrap();
    
    if let Some(channel) = state.channels.get(name) {
        let mut channel_guard = channel.lock().unwrap();
        channel_guard.push(value);
        debug!(name = name, "Sent value on channel");
        Ok(())
    } else {
        Err(Error::from_str(&format!("Channel {} does not exist", name)))
    }
}

/// Receive a value from a channel
/// 
/// This retrieves a value from the specified channel if one is available.
/// 
/// # Arguments
/// 
/// * `name` - The name of the channel to receive from
/// * `blocking` - Whether to block until a value is available
/// * `timeout_ms` - If blocking, maximum time to wait in milliseconds (0 for indefinite)
/// 
/// # Returns
/// 
/// Result<ThreadSafeValue, Error> - The received value or an error
#[instrument(level = "debug")]
pub fn receive_from_channel(
    name: &str, 
    blocking: bool, 
    timeout_ms: u64
) -> Result<ThreadSafeValue, Error> {
    // First check if the channel exists and get a clone of the channel arc
    let channel_option = {
        let state = GOROUTINE_STATE.read().unwrap();
        state.channels.get(name).cloned()
    };
    
    if let Some(channel) = channel_option {
        
        if blocking {
            // Blocking receive with timeout
            let start_time = std::time::Instant::now();
            
            loop {
                // Try to get a value
                let mut channel_guard = channel.lock().unwrap();
                
                if !channel_guard.is_empty() {
                    // Channel has a value, remove and return it
                    let value = channel_guard.remove(0);
                    debug!(name = name, "Received value from channel");
                    return Ok(value);
                }
                
                // No value yet, check timeout if specified
                if timeout_ms > 0 && start_time.elapsed().as_millis() > timeout_ms as u128 {
                    return Err(Error::from_str(&format!(
                        "Timeout waiting for value on channel {}",
                        name
                    )));
                }
                
                // Release the lock and sleep briefly before trying again
                drop(channel_guard);
                thread::sleep(Duration::from_millis(10));
            }
        } else {
            // Non-blocking receive
            let mut channel_guard = channel.lock().unwrap();
            
            if !channel_guard.is_empty() {
                // Channel has a value, remove and return it
                let value = channel_guard.remove(0);
                debug!(name = name, "Received value from channel (non-blocking)");
                Ok(value)
            } else {
                // No value available
                Err(Error::from_str(&format!("No value available on channel {}", name)))
            }
        }
    } else {
        Err(Error::from_str(&format!("Channel {} does not exist", name)))
    }
}

/// Send a regular Object value on a channel, converting it to thread-safe form
/// 
/// This is a convenience function that converts a regular Object to a thread-safe
/// form and sends it on the specified channel.
/// 
/// # Arguments
/// 
/// * `name` - The name of the channel to send on
/// * `value` - The Object value to send
/// 
/// # Returns
/// 
/// Result<(), Error> - Ok if value was sent, Error otherwise
#[instrument(skip(value), level = "debug")]
pub fn send_object_on_channel(name: &str, value: &Object) -> Result<(), Error> {
    // Convert the Object to a ThreadSafeValue
    let thread_safe_value = convert_to_thread_safe(value)?;
    
    // Send the converted value on the channel
    send_on_channel(name, thread_safe_value)
}

/// Receive an Object value from a channel, converting it from thread-safe form
/// 
/// This is a convenience function that receives a thread-safe value from
/// the specified channel and converts it to a regular Object.
/// 
/// # Arguments
/// 
/// * `name` - The name of the channel to receive from
/// * `blocking` - Whether to block until a value is available
/// * `timeout_ms` - If blocking, maximum time to wait in milliseconds
/// 
/// # Returns
/// 
/// Result<Object, Error> - The received Object value or an error
#[instrument(level = "debug")]
pub fn receive_object_from_channel(
    name: &str, 
    blocking: bool, 
    timeout_ms: u64
) -> Result<Object, Error> {
    // Receive a thread-safe value from the channel
    let thread_safe_value = receive_from_channel(name, blocking, timeout_ms)?;
    
    // Convert the ThreadSafeValue to an Object
    convert_from_thread_safe(&thread_safe_value)
}

/// Initialize the runtime for goroutines
///
/// This function sets up any global state needed for goroutine management.
#[instrument(level = "info")]
pub fn init_goroutine_runtime() {
    info!("Initializing goroutine runtime");
    
    // Reset the goroutine state
    let mut state = GOROUTINE_STATE.write().unwrap();
    state.active_count = 0;
    state.channels.clear();
}
