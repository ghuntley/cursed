//! Implementation of regex_vibez functionality
//!
//! This module contains the implementation of the regex_vibez functions
//! that were already defined in the main regex_vibez.rs file.

use crate::error::Error;
use crate::object::Object;
use std::rc::Rc;
use regex::Regex;

/// Finds the first occurrence of a regular expression pattern in a string.
///
/// This function returns the first substring that matches the given regular expression pattern,
/// or null if no match is found.
pub fn find(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "find requires 2 arguments: pattern and string".to_string(),
        ));
    }

    // Get pattern and string from arguments
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => {
            return Err(Error::Runtime(
                "First argument to find must be a string".to_string(),
            ))
        }
    };

    let s = match &*args[1] {
        Object::String(s) => s,
        _ => {
            return Err(Error::Runtime(
                "Second argument to find must be a string".to_string(),
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

    // Find first match
    match regex.find(s) {
        Some(m) => Ok(Rc::new(Object::String(s[m.start()..m.end()].to_string()))),
        None => Ok(Rc::new(Object::Null)),
    }
}

/// Finds all occurrences of a regular expression pattern in a string.
///
/// This function returns all substrings that match the given regular expression pattern,
/// returned as an array of strings.
pub fn find_all(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "find_all requires at least 2 arguments: pattern and string".to_string(),
        ));
    }

    // Get pattern and string from arguments
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => {
            return Err(Error::Runtime(
                "First argument to find_all must be a string".to_string(),
            ))
        }
    };

    let s = match &*args[1] {
        Object::String(s) => s,
        _ => {
            return Err(Error::Runtime(
                "Second argument to find_all must be a string".to_string(),
            ))
        }
    };

    // Get optional limit argument
    let limit = if args.len() > 2 {
        match &*args[2] {
            Object::Integer(n) => {
                if *n < 0 {
                    return Err(Error::Runtime(
                        "Limit cannot be negative".to_string(),
                    ));
                }
                Some(*n as usize)
            },
            _ => {
                return Err(Error::Runtime(
                    "Third argument to find_all must be an integer".to_string(),
                ))
            }
        }
    } else {
        None
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

    // Find all matches
    let mut matches = vec![];
    let find_iter = regex.find_iter(s);
    
    // Apply limit if provided
    let limited_iter = if let Some(limit) = limit {
        find_iter.take(limit)
    } else {
        find_iter.take(usize::MAX) // No practical limit
    };

    // Collect matches into array of String Objects
    for m in limited_iter {
        matches.push(Object::String(s[m.start()..m.end()].to_string()));
    }

    Ok(Rc::new(Object::Array(matches)))
}

/// Checks if a string matches a regular expression pattern.
///
/// This function uses full regex pattern matching to determine if the input
/// string contains any matches for the given pattern.
pub fn matches(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "matches requires 2 arguments: pattern and string".to_string(),
        ));
    }

    // Get pattern and string from arguments
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => {
            return Err(Error::Runtime(
                "First argument to matches must be a string".to_string(),
            ))
        }
    };

    let s = match &*args[1] {
        Object::String(s) => s,
        _ => {
            return Err(Error::Runtime(
                "Second argument to matches must be a string".to_string(),
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
    Ok(Rc::new(Object::Boolean(regex.is_match(s))))
}

/// Splits a string by a regular expression pattern.
///
/// This function splits the input string at each occurrence of the regex pattern,
/// returning an array of the substrings between matches.
pub fn split(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "split requires at least 2 arguments: pattern and string".to_string(),
        ));
    }

    // Get pattern and string from arguments
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => {
            return Err(Error::Runtime(
                "First argument to split must be a string".to_string(),
            ))
        }
    };

    let s = match &*args[1] {
        Object::String(s) => s,
        _ => {
            return Err(Error::Runtime(
                "Second argument to split must be a string".to_string(),
            ))
        }
    };

    // Get optional limit argument
    let limit = if args.len() > 2 {
        match &*args[2] {
            Object::Integer(n) => {
                if *n < 0 {
                    return Err(Error::Runtime(
                        "Limit cannot be negative".to_string(),
                    ));
                }
                Some(*n as usize + 1) // +1 because split takes a limit of matches, not resulting parts
            },
            _ => {
                return Err(Error::Runtime(
                    "Third argument to split must be an integer".to_string(),
                ))
            }
        }
    } else {
        None
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

    // Split the string
    let parts: Vec<Object> = if let Some(n) = limit {
        regex.splitn(s, n).map(|part| Object::String(part.to_string())).collect()
    } else {
        regex.split(s).map(|part| Object::String(part.to_string())).collect()
    };

    Ok(Rc::new(Object::Array(parts)))
}

/// Extracts capture groups from a regular expression match.
///
/// This function extracts the first match of a pattern and its capture groups from a string,
/// returning them as an array. The first element is the full match, followed by each capture group.
pub fn extract(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "extract requires 2 arguments: pattern and string".to_string(),
        ));
    }

    // Get pattern and string from arguments
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => {
            return Err(Error::Runtime(
                "First argument to extract must be a string".to_string(),
            ))
        }
    };

    let s = match &*args[1] {
        Object::String(s) => s,
        _ => {
            return Err(Error::Runtime(
                "Second argument to extract must be a string".to_string(),
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

    // Find captures
    let mut results = vec![];
    if let Some(captures) = regex.captures(s) {
        // Add the full match
        if let Some(m) = captures.get(0) {
            results.push(Object::String(s[m.start()..m.end()].to_string()));
        }
        
        // Add each capture group
        for i in 1..captures.len() {
            if let Some(m) = captures.get(i) {
                results.push(Object::String(s[m.start()..m.end()].to_string()));
            } else {
                results.push(Object::String("".to_string())); // Empty string for unmatched groups
            }
        }
    }

    Ok(Rc::new(Object::Array(results)))
}

/// Tests if a regular expression is valid.
///
/// This function checks if a string is a valid regular expression without
/// attempting to match it against any input.
pub fn is_valid(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "is_valid requires 1 argument: pattern".to_string(),
        ));
    }

    // Get pattern from arguments
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => {
            return Err(Error::Runtime(
                "Argument to is_valid must be a string".to_string(),
            ))
        }
    };

    // Check if the pattern is valid
    let is_valid = Regex::new(pattern).is_ok();
    
    Ok(Rc::new(Object::Boolean(is_valid)))
}

/// Escapes a string for use as a literal in a regular expression.
///
/// This function escapes all characters that have special meaning in regex patterns,
/// so the resulting string will match literally in a pattern.
pub fn escape(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "escape requires 1 argument: string".to_string(),
        ));
    }

    // Get string from arguments
    let s = match &*args[0] {
        Object::String(s) => s,
        _ => {
            return Err(Error::Runtime(
                "Argument to escape must be a string".to_string(),
            ))
        }
    };

    // Escape the string using regex library
    let escaped = regex::escape(s);
    
    Ok(Rc::new(Object::String(escaped)))
}