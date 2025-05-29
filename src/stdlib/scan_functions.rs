//! Scan functions for vibez package
//! Provides text scanning functionality similar to Go's fmt.Scan and fmt.Scanln

use crate::memory::{Traceable, Tag, Visitor};
use crate::object::{self, Object};
use crate::error::Error;
use crate::prelude::*;
use std::sync::Arc;

/// Scans text using format specifiers
/// Similar to Go's fmt.Scan
pub fn scan_string(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::new("ArgumentError", 
            format!("scan_string requires at least 2 arguments, got {}", args.len()),
            None));
    }

    // First argument is the string to scan
    let input = match &*args[0] {
        Object::String(s) => s,
        _ => return Err(Error::new("TypeError", 
            format!("scan_string first argument must be a string, got {:?}", args[0]),
            None)),
    };

    // Remaining arguments must be references to variables
    let scan_result = parse_and_assign_args(input, &args[1..], false)?;
    
    Ok(Arc::new(Object::Integer(scan_result)))
}

/// Scans a line of text (up to a newline) using format specifiers
/// Similar to Go's fmt.Scanln
pub fn scanln_string(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::new("ArgumentError", 
            format!("scanln_string requires at least 2 arguments, got {}", args.len()),
            None));
    }

    // First argument is the string to scan
    let input = match &*args[0] {
        Object::String(s) => s,
        _ => return Err(Error::new("TypeError", 
            format!("scanln_string first argument must be a string, got {:?}", args[0]),
            None)),
    };

    // For scanln, only scan up to the first newline
    let line = match input.split_once('\n') {
        Some((line, _)) => line,
        None => input, // No newline found, use the entire string
    };

    // Remaining arguments must be references to variables
    let scan_result = parse_and_assign_args(line, &args[1..], true)?;
    
    Ok(Arc::new(Object::Integer(scan_result)))
}

/// Parse and assign values to the provided arguments
fn parse_and_assign_args(input: &str, var_refs: &[Arc<Object>], respect_whitespace: bool) -> Result<i64, Error> {
    // Split the input string by whitespace
    let mut parts: Vec<&str> = if respect_whitespace {
        input.split_whitespace().collect()
    } else {
        input.split_whitespace().collect()
    };
    
    let mut items_scanned = 0;
    
    for (i, var_ref) in var_refs.iter().enumerate() {
        if i >= parts.len() {
            break; // No more parts to scan
        }
        
        // Get the variable reference and assign the parsed value
        match &**var_ref {
            Object::Reference(ref_cell) => {
                let obj_type = ref_cell.borrow().downcast_type(); 
                let part = parts[i];
                
                // Parse the value based on the target variable type
                let value = match obj_type.as_str() {
                    "normie" => {
                        // Parse as integer
                        match part.parse::<i64>() {
                            Ok(n) => Object::Integer(n),
                            Err(_) => return Err(Error::new("ParseError", 
                                format!("Could not parse '{}' as an integer", part),
                                None)),
                        }
                    },
                    "meal" => {
                        // Parse as float
                        match part.parse::<f64>() {
                            Ok(f) => Object::Float(f),
                            Err(_) => return Err(Error::new("ParseError", 
                                format!("Could not parse '{}' as a float", part),
                                None)),
                        }
                    },
                    "tea" => {
                        // Parse as string (just use the part as-is)
                        Object::String(part.to_string())
                    },
                    "cap" => {
                        // Parse as boolean
                        match part.to_lowercase().as_str() {
                            "true" | "t" | "1" | "yes" | "y" => Object::Boolean(true),
                            "false" | "f" | "0" | "no" | "n" => Object::Boolean(false),
                            _ => return Err(Error::new("ParseError", 
                                format!("Could not parse '{}' as a boolean", part),
                                None)),
                        }
                    },
                    _ => return Err(Error::new("TypeError", 
                        format!("Unsupported type for scanning: {}", obj_type),
                        None)),
                };
                
                // Assign the parsed value to the reference
                *ref_cell.borrow_mut() = value;
                items_scanned += 1;
            },
            _ => return Err(Error::new("TypeError", 
                format!("Expected a reference, got {:?}", var_ref),
                None)),
        }
    }
    
    Ok(items_scanned)
}