//! The regex_vibez package provides regular expression functionality.
//! This is equivalent to the regexp package in Go.

use std::rc::Rc;
use crate::object::Object;
use crate::error::Error;

/// Check if a string matches a regular expression
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

/// Find all matches of a regular expression in a string
pub fn find_all(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("find_all requires 2 arguments: pattern and string".to_string()));
    }
    
    // Simplified implementation - return empty array
    let results: Vec<Object> = Vec::new();
    Ok(Rc::new(Object::Array(results)))
}

/// Replace all matches of a regular expression in a string
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