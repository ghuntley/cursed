//! The regex_vibez package provides regular expression functionality.
//!
//! This module is equivalent to the regexp package in Go, providing functions
//! for pattern matching and string manipulation using regular expressions.
//! It implements full regex support using Rust's regex library.

use crate::error::Error;
use crate::object::Object;
use std::sync::Arc;
use regex::Regex;

// Import core functionality
mod impl_regex_vibez;

// Re-export the functions that were missing in the old interface
pub use impl_regex_vibez::*;

/// Compiles a regular expression pattern for later use.
/// 
/// This function creates a reusable Regex object that can be stored
/// and used for multiple matches without recompilation.
///
/// # Arguments
///
/// * `args[0]` - The regex pattern as a String Object
///
/// # Returns
///
/// A compiled regex object
///
/// # Errors
///
/// Returns a Runtime error if no argument is provided, 
/// if the argument is not a string, or if the pattern is invalid
pub fn compile(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "compile requires 1 argument: pattern".to_string(),
        ));
    }

    // Get pattern from arguments
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => {
            return Err(Error::Runtime(
                "Argument to compile must be a string".to_string(),
            ))
        }
    };

    // Try to compile the regex
    match Regex::new(pattern) {
        Ok(regex) => {
            // Create a new object to encapsulate the regex
            let regex_obj = Object::ExternalData(Box::new(regex));
            Ok(Arc::new(regex_obj))
        },
        Err(e) => {
            Err(Error::Runtime(
                format!("Invalid regex pattern: {}", e),
            ))
        }
    }
}

/// Tests if a string matches a pattern.
///
/// # Arguments
///
/// * `args[0]` - The regex pattern as a String Object
/// * `args[1]` - The string to search in as a String Object
///
/// # Returns
///
/// A Boolean Object indicating whether the pattern is found in the string
///
/// # Errors
///
/// Returns a Runtime error if arguments are invalid
pub fn match_str(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "match_str requires 2 arguments: pattern and string".to_string(),
        ));
    }

    // Get pattern and string from arguments
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => {
            return Err(Error::Runtime(
                "First argument to match_str must be a string".to_string(),
            ))
        }
    };

    let s = match &*args[1] {
        Object::String(s) => s,
        _ => {
            return Err(Error::Runtime(
                "Second argument to match_str must be a string".to_string(),
            ))
        }
    };

    // Create regex pattern
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => {
            return Err(Error::Runtime(
                format!("Invalid regex pattern: {}", e),
            ))
        }
    };

    // Check if the pattern matches anywhere in the string
    Ok(Arc::new(Object::Boolean(regex.is_match(s))))
}

/// Replaces parts of a string that match a pattern with a replacement.
///
/// # Arguments
///
/// * `args[0]` - The regex pattern as a String Object
/// * `args[1]` - The string to search in as a String Object
/// * `args[2]` - The replacement string as a String Object
///
/// # Returns
///
/// A String Object with the replaced content
///
/// # Errors
///
/// Returns a Runtime error if arguments are invalid
pub fn replace(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime(
            "replace requires 3 arguments: pattern, string, and replacement".to_string(),
        ));
    }

    // Get pattern, string, and replacement from arguments
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => {
            return Err(Error::Runtime(
                "First argument to replace must be a string".to_string(),
            ))
        }
    };

    let s = match &*args[1] {
        Object::String(s) => s,
        _ => {
            return Err(Error::Runtime(
                "Second argument to replace must be a string".to_string(),
            ))
        }
    };

    let replacement = match &*args[2] {
        Object::String(r) => r,
        _ => {
            return Err(Error::Runtime(
                "Third argument to replace must be a string".to_string(),
            ))
        }
    };

    // Create regex pattern
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => {
            return Err(Error::Runtime(
                format!("Invalid regex pattern: {}", e),
            ))
        }
    };

    // Replace first occurrence
    let result = regex.replace(s, replacement);
    
    Ok(Arc::new(Object::String(result.to_string())))
}

/// Replaces all parts of a string that match a pattern with a replacement.
///
/// # Arguments
///
/// * `args[0]` - The regex pattern as a String Object
/// * `args[1]` - The string to search in as a String Object
/// * `args[2]` - The replacement string as a String Object
///
/// # Returns
///
/// A String Object with all matches replaced
///
/// # Errors
///
/// Returns a Runtime error if arguments are invalid
pub fn replace_all(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime(
            "replace_all requires 3 arguments: pattern, string, and replacement".to_string(),
        ));
    }

    // Get pattern, string, and replacement from arguments
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => {
            return Err(Error::Runtime(
                "First argument to replace_all must be a string".to_string(),
            ))
        }
    };

    let s = match &*args[1] {
        Object::String(s) => s,
        _ => {
            return Err(Error::Runtime(
                "Second argument to replace_all must be a string".to_string(),
            ))
        }
    };

    let replacement = match &*args[2] {
        Object::String(r) => r,
        _ => {
            return Err(Error::Runtime(
                "Third argument to replace_all must be a string".to_string(),
            ))
        }
    };

    // Create regex pattern
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => {
            return Err(Error::Runtime(
                format!("Invalid regex pattern: {}", e),
            ))
        }
    };

    // Replace all occurrences
    let result = regex.replace_all(s, replacement);
    
    Ok(Arc::new(Object::String(result.to_string())))
}