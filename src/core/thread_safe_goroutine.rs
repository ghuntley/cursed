//! Thread-safe goroutine operations for CURSED language runtime

use crate::object_thread_safe::{ThreadSafeObject, ThreadSafeCallable};
use crate::error::Error;
use std::thread;
use std::sync::Arc;

/// Launch a new goroutine with the given callable and arguments
pub fn launch_thread_safe_goroutine(callable: &ThreadSafeObject, args: Vec<ThreadSafeObject>) -> Result<ThreadSafeObject, Error> {
    match callable {
        ThreadSafeObject::CompiledFunction { .. } | 
        ThreadSafeObject::Closure { .. } | 
        ThreadSafeObject::Builtin { .. } | 
        ThreadSafeObject::Method { .. } => {
            // Clone the callable and arguments for the new thread
            let callable_clone = callable.clone();
            let args_clone = args.clone();
            
            // Spawn a new thread to execute the function
            thread::spawn(move || {
                // The call should be safe since we're using thread-safe objects and we've filtered by type
                if let Err(err) = ThreadSafeCallable::call(&callable_clone, args_clone) {
                    eprintln!("Goroutine error: {}", err);
                }
            });
            
            // Immediately return, goroutine continues execution in background
            Ok(ThreadSafeObject::Null)
        },
        obj => Err(Error::Runtime(format!(
            "Cannot spawn goroutine with non-callable object: {}", 
            obj.type_name()
        )))
    }
}

/// Sleep for the specified number of seconds
/// This is a utility function for goroutine testing
pub fn thread_safe_sleep(seconds: f64) -> Result<ThreadSafeObject, Error> {
    let millis = (seconds * 1000.0) as u64;
    thread::sleep(std::time::Duration::from_millis(millis));
    Ok(ThreadSafeObject::Null)
}

/// Convert a regular Object to a ThreadSafeObject
/// This is used as a bridge between the old and new implementations
pub fn convert_to_thread_safe_object(obj: &crate::object::Object) -> Result<ThreadSafeObject, Error> {
    match obj {
        crate::object::Object::Integer(val) => Ok(ThreadSafeObject::Integer(*val)),
        crate::object::Object::Float(val) => Ok(ThreadSafeObject::Float(*val)),
        crate::object::Object::Boolean(val) => Ok(ThreadSafeObject::Boolean(*val)),
        crate::object::Object::String(val) => Ok(ThreadSafeObject::String(val.clone())),
        crate::object::Object::Char(val) => Ok(ThreadSafeObject::Char(*val)),
        // For complex types, we'd need more conversion logic
        // This is a simplified implementation
        _ => Err(Error::Runtime(format!(
            "Cannot convert {} to thread-safe object",
            obj.type_name()
        )))
    }
}

/// Convert a ThreadSafeObject back to a regular Object
/// This is used as a bridge between the old and new implementations
pub fn convert_from_thread_safe_object(obj: &ThreadSafeObject) -> Result<crate::object::Object, Error> {
    match obj {
        ThreadSafeObject::Integer(val) => Ok(crate::object::Object::Integer(*val)),
        ThreadSafeObject::Float(val) => Ok(crate::object::Object::Float(*val)),
        ThreadSafeObject::Boolean(val) => Ok(crate::object::Object::Boolean(*val)),
        ThreadSafeObject::String(val) => Ok(crate::object::Object::String(val.clone())),
        ThreadSafeObject::Char(val) => Ok(crate::object::Object::Char(*val)),
        ThreadSafeObject::Null => Ok(crate::object::Object::Null),
        // For complex types, we'd need more conversion logic
        // This is a simplified implementation
        _ => Err(Error::Runtime(format!(
            "Cannot convert thread-safe {} to regular object",
            obj.type_name()
        )))
    }
}