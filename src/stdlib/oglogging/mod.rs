pub mod logger;
pub mod flags;
pub mod formatter;
pub mod standard;

use std::sync::{Arc, Mutex};
use std::io::Write;
use crate::value::Value;
use crate::error::CursedError;

// Re-export main types and functions
pub use logger::{Logger, new_logger};
pub use flags::*;
pub use formatter::format_log_entry;
pub use standard::*;

// Global standard logger instance
lazy_static::lazy_static! {
    static ref STANDARD_LOGGER: Arc<Mutex<Logger>> = Arc::new(Mutex::new(
        Logger::new(Box::new(std::io::stdout()), String::new(), LstdFlags)
    ));
}

/// Standard logger functions that operate on the global logger instance

/// spill - Print args followed by newline
pub fn spill(args: &[Value]) -> Result<(), CursedError> {
    let logger = STANDARD_LOGGER.lock().map_err(|_| {
        CursedError::Runtime("Failed to acquire logger lock".to_string())
    })?;
    logger.spill(args)
}

/// spillf - Print formatted string
pub fn spillf(format: &str, args: &[Value]) -> Result<(), CursedError> {
    let logger = STANDARD_LOGGER.lock().map_err(|_| {
        CursedError::Runtime("Failed to acquire logger lock".to_string())
    })?;
    logger.spillf(format, args)
}

/// fatal - Print args and exit with code 1
pub fn fatal(args: &[Value]) -> ! {
    if let Ok(logger) = STANDARD_LOGGER.lock() {
        let _ = logger.spill(args);
    }
    std::process::exit(1);
}

/// fatalf - Print formatted string and exit with code 1
pub fn fatalf(format: &str, args: &[Value]) -> ! {
    if let Ok(logger) = STANDARD_LOGGER.lock() {
        let _ = logger.spillf(format, args);
    }
    std::process::exit(1);
}

/// shook - Print args and trigger panic
pub fn shook(args: &[Value]) -> ! {
    if let Ok(logger) = STANDARD_LOGGER.lock() {
        let _ = logger.spill(args);
    }
    panic!("shook triggered");
}

/// shookf - Print formatted string and trigger panic
pub fn shookf(format: &str, args: &[Value]) -> ! {
    if let Ok(logger) = STANDARD_LOGGER.lock() {
        let _ = logger.spillf(format, args);
    }
    panic!("shookf triggered");
}

/// setFlags - Set output flags for standard logger
pub fn set_flags(flag: i32) -> Result<(), CursedError> {
    let mut logger = STANDARD_LOGGER.lock().map_err(|_| {
        CursedError::Runtime("Failed to acquire logger lock".to_string())
    })?;
    logger.set_flags(flag);
    Ok(())
}

/// setOutput - Set output destination for standard logger
pub fn set_output(writer: Box<dyn Write + Send>) -> Result<(), CursedError> {
    let mut logger = STANDARD_LOGGER.lock().map_err(|_| {
        CursedError::Runtime("Failed to acquire logger lock".to_string())
    })?;
    logger.set_output(writer);
    Ok(())
}

/// setPrefix - Set output prefix for standard logger
pub fn set_prefix(prefix: &str) -> Result<(), CursedError> {
    let mut logger = STANDARD_LOGGER.lock().map_err(|_| {
        CursedError::Runtime("Failed to acquire logger lock".to_string())
    })?;
    logger.set_prefix(prefix.to_string());
    Ok(())
}

/// Get current flags from standard logger
pub fn flags() -> Result<i32, CursedError> {
    let logger = STANDARD_LOGGER.lock().map_err(|_| {
        CursedError::Runtime("Failed to acquire logger lock".to_string())
    })?;
    Ok(logger.flags())
}

/// Get current prefix from standard logger
pub fn prefix() -> Result<String, CursedError> {
    let logger = STANDARD_LOGGER.lock().map_err(|_| {
        CursedError::Runtime("Failed to acquire logger lock".to_string())
    })?;
    Ok(logger.prefix())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_standard_logger_operations() {
        // Test basic functionality
        assert!(set_flags(Ldate | Ltime).is_ok());
        assert!(set_prefix("test: ").is_ok());
        
        let current_flags = flags().unwrap();
        assert_eq!(current_flags, Ldate | Ltime);
        
        let current_prefix = prefix().unwrap();
        assert_eq!(current_prefix, "test: ");
    }
    
    #[test]
    fn test_spill_functionality() {
        let args = vec![Value::String("test message".to_string())];
        assert!(spill(&args).is_ok());
    }
    
    #[test]
    fn test_spillf_functionality() {
        let args = vec![Value::String("world".to_string())];
        assert!(spillf("Hello, {}!", &args).is_ok());
    }
}
