//! CURSED Error Handling Runtime System
//!
//! This module provides comprehensive error handling for the CURSED runtime:
//! - Error detection, classification, and propagation
//! - Recovery mechanisms and graceful degradation
//! - Integration with panic runtime and goroutine scheduler
//! - Error statistics and performance monitoring
//! - Contextual error information and debugging support

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::thread::ThreadId;
use std::fmt;

use crate::error_types::{Error, Result};
use crate::runtime::{RuntimeError, RuntimeErrorType};
use crate::runtime::goroutine::GoroutineId;

/// Global error runtime instance
static GLOBAL_ERROR_RUNTIME: once_cell::sync::OnceCell<Arc<ErrorRuntime>> = once_cell::sync::OnceCell::new();

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorSeverity {
    /// Informational errors
    Info = 0,
    /// Warning errors that should be noted
    Warning = 1,
    /// Non-critical errors that can be recovered
    Error = 2,
    /// Critical errors requiring immediate attention
    Critical = 3,
    /// Fatal errors that may cause system instability
    Fatal = 4,
}

/// Error categories for classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Memory-related errors
    Memory,
    /// I/O and file system errors
    IO,
    /// Network and communication errors
    Network,
    /// Parsing and syntax errors
    Parsing,
    /// Type system and validation errors
    Type,
    /// Runtime and execution errors
    Runtime,
    /// Security and permission errors
    Security,
    /// Performance and resource errors
    Performance,
    /// Unknown or uncategorized errors
    Unknown,
}

/// Error recovery action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryAction {
    /// Continue execution without action
    Continue,
    /// Retry the operation
    Retry,
    /// Skip the current operation
    Skip,
    /// Use fallback/default value
    UseFallback,
    /// Restart the current goroutine
    RestartGoroutine,
    /// Escalate to panic handler
    EscalateToPanic,
    /// Request graceful shutdown
    GracefulShutdown,
}

/// Comprehensive error context
#[derive(Debug)]
pub struct ErrorContext {
    /// Original error
    pub error: Error,
    /// Error severity level
    pub severity: ErrorSeverity,
    /// Error category
    pub category: ErrorCategory,
    /// Location where error occurred
    pub location: Option<String>,
    /// Goroutine ID where error occurred
    pub goroutine_id: Option<GoroutineId>,
    /// Thread ID where error occurred
    pub thread_id: Option<ThreadId>,
    /// Stack trace at error
    pub stack_trace: Vec<String>,
    /// Timestamp of error
    pub timestamp: Instant,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
    /// Recovery attempts made
    pub recovery_attempts: u32,
    /// Maximum recovery attempts allowed
    pub max_recovery_attempts: u32,
    /// Suggested recovery action
    pub suggested_recovery: RecoveryAction,
}

impl Clone for ErrorContext {
    fn clone(&self) -> Self {
        Self {
            error: self.error.clone(),
            severity: self.severity,
            category: self.category,
            location: self.location.clone(),
            goroutine_id: self.goroutine_id,
            thread_id: self.thread_id, // ThreadId is Copy
            stack_trace: self.stack_trace.clone(),
            timestamp: self.timestamp,
            metadata: self.metadata.clone(),
            recovery_attempts: self.recovery_attempts,
            max_recovery_attempts: self.max_recovery_attempts,
            suggested_recovery: self.suggested_recovery,
        }
    }
}

/// Error handler function type
pub type ErrorHandler = dyn Fn(&ErrorContext) -> RecoveryAction + Send + Sync;

/// Error runtime configuration
#[derive(Debug, Clone)]
pub struct ErrorRuntimeConfig {
    /// Maximum errors per goroutine before escalation
    pub max_errors_per_goroutine: usize,
    /// Maximum total errors before system action 
    pub max_total_errors: usize,
    /// Time window for error rate limiting
    pub error_rate_window: Duration,
    /// Maximum error history to maintain
    pub max_error_history: usize,
    /// Enable stack trace capture
    pub capture_stack_traces: bool,
    /// Maximum stack trace depth
    pub max_stack_trace_depth: usize,
    /// Default recovery action
    pub default_recovery_action: RecoveryAction,
    /// Enable error statistics
    pub enable_statistics: bool,
    /// Enable error correlation analysis
    pub enable_correlation_analysis: bool,
    /// Error log level threshold
    pub log_level_threshold: ErrorSeverity,
}

impl Default for ErrorRuntimeConfig {
    fn default() -> Self {
        Self {
            max_errors_per_goroutine: 50,
            max_total_errors: 1000,
            error_rate_window: Duration::from_secs(60),
            max_error_history: 500,
            capture_stack_traces: true,
            max_stack_trace_depth: 30,
            default_recovery_action: RecoveryAction::Continue,
            enable_statistics: true,
            enable_correlation_analysis: true,
            log_level_threshold: ErrorSeverity::Warning,
        }
    }
}

/// Error statistics and metrics
#[derive(Debug, Clone)]
pub struct ErrorStatistics {
    /// Total errors encountered
    pub total_errors: u64,
    /// Errors by severity
    pub errors_by_severity: HashMap<ErrorSeverity, u64>,
    /// Errors by category
    pub errors_by_category: HashMap<ErrorCategory, u64>,
    /// Errors by goroutine
    pub errors_by_goroutine: HashMap<GoroutineId, u64>,
    /// Recovery actions used
    pub recovery_actions_used: HashMap<RecoveryAction, u64>,
    /// Error rate over time
    pub error_rate: f64,
    /// First error time
    pub first_error_time: Option<Instant>,
    /// Last error time
    pub last_error_time: Option<Instant>,
    /// Error-free duration
    pub error_free_duration: Duration,
    /// Recovery success rate
    pub recovery_success_rate: f64,
    /// Most common error patterns
    pub common_error_patterns: Vec<(String, u64)>,
}

impl Default for ErrorStatistics {
    fn default() -> Self {
        Self {
            total_errors: 0,
            errors_by_severity: HashMap::new(),
            errors_by_category: HashMap::new(),
            errors_by_goroutine: HashMap::new(),
            recovery_actions_used: HashMap::new(),
            error_rate: 0.0,
            first_error_time: None,
            last_error_time: None,
            error_free_duration: Duration::from_secs(0),
            recovery_success_rate: 1.0,
            common_error_patterns: Vec::new(),
        }
    }
}

/// Error correlation information
#[derive(Debug, Clone)]
pub struct ErrorCorrelation {
    /// Related errors that occurred together
    pub related_errors: Vec<String>,
    /// Correlation strength (0.0 to 1.0)
    pub correlation_strength: f64,
    /// Temporal correlation (errors within time window)
    pub temporal_correlation: bool,
    /// Spatial correlation (errors in same goroutine/thread)
    pub spatial_correlation: bool,
}

/// Performance monitoring trait for error runtime
pub trait ErrorPerformanceMonitor: Send + Sync {
    /// Record error event
    fn record_error(&self, context: &ErrorContext, recovery: RecoveryAction);
    /// Record recovery success
    fn record_recovery_success(&self, context: &ErrorContext);
    /// Record recovery failure
    fn record_recovery_failure(&self, context: &ErrorContext, failure_reason: &str);
    /// Get performance metrics
    fn get_error_metrics(&self) -> HashMap<String, f64>;
    /// Analyze error patterns
    fn analyze_error_patterns(&self) -> Vec<String>;
}

/// Panic handling trait for goroutine isolation
pub trait PanicHandler: Send + Sync {
    /// Handle panic in specific goroutine
    fn handle_panic(&self, goroutine_id: GoroutineId, panic_value: &str) -> RecoveryAction;
    /// Pre-panic hook
    fn pre_panic_hook(&self, goroutine_id: GoroutineId, context: &ErrorContext);
    /// Post-panic hook
    fn post_panic_hook(&self, goroutine_id: GoroutineId, recovered: bool);
}

/// Structured error trait for advanced error types
pub trait StructuredError: Send + Sync {
    /// Get error code
    fn code(&self) -> i32;
    /// Get error message
    fn message(&self) -> &str;
    /// Get error details
    fn details(&self) -> &str;
    /// Get error category
    fn category(&self) -> ErrorCategory;
    /// Get error severity
    fn severity(&self) -> ErrorSeverity;
    /// Get error context
    fn context(&self) -> HashMap<String, String>;
    /// Check if error is temporary
    fn is_temporary(&self) -> bool;
    /// Check if error is recoverable
    fn is_recoverable(&self) -> bool;
    /// Get suggested recovery action
    fn suggested_recovery(&self) -> RecoveryAction;
}

/// Enhanced error context with stack trace and correlation
#[derive(Debug, Clone)]
pub struct EnhancedErrorContext {
    /// Base error context
    pub base: ErrorContext,
    /// Stack trace frames
    pub stack_frames: Vec<StackFrame>,
    /// Error correlation ID
    pub correlation_id: String,
    /// Related errors
    pub related_errors: Vec<String>,
    /// Error chain
    pub error_chain: Vec<ErrorContext>,
    /// Performance metrics
    pub performance_metrics: HashMap<String, f64>,
}

/// Stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Function name
    pub function_name: String,
    /// File name
    pub file_name: String,
    /// Line number
    pub line_number: u32,
    /// Column number
    pub column_number: u32,
    /// Source code snippet
    pub source_snippet: Option<String>,
}

/// Panic recovery context
#[derive(Debug, Clone)]
pub struct PanicRecoveryContext {
    /// Goroutine ID where panic occurred
    pub goroutine_id: GoroutineId,
    /// Panic value
    pub panic_value: String,
    /// Stack trace at panic
    pub panic_stack_trace: Vec<StackFrame>,
    /// Recovery attempt count
    pub recovery_attempts: u32,
    /// Maximum recovery attempts
    pub max_recovery_attempts: u32,
    /// Recovery strategy
    pub recovery_strategy: RecoveryStrategy,
}

/// Recovery strategy for panic handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Ignore panic and continue
    Ignore,
    /// Restart goroutine
    Restart,
    /// Escalate to parent
    Escalate,
    /// Shutdown gracefully
    Shutdown,
    /// Custom recovery
    Custom,
}

/// Main error handling runtime system
pub struct ErrorRuntime {
    /// Configuration
    config: ErrorRuntimeConfig,
    /// Error handlers by priority (higher numbers = higher priority)
    handlers: RwLock<Vec<(i32, Arc<ErrorHandler>)>>,
    /// Error statistics
    stats: RwLock<ErrorStatistics>,
    /// Error history for analysis
    error_history: RwLock<VecDeque<ErrorContext>>,
    /// Error correlations
    correlations: RwLock<HashMap<String, ErrorCorrelation>>,
    /// Currently processing error flag
    processing_error: AtomicBool,
    /// Shutdown flag
    shutdown: AtomicBool,
    /// Runtime start time
    start_time: Instant,
    /// Performance monitor
    performance_monitor: Option<Arc<dyn ErrorPerformanceMonitor>>,
    /// Error sequence counter
    error_sequence: AtomicU64,
    /// Panic recovery handlers
    panic_handlers: RwLock<Vec<Arc<dyn PanicHandler>>>,
    /// Error context propagation
    context_propagation: RwLock<HashMap<GoroutineId, ErrorContext>>,
    /// Structured error types registry
    structured_errors: RwLock<HashMap<String, Arc<dyn StructuredError>>>,
}

impl ErrorRuntime {
    /// Create a new error runtime with default configuration
    pub fn new() -> Self {
        Self::with_config(ErrorRuntimeConfig::default())
    }

    /// Create a new error runtime with custom configuration
    pub fn with_config(config: ErrorRuntimeConfig) -> Self {
        Self {
            config,
            handlers: RwLock::new(Vec::new()),
            stats: RwLock::new(ErrorStatistics::default()),
            error_history: RwLock::new(VecDeque::new()),
            correlations: RwLock::new(HashMap::new()),
            processing_error: AtomicBool::new(false),
            shutdown: AtomicBool::new(false),
            start_time: Instant::now(),
            performance_monitor: None,
            error_sequence: AtomicU64::new(1),
            panic_handlers: RwLock::new(Vec::new()),
            context_propagation: RwLock::new(HashMap::new()),
            structured_errors: RwLock::new(HashMap::new()),
        }
    }

    /// Initialize the error runtime
    pub fn initialize(&self) -> Result<()> {
        // Initialize default error handlers if needed
        self.register_default_handlers()?;
        Ok(())
    }

    /// Shutdown the error runtime
    pub fn shutdown(&self) -> Result<()> {
        if self.shutdown.swap(true, Ordering::SeqCst) {
            return Ok(()); // Already shutdown
        }

        // Clear handlers and history
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.clear();
        }

        if let Ok(mut history) = self.error_history.write() {
            history.clear();
        }

        Ok(())
    }

    /// Handle an error with full context and recovery
    pub fn handle_error(&self, error: Error) -> Result<RecoveryAction> {
        self.handle_error_with_context(error, None, None)
    }

    /// Handle an error with additional context
    pub fn handle_error_with_context(
        &self,
        error: Error,
        goroutine_id: Option<GoroutineId>,
        custom_metadata: Option<HashMap<String, String>>,
    ) -> Result<RecoveryAction> {
        if self.shutdown.load(Ordering::Acquire) {
            return Err(Error::Runtime("Error runtime is shutdown".to_string()));
        }

        // Prevent recursive error handling
        if self.processing_error.swap(true, Ordering::AcqRel) {
            eprintln!("Recursive error handling detected: {:?}", error);
            return Ok(RecoveryAction::EscalateToPanic);
        }

        let result = self.process_error(error, goroutine_id, custom_metadata);
        self.processing_error.store(false, Ordering::Release);
        result
    }

    /// Register an error handler with priority
    pub fn register_handler(&self, priority: i32, handler: Arc<ErrorHandler>) -> Result<()> {
        let mut handlers = self.handlers.write().map_err(|_| {
            Error::Runtime("Failed to acquire error handlers lock".to_string())
        })?;

        handlers.push((priority, handler));
        handlers.sort_by_key(|(priority, _)| -priority); // Sort by descending priority

        Ok(())
    }

    /// Remove an error handler
    pub fn remove_handler(&self, priority: i32) -> Result<bool> {
        let mut handlers = self.handlers.write().map_err(|_| {
            Error::Runtime("Failed to acquire error handlers lock".to_string())
        })?;

        let original_len = handlers.len();
        handlers.retain(|(p, _)| *p != priority);
        
        Ok(handlers.len() != original_len)
    }

    /// Get current error statistics
    pub fn get_statistics(&self) -> Result<ErrorStatistics> {
        let stats = self.stats.read().map_err(|_| {
            Error::Runtime("Failed to read error statistics".to_string())
        })?;

        let mut stats_copy = stats.clone();
        
        // Update error-free duration
        if let Some(last_error) = stats_copy.last_error_time {
            stats_copy.error_free_duration = last_error.elapsed();
        } else {
            stats_copy.error_free_duration = self.start_time.elapsed();
        }

        Ok(stats_copy)
    }

    /// Get error history for analysis
    pub fn get_error_history(&self) -> Result<Vec<ErrorContext>> {
        let history = self.error_history.read().map_err(|_| {
            Error::Runtime("Failed to read error history".to_string())
        })?;

        Ok(history.iter().cloned().collect())
    }

    /// Get error correlations
    pub fn get_error_correlations(&self) -> Result<HashMap<String, ErrorCorrelation>> {
        let correlations = self.correlations.read().map_err(|_| {
            Error::Runtime("Failed to read error correlations".to_string())
        })?;

        Ok(correlations.clone())
    }

    /// Analyze error patterns and suggest improvements
    pub fn analyze_error_patterns(&self) -> Result<Vec<String>> {
        let history = self.get_error_history()?;
        let mut patterns = Vec::new();

        // Analyze common error locations
        let mut location_counts: HashMap<String, usize> = HashMap::new();
        for context in &history {
            if let Some(location) = &context.location {
                *location_counts.entry(location.clone()).or_insert(0) += 1;
            }
        }

        for (location, count) in location_counts {
            if count > 5 {
                patterns.push(format!("Frequent errors at location: {} ({} occurrences)", location, count));
            }
        }

        // Analyze error categories
        let mut category_counts: HashMap<ErrorCategory, usize> = HashMap::new();
        for context in &history {
            *category_counts.entry(context.category).or_insert(0) += 1;
        }

        for (category, count) in category_counts {
            if count > 10 {
                patterns.push(format!("High frequency of {:?} errors: {} occurrences", category, count));
            }
        }

        // Use performance monitor if available
        if let Some(monitor) = &self.performance_monitor {
            patterns.extend(monitor.analyze_error_patterns());
        }

        Ok(patterns)
    }

    /// Force error recovery for a specific goroutine
    pub fn force_recovery(&self, goroutine_id: GoroutineId) -> Result<()> {
        // Clear error count for the goroutine
        if let Ok(mut stats) = self.stats.write() {
            stats.errors_by_goroutine.remove(&goroutine_id);
        }

        if let Some(monitor) = &self.performance_monitor {
            let dummy_context = ErrorContext {
                error: Error::Runtime("Force recovery".to_string()),
                severity: ErrorSeverity::Info,
                category: ErrorCategory::Runtime,
                location: None,
                goroutine_id: Some(goroutine_id),
                thread_id: None,
                stack_trace: Vec::new(),
                timestamp: Instant::now(),
                metadata: HashMap::new(),
                recovery_attempts: 0,
                max_recovery_attempts: 0,
                suggested_recovery: RecoveryAction::Continue,
            };
            monitor.record_recovery_success(&dummy_context);
        }

        Ok(())
    }

    /// Set performance monitor
    pub fn set_performance_monitor(&mut self, monitor: Arc<dyn ErrorPerformanceMonitor>) {
        self.performance_monitor = Some(monitor);
    }

    /// Check if error runtime is currently processing an error
    pub fn is_processing_error(&self) -> bool {
        self.processing_error.load(Ordering::Acquire)
    }

    /// Get configuration
    pub fn get_config(&self) -> &ErrorRuntimeConfig {
        &self.config
    }

    /// Register panic handler
    pub fn register_panic_handler(&self, handler: Arc<dyn PanicHandler>) -> Result<()> {
        let mut handlers = self.panic_handlers.write().map_err(|_| {
            Error::Runtime("Failed to acquire panic handlers lock".to_string())
        })?;
        handlers.push(handler);
        Ok(())
    }

    /// Handle panic in goroutine
    pub fn handle_panic(&self, goroutine_id: GoroutineId, panic_value: String) -> Result<RecoveryAction> {
        let handlers = self.panic_handlers.read().map_err(|_| {
            Error::Runtime("Failed to read panic handlers".to_string())
        })?;

        // Create enhanced error context for panic
        let panic_context = ErrorContext {
            error: Error::Runtime(format!("Panic in goroutine {}: {}", goroutine_id, panic_value)),
            severity: ErrorSeverity::Fatal,
            category: ErrorCategory::Runtime,
            location: None,
            goroutine_id: Some(goroutine_id),
            thread_id: Some(std::thread::current().id()),
            stack_trace: self.capture_stack_trace(),
            timestamp: Instant::now(),
            metadata: HashMap::new(),
            recovery_attempts: 0,
            max_recovery_attempts: 3,
            suggested_recovery: RecoveryAction::RestartGoroutine,
        };

        // Store panic context for propagation
        if let Ok(mut propagation) = self.context_propagation.write() {
            propagation.insert(goroutine_id, panic_context.clone());
        }

        // Call panic handlers
        for handler in handlers.iter() {
            handler.pre_panic_hook(goroutine_id, &panic_context);
            let recovery = handler.handle_panic(goroutine_id, &panic_value);
            if recovery != RecoveryAction::Continue {
                handler.post_panic_hook(goroutine_id, true);
                return Ok(recovery);
            }
        }

        // Default panic recovery
        Ok(RecoveryAction::EscalateToPanic)
    }

    /// Register structured error type
    pub fn register_structured_error(&self, name: String, error: Arc<dyn StructuredError>) -> Result<()> {
        let mut errors = self.structured_errors.write().map_err(|_| {
            Error::Runtime("Failed to acquire structured errors lock".to_string())
        })?;
        errors.insert(name, error);
        Ok(())
    }

    /// Get structured error by name
    pub fn get_structured_error(&self, name: &str) -> Result<Option<Arc<dyn StructuredError>>> {
        let errors = self.structured_errors.read().map_err(|_| {
            Error::Runtime("Failed to read structured errors".to_string())
        })?;
        Ok(errors.get(name).cloned())
    }

    /// Create enhanced error context with stack trace
    pub fn create_enhanced_context(&self, error: Error, goroutine_id: Option<GoroutineId>) -> Result<EnhancedErrorContext> {
        let base_context = self.create_error_context(error, goroutine_id, None)?;
        let stack_frames = self.capture_detailed_stack_trace();
        let correlation_id = format!("err-{}-{}", 
            base_context.timestamp.elapsed().as_nanos(),
            self.error_sequence.fetch_add(1, Ordering::SeqCst));

        Ok(EnhancedErrorContext {
            base: base_context,
            stack_frames,
            correlation_id,
            related_errors: Vec::new(),
            error_chain: Vec::new(),
            performance_metrics: HashMap::new(),
        })
    }

    /// Capture detailed stack trace with source information
    pub fn capture_detailed_stack_trace(&self) -> Vec<StackFrame> {
        let mut frames = Vec::new();
        
        // Use backtrace crate to capture the current stack
        let bt = std::backtrace::Backtrace::capture();
        
        // For cross-compilation, create simple stack frames from string representation
        let bt_string = format!("{}", bt);
        for (i, line) in bt_string.lines().enumerate() {
            if i > self.config.max_stack_trace_depth {
                break;
            }
            
            if !line.trim().is_empty() {
                frames.push(StackFrame {
                    function_name: line.trim().to_string(),
                    file_name: "<unknown>".to_string(),
                    line_number: 0,
                    column_number: 0,
                    source_snippet: None,
                });
            }
        }
        
        frames
    }

    /// Propagate error context between goroutines
    pub fn propagate_error_context(&self, source_goroutine: GoroutineId, target_goroutine: GoroutineId) -> Result<()> {
        let mut propagation = self.context_propagation.write().map_err(|_| {
            Error::Runtime("Failed to acquire context propagation lock".to_string())
        })?;
        
        if let Some(context) = propagation.get(&source_goroutine).cloned() {
            // Create derived context for target goroutine
            let mut derived_context = context.clone();
            derived_context.goroutine_id = Some(target_goroutine);
            derived_context.timestamp = Instant::now();
            
            propagation.insert(target_goroutine, derived_context);
        }
        
        Ok(())
    }

    /// Get error context for goroutine
    pub fn get_goroutine_error_context(&self, goroutine_id: GoroutineId) -> Result<Option<ErrorContext>> {
        let propagation = self.context_propagation.read().map_err(|_| {
            Error::Runtime("Failed to read context propagation".to_string())
        })?;
        
        Ok(propagation.get(&goroutine_id).cloned())
    }

    /// Clear error context for goroutine
    pub fn clear_goroutine_error_context(&self, goroutine_id: GoroutineId) -> Result<()> {
        let mut propagation = self.context_propagation.write().map_err(|_| {
            Error::Runtime("Failed to acquire context propagation lock".to_string())
        })?;
        
        propagation.remove(&goroutine_id);
        Ok(())
    }

    // Private methods

    fn process_error(
        &self,
        error: Error,
        goroutine_id: Option<GoroutineId>,
        custom_metadata: Option<HashMap<String, String>>,
    ) -> Result<RecoveryAction> {
        // Create error context
        let context = self.create_error_context(error, goroutine_id, custom_metadata)?;

        // Update statistics
        self.update_statistics(&context)?;

        // Add to history
        self.add_to_history(context.clone())?;

        // Update correlations if enabled
        if self.config.enable_correlation_analysis {
            self.update_correlations(&context)?;
        }

        // Log error if it meets threshold
        if context.severity >= self.config.log_level_threshold {
            self.log_error(&context);
        }

        // Determine recovery action
        let recovery_action = self.determine_recovery_action(&context)?;

        // Record performance metrics
        if let Some(monitor) = &self.performance_monitor {
            monitor.record_error(&context, recovery_action);
        }

        // Execute recovery action
        self.execute_recovery_action(&context, recovery_action)?;

        Ok(recovery_action)
    }

    fn create_error_context(
        &self,
        error: Error,
        goroutine_id: Option<GoroutineId>,
        custom_metadata: Option<HashMap<String, String>>,
    ) -> Result<ErrorContext> {
        let severity = self.classify_error_severity(&error);
        let category = self.classify_error_category(&error);
        let location = self.extract_error_location(&error);
        let stack_trace = if self.config.capture_stack_traces {
            self.capture_stack_trace()
        } else {
            Vec::new()
        };

        Ok(ErrorContext {
            error,
            severity,
            category,
            location,
            goroutine_id,
            thread_id: Some(std::thread::current().id()),
            stack_trace,
            timestamp: Instant::now(),
            metadata: custom_metadata.unwrap_or_default(),
            recovery_attempts: 0,
            max_recovery_attempts: 3, // Default value
            suggested_recovery: self.config.default_recovery_action,
        })
    }

    fn classify_error_severity(&self, error: &Error) -> ErrorSeverity {
        match error {
            Error::Runtime(msg) if msg.contains("panic") => ErrorSeverity::Fatal,
            Error::Runtime(msg) if msg.contains("memory") => ErrorSeverity::Critical,
            Error::Runtime(msg) if msg.contains("parse") => ErrorSeverity::Warning,
            Error::Io(_) => ErrorSeverity::Error,
            Error::Runtime(_) => ErrorSeverity::Error,
            _ => ErrorSeverity::Error,
        }
    }

    fn classify_error_category(&self, error: &Error) -> ErrorCategory {
        match error {
            Error::Runtime(msg) if msg.contains("memory") => ErrorCategory::Memory,
            Error::Runtime(msg) if msg.contains("parse") => ErrorCategory::Parsing,
            Error::Io(_) => ErrorCategory::IO,
            Error::Runtime(_) => ErrorCategory::Runtime,
            _ => ErrorCategory::Unknown,
        }
    }

    fn extract_error_location(&self, _error: &Error) -> Option<String> {
        // In a real implementation, this would extract location information from the error
        None
    }

    pub fn capture_stack_trace(&self) -> Vec<String> {
        let mut trace = Vec::new();
        
        // Use backtrace crate to capture the current stack
        let bt = std::backtrace::Backtrace::capture();
        
        // For cross-compilation, use simple string format
        let bt_string = format!("{}", bt);
        for line in bt_string.lines() {
            if !line.trim().is_empty() {
                trace.push(line.trim().to_string());
            }
        }
        
        // If no symbols were found, provide a fallback
        if trace.is_empty() {
            trace.push("Stack trace unavailable - debug symbols may be missing".to_string());
        }
        
        trace
    }

    fn update_statistics(&self, context: &ErrorContext) -> Result<()> {
        let mut stats = self.stats.write().map_err(|_| {
            Error::Runtime("Failed to write error statistics".to_string())
        })?;

        stats.total_errors += 1;
        
        if stats.first_error_time.is_none() {
            stats.first_error_time = Some(context.timestamp);
        }
        stats.last_error_time = Some(context.timestamp);

        // Update per-severity statistics
        *stats.errors_by_severity.entry(context.severity).or_insert(0) += 1;

        // Update per-category statistics
        *stats.errors_by_category.entry(context.category).or_insert(0) += 1;

        // Update per-goroutine statistics
        if let Some(goroutine_id) = context.goroutine_id {
            *stats.errors_by_goroutine.entry(goroutine_id).or_insert(0) += 1;
        }

        // Calculate error rate
        let window_start = context.timestamp - self.config.error_rate_window;
        let recent_errors = 1; // Simplified - would count errors in window
        stats.error_rate = recent_errors as f64 / self.config.error_rate_window.as_secs_f64();

        Ok(())
    }

    fn add_to_history(&self, context: ErrorContext) -> Result<()> {
        let mut history = self.error_history.write().map_err(|_| {
            Error::Runtime("Failed to write error history".to_string())
        })?;

        history.push_back(context);

        // Maintain history size limit
        while history.len() > self.config.max_error_history {
            history.pop_front();
        }

        Ok(())
    }

    fn update_correlations(&self, context: &ErrorContext) -> Result<()> {
        let error_key = format!("{:?}:{:?}", context.category, context.severity);
        
        let mut correlations = self.correlations.write().map_err(|_| {
            Error::Runtime("Failed to write error correlations".to_string())
        })?;

        // Simple correlation analysis - would be more sophisticated in practice
        let correlation = ErrorCorrelation {
            related_errors: vec![context.error.to_string()],
            correlation_strength: 0.5,
            temporal_correlation: true,
            spatial_correlation: context.goroutine_id.is_some(),
        };

        correlations.insert(error_key, correlation);

        Ok(())
    }

    fn determine_recovery_action(&self, context: &ErrorContext) -> Result<RecoveryAction> {
        // Check handlers in priority order
        let handlers = self.handlers.read().map_err(|_| {
            Error::Runtime("Failed to read error handlers".to_string())
        })?;

        for (_, handler) in handlers.iter() {
            let action = handler(context);
            if action != self.config.default_recovery_action {
                return Ok(action);
            }
        }

        // Check error thresholds
        let stats = self.stats.read().map_err(|_| {
            Error::Runtime("Failed to read error statistics".to_string())
        })?;

        if stats.total_errors >= self.config.max_total_errors as u64 {
            return Ok(RecoveryAction::GracefulShutdown);
        }

        if let Some(goroutine_id) = context.goroutine_id {
            if let Some(&error_count) = stats.errors_by_goroutine.get(&goroutine_id) {
                if error_count >= self.config.max_errors_per_goroutine as u64 {
                    return Ok(RecoveryAction::RestartGoroutine);
                }
            }
        }

        // Check severity
        match context.severity {
            ErrorSeverity::Fatal => Ok(RecoveryAction::EscalateToPanic),
            ErrorSeverity::Critical => Ok(RecoveryAction::RestartGoroutine),
            _ => Ok(self.config.default_recovery_action),
        }
    }

    fn execute_recovery_action(&self, context: &ErrorContext, action: RecoveryAction) -> Result<()> {
        match action {
            RecoveryAction::Continue => {
                // Do nothing, continue execution
            }
            RecoveryAction::Retry => {
                // Mark for retry (would be handled by caller)
            }
            RecoveryAction::Skip => {
                // Skip current operation (would be handled by caller)
            }
            RecoveryAction::UseFallback => {
                // Use fallback value (would be handled by caller)
            }
            RecoveryAction::RestartGoroutine => {
                // Signal goroutine restart
                if let Some(goroutine_id) = context.goroutine_id {
                    eprintln!("Requesting restart of goroutine {}", goroutine_id);
                }
            }
            RecoveryAction::EscalateToPanic => {
                // Escalate to panic runtime
                eprintln!("Escalating error to panic runtime: {:?}", context.error);
            }
            RecoveryAction::GracefulShutdown => {
                // Request graceful shutdown
                eprintln!("Requesting graceful shutdown due to error: {:?}", context.error);
            }
        }

        // Update recovery statistics
        if let Ok(mut stats) = self.stats.write() {
            *stats.recovery_actions_used.entry(action).or_insert(0) += 1;
        }

        // Record success/failure with performance monitor
        if let Some(monitor) = &self.performance_monitor {
            monitor.record_recovery_success(context);
        }

        Ok(())
    }

    fn register_default_handlers(&self) -> Result<()> {
        // Register a default handler for memory errors
        let memory_handler = Arc::new(|context: &ErrorContext| -> RecoveryAction {
            if context.category == ErrorCategory::Memory {
                match context.severity {
                    ErrorSeverity::Fatal => RecoveryAction::EscalateToPanic,
                    ErrorSeverity::Critical => RecoveryAction::RestartGoroutine,
                    _ => RecoveryAction::Continue,
                }
            } else {
                RecoveryAction::Continue
            }
        });

        self.register_handler(100, memory_handler)?;

        Ok(())
    }

    fn log_error(&self, context: &ErrorContext) {
        match context.severity {
            ErrorSeverity::Fatal => eprintln!("FATAL ERROR: {}", context.error),
            ErrorSeverity::Critical => eprintln!("CRITICAL ERROR: {}", context.error),
            ErrorSeverity::Error => eprintln!("ERROR: {}", context.error),
            ErrorSeverity::Warning => eprintln!("WARNING: {}", context.error),
            ErrorSeverity::Info => println!("INFO: {}", context.error),
        }
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error [{:?}:{:?}]: {}", self.severity, self.category, self.error)?;
        
        if let Some(location) = &self.location {
            write!(f, " at {}", location)?;
        }
        
        if let Some(goroutine_id) = self.goroutine_id {
            write!(f, " [goroutine: {}]", goroutine_id)?;
        }
        
        if self.recovery_attempts > 0 {
            write!(f, " [attempts: {}/{}]", self.recovery_attempts, self.max_recovery_attempts)?;
        }
        
        Ok(())
    }
}

// Global error runtime management

/// Initialize the global error runtime
pub fn initialize_global_error_runtime() -> Result<()> {
    initialize_global_error_runtime_with_config(ErrorRuntimeConfig::default())
}

/// Initialize the global error runtime with custom configuration
pub fn initialize_global_error_runtime_with_config(config: ErrorRuntimeConfig) -> Result<()> {
    let runtime = Arc::new(ErrorRuntime::with_config(config));
    
    GLOBAL_ERROR_RUNTIME
        .set(runtime.clone())
        .map_err(|_| Error::Runtime("Global error runtime already initialized".to_string()))?;

    runtime.initialize()
}

/// Get the global error runtime
pub fn get_global_error_runtime() -> Option<Arc<ErrorRuntime>> {
    GLOBAL_ERROR_RUNTIME.get().cloned()
}

/// Shutdown the global error runtime
pub fn shutdown_global_error_runtime() -> Result<()> {
    if let Some(runtime) = get_global_error_runtime() {
        runtime.shutdown()
    } else {
        Ok(())
    }
}

// Utility functions

/// Handle a global error
pub fn handle_global_error(error: Error) -> Result<RecoveryAction> {
    get_global_error_runtime()
        .ok_or_else(|| Error::Runtime("Global error runtime not initialized".to_string()))?
        .handle_error(error)
}

/// Register a global error handler
pub fn register_global_error_handler(priority: i32, handler: Arc<ErrorHandler>) -> Result<()> {
    get_global_error_runtime()
        .ok_or_else(|| Error::Runtime("Global error runtime not initialized".to_string()))?
        .register_handler(priority, handler)
}

/// Get global error statistics
pub fn get_global_error_statistics() -> Result<ErrorStatistics> {
    get_global_error_runtime()
        .ok_or_else(|| Error::Runtime("Global error runtime not initialized".to_string()))?
        .get_statistics()
}

// Default implementation
impl Default for ErrorRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_runtime_creation() {
        let runtime = ErrorRuntime::new();
        assert!(!runtime.is_processing_error());
    }

    #[test]
    fn test_error_context_display() {
        let context = ErrorContext {
            error: Error::Runtime("Test error".to_string()),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Runtime,
            location: Some("test.rs:123".to_string()),
            goroutine_id: Some(42),
            thread_id: None,
            stack_trace: Vec::new(),
            timestamp: Instant::now(),
            metadata: HashMap::new(),
            recovery_attempts: 1,
            max_recovery_attempts: 3,
            suggested_recovery: RecoveryAction::Retry,
        };

        let display = format!("{}", context);
        assert!(display.contains("Test error"));
        assert!(display.contains("goroutine: 42"));
        assert!(display.contains("attempts: 1/3"));
    }

    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverity::Fatal > ErrorSeverity::Critical);
        assert!(ErrorSeverity::Critical > ErrorSeverity::Error);
        assert!(ErrorSeverity::Error > ErrorSeverity::Warning);
        assert!(ErrorSeverity::Warning > ErrorSeverity::Info);
    }

    #[test]
    fn test_error_statistics() {
        let runtime = ErrorRuntime::new();
        let stats = runtime.get_statistics().unwrap();
        assert_eq!(stats.total_errors, 0);
        assert_eq!(stats.recovery_success_rate, 1.0);
    }
}
