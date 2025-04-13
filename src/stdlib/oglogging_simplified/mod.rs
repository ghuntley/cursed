//! Logging package for CURSED language
//!
//! Provides facilities for recording program execution with
//! configurable output formats and destinations, log levels, and
//! advanced formatting options.

use crate::error::Error;
use std::{cell::RefCell, fmt::Write, io, ptr::addr_of};
use std::collections::HashMap;

use crate::prelude::*;

// Flag constants for Logger
pub const LDATE: i64 = 1 << 0;         // the date: 2025/04/13
pub const LTIME: i64 = 1 << 1;         // the time: 15:04:05
pub const LMICROSECONDS: i64 = 1 << 2; // microsecond resolution: 15:04:05.123123
pub const LLONGFILE: i64 = 1 << 3;     // full file name and line number: /a/b/c/d.go:23
pub const LSHORTFILE: i64 = 1 << 4;    // final file name element and line number: d.go:23
pub const LUTC: i64 = 1 << 5;          // use UTC rather than local time zone
pub const LMSGPREFIX: i64 = 1 << 6;    // move prefix to before message
pub const LSTDFLAGS: i64 = LDATE | LTIME; // initial values for the standard logger

// Log level constants
pub const LDEBUG: i64 = 0;    // Debug level - lowest level, most verbose
pub const LINFO: i64 = 1;     // Info level - general operational information
pub const LWARNING: i64 = 2;  // Warning level - something to pay attention to
pub const LERROR: i64 = 3;    // Error level - something went wrong
pub const LFATAL: i64 = 4;    // Fatal level - severe error, application will exit

/// Standard logger global state
static mut STD_LOGGER_PREFIX: String = String::new();
static mut STD_LOGGER_FLAGS: i64 = LSTDFLAGS;
static mut STD_LOGGER_LEVEL: i64 = LINFO; // Default to INFO level

/// Formatting helper function
fn format_message(args: &[crate::object::Object]) -> String {
    args.iter()
        .map(|arg| format!("{}", arg))
        .collect::<Vec<String>>()
        .join(" ")
}

/// Print a log message to stderr
pub fn spill(args: &[crate::object::Object]) -> Result<(), Error> {
    let message = format_message(args);
    eprintln!("{}", message);
    Ok(())
}

/// Print a formatted log message to stderr
pub fn spillf(format_str: &str, args: &[crate::object::Object]) -> Result<(), Error> {
    let result = format_with_args(format_str, args);
    eprintln!("{}", result);
    Ok(())
}

/// Advanced format with sprintf-like functionality
pub fn format_with_args(format_str: &str, args: &[crate::object::Object]) -> String {
    let mut result = String::new();
    let mut chars = format_str.chars().peekable();
    let mut arg_index = 0;
    
    while let Some(c) = chars.next() {
        if c != '%' {
            result.push(c);
            continue;
        }
        
        // Check next char to see if it's another %
        if let Some(&next_char) = chars.peek() {
            if next_char == '%' {
                // Escaped %, just add a single %
                chars.next(); // Skip the second %
                result.push('%');
                continue;
            }
        }
        
        // Handle format specifiers
        let mut format_spec = String::new();
        let mut explicit_arg_index = None;
        
        // Check for positional arguments like %[1]v
        if let Some(&next_char) = chars.peek() {
            if next_char == '[' {
                chars.next(); // Skip '[
                let mut index_str = String::new();
                while let Some(next) = chars.next() {
                    if next == ']' {
                        break;
                    }
                    index_str.push(next);
                }
                if let Ok(idx) = index_str.parse::<usize>() {
                    explicit_arg_index = Some(idx - 1); // Convert to 0-based
                }
            }
        }
        
        // Get width and precision specifiers
        let mut width = String::new();
        while let Some(&next_char) = chars.peek() {
            if next_char.is_digit(10) || next_char == '.' || next_char == '-' || next_char == '+' {
                width.push(next_char);
                chars.next();
            } else {
                break;
            }
        }
        
        // Get the format verb
        let format_verb = chars.next().unwrap_or('v');
        format_spec.push(format_verb);
        
        // Get the argument to format
        let idx = explicit_arg_index.unwrap_or_else(|| {
            let current = arg_index;
            arg_index += 1;
            current
        });
        
        // Format the argument based on the verb
        if idx < args.len() {
            let arg = &args[idx];
            match format_verb {
                'v' => result.push_str(&format!("{}", arg)),
                's' => match arg {
                    crate::object::Object::String(s) => {
                        if width.is_empty() {
                            result.push_str(s);
                        } else if width.starts_with('-') {
                            // Left-justified
                            if let Ok(w) = width[1..].parse::<usize>() {
                                result.push_str(&format!("{:<width$}", s, width=w));
                            } else {
                                result.push_str(s);
                            }
                        } else if let Ok(w) = width.parse::<usize>() {
                            // Right-justified
                            result.push_str(&format!("{:>width$}", s, width=w));
                        } else {
                            result.push_str(s);
                        }
                    },
                    _ => result.push_str(&format!("{}", arg)),
                },
                'd' | 'i' => match arg {
                    crate::object::Object::Integer(i) => {
                        if width.is_empty() {
                            result.push_str(&i.to_string());
                        } else if width.starts_with('-') {
                            // Left-justified
                            if let Ok(w) = width[1..].parse::<usize>() {
                                result.push_str(&format!("{:<width$}", i, width=w));
                            } else {
                                result.push_str(&i.to_string());
                            }
                        } else if width.starts_with('+') {
                            // Force sign
                            if let Ok(w) = width[1..].parse::<usize>() {
                                result.push_str(&format!("{:+width$}", i, width=w));
                            } else {
                                result.push_str(&format!("{:+}", i));
                            }
                        } else if let Ok(w) = width.parse::<usize>() {
                            // Right-justified
                            result.push_str(&format!("{:>width$}", i, width=w));
                        } else {
                            result.push_str(&i.to_string());
                        }
                    },
                    _ => result.push_str(&format!("{}", arg)),
                },
                'f' => match arg {
                    crate::object::Object::Float(f) => {
                        if width.contains('.') {
                            let parts: Vec<&str> = width.split('.').collect();
                            if parts.len() == 2 {
                                let width_str = parts[0];
                                let prec_str = parts[1];
                                if let Ok(prec) = prec_str.parse::<usize>() {
                                    if width_str.is_empty() {
                                        result.push_str(&format!("{:.prec$}", f, prec=prec));
                                    } else if let Ok(w) = width_str.parse::<usize>() {
                                        result.push_str(&format!("{:width$.prec$}", f, width=w, prec=prec));
                                    } else {
                                        result.push_str(&format!("{:.prec$}", f, prec=prec));
                                    }
                                } else {
                                    result.push_str(&f.to_string());
                                }
                            } else {
                                result.push_str(&f.to_string());
                            }
                        } else if width.is_empty() {
                            result.push_str(&f.to_string());
                        } else if let Ok(w) = width.parse::<usize>() {
                            result.push_str(&format!("{:>width$}", f, width=w));
                        } else {
                            result.push_str(&f.to_string());
                        }
                    },
                    _ => result.push_str(&format!("{}", arg)),
                },
                't' => match arg {
                    crate::object::Object::Boolean(b) => result.push_str(&b.to_string()),
                    _ => result.push_str(&format!("{}", arg)),
                },
                'x' => match arg {
                    crate::object::Object::Integer(i) => result.push_str(&format!("{:x}", i)),
                    _ => result.push_str(&format!("{}", arg)),
                },
                'X' => match arg {
                    crate::object::Object::Integer(i) => result.push_str(&format!("{:X}", i)),
                    _ => result.push_str(&format!("{}", arg)),
                },
                'o' => match arg {
                    crate::object::Object::Integer(i) => result.push_str(&format!("{:o}", i)),
                    _ => result.push_str(&format!("{}", arg)),
                },
                'b' => match arg {
                    crate::object::Object::Integer(i) => result.push_str(&format!("{:b}", i)),
                    _ => result.push_str(&format!("{}", arg)),
                },
                _ => result.push_str(&format!("{}", arg)),
            }
        } else {
            // Not enough arguments, just append the format specifier as-is
            result.push('%');
            if let Some(index) = explicit_arg_index {
                result.push_str(&format!("[{}]", index + 1));
            }
            result.push_str(&width);
            result.push(format_verb);
        }
    }
    
    result
}

/// Print a log message and exit with status 1
pub fn fatal(args: &[crate::object::Object]) -> ! {
    let _ = spill(args);
    std::process::exit(1);
}

/// Print a formatted log message and exit with status 1
pub fn fatalf(format_str: &str, args: &[crate::object::Object]) -> ! {
    let _ = spillf(format_str, args);
    std::process::exit(1);
}

/// Print a log message and trigger a panic
pub fn panic_log(args: &[crate::object::Object]) -> ! {
    let message = format_message(args);
    std::panic!("{}", message);
}

/// Print a formatted log message and trigger a panic
pub fn panicf(format_str: &str, args: &[crate::object::Object]) -> ! {
    let result = format_with_args(format_str, args);
    std::panic!("{}", result);
}

/// Set the output flags for the standard logger
pub fn set_flags(flag: i64) {
    unsafe {
        STD_LOGGER_FLAGS = flag;
    }
}

/// Get the current output flags for the standard logger
pub fn flags() -> i64 {
    unsafe {
        STD_LOGGER_FLAGS
    }
}

/// Set the output prefix for the standard logger
pub fn set_prefix(prefix: String) {
    unsafe {
        STD_LOGGER_PREFIX = prefix;
    }
}

/// Get the current output prefix for the standard logger
pub fn prefix() -> String {
    unsafe {
        STD_LOGGER_PREFIX.clone()
    }
}

/// Set the minimum log level
pub fn set_level(level: i64) {
    unsafe {
        STD_LOGGER_LEVEL = level;
    }
}

/// Get the current minimum log level
pub fn level() -> i64 {
    unsafe {
        STD_LOGGER_LEVEL
    }
}

/// Print a debug log message (level 0)
pub fn debug(args: &[crate::object::Object]) -> Result<(), Error> {
    unsafe {
        if STD_LOGGER_LEVEL <= LDEBUG {
            let mut message = String::new();
            if STD_LOGGER_PREFIX.len() > 0 {
                message.push_str(unsafe { &*addr_of!(STD_LOGGER_PREFIX) });
                message.push_str(" ");
            }
            message.push_str("DEBUG: ");
            message.push_str(&format_message(args));
            eprintln!("{}", message);
        }
    }
    Ok(())
}

/// Print a formatted debug log message (level 0)
pub fn debugf(format_str: &str, args: &[crate::object::Object]) -> Result<(), Error> {
    unsafe {
        if STD_LOGGER_LEVEL <= LDEBUG {
            let mut message = String::new();
            if STD_LOGGER_PREFIX.len() > 0 {
                message.push_str(unsafe { &*addr_of!(STD_LOGGER_PREFIX) });
                message.push_str(" ");
            }
            message.push_str("DEBUG: ");
            message.push_str(&format_with_args(format_str, args));
            eprintln!("{}", message);
        }
    }
    Ok(())
}

/// Print an info log message (level 1)
pub fn info(args: &[crate::object::Object]) -> Result<(), Error> {
    unsafe {
        if STD_LOGGER_LEVEL <= LINFO {
            let mut message = String::new();
            if STD_LOGGER_PREFIX.len() > 0 {
                message.push_str(unsafe { &*addr_of!(STD_LOGGER_PREFIX) });
                message.push_str(" ");
            }
            message.push_str("INFO: ");
            message.push_str(&format_message(args));
            eprintln!("{}", message);
        }
    }
    Ok(())
}

/// Print a formatted info log message (level 1)
pub fn infof(format_str: &str, args: &[crate::object::Object]) -> Result<(), Error> {
    unsafe {
        if STD_LOGGER_LEVEL <= LINFO {
            let mut message = String::new();
            if STD_LOGGER_PREFIX.len() > 0 {
                message.push_str(unsafe { &*addr_of!(STD_LOGGER_PREFIX) });
                message.push_str(" ");
            }
            message.push_str("INFO: ");
            message.push_str(&format_with_args(format_str, args));
            eprintln!("{}", message);
        }
    }
    Ok(())
}

/// Print a warning log message (level 2)
pub fn warning(args: &[crate::object::Object]) -> Result<(), Error> {
    unsafe {
        if STD_LOGGER_LEVEL <= LWARNING {
            let mut message = String::new();
            if STD_LOGGER_PREFIX.len() > 0 {
                message.push_str(unsafe { &*addr_of!(STD_LOGGER_PREFIX) });
                message.push_str(" ");
            }
            message.push_str("WARNING: ");
            message.push_str(&format_message(args));
            eprintln!("{}", message);
        }
    }
    Ok(())
}

/// Print a formatted warning log message (level 2)
pub fn warningf(format_str: &str, args: &[crate::object::Object]) -> Result<(), Error> {
    unsafe {
        if STD_LOGGER_LEVEL <= LWARNING {
            let mut message = String::new();
            if STD_LOGGER_PREFIX.len() > 0 {
                message.push_str(unsafe { &*addr_of!(STD_LOGGER_PREFIX) });
                message.push_str(" ");
            }
            message.push_str("WARNING: ");
            message.push_str(&format_with_args(format_str, args));
            eprintln!("{}", message);
        }
    }
    Ok(())
}

/// Print an error log message (level 3)
pub fn error(args: &[crate::object::Object]) -> Result<(), Error> {
    unsafe {
        if STD_LOGGER_LEVEL <= LERROR {
            let mut message = String::new();
            if STD_LOGGER_PREFIX.len() > 0 {
                message.push_str(unsafe { &*addr_of!(STD_LOGGER_PREFIX) });
                message.push_str(" ");
            }
            message.push_str("ERROR: ");
            message.push_str(&format_message(args));
            eprintln!("{}", message);
        }
    }
    Ok(())
}

/// Print a formatted error log message (level 3)
pub fn errorf(format_str: &str, args: &[crate::object::Object]) -> Result<(), Error> {
    unsafe {
        if STD_LOGGER_LEVEL <= LERROR {
            let mut message = String::new();
            if STD_LOGGER_PREFIX.len() > 0 {
                message.push_str(unsafe { &*addr_of!(STD_LOGGER_PREFIX) });
                message.push_str(" ");
            }
            message.push_str("ERROR: ");
            message.push_str(&format_with_args(format_str, args));
            eprintln!("{}", message);
        }
    }
    Ok(())
}

// We don't need an init function for this simplified version