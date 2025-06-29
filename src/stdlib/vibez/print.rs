//! Vibez print module - CURSED I/O operations with Gen Z flair

use crate::error_types::CursedError;
use crate::runtime::value::Value;
use super::format::{parse_format_string, format_value_with_spec};
use super::sprintf::sprintf;
use std::io::{self, Write};
use std::collections::HashMap;

/// Spill - Print values to stdout with newline (like println!)
pub fn spill(args: &[Value]) -> Result<(), CursedError> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            write!(handle, " ").map_err(|e| CursedError::Io(e.to_string()))?;
        }
        write!(handle, "{}", format_value(arg)).map_err(|e| CursedError::Io(e.to_string()))?;
    }
    writeln!(handle).map_err(|e| CursedError::Io(e.to_string()))?;
    handle.flush().map_err(|e| CursedError::Io(e.to_string()))?;
    
    Ok(())
}

/// Spillf - Print formatted string to stdout with newline
pub fn spillf(format: &str, args: &[Value]) -> Result<(), CursedError> {
    let output = format_string_advanced(format, args)?;
    println!("{}", output);
    Ok(())
}

/// Spillstr - Print string directly without formatting
pub fn spillstr(s: &str) -> Result<(), CursedError> {
    println!("{}", s);
    Ok(())
}

/// Scan - Read line from stdin
pub fn scan() -> Result<String, CursedError> {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).map_err(|e| CursedError::Io(e.to_string()))?;
    // Remove trailing newline
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    Ok(line)
}

/// Scanln - Read line from stdin (alias for scan)
pub fn scanln() -> Result<String, CursedError> {
    scan()
}

/// Format a Value for display
fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Integer(i) => i.to_string(),
        Value::Number(f) => f.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(format_value).collect();
            format!("[{}]", elements.join(", "))
        },
        Value::Object(obj) => {
            let entries: Vec<String> = obj.iter()
                .map(|(k, v)| format!("{}: {}", k, format_value(v)))
                .collect();
            format!("{{{}}}", entries.join(", "))
        },
        Value::Binary(data) => {
            format!("<binary data: {} bytes>", data.len())
        },
        _ => format!("<value: {:?}>", value),
    }
}

/// Simple format string implementation (legacy)
fn format_string(format: &str, args: &[Value]) -> Result<String, CursedError> {
    let mut result = String::new();
    let mut chars = format.chars().peekable();
    let mut arg_index = 0;
    
    while let Some(ch) = chars.next() {
        if ch == '{' && chars.peek() == Some(&'}') {
            chars.next(); // consume '}'
            if arg_index < args.len() {
                result.push_str(&format_value(&args[arg_index]));
                arg_index += 1;
            } else {
                return Err(CursedError::Runtime("Not enough arguments for format string".to_string()));
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok(result)
}

/// Advanced format string implementation with full format spec support
fn format_string_advanced(format: &str, args: &[Value]) -> Result<String, CursedError> {
    // Try to parse as advanced format string first
    match parse_format_string(format) {
        Ok((_, placeholders)) => {
            if placeholders.is_empty() {
                // No placeholders, return as-is
                return Ok(format.to_string());
            }
            
            let mut result = String::new();
            let mut chars = format.chars().peekable();
            let mut placeholder_index = 0;
            let mut arg_index = 0;
            
            while let Some(ch) = chars.next() {
                if ch == '{' {
                    if chars.peek() == Some(&'{') {
                        // Escaped brace
                        chars.next();
                        result.push('{');
                    } else {
                        // Find matching placeholder
                        if placeholder_index < placeholders.len() {
                            let placeholder = &placeholders[placeholder_index];
                            
                            // Determine which argument to use
                            let arg_idx = if let Some(idx) = placeholder.arg_index {
                                idx
                            } else {
                                arg_index
                            };
                            
                            if arg_idx < args.len() {
                                let formatted = format_value_with_spec(&args[arg_idx], &placeholder.spec)?;
                                result.push_str(&formatted);
                                arg_index += 1;
                            } else {
                                return Err(CursedError::Runtime("Not enough arguments for format string".to_string()));
                            }
                            
                            placeholder_index += 1;
                        }
                        
                        // Skip to end of placeholder
                        let mut brace_count = 1;
                        while let Some(ch) = chars.next() {
                            if ch == '{' {
                                brace_count += 1;
                            } else if ch == '}' {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    break;
                                }
                            }
                        }
                    }
                } else if ch == '}' {
                    if chars.peek() == Some(&'}') {
                        // Escaped brace
                        chars.next();
                        result.push('}');
                    } else {
                        result.push(ch);
                    }
                } else {
                    result.push(ch);
                }
            }
            
            Ok(result)
        },
        Err(_) => {
            // Fall back to simple format string
            format_string(format, args)
        }
    }
}

/// Printf-style formatting
pub fn spillf_printf(format: &str, args: &[Value]) -> Result<(), CursedError> {
    let output = sprintf(format, args)?;
    println!("{}", output);
    Ok(())
}
