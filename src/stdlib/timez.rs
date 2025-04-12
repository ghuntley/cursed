//! Time-related functions and utilities for CURSED programs
//!
//! The timez package provides functionality for measuring time, scheduling
//! delays, and working with time values, similar to Go's time package.
//! It includes functions for getting the current time, working with time
//! durations, and sleeping.
//!
//! Time constants:
//! - `NANOSECOND`, `MICROSECOND`, `MILLISECOND` - Time unit constants
//! - `SECOND`, `MINUTE`, `HOUR` - Larger time unit constants
//!
//! Key functions:
//! - `now` - Get the current time
//! - `sleep` - Pause execution for a specified duration
//! - `unix_timestamp` - Get the current Unix timestamp

use std::rc::Rc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;
use crate::object::Object;
use crate::error::Error;

// Time duration constants
pub const NANOSECOND: i64 = 1;
pub const MICROSECOND: i64 = 1000 * NANOSECOND;
pub const MILLISECOND: i64 = 1000 * MICROSECOND;
pub const SECOND: i64 = 1000 * MILLISECOND;
pub const MINUTE: i64 = 60 * SECOND;
pub const HOUR: i64 = 60 * MINUTE;

/// Gets the current system time
///
/// Returns the current time as a Unix timestamp (seconds since January 1, 1970 UTC).
/// This is similar to time.Now() in Go but returns the raw timestamp instead of a Time object.
///
/// # Returns
///
/// An integer representing the Unix timestamp in seconds
pub fn now(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    let now = SystemTime::now();
    let unix_time = now.duration_since(UNIX_EPOCH)
        .map_err(|e| Error::Runtime(format!("Failed to get current time: {}", e)))?;
    
    Ok(Rc::new(Object::Integer(unix_time.as_secs() as i64)))
}

/// Pauses the current goroutine for the specified duration
///
/// This function suspends execution of the current goroutine for at least the
/// specified amount of time. The actual sleep duration may be slightly longer due
/// to OS scheduling and timer resolution.
///
/// # Arguments
///
/// * `args[0]` - The sleep duration in milliseconds (integer or float)
///
/// # Returns
///
/// Null after the sleep completes
pub fn sleep(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("sleep requires 1 argument: duration in milliseconds".to_string()));
    }
    
    let duration_ms = match &*args[0] {
        Object::Integer(ms) => *ms,
        Object::Float(ms) => *ms as i64,
        _ => return Err(Error::Runtime("Argument to sleep must be a number".to_string())),
    };
    
    if duration_ms < 0 {
        return Err(Error::Runtime("Sleep duration cannot be negative".to_string()));
    }
    
    thread::sleep(Duration::from_millis(duration_ms as u64));
    Ok(Rc::new(Object::Null))
}

/// Convert duration in seconds to Duration object
pub fn duration_from_secs(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("duration_from_secs requires 1 argument: seconds".to_string()));
    }
    
    let secs = match &*args[0] {
        Object::Integer(s) => *s,
        Object::Float(s) => *s as i64,
        _ => return Err(Error::Runtime("Argument to duration_from_secs must be a number".to_string())),
    };
    
    Ok(Rc::new(Object::Integer(secs * SECOND)))
}

/// Get the Unix timestamp from the current time
pub fn unix_timestamp(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    let now = SystemTime::now();
    let unix_time = now.duration_since(UNIX_EPOCH)
        .map_err(|e| Error::Runtime(format!("Failed to get Unix timestamp: {}", e)))?;
    
    Ok(Rc::new(Object::Integer(unix_time.as_secs() as i64)))
}