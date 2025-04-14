//! The `core` package provides fundamental types and functions.
//!
//! This is equivalent to the `builtin` package in Go and is automatically included
//! in all CURSED programs.

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::Error;
use crate::object::{Object, Channel};

/// Convert to boolean
pub fn lit(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "lit: missing argument", None));
    }
    
    let x = &*args[0];
    let result = match x {
        Object::Null => Object::Boolean(false),
        Object::Integer(i) => Object::Boolean(*i != 0),
        Object::Float(f) => Object::Boolean(*f != 0.0),
        Object::Boolean(b) => Object::Boolean(*b),
        Object::String(s) => Object::Boolean(!s.is_empty()),
        Object::Array(a) => Object::Boolean(!a.is_empty()),
        Object::HashTable(h) => Object::Boolean(!h.is_empty()),
        Object::Channel(ch) => {
            let channel = ch.borrow();
            Object::Boolean(!channel.is_closed())
        },
        Object::Option(opt) => Object::Boolean(opt.is_some()),
        _ => Object::Boolean(true), // Default to true for other types
    };
    
    Ok(Rc::new(result))
}

/// Convert to int32
pub fn normie(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "normie: missing argument", None));
    }
    
    let x = &*args[0];
    let result = match x {
        Object::Integer(i) => Object::Integer(*i as i32 as i64),
        Object::Float(f) => Object::Integer(*f as i32 as i64),
        Object::Boolean(b) => Object::Integer(if *b { 1 } else { 0 }),
        Object::String(s) => {
            if let Ok(i) = s.parse::<i32>() {
                Object::Integer(i as i64)
            } else {
                Object::Integer(0) // Default if parse fails
            }
        },
        Object::Char(c) => Object::Integer(*c as i32 as i64),
        _ => Object::Integer(0), // Default for other types
    };
    
    Ok(Rc::new(result))
}

/// Convert to int64
pub fn thicc(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "thicc: missing argument", None));
    }
    
    let x = &*args[0];
    let result = match x {
        Object::Integer(i) => Object::Integer(*i),
        Object::Float(f) => Object::Integer(*f as i64),
        Object::Boolean(b) => Object::Integer(if *b { 1 } else { 0 }),
        Object::String(s) => {
            if let Ok(i) = s.parse::<i64>() {
                Object::Integer(i)
            } else {
                Object::Integer(0) // Default if parse fails
            }
        },
        Object::Char(c) => Object::Integer(*c as i64),
        _ => Object::Integer(0), // Default for other types
    };
    
    Ok(Rc::new(result))
}

/// Convert to float32
pub fn snack(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "snack: missing argument", None));
    }
    
    let x = &*args[0];
    let result = match x {
        Object::Integer(i) => Object::Float(*i as f32 as f64),
        Object::Float(f) => Object::Float(*f as f32 as f64),
        Object::Boolean(b) => Object::Float(if *b { 1.0 } else { 0.0 }),
        Object::String(s) => {
            if let Ok(f) = s.parse::<f32>() {
                Object::Float(f as f64)
            } else {
                Object::Float(0.0) // Default if parse fails
            }
        },
        Object::Char(c) => Object::Float(*c as u32 as f32 as f64),
        _ => Object::Float(0.0), // Default for other types
    };
    
    Ok(Rc::new(result))
}

/// Convert to float64
pub fn meal(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "meal: missing argument", None));
    }
    
    let x = &*args[0];
    let result = match x {
        Object::Integer(i) => Object::Float(*i as f64),
        Object::Float(f) => Object::Float(*f),
        Object::Boolean(b) => Object::Float(if *b { 1.0 } else { 0.0 }),
        Object::String(s) => {
            if let Ok(f) = s.parse::<f64>() {
                Object::Float(f)
            } else {
                Object::Float(0.0) // Default if parse fails
            }
        },
        Object::Char(c) => Object::Float(*c as u32 as f64),
        _ => Object::Float(0.0), // Default for other types
    };
    
    Ok(Rc::new(result))
}

/// Convert to string
pub fn tea(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "tea: missing argument", None));
    }
    
    let x = &*args[0];
    Ok(Rc::new(Object::String(x.to_string())))
}

/// Get the length of a string, array, slice, map, or channel
pub fn len(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "len: missing argument", None));
    }
    
    let v = &*args[0];
    match v {
        Object::String(s) => Ok(Rc::new(Object::Integer(s.len() as i64))),
        Object::Array(a) => Ok(Rc::new(Object::Integer(a.len() as i64))),
        Object::HashTable(h) => Ok(Rc::new(Object::Integer(h.len() as i64))),
        Object::Channel(ch) => {
            let channel = ch.borrow();
            Ok(Rc::new(Object::Integer(channel.len() as i64)))
        },
        _ => Ok(Rc::new(Object::Integer(0))), // Default for unsupported types
    }
}

/// Get the capacity of a slice, map, or channel
pub fn cap(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "cap: missing argument", None));
    }
    
    let v = &*args[0];
    match v {
        Object::Array(a) => Ok(Rc::new(Object::Integer(a.capacity() as i64))),
        Object::Channel(ch) => {
            let channel = ch.borrow();
            Ok(Rc::new(Object::Integer(channel.capacity() as i64)))
        },
        _ => Ok(Rc::new(Object::Integer(0))), // Default for unsupported types
    }
}

/// Append elements to a slice
///
/// # Arguments
///
/// * `args[0]` - The slice to append to
/// * `args[1..n]` - The elements to append
///
/// # Returns
///
/// A new slice with the elements appended
pub fn append(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "append: missing slice argument", None));
    }

    let slice = &*args[0];
    match slice {
        Object::Array(a) => {
            // Clone the array to avoid modifying the original
            let mut new_array = a.clone();
            
            // Append all elements
            for i in 1..args.len() {
                new_array.push((&*args[i]).clone());
            }
            
            Ok(Rc::new(Object::Array(new_array)))
        },
        _ => Err(Error::new("Runtime", "First argument to append must be an array", None)),
    }
}

/// Check if a map contains a key
pub fn has_key(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::new("Runtime", "has_key: requires map and key arguments", None));
    }
    
    let map = &*args[0];
    let key = &*args[1];
    
    match map {
        Object::HashTable(h) => {
            let key_str = match key {
                Object::String(s) => s.clone(),
                _ => key.to_string(), // Convert non-string keys to string
            };
            
            Ok(Rc::new(Object::Boolean(h.contains_key(&key_str))))
        },
        _ => Err(Error::new("Runtime", "First argument to has_key must be a map", None)),
    }
}

/// Get a value from a map by key
pub fn get_map_value(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::new("Runtime", "get_map_value: requires map and key arguments", None));
    }
    
    let map = &*args[0];
    let key = &*args[1];
    
    match map {
        Object::HashTable(h) => {
            let key_str = match key {
                Object::String(s) => s.clone(),
                _ => key.to_string(), // Convert non-string keys to string
            };
            
            if let Some(value) = h.get(&key_str) {
                Ok(Rc::new(value.clone()))
            } else {
                Ok(Rc::new(Object::Null))
            }
        },
        _ => Err(Error::new("Runtime", "First argument to get_map_value must be a map", None)),
    }
}

/// Set a value in a map by key
pub fn set_map_value(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::new("Runtime", "set_map_value: requires map, key, and value arguments", None));
    }
    
    let map = &*args[0];
    let key = &*args[1];
    let value = &*args[2];
    
    match map {
        Object::HashTable(h) => {
            // We need to create a new map with the updated value to maintain immutability
            let mut new_map = h.clone();
            
            let key_str = match key {
                Object::String(s) => s.clone(),
                _ => key.to_string(), // Convert non-string keys to string
            };
            
            new_map.insert(key_str, value.clone());
            Ok(Rc::new(Object::HashTable(new_map)))
        },
        _ => Err(Error::new("Runtime", "First argument to set_map_value must be a map", None)),
    }
}

/// Create a slice, map, or channel
///
/// # Arguments
///
/// * `args[0]` - The type to create ("slice", "map", or "channel")
/// * `args[1]` - The size/capacity for the created type
/// * `args[2]` - The capacity for channels (buffer size)
///
/// # Returns
///
/// A new instance of the requested type
pub fn make(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "make: missing type argument", None));
    }
    
    // Extract type name
    let type_name = match &*args[0] {
        Object::String(s) => s.as_str(),
        _ => return Err(Error::new("Runtime", "make: first argument must be a string", None)),
    };
    
    // Extract size if provided
    let size = if args.len() > 1 {
        match &*args[1] {
            Object::Integer(i) => Some(*i),
            _ => None,
        }
    } else {
        None
    };
    
    // Extract capacity if provided
    let capacity = if args.len() > 2 {
        match &*args[2] {
            Object::Integer(i) => Some(*i),
            _ => None,
        }
    } else {
        None
    };
    
    match type_name {
        "slice" | "array" => {
            let size = size.unwrap_or(0) as usize;
            let mut vec = Vec::with_capacity(size);
            // Initialize with null values
            for _ in 0..size {
                vec.push(Object::Null);
            }
            Ok(Rc::new(Object::Array(vec)))
        },
        "map" => {
            // Create a new empty map
            let capacity = size.unwrap_or(0) as usize;
            let map = HashMap::with_capacity(capacity);
            Ok(Rc::new(Object::HashTable(map)))
        },
        "channel" => {
            // Create a new channel with the given buffer size
            let buffer_size = size.unwrap_or(0) as usize;
            let element_type = capacity.map(|_| "any".to_string()).unwrap_or_else(|| "any".to_string());
            let channel = Channel::new(element_type, buffer_size);
            Ok(Rc::new(Object::Channel(Rc::new(RefCell::new(channel)))))
        },
        _ => Err(Error::new("Runtime", format!("Unsupported type for make: {}", type_name), None)),
    }
}

/// Create a pointer to zero value of type
///
/// # Arguments
///
/// * `args[0]` - The type to create a pointer to
///
/// # Returns
///
/// A zero value of the specified type
pub fn new(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "new: missing type argument", None));
    }
    
    // Extract type name
    let type_name = match &*args[0] {
        Object::String(s) => s.as_str(),
        _ => return Err(Error::new("Runtime", "new: first argument must be a string", None)),
    };
    
    // Create a zero value of the specified type
    match type_name {
        "int" | "normie" => Ok(Rc::new(Object::Integer(0))),
        "int64" | "thicc" => Ok(Rc::new(Object::Integer(0))),
        "float32" | "snack" => Ok(Rc::new(Object::Float(0.0))),
        "float64" | "meal" => Ok(Rc::new(Object::Float(0.0))),
        "string" | "tea" => Ok(Rc::new(Object::String(String::new()))),
        "bool" | "lit" => Ok(Rc::new(Object::Boolean(false))),
        "map" => Ok(Rc::new(Object::HashTable(HashMap::new()))),
        "channel" => {
            let channel = Channel::new("any".to_string(), 0);
            Ok(Rc::new(Object::Channel(Rc::new(RefCell::new(channel)))))
        },
        "array" | "slice" => Ok(Rc::new(Object::Array(Vec::new()))),
        _ => Ok(Rc::new(Object::Null)),
    }
}

/// Close a channel
pub fn close(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "close: missing channel argument", None));
    }
    
    match &*args[0] {
        Object::Channel(ch) => {
            let mut channel = ch.borrow_mut();
            channel.close();
            Ok(Rc::new(Object::Null))
        },
        _ => Err(Error::new("Runtime", "close: argument must be a channel", None)),
    }
}

/// Send a value to a channel
pub fn send(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::new("Runtime", "send: requires channel and value arguments", None));
    }
    
    match &*args[0] {
        Object::Channel(ch) => {
            let mut channel = ch.borrow_mut();
            let value = (&*args[1]).clone();
            channel.send(value)?;
            Ok(Rc::new(Object::Null))
        },
        _ => Err(Error::new("Runtime", "send: first argument must be a channel", None)),
    }
}

/// Try to send a value to a channel without blocking
pub fn try_send(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::new("Runtime", "try_send: requires channel and value arguments", None));
    }
    
    match &*args[0] {
        Object::Channel(ch) => {
            let mut channel = ch.borrow_mut();
            let value = (&*args[1]).clone();
            let result = channel.try_send(value)?;
            Ok(Rc::new(Object::Boolean(result)))
        },
        _ => Err(Error::new("Runtime", "try_send: first argument must be a channel", None)),
    }
}

/// Receive a value from a channel
pub fn receive(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "receive: missing channel argument", None));
    }
    
    match &*args[0] {
        Object::Channel(ch) => {
            let mut channel = ch.borrow_mut();
            let value = channel.receive()?;
            Ok(Rc::new(value))
        },
        _ => Err(Error::new("Runtime", "receive: argument must be a channel", None)),
    }
}

/// Try to receive a value from a channel without blocking
pub fn try_receive(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("Runtime", "try_receive: missing channel argument", None));
    }
    
    match &*args[0] {
        Object::Channel(ch) => {
            let mut channel = ch.borrow_mut();
            match channel.try_receive()? {
                Some(value) => {
                    // Return a tuple of (value, true) indicating success
                    let result = vec![value, Object::Boolean(true)];
                    Ok(Rc::new(Object::Array(result)))
                },
                None => {
                    // Return a tuple of (null, false) indicating would block
                    let result = vec![Object::Null, Object::Boolean(false)];
                    Ok(Rc::new(Object::Array(result)))
                }
            }
        },
        _ => Err(Error::new("Runtime", "try_receive: argument must be a channel", None)),
    }
}

/// Panic with a value
pub fn panic(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        std::panic!("CURSED panic");
    }
    
    std::panic!("CURSED panic: {}", args[0].to_string());
}

/// Recover from a panic
///
/// This function is meant to be used within a closure that's passed to std::panic::catch_unwind.
/// In CURSED, it would typically be used in deferred functions to recover from panics.
///
/// # Returns
///
/// If called during unwinding from a panic, the panic value as a string.
/// Otherwise, returns null.
pub fn recover(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // In a real implementation, this would be integrated with the runtime's panic handling
    // This is a simplified version that works with the current state of the codebase
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // This is just a placeholder - the real implementation would check if
        // we're currently unwinding from a panic
        false
    })) {
        Ok(false) => Ok(Rc::new(Object::Null)), // Not currently unwinding
        Ok(true) => Ok(Rc::new(Object::String("Recovered from panic".to_string()))), // Unwinding
        Err(e) => {
            // Convert the panic payload to a string if possible
            let result = if let Some(s) = e.downcast_ref::<String>() {
                Object::String(s.clone())
            } else if let Some(s) = e.downcast_ref::<&str>() {
                Object::String((*s).to_string())
            } else {
                Object::String("Recovered from unknown panic".to_string())
            };
            Ok(Rc::new(result))
        }
    }
}

// For backward compatibility
pub fn core_new(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    new(args)
}