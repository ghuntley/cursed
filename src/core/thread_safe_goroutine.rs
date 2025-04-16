//! Thread-safe goroutine implementation
//!
//! This module implements the thread-safe goroutine system for CURSED,
//! providing functions to run code concurrently with proper synchronization.
//! It uses the ThreadSafeCallable and ThreadSafeObject types to ensure
//! memory safety across thread boundaries.

use crate::error::Error;
use crate::object_thread_safe::{ThreadSafeCallable, ThreadSafeObject, ThreadSafeValue};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Global state for goroutine management
#[derive(Default)]
pub struct GoroutineState {
    /// Count of active goroutines
    active_count: usize,
}

/// Lazy-initialized global goroutine state
lazy_static::lazy_static! {
    static ref GOROUTINE_STATE: Arc<Mutex<GoroutineState>> = Arc::new(Mutex::new(GoroutineState::default()));
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
#[tracing::instrument(skip(func, args), fields(args_count = args.len()), level = "info")]
pub fn run_goroutine(
    func: Arc<dyn ThreadSafeCallable>,
    args: Vec<ThreadSafeObject>,
) -> Result<(), Error> {
    tracing::debug!("Launching thread-safe goroutine");
    
    // Extract ThreadSafeValue from ThreadSafeObject for the call
    let safe_args: Vec<ThreadSafeValue> = args.iter()
        .map(|arg| arg.get())
        .collect();
    
    // Increment active goroutine count
    {
        let mut state = GOROUTINE_STATE.lock().unwrap();
        state.active_count += 1;
        tracing::debug!(active_count = state.active_count, "Incremented active goroutine count");
    }
    
    // Create clones for the new thread
    let state_arc = Arc::clone(&GOROUTINE_STATE);
    let func_clone = Arc::clone(&func);
    
    // Spawn a new thread to execute the callable
    thread::spawn(move || {
        tracing::info!("Thread-safe goroutine started");
        
        // Call the function with the provided arguments
        let result = func_clone.call(safe_args);
        
        // Log the result
        match &result {
            Ok(_) => tracing::debug!("Thread-safe goroutine completed successfully"),
            Err(e) => tracing::error!(error = ?e, "Thread-safe goroutine execution failed"),
        }
        
        // Decrement active goroutine count
        let mut state = state_arc.lock().unwrap();
        state.active_count -= 1;
        tracing::debug!(active_count = state.active_count, "Decremented active goroutine count");
        
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
pub fn run_goroutine_fn<F>(f: F) -> Result<(), Error>
where 
    F: FnOnce() + Send + 'static 
{
    // Increment active goroutine count
    {
        let mut state = GOROUTINE_STATE.lock().unwrap();
        state.active_count += 1;
        tracing::debug!(active_count = state.active_count, "Incremented active goroutine count");
    }
    
    // Create clone for the new thread
    let state_arc = Arc::clone(&GOROUTINE_STATE);
    
    // Spawn a new thread to execute the closure
    thread::spawn(move || {
        tracing::info!("Closure-based goroutine started");
        
        // Execute the closure
        f();
        
        // Decrement active goroutine count
        let mut state = state_arc.lock().unwrap();
        state.active_count -= 1;
        tracing::debug!(active_count = state.active_count, "Decremented active goroutine count");
        
        // No result to return
    });
    
    Ok(())
}

/// Get the number of active goroutines
pub fn active_goroutine_count() -> usize {
    let state = GOROUTINE_STATE.lock().unwrap();
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
pub fn wait_all_goroutines(timeout_ms: u64) -> Result<(), Error> {
    tracing::debug!(timeout_ms = timeout_ms, "Waiting for all goroutines");
    
    let start_time = std::time::Instant::now();
    
    loop {
        // Check if all goroutines are done
        let count = active_goroutine_count();
        if count == 0 {
            tracing::debug!("All goroutines completed");
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

/// Initialize the runtime for goroutines
///
/// This function sets up any global state needed for goroutine management.
#[tracing::instrument(level = "info")]
pub fn init_goroutine_runtime() {
    tracing::info!("Initializing goroutine runtime");
    
    // Reset the goroutine state
    let mut state = GOROUTINE_STATE.lock().unwrap();
    state.active_count = 0;
}
