//! String manipulation functions for CURSED programs
//!
//! The stringz package provides utilities for working with strings,
//! similar to Go's strings package. It includes functions for
//! searching, modifying, splitting, joining, and transforming strings.
//!
//! Key functions include:
//!
//! - String searching: `contains`, `has_prefix`, `has_suffix`, `count`, `index`, `last_index`
//! - String splitting/joining: `split`, `join`, `fields`
//! - String transformations: `to_lower`, `to_upper`, `trim`, `trim_space`, `trim_prefix`, `trim_suffix`
//! - String modification: `replace`, `replace_all`, `repeat`
//! - String measurement: `len`
//! - String case conversion: `to_camel_case`, `to_snake_case`, `to_kebab_case`, `to_pascal_case`

use crate::error::Error;
use crate::object::Object;
use std::sync::Arc;

/// Returns the length of a string in characters
///
/// # Arguments
///
/// * `args[0]` - The string to measure
///
/// # Returns
///
/// An integer representing the number of characters in the string
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn len(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        tracing::warn!(provided = args.len(), "Wrong number of arguments to len");
        return Err(Error::new(
            "ArgumentError",
            format!("len takes exactly 1 argument, got {}", args.len()),
            None,
        ));
    }

    let s = match &*args[0] {
        Object::String(s) => s,
        _ => {
            let type_name = args[0].type_name();
            tracing::warn!(actual_type = type_name, "Type error in len function");
            return Err(Error::new(
                "TypeError", 
                format!("len requires a string, got {}", type_name),
                None,
            ));
        },
    };

    let char_count = s.chars().count() as i64;
    tracing::debug!(string = %s, char_count = char_count, "Computed string length");
    Ok(Arc::new(Object::Integer(char_count)))
}

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
pub fn contains(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("contains requires 2 arguments".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to contains must be a string".to_string(),
            ))
        }
    };

    let substr = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to contains must be a string".to_string(),
            ))
        }
    };

    Ok(Arc::new(Object::Boolean(s.contains(&substr))))
}

/// Count occurrences of substr in s
pub fn count(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("count requires 2 arguments".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to count must be a string".to_string(),
            ))
        }
    };

    let substr = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to count must be a string".to_string(),
            ))
        }
    };

    if substr.is_empty() {
        return Ok(Arc::new(Object::Integer(0)));
    }

    let count = s.matches(&substr).count() as i64;
    Ok(Arc::new(Object::Integer(count)))
}

/// Check if s starts with prefix
pub fn has_prefix(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "has_prefix requires 2 arguments".to_string(),
        ));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to has_prefix must be a string".to_string(),
            ))
        }
    };

    let prefix = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to has_prefix must be a string".to_string(),
            ))
        }
    };

    Ok(Arc::new(Object::Boolean(s.starts_with(&prefix))))
}

/// Check if s ends with suffix
pub fn has_suffix(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "has_suffix requires 2 arguments".to_string(),
        ));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to has_suffix must be a string".to_string(),
            ))
        }
    };

    let suffix = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to has_suffix must be a string".to_string(),
            ))
        }
    };

    Ok(Arc::new(Object::Boolean(s.ends_with(&suffix))))
}

/// Join elements with separator
pub fn join(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("join requires 2 arguments".to_string()));
    }

    let elements = match &*args[0] {
        Object::Array(arr) => arr
            .iter()
            .map(|obj| obj.to_string())
            .collect::<Vec<String>>(),
        _ => {
            return Err(Error::Runtime(
                "First argument to join must be an array".to_string(),
            ))
        }
    };

    let sep = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to join must be a string".to_string(),
            ))
        }
    };

    Ok(Arc::new(Object::String(elements.join(&sep))))
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
pub fn split(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("split requires 2 arguments".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to split must be a string".to_string(),
            ))
        }
    };

    let sep = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to split must be a string".to_string(),
            ))
        }
    };

    let parts: Vec<Object> = s
        .split(&sep)
        .map(|part| Object::String(part.to_string()))
        .collect();

    Ok(Arc::new(Object::Array(parts)))
}

/// Convert to lowercase
pub fn to_lower(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("to_lower requires 1 argument".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to to_lower must be a string".to_string(),
            ))
        }
    };

    Ok(Arc::new(Object::String(s.to_lowercase())))
}

/// Convert to uppercase
pub fn to_upper(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("to_upper requires 1 argument".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to to_upper must be a string".to_string(),
            ))
        }
    };

    Ok(Arc::new(Object::String(s.to_uppercase())))
}

/// Trim characters from beginning and end
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn trim(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("trim requires 2 arguments".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to trim must be a string".to_string(),
            ))
        }
    };

    let cutset = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to trim must be a string".to_string(),
            ))
        }
    };

    let chars_to_trim: Vec<char> = cutset.chars().collect();
    let trimmed = s.trim_matches(|c| chars_to_trim.contains(&c));

    Ok(Arc::new(Object::String(trimmed.to_string())))
}

/// Trim whitespace from beginning and end
/// 
/// # Arguments
/// 
/// * `args[0]` - The string to trim
/// 
/// # Returns
/// 
/// A string with all leading and trailing whitespace removed
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn trim_space(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("trim_space requires 1 argument".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to trim_space must be a string".to_string(),
            ))
        }
    };

    let trimmed = s.trim();
    Ok(Arc::new(Object::String(trimmed.to_string())))
}

/// Trim a prefix from a string if it exists
/// 
/// # Arguments
/// 
/// * `args[0]` - The string to trim
/// * `args[1]` - The prefix to remove
/// 
/// # Returns
/// 
/// A string with the prefix removed if it exists, otherwise the original string
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn trim_prefix(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("trim_prefix requires 2 arguments".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to trim_prefix must be a string".to_string(),
            ))
        }
    };

    let prefix = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to trim_prefix must be a string".to_string(),
            ))
        }
    };

    let result = if s.starts_with(&prefix) {
        s[prefix.len()..].to_string()
    } else {
        s
    };

    Ok(Arc::new(Object::String(result)))
}

/// Trim a suffix from a string if it exists
/// 
/// # Arguments
/// 
/// * `args[0]` - The string to trim
/// * `args[1]` - The suffix to remove
/// 
/// # Returns
/// 
/// A string with the suffix removed if it exists, otherwise the original string
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn trim_suffix(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("trim_suffix requires 2 arguments".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to trim_suffix must be a string".to_string(),
            ))
        }
    };

    let suffix = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to trim_suffix must be a string".to_string(),
            ))
        }
    };

    let result = if s.ends_with(&suffix) {
        s[..s.len() - suffix.len()].to_string()
    } else {
        s
    };

    Ok(Arc::new(Object::String(result)))
}

/// Find the index of a substring in a string
/// 
/// # Arguments
/// 
/// * `args[0]` - The string to search in
/// * `args[1]` - The substring to search for
/// 
/// # Returns
/// 
/// The index of the first occurrence of the substring, or -1 if not found
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn index(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("index requires 2 arguments".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to index must be a string".to_string(),
            ))
        }
    };

    let substr = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to index must be a string".to_string(),
            ))
        }
    };

    let index = match s.find(&substr) {
        Some(idx) => idx as i64,
        None => -1,
    };

    Ok(Arc::new(Object::Integer(index)))
}

/// Find the last index of a substring in a string
/// 
/// # Arguments
/// 
/// * `args[0]` - The string to search in
/// * `args[1]` - The substring to search for
/// 
/// # Returns
/// 
/// The index of the last occurrence of the substring, or -1 if not found
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn last_index(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("last_index requires 2 arguments".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to last_index must be a string".to_string(),
            ))
        }
    };

    let substr = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to last_index must be a string".to_string(),
            ))
        }
    };

    let index = match s.rfind(&substr) {
        Some(idx) => idx as i64,
        None => -1,
    };

    Ok(Arc::new(Object::Integer(index)))
}

/// Replace occurrences of a substring in a string
/// 
/// # Arguments
/// 
/// * `args[0]` - The string to modify
/// * `args[1]` - The substring to replace
/// * `args[2]` - The replacement string
/// * `args[3]` (optional) - The number of replacements to make (default: 1)
/// 
/// # Returns
/// 
/// A new string with the replacements made
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn replace(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime("replace requires at least 3 arguments".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to replace must be a string".to_string(),
            ))
        }
    };

    let old = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to replace must be a string".to_string(),
            ))
        }
    };

    let new = match &*args[2] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Third argument to replace must be a string".to_string(),
            ))
        }
    };

    // Get count if provided
    let count = if args.len() > 3 {
        match &*args[3] {
            Object::Integer(n) => *n as usize,
            _ => {
                return Err(Error::Runtime(
                    "Fourth argument to replace must be an integer".to_string(),
                ))
            }
        }
    } else {
        1 // Default to replacing just one occurrence
    };

    // Perform replacement
    let mut result = s.clone();
    let mut replaced = 0;
    
    // Simple implementation for limited replacements
    if count > 0 {
        let mut current_pos = 0;
        while let Some(pos) = result[current_pos..].find(&old) {
            let absolute_pos = current_pos + pos;
            let before = result[..absolute_pos].to_string();
            let after = result[absolute_pos + old.len()..].to_string();
            result = before + &new + &after;
            
            // Move position past the replacement to avoid infinite loop
            current_pos = absolute_pos + new.len();
            
            replaced += 1;
            if replaced >= count {
                break;
            }
        }
    }

    Ok(Arc::new(Object::String(result)))
}

/// Replace all occurrences of a substring in a string
/// 
/// # Arguments
/// 
/// * `args[0]` - The string to modify
/// * `args[1]` - The substring to replace
/// * `args[2]` - The replacement string
/// 
/// # Returns
/// 
/// A new string with all occurrences replaced
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn replace_all(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime("replace_all requires 3 arguments".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to replace_all must be a string".to_string(),
            ))
        }
    };

    let old = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to replace_all must be a string".to_string(),
            ))
        }
    };

    let new = match &*args[2] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Third argument to replace_all must be a string".to_string(),
            ))
        }
    };

    // Simple implementation using standard library
    let result = s.replace(&old, &new);

    Ok(Arc::new(Object::String(result)))
}

/// Repeat a string n times
/// 
/// # Arguments
/// 
/// * `args[0]` - The string to repeat
/// * `args[1]` - The number of times to repeat it
/// 
/// # Returns
/// 
/// A new string consisting of the input string repeated n times
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn repeat(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("repeat requires 2 arguments".to_string()));
    }

    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to repeat must be a string".to_string(),
            ))
        }
    };

    let count = match &*args[1] {
        Object::Integer(n) => {
            if *n < 0 {
                return Err(Error::Runtime(
                    "Repeat count must be non-negative".to_string(),
                ));
            }
            *n as usize
        },
        _ => {
            return Err(Error::Runtime(
                "Second argument to repeat must be an integer".to_string(),
            ))
        }
    };

    Ok(Arc::new(Object::String(s.repeat(count))))
}
