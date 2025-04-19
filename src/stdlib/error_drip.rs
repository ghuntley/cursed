//! error_drip - Standard library package for error handling and testing utilities
//!
//! This module provides an implementation of the "error_drip" standard library package
//! for CURSED, which is similar to Go's errors package. It provides ways to create,
//! wrap, unwrap, and test errors.

use std::fmt;
use std::rc::Rc;
use std::sync::Arc;

use crate::object::Object;
use crate::error_enhanced::{CursedError, ErrorKind};
use crate::error::Error;
use crate::stdlib::dot_registry::DotRegistry;

/// Register the error_drip package functions in the dot registry
pub fn register_functions() {
    if let Ok(mut registry) = crate::stdlib::dot_registry::DOT_REGISTRY.lock() {
        // Basic error creation
        registry.register_handler("error_drip", "New", |args| {
            if args.len() < 1 {
                return Err(Error::from_str("error_drip.New requires a message argument"));
            }
            // Convert the message to a string
            Ok("<error>".to_string())
        });
        
        // Error wrapping - similar to fmt.Errorf("%w", err)
        registry.register_handler("error_drip", "Wrap", |args| {
            if args.len() < 2 {
                return Err(Error::from_str("error_drip.Wrap requires a message and an error"));
            }
            // In a real implementation, we'd wrap the error
            Ok("<wrapped_error>".to_string())
        });
        
        // Check if an error is of a certain type
        registry.register_handler("error_drip", "Is", |args| {
            if args.len() < 2 {
                return Err(Error::from_str("error_drip.Is requires two error arguments"));
            }
            // In a real implementation, we'd check if the first error is or wraps the second error
            Ok("true".to_string())
        });
        
        // Unwrap an error to get the underlying cause
        registry.register_handler("error_drip", "Unwrap", |args| {
            if args.len() < 1 {
                return Err(Error::from_str("error_drip.Unwrap requires an error argument"));
            }
            // In a real implementation, we'd return the wrapped error
            Ok("<unwrapped_error>".to_string())
        });
        
        // Get just the error message
        registry.register_handler("error_drip", "Message", |args| {
            if args.len() < 1 {
                return Err(Error::from_str("error_drip.Message requires an error argument"));
            }
            // In a real implementation, we'd return just the error message
            Ok("Error message".to_string())
        });
        
        // Format a stack trace if available
        registry.register_handler("error_drip", "StackTrace", |args| {
            if args.len() < 1 {
                return Err(Error::from_str("error_drip.StackTrace requires an error argument"));
            }
            // In a real implementation, we'd return the stack trace
            Ok("<stack_trace>".to_string())
        });
        
        // Create a custom error with a specific error code
        registry.register_handler("error_drip", "WithCode", |args| {
            if args.len() < 2 {
                return Err(Error::from_str("error_drip.WithCode requires a message and a code"));
            }
            // In a real implementation, we'd create an error with a code
            Ok("<error_with_code>".to_string())
        });
    }
}

/// Error implementation for use inside the CURSED language
pub struct ErrorObject {
    /// The error message
    pub message: String,
    /// Optional wrapped error
    pub cause: Option<Box<ErrorObject>>,
    /// Error code if available
    pub code: Option<String>,
    /// Error details (key-value pairs)
    pub details: Vec<(String, String)>,
}

impl ErrorObject {
    /// Create a new error with the given message
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            cause: None,
            code: None,
            details: Vec::new(),
        }
    }
    
    /// Wrap another error
    pub fn wrap(message: &str, cause: ErrorObject) -> Self {
        Self {
            message: message.to_string(),
            cause: Some(Box::new(cause)),
            code: None,
            details: Vec::new(),
        }
    }
    
    /// Add a detail to the error
    pub fn with_detail(mut self, key: &str, value: &str) -> Self {
        self.details.push((key.to_string(), value.to_string()));
        self
    }
    
    /// Add an error code
    pub fn with_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }
    
    /// Convert to a CursedError
    pub fn to_cursed_error(&self) -> CursedError {
        let mut error = CursedError::new(ErrorKind::Runtime, &self.message);
        
        // Add code if available
        if let Some(code) = &self.code {
            error = error.with_code(code);
        }
        
        // Add details as context
        for (key, value) in &self.details {
            error = error.with_context(key, value);
        }
        
        // Add cause if available
        if let Some(cause) = &self.cause {
            let cause_error = cause.to_cursed_error();
            error = error.with_cause(cause_error);
        }
        
        error
    }
    
    /// Convert to a standard error object
    pub fn to_object(&self) -> Rc<Object> {
        let mut fields = Vec::new();
        
        // Add message
        fields.push(("message".to_string(), self.message.clone()));
        
        // Add code if available
        if let Some(code) = &self.code {
            fields.push(("code".to_string(), code.clone()));
        }
        
        // Add details
        for (key, value) in &self.details {
            fields.push((key.clone(), value.clone()));
        }
        
        // Create the error object
        Rc::new(Object::Struct {
            name: "Error".to_string(),
            fields,
        })
    }
}

impl fmt::Display for ErrorObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        
        if let Some(code) = &self.code {
            write!(f, " [{}]", code)?;
        }
        
        if !self.details.is_empty() {
            write!(f, " {{")?;
            let mut first = true;
            for (key, value) in &self.details {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", key, value)?;
                first = false;
            }
            write!(f, "}}")?;
        }
        
        if let Some(cause) = &self.cause {
            write!(f, " caused by: {}", cause)?;
        }
        
        Ok(())
    }
}

impl fmt::Debug for ErrorObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorObject {{ message: {}, ", self.message)?;
        
        if let Some(code) = &self.code {
            write!(f, "code: {}, ", code)?;
        }
        
        write!(f, "details: {:?}, ", self.details)?;
        
        if let Some(cause) = &self.cause {
            write!(f, "cause: {:?} ", cause)?;
        }
        
        write!(f, "}}")
    }
}

// Convert from ErrorObject to standard error
impl From<ErrorObject> for Error {
    fn from(err: ErrorObject) -> Self {
        Error::Runtime(err.to_string())
    }
}

/// Utility functions for testing error handling
pub mod test {
    use super::*;
    
    /// Create a test error with a standard format
    pub fn create_test_error(message: &str, code: Option<&str>) -> ErrorObject {
        let mut error = ErrorObject::new(message);
        if let Some(c) = code {
            error = error.with_code(c);
        }
        error.with_detail("test", "true")
    }
    
    /// Create a wrapped test error
    pub fn create_wrapped_error(outer_msg: &str, inner_msg: &str) -> ErrorObject {
        let inner = ErrorObject::new(inner_msg);
        ErrorObject::wrap(outer_msg, inner)
    }
    
    /// Check if an error message contains a substring
    pub fn error_contains(error: &ErrorObject, substring: &str) -> bool {
        error.message.contains(substring) ||
            error.cause.as_ref().map_or(false, |c| error_contains(c, substring))
    }
    
    /// Check if an error has a specific code
    pub fn error_has_code(error: &ErrorObject, code: &str) -> bool {
        error.code.as_ref().map_or(false, |c| c == code) ||
            error.cause.as_ref().map_or(false, |c| error_has_code(c, code))
    }
}