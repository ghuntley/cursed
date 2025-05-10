//! The regex_vibez package provides regular expression functionality.
//!
//! This module is equivalent to the regexp package in Go, providing functions
//! for pattern matching and string manipulation using regular expressions.
//! It implements full regex support using Rust's regex library.
//!
//! # Features
//!
//! - Pattern matching with `matches`
//! - Finding all matches with `find_all`
//! - Replacing text with `replace_all`
//! - Finding the first match with `find`
//! - Splitting strings with `split`
//! - Extracting submatches with `extract`
//!
//! # Examples
//!
//! ```cursed
//! import "regex_vibez"
//!
//! // Check if a string matches a pattern
//! if regex_vibez.matches("\\bno cap\\b", "that's no cap for real") {
//!     vibez.println("It's a match!")
//! }
//!
//! // Find all occurrences
//! matches := regex_vibez.find_all("vibes?", "good vibes only, positive vibe")
//! // Result: ["vibes", "vibe"]
//!
//! // Replace text
//! newText := regex_vibez.replace_all("\\bmid\\b", "that movie was mid", "fire")
//! // Result: "that movie was fire"
//!
//! // Find first match
//! first := regex_vibez.find("\\d+", "The year is 2025 not 2024")
//! // Result: "2025"
//!
//! // Split string
//! parts := regex_vibez.split("[ ,]+", "clean, precise  regex  splitting")
//! // Result: ["clean", "precise", "regex", "splitting"]
//!
//! // Extract submatches
//! captures := regex_vibez.extract("(\\d{4})-(\\d{2})-(\\d{2})", "Event date: 2025-05-10")
//! // Result: ["2025-05-10", "2025", "05", "10"]
//! ```

use crate::error::Error;
use crate::object::Object;
use std::rc::Rc;
use regex::Regex;

/// Checks if a string matches a regular expression pattern.
///
/// This function uses full regex pattern matching to determine if the input
/// string contains any matches for the given pattern.
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
/// Returns a Runtime error if:
/// - Fewer than 2 arguments are provided
/// - Arguments are not String Objects
/// - The pattern is not a valid regular expression
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

/// Finds all occurrences of a regular expression pattern in a string.
///
/// This function returns all substrings that match the given regular expression pattern,
/// returned as an array of strings.
///
/// # Arguments
///
/// * `args[0]` - The regex pattern as a String Object
/// * `args[1]` - The string to search in as a String Object
/// * `args[2]` - (Optional) Maximum number of matches to return as an Integer Object
///
/// # Returns
///
/// An Array Object containing all matched substrings as String Objects
///
/// # Errors
///
/// Returns a Runtime error if:
/// - Fewer than 2 arguments are provided
/// - Arguments are of incorrect types
/// - The pattern is not a valid regular expression
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

/// Replaces all occurrences of a regular expression pattern in a string with a replacement string.
///
/// This function supports full regular expression patterns and replacement, including capture groups
/// with $1, $2, etc. replacements.
///
/// # Arguments
///
/// * `args[0]` - The regex pattern as a String Object
/// * `args[1]` - The string to perform replacement on as a String Object
/// * `args[2]` - The replacement string as a String Object
/// * `args[3]` - (Optional) Maximum number of replacements to make as an Integer Object
///
/// # Returns
///
/// A String Object with all occurrences of the pattern replaced
///
/// # Errors
///
/// Returns a Runtime error if:
/// - Fewer than 3 arguments are provided
/// - Arguments are of incorrect types
/// - The pattern is not a valid regular expression
pub fn replace_all(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime(
            "replace_all requires at least 3 arguments: pattern, string, and replacement".to_string(),
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

    // Get optional limit argument
    let limit = if args.len() > 3 {
        match &*args[3] {
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
                    "Fourth argument to replace_all must be an integer".to_string(),
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

    // Perform replacement based on limit
    let result = if let Some(limit) = limit {
        regex.replacen(s, limit, replacement)
    } else {
        regex.replace_all(s, replacement)
    };

    Ok(Rc::new(Object::String(result.to_string())))
}

/// Finds the first occurrence of a regular expression pattern in a string.
///
/// This function returns the first substring that matches the given regular expression pattern,
/// or null if no match is found.
///
/// # Arguments
///
/// * `args[0]` - The regex pattern as a String Object
/// * `args[1]` - The string to search in as a String Object
///
/// # Returns
///
/// A String Object containing the first matched substring, or Null if no match is found
///
/// # Errors
///
/// Returns a Runtime error if:
/// - Fewer than 2 arguments are provided
/// - Arguments are not String Objects
/// - The pattern is not a valid regular expression
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

/// Splits a string by a regular expression pattern.
///
/// This function splits the input string at each occurrence of the regex pattern,
/// returning an array of the substrings between matches.
///
/// # Arguments
///
/// * `args[0]` - The regex pattern as a String Object
/// * `args[1]` - The string to split as a String Object
/// * `args[2]` - (Optional) Maximum number of splits to perform as an Integer Object
///
/// # Returns
///
/// An Array Object containing the split substrings as String Objects
///
/// # Errors
///
/// Returns a Runtime error if:
/// - Fewer than 2 arguments are provided
/// - Arguments are of incorrect types
/// - The pattern is not a valid regular expression
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
///
/// # Arguments
///
/// * `args[0]` - The regex pattern with capture groups as a String Object
/// * `args[1]` - The string to extract from as a String Object
///
/// # Returns
///
/// An Array Object containing matches as String Objects. The first element is the full match, followed by capture groups.
/// Returns an empty array if no match is found.
///
/// # Errors
///
/// Returns a Runtime error if:
/// - Fewer than 2 arguments are provided
/// - Arguments are not String Objects
/// - The pattern is not a valid regular expression
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
///
/// # Arguments
///
/// * `args[0]` - The regex pattern to validate as a String Object
///
/// # Returns
///
/// A Boolean Object indicating whether the pattern is a valid regular expression
///
/// # Errors
///
/// Returns a Runtime error if no argument is provided or if the argument is not a string
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
///
/// # Arguments
///
/// * `args[0]` - The string to escape as a String Object
///
/// # Returns
///
/// A String Object containing the escaped string
///
/// # Errors
///
/// Returns a Runtime error if no argument is provided or if the argument is not a string
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