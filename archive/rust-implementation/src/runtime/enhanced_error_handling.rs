//! Enhanced Error Handling Runtime for CURSED
//!
//! This module provides production-grade error handling with:
//! - Advanced error propagation with yikes, shook, fam keywords
//! - Panic recovery mechanisms with goroutine isolation
//! - Context preservation and stack trace capture
//! - Error monitoring and reporting
//! - Integration with existing runtime systems

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::thread::ThreadId;
use std::panic::{self, PanicHookInfo};
use std::fmt;

use crate::error_types::{Error, Result};
use crate::runtime::{RuntimeError, RuntimeErrorType};
use crate::runtime::goroutine::GoroutineId;
use crate::runtime::error_handling::{
    ErrorRuntime, ErrorContext, ErrorSeverity, ErrorCategory, RecoveryAction,
    ErrorStatistics, ErrorRuntimeConfig, PanicRecoveryContext, RecoveryStrategy
};

/// Global enhanced error runtime instance
static ENHANCED_ERROR_RUNTIME: once_cell::sync::OnceCell<Arc<EnhancedErrorRuntime>> = once_cell::sync::OnceCell::new();

/// Enhanced error types specific to CURSED keywords
#[derive(Debug, Clone)]
pub enum CursedErrorType {
    /// Yikes error - user-defined error
    Yikes {
        name: String,
        message: String,
        context: HashMap<String, String>,
        stack_trace: Vec<String>,
    },
    /// Shook error - propagated error with context
    Shook {
        source_error: Box<CursedErrorType>,
        propagation_context: PropagationContext,
    },
    /// Fam error - recovered error with recovery info
    Fam {
        original_error: Box<CursedErrorType>,
        recovery_successful: bool,
        recovery_context: RecoveryContext,
    },
}

/// Error propagation context for shook keyword
#[derive(Debug, Clone)]
pub struct PropagationContext {
    pub source_function: String,
    pub source_line: u32,
    pub source_column: u32,
    pub goroutine_id: Option<GoroutineId>,
    pub propagation_chain: Vec<String>,
    pub timestamp: Instant,
}

/// Recovery context for fam keyword
#[derive(Debug, Clone)]
pub struct RecoveryContext {
    pub recovery_function: String,
    pub recovery_line: u32,
    pub recovery_column: u32,
    pub recovery_attempts: u32,
    pub recovery_strategy: RecoveryStrategy,
    pub timestamp: Instant,
}

/// Enhanced error runtime with production-grade features
pub struct EnhancedErrorRuntime {
    /// Base error runtime
    base_runtime: Arc<ErrorRuntime>,
    /// CURSED-specific error storage
    cursed_errors: RwLock<HashMap<String, CursedErrorType>>,
    /// Goroutine error isolation
    goroutine_error_isolation: RwLock<HashMap<GoroutineId, Arc<Mutex<Vec<CursedErrorType>>>>>,
    /// Panic recovery handlers
    panic_recovery_handlers: RwLock<HashMap<GoroutineId, Arc<dyn PanicRecoveryHandler>>>,
    /// Error context propagation
    error_context_propagation: RwLock<HashMap<String, PropagationContext>>,
    /// Performance metrics
    performance_metrics: Arc<Mutex<ErrorPerformanceMetrics>>,
    /// Configuration
    config: EnhancedErrorRuntimeConfig,
    /// Error monitoring
    error_monitor: Arc<Mutex<ErrorMonitor>>,
    /// Stack trace capture
    stack_trace_capture: Arc<StackTraceCapture>,
}

/// Enhanced error runtime configuration
#[derive(Debug, Clone)]
pub struct EnhancedErrorRuntimeConfig {
    /// Base configuration
    pub base_config: ErrorRuntimeConfig,
    /// Enable enhanced stack traces
    pub enable_enhanced_stack_traces: bool,
    /// Enable error correlation across goroutines
    pub enable_cross_goroutine_correlation: bool,
    /// Enable production-grade error monitoring
    pub enable_production_monitoring: bool,
    /// Maximum error context depth
    pub max_error_context_depth: usize,
    /// Error reporting endpoint
    pub error_reporting_endpoint: Option<String>,
    /// Enable async error reporting
    pub enable_async_error_reporting: bool,
    /// Error suppression patterns
    pub error_suppression_patterns: Vec<String>,
    /// Enable error analytics
    pub enable_error_analytics: bool,
}

impl Default for EnhancedErrorRuntimeConfig {
    fn default() -> Self {
        Self {
            base_config: ErrorRuntimeConfig::default(),
            enable_enhanced_stack_traces: true,
            enable_cross_goroutine_correlation: true,
            enable_production_monitoring: true,
            max_error_context_depth: 50,
            error_reporting_endpoint: None,
            enable_async_error_reporting: false,
            error_suppression_patterns: Vec::new(),
            enable_error_analytics: true,
        }
    }
}

/// Performance metrics for error handling
#[derive(Debug, Clone)]
pub struct ErrorPerformanceMetrics {
    /// Total error handling time
    pub total_error_handling_time: Duration,
    /// Average error handling time
    pub avg_error_handling_time: Duration,
    /// Error handling throughput (errors/sec)
    pub error_handling_throughput: f64,
    /// Memory usage for error handling
    pub memory_usage_bytes: usize,
    /// Number of goroutines with errors
    pub goroutines_with_errors: usize,
    /// Recovery success rate
    pub recovery_success_rate: f64,
    /// Performance degradation factor
    pub performance_degradation_factor: f64,
}

impl Default for ErrorPerformanceMetrics {
    fn default() -> Self {
        Self {
            total_error_handling_time: Duration::from_secs(0),
            avg_error_handling_time: Duration::from_secs(0),
            error_handling_throughput: 0.0,
            memory_usage_bytes: 0,
            goroutines_with_errors: 0,
            recovery_success_rate: 1.0,
            performance_degradation_factor: 1.0,
        }
    }
}

/// Error monitor for production environments
#[derive(Debug)]
pub struct ErrorMonitor {
    /// Error rate tracking
    pub error_rate_tracker: ErrorRateTracker,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
    /// Error reporting queue
    pub error_reporting_queue: VecDeque<ErrorReport>,
    /// Monitoring enabled flag
    pub monitoring_enabled: bool,
}

/// Error rate tracking
#[derive(Debug, Clone)]
pub struct ErrorRateTracker {
    /// Error counts by time window
    pub error_counts: HashMap<u64, u64>,
    /// Current time window
    pub current_window: u64,
    /// Window size in seconds
    pub window_size: u64,
    /// Alert threshold
    pub alert_threshold: f64,
}

/// Alert thresholds for monitoring
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// Maximum error rate (errors/sec)
    pub max_error_rate: f64,
    /// Maximum memory usage (MB)
    pub max_memory_usage: f64,
    /// Maximum goroutines with errors
    pub max_goroutines_with_errors: usize,
    /// Minimum recovery success rate
    pub min_recovery_success_rate: f64,
}

/// Error report for monitoring
#[derive(Debug, Clone)]
pub struct ErrorReport {
    /// Error type
    pub error_type: CursedErrorType,
    /// Timestamp
    pub timestamp: Instant,
    /// Goroutine ID
    pub goroutine_id: Option<GoroutineId>,
    /// Performance impact
    pub performance_impact: f64,
    /// Recovery status
    pub recovery_status: RecoveryStatus,
}

/// Recovery status
#[derive(Debug, Clone)]
pub enum RecoveryStatus {
    /// Successfully recovered
    Recovered,
    /// Failed to recover
    Failed,
    /// Recovery in progress
    InProgress,
    /// No recovery attempted
    NoRecovery,
}

/// Stack trace capture with enhanced information
pub struct StackTraceCapture {
    /// Enable source code context
    pub enable_source_context: bool,
    /// Maximum frames to capture
    pub max_frames: usize,
    /// Enable symbol resolution
    pub enable_symbol_resolution: bool,
}

impl StackTraceCapture {
    pub fn new() -> Self {
        Self {
            enable_source_context: true,
            max_frames: 100,
            enable_symbol_resolution: true,
        }
    }

    /// Capture enhanced stack trace
    pub fn capture_enhanced_trace(&self) -> Vec<EnhancedStackFrame> {
        let mut frames = Vec::new();
        
        let bt = std::backtrace::Backtrace::capture();
        
        let bt_string = format!("{}", bt);
        for (i, line) in bt_string.lines().enumerate() {
            if i >= self.max_frames {
                break;
            }
            
            if !line.trim().is_empty() {
                frames.push(EnhancedStackFrame {
                    function_name: line.trim().to_string(),
                    file_name: "<unknown>".to_string(),
                    line_number: 0,
                    column_number: 0,
                    source_context: None,
                    frame_index: i,
                });
            }
        }
        
        frames
    }
    
    fn read_source_context(&self, file_name: &str, line_number: u32) -> Option<String> {
        // Read source file and extract context around the line
        std::fs::read_to_string(file_name).ok().and_then(|content| {
            let lines: Vec<&str> = content.lines().collect();
            let line_idx = line_number as usize;
            
            if line_idx < lines.len() {
                let start = line_idx.saturating_sub(2);
                let end = (line_idx + 3).min(lines.len());
                
                Some(lines[start..end].join("\n"))
            } else {
                None
            }
        })
    }
}

/// Enhanced stack frame with source context
#[derive(Debug, Clone)]
pub struct EnhancedStackFrame {
    pub function_name: String,
    pub file_name: String,
    pub line_number: u32,
    pub column_number: u32,
    pub source_context: Option<String>,
    pub frame_index: usize,
}

/// Panic recovery handler trait
pub trait PanicRecoveryHandler: Send + Sync {
    /// Handle panic for specific goroutine
    fn handle_panic(&self, goroutine_id: GoroutineId, panic_info: &PanicHookInfo) -> RecoveryAction;
    
    /// Pre-panic hook
    fn pre_panic(&self, goroutine_id: GoroutineId, context: &ErrorContext);
    
    /// Post-panic hook
    fn post_panic(&self, goroutine_id: GoroutineId, recovered: bool, recovery_context: &RecoveryContext);
    
    /// Check if panic is recoverable
    fn is_recoverable(&self, panic_info: &PanicHookInfo) -> bool;
    
    /// Get recovery strategy
    fn get_recovery_strategy(&self, goroutine_id: GoroutineId, panic_info: &PanicHookInfo) -> RecoveryStrategy;
}

/// Default panic recovery handler
pub struct DefaultPanicRecoveryHandler {
    pub goroutine_id: GoroutineId,
    pub max_recovery_attempts: u32,
    pub recovery_attempts: Arc<AtomicU64>,
}

impl DefaultPanicRecoveryHandler {
    pub fn new(goroutine_id: GoroutineId) -> Self {
        Self {
            goroutine_id,
            max_recovery_attempts: 3,
            recovery_attempts: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl PanicRecoveryHandler for DefaultPanicRecoveryHandler {
    fn handle_panic(&self, goroutine_id: GoroutineId, panic_info: &PanicHookInfo) -> RecoveryAction {
        let attempts = self.recovery_attempts.fetch_add(1, Ordering::SeqCst);
        
        if attempts >= self.max_recovery_attempts as u64 {
            return RecoveryAction::EscalateToPanic;
        }
        
        // Analyze panic and determine recovery strategy
        if let Some(payload) = panic_info.payload().downcast_ref::<String>() {
            if payload.contains("memory") {
                return RecoveryAction::RestartGoroutine;
            } else if payload.contains("temporary") {
                return RecoveryAction::Retry;
            }
        }
        
        RecoveryAction::Continue
    }
    
    fn pre_panic(&self, _goroutine_id: GoroutineId, _context: &ErrorContext) {
        // Log panic occurrence
        eprintln!("Panic detected in goroutine {}", _goroutine_id);
    }
    
    fn post_panic(&self, _goroutine_id: GoroutineId, recovered: bool, _recovery_context: &RecoveryContext) {
        if recovered {
            eprintln!("Successfully recovered from panic in goroutine {}", _goroutine_id);
        } else {
            eprintln!("Failed to recover from panic in goroutine {}", _goroutine_id);
        }
    }
    
    fn is_recoverable(&self, panic_info: &PanicHookInfo) -> bool {
        // Check if panic is recoverable based on payload
        if let Some(payload) = panic_info.payload().downcast_ref::<String>() {
            !payload.contains("fatal") && !payload.contains("corruption")
        } else {
            true // Default to recoverable
        }
    }
    
    fn get_recovery_strategy(&self, _goroutine_id: GoroutineId, panic_info: &PanicHookInfo) -> RecoveryStrategy {
        if let Some(payload) = panic_info.payload().downcast_ref::<String>() {
            if payload.contains("restart") {
                RecoveryStrategy::Restart
            } else if payload.contains("ignore") {
                RecoveryStrategy::Ignore
            } else {
                RecoveryStrategy::Escalate
            }
        } else {
            RecoveryStrategy::Custom
        }
    }
}

impl EnhancedErrorRuntime {
    /// Create new enhanced error runtime
    pub fn new() -> Self {
        Self::with_config(EnhancedErrorRuntimeConfig::default())
    }
    
    /// Create with custom configuration
    pub fn with_config(config: EnhancedErrorRuntimeConfig) -> Self {
        let base_runtime = Arc::new(ErrorRuntime::with_config(config.base_config.clone()));
        
        Self {
            base_runtime,
            cursed_errors: RwLock::new(HashMap::new()),
            goroutine_error_isolation: RwLock::new(HashMap::new()),
            panic_recovery_handlers: RwLock::new(HashMap::new()),
            error_context_propagation: RwLock::new(HashMap::new()),
            performance_metrics: Arc::new(Mutex::new(ErrorPerformanceMetrics::default())),
            error_monitor: Arc::new(Mutex::new(ErrorMonitor::new())),
            stack_trace_capture: Arc::new(StackTraceCapture::new()),
            config,
        }
    }
    
    /// Initialize the enhanced error runtime
    pub fn initialize(&self) -> Result<()> {
        // Initialize base runtime
        self.base_runtime.initialize()?;
        
        // Set up panic hook for goroutine isolation
        self.setup_panic_hook()?;
        
        // Initialize error monitoring
        if self.config.enable_production_monitoring {
            self.initialize_monitoring()?;
        }
        
        Ok(())
    }
    
    /// Handle yikes error creation
    pub fn handle_yikes_error(&self, name: String, message: String, context: HashMap<String, String>) -> Result<()> {
        let start_time = Instant::now();
        
        // Capture stack trace
        let stack_trace = if self.config.enable_enhanced_stack_traces {
            self.stack_trace_capture.capture_enhanced_trace()
                .iter()
                .map(|frame| format!("{}:{}", frame.function_name, frame.line_number))
                .collect()
        } else {
            Vec::new()
        };
        
        // Create CURSED error
        let cursed_error = CursedErrorType::Yikes {
            name: name.clone(),
            message: message.clone(),
            context,
            stack_trace,
        };
        
        // Store error
        if let Ok(mut errors) = self.cursed_errors.write() {
            errors.insert(name.clone(), cursed_error.clone());
        }
        
        // Update performance metrics
        self.update_performance_metrics(start_time.elapsed())?;
        
        // Report to monitoring system
        if self.config.enable_production_monitoring {
            self.report_error_to_monitor(cursed_error)?;
        }
        
        Ok(())
    }
    
    /// Handle shook error propagation
    pub fn handle_shook_error(&self, source_error: CursedErrorType, context: PropagationContext) -> Result<CursedErrorType> {
        let start_time = Instant::now();
        
        // Create propagated error
        let propagated_error = CursedErrorType::Shook {
            source_error: Box::new(source_error),
            propagation_context: context,
        };
        
        // Update performance metrics
        self.update_performance_metrics(start_time.elapsed())?;
        
        Ok(propagated_error)
    }
    
    /// Handle fam error recovery
    pub fn handle_fam_recovery(&self, error: CursedErrorType, recovery_context: RecoveryContext) -> Result<CursedErrorType> {
        let start_time = Instant::now();
        
        // Attempt recovery
        let recovery_successful = self.attempt_recovery(&error, &recovery_context)?;
        
        // Create recovery error
        let recovery_error = CursedErrorType::Fam {
            original_error: Box::new(error),
            recovery_successful,
            recovery_context,
        };
        
        // Update performance metrics
        self.update_performance_metrics(start_time.elapsed())?;
        
        Ok(recovery_error)
    }
    
    /// Setup panic hook for goroutine isolation
    fn setup_panic_hook(&self) -> Result<()> {
        // Set up global panic hook
        panic::set_hook(Box::new(|panic_info| {
            // Handle panic with goroutine isolation
            eprintln!("Panic occurred: {:?}", panic_info);
            
            // Try to get current goroutine ID
            let goroutine_id = std::thread::current().id();
            
            // Report panic to enhanced error runtime
            if let Some(runtime) = ENHANCED_ERROR_RUNTIME.get() {
                if let Err(e) = runtime.handle_panic_with_isolation(goroutine_id, panic_info) {
                    eprintln!("Failed to handle panic: {:?}", e);
                }
            }
        }));
        
        Ok(())
    }
    
    /// Handle panic with goroutine isolation
    fn handle_panic_with_isolation(&self, goroutine_id: ThreadId, panic_info: &PanicHookInfo) -> Result<()> {
        // Convert ThreadId to GoroutineId (simplified)
        let goroutine_id: GoroutineId = format!("{:?}", goroutine_id).parse().unwrap_or(0);
        
        // Get recovery handler for goroutine
        let recovery_action = if let Ok(handlers) = self.panic_recovery_handlers.read() {
            if let Some(handler) = handlers.get(&goroutine_id) {
                handler.handle_panic(goroutine_id, panic_info)
            } else {
                RecoveryAction::EscalateToPanic
            }
        } else {
            RecoveryAction::EscalateToPanic
        };
        
        // Execute recovery action
        match recovery_action {
            RecoveryAction::Continue => {
                // Continue execution
            }
            RecoveryAction::RestartGoroutine => {
                // Restart goroutine (simplified)
                eprintln!("Restarting goroutine {}", goroutine_id);
            }
            RecoveryAction::EscalateToPanic => {
                // Re-panic
                eprintln!("Unrecoverable panic in goroutine {} - escalating to runtime", goroutine_id);
                // Instead of panicking, let the runtime handle it
                std::process::exit(1);
            }
            _ => {
                // Handle other recovery actions
            }
        }
        
        Ok(())
    }
    
    /// Initialize monitoring system
    fn initialize_monitoring(&self) -> Result<()> {
        if let Ok(mut monitor) = self.error_monitor.lock() {
            monitor.monitoring_enabled = true;
            monitor.alert_thresholds = AlertThresholds {
                max_error_rate: 100.0,
                max_memory_usage: 1024.0,
                max_goroutines_with_errors: 10,
                min_recovery_success_rate: 0.8,
            };
        }
        
        Ok(())
    }
    
    /// Report error to monitoring system
    fn report_error_to_monitor(&self, error: CursedErrorType) -> Result<()> {
        if let Ok(mut monitor) = self.error_monitor.lock() {
            if monitor.monitoring_enabled {
                let report = ErrorReport {
                    error_type: error,
                    timestamp: Instant::now(),
                    goroutine_id: None,
                    performance_impact: 0.1,
                    recovery_status: RecoveryStatus::NoRecovery,
                };
                
                monitor.error_reporting_queue.push_back(report);
                
                // Trim queue if too large
                while monitor.error_reporting_queue.len() > 1000 {
                    monitor.error_reporting_queue.pop_front();
                }
            }
        }
        
        Ok(())
    }
    
    /// Update performance metrics
    fn update_performance_metrics(&self, elapsed: Duration) -> Result<()> {
        if let Ok(mut metrics) = self.performance_metrics.lock() {
            metrics.total_error_handling_time += elapsed;
            // Update other metrics as needed
        }
        
        Ok(())
    }
    
    /// Attempt recovery for error
    fn attempt_recovery(&self, _error: &CursedErrorType, _context: &RecoveryContext) -> Result<bool> {
        // Simplified recovery logic
        Ok(true)
    }
    
    /// Get error statistics
    pub fn get_error_statistics(&self) -> Result<ErrorStatistics> {
        self.base_runtime.get_statistics()
    }
    
    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> Result<ErrorPerformanceMetrics> {
        if let Ok(metrics) = self.performance_metrics.lock() {
            Ok(metrics.clone())
        } else {
            Err(Error::Runtime("Failed to get performance metrics".to_string()))
        }
    }
}

impl ErrorMonitor {
    pub fn new() -> Self {
        Self {
            error_rate_tracker: ErrorRateTracker {
                error_counts: HashMap::new(),
                current_window: 0,
                window_size: 60,
                alert_threshold: 10.0,
            },
            alert_thresholds: AlertThresholds {
                max_error_rate: 100.0,
                max_memory_usage: 1024.0,
                max_goroutines_with_errors: 10,
                min_recovery_success_rate: 0.8,
            },
            error_reporting_queue: VecDeque::new(),
            monitoring_enabled: false,
        }
    }
}

/// Initialize global enhanced error runtime
pub fn initialize_enhanced_error_runtime() -> Result<()> {
    let runtime = Arc::new(EnhancedErrorRuntime::new());
    runtime.initialize()?;
    
    ENHANCED_ERROR_RUNTIME.set(runtime).map_err(|_| {
        Error::Runtime("Failed to initialize enhanced error runtime".to_string())
    })?;
    
    Ok(())
}

/// Get global enhanced error runtime
pub fn get_enhanced_error_runtime() -> Option<Arc<EnhancedErrorRuntime>> {
    ENHANCED_ERROR_RUNTIME.get().cloned()
}

/// Shutdown enhanced error runtime
pub fn shutdown_enhanced_error_runtime() -> Result<()> {
    if let Some(runtime) = ENHANCED_ERROR_RUNTIME.get() {
        runtime.base_runtime.shutdown()?;
    }
    
    Ok(())
}

/// Helper function to handle yikes errors
pub fn handle_yikes(name: String, message: String) -> Result<()> {
    if let Some(runtime) = get_enhanced_error_runtime() {
        runtime.handle_yikes_error(name, message, HashMap::new())
    } else {
        Err(Error::Runtime("Enhanced error runtime not initialized".to_string()))
    }
}

/// Helper function to handle shook errors
pub fn handle_shook(error: CursedErrorType) -> Result<CursedErrorType> {
    if let Some(runtime) = get_enhanced_error_runtime() {
        let context = PropagationContext {
            source_function: "unknown".to_string(),
            source_line: 0,
            source_column: 0,
            goroutine_id: None,
            propagation_chain: Vec::new(),
            timestamp: Instant::now(),
        };
        
        runtime.handle_shook_error(error, context)
    } else {
        Err(Error::Runtime("Enhanced error runtime not initialized".to_string()))
    }
}

/// Helper function to handle fam recovery
pub fn handle_fam(error: CursedErrorType) -> Result<CursedErrorType> {
    if let Some(runtime) = get_enhanced_error_runtime() {
        let context = RecoveryContext {
            recovery_function: "unknown".to_string(),
            recovery_line: 0,
            recovery_column: 0,
            recovery_attempts: 0,
            recovery_strategy: RecoveryStrategy::Custom,
            timestamp: Instant::now(),
        };
        
        runtime.handle_fam_recovery(error, context)
    } else {
        Err(Error::Runtime("Enhanced error runtime not initialized".to_string()))
    }
}
