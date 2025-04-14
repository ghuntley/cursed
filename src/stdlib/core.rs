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

/// Append elements to a slice
///
/// # Arguments
///
/// * `slice` - The slice to append to
/// * `elems` - The elements to append
///
/// # Returns
///
/// A new slice with the elements appended
pub fn append(slice: Object, elems: Vec<Object>) -> Object {
    match slice {
        Object::Array(mut a) => {
            // Clone the array to avoid modifying the original
            let mut new_array = a.clone();
            
            // Append all elements
            for elem in elems {
                new_array.push(elem);
            }
            
            Object::Array(new_array)
        },
        _ => panic!("First argument to append must be an array"),
    }
}

/// Create a slice, map, or channel
///
/// # Arguments
///
/// * `type_name` - The type to create ("slice", "map", or "channel")
/// * `size` - The size/capacity for the created type
/// * `capacity` - The capacity for channels (buffer size)
///
/// # Returns
///
/// A new instance of the requested type
pub fn make(type_name: &str, size: Option<i64>, capacity: Option<i64>) -> Object {
    match type_name {
        "slice" | "array" => {
            let size = size.unwrap_or(0) as usize;
            let mut vec = Vec::with_capacity(size);
            // Initialize with null values
            for _ in 0..size {
                vec.push(Object::Null);
            }
            Object::Array(vec)
        },
        "map" => {
            // In a real implementation, this would create a map type
            // For now, we return an empty array as a placeholder
            Object::Array(Vec::new())
        },
        "channel" => {
            // In a real implementation, this would create a channel type
            // with a given buffer size. For now, return null as a placeholder
            Object::Null
        },
        _ => panic!("Unsupported type for make: {}", type_name),
    }
}

/// Create a pointer to zero value of type
///
/// # Arguments
///
/// * `type_name` - The type to create a pointer to
///
/// # Returns
///
/// A zero value of the specified type
pub fn new(type_name: &str) -> Object {
    // In a real implementation, this would create a pointer to a zero value
    // of the specified type. For this simplified version, return the zero value directly.
    match type_name {
        "int" | "normie" => Object::Integer(0),
        "int64" | "thicc" => Object::Integer(0),
        "float32" | "snack" => Object::Float(0.0),
        "float64" | "meal" => Object::Float(0.0),
        "string" | "tea" => Object::String(String::new()),
        "bool" | "lit" => Object::Boolean(false),
        _ => Object::Null,
    }
}

/// Panic with a value
pub fn panic(v: Object) -> ! {
    panic!("CURSED panic: {}", v.to_string())
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
pub fn recover() -> Object {
    // In a real implementation, this would be integrated with the runtime's panic handling
    // This is a simplified version that works with the current state of the codebase
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // This is just a placeholder - the real implementation would check if
        // we're currently unwinding from a panic
        false
    })) {
        Ok(false) => Object::Null, // Not currently unwinding
        Ok(true) => Object::String("Recovered from panic".to_string()), // Unwinding
        Err(e) => {
            // Convert the panic payload to a string if possible
            if let Some(s) = e.downcast_ref::<String>() {
                Object::String(s.clone())
            } else if let Some(s) = e.downcast_ref::<&str>() {
                Object::String((*s).to_string())
            } else {
                Object::String("Recovered from unknown panic".to_string())
            }
        }
    }
}