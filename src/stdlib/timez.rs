//! The timez package provides time-related functionality.
//! This is equivalent to the time package in Go.

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

/// Get the current time
pub fn now(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    let now = SystemTime::now();
    let unix_time = now.duration_since(UNIX_EPOCH)
        .map_err(|e| Error::Runtime(format!("Failed to get current time: {}", e)))?;
    
    Ok(Rc::new(Object::Integer(unix_time.as_secs() as i64)))
}

/// Sleep for the given duration in milliseconds
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