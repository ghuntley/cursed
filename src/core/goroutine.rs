//! Core goroutine operations for CURSED language runtime

use crate::object::{Object, Callable};
use crate::error::Error;
use crate::core::thread_safe_goroutine;
use std::thread;
use std::sync::Arc;

/// Launch a new goroutine with the given callable and arguments
/// Using thread-safe object implementation for actual concurrent execution
pub fn launch_goroutine(callable: &Object, args: Vec<Object>) -> Result<Object, Error> {
    // Convert objects to thread-safe variants
    let thread_safe_callable = thread_safe_goroutine::convert_to_thread_safe_object(callable)?;
    
    let mut thread_safe_args = Vec::new();
    for arg in args {
        thread_safe_args.push(thread_safe_goroutine::convert_to_thread_safe_object(&arg)?); 
    }
    
    // Launch the goroutine using the thread-safe implementation
    match thread_safe_goroutine::launch_thread_safe_goroutine(&thread_safe_callable, thread_safe_args) {
        Ok(_) => Ok(Object::Null),
        Err(e) => Err(e),
    }
}

/// Sleep for the specified number of seconds
/// This is a utility function for goroutine testing
pub fn sleep(seconds: f64) -> Result<Object, Error> {
    thread_safe_goroutine::thread_safe_sleep(seconds)?;
    Ok(Object::Null)
}