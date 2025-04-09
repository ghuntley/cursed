//! The `core` package provides fundamental types and functions.
//!
//! This is equivalent to the `builtin` package in Go and is automatically included
//! in all CURSED programs.

use crate::object::Object;

/// Convert to boolean
pub fn lit(x: Object) -> Object {
    match x {
        Object::Null => Object::Boolean(false),
        Object::Integer(i) => Object::Boolean(i != 0),
        Object::Float(f) => Object::Boolean(f != 0.0),
        Object::Boolean(b) => Object::Boolean(b),
        Object::String(s) => Object::Boolean(!s.is_empty()),
        // Add other types as needed
        _ => Object::Boolean(true), // Default to true for other types
    }
}

/// Convert to int32
pub fn normie(x: Object) -> Object {
    match x {
        Object::Integer(i) => Object::Integer(i as i32 as i64),
        Object::Float(f) => Object::Integer(f as i32 as i64),
        Object::Boolean(b) => Object::Integer(if b { 1 } else { 0 }),
        Object::String(s) => {
            if let Ok(i) = s.parse::<i32>() {
                Object::Integer(i as i64)
            } else {
                Object::Integer(0) // Default if parse fails
            }
        },
        // Add other types as needed
        _ => Object::Integer(0), // Default for other types
    }
}

/// Convert to int64
pub fn thicc(x: Object) -> Object {
    match x {
        Object::Integer(i) => Object::Integer(i),
        Object::Float(f) => Object::Integer(f as i64),
        Object::Boolean(b) => Object::Integer(if b { 1 } else { 0 }),
        Object::String(s) => {
            if let Ok(i) = s.parse::<i64>() {
                Object::Integer(i)
            } else {
                Object::Integer(0) // Default if parse fails
            }
        },
        // Add other types as needed
        _ => Object::Integer(0), // Default for other types
    }
}

/// Convert to float32
pub fn snack(x: Object) -> Object {
    match x {
        Object::Integer(i) => Object::Float(i as f32 as f64),
        Object::Float(f) => Object::Float(f as f32 as f64),
        Object::Boolean(b) => Object::Float(if b { 1.0 } else { 0.0 }),
        Object::String(s) => {
            if let Ok(f) = s.parse::<f32>() {
                Object::Float(f as f64)
            } else {
                Object::Float(0.0) // Default if parse fails
            }
        },
        // Add other types as needed
        _ => Object::Float(0.0), // Default for other types
    }
}

/// Convert to float64
pub fn meal(x: Object) -> Object {
    match x {
        Object::Integer(i) => Object::Float(i as f64),
        Object::Float(f) => Object::Float(f),
        Object::Boolean(b) => Object::Float(if b { 1.0 } else { 0.0 }),
        Object::String(s) => {
            if let Ok(f) = s.parse::<f64>() {
                Object::Float(f)
            } else {
                Object::Float(0.0) // Default if parse fails
            }
        },
        // Add other types as needed
        _ => Object::Float(0.0), // Default for other types
    }
}

/// Convert to string
pub fn tea(x: Object) -> Object {
    Object::String(x.to_string())
}

/// Get the length of a string, array, slice, map, or channel
pub fn len(v: Object) -> Object {
    match v {
        Object::String(s) => Object::Integer(s.len() as i64),
        Object::Array(a) => Object::Integer(a.len() as i64),
        // Add other types like Map once they're implemented
        _ => Object::Integer(0), // Default for unsupported types
    }
}

/// Get the capacity of a slice, map, or channel
pub fn cap(v: Object) -> Object {
    match v {
        Object::Array(a) => Object::Integer(a.capacity() as i64),
        // Add other types like Map or Channel once they're implemented
        _ => Object::Integer(0), // Default for unsupported types
    }
}

/// Panic with a value
pub fn panic(v: Object) -> ! {
    panic!("CURSED panic: {}", v.to_string())
}

/// Recover from a panic
pub fn recover() -> Object {
    // This would need to be implemented with Rust's catch_unwind or similar
    // For now, just return Null
    Object::Null
}