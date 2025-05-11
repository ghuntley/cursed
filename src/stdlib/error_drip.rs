//! Error handling utilities for CURSED programming language
//!
//! This module provides error handling utilities similar to Go's errors package
//! but with CURSED naming conventions. It allows for creating, wrapping, and
//! unwrapping errors.

use crate::error::Error;
use crate::object::Object;

/// Create a new error with the given message
///
/// # Arguments
///
/// * `message` - The error message
///
/// # Returns
///
/// A new error object
pub fn new_error(message: &str) -> Result<Object, Error> {
    Ok(Object::Error {
        message: message.to_string(),
        error_type: None,
        stack_trace: Vec::new(),
    })
}

/// Wrap an error with a new message to provide additional context
///
/// # Arguments
///
/// * `error` - The original error
/// * `message` - The additional context message
///
/// # Returns
///
/// A new error that wraps the original error
pub fn wrap_error(error: &Object, message: &str) -> Result<Object, Error> {
    match error {
        Object::Error { message: orig_msg, error_type, stack_trace } => {
            let new_message = format!("{}: {}", message, orig_msg);
            Ok(Object::Error {
                message: new_message,
                error_type: error_type.clone(),
                stack_trace: stack_trace.clone(),
            })
        },
        _ => Err(Error::from_str("Not an error object")),
    }
}

/// Unwrap an error to get the original error message
///
/// # Arguments
///
/// * `error` - The error to unwrap
///
/// # Returns
///
/// The original error message
pub fn unwrap_error(error: &Object) -> Result<Object, Error> {
    match error {
        Object::Error { message, .. } => {
            Ok(Object::String(message.clone()))
        },
        _ => Err(Error::from_str("Not an error object")),
    }
}

/// Check if an object is an error
///
/// # Arguments
///
/// * `obj` - The object to check
///
/// # Returns
///
/// Boolean indicating if the object is an error
pub fn is_error(obj: &Object) -> Result<Object, Error> {
    match obj {
        Object::Error { .. } => Ok(Object::Boolean(true)),
        _ => Ok(Object::Boolean(false)),
    }
}

/// Get the error message from an error object
///
/// # Arguments
///
/// * `error` - The error object
///
/// # Returns
///
/// The error message
pub fn error_message(error: &Object) -> Result<Object, Error> {
    match error {
        Object::Error { message, .. } => {
            Ok(Object::String(message.clone()))
        },
        _ => Err(Error::from_str("Not an error object")),
    }
}