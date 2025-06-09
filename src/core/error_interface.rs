//! Built-in error interface type for the CURSED language
//!
//! This module implements Go-style error handling patterns by providing:
//! - A built-in `error` interface type
//! - Error creation and conversion functions
//! - Integration with the existing type system
//! - Support for error propagation with the `?` operator

use crate::core::type_checker::Type;
use crate::error::Error;
use crate::object::Object;
use std::collections::HashMap;
use std::sync::Arc;

/// Built-in error interface type definition
///
/// This interface mimics Go's built-in error interface:
/// ```go
/// type error interface {
///     Error() string
/// }
/// ```
///
/// In CURSED syntax:
/// ```cursed
/// collab error {
///     Error() tea
/// }
/// ```
pub fn create_error_interface() -> Type {
    Type::Interface("error".to_string(), Vec::new())
}

/// Create the built-in error type that implements the error interface
pub fn create_error_type() -> Type {
    Type::Named("Error".to_string())
}

/// Check if a type implements the error interface
pub fn implements_error_interface(t: &Type) -> bool {
    match t {
        Type::Named(name) if name == "Error" => true,
        Type::Interface(name, _) if name == "error" => true,
        _ => false,
    }
}

/// Create a new error object with the given message
pub fn new_error_object(message: String) -> Object {
    Object::Error {
        message: message.clone(),
        error_type: Some("Error".to_string()),
        stack_trace: Vec::new(),
    }
}

/// Extract error message from an error object
pub fn error_message(obj: &Object) -> Option<String> {
    match obj {
        Object::Error { message, .. } => Some(message.clone()),
        _ => None,
    }
}

/// Convert any object that implements the error interface to an error
pub fn to_error_interface(obj: &Object) -> Result<Object, Error> {
    match obj {
        Object::Error { .. } => Ok(obj.clone()),
        Object::String(msg) => Ok(new_error_object(msg.clone())),
        _ => Err(Error::Runtime("Object does not implement error interface".to_string())),
    }
}

/// Check if an object is an error (implements error interface)
pub fn is_error_type(obj: &Object) -> bool {
    matches!(obj, Object::Error { .. })
}

/// Error interface implementation details for type checking
pub struct ErrorInterface {
    methods: HashMap<String, (Vec<Type>, Option<Type>)>,
}

impl ErrorInterface {
    pub fn new() -> Self {
        let mut methods = HashMap::new();
        
        // Error() tea - the single method required by the error interface
        methods.insert(
            "Error".to_string(),
            (Vec::new(), Some(Type::Tea)), // no parameters, returns string (tea)
        );
        
        Self { methods }
    }
    
    pub fn get_methods(&self) -> &HashMap<String, (Vec<Type>, Option<Type>)> {
        &self.methods
    }
    
    /// Check if a type properly implements the error interface
    pub fn check_implementation(&self, type_methods: &HashMap<String, (Vec<Type>, Option<Type>)>) -> bool {
        for (method_name, (params, return_type)) in &self.methods {
            if let Some((impl_params, impl_return)) = type_methods.get(method_name) {
                // Check parameter compatibility
                if params.len() != impl_params.len() {
                    return false;
                }
                
                for (expected, actual) in params.iter().zip(impl_params.iter()) {
                    if expected != actual {
                        return false;
                    }
                }
                
                // Check return type compatibility
                if return_type != impl_return {
                    return false;
                }
            } else {
                return false; // Required method not found
            }
        }
        
        true
    }
}

impl Default for ErrorInterface {
    fn default() -> Self {
        Self::new()
    }
}

/// Error propagation utilities for the `?` operator
pub mod propagation {
    use super::*;
    use crate::ast::traits::Expression;
    
    /// Result type for error propagation (similar to Rust's Result)
    #[derive(Debug, Clone)]
    pub enum ErrorResult<T> {
        Ok(T),
        Err(Object),
    }
    
    impl<T> ErrorResult<T> {
        /// Create a success result
        pub fn ok(value: T) -> Self {
            ErrorResult::Ok(value)
        }
        
        /// Create an error result
        pub fn err(error: Object) -> Self {
            ErrorResult::Err(error)
        }
        
        /// Check if the result is an error
        pub fn is_err(&self) -> bool {
            matches!(self, ErrorResult::Err(_))
        }
        
        /// Check if the result is successful
        pub fn is_ok(&self) -> bool {
            matches!(self, ErrorResult::Ok(_))
        }
        
        /// Unwrap the value (panics on error)
        pub fn unwrap(self) -> T {
            match self {
                ErrorResult::Ok(value) => value,
                ErrorResult::Err(err) => panic!("Called unwrap on an error: {:?}", err),
            }
        }
        
        /// Unwrap the error (panics on success)
        pub fn unwrap_err(self) -> Object {
            match self {
                ErrorResult::Err(err) => err,
                ErrorResult::Ok(_) => panic!("Called unwrap_err on a success value"),
            }
        }
        
        /// Map the success value if present
        pub fn map<U, F>(self, f: F) -> ErrorResult<U>
        where
            F: FnOnce(T) -> U,
        {
            match self {
                ErrorResult::Ok(value) => ErrorResult::Ok(f(value)),
                ErrorResult::Err(err) => ErrorResult::Err(err),
            }
        }
        
        /// Chain error handling operations
        pub fn and_then<U, F>(self, f: F) -> ErrorResult<U>
        where
            F: FnOnce(T) -> ErrorResult<U>,
        {
            match self {
                ErrorResult::Ok(value) => f(value),
                ErrorResult::Err(err) => ErrorResult::Err(err),
            }
        }
    }
    
    /// Helper function to propagate errors in expressions
    pub fn propagate_error(result: ErrorResult<Object>) -> Result<Object, Error> {
        match result {
            ErrorResult::Ok(value) => Ok(value),
            ErrorResult::Err(error_obj) => {
                if let Some(msg) = error_message(&error_obj) {
                    Err(Error::Runtime(msg))
                } else {
                    Err(Error::Runtime("Unknown error occurred".to_string()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_error_interface() {
        let error_interface = create_error_interface();
        assert_eq!(error_interface, Type::Interface("error".to_string(), Vec::new()));
    }
    
    #[test]
    fn test_implements_error_interface() {
        assert!(implements_error_interface(&Type::Named("Error".to_string())));
        assert!(implements_error_interface(&Type::Interface("error".to_string(), Vec::new())));
        assert!(!implements_error_interface(&Type::Named("String".to_string())));
    }
    
    #[test]
    fn test_new_error_object() {
        let error = new_error_object("test error".to_string());
        match error {
            Object::Error { message, error_type, .. } => {
                assert_eq!(message, "test error");
                assert_eq!(error_type, Some("Error".to_string()));
            }
            _ => panic!("Expected Error object"),
        }
    }
    
    #[test]
    fn test_error_message() {
        let error = new_error_object("test message".to_string());
        assert_eq!(error_message(&error), Some("test message".to_string()));
        
        let non_error = Object::String("not an error".to_string());
        assert_eq!(error_message(&non_error), None);
    }
    
    #[test]
    fn test_error_interface_implementation() {
        let error_interface = ErrorInterface::new();
        let methods = error_interface.get_methods();
        
        assert!(methods.contains_key("Error"));
        let (params, return_type) = &methods["Error"];
        assert!(params.is_empty());
        assert_eq!(return_type, &Some(Type::Tea));
    }
    
    #[test]
    fn test_error_result() {
        use propagation::ErrorResult;
        
        let success: ErrorResult<i32> = ErrorResult::ok(42);
        assert!(success.is_ok());
        assert!(!success.is_err());
        assert_eq!(success.unwrap(), 42);
        
        let error: ErrorResult<i32> = ErrorResult::err(new_error_object("test error".to_string()));
        assert!(error.is_err());
        assert!(!error.is_ok());
    }
}
