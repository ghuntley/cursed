//! Standard library error handling utilities for CURSED
//!
//! This module provides common error constructors, formatting functions,
//! and integration utilities for the CURSED error handling system.

use crate::error::{Error, CursedError, SourceLocation};
use crate::stdlib::value::Value;
use crate::stdlib::io::error::IoError as StdlibIoError;

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::Error;
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

pub struct ErrorManager {
    errors: Vec<crate::error::Error>,
    severity: ErrorSeverity,
}

impl ErrorManager {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            severity: ErrorSeverity::Low,
        }
    }
}


/// Initialize the global error system (minimal implementation)
pub fn init_error_system() -> Result<(), Error> {
    // Basic error system initialization
    tracing::debug!("Initialized CURSED error system");
    Ok(())
}

/// Common error result type for CURSED standard library
pub type CursedResult<T> = std::result::Result<T, CursedError>;

/// Common option type for CURSED standard library  
pub type CursedOption<T> = std::option::Option<T>;

/// Error formatting utilities
pub struct ErrorFormatter {
    pub use_colors: bool,
    pub show_stack_trace: bool,
    pub show_error_code: bool,
    pub max_width: usize,
}

impl Default for ErrorFormatter {
    fn default() -> Self {
        Self {
            use_colors: true,
            show_stack_trace: true,
            show_error_code: true,
            max_width: 80,
        }
    }
}

impl ErrorFormatter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_colors(mut self, use_colors: bool) -> Self {
        self.use_colors = use_colors;
        self
    }

    pub fn with_stack_trace(mut self, show_stack_trace: bool) -> Self {
        self.show_stack_trace = show_stack_trace;
        self
    }

    pub fn with_error_code(mut self, show_error_code: bool) -> Self {
        self.show_error_code = show_error_code;
        self
    }

    pub fn with_max_width(mut self, max_width: usize) -> Self {
        self.max_width = max_width;
        self
    }

    /// Format a CursedError for display
    pub fn format_error(&self, error: &CursedError) -> String {
        let mut output = String::new();

        // Add category and severity prefix
        if let std::option::Option::Some(error_trait) = self.error_to_trait(error) {
            if self.use_colors {
                output.push_str(error_trait.category().color_code());
                output.push_str(error_trait.severity().color_code());
            }

            output.push_str("[");
            output.push_str(error_trait.category().name());
            output.push_str(" ");
            output.push_str(error_trait.severity().name());
            output.push_str("]");

            if self.use_colors {
                output.push_str(ErrorCategory::RESET);
            }

            output.push_str(": ");

            // Add error code if enabled
            if self.show_error_code {
                output.push_str("[");
                output.push_str(error_trait.error_code());
                output.push_str("] ");
            }
        }

        // Add main error message
        output.push_str(&error.to_string());

        // Add source location if available
        if let Some(location) = error.get_source_location() {
            output.push_str(" at ");
            output.push_str(&location.to_string());
        }

        // Wrap text if needed
        if output.len() > self.max_width {
            output = self.wrap_text(&output, self.max_width);
        }

        output
    }

    /// Format multiple errors as a list
    pub fn format_error_list(&self, errors: &[CursedError]) -> String {
        let mut output = String::new();
        
        for (i, error) in errors.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, self.format_error(error)));
        }

        output
    }

    /// Format error with context information
    pub fn format_error_with_context(&self, error: &CursedError, context: &str) -> String {
        let mut output = self.format_error(error);
        output.push_str("\n  Context: ");
        output.push_str(context);
        output
    }

    /// Format error with suggestions
    pub fn format_error_with_suggestion(&self, error: &CursedError, suggestion: &str) -> String {
        let mut output = self.format_error(error);
        output.push_str("\n  Suggestion: ");
        output.push_str(suggestion);
        output
    }

    /// Convert CursedError to trait object for enhanced formatting
    fn error_to_trait(&self, error: &CursedError) -> std::option::Option<Box<dyn CursedErrorTrait>> {
        match error {
            CursedError::Io(io_err) => {
                std::option::Option::Some(Box::new(IoError::new(
                    "IO_ERROR".to_string(),
                    io_err.to_string(),
                    io_err.kind(),
                )))
            }
            CursedError::Parse(msg) => {
                std::option::Option::Some(Box::new(ParseError::new("PARSE_ERROR".to_string(), msg.clone())))
            }
            CursedError::Runtime(msg) => {
                std::option::Option::Some(Box::new(RuntimeError::new("RUNTIME_ERROR".to_string(), msg.clone())))
            }
            CursedError::ParseError { message, line, column } => {
                let mut error = ParseError::new("PARSE_ERROR".to_string(), message.clone());
                if let (std::option::Option::Some(line), std::option::Option::Some(column)) = (line, column) {
                    error = error.with_location(SourceLocation::new(*line, *column));
                }
                std::option::Option::Some(Box::new(error))
            }
            _ => std::option::Option::None,
        }
    }

    /// Wrap text to specified width
    fn wrap_text(&self, text: &str, width: usize) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 > width && !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
            }
            
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines.join("\n")
    }
}

/// Standard error constructors for common scenarios
pub mod std_errors {
    use super::*;

    /// File system errors
    pub fn file_not_found(path: &str) -> CursedResult<()> {
        error_patterns::io_error(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", path)
        ))
    }

    pub fn permission_denied(path: &str) -> CursedResult<()> {
        error_patterns::io_error(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("Permission denied: {}", path)
        ))
    }

    pub fn directory_not_found(path: &str) -> CursedResult<()> {
        error_patterns::io_error(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory not found: {}", path)
        ))
    }

    /// Parsing errors
    pub fn syntax_error(message: &str, line: usize, column: usize) -> CursedResult<()> {
        error_patterns::parse_error(message, line, column)
    }

    pub fn unexpected_token(expected: &str, found: &str, line: usize, column: usize) -> CursedResult<()> {
        error_patterns::parse_error(
            &format!("Expected {}, found {}", expected, found),
            line,
            column
        )
    }

    pub fn unclosed_delimiter(delimiter: &str, line: usize, column: usize) -> CursedResult<()> {
        error_patterns::parse_error(
            &format!("Unclosed delimiter: {}", delimiter),
            line,
            column
        )
    }

    /// Runtime errors
    pub fn division_by_zero(line: usize, column: usize) -> CursedResult<()> {
        Err(CursedError::parse_error_with_location(
            "Division by zero".to_string(),
            line,
            column
        ))
    }

    pub fn null_pointer_access(line: usize, column: usize) -> CursedResult<()> {
        Err(CursedError::parse_error_with_location(
            "Null pointer access".to_string(),
            line,
            column
        ))
    }

    pub fn index_out_of_bounds(index: usize, length: usize, line: usize, column: usize) -> CursedResult<()> {
        Err(CursedError::parse_error_with_location(
            format!("Index {} out of bounds for length {}", index, length),
            line,
            column
        ))
    }

    pub fn type_mismatch(expected: &str, actual: &str, line: usize, column: usize) -> CursedResult<()> {
        error_patterns::type_error(&format!("Type mismatch: expected {}, got {}", expected, actual))
    }

    /// Memory errors
    pub fn out_of_memory(requested: usize) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Out of memory: requested {} bytes", requested))
    }

    pub fn memory_corruption(address: usize) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Memory corruption detected at address 0x{:x}", address))
    }

    /// Concurrency errors
    pub fn deadlock_detected(goroutine_id: u64) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Deadlock detected in goroutine {}", goroutine_id))
    }

    pub fn channel_closed() -> CursedResult<()> {
        error_patterns::runtime_error("Channel is closed")
    }

    /// Network errors
    pub fn connection_failed(address: &str) -> CursedResult<()> {
        error_patterns::io_error(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            format!("Connection failed to {}", address)
        ))
    }

    pub fn timeout_error(operation: &str, duration_ms: u64) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("{} timed out after {}ms", operation, duration_ms))
    }

    /// Database errors
    pub fn database_connection_failed(url: &str) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Database connection failed: {}", url))
    }

    pub fn query_execution_failed(query: &str) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Query execution failed: {}", query))
    }

    /// Configuration errors
    pub fn invalid_configuration(key: &str, value: &str) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Invalid configuration: {}={}", key, value))
    }

    pub fn missing_configuration(key: &str) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Missing required configuration: {}", key))
    }
}

/// Error reporting and logging integration
pub struct ErrorReporter {
    pub manager: Arc<ErrorManager>,
    pub formatter: ErrorFormatter,
    pub log_level: ErrorSeverity,
    pub report_to_stdout: bool,
    pub report_to_stderr: bool,
}

impl ErrorReporter {
    pub fn new() -> Result<(), Error> {
        Ok(Self {
            manager: get_error_manager()?,
            formatter: ErrorFormatter::new(),
            log_level: ErrorSeverity::Warning,
            report_to_stdout: false,
            report_to_stderr: true,
        })
    }

    pub fn with_formatter(mut self, formatter: ErrorFormatter) -> Self {
        self.formatter = formatter;
        self
    }

    pub fn with_log_level(mut self, level: ErrorSeverity) -> Self {
        self.log_level = level;
        self
    }

    pub fn with_stdout(mut self, enabled: bool) -> Self {
        self.report_to_stdout = enabled;
        self
    }

    pub fn with_stderr(mut self, enabled: bool) -> Self {
        self.report_to_stderr = enabled;
        self
    }

    /// Report an error
    pub fn report_error(&self, error: &CursedError) -> Result<(), Error> {
        // Add to error manager
        if let Some(error_trait) = self.convert_to_trait(error) {
            self.manager.add_error(error_trait)?;
        }

        // Format for display
        let formatted = self.formatter.format_error(error);

        // Output to specified streams
        if self.report_to_stdout {
            println!("{}", formatted);
        }

        if self.report_to_stderr {
            eprintln!("{}", formatted);
        }

        Ok(())
    }

    /// Report multiple errors
    pub fn report_errors(&self, errors: &[CursedError]) -> Result<(), Error> {
        for error in errors {
            self.report_error(error)?;
        }
        Ok(())
    }

    /// Get error statistics
    pub fn get_statistics(&self) -> Result<(), Error> {
        let stats = self.manager.get_statistics()?;
        Ok(format!("{}", stats))
    }

    /// Clear all errors
    pub fn clear_errors(&self) -> Result<(), Error> {
        self.manager.clear_errors()
    }

    fn convert_to_trait(&self, error: &CursedError) -> Option<Box<dyn CursedErrorTrait>> {
        match error {
            CursedError::Io(io_err) => {
                Some(Box::new(IoError::new(
                    "IO_ERROR".to_string(),
                    io_err.to_string(),
                    io_err.kind(),
                )))
            }
            CursedError::Parse(msg) => {
                Some(Box::new(ParseError::new("PARSE_ERROR".to_string(), msg.clone())))
            }
            CursedError::Runtime(msg) => {
                Some(Box::new(RuntimeError::new("RUNTIME_ERROR".to_string(), msg.clone())))
            }
            _ => None,
        }
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback implementation if error manager not initialized
            Self {
                manager: Arc::new(ErrorManager::new()),
                formatter: ErrorFormatter::new(),
                log_level: ErrorSeverity::Warning,
                report_to_stdout: false,
                report_to_stderr: true,
            }
        })
    }
}

/// Integration with I/O operations
pub mod io_integration {
    use super::*;

    /// Wrap std::io::Result to CursedResult
    pub fn wrap_io_result<T>(result: std::io::Result<T>) -> CursedResult<T> {
        result.map_err(|io_err| CursedError::Io(io_err))
    }

    /// Convert stdlib IoError to CursedError
    pub fn convert_stdlib_io_error(error: StdlibIoError) -> CursedError {
        match error {
            StdlibIoError::UnexpectedEof => CursedError::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Unexpected end of file"
            )),
            StdlibIoError::InvalidUtf8 => CursedError::Runtime("Invalid UTF-8 sequence".to_string()),
            StdlibIoError::Interrupted => CursedError::Io(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "Operation interrupted"
            )),
            StdlibIoError::PermissionDenied => CursedError::Io(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Permission denied"
            )),
            StdlibIoError::InvalidInput => CursedError::Runtime("Invalid input".to_string()),
            StdlibIoError::BufferOverflow => CursedError::Runtime("Buffer overflow".to_string()),
            StdlibIoError::StreamClosed => CursedError::Runtime("Stream closed".to_string()),
            StdlibIoError::Timeout => CursedError::Io(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Operation timed out"
            )),
            StdlibIoError::General(msg) => CursedError::Runtime(msg),
            StdlibIoError::System(code) => CursedError::Runtime(format!("System error: {}", code)),
        }
    }

    /// Safe file reading with error handling
    pub fn read_file_safe(path: &str) -> CursedResult<String> {
        std::fs::read_to_string(path)
            .map_err(|io_err| CursedError::Io(io_err))
    }

    /// Safe file writing with error handling
    pub fn write_file_safe(path: &str, content: &str) -> CursedResult<()> {
        std::fs::write(path, content)
            .map_err(|io_err| CursedError::Io(io_err))
    }

    /// Safe directory creation with error handling
    pub fn create_dir_safe(path: &str) -> CursedResult<()> {
        std::fs::create_dir_all(path)
            .map_err(|io_err| CursedError::Io(io_err))
    }

    /// Check if file exists safely
    pub fn file_exists_safe(path: &str) -> bool {
        std::fs::metadata(path).is_ok()
    }
}

/// Error recovery utilities
pub mod recovery {
    use super::*;

    /// Retry an operation with exponential backoff
    pub fn retry_with_backoff<T, F>(
        mut operation: F,
        max_attempts: usize,
        base_delay_ms: u64,
    ) -> CursedResult<T>
    where
        F: FnMut() -> CursedResult<T>,
    {
        let mut attempts = 0;
        let mut delay = base_delay_ms;

        loop {
            attempts += 1;
            
            match operation() {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if attempts >= max_attempts {
                        return Err(error);
                    }
                    
                    // Exponential backoff
                    std::thread::sleep(std::time::Duration::from_millis(delay));
                    delay *= 2;
                }
            }
        }
    }

    /// Try an operation and return Option instead of Result
    pub fn try_or_none<T, F>(operation: F) -> CursedOption<T>
    where
        F: FnOnce() -> CursedResult<T>,
    {
        match operation() {
            Ok(value) => Option::Some(value),
            Err(_) => Option::None,
        }
    }

    /// Try an operation and return default on error
    pub fn try_or_default<T, F>(operation: F, default: T) -> T
    where
        F: FnOnce() -> CursedResult<T>,
    {
        match operation() {
            Ok(value) => value,
            Err(_) => default,
        }
    }

    /// Try multiple operations and return first success
    pub fn try_alternatives<T>(operations: Vec<Box<dyn FnOnce() -> CursedResult<T>>>) -> CursedResult<T> {
        let mut last_error = error_patterns::runtime_error("No operations provided");

        for operation in operations {
            match operation() {
                Ok(result) => return Ok(result),
                Err(error) => last_error = Err(error),
            }
        }

        last_error
    }
}

/// Function registry for error-aware CURSED functions
pub struct ErrorAwareFunctionRegistry {
    pub functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> CursedResult<Value> + Send + Sync>>,
}

impl ErrorAwareFunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, name: &str, function: F)
    where
        F: Fn(Vec<Value>) -> CursedResult<Value> + Send + Sync + 'static,
    {
        self.functions.insert(name.to_string(), Box::new(function));
    }

    pub fn call(&self, name: &str, args: Vec<Value>) -> CursedResult<Value> {
        let function = self.functions.get(name)
            .ok_or_else(|| CursedError::Runtime(format!("Function not found: {}", name)))?;
        
        function(args)
    }

    pub fn list_functions(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
}

impl Default for ErrorAwareFunctionRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        
        // Register common error-aware functions
        registry.register("safe_divide", |args| {
            if args.len() != 2 {
                return error_patterns::runtime_error("safe_divide requires 2 arguments");
            }

            let a = match &args[0] {
                Value::Number(n) => *n,
                _ => return error_patterns::type_error("First argument must be a number"),
            };

            let b = match &args[1] {
                Value::Number(n) => *n,
                _ => return error_patterns::type_error("Second argument must be a number"),
            };

            if b == 0.0 {
                return error_patterns::runtime_error("Division by zero");
            }

            Ok(Value::Number(a / b))
        });

        registry.register("safe_index", |args| {
            if args.len() != 2 {
                return error_patterns::runtime_error("safe_index requires 2 arguments");
            }

            let array = match &args[0] {
                Value::Array(arr) => arr,
                _ => return error_patterns::type_error("First argument must be an array"),
            };

            let index = match &args[1] {
                Value::Number(n) => *n as usize,
                _ => return error_patterns::type_error("Second argument must be a number"),
            };

            if index >= array.len() {
                return error_patterns::runtime_error(&format!(
                    "Index {} out of bounds for array of length {}",
                    index,
                    array.len()
                ));
            }

            Ok(array[index].clone())
        });

        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_formatter() {
        let formatter = ErrorFormatter::new();
        let error = CursedError::Runtime("Test error".to_string());
        let formatted = formatter.format_error(&error);
        
        assert!(formatted.contains("Test error"));
    }

    #[test]
    fn test_std_errors() {
        let result = std_errors::file_not_found("/nonexistent/file.txt");
        assert!(result.is_err());

        let result = std_errors::division_by_zero(10, 5);
        assert!(result.is_err());

        let result = std_errors::type_mismatch("String", "Number", 15, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_reporter() {
        let reporter = ErrorReporter::default();
        let error = CursedError::Runtime("Test error".to_string());
        
        // This should not fail even if error manager is not initialized
        let result = reporter.report_error(&error);
        // We can't assert much here since we're using default reporter
    }

    #[test]
    fn test_io_integration() {
        let result = io_integration::wrap_io_result::<String>(Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Test error"
        )));
        
        assert!(result.is_err());
        match result {
            Err(CursedError::Io(_)) => {}, // Expected
            _ => panic!("Expected I/O error"),
        }
    }

    #[test]
    fn test_recovery_utilities() {
        let mut attempt_count = 0;
        let result = recovery::retry_with_backoff(
            || {
                attempt_count += 1;
                if attempt_count < 3 {
                    error_patterns::runtime_error("Test error")
                } else {
                    Ok(42)
                }
            },
            5,
            1
        );

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempt_count, 3);

        let option_result = recovery::try_or_none(|| error_patterns::runtime_error::<i32>("Test error"));
        assert!(option_result.is_none());

        let default_result = recovery::try_or_default(|| error_patterns::runtime_error("Test error"), "default");
        assert_eq!(default_result, "default");
    }

    #[test]
    fn test_error_aware_function_registry() {
        let registry = ErrorAwareFunctionRegistry::default();
        
        // Test safe_divide function
        let result = registry.call("safe_divide", vec![Value::Number(10.0), Value::Number(2.0)]);
        assert_eq!(result.unwrap(), Value::Number(5.0));

        let result = registry.call("safe_divide", vec![Value::Number(10.0), Value::Number(0.0)]);
        assert!(result.is_err());

        // Test safe_index function
        let array = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        let result = registry.call("safe_index", vec![Value::Array(array.clone()), Value::Number(1.0)]);
        assert_eq!(result.unwrap(), Value::Number(2.0));

        let result = registry.call("safe_index", vec![Value::Array(array), Value::Number(10.0)]);
        assert!(result.is_err());
    }

    #[test]
    fn test_text_wrapping() {
        let formatter = ErrorFormatter::new().with_max_width(20);
        let long_text = "This is a very long error message that should be wrapped";
        let wrapped = formatter.wrap_text(long_text, 20);
        
        let lines: Vec<&str> = wrapped.split('\n').collect();
        assert!(lines.len() > 1);
        assert!(lines.iter().all(|line| line.len() <= 20));
    }
}
