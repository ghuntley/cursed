//! Error propagation system for CURSED runtime
//!
//! Implements error handling and propagation mechanisms, including
//! error chains, recovery strategies, and the `?` operator support.

use crate::error_types::{Error, Result as CursedResult};
use crate::runtime::stack_trace::{StackTrace, StackFrame};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

/// Error propagation context
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Original error
    pub error: Error,
    /// Stack trace when error occurred
    pub stack_trace: Option<StackTrace>,
    /// Error chain (nested errors)
    pub chain: Vec<Error>,
    /// Additional context data
    pub context_data: HashMap<String, String>,
    /// Recovery hints
    pub recovery_hints: Vec<RecoveryHint>,
    /// Error severity level
    pub severity: ErrorSeverity,
    /// Error timestamp
    pub timestamp: std::time::Instant,
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorSeverity {
    /// Informational error (warning)
    Info,
    /// Normal error (recoverable)
    Warning,
    /// Serious error (might be recoverable)
    Error,
    /// Critical error (likely unrecoverable)
    Critical,
    /// Fatal error (system failure)
    Fatal,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Info => write!(f, "INFO"),
            ErrorSeverity::Warning => write!(f, "WARNING"),
            ErrorSeverity::Error => write!(f, "ERROR"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
            ErrorSeverity::Fatal => write!(f, "FATAL"),
        }
    }
}

/// Recovery hint for error handling
#[derive(Debug, Clone)]
pub struct RecoveryHint {
    /// Description of the recovery strategy
    pub description: String,
    /// Recovery action type
    pub action: RecoveryAction,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
}

/// Types of recovery actions
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryAction {
    /// Retry the operation
    Retry,
    /// Use a default value
    UseDefault,
    /// Skip the operation
    Skip,
    /// Fallback to alternative implementation
    Fallback,
    /// Ask user for input
    UserInput,
    /// Terminate gracefully
    Terminate,
}

impl fmt::Display for RecoveryAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecoveryAction::Retry => write!(f, "retry"),
            RecoveryAction::UseDefault => write!(f, "use default"),
            RecoveryAction::Skip => write!(f, "skip"),
            RecoveryAction::Fallback => write!(f, "fallback"),
            RecoveryAction::UserInput => write!(f, "user input"),
            RecoveryAction::Terminate => write!(f, "terminate"),
        }
    }
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(error: Error) -> Self {
        Self {
            error,
            stack_trace: None,
            chain: Vec::new(),
            context_data: HashMap::new(),
            recovery_hints: Vec::new(),
            severity: ErrorSeverity::Error,
            timestamp: std::time::Instant::now(),
        }
    }

    /// Create error context with stack trace
    pub fn with_stack_trace(error: Error, stack_trace: StackTrace) -> Self {
        Self {
            error,
            stack_trace: Some(stack_trace),
            chain: Vec::new(),
            context_data: HashMap::new(),
            recovery_hints: Vec::new(),
            severity: ErrorSeverity::Error,
            timestamp: std::time::Instant::now(),
        }
    }

    /// Set error severity
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Add context data
    pub fn add_context(&mut self, key: String, value: String) {
        self.context_data.insert(key, value);
    }

    /// Add to error chain
    pub fn add_to_chain(&mut self, error: Error) {
        self.chain.push(error);
    }

    /// Add recovery hint
    pub fn add_recovery_hint(&mut self, hint: RecoveryHint) {
        self.recovery_hints.push(hint);
    }

    /// Get the root cause error
    pub fn root_cause(&self) -> &Error {
        self.chain.first().unwrap_or(&self.error)
    }

    /// Get error message with context
    pub fn full_message(&self) -> String {
        let mut message = format!("[{}] {}", self.severity, self.error);
        
        if !self.chain.is_empty() {
            message.push_str("\\nCaused by:");
            for (i, error) in self.chain.iter().enumerate() {
                message.push_str(&format!("\\n  {}: {}", i + 1, error));
            }
        }
        
        if !self.context_data.is_empty() {
            message.push_str("\\nContext:");
            for (key, value) in &self.context_data {
                message.push_str(&format!("\\n  {}: {}", key, value));
            }
        }
        
        if !self.recovery_hints.is_empty() {
            message.push_str("\\nRecovery suggestions:");
            for hint in &self.recovery_hints {
                message.push_str(&format!("\\n  - {} (confidence: {:.1}%)", 
                    hint.description, hint.confidence * 100.0));
            }
        }
        
        message
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(self.severity, ErrorSeverity::Info | ErrorSeverity::Warning | ErrorSeverity::Error)
    }

    /// Get best recovery hint
    pub fn best_recovery_hint(&self) -> Option<&RecoveryHint> {
        self.recovery_hints
            .iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_message())
    }
}

/// Error propagation operator implementation (for `?` operator)
pub struct ErrorPropagationOperator;

impl ErrorPropagationOperator {
    /// Propagate an error with additional context
    pub fn propagate<T>(
        result: CursedResult<T>,
        context: &str,
        file: &str,
        line: usize,
    ) -> CursedResult<T> {
        match result {
            Ok(value) => Ok(value),
            Err(error) => {
                let mut error_context = ErrorContext::new(error);
                error_context.add_context("propagation_context".to_string(), context.to_string());
                error_context.add_context("file".to_string(), file.to_string());
                error_context.add_context("line".to_string(), line.to_string());
                
                // Try to capture stack trace
                if let Ok(trace) = crate::runtime::stack_trace::get_global_stack_trace_collector().capture_trace() {
                    error_context.stack_trace = Some(trace);
                }
                
                Err(Error::Runtime(error_context.full_message()))
            }
        }
    }

    /// Chain errors together
    pub fn chain_error(original: Error, new_error: Error) -> Error {
        let mut context = ErrorContext::new(new_error);
        context.add_to_chain(original);
        Error::Runtime(context.full_message())
    }
}

/// Error recovery system
pub struct ErrorRecoverySystem {
    /// Recovery strategies by error type
    strategies: Mutex<HashMap<String, Vec<RecoveryStrategy>>>,
    /// Recovery statistics
    stats: Mutex<RecoveryStats>,
}

/// Recovery strategy definition
#[derive(Debug, Clone)]
pub struct RecoveryStrategy {
    /// Strategy name
    pub name: String,
    /// Recovery action
    pub action: RecoveryAction,
    /// Conditions when this strategy applies
    pub conditions: Vec<String>,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Strategy priority (higher = preferred)
    pub priority: i32,
}

/// Recovery statistics
#[derive(Debug, Default, Clone)]
pub struct RecoveryStats {
    pub recovery_attempts: usize,
    pub successful_recoveries: usize,
    pub failed_recoveries: usize,
    pub strategies_used: HashMap<String, usize>,
}

impl ErrorRecoverySystem {
    /// Create a new error recovery system
    pub fn new() -> Self {
        let mut system = Self {
            strategies: Mutex::new(HashMap::new()),
            stats: Mutex::new(RecoveryStats::default()),
        };
        
        // Add default recovery strategies
        system.add_default_strategies();
        system
    }

    /// Add default recovery strategies
    fn add_default_strategies(&mut self) {
        // Runtime error strategies
        self.register_strategy(
            "Runtime".to_string(),
            RecoveryStrategy {
                name: "retry_operation".to_string(),
                action: RecoveryAction::Retry,
                conditions: vec!["transient".to_string(), "timeout".to_string()],
                success_rate: 0.7,
                priority: 10,
            }
        );

        self.register_strategy(
            "Runtime".to_string(),
            RecoveryStrategy {
                name: "use_default_value".to_string(),
                action: RecoveryAction::UseDefault,
                conditions: vec!["missing_value".to_string(), "conversion_error".to_string()],
                success_rate: 0.9,
                priority: 5,
            }
        );

        // I/O error strategies
        self.register_strategy(
            "Io".to_string(),
            RecoveryStrategy {
                name: "retry_with_backoff".to_string(),
                action: RecoveryAction::Retry,
                conditions: vec!["network_error".to_string(), "file_locked".to_string()],
                success_rate: 0.8,
                priority: 15,
            }
        );

        // Memory error strategies
        self.register_strategy(
            "Memory".to_string(),
            RecoveryStrategy {
                name: "garbage_collect".to_string(),
                action: RecoveryAction::Retry,
                conditions: vec!["out_of_memory".to_string()],
                success_rate: 0.6,
                priority: 20,
            }
        );
    }

    /// Register a recovery strategy
    pub fn register_strategy(&mut self, error_type: String, strategy: RecoveryStrategy) {
        let mut strategies = self.strategies.lock().unwrap();
        strategies.entry(error_type).or_insert_with(Vec::new).push(strategy);
    }

    /// Attempt to recover from an error
    pub fn attempt_recovery(&self, error_context: &ErrorContext) -> CursedResult<RecoveryResult> {
        let error_type = match &error_context.error {
            Error::Runtime(_) => "Runtime",
            Error::Io(_) => "Io",
            Error::Memory(_) => "Memory",
            Error::Parse(_) => "Parse",
            Error::TypeCheck(_) => "TypeCheck",
            Error::Compile(_) => "Compile",
            Error::Import(_) => "Import",
            Error::Package(_) => "Package",
            Error::Template(_) => "Template",
            Error::Optimization(_) => "Optimization",
            Error::Debug(_) => "Debug",
            Error::InvalidOptimizationLevel(_) => "InvalidOptimizationLevel",
            Error::Type(_) => "Type",
            Error::Lexer(_) => "Lexer",
        };

        let strategies = self.strategies.lock().unwrap();
        let applicable_strategies = strategies.get(error_type).cloned().unwrap_or_default();

        // Find the best strategy
        let best_strategy = applicable_strategies
            .iter()
            .filter(|s| self.strategy_applies(s, error_context))
            .max_by_key(|s| s.priority);

        if let Some(strategy) = best_strategy {
            let mut stats = self.stats.lock().unwrap();
            stats.recovery_attempts += 1;
            *stats.strategies_used.entry(strategy.name.clone()).or_insert(0) += 1;

            // Simulate recovery attempt
            let success = rand::random::<f64>() < strategy.success_rate;
            
            if success {
                stats.successful_recoveries += 1;
                Ok(RecoveryResult::Success {
                    strategy_used: strategy.name.clone(),
                    action_taken: strategy.action.clone(),
                })
            } else {
                stats.failed_recoveries += 1;
                Ok(RecoveryResult::Failed {
                    strategy_attempted: strategy.name.clone(),
                    reason: "Recovery strategy failed".to_string(),
                })
            }
        } else {
            Ok(RecoveryResult::NoStrategy)
        }
    }

    /// Check if a strategy applies to an error context
    fn strategy_applies(&self, strategy: &RecoveryStrategy, context: &ErrorContext) -> bool {
        if strategy.conditions.is_empty() {
            return true;
        }

        // Check if any condition matches context data
        for condition in &strategy.conditions {
            if context.context_data.values().any(|v| v.contains(condition)) {
                return true;
            }
        }

        false
    }

    /// Get recovery statistics
    pub fn get_stats(&self) -> RecoveryStats {
        self.stats.lock().unwrap().clone()
    }

    /// Clear statistics
    pub fn clear_stats(&mut self) {
        let mut stats = self.stats.lock().unwrap();
        *stats = RecoveryStats::default();
    }
}

impl Default for ErrorRecoverySystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a recovery attempt
#[derive(Debug, Clone)]
pub enum RecoveryResult {
    /// Recovery was successful
    Success {
        strategy_used: String,
        action_taken: RecoveryAction,
    },
    /// Recovery failed
    Failed {
        strategy_attempted: String,
        reason: String,
    },
    /// No applicable recovery strategy found
    NoStrategy,
}

/// Error propagation manager
pub struct ErrorPropagationManager {
    /// Recovery system
    recovery_system: ErrorRecoverySystem,
    /// Error history for analysis
    error_history: Mutex<Vec<ErrorContext>>,
    /// Maximum history size
    max_history_size: usize,
}

impl ErrorPropagationManager {
    /// Create a new error propagation manager
    pub fn new() -> Self {
        Self {
            recovery_system: ErrorRecoverySystem::new(),
            error_history: Mutex::new(Vec::new()),
            max_history_size: 1000,
        }
    }

    /// Process an error and attempt recovery
    pub fn process_error(&self, mut error_context: ErrorContext) -> CursedResult<RecoveryResult> {
        // Add to history
        {
            let mut history = self.error_history.lock().unwrap();
            if history.len() >= self.max_history_size {
                history.remove(0); // Remove oldest
            }
            history.push(error_context.clone());
        }

        // Attempt recovery
        let recovery_result = self.recovery_system.attempt_recovery(&error_context)?;

        // Add recovery hints based on result
        match &recovery_result {
            RecoveryResult::Success { strategy_used, action_taken } => {
                error_context.add_recovery_hint(RecoveryHint {
                    description: format!("Successfully recovered using {} strategy", strategy_used),
                    action: action_taken.clone(),
                    confidence: 0.9,
                });
            }
            RecoveryResult::Failed { strategy_attempted, reason } => {
                error_context.add_recovery_hint(RecoveryHint {
                    description: format!("Failed to recover with {}: {}", strategy_attempted, reason),
                    action: RecoveryAction::Terminate,
                    confidence: 0.1,
                });
            }
            RecoveryResult::NoStrategy => {
                error_context.add_recovery_hint(RecoveryHint {
                    description: "No recovery strategy available".to_string(),
                    action: RecoveryAction::Terminate,
                    confidence: 0.0,
                });
            }
        }

        Ok(recovery_result)
    }

    /// Get error history
    pub fn get_error_history(&self) -> Vec<ErrorContext> {
        self.error_history.lock().unwrap().clone()
    }

    /// Clear error history
    pub fn clear_error_history(&self) {
        self.error_history.lock().unwrap().clear();
    }

    /// Get recovery statistics
    pub fn get_recovery_stats(&self) -> RecoveryStats {
        self.recovery_system.get_stats()
    }
}

impl Default for ErrorPropagationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global error propagation manager
static GLOBAL_ERROR_PROPAGATION_MANAGER: std::sync::LazyLock<ErrorPropagationManager> = 
    std::sync::LazyLock::new(|| ErrorPropagationManager::new());

/// Get the global error propagation manager
pub fn get_global_error_propagation_manager() -> &'static ErrorPropagationManager {
    &GLOBAL_ERROR_PROPAGATION_MANAGER
}

/// Utility functions for error propagation
pub mod utils {
    use super::*;

    /// Create an error context with common recovery hints
    pub fn create_error_context_with_hints(error: Error, error_type: &str) -> ErrorContext {
        let mut context = ErrorContext::new(error);
        
        // Add common recovery hints based on error type
        match error_type {
            "file_not_found" => {
                context.add_recovery_hint(RecoveryHint {
                    description: "Check if the file path is correct".to_string(),
                    action: RecoveryAction::UserInput,
                    confidence: 0.8,
                });
                context.add_recovery_hint(RecoveryHint {
                    description: "Use a default file if available".to_string(),
                    action: RecoveryAction::UseDefault,
                    confidence: 0.6,
                });
            }
            "network_error" => {
                context.add_recovery_hint(RecoveryHint {
                    description: "Retry the network operation".to_string(),
                    action: RecoveryAction::Retry,
                    confidence: 0.7,
                });
                context.add_recovery_hint(RecoveryHint {
                    description: "Use cached data if available".to_string(),
                    action: RecoveryAction::Fallback,
                    confidence: 0.5,
                });
            }
            "parse_error" => {
                context.add_recovery_hint(RecoveryHint {
                    description: "Skip malformed data".to_string(),
                    action: RecoveryAction::Skip,
                    confidence: 0.4,
                });
                context.add_recovery_hint(RecoveryHint {
                    description: "Use default values for missing fields".to_string(),
                    action: RecoveryAction::UseDefault,
                    confidence: 0.6,
                });
            }
            _ => {
                context.add_recovery_hint(RecoveryHint {
                    description: "Terminate with error".to_string(),
                    action: RecoveryAction::Terminate,
                    confidence: 0.1,
                });
            }
        }
        
        context
    }

    /// Propagate error with file and line information
    pub fn propagate_error_with_location<T>(
        result: CursedResult<T>,
        context: &str,
        file: &str,
        line: usize,
    ) -> CursedResult<T> {
        ErrorPropagationOperator::propagate(result, context, file, line)
    }
}

/// Macro for easy error propagation with context
#[macro_export]
macro_rules! propagate_error {
    ($result:expr, $context:expr) => {
        $crate::runtime::error_propagation::utils::propagate_error_with_location(
            $result, 
            $context, 
            file!(), 
            line!() as usize
        )
    };
}

/// Public function that was likely being used by external code
pub fn get_minimal_result() -> CursedResult<String> {
    Ok("CURSED error propagation system initialized".to_string())
}
