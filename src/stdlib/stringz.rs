//! The stringz package provides string manipulation functions.
//! This is equivalent to the strings package in Go.

use std::rc::Rc;
use crate::object::Object;
use crate::error::Error;

/// Check if s contains substr
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

/// Split s by separator
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