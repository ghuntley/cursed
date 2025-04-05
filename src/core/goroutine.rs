//! Core goroutine operations for CURSED language runtime

use crate::object::{Object, Callable};
use crate::error::Error;
use std::thread;
use std::sync::Arc;

/// Launch a new goroutine with the given callable and arguments
/// This is a simplified implementation that doesn't actually execute functions concurrently yet
/// but returns immediately to simulate goroutine behavior.
pub fn launch_goroutine(callable: &Object, args: Vec<Object>) -> Result<Object, Error> {
    // For now, we just return immediately without actually spawning a thread
    // since there are thread-safety issues with the current Object implementation
    // In a real implementation, we would need to make Object thread-safe (using Arc instead of Rc)
    // and properly handle execution in a separate thread
    
    // Log the goroutine call for debugging
    println!("Stan (goroutine) called with: {} and {} args", callable.type_name(), args.len());
    
    // Immediate return, simulating goroutine launch
    Ok(Object::Null)
}

/// Sleep for the specified number of seconds
/// This is a utility function for goroutine testing
pub fn sleep(seconds: f64) -> Result<Object, Error> {
    let millis = (seconds * 1000.0) as u64;
    thread::sleep(std::time::Duration::from_millis(millis));
    Ok(Object::Null)
}