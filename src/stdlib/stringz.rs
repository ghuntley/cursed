//! String manipulation functions for CURSED programs
//!
//! The stringz package provides utilities for working with strings,
//! similar to Go's strings package. It includes functions for
//! searching, modifying, splitting, joining, and transforming strings.
//!
//! Key functions include:
//!
//! - String searching: `contains`, `has_prefix`, `has_suffix`, `count`
//! - String splitting/joining: `split`, `join` 
//! - String transformations: `to_lower`, `to_upper`, `trim`

use std::rc::Rc;
use crate::object::Object;
use crate::error::Error;

/// Checks if a string contains a substring
///
/// Determines whether the first string argument contains the second string argument
/// as a substring anywhere within it.
///
/// # Arguments
///
/// * `args[0]` - The string to search in
/// * `args[1]` - The substring to search for
///
/// # Returns
///
/// A boolean value: true if the substring is found, false otherwise
pub fn contains(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("contains requires 2 arguments".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("First argument to contains must be a string".to_string())),
    };
    
    let substr = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("Second argument to contains must be a string".to_string())),
    };
    
    Ok(Rc::new(Object::Boolean(s.contains(&substr))))
}

/// Count occurrences of substr in s
pub fn count(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("count requires 2 arguments".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("First argument to count must be a string".to_string())),
    };
    
    let substr = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("Second argument to count must be a string".to_string())),
    };
    
    if substr.is_empty() {
        return Ok(Rc::new(Object::Integer(0)));
    }
    
    let count = s.matches(&substr).count() as i64;
    Ok(Rc::new(Object::Integer(count)))
}

/// Check if s starts with prefix
pub fn has_prefix(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("has_prefix requires 2 arguments".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("First argument to has_prefix must be a string".to_string())),
    };
    
    let prefix = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("Second argument to has_prefix must be a string".to_string())),
    };
    
    Ok(Rc::new(Object::Boolean(s.starts_with(&prefix))))
}

/// Check if s ends with suffix
pub fn has_suffix(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("has_suffix requires 2 arguments".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("First argument to has_suffix must be a string".to_string())),
    };
    
    let suffix = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("Second argument to has_suffix must be a string".to_string())),
    };
    
    Ok(Rc::new(Object::Boolean(s.ends_with(&suffix))))
}

/// Join elements with separator
pub fn join(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("join requires 2 arguments".to_string()));
    }
    
    let elements = match &*args[0] {
        Object::Array(arr) => arr.iter().map(|obj| obj.to_string()).collect::<Vec<String>>(),
        _ => return Err(Error::Runtime("First argument to join must be an array".to_string())),
    };
    
    let sep = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("Second argument to join must be a string".to_string())),
    };
    
    Ok(Rc::new(Object::String(elements.join(&sep))))
}

/// Splits a string into substrings based on a separator
///
/// This function divides a string into substrings at each occurrence of the specified
/// separator string, returning an array of the resulting substrings.
///
/// # Arguments
///
/// * `args[0]` - The string to split
/// * `args[1]` - The separator string
///
/// # Returns
///
/// An array of substring strings from the original string
pub fn split(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("split requires 2 arguments".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("First argument to split must be a string".to_string())),
    };
    
    let sep = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("Second argument to split must be a string".to_string())),
    };
    
    let parts: Vec<Object> = s.split(&sep)
        .map(|part| Object::String(part.to_string()))
        .collect();
    
    Ok(Rc::new(Object::Array(parts)))
}

/// Convert to lowercase
pub fn to_lower(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("to_lower requires 1 argument".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("Argument to to_lower must be a string".to_string())),
    };
    
    Ok(Rc::new(Object::String(s.to_lowercase())))
}

/// Convert to uppercase
pub fn to_upper(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("to_upper requires 1 argument".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("Argument to to_upper must be a string".to_string())),
    };
    
    Ok(Rc::new(Object::String(s.to_uppercase())))
}

/// Trim characters from beginning and end
pub fn trim(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("trim requires 2 arguments".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("First argument to trim must be a string".to_string())),
    };
    
    let cutset = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("Second argument to trim must be a string".to_string())),
    };
    
    let chars_to_trim: Vec<char> = cutset.chars().collect();
    let trimmed = s.trim_matches(|c| chars_to_trim.contains(&c));
    
    Ok(Rc::new(Object::String(trimmed.to_string())))
}