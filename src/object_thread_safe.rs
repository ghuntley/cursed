//! Thread-safe version of the Object type
//!
//! This module provides thread-safe wrappers around Object and related types
//! to enable safe concurrent access from multiple threads. This is essential
//! for the goroutine system to function correctly.

use crate::error::Error;
use crate::memory::Traceable;
use crate::memory::Visitor;
use crate::object::Object;
use std::fmt;
use std::sync::{Arc, Mutex};
use tracing::{debug, info, instrument, warn};

/// Simplified thread-safe object value types
/// 
/// This enum represents a subset of the regular Object types that are
/// thread-safe and can be used in goroutines. We only include simple
/// types that don't contain non-thread-safe constructs like Rc.
#[derive(Debug, Clone, PartialEq)]
pub enum ThreadSafeValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Array(Vec<ThreadSafeValue>),
    Map(std::collections::HashMap<String, ThreadSafeValue>),
    Null,
}

impl From<ThreadSafeValue> for String {
    fn from(value: ThreadSafeValue) -> Self {
        match value {
            ThreadSafeValue::Integer(i) => i.to_string(),
            ThreadSafeValue::Float(f) => f.to_string(),
            ThreadSafeValue::Boolean(b) => b.to_string(),
            ThreadSafeValue::String(s) => s,
            ThreadSafeValue::Array(a) => {
                let elements: Vec<String> = a.iter().map(|v| String::from(v.clone())).collect();
                format!("[{}]", elements.join(", "))
            },
            ThreadSafeValue::Map(m) => {
                let entries: Vec<String> = m.iter()
                    .map(|(k, v)| format!("{}: {}", k, String::from(v.clone())))
                    .collect();
                format!("{{{}}}", entries.join(", "))
            },
            ThreadSafeValue::Null => "null".to_string(),
        }
    }
}

/// Thread-safe wrapper around a value
///
/// This structure wraps a ThreadSafeValue with thread synchronization primitives,
/// allowing it to be safely shared and modified across multiple threads.
#[derive(Clone)]
pub struct ThreadSafeObject {
    /// The wrapped value protected by a mutex
    inner: Arc<Mutex<ThreadSafeValue>>,
}

impl ThreadSafeObject {
    /// Create a new thread-safe object with the given value
    pub fn new(value: impl Into<ThreadSafeValue>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value.into())),
        }
    }

    /// Get a clone of the current value
    pub fn get(&self) -> ThreadSafeValue {
        self.inner.lock().unwrap().clone()
    }

    /// Set the object to a new value
    pub fn set(&self, value: impl Into<ThreadSafeValue>) {
        let mut inner = self.inner.lock().unwrap();
        *inner = value.into();
    }
}

impl fmt::Display for ThreadSafeObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.get();
        write!(f, "{}", String::from(value))
    }
}

/// Thread-safe callable interface
///
/// This trait defines the interface for objects that can be called safely
/// from multiple threads.
pub trait ThreadSafeCallable: Send + Sync {
    fn call(&self, args: Vec<ThreadSafeValue>) -> Result<ThreadSafeValue, Error>;
}

// Implement From for converting between Object and ThreadSafeValue
impl From<i64> for ThreadSafeValue {
    fn from(value: i64) -> Self {
        ThreadSafeValue::Integer(value)
    }
}

impl From<f64> for ThreadSafeValue {
    fn from(value: f64) -> Self {
        ThreadSafeValue::Float(value)
    }
}

impl From<bool> for ThreadSafeValue {
    fn from(value: bool) -> Self {
        ThreadSafeValue::Boolean(value)
    }
}

impl From<String> for ThreadSafeValue {
    fn from(value: String) -> Self {
        ThreadSafeValue::String(value)
    }
}

impl From<&str> for ThreadSafeValue {
    fn from(value: &str) -> Self {
        ThreadSafeValue::String(value.to_string())
    }
}

/// Convert a regular Object to a thread-safe ThreadSafeValue
///
/// This function creates a ThreadSafeValue from a regular Object, ensuring
/// that it can be safely shared across thread boundaries. Not all Object types
/// can be converted - only those that have thread-safe equivalents.
///
/// # Arguments
///
/// * `obj` - A reference to the Object to convert
///
/// # Returns
///
/// * `Result<ThreadSafeValue, Error>` - The converted value or an error if
///   the object type is not supported in a thread-safe context
#[instrument(skip(obj), level = "debug")]
pub fn convert_to_thread_safe(obj: &Object) -> Result<ThreadSafeValue, Error> {
    match obj {
        Object::Integer(i) => {
            debug!(value = i, "Converting integer to thread-safe");
            Ok(ThreadSafeValue::Integer(*i))
        },
        Object::Float(f) => {
            debug!(value = f, "Converting float to thread-safe");
            Ok(ThreadSafeValue::Float(*f))
        },
        Object::Boolean(b) => {
            debug!(value = b, "Converting boolean to thread-safe");
            Ok(ThreadSafeValue::Boolean(*b))
        },
        Object::String(s) => {
            debug!("Converting string to thread-safe");
            Ok(ThreadSafeValue::String(s.clone()))
        },
        Object::Array(arr) => {
            debug!(length = arr.len(), "Converting array to thread-safe");
            let mut thread_safe_arr = Vec::with_capacity(arr.len());
            
            for item in arr {
                thread_safe_arr.push(convert_to_thread_safe(item)?);
            }
            
            Ok(ThreadSafeValue::Array(thread_safe_arr))
        },
        Object::HashTable(map) => {
            debug!(size = map.len(), "Converting hashtable to thread-safe");
            let mut thread_safe_map = std::collections::HashMap::new();
            
            for (key, value) in map {
                thread_safe_map.insert(key.clone(), convert_to_thread_safe(value)?);
            }
            
            Ok(ThreadSafeValue::Map(thread_safe_map))
        },
        Object::Null => {
            debug!("Converting null to thread-safe");
            Ok(ThreadSafeValue::Null)
        },
        _ => {
            // Other types like Function, Closure, etc. are not thread-safe
            let error_msg = format!("Object type {:?} cannot be converted to thread-safe", obj);
            warn!(object_type = ?obj, error = error_msg, "Conversion to thread-safe failed");
            Err(Error::from_str(&error_msg))
        },
    }
}

/// Convert a thread-safe ThreadSafeValue back to a regular Object
///
/// This function creates a regular Object from a ThreadSafeValue, which is useful
/// when thread-safe values need to be used in a single-threaded context.
///
/// # Arguments
///
/// * `value` - A reference to the ThreadSafeValue to convert
///
/// # Returns
///
/// * `Result<Object, Error>` - The converted Object or an error
#[instrument(skip(value), level = "debug")]
pub fn convert_from_thread_safe(value: &ThreadSafeValue) -> Result<Object, Error> {
    match value {
        ThreadSafeValue::Integer(i) => {
            debug!(value = i, "Converting thread-safe integer to regular");
            Ok(Object::Integer(*i))
        },
        ThreadSafeValue::Float(f) => {
            debug!(value = f, "Converting thread-safe float to regular");
            Ok(Object::Float(*f))
        },
        ThreadSafeValue::Boolean(b) => {
            debug!(value = b, "Converting thread-safe boolean to regular");
            Ok(Object::Boolean(*b))
        },
        ThreadSafeValue::String(s) => {
            debug!("Converting thread-safe string to regular");
            Ok(Object::String(s.clone()))
        },
        ThreadSafeValue::Array(arr) => {
            debug!(length = arr.len(), "Converting thread-safe array to regular");
            let mut regular_arr = Vec::with_capacity(arr.len());
            
            for item in arr {
                regular_arr.push(convert_from_thread_safe(item)?);
            }
            
            Ok(Object::Array(regular_arr))
        },
        ThreadSafeValue::Map(map) => {
            debug!(size = map.len(), "Converting thread-safe map to regular");
            let mut regular_map = std::collections::HashMap::new();
            
            for (key, value) in map {
                regular_map.insert(key.clone(), convert_from_thread_safe(value)?);
            }
            
            Ok(Object::HashTable(regular_map))
        },
        ThreadSafeValue::Null => {
            debug!("Converting thread-safe null to regular");
            Ok(Object::Null)
        },
    }
}

/// Thread-safe wrapper for a Traceable object
///
/// This structure allows Traceable objects to be used safely in a concurrent context.
#[derive(Clone)]
pub struct ThreadSafeTraceable {
    /// The wrapped traceable object protected by a mutex
    inner: Arc<Mutex<Box<dyn Traceable + Send + Sync>>>,
}

impl ThreadSafeTraceable {
    /// Create a new thread-safe traceable object
    pub fn new<T: Traceable + Send + Sync + 'static>(obj: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Box::new(obj))),
        }
    }

    /// Get the inner traceable object
    pub fn get_traceable(&self) -> impl Traceable + '_ {
        struct TraceableWrapper<'a> {
            inner: &'a ThreadSafeTraceable,
        }

        impl<'a> Traceable for TraceableWrapper<'a> {
            fn trace(&self, visitor: &mut dyn Visitor) {
                if let Ok(inner) = self.inner.inner.lock() {
                    inner.trace(visitor);
                }
            }

            fn size(&self) -> usize {
                if let Ok(inner) = self.inner.inner.lock() {
                    inner.size()
                } else {
                    0
                }
            }

            fn tag(&self) -> crate::memory::Tag {
                if let Ok(inner) = self.inner.inner.lock() {
                    inner.tag()
                } else {
                    crate::memory::Tag::Null
                }
            }
        }

        TraceableWrapper { inner: self }
    }
}

/// Thread-safe visitor implementation for garbage collection
///
/// This implementation of Visitor is safe to use across thread boundaries,
/// which is essential for performing garbage collection in concurrent contexts.
pub struct ThreadSafeVisitor {
    /// The set of objects that have been visited
    visited: Arc<Mutex<std::collections::HashSet<usize>>>,
    /// The marked objects that are still reachable
    marked: Arc<Mutex<std::collections::HashSet<usize>>>,
}

impl ThreadSafeVisitor {
    /// Create a new thread-safe visitor for garbage collection
    pub fn new() -> Self {
        Self {
            visited: Arc::new(Mutex::new(std::collections::HashSet::new())),
            marked: Arc::new(Mutex::new(std::collections::HashSet::new())),
        }
    }
    
    /// Get the set of marked objects
    pub fn get_marked(&self) -> std::collections::HashSet<usize> {
        self.marked.lock().unwrap().clone()
    }
}

impl Visitor for ThreadSafeVisitor {
    fn visit(&mut self, ptr: std::ptr::NonNull<dyn Traceable>) {
        // For thread-safe objects, we use the pointer address as the ID
        let ptr_addr = ptr.as_ptr() as *const () as usize;
        self.visit_ptr(ptr_addr, crate::memory::Tag::Object);
    }
    
    fn visit_ptr(&mut self, id: usize, _tag: crate::memory::Tag) {
        // Track visited objects to avoid cycles
        let already_visited = {
            let mut visited = self.visited.lock().unwrap();
            if visited.contains(&id) {
                true
            } else {
                visited.insert(id);
                false
            }
        };
        
        if !already_visited {
            // Mark the object as reachable
            let mut marked = self.marked.lock().unwrap();
            marked.insert(id);
        }
    }
}

/// Initialize the thread-safe object system
///
/// This function sets up any global state needed for thread-safe objects.
#[instrument(level = "info")]
pub fn init_thread_safe_objects() {
    info!("Initializing thread-safe object system");
    // Register any globals or initialize state as needed
}
