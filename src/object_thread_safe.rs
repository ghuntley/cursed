//! Thread-safe object implementation with GC support
//!
//! This module provides a thread-safe version of the Object type
//! that can be safely shared across threads and works with the
//! concurrent garbage collector.

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::fmt;

use tracing::{debug, error, info, trace, instrument};

use crate::error::Error;
use crate::object::Object;
use crate::memory::{Traceable, Tag, Visitor, ThreadSafeGc};
use crate::runtime::channel_gc::ThreadSafeChannel;

/// A thread-safe object that can be shared across threads
#[derive(Clone)]
pub enum ThreadSafeObject {
    /// Integer value
    Integer(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// String value
    String(Arc<String>),
    /// Array of objects
    Array(Arc<Mutex<Vec<ThreadSafeGc<ThreadSafeObject>>>>),
    /// Hash table
    HashTable(Arc<RwLock<HashMap<String, ThreadSafeGc<ThreadSafeObject>>>>),
    /// Channel
    Channel(Arc<ThreadSafeChannel>),
    /// Null value
    Null,
    /// Error
    Error {
        message: Arc<String>,
        error_type: Option<Arc<String>>,
        stack_trace: Arc<Vec<String>>,
    },
}

impl ThreadSafeObject {
    /// Create a new thread-safe channel
    #[instrument(fields(element_type = ?element_type, buffer_size = buffer_size), level = "debug")]
    pub fn new_channel(element_type: String, buffer_size: usize) -> Self {
        let channel = ThreadSafeChannel::new(element_type, buffer_size);
        debug!("Created new thread-safe channel object");
        ThreadSafeObject::Channel(Arc::new(channel))
    }
    
    /// Send a value to a channel
    pub fn channel_send(&self, value: ThreadSafeGc<ThreadSafeObject>) -> Result<(), Error> {
        match self {
            ThreadSafeObject::Channel(channel) => {
                // Convert the thread-safe value to a regular Object
                let value_obj = Arc::new(value.inner().unwrap().clone());
                let regular_obj = convert_from_thread_safe(&value_obj)?;
                // Send the converted value
                channel.send(regular_obj)
            },
            _ => Err(Error::Runtime(format!(
                "Cannot send to non-channel object: {}",
                self.type_name()
            ))),
        }
    }
    
    /// Receive a value from a channel
    pub fn channel_receive(&self, gc: &Arc<crate::memory::GarbageCollector>) -> Result<ThreadSafeGc<ThreadSafeObject>, Error> {
        match self {
            ThreadSafeObject::Channel(channel) => {
                // Receive the value from the channel
                match channel.receive() {
                    Ok(regular_obj) => {
                        // Convert the regular object to a thread-safe object
                        let thread_safe_obj = convert_to_thread_safe(&regular_obj)?;
                        // Allocate a new object in the GC
                        Ok(gc.allocate_thread_safe((*thread_safe_obj).clone()))
                    },
                    Err(e) => Err(e),
                }
            },
            _ => Err(Error::Runtime(format!(
                "Cannot receive from non-channel object: {}",
                self.type_name()
            ))),
        }
    }
    
    /// Close a channel
    pub fn channel_close(&self) -> Result<(), Error> {
        match self {
            ThreadSafeObject::Channel(channel) => {
                channel.close();
                Ok(())
            },
            _ => Err(Error::Runtime(format!(
                "Cannot close non-channel object: {}",
                self.type_name()
            ))),
        }
    }
    
    /// Get the inner value of this ThreadSafeObject
    pub fn get(&self) -> ThreadSafeValue {
        Arc::new(self.clone())
    }
    
    /// Get the type name of the object
    pub fn type_name(&self) -> &'static str {
        match self {
            ThreadSafeObject::Integer(_) => "integer",
            ThreadSafeObject::Float(_) => "float",
            ThreadSafeObject::Boolean(_) => "boolean",
            ThreadSafeObject::String(_) => "string",
            ThreadSafeObject::Array(_) => "array",
            ThreadSafeObject::HashTable(_) => "hash",
            ThreadSafeObject::Channel(_) => "channel",
            ThreadSafeObject::Null => "null",
            ThreadSafeObject::Error { .. } => "error",
        }
    }
    
    /// Estimate the size of the object in memory
    pub fn size_estimate(&self) -> usize {
        match self {
            ThreadSafeObject::Integer(_) => std::mem::size_of::<i64>(),
            ThreadSafeObject::Float(_) => std::mem::size_of::<f64>(),
            ThreadSafeObject::Boolean(_) => std::mem::size_of::<bool>(),
            ThreadSafeObject::String(s) => std::mem::size_of::<Arc<String>>() + s.len(),
            ThreadSafeObject::Array(a) => {
                let base_size = std::mem::size_of::<Arc<Mutex<Vec<ThreadSafeGc<ThreadSafeObject>>>>>();
                if let Ok(arr) = a.lock() {
                    base_size + arr.len() * std::mem::size_of::<ThreadSafeGc<ThreadSafeObject>>()
                } else {
                    base_size
                }
            },
            ThreadSafeObject::HashTable(h) => {
                let base_size = std::mem::size_of::<Arc<RwLock<HashMap<String, ThreadSafeGc<ThreadSafeObject>>>>>();
                if let Ok(map) = h.read() {
                    base_size + map.len() * (std::mem::size_of::<String>() + std::mem::size_of::<ThreadSafeGc<ThreadSafeObject>>())
                } else {
                    base_size
                }
            },
            ThreadSafeObject::Channel(c) => std::mem::size_of::<Arc<ThreadSafeChannel>>() + c.size(),
            ThreadSafeObject::Null => std::mem::size_of::<()>(),
            ThreadSafeObject::Error { message, stack_trace, .. } => {
                std::mem::size_of::<Arc<String>>() + message.len() +
                std::mem::size_of::<Arc<Vec<String>>>() + stack_trace.len() * std::mem::size_of::<String>()
            },
        }
    }
}

impl fmt::Debug for ThreadSafeObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreadSafeObject::Integer(i) => write!(f, "Integer({})", i),
            ThreadSafeObject::Float(fl) => write!(f, "Float({})", fl),
            ThreadSafeObject::Boolean(b) => write!(f, "Boolean({})", b),
            ThreadSafeObject::String(s) => write!(f, "String(\"{}\")", s),
            ThreadSafeObject::Array(_) => write!(f, "Array([...])"),
            ThreadSafeObject::HashTable(_) => write!(f, "HashTable({{...}})"),
            ThreadSafeObject::Channel(c) => write!(f, "Channel({})", c.element_type()),
            ThreadSafeObject::Null => write!(f, "Null"),
            ThreadSafeObject::Error { message, error_type, .. } => {
                if let Some(err_type) = error_type {
                    write!(f, "{}Error: {}", err_type, message)
                } else {
                    write!(f, "Error: {}", message)
                }
            },
        }
    }
}

impl Traceable for ThreadSafeObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        match self {
            ThreadSafeObject::Array(arr) => {
                if let Ok(elements) = arr.lock() {
                    for elem in elements.iter() {
                        visitor.visit_ptr(elem.id(), Tag::Object);
                    }
                }
            },
            ThreadSafeObject::HashTable(map) => {
                if let Ok(elements) = map.read() {
                    for (_, value) in elements.iter() {
                        visitor.visit_ptr(value.id(), Tag::Object);
                    }
                }
            },
            ThreadSafeObject::Channel(channel) => {
                // Trace objects in the channel buffer
                channel.trace(visitor);
            },
            // Other types don't contain references to trace
            _ => {},
        }
    }
    
    fn size(&self) -> usize {
        self.size_estimate()
    }
    
    fn tag(&self) -> Tag {
        match self {
            ThreadSafeObject::Integer(_) => Tag::Integer,
            ThreadSafeObject::Float(_) => Tag::Float,
            ThreadSafeObject::Boolean(_) => Tag::Boolean,
            ThreadSafeObject::String(_) => Tag::String,
            ThreadSafeObject::Array(_) => Tag::Array,
            ThreadSafeObject::HashTable(_) => Tag::HashTable,
            ThreadSafeObject::Channel(_) => Tag::Channel,
            ThreadSafeObject::Null => Tag::Null,
            ThreadSafeObject::Error { .. } => Tag::Error,
        }
    }
}

// Type alias for a thread-safe reference to an object
pub type ThreadSafeValue = Arc<ThreadSafeObject>;

/// A thread-safe callable object, similar to a function
pub trait ThreadSafeCallable: Send + Sync {
    /// Call the object with the given arguments
    fn call(&self, args: Vec<ThreadSafeValue>) -> Result<ThreadSafeValue, Error>;
}

/// Convert a regular Object to a ThreadSafeObject
pub fn convert_to_thread_safe(obj: &Object) -> Result<ThreadSafeValue, Error> {
    let thread_safe_obj = match obj {
        Object::Integer(i) => ThreadSafeObject::Integer(*i),
        Object::Float(f) => ThreadSafeObject::Float(*f),
        Object::Boolean(b) => ThreadSafeObject::Boolean(*b),
        Object::String(s) => ThreadSafeObject::String(Arc::new(s.clone())),
        Object::Null => ThreadSafeObject::Null,
        _ => return Err(Error::from_str("Cannot convert complex object to thread-safe form"))
    };
    
    Ok(Arc::new(thread_safe_obj))
}

/// Convert a ThreadSafeObject back to a regular Object
pub fn convert_from_thread_safe(obj: &ThreadSafeValue) -> Result<Object, Error> {
    let regular_obj = match &**obj {
        ThreadSafeObject::Integer(i) => Object::Integer(*i),
        ThreadSafeObject::Float(f) => Object::Float(*f),
        ThreadSafeObject::Boolean(b) => Object::Boolean(*b),
        ThreadSafeObject::String(s) => Object::String(s.to_string()),
        ThreadSafeObject::Null => Object::Null,
        _ => return Err(Error::from_str("Cannot convert complex thread-safe object to regular form"))
    };
    
    Ok(regular_obj)
}

// These impls are required for thread-safe usage
unsafe impl Send for ThreadSafeObject {}
unsafe impl Sync for ThreadSafeObject {}