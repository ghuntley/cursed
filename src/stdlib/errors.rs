// Standard library error handling utilities for CURSED
//
// This module provides common error constructors, formatting functions,
// and integration utilities for the CURSED error handling system.

use crate::error::{CursedError, CursedError, SourceLocation};
// use crate::stdlib::value::Value;
use crate::types::result::error_patterns;

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
pub enum ErrorSeverity {
pub struct ErrorManager {
impl ErrorManager {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn add_error(&self, _error: Box<dyn CursedErrorTrait>) -> crate::error::crate::error::Result<()> {
        // Simplified implementation
        Ok(())
    pub fn get_statistics(&self) -> Result<String> {
        Ok(format!("Errors: {}", self.errors.len()))
    pub fn clear_errors(&self) -> crate::error::crate::error::Result<()> {
        // Simplified implementation
        Ok(())
    }
}

/// Initialize the global error system (minimal implementation)
pub fn init_error_system() -> crate::error::Result<()> {
    // Basic error system initialization
    tracing::debug!("Initialized CURSED error system");
    Ok(())
/// Common error result type for CURSED standard library
pub type CursedResult<T> = std::result::Result<T>;

/// Common option type for CURSED standard library  
pub type CursedOption<T> = std::option::Option<T>;

/// CursedError formatting utilities
pub struct ErrorFormatter {
impl Default for ErrorFormatter {
    fn default() -> Self {
        Self {
        }
    }
impl ErrorFormatter {
    pub fn new() -> Self {
        Self::default()
    pub fn with_colors(mut self, use_colors: bool) -> Self {
        self.use_colors = use_colors;
        self
    pub fn with_stack_trace(mut self, show_stack_trace: bool) -> Self {
        self.show_stack_trace = show_stack_trace;
        self
    pub fn with_error_code(mut self, show_error_code: bool) -> Self {
        self.show_error_code = show_error_code;
        self
    pub fn with_max_width(mut self, max_width: usize) -> Self {
        self.max_width = max_width;
        self
    /// Format a CursedError for display
    pub fn format_error(&self, error: &CursedError) -> String {
        let mut output = String::new();

        // Add category and severity prefix
        if let std::option::Option::Some(error_trait) = self.error_to_trait(error) {
            if self.use_colors {
                output.push_str(error_trait.category().color_code());
                output.push_str(error_trait.severity().color_code());
            output.push_str("[");
            output.push_str(error_trait.category().name());
            output.push_str(" ");
            output.push_str(error_trait.severity().name());
            output.push_str("]");

            if self.use_colors {
                output.push_str(ErrorCategory::RESET);
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
        // Wrap text if needed
        if output.len() > self.max_width {
            output = self.wrap_text(&output, self.max_width);
        output
    /// Format multiple errors as a list
    pub fn format_error_list(&self, errors: &[CursedError]) -> String {
        let mut output = String::new();
        
        for (i, error) in errors.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, self.format_error(error)));
        output
    /// Format error with context information
    pub fn format_error_with_context(&self, error: &CursedError, context: &str) -> String {
        let mut output = self.format_error(error);
        output.push_str("\n  Context: ");
        output.push_str(context);
        output
    /// Format error with suggestions
    pub fn format_error_with_suggestion(&self, error: &CursedError, suggestion: &str) -> String {
        let mut output = self.format_error(error);
        output.push_str("\n  Suggestion: ");
        output.push_str(suggestion);
        output
    /// Convert CursedError to trait object for enhanced formatting
    fn error_to_trait(&self, error: &CursedError) -> std::option::Option<Box<dyn CursedErrorTrait>> {
        match error {
            CursedError::Io(io_err) => {
                std::option::Option::Some(Box::new(IoError::new(
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
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        if !current_line.is_empty() {
            lines.push(current_line);
        lines.join("\n")
    }
}

/// Standard error constructors for common scenarios
pub mod std_errors {
    use super::*;

    /// File system errors
    pub fn file_not_found(path: &str) -> CursedResult<()> {
        error_patterns::io_error(std::io::Error::new(
            format!("File not found: {}", path)
        ))
    pub fn permission_denied(path: &str) -> CursedResult<()> {
        error_patterns::io_error(std::io::Error::new(
            format!("Permission denied: {}", path)
        ))
    pub fn directory_not_found(path: &str) -> CursedResult<()> {
        error_patterns::io_error(std::io::Error::new(
            format!("Directory not found: {}", path)
        ))
    /// Parsing errors
    pub fn syntax_error(message: &str, line: usize, column: usize) -> CursedResult<()> {
        error_patterns::parse_error(message, line, column)
    pub fn unexpected_token(expected: &str, found: &str, line: usize, column: usize) -> CursedResult<()> {
        error_patterns::parse_error(
            column
        )
    pub fn unclosed_delimiter(delimiter: &str, line: usize, column: usize) -> CursedResult<()> {
        error_patterns::parse_error(
            column
        )
    /// Runtime errors
    pub fn division_by_zero(line: usize, column: usize) -> CursedResult<()> {
        Err(CursedError::parse_error_with_location(
            column
        ))
    pub fn null_pointer_access(line: usize, column: usize) -> CursedResult<()> {
        Err(CursedError::parse_error_with_location(
            column
        ))
    pub fn index_out_of_bounds(index: usize, length: usize, line: usize, column: usize) -> CursedResult<()> {
        Err(CursedError::parse_error_with_location(
            column
        ))
    pub fn type_mismatch(expected: &str, actual: &str, line: usize, column: usize) -> CursedResult<()> {
        error_patterns::type_error(&format!("Type mismatch: expected {}, got {}", expected, actual))
    /// Memory errors
    pub fn out_of_memory(requested: usize) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Out of memory: requested {} bytes", requested))
    pub fn memory_corruption(address: usize) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Memory corruption detected at address 0x{:x}", address))
    /// Concurrency errors
    pub fn deadlock_detected(goroutine_id: u64) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Deadlock detected in goroutine {}", goroutine_id))
    pub fn channel_closed() -> CursedResult<()> {
        error_patterns::runtime_error("Channel is closed")
    /// Network errors
    pub fn connection_failed(address: &str) -> CursedResult<()> {
        error_patterns::io_error(std::io::Error::new(
            format!("Connection failed to {}", address)
        ))
    pub fn timeout_error(operation: &str, duration_ms: u64) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("{} timed out after {}ms", operation, duration_ms))
    /// Database errors
    pub fn database_connection_failed(url: &str) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Database connection failed: {}", url))
    pub fn query_execution_failed(query: &str) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Query execution failed: {}", query))
    /// Configuration errors
    pub fn invalid_configuration(key: &str, value: &str) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Invalid configuration: {}={}", key, value))
    pub fn missing_configuration(key: &str) -> CursedResult<()> {
        error_patterns::runtime_error(&format!("Missing required configuration: {}", key))
    }
}

/// CursedError reporting and logging integration
pub struct ErrorReporter {
impl ErrorReporter {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn with_formatter(mut self, formatter: ErrorFormatter) -> Self {
        self.formatter = formatter;
        self
    pub fn with_log_level(mut self, level: ErrorSeverity) -> Self {
        self.log_level = level;
        self
    pub fn with_stdout(mut self, enabled: bool) -> Self {
        self.report_to_stdout = enabled;
        self
    pub fn with_stderr(mut self, enabled: bool) -> Self {
        self.report_to_stderr = enabled;
        self
    /// Report an error
    pub fn report_error(&self, error: &CursedError) -> crate::error::Result<()> {
        // Add to error manager
        if let Some(error_trait) = self.convert_to_trait(error) {
            self.manager.add_error(error_trait)?;
        // Format for display
        let formatted = self.formatter.format_error(error);

        // Output to specified streams
        if self.report_to_stdout {
            println!("{}", formatted);
        if self.report_to_stderr {
            eprintln!("{}", formatted);
        Ok(())
    /// Report multiple errors
    pub fn report_errors(&self, errors: &[CursedError]) -> crate::error::crate::error::Result<()> {
        for error in errors {
            self.report_error(error)?;
        }
        Ok(())
    /// Get error statistics
    pub fn get_statistics(&self) -> Result<String> {
        let stats = self.manager.get_statistics()?;
        Ok(stats)
    /// Clear all errors
    pub fn clear_errors(&self) -> crate::error::Result<()> {
        self.manager.clear_errors()
    fn convert_to_trait(&self, error: &CursedError) -> Option<Box<dyn CursedErrorTrait>> {
        match error {
            CursedError::Io(io_err) => {
                Some(Box::new(IoError::new(
                )))
            }
            CursedError::Parse(msg) => {
                Some(Box::new(ParseError::new("PARSE_ERROR".to_string(), msg.clone())))
            }
            CursedError::Runtime(msg) => {
                Some(Box::new(RuntimeError::new("RUNTIME_ERROR".to_string(), msg.clone())))
            }
        }
    }
impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback implementation if error manager not initialized
            Self {
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
    /// Convert stdlib IoError to CursedError
    pub fn convert_stdlib_io_error(error: StdlibIoError) -> CursedError {
        match error {
            StdlibIoError::UnexpectedEof => CursedError::Io(std::io::Error::new(
                "Unexpected end of file"
            StdlibIoError::Interrupted => CursedError::Io(std::io::Error::new(
                "Operation interrupted"
            StdlibIoError::PermissionDenied => CursedError::Io(std::io::Error::new(
                "Permission denied"
            StdlibIoError::Timeout => CursedError::Io(std::io::Error::new(
                "Operation timed out"
        }
    }

    /// Safe file reading with error handling
    pub fn read_file_safe(path: &str) -> CursedResult<String> {
        std::fs::read_to_string(path)
            .map_err(|io_err| CursedError::Io(io_err))
    /// Safe file writing with error handling
    pub fn write_file_safe(path: &str, content: &str) -> CursedResult<()> {
        std::fs::write(path, content)
            .map_err(|io_err| CursedError::Io(io_err))
    /// Safe directory creation with error handling
    pub fn create_dir_safe(path: &str) -> CursedResult<()> {
        std::fs::create_dir_all(path)
            .map_err(|io_err| CursedError::Io(io_err))
    /// Check if file exists safely
    pub fn file_exists_safe(path: &str) -> bool {
        std::fs::metadata(path).is_ok()
    }
}

/// CursedError recovery utilities
pub mod recovery {
    use super::*;

    /// Retry an operation with exponential backoff
    pub fn retry_with_backoff<T, F>(
    ) -> CursedResult<T>
    where
    {
        let mut attempts = 0;
        let mut delay = base_delay_ms;

        loop {
            attempts += 1;
            
            match operation() {
                Err(error) => {
                    if attempts >= max_attempts {
                        return Err(error);
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
    {
        match operation() {
        }
    }

    /// Try an operation and return default on error
    pub fn try_or_default<T, F>(operation: F, default: T) -> T
    where
    {
        match operation() {
        }
    }

    /// Try multiple operations and return first success
    pub fn try_alternatives<T>(operations: Vec<Box<dyn FnOnce() -> CursedResult<T>>>) -> CursedResult<T> {
        let mut last_error = error_patterns::runtime_error("No operations provided");

        for operation in operations {
            match operation() {
            }
        }

        last_error
    }
}

/// Function registry for error-aware CURSED functions
pub struct ErrorAwareFunctionRegistry {
impl ErrorAwareFunctionRegistry {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn register<F>(&mut self, name: &str, function: F)
    where
    {
        self.functions.insert(name.to_string(), Box::new(function));
    pub fn call(&self, name: &str, args: Vec<Value>) -> CursedResult<Value> {
        let function = self.functions.get(name)
            .ok_or_else(|| CursedError::Runtime(format!("Function not found: {}", name)))?;
        
        function(args)
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
            let a = match &args[0] {

            let b = match &args[1] {

            if b == 0.0 {
                return error_patterns::runtime_error("Division by zero");
            Ok(Value::Number(a / b))
        });

        registry.register("safe_index", |args| {
            if args.len() != 2 {
                return error_patterns::runtime_error("safe_index requires 2 arguments");
            let array = match &args[0] {

            let index = match &args[1] {

            if index >= array.len() {
                return error_patterns::runtime_error(&format!(
                    array.len()
                ));
            Ok(array[index].clone())
        });

        registry
    }
}

