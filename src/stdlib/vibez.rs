//! The vibez package provides standardized logging utilities for CURSED applications.
//!
//! It offers structured logging with different severity levels, formatted output,
//! and customization options.

use crate::error::Error;
use crate::object::Object;
use std::rc::Rc;
use std::fmt::Write;

/// Log a message to standard output
pub fn spill(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        println!();
        return Ok(Rc::new(Object::Nil));
    }
    
    let message = format_args(args)?;
    println!("{}", message);
    
    Ok(Rc::new(Object::Nil))
}

/// Log a formatted message to standard output
pub fn spillf(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::InvalidArguments("spillf requires at least one argument".to_string()));
    }
    
    let format_str = args[0].to_string();
    let format_args = &args[1..];
    
    let result = format_string(&format_str, format_args)?;
    println!("{}", result);
    
    Ok(Rc::new(Object::Nil))
}

/// Format objects into a string and return it
pub fn spillstr(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Ok(Rc::new(Object::String("".to_string())));
    }
    
    let message = format_args(args)?;
    Ok(Rc::new(Object::String(message)))
}

/// Helper function to format arguments into a string
fn format_args(args: &[Rc<Object>]) -> Result<String, Error> {
    let mut result = String::new();
    
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        
        result.push_str(&arg.to_string());
    }
    
    Ok(result)
}

/// Helper function to format a string with placeholders
fn format_string(format_str: &str, args: &[Rc<Object>]) -> Result<String, Error> {
    let mut result = String::new();
    let mut arg_index = 0;
    let mut chars = format_str.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '{' {
            if let Some(&next) = chars.peek() {
                if next == '}' {
                    // Found a {} placeholder
                    chars.next();  // Consume the closing }
                    
                    if arg_index < args.len() {
                        result.push_str(&args[arg_index].to_string());
                        arg_index += 1;
                    } else {
                        return Err(Error::Runtime(format!(
                            "Not enough arguments for format string. Expected at least {}", 
                            arg_index + 1
                        )));
                    }
                    
                    continue;
                }
            }
        }
        
        // Regular character
        result.push(c);
    }
    
    Ok(result)
}

/// Helper module for documentation
mod docs {
    //! Documentation for the vibez logging module
    
    /// Why Logging Tests Are Important
    /// 
    /// Testing logging functionality is crucial because:
    /// 1. Logs are often the primary way to understand what's happening in a program
    /// 2. Format string errors can cause logging to fail silently or crash
    /// 3. Performance impact of logging can be significant
    /// 4. Log messages need to be correctly formatted for parsing/analysis
    /// 5. In production systems, logs may be the only diagnostic tool available
    #[cfg(test)]
    fn test_requirements() {}
}