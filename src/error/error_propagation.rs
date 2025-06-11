use crate::error::Error;
use crate::debug::source_location::SourceLocation;
use std::fmt;
use std::error::Error as StdError;

/// Enhanced error types for error propagation in CURSED
/// 
/// This module provides comprehensive error handling for the `?` operator,
/// including detailed error context, propagation chains, and integration
/// with the CURSED runtime system.

/// Error propagation specific error type
#[derive(Debug, Clone)]
pub struct ErrorPropagationError {
    /// The underlying error being propagated
    pub inner_error: Box<Error>,
    
    /// Source location where propagation occurred
    pub propagation_site: SourceLocation,
    
    /// Chain of propagation sites (for nested propagations)
    pub propagation_chain: Vec<SourceLocation>,
    
    /// Function context where propagation occurred
    pub function_context: Option<String>,
    
    /// Expected return type
    pub expected_type: Option<String>,
    
    /// Additional context information
    pub context: Option<String>,
}

impl ErrorPropagationError {
    /// Create a new error propagation error
    pub fn new(
        inner_error: Error,
        propagation_site: SourceLocation,
    ) -> Self {
        Self {
            inner_error: Box::new(inner_error),
            propagation_site,
            propagation_chain: Vec::new(),
            function_context: None,
            expected_type: None,
            context: None,
        }
    }
    
    /// Create with full context
    pub fn with_context(
        inner_error: Error,
        propagation_site: SourceLocation,
        function_context: Option<String>,
        expected_type: Option<String>,
    ) -> Self {
        Self {
            inner_error: Box::new(inner_error),
            propagation_site,
            propagation_chain: Vec::new(),
            function_context,
            expected_type,
            context: None,
        }
    }
    
    /// Add a propagation site to the chain
    pub fn add_propagation_site(mut self, site: SourceLocation) -> Self {
        self.propagation_chain.push(site);
        self
    }
    
    /// Set additional context
    pub fn with_additional_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
    
    /// Get the original error
    pub fn original_error(&self) -> &Error {
        &self.inner_error
    }
    
    /// Get the full propagation chain including the original site
    pub fn full_propagation_chain(&self) -> Vec<SourceLocation> {
        let mut chain = vec![self.propagation_site.clone()];
        chain.extend_from_slice(&self.propagation_chain);
        chain
    }
    
    /// Get a formatted stack trace of the propagation
    pub fn propagation_stack_trace(&self) -> String {
        let mut trace = String::new();
        trace.push_str(&format!("Error propagated through {} sites:\n", 
                              self.propagation_chain.len() + 1));
        
        for (i, site) in self.full_propagation_chain().iter().enumerate() {
            trace.push_str(&format!("  {}. at line {}, column {}", 
                                  i + 1, site.line, site.column));
            if let Some(ref context) = self.function_context {
                trace.push_str(&format!(" in function '{}'", context));
            }
            trace.push('\n');
        }
        
        trace
    }
}

impl fmt::Display for ErrorPropagationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error propagation failed at line {}, column {}: {}", 
               self.propagation_site.line, 
               self.propagation_site.column,
               self.inner_error)?;
        
        if !self.propagation_chain.is_empty() {
            write!(f, "\nPropagation chain: {} sites", self.propagation_chain.len())?;
        }
        
        if let Some(ref context) = self.function_context {
            write!(f, " in function '{}'", context)?;
        }
        
        if let Some(ref expected) = self.expected_type {
            write!(f, " (expected type: {})", expected)?;
        }
        
        Ok(())
    }
}

impl StdError for ErrorPropagationError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        // Cannot deref Error as it doesn't implement StdError
        None
    }
}

/// Type mismatch error in error propagation
#[derive(Debug, Clone)]
pub struct PropagationTypeMismatchError {
    /// Expected type for propagation
    pub expected_type: String,
    
    /// Actual type found
    pub actual_type: String,
    
    /// Source location of the mismatch
    pub location: SourceLocation,
    
    /// Function context
    pub function_context: Option<String>,
}

impl PropagationTypeMismatchError {
    /// Create a new type mismatch error
    pub fn new(
        expected_type: String,
        actual_type: String,
        location: SourceLocation,
    ) -> Self {
        Self {
            expected_type,
            actual_type,
            location,
            function_context: None,
        }
    }
    
    /// Set function context
    pub fn with_function_context(mut self, context: String) -> Self {
        self.function_context = Some(context);
        self
    }
}

impl fmt::Display for PropagationTypeMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type mismatch in error propagation at line {}, column {}: expected '{}', found '{}'",
               self.location.line, self.location.column, 
               self.expected_type, self.actual_type)?;
        
        if let Some(ref context) = self.function_context {
            write!(f, " in function '{}'", context)?;
        }
        
        Ok(())
    }
}

impl StdError for PropagationTypeMismatchError {}

/// Context validation error for error propagation
#[derive(Debug, Clone)]
pub struct PropagationContextError {
    /// Error message
    pub message: String,
    
    /// Source location
    pub location: SourceLocation,
    
    /// Invalid context type
    pub context_type: String,
    
    /// Suggested solution
    pub suggestion: Option<String>,
}

impl PropagationContextError {
    /// Create a new context error
    pub fn new(
        message: String,
        location: SourceLocation,
        context_type: String,
    ) -> Self {
        Self {
            message,
            location,
            context_type,
            suggestion: None,
        }
    }
    
    /// Add a suggestion for fixing the error
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
}

impl fmt::Display for PropagationContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid context for error propagation at line {}, column {}: {} in {} context",
               self.location.line, self.location.column,
               self.message, self.context_type)?;
        
        if let Some(ref suggestion) = self.suggestion {
            write!(f, ". Suggestion: {}", suggestion)?;
        }
        
        Ok(())
    }
}

impl StdError for PropagationContextError {}

/// Result type for error propagation operations
pub type PropagationResult<T> = Result<T, ErrorPropagationError>;

/// Helper functions for creating common error propagation errors
impl Error {
    /// Create an error propagation error
    pub fn error_propagation_error(
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    ) -> Self {
        Error::ErrorPropagation { message, line, column }
    }
    
    /// Create a type mismatch error for propagation
    pub fn propagation_type_mismatch(
        expected: String,
        actual: String,
        location: SourceLocation,
    ) -> Self {
        Error::ErrorPropagation {
            message: format!("Type mismatch in error propagation: expected '{}', found '{}'", 
                           expected, actual),
            line: Some(location.line as usize),
            column: Some(location.column as usize),
        }
    }
    
    /// Create a context error for propagation
    pub fn propagation_context_error(
        message: String,
        location: SourceLocation,
    ) -> Self {
        Error::ErrorPropagation {
            message,
            line: Some(location.line as usize),
            column: Some(location.column as usize),
        }
    }
}

/// Error propagation utilities
pub struct ErrorPropagationUtils;

impl ErrorPropagationUtils {
    /// Check if an error is propagatable
    pub fn is_propagatable_error(error: &Error) -> bool {
        match error {
            Error::Runtime(_) => true,
            Error::Compile(_) => true,
            Error::ErrorPropagation { .. } => true,
            Error::Panic { .. } => true,
            _ => false,
        }
    }
    
    /// Extract the error message for propagation
    pub fn extract_error_message(error: &Error) -> String {
        match error {
            Error::Io(e) => e.to_string(),
            Error::Parse(msg) => msg.clone(),
            Error::Compile(msg) => msg.clone(),
            Error::Runtime(msg) => msg.clone(),
            Error::Package(msg) => msg.clone(),
            Error::Repl(msg) => msg.clone(),
            Error::TemplateError { message, .. } => message.clone(),
            Error::TypeCompilation(msg) => msg.clone(),
            Error::Type(msg) => msg.clone(),
            Error::Panic { message, .. } => message.clone(),
            Error::Recovery { message, .. } => message.clone(),
            Error::ErrorPropagation { message, .. } => message.clone(),
            Error::ParseError { message, .. } => message.clone(),
            Error::CodeGeneration { message, .. } => message.clone(),
            Error::ProcessError(msg) => msg.clone(),
        }
    }
    
    /// Get the source location from an error if available
    pub fn extract_source_location(error: &Error) -> Option<SourceLocation> {
        match error {
            Error::TemplateError { source_location, .. } => source_location.as_ref().map(|loc| loc.clone().into()),
            Error::Panic { source_location, .. } => source_location.as_ref().map(|loc| loc.clone().into()),
            Error::Recovery { source_location, .. } => source_location.as_ref().map(|loc| loc.clone().into()),
            Error::ErrorPropagation { line, column, .. } => {
                if let (Some(line), Some(column)) = (line, column) {
                    Some(SourceLocation::new(
                        std::path::PathBuf::from("<unknown>"),
                        *line as u32,
                        *column as u32,
                    ))
                } else {
                    None
                }
            },
            Error::ParseError { line, column, .. } => {
                if let (Some(line), Some(column)) = (line, column) {
                    Some(SourceLocation::new(
                        std::path::PathBuf::from("<unknown>"),
                        *line as u32,
                        *column as u32,
                    ))
                } else {
                    None
                }
            },
            Error::CodeGeneration { line, column, .. } => {
                if let (Some(line), Some(column)) = (line, column) {
                    Some(SourceLocation::new(
                        std::path::PathBuf::from("<unknown>"),
                        *line as u32,
                        *column as u32,
                    ))
                } else {
                    None
                }
            },
            _ => None,
        }
    }
    
    /// Chain error propagation
    pub fn chain_propagation(
        original: ErrorPropagationError,
        new_site: SourceLocation,
    ) -> ErrorPropagationError {
        original.add_propagation_site(new_site)
    }
    
    /// Create a comprehensive error propagation trace
    pub fn create_propagation_trace(
        errors: &[ErrorPropagationError],
    ) -> String {
        let mut trace = String::new();
        trace.push_str("Error propagation trace:\n");
        
        for (i, error) in errors.iter().enumerate() {
            trace.push_str(&format!("{}. {}\n", i + 1, error));
            trace.push_str(&error.propagation_stack_trace());
            trace.push('\n');
        }
        
        trace
    }
}

/// Macro for creating error propagation errors with context
#[macro_export]
macro_rules! propagation_error {
    ($inner:expr, $line:expr, $column:expr) => {
        ErrorPropagationError::new(
            $inner,
            SourceLocation::new(
                std::path::PathBuf::from("<unknown>"),
                $line as u32,
                $column as u32,
            ),
        )
    };
    
    ($inner:expr, $line:expr, $column:expr, $context:expr) => {
        ErrorPropagationError::new(
            $inner,
            SourceLocation::new(
                std::path::PathBuf::from("<unknown>"),
                $line as u32,
                $column as u32,
            ),
        ).with_additional_context($context.to_string())
    };
    
    ($inner:expr, $line:expr, $column:expr, $func:expr, $type:expr) => {
        ErrorPropagationError::with_context(
            $inner,
            SourceLocation::new(
                std::path::PathBuf::from("<unknown>"),
                $line as u32,
                $column as u32,
            ),
            Some($func.to_string()),
            Some($type.to_string()),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_propagation_error_creation() {
        let inner = Error::Runtime("Test error".to_string());
        let location = SourceLocation::new(
            std::path::PathBuf::from("<test>"),
            1,
            5,
        );
        let error = ErrorPropagationError::new(inner, location);
        
        assert_eq!(error.propagation_site.line, 1);
        assert_eq!(error.propagation_site.column, 5);
        assert!(error.propagation_chain.is_empty());
    }
    
    #[test]
    fn test_error_propagation_chain() {
        let inner = Error::Runtime("Test error".to_string());
        let location1 = SourceLocation::new(
            std::path::PathBuf::from("<test>"),
            1,
            5,
        );
        let location2 = SourceLocation::new(
            std::path::PathBuf::from("<test>"),
            2,
            10,
        );
        
        let error = ErrorPropagationError::new(inner, location1)
            .add_propagation_site(location2);
        
        assert_eq!(error.propagation_chain.len(), 1);
        assert_eq!(error.full_propagation_chain().len(), 2);
    }
    
    #[test]
    fn test_propagation_type_mismatch_error() {
        let location = SourceLocation::new(
            std::path::PathBuf::from("<test>"),
            1,
            5,
        );
        let error = PropagationTypeMismatchError::new(
            "Result<i32, String>".to_string(),
            "Option<i32>".to_string(),
            location,
        );
        
        assert_eq!(error.expected_type, "Result<i32, String>");
        assert_eq!(error.actual_type, "Option<i32>");
    }
    
    #[test]
    fn test_propagation_context_error() {
        let location = SourceLocation::new(
            std::path::PathBuf::from("<test>"),
            1,
            5,
        );
        let error = PropagationContextError::new(
            "Cannot propagate in global scope".to_string(),
            location,
            "global".to_string(),
        ).with_suggestion("Move to function context".to_string());
        
        assert!(error.suggestion.is_some());
    }
    
    #[test]
    fn test_error_propagation_utils() {
        let runtime_error = Error::Runtime("Test".to_string());
        assert!(ErrorPropagationUtils::is_propagatable_error(&runtime_error));
        
        let io_error = Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "test"));
        assert!(!ErrorPropagationUtils::is_propagatable_error(&io_error));
    }
    
    #[test]
    fn test_extract_error_message() {
        let error = Error::Runtime("Test runtime error".to_string());
        let message = ErrorPropagationUtils::extract_error_message(&error);
        assert_eq!(message, "Test runtime error");
    }
    
    #[test]
    fn test_extract_source_location() {
        let error = Error::ErrorPropagation {
            message: "Test".to_string(),
            line: Some(1),
            column: Some(5),
        };
        
        let location = ErrorPropagationUtils::extract_source_location(&error);
        assert!(location.is_some());
        let loc = location.as_ref().unwrap();
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 5);
    }
    
    #[test]
    fn test_propagation_error_macro() {
        let inner = Error::Runtime("Test".to_string());
        let error = propagation_error!(inner, 1, 5);
        
        assert_eq!(error.propagation_site.line, 1);
        assert_eq!(error.propagation_site.column, 5);
    }
    
    #[test]
    fn test_propagation_stack_trace() {
        let inner = Error::Runtime("Test error".to_string());
        let location1 = SourceLocation::new(
            std::path::PathBuf::from("<test>"),
            1,
            5,
        );
        let location2 = SourceLocation::new(
            std::path::PathBuf::from("<test>"),
            2,
            10,
        );
        
        let error = ErrorPropagationError::new(inner, location1)
            .add_propagation_site(location2)
            .with_additional_context("test function".to_string());
        
        let trace = error.propagation_stack_trace();
        assert!(trace.contains("2 sites"));
        assert!(trace.contains("line 1"));
        assert!(trace.contains("line 2"));
    }
    
    #[test]
    fn test_error_display() {
        let inner = Error::Runtime("Original error".to_string());
        let location = SourceLocation::new(
            std::path::PathBuf::from("<test>"),
            1,
            5,
        );
        let error = ErrorPropagationError::with_context(
            inner,
            location,
            Some("test_function".to_string()),
            Some("Result<i32, String>".to_string()),
        );
        
        let display = format!("{}", error);
        assert!(display.contains("line 1"));
        assert!(display.contains("column 5"));
        assert!(display.contains("test_function"));
        assert!(display.contains("Result<i32, String>"));
    }
}
