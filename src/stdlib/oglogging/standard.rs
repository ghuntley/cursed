//! Standard logging convenience functions for the CURSED programming language
//! 
//! This module provides global logging functionality that operates on a shared
//! default logger instance, offering convenient access to logging operations
//! without needing to explicitly manage a logger instance.
//! 
//! # Functions
//! 
//! ## Output Functions
//! - `spill(args...)` - Print arguments with newline
//! - `spillf(format, args...)` - Formatted print with newline  
//! - `fatal(args...)` - Print and exit with code 1
//! - `fatalf(format, args...)` - Formatted print and exit with code 1
//! - `shook(args...)` - Print and panic
//! - `shookf(format, args...)` - Formatted print and panic
//! 
//! ## Configuration Functions
//! - `set_flags(flags)` - Set default logger flags
//! - `set_output(writer)` - Set default logger output destination
//! - `set_prefix(prefix)` - Set default logger prefix
//! - `flags()` - Get current flags
//! - `prefix()` - Get current prefix
//! - `writer()` - Get current writer (limited functionality)
//! 
//! # Thread Safety
//! 
//! All functions are thread-safe and can be called concurrently from multiple
//! goroutines. The global logger instance is protected by a mutex to ensure
//! serialized access.
//! 
//! # Error Handling
//! 
//! Functions that can fail return appropriate CURSED error types. Fatal functions
//! and panic functions handle errors internally before terminating the program.

use std::sync::{Arc, Mutex};
use std::io::{Write, stdout, stderr};
use crate::stdlib::value::Value;
use crate::error::CursedError;
use super::logger::Logger;
use super::flags::LstdFlags;
use crate::error::Error;

/// Global standard logger instance
/// 
/// This is initialized with stdout as the output destination, empty prefix,
/// and standard flags (LstdFlags). It can be reconfigured at runtime using
/// the configuration functions.
lazy_static::lazy_static! {
    static ref STANDARD_LOGGER: Arc<Mutex<Logger>> = Arc::new(Mutex::new(
        Logger::new(Box::new(stdout()), String::new(), LstdFlags)
    ));
}

/// Helper function to acquire logger lock with proper error handling
#[inline]
fn with_logger<F, R>(f: F) -> Result<(), Error>
where
    F: FnOnce(&Logger) -> Result<(), Error>,
{
    let logger = STANDARD_LOGGER.lock().map_err(|_| {
        CursedError::Runtime("Failed to acquire global logger lock".to_string())
    })?;
    f(&*logger)
}

/// Helper function to acquire mutable logger lock with proper error handling
#[inline]
fn with_logger_mut<F, R>(f: F) -> Result<(), Error>
where
    F: FnOnce(&mut Logger) -> Result<(), Error>,
{
    let mut logger = STANDARD_LOGGER.lock().map_err(|_| {
        CursedError::Runtime("Failed to acquire global logger lock".to_string())
    })?;
    f(&mut *logger)
}

// =============================================================================
// Output Functions
// =============================================================================

/// Print arguments followed by newline to the standard logger
/// 
/// This is equivalent to calling `spill` on the global logger instance.
/// Arguments are separated by spaces and terminated with a newline.
/// 
/// # Arguments
/// * `args` - Slice of Values to print
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(CursedError)` on I/O error or lock acquisition failure
/// 
/// # Example
/// ```
/// use cursed::stdlib::oglogging::standard::spill;
/// use cursed::value::Value;
/// 
/// let args = vec![
///     Value::String("Hello".to_string()),
///     Value::String("world".to_string()),
/// ];
/// spill(&args)?; // Prints: "Hello world\n"
/// ```
pub fn spill(args: &[Value]) -> Result<(), Error> {
    with_logger(|logger| logger.spill(args))
}

/// Print formatted string to the standard logger
/// 
/// This is equivalent to calling `spillf` on the global logger instance.
/// Uses CURSED's formatting system with {} placeholders.
/// 
/// # Arguments
/// * `format` - Format string with {} placeholders
/// * `args` - Slice of Values to substitute into format string
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(CursedError)` on I/O error, formatting error, or lock acquisition failure
/// 
/// # Example
/// ```
/// spillf("Hello, {}! You have {} messages.", &[
///     Value::String("Alice".to_string()),
///     Value::Integer(5),
/// ])?; // Prints: "Hello, Alice! You have 5 messages.\n"
/// ```
pub fn spillf(format: &str, args: &[Value]) -> Result<(), Error> {
    with_logger(|logger| logger.spillf(format, args))
}

/// Print arguments and exit the program with exit code 1
/// 
/// This function prints the arguments using the standard logger and then
/// immediately exits the program. It cannot return normally.
/// 
/// # Arguments
/// * `args` - Slice of Values to print before exiting
/// 
/// # Note
/// This function never returns - it always exits the process.
/// 
/// # Example
/// ```
/// fatal(&[Value::String("Critical error occurred".to_string())]);
/// // Program exits here with code 1
/// ```
pub fn fatal(args: &[Value]) -> ! {
    // Best effort to print the message
    if let Ok(logger) = STANDARD_LOGGER.lock() {
        let _ = logger.spill(args);
        let _ = logger.flush();
    }
    std::process::exit(1);
}

/// Print formatted string and exit the program with exit code 1
/// 
/// This function prints a formatted message using the standard logger and then
/// immediately exits the program. It cannot return normally.
/// 
/// # Arguments
/// * `format` - Format string with {} placeholders
/// * `args` - Slice of Values to substitute into format string
/// 
/// # Note
/// This function never returns - it always exits the process.
/// 
/// # Example
/// ```
/// fatalf("Fatal error: {} (code: {})", &[
///     Value::String("Database connection failed".to_string()),
///     Value::Integer(500),
/// ]);
/// // Program exits here with code 1
/// ```
pub fn fatalf(format: &str, args: &[Value]) -> ! {
    // Best effort to print the formatted message
    if let Ok(logger) = STANDARD_LOGGER.lock() {
        let _ = logger.spillf(format, args);
        let _ = logger.flush();
    }
    std::process::exit(1);
}

/// Print arguments and trigger a panic
/// 
/// This function prints the arguments using the standard logger and then
/// triggers a panic, which can potentially be caught by panic handlers.
/// 
/// # Arguments
/// * `args` - Slice of Values to print before panicking
/// 
/// # Note
/// This function never returns normally - it always panics.
/// 
/// # Example
/// ```
/// shook(&[Value::String("Assertion failed".to_string())]);
/// // Panic occurs here
/// ```
pub fn shook(args: &[Value]) -> ! {
    // Best effort to print the message
    if let Ok(logger) = STANDARD_LOGGER.lock() {
        let _ = logger.spill(args);
        let _ = logger.flush();
    }
    panic!("shook: logging panic triggered");
}

/// Print formatted string and trigger a panic
/// 
/// This function prints a formatted message using the standard logger and then
/// triggers a panic, which can potentially be caught by panic handlers.
/// 
/// # Arguments
/// * `format` - Format string with {} placeholders
/// * `args` - Slice of Values to substitute into format string
/// 
/// # Note
/// This function never returns normally - it always panics.
/// 
/// # Example
/// ```
/// shookf("Panic condition: {} (value: {})", &[
///     Value::String("Invalid state".to_string()),
///     Value::Integer(-1),
/// ]);
/// // Panic occurs here
/// ```
pub fn shookf(format: &str, args: &[Value]) -> ! {
    // Best effort to print the formatted message
    if let Ok(logger) = STANDARD_LOGGER.lock() {
        let _ = logger.spillf(format, args);
        let _ = logger.flush();
    }
    panic!("shookf: logging panic triggered");
}

// =============================================================================
// Configuration Functions
// =============================================================================

/// Set the output flags for the standard logger
/// 
/// Flags control the format of log output, including timestamps, file names,
/// line numbers, and other metadata. See the flags module for available options.
/// 
/// # Arguments
/// * `flags` - Integer containing the OR'd flag values
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(CursedError)` on lock acquisition failure
/// 
/// # Example
/// ```
/// use cursed::stdlib::oglogging::flags::{Ldate, Ltime};
/// 
/// set_flags(Ldate | Ltime)?; // Enable date and time in output
/// ```
pub fn set_flags(flags: i32) -> Result<(), Error> {
    with_logger_mut(|logger| {
        logger.set_flags(flags);
        Ok(())
    })
}

/// Set the output destination for the standard logger
/// 
/// Changes where the standard logger writes its output. The writer must
/// implement Write + Send to be compatible with concurrent access.
/// 
/// # Arguments
/// * `writer` - Box containing the new output destination
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(CursedError)` on lock acquisition failure
/// 
/// # Example
/// ```
/// use std::fs::File;
/// 
/// let file = File::create("log.txt")?;
/// set_output(Box::new(file))?; // Redirect output to file
/// ```
pub fn set_output(writer: Box<dyn Write + Send>) -> Result<(), Error> {
    with_logger_mut(|logger| {
        logger.set_output(writer);
        Ok(())
    })
}

/// Set the prefix for the standard logger
/// 
/// The prefix is prepended to each log message. This is useful for identifying
/// the source of log messages in complex applications.
/// 
/// # Arguments
/// * `prefix` - String prefix to prepend to log messages
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(CursedError)` on lock acquisition failure
/// 
/// # Example
/// ```
/// set_prefix("MyApp: ")?;
/// spill(&[Value::String("test".to_string())])?; 
/// // Output: "MyApp: test"
/// ```
pub fn set_prefix(prefix: &str) -> Result<(), Error> {
    with_logger_mut(|logger| {
        logger.set_prefix(prefix.to_string());
        Ok(())
    })
}

/// Get the current flags from the standard logger
/// 
/// Returns the current flag configuration that controls log message formatting.
/// 
/// # Returns
/// * `Ok(i32)` containing the current flags on success
/// * `Err(CursedError)` on lock acquisition failure
/// 
/// # Example
/// ```
/// let current_flags = flags()?;
/// println!("Current flags: {}", current_flags);
/// ```
pub fn flags() -> Result<(), Error> {
    with_logger(|logger| Ok(logger.flags()))
}

/// Get the current prefix from the standard logger
/// 
/// Returns the current prefix string that is prepended to log messages.
/// 
/// # Returns
/// * `Ok(String)` containing the current prefix on success
/// * `Err(CursedError)` on lock acquisition failure
/// 
/// # Example
/// ```
/// let current_prefix = prefix()?;
/// println!("Current prefix: '{}'", current_prefix);
/// ```
pub fn prefix() -> Result<(), Error> {
    with_logger(|logger| Ok(logger.prefix()))
}

/// Get information about the current writer
/// 
/// Note: Due to Rust's type system limitations, this function cannot return
/// the actual writer object. Instead, it returns a string description of the
/// writer type for diagnostic purposes.
/// 
/// # Returns
/// * `Ok(String)` containing writer information on success
/// * `Err(CursedError)` on lock acquisition failure
/// 
/// # Example
/// ```
/// let writer_info = writer()?;
/// println!("Current writer: {}", writer_info);
/// ```
pub fn writer() -> Result<(), Error> {
    with_logger(|logger| {
        // Since we can't actually return the writer due to type system limitations,
        // we return a descriptive string instead
        Ok("Global standard logger writer (type information not available)".to_string())
    })
}

/// Flush the standard logger's output
/// 
/// Forces any buffered output to be written immediately. This is useful when
/// you need to ensure log messages are visible before the program continues.
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(CursedError)` on I/O error or lock acquisition failure
/// 
/// # Example
/// ```
/// spill(&[Value::String("Important message".to_string())])?;
/// flush()?; // Ensure message is written immediately
/// ```
pub fn flush() -> Result<(), Error> {
    with_logger(|logger| logger.flush())
}

/// Reset the standard logger to default settings
/// 
/// Resets the standard logger to use stdout as output, empty prefix,
/// and standard flags. This is useful for testing or resetting state.
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(CursedError)` on lock acquisition failure
/// 
/// # Example
/// ```
/// reset_standard_logger()?; // Reset to defaults
/// ```
pub fn reset_standard_logger() -> Result<(), Error> {
    let mut logger = STANDARD_LOGGER.lock().map_err(|_| {
        CursedError::Runtime("Failed to acquire global logger lock".to_string())
    })?;
    *logger = Logger::new(Box::new(stdout()), String::new(), LstdFlags);
    Ok(())
}

/// Get a clone of the standard logger for advanced use cases
/// 
/// Returns a clone of the global standard logger, which can be used
/// independently while sharing the same configuration. This is useful
/// for advanced logging scenarios.
/// 
/// # Returns
/// * `Ok(Logger)` containing a clone of the standard logger on success
/// * `Err(CursedError)` on lock acquisition failure
/// 
/// # Example
/// ```
/// let logger_clone = get_standard_logger()?;
/// // Use logger_clone independently
/// ```
pub fn get_standard_logger() -> Result<(), Error> {
    with_logger(|logger| Ok(logger.clone()))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::value::Value;
    use std::io::Cursor;
    use super::super::flags::{Ldate, Ltime};

    #[test]
    fn test_standard_logger_basic_operations() {
        // Reset to known state
        assert!(reset_standard_logger().is_ok());
        
        // Test configuration functions
        assert!(set_flags(Ldate | Ltime).is_ok());
        assert!(set_prefix("test: ").is_ok());
        
        let current_flags = flags().unwrap();
        assert_eq!(current_flags, Ldate | Ltime);
        
        let current_prefix = prefix().unwrap();
        assert_eq!(current_prefix, "test: ");
    }
    
    #[test]
    fn test_spill_functionality() {
        assert!(reset_standard_logger().is_ok());
        
        let args = vec![
            Value::String("test".to_string()),
            Value::String("message".to_string())
        ];
        assert!(spill(&args).is_ok());
    }
    
    #[test]
    fn test_spillf_functionality() {
        assert!(reset_standard_logger().is_ok());
        
        let args = vec![
            Value::String("world".to_string()),
            Value::Integer(42)
        ];
        assert!(spillf("Hello, {}! Number: {}", &args).is_ok());
    }
    
    #[test]
    fn test_output_redirection() {
        assert!(reset_standard_logger().is_ok());
        
        // Redirect to in-memory buffer for testing
        let buffer = Cursor::new(Vec::new());
        assert!(set_output(Box::new(buffer)).is_ok());
        
        // Test that we can still log (though we can't easily verify the output
        // due to the buffer being moved into the logger)
        let args = vec![Value::String("test output".to_string())];
        assert!(spill(&args).is_ok());
        
        // Reset for other tests
        assert!(reset_standard_logger().is_ok());
    }
    
    #[test]
    fn test_flush_operation() {
        assert!(reset_standard_logger().is_ok());
        assert!(flush().is_ok());
    }
    
    #[test]
    fn test_writer_info() {
        assert!(reset_standard_logger().is_ok());
        
        let writer_info = writer().unwrap();
        assert!(writer_info.contains("writer"));
    }
    
    #[test]
    fn test_get_standard_logger_clone() {
        assert!(reset_standard_logger().is_ok());
        
        let logger_clone = get_standard_logger();
        assert!(logger_clone.is_ok());
    }
    
    #[test]
    fn test_concurrent_access() {
        use std::thread;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        
        assert!(reset_standard_logger().is_ok());
        
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        // Spawn multiple threads that use the standard logger
        for i in 0..4 {
            let counter_clone = counter.clone();
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let args = vec![
                        Value::String(format!("Thread {}", i)),
                        Value::Integer(j as i64)
                    ];
                    if spill(&args).is_ok() {
                        counter_clone.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify that all operations succeeded
        assert_eq!(counter.load(Ordering::Relaxed), 40);
    }
    
    #[test]
    fn test_error_handling() {
        assert!(reset_standard_logger().is_ok());
        
        // Test with empty args (should succeed)
        assert!(spill(&[]).is_ok());
        
        // Test with various value types
        let mixed_args = vec![
            Value::String("string".to_string()),
            Value::Integer(42),
            Value::Float(3.14),
            Value::Boolean(true)
        ];
        assert!(spill(&mixed_args).is_ok());
        assert!(spillf("Mixed: {} {} {} {}", &mixed_args).is_ok());
    }
    
    #[test]
    #[should_panic(expected = "shook")]
    fn test_shook_panics() {
        let args = vec![Value::String("test panic".to_string())];
        shook(&args);
    }
    
    #[test]
    #[should_panic(expected = "shookf")]
    fn test_shookf_panics() {
        let args = vec![Value::String("test".to_string())];
        shookf("Formatted panic: {}", &args);
    }
}
