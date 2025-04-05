//! Thread-safe object implementation for CURSED language

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter, Debug};
use crate::error::Error;
use crate::memory::Visitor;
use crate::memory::Traceable;
use crate::core::CompiledFunction;

/// Thread-safe version of Channel
#[derive(Debug, Clone)]
pub struct ThreadSafeChannel {
    /// The type of elements in the channel
    pub element_type: String,
    /// The buffer for channel messages, protected by a mutex
    buffer: Arc<Mutex<Vec<ThreadSafeObject>>>,
    /// Maximum buffer size (0 for unbuffered)
    pub buffer_size: usize,
    /// Flag indicating if channel is closed, protected by RwLock
    closed: Arc<RwLock<bool>>,
}

// Custom implementation of PartialEq for ThreadSafeChannel to ignore mutex fields
impl PartialEq for ThreadSafeChannel {
    fn eq(&self, other: &Self) -> bool {
        self.element_type == other.element_type && self.buffer_size == other.buffer_size
        // We don't compare buffer and closed because they're wrapped in Arc<Mutex/RwLock> which don't implement PartialEq
    }
}

/// A location in the code for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorLocation {
    pub ip: usize,
    pub function_name: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

impl ErrorLocation {
    pub fn new(ip: usize) -> Self {
        Self {
            ip,
            function_name: None,
            line: None,
            column: None,
        }
    }
    
    pub fn with_function(ip: usize, function_name: String) -> Self {
        Self {
            ip,
            function_name: Some(function_name),
            line: None,
            column: None,
        }
    }
    
    pub fn with_location(ip: usize, function_name: Option<String>, line: usize, column: usize) -> Self {
        Self {
            ip,
            function_name,
            line: Some(line),
            column: Some(column),
        }
    }
}

/// Thread-safe Object represents a runtime value that can be shared between threads
#[derive(Clone)]
pub enum ThreadSafeObject {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Char(char),
    Array(Arc<Mutex<Vec<ThreadSafeObject>>>),
    HashTable(Arc<Mutex<HashMap<String, ThreadSafeObject>>>),
    Channel(Arc<ThreadSafeChannel>),
    CompiledFunction {
        ir_representation: String,
        num_locals: usize,
        num_parameters: usize,
        free_variables: Arc<Vec<ThreadSafeObject>>,
        name: Option<String>,
        is_variadic: bool,
    },
    Closure {
        function: Arc<CompiledFunction>,
        free_vars: Arc<Vec<ThreadSafeObject>>,
    },
    Builtin {
        name: String,
        function: Arc<dyn Fn(Vec<ThreadSafeObject>) -> Result<ThreadSafeObject, Error> + Send + Sync>,
    },
    Struct {
        name: String,
        fields: Vec<(String, String)>, // (name, type)
    },
    Instance {
        struct_type: Arc<ThreadSafeObject>,
        fields: Arc<Mutex<HashMap<String, ThreadSafeObject>>>,
    },
    Interface {
        name: String,
        methods: Vec<(String, Vec<(String, String)>, Option<String>)>, // (method_name, parameters [(name, type)], return_type)
    },
    Method {
        receiver_type: String, // The type this method belongs to
        name: String, // Method name
        parameters: Vec<(String, String)>, // Parameters (name, type)
        return_type: Option<String>, // Optional return type
        function: Arc<CompiledFunction>, // The compiled method body
    },
    Error {
        message: String,
        error_type: Option<String>,
        stack_trace: Vec<ErrorLocation>,
    },
    Null,
}

// Implement Debug manually since Arc<dyn Fn...> doesn't implement Debug
impl Debug for ThreadSafeObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ThreadSafeObject::Integer(val) => write!(f, "Integer({})", val),
            ThreadSafeObject::Float(val) => write!(f, "Float({})", val),
            ThreadSafeObject::Boolean(val) => write!(f, "Boolean({})", val),
            ThreadSafeObject::String(val) => write!(f, "String(\"{}\")", val),
            ThreadSafeObject::Char(val) => write!(f, "Char('{}')", val),
            ThreadSafeObject::Array(_) => write!(f, "Array([...])"),
            ThreadSafeObject::HashTable(_) => write!(f, "HashTable({{...}})"),
            ThreadSafeObject::Channel(_) => write!(f, "Channel(...)"),
            ThreadSafeObject::CompiledFunction { name, .. } => {
                if let Some(name) = name {
                    write!(f, "CompiledFunction({})", name)
                } else {
                    write!(f, "CompiledFunction(anonymous)")
                }
            },
            ThreadSafeObject::Closure { .. } => write!(f, "Closure(...)"),
            ThreadSafeObject::Builtin { name, .. } => write!(f, "Builtin({})", name),
            ThreadSafeObject::Struct { name, .. } => write!(f, "Struct({})", name),
            ThreadSafeObject::Instance { .. } => write!(f, "Instance(...)"),
            ThreadSafeObject::Interface { name, .. } => write!(f, "Interface({})", name),
            ThreadSafeObject::Method { name, .. } => write!(f, "Method({})", name),
            ThreadSafeObject::Error { message, .. } => write!(f, "Error({})", message),
            ThreadSafeObject::Null => write!(f, "Null"),
        }
    }
}

// Implement PartialEq manually since Arc<dyn Fn...> doesn't implement PartialEq
impl PartialEq for ThreadSafeObject {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ThreadSafeObject::Integer(a), ThreadSafeObject::Integer(b)) => a == b,
            (ThreadSafeObject::Float(a), ThreadSafeObject::Float(b)) => a == b,
            (ThreadSafeObject::Boolean(a), ThreadSafeObject::Boolean(b)) => a == b,
            (ThreadSafeObject::String(a), ThreadSafeObject::String(b)) => a == b,
            (ThreadSafeObject::Char(a), ThreadSafeObject::Char(b)) => a == b,
            // For complex types like Arrays, HashTables, etc., we consider them equal if they're the same instance
            // A deep equality check would be more complex and potentially require locking
            (ThreadSafeObject::Null, ThreadSafeObject::Null) => true,
            _ => false,
        }
    }
}

// Implement Send and Sync traits for ThreadSafeObject
// This is safe because we've replaced all non-thread-safe components with thread-safe alternatives
unsafe impl Send for ThreadSafeObject {}
unsafe impl Sync for ThreadSafeObject {}

// Trait for callable objects that can be invoked from multiple threads
pub trait ThreadSafeCallable: Send + Sync {
    fn call(&self, args: Vec<ThreadSafeObject>) -> Result<ThreadSafeObject, Error>;
}

// Implement ThreadSafeCallable for ThreadSafeObject
impl ThreadSafeCallable for ThreadSafeObject {
    fn call(&self, args: Vec<ThreadSafeObject>) -> Result<ThreadSafeObject, Error> {
        match self {
            ThreadSafeObject::CompiledFunction { .. } => {
                // Simplified implementation for now
                // This would dispatch to a VM to execute the compiled function
                Ok(ThreadSafeObject::Null)
            },
            ThreadSafeObject::Closure { .. } => {
                // Similar to CompiledFunction
                Ok(ThreadSafeObject::Null)
            },
            ThreadSafeObject::Builtin { function, .. } => {
                // Call the function directly
                function(args)
            },
            ThreadSafeObject::Method { .. } => {
                // Simplified method call - would need more complex implementation for real usage
                Ok(ThreadSafeObject::Null)
            },
            _ => Err(Error::Runtime(format!(
                "Cannot call non-callable object: {}", 
                self.type_name()
            )))
        }
    }
}

impl ThreadSafeObject {
    /// Get the type name of this object
    pub fn type_name(&self) -> &'static str {
        match self {
            ThreadSafeObject::Integer(_) => "integer",
            ThreadSafeObject::Float(_) => "float",
            ThreadSafeObject::Boolean(_) => "boolean",
            ThreadSafeObject::Char(_) => "char",
            ThreadSafeObject::String(_) => "string",
            ThreadSafeObject::Array(_) => "array",
            ThreadSafeObject::HashTable(_) => "hash",
            ThreadSafeObject::Channel(_) => "channel",
            ThreadSafeObject::CompiledFunction { .. } => "function",
            ThreadSafeObject::Closure { .. } => "closure",
            ThreadSafeObject::Builtin { .. } => "builtin",
            ThreadSafeObject::Struct { .. } => "struct",
            ThreadSafeObject::Interface { .. } => "interface",
            ThreadSafeObject::Method { .. } => "method",
            ThreadSafeObject::Instance { .. } => "instance",
            ThreadSafeObject::Error { .. } => "error",
            ThreadSafeObject::Null => "null",
        }
    }
    
    /// Check if the object is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            ThreadSafeObject::Integer(n) => *n != 0,
            ThreadSafeObject::Float(n) => *n != 0.0,
            ThreadSafeObject::Boolean(b) => *b,
            ThreadSafeObject::String(s) => !s.is_empty(),
            ThreadSafeObject::Char(_) => true,
            ThreadSafeObject::Array(a) => {
                if let Ok(guard) = a.lock() {
                    !guard.is_empty()
                } else {
                    // If we can't acquire the lock, consider it non-empty as a safe default
                    true
                }
            },
            ThreadSafeObject::HashTable(h) => {
                if let Ok(guard) = h.lock() {
                    !guard.is_empty()
                } else {
                    // If we can't acquire the lock, consider it non-empty as a safe default
                    true
                }
            },
            ThreadSafeObject::Channel(ch) => {
                // A channel is truthy if it's not closed
                if let Ok(closed) = ch.is_closed() {
                    !closed
                } else {
                    // If we can't acquire the lock, consider it open as a safe default
                    true
                }
            },
            ThreadSafeObject::CompiledFunction { .. } => true,
            ThreadSafeObject::Closure { .. } => true,
            ThreadSafeObject::Builtin { .. } => true,
            ThreadSafeObject::Struct { .. } => true,
            ThreadSafeObject::Interface { .. } => true,
            ThreadSafeObject::Method { .. } => true,
            ThreadSafeObject::Instance { .. } => true,
            ThreadSafeObject::Error { .. } => false,
            ThreadSafeObject::Null => false,
        }
    }
}

impl ThreadSafeChannel {
    pub fn new(element_type: String, buffer_size: usize) -> Self {
        Self {
            element_type,
            buffer: Arc::new(Mutex::new(Vec::with_capacity(buffer_size))),
            buffer_size,
            closed: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Send a value to the channel
    /// 
    /// For buffered channels:
    /// - If buffer not full, adds value and returns Ok
    /// - If buffer full, returns an Error (would block in full implementation)
    /// 
    /// For unbuffered channels (buffer_size = 0):
    /// - Always returns Ok as we don't implement true blocking in this version
    pub fn send(&self, value: ThreadSafeObject) -> Result<(), Error> {
        // Check if channel is closed
        if let Ok(closed) = self.closed.read() {
            if *closed {
                return Err(Error::Runtime("send on closed channel".to_string()));
            }
        } else {
            // If we can't acquire the lock, assume something is wrong
            return Err(Error::Runtime("channel lock error".to_string()));
        }
        
        // Check if buffer is full for buffered channels
        let mut buffer_guard = match self.buffer.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Error::Runtime("channel buffer lock error".to_string())),
        };
        
        if self.buffer_size > 0 && buffer_guard.len() >= self.buffer_size {
            return Err(Error::Runtime("send on full channel would block".to_string()));
        }
        
        // Add the value to the buffer
        buffer_guard.push(value);
        Ok(())
    }
    
    /// Receive a value from the channel
    pub fn receive(&self) -> Result<ThreadSafeObject, Error> {
        let mut buffer_guard = match self.buffer.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Error::Runtime("channel buffer lock error".to_string())),
        };
        
        // Check if buffer is empty
        if buffer_guard.is_empty() {
            // If channel is closed, this is a receive from closed channel
            if let Ok(closed) = self.closed.read() {
                if *closed {
                    return Err(Error::Runtime("receive from closed channel".to_string()));
                }
            }
            // Otherwise, this would block in a full implementation
            return Err(Error::Runtime("receive from empty channel would block".to_string()));
        }
        
        // Remove and return the first value
        Ok(buffer_guard.remove(0))
    }
    
    /// Close the channel
    pub fn close(&self) -> Result<(), Error> {
        let mut closed_guard = match self.closed.write() {
            Ok(guard) => guard,
            Err(_) => return Err(Error::Runtime("channel close lock error".to_string())),
        };
        
        *closed_guard = true;
        Ok(())
    }
    
    /// Check if the channel is closed
    pub fn is_closed(&self) -> Result<bool, Error> {
        match self.closed.read() {
            Ok(guard) => Ok(*guard),
            Err(_) => Err(Error::Runtime("channel status lock error".to_string())),
        }
    }
    
    /// Get the current buffer length
    pub fn len(&self) -> Result<usize, Error> {
        match self.buffer.lock() {
            Ok(guard) => Ok(guard.len()),
            Err(_) => Err(Error::Runtime("channel buffer lock error".to_string())),
        }
    }
    
    /// Check if the buffer is empty
    pub fn is_empty(&self) -> Result<bool, Error> {
        match self.buffer.lock() {
            Ok(guard) => Ok(guard.is_empty()),
            Err(_) => Err(Error::Runtime("channel buffer lock error".to_string())),
        }
    }
}

impl From<i64> for ThreadSafeObject {
    fn from(val: i64) -> Self {
        ThreadSafeObject::Integer(val)
    }
}

impl From<f64> for ThreadSafeObject {
    fn from(val: f64) -> Self {
        ThreadSafeObject::Float(val)
    }
}

impl From<bool> for ThreadSafeObject {
    fn from(val: bool) -> Self {
        ThreadSafeObject::Boolean(val)
    }
}

impl From<String> for ThreadSafeObject {
    fn from(val: String) -> Self {
        ThreadSafeObject::String(val)
    }
}

impl From<&str> for ThreadSafeObject {
    fn from(val: &str) -> Self {
        ThreadSafeObject::String(val.to_string())
    }
}

impl From<char> for ThreadSafeObject {
    fn from(val: char) -> Self {
        ThreadSafeObject::Char(val)
    }
}

impl Display for ThreadSafeObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ThreadSafeObject::Integer(val) => write!(f, "{}", val),
            ThreadSafeObject::Float(val) => write!(f, "{}", val),
            ThreadSafeObject::Boolean(val) => write!(f, "{}", val),
            ThreadSafeObject::Char(val) => write!(f, "'{}'", val),
            ThreadSafeObject::String(val) => write!(f, "{}", val),
            ThreadSafeObject::Array(arr) => {
                write!(f, "[")?;
                if let Ok(guard) = arr.lock() {
                    for (i, obj) in guard.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", obj)?;
                    }
                }
                write!(f, "]")
            },
            ThreadSafeObject::HashTable(map) => {
                write!(f, "{{")?;
                if let Ok(guard) = map.lock() {
                    for (i, (key, val)) in guard.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}: {}", key, val)?;
                    }
                }
                write!(f, "}}")
            },
            ThreadSafeObject::Channel(channel) => {
                let len = channel.len().unwrap_or(0);
                write!(f, "channel<{}>[{}]", channel.element_type, len)
            },
            ThreadSafeObject::CompiledFunction { name, .. } => {
                if let Some(name) = name {
                    write!(f, "[Function: {}]", name)
                } else {
                    write!(f, "[Function]")
                }
            },
            ThreadSafeObject::Closure { function, .. } => {
                write!(f, "closure[{}]", function.name)
            },
            ThreadSafeObject::Builtin { name, .. } => {
                write!(f, "builtin[{}]", name)
            },
            ThreadSafeObject::Struct { name, .. } => write!(f, "struct[{}]", name),
            ThreadSafeObject::Interface { name, .. } => write!(f, "interface[{}]", name),
            ThreadSafeObject::Instance { struct_type, .. } => {
                match struct_type.as_ref() {
                    ThreadSafeObject::Struct { name, .. } => write!(f, "instance[{}]", name),
                    _ => write!(f, "instance[unknown]"),
                }
            },
            ThreadSafeObject::Error { message, error_type, .. } => {
                if let Some(err_type) = error_type {
                    write!(f, "{}Error: {}", err_type, message)
                } else {
                    write!(f, "Error: {}", message)
                }
            },
            ThreadSafeObject::Null => write!(f, "null"),
            ThreadSafeObject::Method { receiver_type, name, parameters, return_type, .. } => {
                let params_str = parameters
                    .iter()
                    .map(|(param_name, param_type)| format!("{}: {}", param_name, param_type))
                    .collect::<Vec<String>>()
                    .join(", ");
                
                let return_str = match return_type {
                    Some(ret) => format!(": {}", ret),
                    None => String::new(),
                };
                
                write!(f, "method {}:{}({}){}{{ ... }}", receiver_type, name, params_str, return_str)
            },
        }
    }
}