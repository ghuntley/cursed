//! The regex_vibez package provides regular expression functionality.
//!
//! This module is equivalent to the regexp package in Go, providing functions
//! for pattern matching and string manipulation using regular expressions.
//! Currently, this is a simplified implementation that provides basic string
//! operations without full regex capabilities.
//!
//! # Features
//!
//! - Pattern matching with `matches`
//! - Finding all matches with `find_all`
//! - Replacing text with `replace_all`
//!
//! # Examples
//!
//! ```cursed
//! import "regex_vibez"
//!
//! // Check if a string matches a pattern
//! if regex_vibez.matches("no cap", "that's fire") {
//!     vibez.println("It's a match!")
//! }
//!
//! // Find all occurrences
//! matches := regex_vibez.find_all("vibes", "good vibes only, positive vibes")
//!
//! // Replace text
//! newText := regex_vibez.replace_all("mid", "that movie was mid", "fire")
//! // Result: "that movie was fire"
//! ```

use std::rc::Rc;
use crate::object::Object;
use crate::error::Error;

/// Checks if a string contains a pattern (simplified regex matching).
///
/// Note: The current implementation is simplified and only checks for substring presence,
/// not full regular expression matching.
///
/// # Arguments
///
/// * `args[0]` - The pattern to search for as a String Object
/// * `args[1]` - The string to search in as a String Object
///
/// # Returns
///
/// A Boolean Object indicating whether the pattern is found in the string
///
/// # Errors
///
/// Returns a Runtime error if:
/// - Fewer than 2 arguments are provided
/// - Arguments are not String Objects
pub fn matches(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("matches requires 2 arguments: pattern and string".to_string()));
    }
    
    // Simplified implementation - just check if the string contains the pattern
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => return Err(Error::Runtime("First argument to matches must be a string".to_string())),
    };
    
    let s = match &*args[1] {
        Object::String(s) => s,
        _ => return Err(Error::Runtime("Second argument to matches must be a string".to_string())),
    };
    
    Ok(Rc::new(Object::Boolean(s.contains(pattern))))
}

/// Finds all occurrences of a pattern in a string.
///
/// Note: The current implementation is simplified and returns an empty array.
/// A full implementation would return all matches of the pattern.
///
/// # Arguments
///
/// * `args[0]` - The pattern to search for as a String Object
/// * `args[1]` - The string to search in as a String Object
///
/// # Returns
///
/// An Array Object containing all matched substrings (currently empty)
///
/// # Errors
///
/// Returns a Runtime error if fewer than 2 arguments are provided
pub fn find_all(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("find_all requires 2 arguments: pattern and string".to_string()));
    }
    
    // Simplified implementation - return empty array
    let results: Vec<Object> = Vec::new();
    Ok(Rc::new(Object::Array(results)))
}

/// Replaces all occurrences of a pattern in a string with a replacement string.
///
/// Note: The current implementation is simplified and only does basic string replacement,
/// not full regular expression replacement.
///
/// # Arguments
///
/// * `args[0]` - The pattern to search for as a String Object
/// * `args[1]` - The string to perform replacement on as a String Object
/// * `args[2]` - The replacement string as a String Object
///
/// # Returns
///
/// A String Object with all occurrences of the pattern replaced
///
/// # Errors
///
/// Returns a Runtime error if:
/// - Fewer than 3 arguments are provided
/// - Arguments are not String Objects
pub fn replace_all(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime("replace_all requires 3 arguments: pattern, string, and replacement".to_string()));
    }
    
    // Simplified implementation - basic string replacement
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => return Err(Error::Runtime("First argument to replace_all must be a string".to_string())),
    };
    
    let s = match &*args[1] {
        Object::String(s) => s,
        _ => return Err(Error::Runtime("Second argument to replace_all must be a string".to_string())),
    };
    
    let replacement = match &*args[2] {
        Object::String(r) => r,
        _ => return Err(Error::Runtime("Third argument to replace_all must be a string".to_string())),
    };
    
    let result = s.replace(pattern, replacement);
    Ok(Rc::new(Object::String(result)))
}