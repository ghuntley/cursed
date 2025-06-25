/// Enhanced Runtime CursedError Types for CURSED CursedError Handling
///
/// This module extends the base error system with runtime-specific
/// error handling capabilities including:
/// - Panic-aware error types
/// - CursedError chaining and cause tracking
/// - Runtime context preservation
/// - Integration with stack traces and debug info

use crate::error::{CursedError, SourceLocation};
use crate::runtime::stack_trace::{StackTrace, CallFrame};

use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use std::fmt;

/// Enhanced runtime error with comprehensive context
#[derive(Debug)]
pub struct RuntimeError {
    /// Base error information
    /// Unique error identifier
    /// CursedError severity level
    /// CursedError category for classification
    /// Source location where error originated
    /// Stack trace at time of error
    /// Chain of underlying errors (causes)
    /// Thread ID where error occurred
    /// Goroutine ID if error occurred in goroutine
    /// Timestamp when error occurred
    /// Additional context metadata
    /// Whether this error can trigger recovery
    /// Suggested recovery actions
/// CursedError severity levels for runtime handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Informational - no action needed
    /// Warning - operation can continue but should be noted
    /// CursedError - operation failed but system is stable
    /// Critical - significant system issue, immediate attention needed
    /// Fatal - system integrity compromised, shutdown may be necessary
/// CursedError categories for classification and handling
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Memory-related errors
    /// Type system errors
    /// I/O operation errors
    /// Network-related errors
    /// Concurrency/synchronization errors
    /// Security-related errors
    /// Configuration errors
    /// Validation errors
    /// Compilation errors
    /// Runtime execution errors
    /// External dependency errors
    /// User-defined errors
    /// Unknown/uncategorized errors
/// Recovery suggestions for error handling
#[derive(Debug, Clone)]
pub struct RecoverySuggestion {
    /// Description of the suggested action
    /// Confidence level in the suggestion (0.0 - 1.0)
    /// Whether this action is automatically applicable
    /// Code example or specific instructions
impl RecoverySuggestion {
    pub fn new(description: String, confidence: f32) -> Self {
        RecoverySuggestion {
        }
    }

    pub fn automatic(mut self) -> Self {
        self.automatic = true;
        self
    pub fn with_instructions(mut self, instructions: String) -> Self {
        self.instructions = Some(instructions);
        self
    }
}

impl RuntimeError {
    /// Create a new runtime error
    pub fn new(base_error: CursedError, severity: ErrorSeverity, category: ErrorCategory) -> Self {
        RuntimeError {
        }
    }

    /// Create from base error with automatic categorization
    pub fn from_base_error(base_error: CursedError) -> Self {
        let (severity, category) = Self::categorize_base_error(&base_error);
        Self::new(base_error, severity, category)
    /// Add source location information
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    /// Add goroutine context
    pub fn with_goroutine(mut self, goroutine_id: u64) -> Self {
        self.goroutine_id = Some(goroutine_id);
        self
    /// Add stack trace
    pub fn with_stack_trace(mut self, stack_trace: StackTrace) -> Self {
        self.stack_trace = Some(stack_trace);
        self
    /// Add context metadata
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    /// Mark as recoverable or non-recoverable
    pub fn set_recoverable(mut self, recoverable: bool) -> Self {
        self.recoverable = recoverable;
        self
    /// Add recovery suggestion
    pub fn with_recovery_suggestion(mut self, suggestion: RecoverySuggestion) -> Self {
        self.recovery_suggestions.push(suggestion);
        self
    /// Add a cause to the error chain
    pub fn with_cause(mut self, cause: RuntimeError) -> Self {
        self.error_chain.push(cause);
        self
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
    /// Check if error is of a specific category
    pub fn is_category(&self, category: ErrorCategory) -> bool {
        self.category == category
    /// Check if error is at or above a severity level
    pub fn is_severity_at_least(&self, severity: ErrorSeverity) -> bool {
        self.severity >= severity
    /// Get the best recovery suggestion
    pub fn best_recovery_suggestion(&self) -> Option<&RecoverySuggestion> {
        self.recovery_suggestions
            .iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap_or(std::cmp::Ordering::Equal))
    /// Get automatic recovery suggestions
    pub fn automatic_recovery_suggestions(&self) -> Vec<&RecoverySuggestion> {
        self.recovery_suggestions
            .iter()
            .filter(|s| s.automatic)
            .collect()
    /// Convert to base CURSED error
    pub fn to_cursed_error(&self) -> CursedError {
        match self.severity {
            ErrorSeverity::Fatal | ErrorSeverity::Critical => {
                CursedError::panic_with_details(
                )
            }
        }
    }

    /// Create error context summary
    pub fn create_context_summary(&self) -> String {
        let mut summary = Vec::new();
        
        summary.push(format!("CursedError #{}: {}", self.error_id, self.base_error));
        summary.push(format!("Severity: {:?}, Category: {:?}", self.severity, self.category));
        
        if let Some(location) = &self.source_location {
            summary.push(format!("Location: {}", location));
        if let Some(goroutine_id) = self.goroutine_id {
            summary.push(format!("Goroutine: #{}", goroutine_id));
        if !self.context.is_empty() {
            summary.push("Context:".to_string());
            for (key, value) in &self.context {
                summary.push(format!("  {}: {}", key, value));
            }
        }
        
        if !self.error_chain.is_empty() {
            for (i, cause) in self.error_chain.iter().enumerate() {
                summary.push(format!("  {}: {}", i + 1, cause.base_error));
            }
        }
        
        summary.join("\n")
    // Helper method to categorize base errors
    fn categorize_base_error(error: &CursedError) -> (ErrorSeverity, ErrorCategory) {
        match error {
        }
    }
// impl fmt::Display for RuntimeError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "{:?} CursedError #{}: {}", self.severity, self.error_id, self.base_error)?;
//         
//         if let Some(location) = &self.source_location {
//             writeln!(f, "  at {}", location)?;
//         }
//         
//         if let Some(goroutine_id) = self.goroutine_id {
//             writeln!(f, "  in goroutine #{}", goroutine_id)?;
//         }
//         
//         if !self.context.is_empty() {
//             writeln!(f, "Context:")?;
//             for (key, value) in &self.context {
//                 writeln!(f, "  {}: {}", key, value)?;
//             }
//         }
//         
//         if !self.error_chain.is_empty() {
//             writeln!(f, "Caused by:")?;
//             for (i, cause) in self.error_chain.iter().enumerate() {
//                 writeln!(f, "  {}: {}", i + 1, cause.base_error)?;
//             }
//         }
//         
//         if let Some(trace) = &self.stack_trace {
//             writeln!(f, "\n{}", trace)?;
//         }
//         
//         if !self.recovery_suggestions.is_empty() {
//             writeln!(f, "Recovery suggestions:")?;
//             for suggestion in &self.recovery_suggestions {
//                 write!(f, "  - {} (confidence: {:.0}%)", 
//                        suggestion.description, suggestion.confidence * 100.0)?;
//                 if suggestion.automatic {
//                     write!(f, " [automatic]")?;
//                 }
//                 writeln!(f)?;
//                 if let Some(instructions) = &suggestion.instructions {
//                     writeln!(f, "    {}", instructions)?;
//                 }
//             }
//         }
//         
//         Ok(())
//     }
// }

impl StdError for RuntimeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        // Return the first cause in the chain as the source
        self.error_chain.first().map(|e| e as &dyn StdError)
    }
}

// impl From<CursedError> for RuntimeError {
//     fn from(error: CursedError) -> Self {
//         RuntimeError::from_base_error(error)
//     }
// }

// impl From<RuntimeError> for CursedError {
//     fn from(error: RuntimeError) -> Self {
//         error.to_cursed_error()
//     }
// }

/// CursedError builder for fluent error construction
pub struct RuntimeErrorBuilder {
impl RuntimeErrorBuilder {
    pub fn new(base_error: CursedError) -> Self {
        RuntimeErrorBuilder {
        }
    }

    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.error.severity = severity;
        self
    pub fn with_category(mut self, category: ErrorCategory) -> Self {
        self.error.category = category;
        self
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.error = self.error.with_location(location);
        self
    pub fn with_goroutine(mut self, goroutine_id: u64) -> Self {
        self.error = self.error.with_goroutine(goroutine_id);
        self
    pub fn with_stack_trace(mut self, stack_trace: StackTrace) -> Self {
        self.error = self.error.with_stack_trace(stack_trace);
        self
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.error = self.error.with_context(key, value);
        self
    pub fn recoverable(mut self, recoverable: bool) -> Self {
        self.error = self.error.set_recoverable(recoverable);
        self
    pub fn with_recovery_suggestion(mut self, suggestion: RecoverySuggestion) -> Self {
        self.error = self.error.with_recovery_suggestion(suggestion);
        self
    pub fn with_cause(mut self, cause: RuntimeError) -> Self {
        self.error = self.error.with_cause(cause);
        self
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
            0.8
        ))
        .build()
/// Create a type error
pub fn type_error(message: String) -> RuntimeError {
    RuntimeErrorBuilder::new(CursedError::Type(message))
        .with_severity(ErrorSeverity::CursedError)
        .with_category(ErrorCategory::Type)
        .with_recovery_suggestion(RecoverySuggestion::new(
            0.9
        ))
        .build()
/// Create a concurrency error
pub fn concurrency_error(message: String) -> RuntimeError {
    RuntimeErrorBuilder::new(CursedError::Runtime(message))
        .with_severity(ErrorSeverity::CursedError)
        .with_category(ErrorCategory::Concurrency)
        .with_recovery_suggestion(RecoverySuggestion::new(
            0.7
        ))
        .build()
/// Create a validation error
pub fn validation_error(message: String, field: Option<String>) -> RuntimeError {
    let mut builder = RuntimeErrorBuilder::new(CursedError::Runtime(message))
        .with_severity(ErrorSeverity::Warning)
        .with_category(ErrorCategory::Validation)
        .with_recovery_suggestion(RecoverySuggestion::new(
            0.95
        ));

    if let Some(field_name) = field {
        builder = builder.with_context("invalid_field".to_string(), field_name);
    builder.build()
/// Create a configuration error
pub fn configuration_error(message: String, config_key: Option<String>) -> RuntimeError {
    let mut builder = RuntimeErrorBuilder::new(CursedError::Runtime(message))
        .with_severity(ErrorSeverity::CursedError)
        .with_category(ErrorCategory::Configuration)
        .with_recovery_suggestion(RecoverySuggestion::new(
            0.9
        ));

    if let Some(key) = config_key {
        builder = builder.with_context("config_key".to_string(), key);
    builder.build()
/// Create a network error
pub fn network_error(message: String, endpoint: Option<String>) -> RuntimeError {
    let mut builder = RuntimeErrorBuilder::new(CursedError::Runtime(message))
        .with_severity(ErrorSeverity::CursedError)
        .with_category(ErrorCategory::Network)
        .with_recovery_suggestion(RecoverySuggestion::new(
            0.6
        ))
        .with_recovery_suggestion(RecoverySuggestion::new(
            0.8
        ));

    if let Some(endpoint_addr) = endpoint {
        builder = builder.with_context("endpoint".to_string(), endpoint_addr);
    builder.build()
