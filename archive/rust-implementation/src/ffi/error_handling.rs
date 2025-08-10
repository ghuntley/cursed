//! Error handling for FFI operations
//!
//! This module provides comprehensive error handling for FFI operations
//! including error recovery, logging, and debugging support.

use std::fmt;
use crate::error::CursedError;

/// FFI-specific error types
#[derive(Debug, Clone)]
pub enum FfiError {
    /// Type conversion error
    TypeConversion {
        from_type: String,
        to_type: String,
        reason: String,
    },
    
    /// Memory allocation error
    MemoryAllocation {
        size: usize,
        reason: String,
    },
    
    /// Invalid pointer error
    InvalidPointer {
        address: usize,
        reason: String,
    },
    
    /// Function call error
    FunctionCall {
        function_name: String,
        error_code: i32,
        message: String,
    },
    
    /// Library loading error
    LibraryLoad {
        library_path: String,
        error: String,
    },
    
    /// Symbol resolution error
    SymbolNotFound {
        symbol_name: String,
        library: String,
    },
    
    /// Marshalling error
    Marshalling {
        operation: String,
        reason: String,
    },
    
    /// Callback error
    Callback {
        callback_id: usize,
        error: String,
    },
    
    /// Timeout error
    Timeout {
        operation: String,
        timeout_ms: u64,
    },
    
    /// Safety violation error
    SafetyViolation {
        violation_type: String,
        details: String,
    },
}

impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiError::TypeConversion { from_type, to_type, reason } => {
                write!(f, "Type conversion error from {} to {}: {}", from_type, to_type, reason)
            }
            FfiError::MemoryAllocation { size, reason } => {
                write!(f, "Memory allocation error for {} bytes: {}", size, reason)
            }
            FfiError::InvalidPointer { address, reason } => {
                write!(f, "Invalid pointer at address 0x{:x}: {}", address, reason)
            }
            FfiError::FunctionCall { function_name, error_code, message } => {
                write!(f, "Function call error in {}: {} (code: {})", function_name, message, error_code)
            }
            FfiError::LibraryLoad { library_path, error } => {
                write!(f, "Library load error for {}: {}", library_path, error)
            }
            FfiError::SymbolNotFound { symbol_name, library } => {
                write!(f, "Symbol {} not found in library {}", symbol_name, library)
            }
            FfiError::Marshalling { operation, reason } => {
                write!(f, "Marshalling error in {}: {}", operation, reason)
            }
            FfiError::Callback { callback_id, error } => {
                write!(f, "Callback error in callback {}: {}", callback_id, error)
            }
            FfiError::Timeout { operation, timeout_ms } => {
                write!(f, "Timeout in {} after {} ms", operation, timeout_ms)
            }
            FfiError::SafetyViolation { violation_type, details } => {
                write!(f, "Safety violation ({}): {}", violation_type, details)
            }
        }
    }
}

impl std::error::Error for FfiError {}

impl From<FfiError> for CursedError {
    fn from(error: FfiError) -> Self {
        CursedError::General(error.to_string())
    }
}

/// Error recovery strategies
#[derive(Debug, Clone)]
pub enum ErrorRecoveryStrategy {
    /// Retry the operation
    Retry { max_attempts: u32, delay_ms: u64 },
    
    /// Fallback to alternative implementation
    Fallback { alternative: String },
    
    /// Return default value
    ReturnDefault,
    
    /// Propagate error up the call stack
    Propagate,
    
    /// Abort the operation
    Abort,
}

/// Error recovery manager
pub struct ErrorRecoveryManager {
    /// Recovery strategies for different error types
    strategies: std::collections::HashMap<String, ErrorRecoveryStrategy>,
    
    /// Error history for learning
    error_history: Vec<ErrorRecord>,
    
    /// Recovery statistics
    recovery_stats: RecoveryStats,
}

/// Error record for learning and analysis
#[derive(Debug, Clone)]
struct ErrorRecord {
    /// Error type
    error_type: String,
    
    /// Error message
    message: String,
    
    /// Timestamp
    timestamp: std::time::Instant,
    
    /// Recovery strategy used
    recovery_strategy: ErrorRecoveryStrategy,
    
    /// Recovery success
    recovery_success: bool,
}

/// Recovery statistics
#[derive(Debug, Clone)]
struct RecoveryStats {
    /// Total errors encountered
    total_errors: u64,
    
    /// Total recovery attempts
    total_recovery_attempts: u64,
    
    /// Successful recoveries
    successful_recoveries: u64,
    
    /// Recovery success rate
    success_rate: f64,
}

impl ErrorRecoveryManager {
    /// Create new error recovery manager
    pub fn new() -> Self {
        let mut manager = Self {
            strategies: std::collections::HashMap::new(),
            error_history: Vec::new(),
            recovery_stats: RecoveryStats {
                total_errors: 0,
                total_recovery_attempts: 0,
                successful_recoveries: 0,
                success_rate: 0.0,
            },
        };
        
        manager.register_default_strategies();
        manager
    }
    
    /// Register default recovery strategies
    fn register_default_strategies(&mut self) {
        // Memory allocation errors - retry with exponential backoff
        self.strategies.insert(
            "MemoryAllocation".to_string(),
            ErrorRecoveryStrategy::Retry {
                max_attempts: 3,
                delay_ms: 100,
            },
        );
        
        // Function call errors - retry once
        self.strategies.insert(
            "FunctionCall".to_string(),
            ErrorRecoveryStrategy::Retry {
                max_attempts: 1,
                delay_ms: 50,
            },
        );
        
        // Type conversion errors - return default
        self.strategies.insert(
            "TypeConversion".to_string(),
            ErrorRecoveryStrategy::ReturnDefault,
        );
        
        // Library load errors - propagate
        self.strategies.insert(
            "LibraryLoad".to_string(),
            ErrorRecoveryStrategy::Propagate,
        );
        
        // Safety violations - abort
        self.strategies.insert(
            "SafetyViolation".to_string(),
            ErrorRecoveryStrategy::Abort,
        );
    }
    
    /// Register recovery strategy for error type
    pub fn register_strategy(&mut self, error_type: &str, strategy: ErrorRecoveryStrategy) {
        self.strategies.insert(error_type.to_string(), strategy);
    }
    
    /// Handle error with recovery
    pub fn handle_error(&mut self, error: &FfiError) -> Result<ErrorRecoveryAction, CursedError> {
        let error_type = self.get_error_type(error);
        
        self.recovery_stats.total_errors += 1;
        
        let strategy = self.strategies.get(&error_type)
            .cloned()
            .unwrap_or(ErrorRecoveryStrategy::Propagate);
        
        let recovery_action = match strategy {
            ErrorRecoveryStrategy::Retry { max_attempts, delay_ms } => {
                ErrorRecoveryAction::Retry {
                    max_attempts,
                    delay_ms,
                    current_attempt: 0,
                }
            }
            ErrorRecoveryStrategy::Fallback { alternative } => {
                ErrorRecoveryAction::Fallback { alternative }
            }
            ErrorRecoveryStrategy::ReturnDefault => {
                ErrorRecoveryAction::ReturnDefault
            }
            ErrorRecoveryStrategy::Propagate => {
                ErrorRecoveryAction::Propagate
            }
            ErrorRecoveryStrategy::Abort => {
                ErrorRecoveryAction::Abort
            }
        };
        
        // Record error for learning
        let error_record = ErrorRecord {
            error_type: error_type.clone(),
            message: error.to_string(),
            timestamp: std::time::Instant::now(),
            recovery_strategy: strategy,
            recovery_success: false, // Will be updated later
        };
        
        self.error_history.push(error_record);
        
        Ok(recovery_action)
    }
    
    /// Report recovery success/failure
    pub fn report_recovery_result(&mut self, success: bool) {
        self.recovery_stats.total_recovery_attempts += 1;
        
        if success {
            self.recovery_stats.successful_recoveries += 1;
        }
        
        // Update success rate
        self.recovery_stats.success_rate = 
            self.recovery_stats.successful_recoveries as f64 / 
            self.recovery_stats.total_recovery_attempts as f64;
        
        // Update last error record
        if let Some(last_record) = self.error_history.last_mut() {
            last_record.recovery_success = success;
        }
    }
    
    /// Get error type string
    fn get_error_type(&self, error: &FfiError) -> String {
        match error {
            FfiError::TypeConversion { .. } => "TypeConversion".to_string(),
            FfiError::MemoryAllocation { .. } => "MemoryAllocation".to_string(),
            FfiError::InvalidPointer { .. } => "InvalidPointer".to_string(),
            FfiError::FunctionCall { .. } => "FunctionCall".to_string(),
            FfiError::LibraryLoad { .. } => "LibraryLoad".to_string(),
            FfiError::SymbolNotFound { .. } => "SymbolNotFound".to_string(),
            FfiError::Marshalling { .. } => "Marshalling".to_string(),
            FfiError::Callback { .. } => "Callback".to_string(),
            FfiError::Timeout { .. } => "Timeout".to_string(),
            FfiError::SafetyViolation { .. } => "SafetyViolation".to_string(),
        }
    }
    
    /// Get recovery statistics
    pub fn get_stats(&self) -> &RecoveryStats {
        &self.recovery_stats
    }
    
    /// Get error history
    pub fn get_error_history(&self) -> &[ErrorRecord] {
        &self.error_history
    }
    
    /// Clear error history
    pub fn clear_history(&mut self) {
        self.error_history.clear();
    }
}

/// Error recovery action
#[derive(Debug, Clone)]
pub enum ErrorRecoveryAction {
    /// Retry the operation
    Retry {
        max_attempts: u32,
        delay_ms: u64,
        current_attempt: u32,
    },
    
    /// Use fallback implementation
    Fallback {
        alternative: String,
    },
    
    /// Return default value
    ReturnDefault,
    
    /// Propagate error
    Propagate,
    
    /// Abort operation
    Abort,
}

/// Error logger for FFI operations
pub struct FfiErrorLogger {
    /// Log entries
    log_entries: Vec<LogEntry>,
    
    /// Log configuration
    config: LogConfig,
}

/// Log entry
#[derive(Debug, Clone)]
struct LogEntry {
    /// Timestamp
    timestamp: std::time::SystemTime,
    
    /// Log level
    level: LogLevel,
    
    /// Error message
    message: String,
    
    /// Error details
    details: Option<String>,
    
    /// Stack trace
    stack_trace: Option<Vec<String>>,
}

/// Log level
#[derive(Debug, Clone)]
enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

/// Log configuration
#[derive(Debug, Clone)]
struct LogConfig {
    /// Enable logging
    enabled: bool,
    
    /// Log level threshold
    level: LogLevel,
    
    /// Log to file
    log_to_file: bool,
    
    /// Log file path
    log_file_path: Option<String>,
    
    /// Maximum log entries
    max_entries: usize,
}

impl FfiErrorLogger {
    /// Create new error logger
    pub fn new() -> Self {
        Self {
            log_entries: Vec::new(),
            config: LogConfig {
                enabled: true,
                level: LogLevel::Error,
                log_to_file: false,
                log_file_path: None,
                max_entries: 10000,
            },
        }
    }
    
    /// Log an error
    pub fn log_error(&mut self, error: &FfiError, details: Option<String>) {
        if !self.config.enabled {
            return;
        }
        
        let entry = LogEntry {
            timestamp: std::time::SystemTime::now(),
            level: LogLevel::Error,
            message: error.to_string(),
            details,
            stack_trace: None, // Could capture stack trace here
        };
        
        self.log_entries.push(entry);
        
        // Limit log entries
        if self.log_entries.len() > self.config.max_entries {
            self.log_entries.remove(0);
        }
        
        // Write to file if configured
        if self.config.log_to_file {
            if let Some(ref path) = self.config.log_file_path {
                let _ = self.write_to_file(path, &self.log_entries.last().unwrap());
            }
        }
    }
    
    /// Write log entry to file
    fn write_to_file(&self, path: &str, entry: &LogEntry) -> Result<(), std::io::Error> {
        use std::io::Write;
        
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        writeln!(file, "{:?}: {}", entry.timestamp, entry.message)?;
        
        if let Some(ref details) = entry.details {
            writeln!(file, "Details: {}", details)?;
        }
        
        Ok(())
    }
    
    /// Get log entries
    pub fn get_entries(&self) -> &[LogEntry] {
        &self.log_entries
    }
    
    /// Clear log entries
    pub fn clear(&mut self) {
        self.log_entries.clear();
    }
    
    /// Enable file logging
    pub fn enable_file_logging(&mut self, path: &str) {
        self.config.log_to_file = true;
        self.config.log_file_path = Some(path.to_string());
    }
    
    /// Disable file logging
    pub fn disable_file_logging(&mut self) {
        self.config.log_to_file = false;
        self.config.log_file_path = None;
    }
}

impl Default for ErrorRecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FfiErrorLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ffi_error_display() {
        let error = FfiError::TypeConversion {
            from_type: "int".to_string(),
            to_type: "string".to_string(),
            reason: "Invalid conversion".to_string(),
        };
        
        let display = format!("{}", error);
        assert!(display.contains("Type conversion error"));
        assert!(display.contains("int"));
        assert!(display.contains("string"));
    }
    
    #[test]
    fn test_error_recovery_manager() {
        let mut manager = ErrorRecoveryManager::new();
        
        let error = FfiError::MemoryAllocation {
            size: 1024,
            reason: "Out of memory".to_string(),
        };
        
        let action = manager.handle_error(&error).unwrap();
        
        match action {
            ErrorRecoveryAction::Retry { max_attempts, .. } => {
                assert_eq!(max_attempts, 3);
            }
            _ => panic!("Expected retry action"),
        }
        
        assert_eq!(manager.get_stats().total_errors, 1);
    }
    
    #[test]
    fn test_error_logger() {
        let mut logger = FfiErrorLogger::new();
        
        let error = FfiError::FunctionCall {
            function_name: "test_function".to_string(),
            error_code: -1,
            message: "Function failed".to_string(),
        };
        
        logger.log_error(&error, Some("Additional details".to_string()));
        
        assert_eq!(logger.get_entries().len(), 1);
        assert!(logger.get_entries()[0].message.contains("Function failed"));
    }
    
    #[test]
    fn test_recovery_strategies() {
        let mut manager = ErrorRecoveryManager::new();
        
        // Test custom strategy registration
        manager.register_strategy(
            "CustomError",
            ErrorRecoveryStrategy::Fallback {
                alternative: "alternative_impl".to_string(),
            },
        );
        
        assert!(manager.strategies.contains_key("CustomError"));
    }
    
    #[test]
    fn test_recovery_stats() {
        let mut manager = ErrorRecoveryManager::new();
        
        let error = FfiError::TypeConversion {
            from_type: "int".to_string(),
            to_type: "string".to_string(),
            reason: "Test error".to_string(),
        };
        
        manager.handle_error(&error).unwrap();
        manager.report_recovery_result(true);
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_errors, 1);
        assert_eq!(stats.total_recovery_attempts, 1);
        assert_eq!(stats.successful_recoveries, 1);
        assert_eq!(stats.success_rate, 1.0);
    }
}
