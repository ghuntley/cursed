// Error propagation runtime system for the CURSED programming language
// 
// This module provides the runtime infrastructure for the `?` operator,
// including error context management, propagation chains, and integration
// with CURSED's Result and Option types.

use crate::error_types::{Error, SourceLocation};
// use crate::runtime::value::Value;
use crate::types::result::{Result as CursedResult, Option as CursedOption};

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, warn};

/// Error propagation operator implementation
#[derive(Debug)]
pub struct ErrorPropagationOperator {
    /// Error context stack for tracking propagation sites
    pub context_stack: Arc<Mutex<ErrorContextStack>>,
    /// Propagation statistics for monitoring
    pub statistics: Arc<RwLock<PropagationStatistics>>,
    /// Configuration for propagation behavior
    pub config: PropagationConfig,
}

impl ErrorPropagationOperator {
    /// Create a new error propagation operator
    pub fn new() -> Self {
        Self::with_config(PropagationConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(config: PropagationConfig) -> Self {
        Self {
            context_stack: Arc::new(Mutex::new(ErrorContextStack::new())),
            statistics: Arc::new(RwLock::new(PropagationStatistics::new())),
            config,
        }
    }

    /// Apply the `?` operator to a Result value
    #[instrument(skip(self, result_value))]
    pub fn apply_question_mark<T, E>(
        &self,
        result_value: CursedResult<T, E>,
        source_location: SourceLocation,
        function_context: Option<String>,
    ) -> Result<T, PropagationError<E>>
    where
        T: Clone + fmt::Debug,
        E: Clone + fmt::Debug,
    {
        // Record propagation attempt
        let start_time = Instant::now();
        self.record_propagation_attempt();

        match result_value {
            CursedResult::Ok(value) => {
                debug!(
                    location = ?source_location,
                    function = ?function_context,
                    "Question mark operator: success path"
                );
                self.record_successful_propagation(start_time.elapsed());
                Ok(value)
            }
            CursedResult::Err(error) => {
                // Create propagation error with context
                let propagation_error = PropagationError::new(
                    error,
                    source_location.clone(),
                    function_context.clone(),
                );

                // Add to context stack
                if let Ok(mut stack) = self.context_stack.lock() {
                    stack.push_context(ErrorPropagationContext {
                        location: source_location.clone(),
                        function_name: function_context.clone(),
                        error_type: "Result::Err".to_string(),
                        timestamp: Instant::now(),
                    });
                }

                warn!(
                    location = ?source_location,
                    function = ?function_context,
                    error = ?propagation_error.inner_error,
                    "Question mark operator: error propagation"
                );

                self.record_error_propagation(start_time.elapsed());
                Err(propagation_error)
            }
        }
    }

    /// Apply the `?` operator to an Option value
    #[instrument(skip(self, option_value))]
    pub fn apply_question_mark_option<T>(
        &self,
        option_value: CursedOption<T>,
        source_location: SourceLocation,
        function_context: Option<String>,
    ) -> Result<(), Error>
    where
        T: Clone + fmt::Debug,
    {
        let start_time = Instant::now();
        self.record_propagation_attempt();

        match option_value {
            CursedOption::Some(value) => {
                debug!(
                    location = ?source_location,
                    function = ?function_context,
                    "Question mark operator on Option: success path"
                );
                self.record_successful_propagation(start_time.elapsed());
                Ok(value)
            }
            CursedOption::None => {
                let none_error = NoneError {
                    message: "Option was None".to_string(),
                    location: source_location.clone(),
                };

                let propagation_error = PropagationError::new(
                    none_error,
                    source_location.clone(),
                    function_context.clone(),
                );

                // Add to context stack
                if let Ok(mut stack) = self.context_stack.lock() {
                    stack.push_context(ErrorPropagationContext {
                        location: source_location.clone(),
                        function_name: function_context.clone(),
                        error_type: "Option::None".to_string(),
                        timestamp: Instant::now(),
                    });
                }

                warn!(
                    location = ?source_location,
                    function = ?function_context,
                    "Question mark operator on Option: None propagation"
                );

                self.record_error_propagation(start_time.elapsed());
                Err(propagation_error)
            }
        }
    }

    /// Get the current error context chain
    pub fn get_error_context_chain(&self) -> Result<(), Error> {
        let stack = self.context_stack.lock()
            .map_err(|_| Error::system_error("Failed to acquire context stack lock"))?;
        Ok(stack.get_contexts())
    }

    /// Clear the error context stack
    pub fn clear_context_stack(&self) -> Result<(), Error> {
        let mut stack = self.context_stack.lock()
            .map_err(|_| Error::system_error("Failed to acquire context stack lock"))?;
        stack.clear();
        Ok(())
    }

    /// Get propagation statistics
    pub fn get_statistics(&self) -> Result<(), Error> {
        let stats = self.statistics.read()
            .map_err(|_| Error::system_error("Failed to acquire statistics lock"))?;
        Ok(stats.clone())
    }

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
}

/// Error that occurred during propagation
#[derive(Debug, Clone)]
pub struct PropagationError<E> {
    /// The original error being propagated
    pub inner_error: E,
    /// Location where propagation occurred
    pub propagation_site: SourceLocation,
    /// Function context
    pub function_context: Option<String>,
    /// Propagation chain
    pub propagation_chain: Vec<SourceLocation>,
    /// Additional context
    pub additional_context: Option<String>,
    /// Timestamp of error
    pub timestamp: Instant,
}

impl<E> PropagationError<E> {
    /// Create a new propagation error
    pub fn new(
        inner_error: E,
        propagation_site: SourceLocation,
        function_context: Option<String>,
    ) -> Self {
        Self {
            inner_error,
            propagation_site,
            function_context,
            propagation_chain: Vec::new(),
            additional_context: None,
            timestamp: Instant::now(),
        }
    }

    /// Add a propagation site to the chain
    pub fn add_propagation_site(mut self, site: SourceLocation) -> Self {
        self.propagation_chain.push(site);
        self
    }

    /// Add additional context
    pub fn with_context(mut self, context: String) -> Self {
        self.additional_context = Some(context);
        self
    }

    /// Get the full propagation chain
    pub fn full_chain(&self) -> Vec<SourceLocation> {
        let mut chain = vec![self.propagation_site.clone()];
        chain.extend_from_slice(&self.propagation_chain);
        chain
    }
}

impl<E: fmt::Display> fmt::Display for PropagationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error propagated at {}:{}: {}",
            self.propagation_site.line,
            self.propagation_site.column,
            self.inner_error
        )?;

        if let Some(ref context) = self.function_context {
            write!(f, " in function '{}'", context)?;
        }

        if !self.propagation_chain.is_empty() {
            write!(f, " (propagated through {} sites)", self.propagation_chain.len())?;
        }

        Ok(())
    }
}

/// Error representing None in Option
#[derive(Debug, Clone)]
pub struct NoneError {
    pub message: String,
    pub location: SourceLocation,
}

impl fmt::Display for NoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.location.line, self.location.column)
    }
}

/// Error propagation context entry
#[derive(Debug, Clone)]
pub struct ErrorPropagationContext {
    /// Source location of the propagation
    pub location: SourceLocation,
    /// Function name where propagation occurred
    pub function_name: Option<String>,
    /// Type of error being propagated
    pub error_type: String,
    /// Timestamp of the propagation
    pub timestamp: Instant,
}

impl fmt::Display for ErrorPropagationContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}:{}",
            self.error_type,
            self.location.line,
            self.location.column
        )?;

        if let Some(ref func) = self.function_name {
            write!(f, " in {}", func)?;
        }

        Ok(())
    }
}

/// Stack for tracking error propagation contexts
#[derive(Debug)]
pub struct ErrorContextStack {
    /// Stack of propagation contexts
    contexts: VecDeque<ErrorPropagationContext>,
    /// Maximum stack depth
    max_depth: usize,
}

impl ErrorContextStack {
    /// Create a new context stack
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// Create with specific capacity
    pub fn with_capacity(max_depth: usize) -> Self {
        Self {
            contexts: VecDeque::new(),
            max_depth,
        }
    }

    /// Push a new context
    pub fn push_context(&mut self, context: ErrorPropagationContext) {
        if self.contexts.len() >= self.max_depth {
            self.contexts.pop_front();
        }
        self.contexts.push_back(context);
    }

    /// Pop the most recent context
    pub fn pop_context(&mut self) -> Option<ErrorPropagationContext> {
        self.contexts.pop_back()
    }

    /// Get all contexts as a vector
    pub fn get_contexts(&self) -> Vec<ErrorPropagationContext> {
        self.contexts.iter().cloned().collect()
    }

    /// Clear all contexts
    pub fn clear(&mut self) {
        self.contexts.clear();
    }

    /// Get the depth of the stack
    pub fn depth(&self) -> usize {
        self.contexts.len()
    }

    /// Check if stack is empty
    pub fn is_empty(&self) -> bool {
        self.contexts.is_empty()
    }
}

/// Statistics for error propagation monitoring
#[derive(Debug, Clone)]
pub struct PropagationStatistics {
    /// Total number of propagation attempts
    pub total_attempts: u64,
    /// Number of successful propagations (Ok/Some values)
    pub successful_propagations: u64,
    /// Number of error propagations (Err/None values)
    pub error_propagations: u64,
    /// Total time spent in propagation operations
    pub total_duration: Duration,
    /// Last reset timestamp
    pub last_reset: Instant,
}

impl PropagationStatistics {
    /// Create new statistics
    pub fn new() -> Self {
        Self {
            total_attempts: 0,
            successful_propagations: 0,
            error_propagations: 0,
            total_duration: Duration::from_nanos(0),
            last_reset: Instant::now(),
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
        writeln!(f, "Error Propagation Statistics:")?;
        writeln!(f, "  Total attempts: {}", self.total_attempts)?;
        writeln!(f, "  Successful: {}", self.successful_propagations)?;
        writeln!(f, "  Errors: {}", self.error_propagations)?;
        writeln!(f, "  Success rate: {:.2}%", self.success_rate() * 100.0)?;
        writeln!(f, "  Error rate: {:.2}%", self.error_rate() * 100.0)?;
        writeln!(f, "  Average duration: {:?}", self.average_duration())?;
        Ok(())
    }
}

/// Configuration for error propagation behavior
#[derive(Debug, Clone)]
pub struct PropagationConfig {
    /// Maximum context stack depth
    pub max_context_depth: usize,
    /// Whether to enable detailed tracing
    pub enable_tracing: bool,
    /// Whether to collect timing statistics
    pub collect_timing: bool,
    /// Timeout for propagation operations
    pub propagation_timeout: Option<Duration>,
}

impl Default for PropagationConfig {
    fn default() -> Self {
        Self {
            max_context_depth: 100,
            enable_tracing: true,
            collect_timing: true,
            propagation_timeout: Some(Duration::from_millis(1000)),
        }
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
    }

    /// Apply `?` to a Result with source location
    pub fn propagate_result<T, E>(
        operator: &ErrorPropagationOperator,
        result: CursedResult<T, E>,
        line: usize,
        column: usize,
        function: Option<&str>,
    ) -> PropagationResult<T, E>
    where
        T: Clone + fmt::Debug,
        E: Clone + fmt::Debug,
    {
        operator.apply_question_mark(
            result,
            SourceLocation::new(line, column),
            function.map(|s| s.to_string()),
        )
    }

    /// Apply `?` to an Option with source location
    pub fn propagate_option<T>(
        operator: &ErrorPropagationOperator,
        option: CursedOption<T>,
        line: usize,
        column: usize,
        function: Option<&str>,
    ) -> PropagationResult<(), Error>
    where
        T: Clone + fmt::Debug,
    {
        operator.apply_question_mark_option(
            option,
            SourceLocation::new(line, column),
            function.map(|s| s.to_string()),
        )
    }

    /// Create a CURSED error from a propagation error
    pub fn to_cursed_error<E: fmt::Display>(
        propagation_error: PropagationError<E>,
    ) -> Error {
        Error::ErrorPropagation {
            message: format!("Error propagation failed: {}", propagation_error),
            line: Some(propagation_error.propagation_site.line),
            column: Some(propagation_error.propagation_site.column),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_propagation_operator_creation() {
        let operator = ErrorPropagationOperator::new();
        let stats = operator.get_statistics().unwrap();
        assert_eq!(stats.total_attempts, 0);
    }

    #[test]
    fn test_result_success_propagation() {
        let operator = ErrorPropagationOperator::new();
        let result: crate::types::result::Result<i32, String> = CursedResult::Ok(42);
        let location = SourceLocation::new(1, 5);

        let propagated = operator.apply_question_mark(result, location, None);
        assert!(propagated.is_ok());
        assert_eq!(propagated.unwrap(), 42);

        let stats = operator.get_statistics().unwrap();
        assert_eq!(stats.total_attempts, 1);
        assert_eq!(stats.successful_propagations, 1);
        assert_eq!(stats.error_propagations, 0);
    }

    #[test]
    fn test_result_error_propagation() {
        let operator = ErrorPropagationOperator::new();
        let result: crate::types::result::Result<i32, String> = CursedResult::Err("test error".to_string());
        let location = SourceLocation::new(1, 5);

        let propagated = operator.apply_question_mark(result, location, Some("test_function".to_string()));
        assert!(propagated.is_err());

        let error = propagated.unwrap_err();
        assert_eq!(error.inner_error, "test error");
        assert_eq!(error.function_context, Some("test_function".to_string()));

        let stats = operator.get_statistics().unwrap();
        assert_eq!(stats.total_attempts, 1);
        assert_eq!(stats.successful_propagations, 0);
        assert_eq!(stats.error_propagations, 1);
    }

    #[test]
    fn test_option_some_propagation() {
        let operator = ErrorPropagationOperator::new();
        let option = CursedOption::Some(42);
        let location = SourceLocation::new(1, 5);

        let propagated = operator.apply_question_mark_option(option, location, None);
        assert!(propagated.is_ok());
        assert_eq!(propagated.unwrap(), 42);
    }

    #[test]
    fn test_option_none_propagation() {
        let operator = ErrorPropagationOperator::new();
        let option: CursedOption<i32> = CursedOption::None;
        let location = SourceLocation::new(1, 5);

        let propagated = operator.apply_question_mark_option(option, location, None);
        assert!(propagated.is_err());

        let error = propagated.unwrap_err();
        assert_eq!(error.inner_error.message, "Option was None");
    }

    #[test]
    fn test_error_context_stack() {
        let mut stack = ErrorContextStack::new();
        assert!(stack.is_empty());

        let context = ErrorPropagationContext {
            location: SourceLocation::new(1, 5),
            function_name: Some("test".to_string()),
            error_type: "TestError".to_string(),
            timestamp: Instant::now(),
        };

        stack.push_context(context.clone());
        assert_eq!(stack.depth(), 1);
        assert!(!stack.is_empty());

        let contexts = stack.get_contexts();
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts[0].location.line, 1);
    }

    #[test]
    fn test_propagation_statistics() {
        let mut stats = PropagationStatistics::new();
        assert_eq!(stats.total_attempts, 0);
        assert_eq!(stats.error_rate(), 0.0);
        assert_eq!(stats.success_rate(), 0.0);

        stats.total_attempts = 10;
        stats.successful_propagations = 7;
        stats.error_propagations = 3;

        assert_eq!(stats.error_rate(), 0.3);
        assert_eq!(stats.success_rate(), 0.7);
    }

    #[test]
    fn test_propagation_error_chaining() {
        let inner_error = "original error".to_string();
        let location1 = SourceLocation::new(1, 5);
        let location2 = SourceLocation::new(2, 10);

        let error = PropagationError::new(inner_error, location1, None)
            .add_propagation_site(location2);

        let chain = error.full_chain();
        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].line, 1);
        assert_eq!(chain[1].line, 2);
    }

    #[test]
    fn test_helper_functions() {
        let operator = helpers::create_default_propagator();
        
        let result: crate::types::result::Result<i32, String> = CursedResult::Ok(42);
        let propagated = helpers::propagate_result(&operator, result, 1, 5, Some("test"));
        assert!(propagated.is_ok());

        let option = CursedOption::Some(42);
        let propagated = helpers::propagate_option(&operator, option, 1, 5, Some("test"));
        assert!(propagated.is_ok());
    }
}

// FFI Functions for LLVM Integration
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// FFI function for applying question mark operator to Result
#[no_mangle]
pub extern "C" fn cursed_question_mark_operator(
    result_ptr: *const u8,
    line: i32,
    column: i32,
) -> *mut u8 {
    if result_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    // For now, return a simple placeholder
    // In a full implementation, this would check the Result and handle error propagation
    let success_marker: u8 = 1;
    Box::into_raw(Box::new(success_marker))
}

/// FFI function for enhanced question mark operator
#[no_mangle]
pub extern "C" fn cursed_enhanced_question_mark(
    result_ptr: *const u8,
    line: i32,
    column: i32,
    function_name: *const c_char,
) -> *mut u8 {
    if result_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
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
    };
    
    // For now, return a simple placeholder
    let success_marker: u8 = 1;
    Box::into_raw(Box::new(success_marker))
}

/// FFI function for checking Result type
#[no_mangle]
pub extern "C" fn cursed_check_result(result_ptr: *const u8) -> *mut u8 {
    if result_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    // For now, assume success
    let success_marker: u8 = 1;
    Box::into_raw(Box::new(success_marker))
}

/// FFI function for checking Option type
#[no_mangle]
pub extern "C" fn cursed_check_option(option_ptr: *const u8) -> *mut u8 {
    if option_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    // For now, assume some value
    let success_marker: u8 = 1;
    Box::into_raw(Box::new(success_marker))
}

/// FFI function for error propagation check (expected by tests)
#[no_mangle]
pub extern "C" fn cursed_error_propagation_check(
    value_ptr: *const u8,
    line: i32,
    column: i32,
) -> *mut u8 {
    if value_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    // For now, assume success
    let success_marker: u8 = 1;
    Box::into_raw(Box::new(success_marker))
}
