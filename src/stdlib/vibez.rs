//! The vibez package provides formatted I/O functions.
//! This is equivalent to the fmt package in Go.

use std::rc::Rc;
use std::fmt::Write;
use crate::object::Object;
use crate::error::Error;

/// Print arguments followed by a newline
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

/// Scan input into variables
pub fn scan(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Placeholder implementation
    Err(Error::Runtime("scan not implemented yet".to_string()))
}

/// Scan a line of input into variables
pub fn scanln(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Placeholder implementation
    Err(Error::Runtime("scanln not implemented yet".to_string()))
}