//! Concurrency through goroutines in the CURSED language
//!
//! This module implements support for goroutines, which are lightweight
//! concurrent execution units similar to Go's goroutines. Goroutines allow
//! CURSED programs to perform tasks concurrently without the overhead of
//! traditional operating system threads.
//!
//! Goroutines are launched with the `stan` keyword in CURSED code, which
//! is handled internally by the functions in this module. Communication
//! between goroutines is managed through channels.

use crate::error::Error;
use crate::object::{Object};
use crate::object_thread_safe::{ThreadSafeObject, ThreadSafeCallable, ThreadSafeValue};
use crate::core::thread_safe_goroutine;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Launches a new goroutine that executes the given function
///
/// This is a simplified version that just runs the provided function in a new thread.
/// It doesn't attempt to use the regular Object system which isn't thread-safe.
/// For concurrent code, use the thread-safe version instead.
///
/// # Arguments
///
/// * `function` - Closure to execute in the goroutine
///
/// # Returns
///
/// Result<Object, Error> - Ok(Null) if goroutine was launched successfully, Error otherwise
#[tracing::instrument(skip(function), level = "info")]
pub fn launch_goroutine_fn<F>(function: F) -> Result<Object, Error> 
where 
    F: FnOnce() + Send + 'static
{
    tracing::debug!("Launching new goroutine from function");
    
    // Spawn a new thread to execute the function
    thread::spawn(move || {
        tracing::info!("Goroutine started");
        function();
        tracing::debug!("Goroutine completed");
    });

    Ok(Object::Null)
}

/// Launches a new goroutine with a simple function and integer argument
///
/// This is a simplified version for the CURSED interpreter that doesn't try
/// to send complex non-thread-safe objects between threads.
///
/// # Arguments
///
/// * `name` - Name of the function to execute
/// * `arg` - Simple integer argument to pass
///
/// # Returns
///
/// Result<Object, Error> - Ok(Null) if goroutine was launched successfully, Error otherwise
#[tracing::instrument(skip(name, arg), level = "info")]
pub fn launch_simple_goroutine(name: &str, arg: i64) -> Result<Object, Error> {
    tracing::debug!("Launching simple goroutine: {}", name);
    
    // Clone the name for the new thread
    let name_clone = name.to_string();
    
    // Spawn a new thread to execute the function
    thread::spawn(move || {
        tracing::info!("Simple goroutine started: {}", name_clone);
        tracing::debug!("With argument: {}", arg);
        
        // In a real implementation, we would look up the function by name
        // and execute it with the provided arguments
        
        tracing::debug!("Simple goroutine completed");
    });

    Ok(Object::Null)
}

/// Launches a goroutine with a thread-safe callable object
///
/// This is a more advanced version that uses the thread-safe
/// object system to execute the callable. This ensures proper synchronization
/// and memory safety when goroutines interact with shared state.
///
/// # Arguments
///
/// * `callable` - The thread-safe callable to execute
/// * `args` - The thread-safe arguments to pass to the callable
///
/// # Returns
///
/// Result<Object, Error> - Ok(Null) if goroutine was launched successfully, Error otherwise
pub fn launch_thread_safe_goroutine(
    callable: Arc<dyn ThreadSafeCallable>,
    args: Vec<ThreadSafeObject>,
) -> Result<Object, Error> {
    // Use the thread_safe_goroutine module to run the goroutine
    thread_safe_goroutine::run_goroutine(callable, args)?;
    Ok(Object::Null)
}

/// Sleep for the specified number of seconds
/// This is a utility function for goroutine testing
#[tracing::instrument(fields(seconds = seconds), level = "debug")]
pub fn sleep(seconds: f64) -> Result<Object, Error> {
    // Simple sleep implementation
    let millis = (seconds * 1000.0) as u64;
    tracing::debug!(millis = millis, "Sleeping goroutine");
    thread::sleep(Duration::from_millis(millis));
    Ok(Object::Null)
}

/// Wait for all goroutines to complete
///
/// This is a convenience function that waits for all goroutines to complete
/// before returning. This is useful for testing and synchronization.
pub fn wait_all_goroutines(timeout_ms: u64) -> Result<Object, Error> {
    // Use the thread_safe_goroutine module to wait for all goroutines
    thread_safe_goroutine::wait_all_goroutines(timeout_ms)?;
    Ok(Object::Null)
}
