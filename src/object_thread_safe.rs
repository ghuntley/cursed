//! Thread-safe version of the Object type
//!
//! This module provides thread-safe wrappers around Object and related types
//! to enable safe concurrent access from multiple threads. This is essential
//! for the goroutine system to function correctly.

use crate::error::Error;
use crate::memory::Traceable;
use crate::memory::Visitor;
use std::fmt;
use std::sync::{Arc, Mutex};

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

/// Initialize the thread-safe object system
///
/// This function sets up any global state needed for thread-safe objects.
#[tracing::instrument(level = "info")]
pub fn init_thread_safe_objects() {
    tracing::info!("Initializing thread-safe object system");
    // Register any globals or initialize state as needed
}
