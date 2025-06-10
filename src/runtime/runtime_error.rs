/// Enhanced Runtime Error Types for CURSED Error Handling
///
/// This module extends the base error system with runtime-specific
/// error handling capabilities including:
/// - Panic-aware error types
/// - Error chaining and cause tracking
/// - Runtime context preservation
/// - Integration with stack traces and debug info

use crate::error::{Error as CursedError, SourceLocation};
use crate::runtime::stack_trace::{StackTrace, CallFrame};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use std::fmt;
use std::error::Error as StdError;

/// Enhanced runtime error with comprehensive context
#[derive(Debug)]
pub struct RuntimeError {
    /// Base error information
    pub base_error: CursedError,
    /// Unique error identifier
    pub error_id: u64,
    /// Error severity level
    pub severity: ErrorSeverity,
    /// Error category for classification
    pub category: ErrorCategory,
    /// Source location where error originated
    pub source_location: Option<SourceLocation>,
    /// Stack trace at time of error
    pub stack_trace: Option<StackTrace>,
    /// Chain of underlying errors (causes)
    pub error_chain: Vec<RuntimeError>,
    /// Thread ID where error occurred
    pub thread_id: std::thread::ThreadId,
    /// Goroutine ID if error occurred in goroutine
    pub goroutine_id: Option<u64>,
    /// Timestamp when error occurred
    pub timestamp: SystemTime,
    /// Additional context metadata
    pub context: HashMap<String, String>,
    /// Whether this error can trigger recovery
    pub recoverable: bool,
    /// Suggested recovery actions
    pub recovery_suggestions: Vec<RecoverySuggestion>,
}

/// Error severity levels for runtime handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Informational - no action needed
    Info,
    /// Warning - operation can continue but should be noted
    Warning,
    /// Error - operation failed but system is stable
    Error,
    /// Critical - significant system issue, immediate attention needed
    Critical,
    /// Fatal - system integrity compromised, shutdown may be necessary
    Fatal,
}

/// Error categories for classification and handling
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Memory-related errors
    Memory,
    /// Type system errors
    Type,
    /// I/O operation errors
    Io,
    /// Network-related errors
    Network,
    /// Concurrency/synchronization errors
    Concurrency,
    /// Security-related errors
    Security,
    /// Configuration errors
    Configuration,
    /// Validation errors
    Validation,
    /// Compilation errors
    Compilation,
    /// Runtime execution errors
    Runtime,
    /// External dependency errors
    External,
    /// User-defined errors
    User,
    /// Unknown/uncategorized errors
    Unknown,
}

/// Recovery suggestions for error handling
#[derive(Debug, Clone)]
pub struct RecoverySuggestion {
    /// Description of the suggested action
    pub description: String,
    /// Confidence level in the suggestion (0.0 - 1.0)
    pub confidence: f32,
    /// Whether this action is automatically applicable
    pub automatic: bool,
    /// Code example or specific instructions
    pub instructions: Option<String>,
}

impl RecoverySuggestion {
    pub fn new(description: String, confidence: f32) -> Self {
        RecoverySuggestion {
            description,
            confidence: confidence.clamp(0.0, 1.0),
            automatic: false,
            instructions: None,
        }
    }

    pub fn automatic(mut self) -> Self {
        self.automatic = true;
        self
    }

    pub fn with_instructions(mut self, instructions: String) -> Self {
        self.instructions = Some(instructions);
        self
    }
}

impl RuntimeError {
    /// Create a new runtime error
    pub fn new(base_error: CursedError, severity: ErrorSeverity, category: ErrorCategory) -> Self {
        RuntimeError {
            error_id: crate::runtime::error_handling::next_error_id(),
            base_error,
            severity,
            category,
            source_location: None,
            stack_trace: None,
            error_chain: Vec::new(),
            thread_id: std::thread::current().id(),
            goroutine_id: None,
            timestamp: SystemTime::now(),
            context: HashMap::new(),
            recoverable: matches!(severity, ErrorSeverity::Info | ErrorSeverity::Warning | ErrorSeverity::Error),
            recovery_suggestions: Vec::new(),
        }
    }

    /// Create from base error with automatic categorization
    pub fn from_base_error(base_error: CursedError) -> Self {
        let (severity, category) = Self::categorize_base_error(&base_error);
        Self::new(base_error, severity, category)
    }

    /// Add source location information
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    }

    /// Add goroutine context
    pub fn with_goroutine(mut self, goroutine_id: u64) -> Self {
        self.goroutine_id = Some(goroutine_id);
        self
    }

    /// Add stack trace
    pub fn with_stack_trace(mut self, stack_trace: StackTrace) -> Self {
        self.stack_trace = Some(stack_trace);
        self
    }

    /// Add context metadata
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }

    /// Mark as recoverable or non-recoverable
    pub fn set_recoverable(mut self, recoverable: bool) -> Self {
        self.recoverable = recoverable;
        self
    }

    /// Add recovery suggestion
    pub fn with_recovery_suggestion(mut self, suggestion: RecoverySuggestion) -> Self {
        self.recovery_suggestions.push(suggestion);
        self
    }

    /// Add a cause to the error chain
    pub fn with_cause(mut self, cause: RuntimeError) -> Self {
        self.error_chain.push(cause);
        self
    }

    /// Get the root cause (deepest error in chain)
    pub fn root_cause(&self) -> &RuntimeError {
        if let Some(last_cause) = self.error_chain.last() {
            last_cause.root_cause()
        } else {
            self
        }
    }

    /// Get all errors in the chain (including self)
    pub fn error_chain(&self) -> Vec<&RuntimeError> {
        let mut chain = vec![self];
        for cause in &self.error_chain {
            chain.extend(cause.error_chain());
        }
        chain
    }

    /// Check if error is of a specific category
    pub fn is_category(&self, category: ErrorCategory) -> bool {
        self.category == category
    }

    /// Check if error is at or above a severity level
    pub fn is_severity_at_least(&self, severity: ErrorSeverity) -> bool {
        self.severity >= severity
    }

    /// Get the best recovery suggestion
    pub fn best_recovery_suggestion(&self) -> Option<&RecoverySuggestion> {
        self.recovery_suggestions
            .iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Get automatic recovery suggestions
    pub fn automatic_recovery_suggestions(&self) -> Vec<&RecoverySuggestion> {
        self.recovery_suggestions
            .iter()
            .filter(|s| s.automatic)
            .collect()
    }

    /// Convert to base CURSED error
    pub fn to_cursed_error(&self) -> CursedError {
        match self.severity {
            ErrorSeverity::Fatal | ErrorSeverity::Critical => {
                CursedError::panic_with_details(
                    self.base_error.to_string(),
                    self.error_id,
                    self.recoverable,
                    self.source_location.clone(),
                )
            }
            _ => self.base_error.clone(),
        }
    }

    /// Create error context summary
    pub fn create_context_summary(&self) -> String {
        let mut summary = Vec::new();
        
        summary.push(format!("Error #{}: {}", self.error_id, self.base_error));
        summary.push(format!("Severity: {:?}, Category: {:?}", self.severity, self.category));
        
        if let Some(location) = &self.source_location {
            summary.push(format!("Location: {}", location));
        }
        
        if let Some(goroutine_id) = self.goroutine_id {
            summary.push(format!("Goroutine: #{}", goroutine_id));
        }
        
        if !self.context.is_empty() {
            summary.push("Context:".to_string());
            for (key, value) in &self.context {
                summary.push(format!("  {}: {}", key, value));
            }
        }
        
        if !self.error_chain.is_empty() {
            summary.push(format!("Error chain ({} causes):", self.error_chain.len()));
            for (i, cause) in self.error_chain.iter().enumerate() {
                summary.push(format!("  {}: {}", i + 1, cause.base_error));
            }
        }
        
        summary.join("\n")
    }

    // Helper method to categorize base errors
    fn categorize_base_error(error: &CursedError) -> (ErrorSeverity, ErrorCategory) {
        match error {
            CursedError::Io(_) => (ErrorSeverity::Error, ErrorCategory::Io),
            CursedError::Parse(_) => (ErrorSeverity::Error, ErrorCategory::Compilation),
            CursedError::Compile(_) => (ErrorSeverity::Error, ErrorCategory::Compilation),
            CursedError::Runtime(_) => (ErrorSeverity::Error, ErrorCategory::Runtime),
            CursedError::Type(_) => (ErrorSeverity::Error, ErrorCategory::Type),
            CursedError::TypeCompilation(_) => (ErrorSeverity::Error, ErrorCategory::Type),
            CursedError::Package(_) => (ErrorSeverity::Warning, ErrorCategory::External),
            CursedError::Repl(_) => (ErrorSeverity::Warning, ErrorCategory::User),
            CursedError::TemplateError { .. } => (ErrorSeverity::Error, ErrorCategory::Runtime),
            CursedError::Panic { recoverable: true, .. } => (ErrorSeverity::Error, ErrorCategory::Runtime),
            CursedError::Panic { recoverable: false, .. } => (ErrorSeverity::Critical, ErrorCategory::Runtime),
            CursedError::Recovery { .. } => (ErrorSeverity::Warning, ErrorCategory::Runtime),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?} Error #{}: {}", self.severity, self.error_id, self.base_error)?;
        
        if let Some(location) = &self.source_location {
            writeln!(f, "  at {}", location)?;
        }
        
        if let Some(goroutine_id) = self.goroutine_id {
            writeln!(f, "  in goroutine #{}", goroutine_id)?;
        }
        
        if !self.context.is_empty() {
            writeln!(f, "Context:")?;
            for (key, value) in &self.context {
                writeln!(f, "  {}: {}", key, value)?;
            }
        }
        
        if !self.error_chain.is_empty() {
            writeln!(f, "Caused by:")?;
            for (i, cause) in self.error_chain.iter().enumerate() {
                writeln!(f, "  {}: {}", i + 1, cause.base_error)?;
            }
        }
        
        if let Some(trace) = &self.stack_trace {
            writeln!(f, "\n{}", trace)?;
        }
        
        if !self.recovery_suggestions.is_empty() {
            writeln!(f, "Recovery suggestions:")?;
            for suggestion in &self.recovery_suggestions {
                write!(f, "  - {} (confidence: {:.0}%)", 
                       suggestion.description, suggestion.confidence * 100.0)?;
                if suggestion.automatic {
                    write!(f, " [automatic]")?;
                }
                writeln!(f)?;
                if let Some(instructions) = &suggestion.instructions {
                    writeln!(f, "    {}", instructions)?;
                }
            }
        }
        
        Ok(())
    }
}

impl StdError for RuntimeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        // Return the first cause in the chain as the source
        self.error_chain.first().map(|e| e as &dyn StdError)
    }
}

impl From<CursedError> for RuntimeError {
    fn from(error: CursedError) -> Self {
        RuntimeError::from_base_error(error)
    }
}

impl From<RuntimeError> for CursedError {
    fn from(error: RuntimeError) -> Self {
        error.to_cursed_error()
    }
}

/// Error builder for fluent error construction
pub struct RuntimeErrorBuilder {
    error: RuntimeError,
}

impl RuntimeErrorBuilder {
    pub fn new(base_error: CursedError) -> Self {
        RuntimeErrorBuilder {
            error: RuntimeError::from_base_error(base_error),
        }
    }

    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.error.severity = severity;
        self
    }

    pub fn with_category(mut self, category: ErrorCategory) -> Self {
        self.error.category = category;
        self
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.error = self.error.with_location(location);
        self
    }

    pub fn with_goroutine(mut self, goroutine_id: u64) -> Self {
        self.error = self.error.with_goroutine(goroutine_id);
        self
    }

    pub fn with_stack_trace(mut self, stack_trace: StackTrace) -> Self {
        self.error = self.error.with_stack_trace(stack_trace);
        self
    }

    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.error = self.error.with_context(key, value);
        self
    }

    pub fn recoverable(mut self, recoverable: bool) -> Self {
        self.error = self.error.set_recoverable(recoverable);
        self
    }

    pub fn with_recovery_suggestion(mut self, suggestion: RecoverySuggestion) -> Self {
        self.error = self.error.with_recovery_suggestion(suggestion);
        self
    }

    pub fn with_cause(mut self, cause: RuntimeError) -> Self {
        self.error = self.error.with_cause(cause);
        self
    }

    pub fn build(self) -> RuntimeError {
        self.error
    }
}

/// Convenience functions for creating common runtime errors

/// Create a memory error
pub fn memory_error(message: String) -> RuntimeError {
    RuntimeErrorBuilder::new(CursedError::Runtime(message))
        .with_severity(ErrorSeverity::Critical)
        .with_category(ErrorCategory::Memory)
        .with_recovery_suggestion(RecoverySuggestion::new(
            "Try reducing memory usage or increasing available memory".to_string(),
            0.8
        ))
        .build()
}

/// Create a type error
pub fn type_error(message: String) -> RuntimeError {
    RuntimeErrorBuilder::new(CursedError::Type(message))
        .with_severity(ErrorSeverity::Error)
        .with_category(ErrorCategory::Type)
        .with_recovery_suggestion(RecoverySuggestion::new(
            "Check type annotations and ensure type compatibility".to_string(),
            0.9
        ))
        .build()
}

/// Create a concurrency error
pub fn concurrency_error(message: String) -> RuntimeError {
    RuntimeErrorBuilder::new(CursedError::Runtime(message))
        .with_severity(ErrorSeverity::Error)
        .with_category(ErrorCategory::Concurrency)
        .with_recovery_suggestion(RecoverySuggestion::new(
            "Review synchronization and check for race conditions".to_string(),
            0.7
        ))
        .build()
}

/// Create a validation error
pub fn validation_error(message: String, field: Option<String>) -> RuntimeError {
    let mut builder = RuntimeErrorBuilder::new(CursedError::Runtime(message))
        .with_severity(ErrorSeverity::Warning)
        .with_category(ErrorCategory::Validation)
        .with_recovery_suggestion(RecoverySuggestion::new(
            "Check input validation rules and correct the invalid data".to_string(),
            0.95
        ));

    if let Some(field_name) = field {
        builder = builder.with_context("invalid_field".to_string(), field_name);
    }

    builder.build()
}

/// Create a configuration error
pub fn configuration_error(message: String, config_key: Option<String>) -> RuntimeError {
    let mut builder = RuntimeErrorBuilder::new(CursedError::Runtime(message))
        .with_severity(ErrorSeverity::Error)
        .with_category(ErrorCategory::Configuration)
        .with_recovery_suggestion(RecoverySuggestion::new(
            "Review configuration settings and ensure all required values are provided".to_string(),
            0.9
        ));

    if let Some(key) = config_key {
        builder = builder.with_context("config_key".to_string(), key);
    }

    builder.build()
}

/// Create a network error
pub fn network_error(message: String, endpoint: Option<String>) -> RuntimeError {
    let mut builder = RuntimeErrorBuilder::new(CursedError::Runtime(message))
        .with_severity(ErrorSeverity::Error)
        .with_category(ErrorCategory::Network)
        .with_recovery_suggestion(RecoverySuggestion::new(
            "Check network connectivity and retry the operation".to_string(),
            0.6
        ))
        .with_recovery_suggestion(RecoverySuggestion::new(
            "Implement exponential backoff for retries".to_string(),
            0.8
        ));

    if let Some(endpoint_addr) = endpoint {
        builder = builder.with_context("endpoint".to_string(), endpoint_addr);
    }

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_error_creation() {
        let base_error = CursedError::Runtime("Test error".to_string());
        let runtime_error = RuntimeError::new(base_error, ErrorSeverity::Error, ErrorCategory::Runtime);

        assert!(runtime_error.error_id > 0);
        assert_eq!(runtime_error.severity, ErrorSeverity::Error);
        assert_eq!(runtime_error.category, ErrorCategory::Runtime);
        assert!(runtime_error.recoverable);
    }

    #[test]
    fn test_error_builder() {
        let base_error = CursedError::Type("Invalid type".to_string());
        let runtime_error = RuntimeErrorBuilder::new(base_error)
            .with_severity(ErrorSeverity::Critical)
            .with_category(ErrorCategory::Type)
            .with_location(SourceLocation::new(10, 5))
            .with_context("function".to_string(), "test_func".to_string())
            .recoverable(false)
            .build();

        assert_eq!(runtime_error.severity, ErrorSeverity::Critical);
        assert_eq!(runtime_error.category, ErrorCategory::Type);
        assert!(runtime_error.source_location.is_some());
        assert!(!runtime_error.recoverable);
        assert_eq!(runtime_error.context.get("function"), Some(&"test_func".to_string()));
    }

    #[test]
    fn test_error_chain() {
        let cause1 = RuntimeError::from_base_error(CursedError::Runtime("Root cause".to_string()));
        let cause2 = RuntimeError::from_base_error(CursedError::Runtime("Intermediate cause".to_string()))
            .with_cause(cause1);
        let main_error = RuntimeError::from_base_error(CursedError::Runtime("Main error".to_string()))
            .with_cause(cause2);

        let chain = main_error.error_chain();
        assert_eq!(chain.len(), 3); // main + 2 causes
        
        let root = main_error.root_cause();
        assert!(root.base_error.to_string().contains("Root cause"));
    }

    #[test]
    fn test_recovery_suggestions() {
        let suggestion1 = RecoverySuggestion::new("Try again".to_string(), 0.5);
        let suggestion2 = RecoverySuggestion::new("Check configuration".to_string(), 0.9)
            .automatic()
            .with_instructions("Run config --validate".to_string());

        let runtime_error = RuntimeErrorBuilder::new(CursedError::Runtime("Test".to_string()))
            .with_recovery_suggestion(suggestion1)
            .with_recovery_suggestion(suggestion2)
            .build();

        assert_eq!(runtime_error.recovery_suggestions.len(), 2);
        
        let best = runtime_error.best_recovery_suggestion().unwrap();
        assert_eq!(best.confidence, 0.9);
        
        let automatic = runtime_error.automatic_recovery_suggestions();
        assert_eq!(automatic.len(), 1);
        assert!(automatic[0].automatic);
    }

    #[test]
    fn test_convenience_functions() {
        let mem_error = memory_error("Out of memory".to_string());
        assert_eq!(mem_error.category, ErrorCategory::Memory);
        assert_eq!(mem_error.severity, ErrorSeverity::Critical);

        let type_err = type_error("Type mismatch".to_string());
        assert_eq!(type_err.category, ErrorCategory::Type);
        assert_eq!(type_err.severity, ErrorSeverity::Error);

        let validation_err = validation_error("Invalid input".to_string(), Some("email".to_string()));
        assert_eq!(validation_err.category, ErrorCategory::Validation);
        assert_eq!(validation_err.context.get("invalid_field"), Some(&"email".to_string()));
    }

    #[test]
    fn test_error_categorization() {
        let io_error = RuntimeError::from_base_error(CursedError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound, "File not found"
        )));
        assert_eq!(io_error.category, ErrorCategory::Io);
        assert_eq!(io_error.severity, ErrorSeverity::Error);

        let panic_error = RuntimeError::from_base_error(CursedError::panic_error("Panic!".to_string()));
        assert_eq!(panic_error.severity, ErrorSeverity::Critical);
    }

    #[test]
    fn test_context_summary() {
        let runtime_error = RuntimeErrorBuilder::new(CursedError::Runtime("Test error".to_string()))
            .with_location(SourceLocation::new(10, 5).with_file("test.csd"))
            .with_goroutine(123)
            .with_context("operation".to_string(), "file_read".to_string())
            .build();

        let summary = runtime_error.create_context_summary();
        assert!(summary.contains("Error #"));
        assert!(summary.contains("test.csd:10:5"));
        assert!(summary.contains("Goroutine: #123"));
        assert!(summary.contains("operation: file_read"));
    }

    #[test]
    fn test_severity_comparison() {
        assert!(ErrorSeverity::Info < ErrorSeverity::Warning);
        assert!(ErrorSeverity::Warning < ErrorSeverity::Error);
        assert!(ErrorSeverity::Error < ErrorSeverity::Critical);
        assert!(ErrorSeverity::Critical < ErrorSeverity::Fatal);

        let error = RuntimeError::new(
            CursedError::Runtime("Test".to_string()),
            ErrorSeverity::Error,
            ErrorCategory::Runtime
        );

        assert!(error.is_severity_at_least(ErrorSeverity::Warning));
        assert!(error.is_severity_at_least(ErrorSeverity::Error));
        assert!(!error.is_severity_at_least(ErrorSeverity::Critical));
    }
}
