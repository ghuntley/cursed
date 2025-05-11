//! The timez package provides functionality for working with time and durations.
//!
//! This module implements functionality similar to Go's time package, providing
//! tools for measuring time, formatting dates, creating timers, and handling durations.

use crate::error::Error;
use crate::object::Object;
use std::rc::Rc;
use std::time::{Duration as StdDuration, Instant, SystemTime, UNIX_EPOCH};
use std::thread;

/// Constant representing a second in nanoseconds
pub const SECOND: u64 = 1_000_000_000;
use chrono::{DateTime, Local, TimeZone, Utc, NaiveDateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A Duration represents a time span
#[derive(Debug, Clone)]
pub struct TimezDuration {
    /// The duration in nanoseconds
    pub nanos: u64,
}

impl TimezDuration {
    /// Create a new TimezDuration from nanoseconds
    pub fn from_nanos(nanos: u64) -> Self {
        Self { nanos }
    }

    /// Create a new TimezDuration from seconds
    pub fn from_seconds(seconds: f64) -> Self {
        Self {
            nanos: (seconds * 1_000_000_000.0) as u64,
        }
    }

    /// Convert TimezDuration to milliseconds
    pub fn as_millis(&self) -> u64 {
        self.nanos / 1_000_000
    }

    /// Convert TimezDuration to seconds
    pub fn as_secs(&self) -> f64 {
        self.nanos as f64 / 1_000_000_000.0
    }

    /// Convert to Rust's std::time::Duration
    pub fn to_std(&self) -> StdDuration {
        StdDuration::from_nanos(self.nanos)
    }
}

/// Get the Unix timestamp in seconds
pub fn unix_timestamp(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Get the current time as a Unix timestamp
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH)
        .map_err(|e| Error::Runtime(format!("Failed to get Unix timestamp: {}", e)))?
        .as_secs_f64();
    
    Ok(Rc::new(Object::Float(duration_since_epoch)))
}

/// Create a duration from seconds
pub fn duration_from_secs(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::InvalidArguments("duration_from_secs requires a number of seconds".to_string()));
    }
    
    let seconds = match &*args[0] {
        Object::Integer(secs) => *secs as f64,
        Object::Float(secs) => *secs,
        _ => return Err(Error::InvalidArguments("duration_from_secs requires a number".to_string())),
    };
    
    let duration = TimezDuration { nanos: (seconds * 1_000_000_000.0) as u64 };
    Ok(Rc::new(Object::ExternalData(Rc::new(duration))))
}

/// Returns the current local time.
///
/// # Returns
///
/// A timestamp object representing the current time
pub fn now(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    let now = SystemTime::now();
    let now_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .unwrap_or(StdDuration::from_secs(0));
    
    let seconds = now_since_epoch.as_secs();
    let nanos = now_since_epoch.subsec_nanos();
    
    let timestamp = Object::HashTable({
        let mut map = HashMap::new();
        map.insert("unix".to_string(), Object::Integer(seconds as i64));
        map.insert("nanos".to_string(), Object::Integer(nanos as i64));
        map
    });
    
    Ok(Rc::new(timestamp))
}

/// Formats a timestamp into a string using the specified layout.
///
/// # Arguments
///
/// * `args[0]` - The timestamp to format
/// * `args[1]` - The format string (using strftime format)
///
/// # Returns
///
/// A formatted time string
pub fn format(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "format requires 2 arguments: time and layout".to_string(),
        ));
    }
    
    // Get the timestamp
    let timestamp = match &*args[0] {
        Object::HashTable(map) => {
            let unix = match map.get("unix") {
                Some(Object::Integer(secs)) => *secs,
                _ => return Err(Error::Runtime("Invalid timestamp: missing unix field".to_string())),
            };
            
            let nanos = match map.get("nanos") {
                Some(Object::Integer(ns)) => *ns as u32,
                _ => 0, // Default to 0 nanoseconds if not present
            };
            
            // Create a DateTime from the timestamp
            match NaiveDateTime::from_timestamp_opt(unix, nanos) {
                Some(dt) => DateTime::<Utc>::from_utc(dt, Utc),
                None => return Err(Error::Runtime("Invalid timestamp".to_string())),
            }
        },
        _ => return Err(Error::Runtime("First argument must be a timestamp".to_string())),
    };
    
    // Get the format string
    let format = match &*args[1] {
        Object::String(fmt) => fmt,
        _ => return Err(Error::Runtime("Second argument must be a format string".to_string())),
    };
    
    // Format the timestamp
    let formatted = timestamp.format(format).to_string();
    
    Ok(Rc::new(Object::String(formatted)))
}

/// Parses a time string using the specified layout.
///
/// # Arguments
///
/// * `args[0]` - The time string to parse
/// * `args[1]` - The format string (using strftime format)
///
/// # Returns
///
/// A timestamp object representing the parsed time
pub fn parse(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "parse requires 2 arguments: time string and layout".to_string(),
        ));
    }
    
    // Get the time string
    let time_str = match &*args[0] {
        Object::String(s) => s,
        _ => return Err(Error::Runtime("First argument must be a string".to_string())),
    };
    
    // Get the format string
    let format = match &*args[1] {
        Object::String(fmt) => fmt,
        _ => return Err(Error::Runtime("Second argument must be a format string".to_string())),
    };
    
    // Parse the time string
    let datetime = match DateTime::parse_from_str(time_str, format) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(e) => return Err(Error::Runtime(format!("Failed to parse time: {}", e))),
    };
    
    // Convert to timestamp
    let timestamp = Object::HashTable({
        let mut map = HashMap::new();
        map.insert("unix".to_string(), Object::Integer(datetime.timestamp()));
        map.insert("nanos".to_string(), Object::Integer(datetime.timestamp_subsec_nanos() as i64));
        map
    });
    
    Ok(Rc::new(timestamp))
}

/// Returns the Unix time (seconds since January 1, 1970 UTC).
///
/// # Arguments
///
/// * `args[0]` - (Optional) The timestamp to get Unix time from
///
/// # Returns
///
/// The Unix time as an integer
pub fn unix(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // If no argument is provided, use the current time
    if args.is_empty() {
        let now = SystemTime::now();
        let unix_time = now
            .duration_since(UNIX_EPOCH)
            .unwrap_or(StdDuration::from_secs(0))
            .as_secs();
        
        return Ok(Rc::new(Object::Integer(unix_time as i64)));
    }
    
    // Otherwise, get the Unix time from the provided timestamp
    match &*args[0] {
        Object::HashTable(map) => {
            match map.get("unix") {
                Some(Object::Integer(secs)) => Ok(Rc::new(Object::Integer(*secs))),
                _ => Err(Error::Runtime("Invalid timestamp: missing unix field".to_string())),
            }
        },
        _ => Err(Error::Runtime("Argument must be a timestamp".to_string())),
    }
}

/// Pauses the current goroutine for the specified duration.
///
/// # Arguments
///
/// * `args[0]` - The duration to sleep in seconds (float)
///
/// # Returns
///
/// Null
pub fn sleep(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "sleep requires 1 argument: duration in seconds".to_string(),
        ));
    }
    
    // Get the duration
    let seconds = match &*args[0] {
        Object::Integer(i) => *i as f64,
        Object::Float(f) => *f,
        _ => return Err(Error::Runtime("Argument must be a number".to_string())),
    };
    
    // Convert to TimezDuration
    let duration = TimezDuration::from_seconds(seconds);
    
    // Sleep
    thread::sleep(duration.to_std());
    
    Ok(Rc::new(Object::Null))
}

/// Waits until the specified duration has elapsed and then sends the current time on a channel.
///
/// # Arguments
///
/// * `args[0]` - The duration to wait in seconds (float)
///
/// # Returns
///
/// A channel that will receive the time after the duration elapses
pub fn after(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "after requires 1 argument: duration in seconds".to_string(),
        ));
    }
    
    // Get the duration
    let seconds = match &*args[0] {
        Object::Integer(i) => *i as f64,
        Object::Float(f) => *f,
        _ => return Err(Error::Runtime("Argument must be a number".to_string())),
    };
    
    // Convert to TimezDuration
    let duration = TimezDuration::from_seconds(seconds);
    
    // Create a channel and spawn a thread
    let (sender, receiver) = std::sync::mpsc::channel();
    
    thread::spawn(move || {
        thread::sleep(duration.to_std());
        let _ = sender.send(SystemTime::now());
    });
    
    // Create a channel object
    // Note: In a real implementation, this would be a proper channel object
    // For this example, we'll return a placeholder
    let channel = Object::HashTable({
        let mut map = HashMap::new();
        map.insert("_type".to_string(), Object::String("time_channel".to_string()));
        map.insert("_description".to_string(), Object::String(format!("Timer channel for {}s", seconds)));
        map
    });
    
    Ok(Rc::new(channel))
}

/// Returns a channel that sends the current time at intervals of the specified duration.
///
/// # Arguments
///
/// * `args[0]` - The interval duration in seconds (float)
///
/// # Returns
///
/// A channel that will receive the time at regular intervals
pub fn tick(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "tick requires 1 argument: interval in seconds".to_string(),
        ));
    }
    
    // Get the interval
    let seconds = match &*args[0] {
        Object::Integer(i) => *i as f64,
        Object::Float(f) => *f,
        _ => return Err(Error::Runtime("Argument must be a number".to_string())),
    };
    
    // Convert to TimezDuration
    let duration = TimezDuration::from_seconds(seconds);
    
    // Create a channel and spawn a thread
    let (sender, receiver) = std::sync::mpsc::channel();
    
    thread::spawn(move || {
        loop {
            thread::sleep(duration.to_std());
            if sender.send(SystemTime::now()).is_err() {
                break; // Stop if the receiver is dropped
            }
        }
    });
    
    // Create a channel object
    // Note: In a real implementation, this would be a proper channel object
    let channel = Object::HashTable({
        let mut map = HashMap::new();
        map.insert("_type".to_string(), Object::String("ticker_channel".to_string()));
        map.insert("_description".to_string(), Object::String(format!("Ticker channel with interval {}s", seconds)));
        map
    });
    
    Ok(Rc::new(channel))
}

/// Creates a new Duration object.
///
/// # Arguments
///
/// * `args[0]` - The duration in seconds (float)
///
/// # Returns
///
/// A Duration object
pub fn duration(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "duration requires 1 argument: seconds".to_string(),
        ));
    }
    
    // Get the duration in seconds
    let seconds = match &*args[0] {
        Object::Integer(i) => *i as f64,
        Object::Float(f) => *f,
        _ => return Err(Error::Runtime("Argument must be a number".to_string())),
    };
    
    // Convert to TimezDuration object
    let duration_obj = TimezDuration::from_seconds(seconds);
    
    // Convert to a Duration HashTable for external use
    let duration_map = Object::HashTable({
        let mut map = HashMap::new();
        map.insert("seconds".to_string(), Object::Float(duration_obj.as_secs()));
        map.insert("nanos".to_string(), Object::Integer(duration_obj.nanos as i64));
        map
    });
    
    Ok(Rc::new(duration_map))
}

/// Adds a duration to a time.
///
/// # Arguments
///
/// * `args[0]` - The timestamp
/// * `args[1]` - The duration to add (in seconds)
///
/// # Returns
///
/// A new timestamp with the duration added
pub fn add(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "add requires 2 arguments: time and duration".to_string(),
        ));
    }
    
    // Get the timestamp
    let timestamp = match &*args[0] {
        Object::HashTable(map) => {
            let unix = match map.get("unix") {
                Some(Object::Integer(secs)) => *secs,
                _ => return Err(Error::Runtime("Invalid timestamp: missing unix field".to_string())),
            };
            
            let nanos = match map.get("nanos") {
                Some(Object::Integer(ns)) => *ns as u32,
                _ => 0, // Default to 0 nanoseconds if not present
            };
            
            // Create a DateTime from the timestamp
            match NaiveDateTime::from_timestamp_opt(unix, nanos) {
                Some(dt) => DateTime::<Utc>::from_utc(dt, Utc),
                None => return Err(Error::Runtime("Invalid timestamp".to_string())),
            }
        },
        _ => return Err(Error::Runtime("First argument must be a timestamp".to_string())),
    };
    
    // Get the duration in seconds
    let seconds = match &*args[1] {
        Object::Integer(i) => *i as f64,
        Object::Float(f) => *f,
        _ => return Err(Error::Runtime("Second argument must be a number".to_string())),
    };
    
    // Add the duration
    let nanos = (seconds * 1_000_000_000.0) as i64;
    let duration = chrono::Duration::nanoseconds(nanos);
    let new_time = timestamp + duration;
    
    // Convert to timestamp object
    let new_timestamp = Object::HashTable({
        let mut map = HashMap::new();
        map.insert("unix".to_string(), Object::Integer(new_time.timestamp()));
        map.insert("nanos".to_string(), Object::Integer(new_time.timestamp_subsec_nanos() as i64));
        map
    });
    
    Ok(Rc::new(new_timestamp))
}

/// Subtracts a duration from a time.
///
/// # Arguments
///
/// * `args[0]` - The timestamp
/// * `args[1]` - The duration to subtract (in seconds)
///
/// # Returns
///
/// A new timestamp with the duration subtracted
pub fn sub(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "sub requires 2 arguments: time and duration".to_string(),
        ));
    }
    
    // Get the timestamp
    let timestamp = match &*args[0] {
        Object::HashTable(map) => {
            let unix = match map.get("unix") {
                Some(Object::Integer(secs)) => *secs,
                _ => return Err(Error::Runtime("Invalid timestamp: missing unix field".to_string())),
            };
            
            let nanos = match map.get("nanos") {
                Some(Object::Integer(ns)) => *ns as u32,
                _ => 0, // Default to 0 nanoseconds if not present
            };
            
            // Create a DateTime from the timestamp
            match NaiveDateTime::from_timestamp_opt(unix, nanos) {
                Some(dt) => DateTime::<Utc>::from_utc(dt, Utc),
                None => return Err(Error::Runtime("Invalid timestamp".to_string())),
            }
        },
        _ => return Err(Error::Runtime("First argument must be a timestamp".to_string())),
    };
    
    // Get the duration in seconds
    let seconds = match &*args[1] {
        Object::Integer(i) => *i as f64,
        Object::Float(f) => *f,
        _ => return Err(Error::Runtime("Second argument must be a number".to_string())),
    };
    
    // Subtract the duration
    let nanos = (seconds * 1_000_000_000.0) as i64;
    let duration = chrono::Duration::nanoseconds(nanos);
    let new_time = timestamp - duration;
    
    // Convert to timestamp object
    let new_timestamp = Object::HashTable({
        let mut map = HashMap::new();
        map.insert("unix".to_string(), Object::Integer(new_time.timestamp()));
        map.insert("nanos".to_string(), Object::Integer(new_time.timestamp_subsec_nanos() as i64));
        map
    });
    
    Ok(Rc::new(new_timestamp))
}