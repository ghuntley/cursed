use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::stdlib::value::Value;
use crate::error::CursedError;
use super::formatter::format_log_entry;
use super::flags::LstdFlags;

/// Logger represents an active logging object that outputs formatted text to a Writer.
/// It can be used concurrently from multiple goroutines as it guarantees serialized
/// access to the Writer.
#[derive(Clone)]
pub struct Logger {
    inner: Arc<Mutex<LoggerInner>>,
}

struct LoggerInner {
    output: Box<dyn Write + Send>,
    prefix: String,
    flags: i32,
}

impl Logger {
    /// Create a new Logger with the specified output destination, prefix, and flags
    pub fn new(output: Box<dyn Write + Send>, prefix: String, flags: i32) -> Self {
        Logger {
            inner: Arc::new(Mutex::new(LoggerInner {
                output,
                prefix,
                flags,
            })),
        }
    }

    /// spill - Print args followed by newline
    pub fn spill(&self, args: &[Value]) -> Result<(), CursedError> {
        let message = args.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        self.output(2, &message)
    }

    /// spillf - Print formatted string
    pub fn spillf(&self, format: &str, args: &[Value]) -> Result<(), CursedError> {
        let formatted = self.format_string(format, args)?;
        self.output(2, &formatted)
    }

    /// fatal - Print args and exit with code 1
    pub fn fatal(&self, args: &[Value]) -> ! {
        let _ = self.spill(args);
        std::process::exit(1);
    }

    /// fatalf - Print formatted string and exit with code 1
    pub fn fatalf(&self, format: &str, args: &[Value]) -> ! {
        let _ = self.spillf(format, args);
        std::process::exit(1);
    }

    /// shook - Print args and trigger panic
    pub fn shook(&self, args: &[Value]) -> ! {
        let _ = self.spill(args);
        panic!("shook triggered");
    }

    /// shookf - Print formatted string and trigger panic
    pub fn shookf(&self, format: &str, args: &[Value]) -> ! {
        let _ = self.spillf(format, args);
        panic!("shookf triggered");
    }

    /// output - Low-level output method
    pub fn output(&self, call_depth: usize, message: &str) -> Result<(), CursedError> {
        let mut inner = self.inner.lock().map_err(|_| {
            CursedError::Runtime("Failed to acquire logger lock".to_string())
        })?;

        let formatted_entry = format_log_entry(
            &inner.prefix,
            inner.flags,
            call_depth,
            message,
        )?;

        inner.output.write_all(formatted_entry.as_bytes()).map_err(|e| {
            CursedError::Runtime(format!("Failed to write log entry: {}", e))
        })?;

        inner.output.flush().map_err(|e| {
            CursedError::Runtime(format!("Failed to flush log output: {}", e))
        })?;

        Ok(())
    }

    /// setFlags - Set output flags
    pub fn set_flags(&self, flags: i32) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.flags = flags;
        }
    }

    /// setOutput - Set output destination
    pub fn set_output(&self, writer: Box<dyn Write + Send>) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.output = writer;
        }
    }

    /// setPrefix - Set output prefix
    pub fn set_prefix(&self, prefix: String) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.prefix = prefix;
        }
    }

    /// flags - Get current flags
    pub fn flags(&self) -> i32 {
        self.inner.lock()
            .map(|inner| inner.flags)
            .unwrap_or(0)
    }

    /// prefix - Get current prefix
    pub fn prefix(&self) -> String {
        self.inner.lock()
            .map(|inner| inner.prefix.clone())
            .unwrap_or_default()
    }

    /// Simple format string implementation for spillf
    fn format_string(&self, format: &str, args: &[Value]) -> Result<String, CursedError> {
        let mut result = String::new();
        let mut chars = format.chars().peekable();
        let mut arg_index = 0;

        while let Some(ch) = chars.next() {
            if ch == '{' {
                if chars.peek() == Some(&'{') {
                    // Escaped brace
                    chars.next();
                    result.push('{');
                } else if chars.peek() == Some(&'}') {
                    // Simple placeholder
                    chars.next();
                    if arg_index < args.len() {
                        result.push_str(&args[arg_index].to_string());
                        arg_index += 1;
                    } else {
                        return Err(CursedError::Runtime(
                            "Not enough arguments for format string".to_string()
                        ));
                    }
                } else {
                    // Look for indexed placeholder like {0}, {1}
                    let mut index_str = String::new();
                    let mut found_closing = false;
                    
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch == '}' {
                            chars.next();
                            found_closing = true;
                            break;
                        } else if next_ch.is_ascii_digit() {
                            index_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    if found_closing && !index_str.is_empty() {
                        if let Ok(index) = index_str.parse::<usize>() {
                            if index < args.len() {
                                result.push_str(&args[index].to_string());
                            } else {
                                return Err(CursedError::Runtime(
                                    format!("Argument index {} out of bounds", index)
                                ));
                            }
                        } else {
                            return Err(CursedError::Runtime(
                                "Invalid format placeholder".to_string()
                            ));
                        }
                    } else {
                        result.push(ch);
                    }
                }
            } else if ch == '}' && chars.peek() == Some(&'}') {
                // Escaped closing brace
                chars.next();
                result.push('}');
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }
}

/// Convenience function to create a new logger
pub fn new_logger(output: Box<dyn Write + Send>, prefix: String, flags: i32) -> Logger {
    Logger::new(output, prefix, flags)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_logger_creation() {
        let output = Box::new(Cursor::new(Vec::new()));
        let logger = Logger::new(output, "test: ".to_string(), LstdFlags);
        
        assert_eq!(logger.prefix(), "test: ");
        assert_eq!(logger.flags(), LstdFlags);
    }

    #[test]
    fn test_logger_spill() {
        let output = Box::new(Cursor::new(Vec::new()));
        let logger = Logger::new(output, "".to_string(), 0);
        
        let args = vec![
            Value::String("Hello".to_string()),
            Value::String("World".to_string())
        ];
        
        assert!(logger.spill(&args).is_ok());
    }

    #[test]
    fn test_logger_spillf() {
        let output = Box::new(Cursor::new(Vec::new()));
        let logger = Logger::new(output, "".to_string(), 0);
        
        let args = vec![Value::String("World".to_string())];
        assert!(logger.spillf("Hello {}!", &args).is_ok());
    }

    #[test]
    fn test_format_string() {
        let output = Box::new(Cursor::new(Vec::new()));
        let logger = Logger::new(output, "".to_string(), 0);
        
        let args = vec![
            Value::String("World".to_string()),
            Value::Integer(42)
        ];
        
        let result = logger.format_string("Hello {}, number: {}", &args).unwrap();
        assert_eq!(result, "Hello World, number: 42");
        
        let result2 = logger.format_string("Hello {1}, number: {0}", &args).unwrap();
        assert_eq!(result2, "Hello 42, number: World");
    }

    #[test]
    fn test_logger_configuration() {
        let output = Box::new(Cursor::new(Vec::new()));
        let logger = Logger::new(output, "initial: ".to_string(), 0);
        
        logger.set_prefix("new: ".to_string());
        logger.set_flags(42);
        
        assert_eq!(logger.prefix(), "new: ");
        assert_eq!(logger.flags(), 42);
    }
    
    #[test]
    fn test_escaped_braces() {
        let output = Box::new(Cursor::new(Vec::new()));
        let logger = Logger::new(output, "".to_string(), 0);
        
        let args = vec![Value::String("test".to_string())];
        let result = logger.format_string("{{{}}} and {{}}", &args).unwrap();
        assert_eq!(result, "{test} and {}");
    }
}
