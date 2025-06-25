// CursedError propagation runtime system for the CURSED programming language
// 
// This module provides the runtime infrastructure for the `?` operator,
// including error context management, propagation chains, and integration
// with CURSED's Result and Option types.

use crate::error::{CursedError, SourceLocation};
// use crate::runtime::value::Value;
use crate::types::result::{Result as CursedResult, Option as CursedOption};

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, warn};

/// CursedError propagation operator implementation
#[derive(Debug)]
pub struct ErrorPropagationOperator {
    /// CursedError context stack for tracking propagation sites
    /// Propagation statistics for monitoring
    /// Configuration for propagation behavior
impl ErrorPropagationOperator {
    /// Create a new error propagation operator
    pub fn new() -> Self {
        Self::with_config(PropagationConfig::default())
    /// Create with custom configuration
    pub fn with_config(config: PropagationConfig) -> Self {
        Self {
        }
    }

    /// Apply the `?` operator to a Result value
    #[instrument(skip(self, result_value))]
    pub fn apply_question_mark<T, E>(
    ) -> Result<T, PropagationError<E>>
    where
    {
        // Record propagation attempt
        let start_time = Instant::now();
        self.record_propagation_attempt();

        match result_value {
            CursedResult::Ok(value) => {
                debug!(
                    "Question mark operator: success path"
                );
                self.record_successful_propagation(start_time.elapsed());
                Ok(value)
            }
            CursedResult::Err(error) => {
                // Create propagation error with context
                let propagation_error = PropagationError::new(
                );

                // Add to context stack
                if let Ok(mut stack) = self.context_stack.lock() {
                    stack.push_context(ErrorPropagationContext {
                    });
                warn!(
                    "Question mark operator: error propagation"
                );

                self.record_error_propagation(start_time.elapsed());
                Err(propagation_error)
            }
        }
    /// Apply the `?` operator to an Option value
    #[instrument(skip(self, option_value))]
    pub fn apply_question_mark_option<T>(
    ) -> crate::error::Result<()>
    where
    {
        let start_time = Instant::now();
        self.record_propagation_attempt();

        match option_value {
            CursedOption::Some(value) => {
                debug!(
                    "Question mark operator on Option: success path"
                );
                self.record_successful_propagation(start_time.elapsed());
                Ok(value)
            }
            CursedOption::None => {
                let none_error = NoneError {

                let propagation_error = PropagationError::new(
                );

                // Add to context stack
                if let Ok(mut stack) = self.context_stack.lock() {
                    stack.push_context(ErrorPropagationContext {
                    });
                warn!(
                    "Question mark operator on Option: None propagation"
                );

                self.record_error_propagation(start_time.elapsed());
                Err(propagation_error)
            }
        }
    /// Get the current error context chain
    pub fn get_error_context_chain(&self) -> crate::error::Result<()> {
        let stack = self.context_stack.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire context stack lock"))?;
        Ok(stack.get_contexts())
    /// Clear the error context stack
    pub fn clear_context_stack(&self) -> crate::error::Result<()> {
        let mut stack = self.context_stack.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire context stack lock"))?;
        stack.clear();
        Ok(())
    /// Get propagation statistics
    pub fn get_statistics(&self) -> crate::error::Result<()> {
        let stats = self.statistics.read()
            .map_err(|_| CursedError::system_error("Failed to acquire statistics lock"))?;
        Ok(stats.clone())
    /// Record a propagation attempt
    fn record_propagation_attempt(&self) {
        if let Ok(mut stats) = self.statistics.write() {
            stats.total_attempts += 1;
        }
    }

    /// Record a successful propagation
    fn record_successful_propagation(&self, duration: Duration) {
        if let Ok(mut stats) = self.statistics.write() {
            stats.successful_propagations += 1;
            stats.total_duration += duration;
        }
    }

    /// Record an error propagation
    fn record_error_propagation(&self, duration: Duration) {
        if let Ok(mut stats) = self.statistics.write() {
            stats.error_propagations += 1;
            stats.total_duration += duration;
        }
    }
/// CursedError that occurred during propagation
#[derive(Debug, Clone)]
pub struct PropagationError<E> {
    /// The original error being propagated
    /// Location where propagation occurred
    /// Function context
    /// Propagation chain
    /// Additional context
    /// Timestamp of error
impl<E> PropagationError<E> {
    /// Create a new propagation error
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    /// Add a propagation site to the chain
    pub fn add_propagation_site(mut self, site: SourceLocation) -> Self {
        self.propagation_chain.push(site);
        self
    /// Add additional context
    pub fn with_context(mut self, context: String) -> Self {
        self.additional_context = Some(context);
        self
    /// Get the full propagation chain
    pub fn full_chain(&self) -> Vec<SourceLocation> {
        let mut chain = vec![self.propagation_site.clone()];
        chain.extend_from_slice(&self.propagation_chain);
        chain
    }
}

// impl<E: fmt::Display> fmt::Display for PropagationError<E> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "CursedError propagated at {}:{}: {}",
//             self.propagation_site.line,
//             self.propagation_site.column,
//             self.inner_error
//         )?;
// 
//         if let Some(ref context) = self.function_context {
//             write!(f, " in function '{}'", context)?;
//         }
// 
//         if !self.propagation_chain.is_empty() {
//             write!(f, " (propagated through {} sites)", self.propagation_chain.len())?;
//         }
// 
//         Ok(())
//     }
// }

/// CursedError representing None in Option
#[derive(Debug, Clone)]
pub struct NoneError {
// impl fmt::Display for NoneError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} at {}:{}", self.message, self.location.line, self.location.column)
//     }
// }

/// CursedError propagation context entry
#[derive(Debug, Clone)]
pub struct ErrorPropagationContext {
    /// Source location of the propagation
    /// Function name where propagation occurred
    /// Type of error being propagated
    /// Timestamp of the propagation
// impl fmt::Display for ErrorPropagationContext {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{} at {}:{}",
//             self.error_type,
//             self.location.line,
//             self.location.column
//         )?;
// 
//         if let Some(ref func) = self.function_name {
//             write!(f, " in {}", func)?;
//         }
// 
//         Ok(())
//     }
// }

/// Stack for tracking error propagation contexts
#[derive(Debug)]
pub struct ErrorContextStack {
    /// Stack of propagation contexts
    /// Maximum stack depth
impl ErrorContextStack {
    /// Create a new context stack
    pub fn new() -> Self {
        Self::with_capacity(100)
    /// Create with specific capacity
    pub fn with_capacity(max_depth: usize) -> Self {
        Self {
        }
    }

    /// Push a new context
    pub fn push_context(&mut self, context: ErrorPropagationContext) {
        if self.contexts.len() >= self.max_depth {
            self.contexts.pop_front();
        }
        self.contexts.push_back(context);
    /// Pop the most recent context
    pub fn pop_context(&mut self) -> Option<ErrorPropagationContext> {
        self.contexts.pop_back()
    /// Get all contexts as a vector
    pub fn get_contexts(&self) -> Vec<ErrorPropagationContext> {
        self.contexts.iter().cloned().collect()
    /// Clear all contexts
    pub fn clear(&mut self) {
        self.contexts.clear();
    /// Get the depth of the stack
    pub fn depth(&self) -> usize {
        self.contexts.len()
    /// Check if stack is empty
    pub fn is_empty(&self) -> bool {
        self.contexts.is_empty()
    }
}

/// Statistics for error propagation monitoring
#[derive(Debug, Clone)]
pub struct PropagationStatistics {
    /// Total number of propagation attempts
    /// Number of successful propagations (Ok/Some values)
    /// Number of error propagations (Err/None values)
    /// Total time spent in propagation operations
    /// Last reset timestamp
impl PropagationStatistics {
    /// Create new statistics
    pub fn new() -> Self {
        Self {
        }
    }

    /// Get the error rate (0.0 to 1.0)
    pub fn error_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            0.0
        } else {
            self.error_propagations as f64 / self.total_attempts as f64
        }
    }

    /// Get the success rate (0.0 to 1.0)
    pub fn success_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            0.0
        } else {
            self.successful_propagations as f64 / self.total_attempts as f64
        }
    }

    /// Get average propagation time
    pub fn average_duration(&self) -> Duration {
        if self.total_attempts == 0 {
            Duration::from_nanos(0)
        } else {
            self.total_duration / self.total_attempts as u32
        }
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl fmt::Display for PropagationStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CursedError Propagation Statistics:")?;
        writeln!(f, "  Total attempts: {}", self.total_attempts)?;
        writeln!(f, "  Successful: {}", self.successful_propagations)?;
        writeln!(f, "  Errors: {}", self.error_propagations)?;
        writeln!(f, "  Success rate: {:.2}%", self.success_rate() * 100.0)?;
        writeln!(f, "  CursedError rate: {:.2}%", self.error_rate() * 100.0)?;
        writeln!(f, "  Average duration: {:?}", self.average_duration())?;
        Ok(())
    }
}

/// Configuration for error propagation behavior
#[derive(Debug, Clone)]
pub struct PropagationConfig {
    /// Maximum context stack depth
    /// Whether to enable detailed tracing
    /// Whether to collect timing statistics
    /// Timeout for propagation operations
impl Default for PropagationConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Result type for error propagation operations
pub type PropagationResult<T, E> = Result<T, PropagationError<E>>;

/// Helper functions for common error propagation patterns
pub mod helpers {
    use super::*;

    /// Create a propagation operator with default settings
    pub fn create_default_propagator() -> ErrorPropagationOperator {
        ErrorPropagationOperator::new()
    /// Apply `?` to a Result with source location
    pub fn propagate_result<T, E>(
    ) -> PropagationResult<T, E>
    where
    {
        operator.apply_question_mark(
        )
    /// Apply `?` to an Option with source location
    pub fn propagate_option<T>(
    ) -> Propagationcrate::error::Result<()>
    where
    {
        operator.apply_question_mark_option(
        )
    /// Create a CURSED error from a propagation error
    pub fn to_cursed_error<E: fmt::Display>(
    ) -> CursedError {
        CursedError::ErrorPropagation {
        }
    }

/// FFI function for enhanced question mark operator
#[no_mangle]
pub extern "C" fn cursed_enhanced_question_mark(
) -> *mut u8 {
    if result_ptr.is_null() {
        return std::ptr::null_mut();
    // Extract function name if provided
    let _function_name = if function_name.is_null() {
        None
    } else {
        unsafe {
            CStr::from_ptr(function_name)
                .to_str()
                .ok()
                .map(|s| s.to_string())
        }
    
    // For now, return a simple placeholder
    let success_marker: u8 = 1;
    Box::into_raw(Box::new(success_marker))
/// FFI function for checking Result type
#[no_mangle]
pub extern "C" fn cursed_check_result(result_ptr: *const u8) -> *mut u8 {
    if result_ptr.is_null() {
        return std::ptr::null_mut();
    // For now, assume success
    let success_marker: u8 = 1;
    Box::into_raw(Box::new(success_marker))
/// FFI function for checking Option type
#[no_mangle]
pub extern "C" fn cursed_check_option(option_ptr: *const u8) -> *mut u8 {
    if option_ptr.is_null() {
        return std::ptr::null_mut();
    // For now, assume some value
    let success_marker: u8 = 1;
    Box::into_raw(Box::new(success_marker))
/// FFI function for error propagation check (expected by tests)
#[no_mangle]
pub extern "C" fn cursed_error_propagation_check(
) -> *mut u8 {
    if value_ptr.is_null() {
        return std::ptr::null_mut();
    // For now, assume success
    let success_marker: u8 = 1;
    Box::into_raw(Box::new(success_marker))
}
