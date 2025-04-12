//! Formatted I/O functions for CURSED programs
//!
//! The vibez package provides functionality for formatted input and output
//! operations, similar to Go's fmt package. It supports printing to standard
//! output with formatting options, string formatting, and scanning input from
//! standard input or strings.
//!
//! Key functions include:
//!
//! - `spill`: Print arguments followed by a newline (like fmt.Println)
//! - `spillf`: Formatted printing (like fmt.Printf)
//! - `spillstr`: Returns a formatted string (like fmt.Sprintf)
//! - `scan`: Read from standard input into variables (like fmt.Scan)
//! - `scanln`: Read a line from standard input (like fmt.Scanln)

use std::rc::Rc;
use std::fmt::Write;
use crate::object::Object;
use crate::error::Error;

/// Prints arguments to standard output followed by a newline
///
/// This function is the equivalent of fmt.Println in Go. It takes any number 
/// of arguments, converts them to strings, and prints them separated by spaces
/// and followed by a newline character.
///
/// # Arguments
///
/// * `args` - A slice of Object references to print
///
/// # Returns
///
/// Result<Rc<Object>, Error> - Ok with null object if successful, Error otherwise
pub fn spill(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    println!();
    Ok(Rc::new(Object::Null))
}

/// Formatted print with format string and args
pub fn spillf(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("spillf requires a format string".to_string()));
    }
    
    let format_str = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("First argument to spillf must be a string".to_string())),
    };
    
    let result = format_string(&format_str, &args[1..])?
        .unwrap_or_else(|| String::new());
    
    print!("{}", result);
    Ok(Rc::new(Object::Null))
}

/// Return formatted string
pub fn spillstr(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("spillstr requires a format string".to_string()));
    }
    
    let format_str = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("First argument to spillstr must be a string".to_string())),
    };
    
    let result = format_string(&format_str, &args[1..])?
        .unwrap_or_else(|| String::new());
    
    Ok(Rc::new(Object::String(result)))
}

/// Internal helper to format a string with arguments
fn format_string(format_str: &str, args: &[Rc<Object>]) -> Result<Option<String>, Error> {
    let mut result = String::new();
    let mut chars = format_str.chars().peekable();
    let mut arg_index = 0;
    
    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(next) = chars.peek() {
                match next {
                    's' => {
                        // String format
                        chars.next(); // consume the 's'
                        if arg_index < args.len() {
                            write!(result, "{}", args[arg_index]).unwrap();
                        } else {
                            return Err(Error::Runtime("Not enough arguments for format string".to_string()));
                        }
                        arg_index += 1;
                    },
                    'd' => {
                        // Integer format
                        chars.next(); // consume the 'd'
                        if arg_index < args.len() {
                            match &*args[arg_index] {
                                Object::Integer(i) => write!(result, "{}", i).unwrap(),
                                _ => return Err(Error::Runtime(format!("Format %d requires an integer, got {}", args[arg_index].type_name()))),
                            }
                        } else {
                            return Err(Error::Runtime("Not enough arguments for format string".to_string()));
                        }
                        arg_index += 1;
                    },
                    'f' => {
                        // Float format
                        chars.next(); // consume the 'f'
                        if arg_index < args.len() {
                            match &*args[arg_index] {
                                Object::Float(f) => write!(result, "{}", f).unwrap(),
                                Object::Integer(i) => write!(result, "{}", *i as f64).unwrap(),
                                _ => return Err(Error::Runtime(format!("Format %f requires a float, got {}", args[arg_index].type_name()))),
                            }
                        } else {
                            return Err(Error::Runtime("Not enough arguments for format string".to_string()));
                        }
                        arg_index += 1;
                    },
                    '%' => {
                        // Literal %
                        chars.next(); // consume the second '%'
                        result.push('%');
                    },
                    _ => {
                        // Unknown format specifier, treat as literal
                        result.push(c);
                    }
                }
            } else {
                // % at end of string
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }
    
    Ok(Some(result))
}

/// Scan input into variables from stdin
pub fn scan(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .map_err(|e| Error::Runtime(format!("Failed to read from stdin: {}", e)))?;
    
    scan_string_impl(&[Rc::new(Object::String(input.trim().to_string()))]
        .iter()
        .chain(args.iter())
        .cloned()
        .collect::<Vec<_>>())
}

/// Scan a line of input into variables from stdin
pub fn scanln(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .map_err(|e| Error::Runtime(format!("Failed to read from stdin: {}", e)))?;
    
    scanln_string_impl(&[Rc::new(Object::String(input.trim().to_string()))]
        .iter()
        .chain(args.iter())
        .cloned()
        .collect::<Vec<_>>())
}

/// Scan input from a string for testing purposes
pub fn scan_string(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    scan_string_impl(args)
}

/// Scan a line of input from a string for testing purposes
pub fn scanln_string(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    scanln_string_impl(args)
}

/// Internal implementation for scan_string
fn scan_string_impl(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("scan_string requires at least 1 argument".to_string()));
    }
    
    let input = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("First argument to scan_string must be a string".to_string())),
    };
    
    let parts: Vec<&str> = input.split_whitespace().collect();
    let reference_args = &args[1..];
    
    for (i, arg) in reference_args.iter().enumerate() {
        if i >= parts.len() {
            return Err(Error::Runtime("Not enough values in input string".to_string()));
        }
        
        match &**arg {
            Object::Reference(ref_obj) => {
                let mut ref_mut = ref_obj.borrow_mut();
                let value_str = parts[i];
                
                match &mut *ref_mut {
                    Object::Integer(_) => {
                        let parsed = value_str.parse::<i64>()
                            .map_err(|_| Error::Runtime(format!("Failed to parse '{}' as integer", value_str)))?;
                        *ref_mut = Object::Integer(parsed);
                    },
                    Object::Float(_) => {
                        let parsed = value_str.parse::<f64>()
                            .map_err(|_| Error::Runtime(format!("Failed to parse '{}' as float", value_str)))?;
                        *ref_mut = Object::Float(parsed);
                    },
                    Object::String(_) => {
                        *ref_mut = Object::String(value_str.to_string());
                    },
                    Object::Boolean(_) => {
                        let parsed = match value_str.to_lowercase().as_str() {
                            "true" | "based" => true,
                            "false" | "sus" => false,
                            _ => return Err(Error::Runtime(format!("Failed to parse '{}' as boolean", value_str))),
                        };
                        *ref_mut = Object::Boolean(parsed);
                    },
                    _ => return Err(Error::Runtime(format!("Unsupported reference type for scanning: {}", ref_mut.type_name()))),
                }
            },
            _ => return Err(Error::Runtime("Arguments to scan_string must be references".to_string())),
        }
    }
    
    Ok(Rc::new(Object::Integer(reference_args.len() as i64)))
}

/// Internal implementation for scanln_string
fn scanln_string_impl(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("scanln_string requires at least 1 argument".to_string()));
    }
    
    let input = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("First argument to scanln_string must be a string".to_string())),
    };
    
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err(Error::Runtime("Input string contains no lines".to_string()));
    }
    
    let first_line = lines[0];
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let reference_args = &args[1..];
    
    for (i, arg) in reference_args.iter().enumerate() {
        if i >= parts.len() {
            return Err(Error::Runtime("Not enough values in input line".to_string()));
        }
        
        match &**arg {
            Object::Reference(ref_obj) => {
                let mut ref_mut = ref_obj.borrow_mut();
                let value_str = parts[i];
                
                match &mut *ref_mut {
                    Object::Integer(_) => {
                        let parsed = value_str.parse::<i64>()
                            .map_err(|_| Error::Runtime(format!("Failed to parse '{}' as integer", value_str)))?;
                        *ref_mut = Object::Integer(parsed);
                    },
                    Object::Float(_) => {
                        let parsed = value_str.parse::<f64>()
                            .map_err(|_| Error::Runtime(format!("Failed to parse '{}' as float", value_str)))?;
                        *ref_mut = Object::Float(parsed);
                    },
                    Object::String(_) => {
                        *ref_mut = Object::String(value_str.to_string());
                    },
                    Object::Boolean(_) => {
                        let parsed = match value_str.to_lowercase().as_str() {
                            "true" | "based" => true,
                            "false" | "sus" => false,
                            _ => return Err(Error::Runtime(format!("Failed to parse '{}' as boolean", value_str))),
                        };
                        *ref_mut = Object::Boolean(parsed);
                    },
                    _ => return Err(Error::Runtime(format!("Unsupported reference type for scanning: {}", ref_mut.type_name()))),
                }
            },
            _ => return Err(Error::Runtime("Arguments to scanln_string must be references".to_string())),
        }
    }
    
    Ok(Rc::new(Object::Integer(reference_args.len() as i64)))
}