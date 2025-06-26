//! CURSED Panic Runtime System
//!
//! This module provides panic handling and recovery for the CURSED runtime:
//! - Panic detection and stack unwinding
//! - Recovery mechanisms and graceful degradation  
//! - Integration with goroutine scheduler for panic isolation
//! - Performance monitoring and panic statistics
//! - Error propagation and recovery strategies

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::panic::{PanicInfo, set_hook, take_hook};
use std::thread::ThreadId;
use std::fmt;

use crate::error_types::{Error, Result};
use crate::runtime::{RuntimeError, RuntimeErrorType};
use crate::runtime::goroutine::GoroutineId;

/// Global panic runtime instance
static GLOBAL_PANIC_RUNTIME: once_cell::sync::OnceCell<Arc<PanicRuntime>> = once_cell::sync::OnceCell::new();

/// Panic severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PanicSeverity {
    /// Recoverable panic that can be handled gracefully
    Recoverable = 0,
    /// Critical panic that requires immediate attention
    Critical = 1,
    /// Fatal panic that should terminate the program
    Fatal = 2,
}

/// Panic recovery strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryStrategy {
    /// Ignore the panic and continue
    Ignore,
    /// Log the panic and continue
    LogAndContinue,
    /// Restart the affected goroutine
    RestartGoroutine,
    /// Restart the entire scheduler
    RestartScheduler,
    /// Graceful shutdown
    GracefulShutdown,
    /// Immediate termination
    ImmediateTermination,
}

/// Panic context information
#[derive(Debug, Clone)]
pub struct PanicContext {
    /// Panic message
    pub message: String,
    /// Location where panic occurred
    pub location: Option<String>,
    /// Goroutine ID where panic occurred
    pub goroutine_id: Option<GoroutineId>,
    /// Thread ID where panic occurred
    pub thread_id: Option<ThreadId>,
    /// Stack trace at panic
    pub stack_trace: Vec<String>,
    /// Panic severity
    pub severity: PanicSeverity,
    /// Timestamp of panic
    pub timestamp: Instant,
    /// Custom panic data
    pub metadata: HashMap<String, String>,
}

/// Panic handler function type
pub type PanicHandler = dyn Fn(&PanicContext) -> RecoveryStrategy + Send + Sync;

/// Panic runtime configuration
#[derive(Debug, Clone)]
pub struct PanicRuntimeConfig {
    /// Maximum number of panics before shutdown
    pub max_panics_per_goroutine: usize,
    /// Maximum total panics before system shutdown
    pub max_total_panics: usize,
    /// Time window for panic rate limiting
    pub panic_rate_window: Duration,
    /// Enable stack trace capture
    pub capture_stack_traces: bool,
    /// Maximum stack trace depth
    pub max_stack_trace_depth: usize,
    /// Default recovery strategy
    pub default_recovery_strategy: RecoveryStrategy,
    /// Enable panic statistics
    pub enable_statistics: bool,
    /// Panic log level
    pub log_level: PanicLogLevel,
}

/// Panic logging levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanicLogLevel {
    None,
    Error,
    Warn,
    Info,
    Debug,
}

impl Default for PanicRuntimeConfig {
    fn default() -> Self {
        Self {
            max_panics_per_goroutine: 5,
            max_total_panics: 100,
            panic_rate_window: Duration::from_secs(60),
            capture_stack_traces: true,
            max_stack_trace_depth: 50,
            default_recovery_strategy: RecoveryStrategy::LogAndContinue,
            enable_statistics: true,
            log_level: PanicLogLevel::Error,
        }
    }
}

/// Panic statistics tracking
#[derive(Debug, Clone)]
pub struct PanicStatistics {
    /// Total panics encountered
    pub total_panics: u64,
    /// Panics by goroutine
    pub panics_by_goroutine: HashMap<GoroutineId, u64>,
    /// Panics by severity
    pub panics_by_severity: HashMap<PanicSeverity, u64>,
    /// Recovery strategies used
    pub recovery_strategies_used: HashMap<RecoveryStrategy, u64>,
    /// Recent panic rate
    pub recent_panic_rate: f64,
    /// First panic time
    pub first_panic_time: Option<Instant>,
    /// Last panic time
    pub last_panic_time: Option<Instant>,
    /// Panic-free duration
    pub panic_free_duration: Duration,
}

impl Default for PanicStatistics {
    fn default() -> Self {
        Self {
            total_panics: 0,
            panics_by_goroutine: HashMap::new(),
            panics_by_severity: HashMap::new(),
            recovery_strategies_used: HashMap::new(),
            recent_panic_rate: 0.0,
            first_panic_time: None,
            last_panic_time: None,
            panic_free_duration: Duration::from_secs(0),
        }
    }
}

/// Main panic runtime system
pub struct PanicRuntime {
    /// Configuration
    config: PanicRuntimeConfig,
    /// Panic handlers by priority (higher numbers = higher priority)
    handlers: RwLock<Vec<(i32, Arc<PanicHandler>)>>,
    /// Panic statistics
    stats: RwLock<PanicStatistics>,
    /// Active panic flag
    in_panic: AtomicBool,
    /// Shutdown flag
    shutdown: AtomicBool,
    /// Runtime start time
    start_time: Instant,
    /// Previous panic hook
    previous_hook: Mutex<Option<Box<dyn Fn(&PanicInfo<'_>) + 'static + Sync + Send>>>,
    /// Performance monitoring
    performance_monitor: Option<Arc<dyn PerformanceMonitor>>,
}

/// Performance monitoring trait for panic runtime
pub trait PerformanceMonitor: Send + Sync {
    /// Record panic event
    fn record_panic(&self, context: &PanicContext, recovery: RecoveryStrategy);
    /// Record recovery success
    fn record_recovery_success(&self, goroutine_id: Option<GoroutineId>);
    /// Record recovery failure
    fn record_recovery_failure(&self, goroutine_id: Option<GoroutineId>, error: &str);
    /// Get performance metrics
    fn get_metrics(&self) -> HashMap<String, f64>;
}

impl PanicRuntime {
    /// Create a new panic runtime with default configuration
    pub fn new() -> Self {
        Self::with_config(PanicRuntimeConfig::default())
    }

    /// Create a new panic runtime with custom configuration
    pub fn with_config(config: PanicRuntimeConfig) -> Self {
        Self {
            config,
            handlers: RwLock::new(Vec::new()),
            stats: RwLock::new(PanicStatistics::default()),
            in_panic: AtomicBool::new(false),
            shutdown: AtomicBool::new(false),
            start_time: Instant::now(),
            previous_hook: Mutex::new(None),
            performance_monitor: None,
        }
    }

    /// Initialize the panic runtime and install panic hooks
    pub fn initialize(&self) -> Result<()> {
        // Install global panic hook
        // Store the previous hook
        let previous = take_hook();
        *self.previous_hook.lock().unwrap() = Some(previous);

        // Install our custom hook - simplified for now
        set_hook(Box::new(|panic_info| {
            eprintln!("PANIC: {:?}", panic_info);
        }));

        Ok(())
    }

    /// Shutdown the panic runtime and restore previous hooks
    pub fn shutdown(&self) -> Result<()> {
        if self.shutdown.swap(true, Ordering::SeqCst) {
            return Ok(()); // Already shutdown
        }

        // Restore previous panic hook
        if let Some(previous_hook) = self.previous_hook.lock().unwrap().take() {
            set_hook(previous_hook);
        }

        Ok(())
    }

    /// Register a panic handler with priority
    pub fn register_handler(&self, priority: i32, handler: Arc<PanicHandler>) -> Result<()> {
        let mut handlers = self.handlers.write().map_err(|_| {
            Error::Runtime("Failed to acquire panic handlers lock".to_string())
        })?;

        handlers.push((priority, handler));
        handlers.sort_by_key(|(priority, _)| -priority); // Sort by descending priority

        Ok(())
    }

    /// Remove a panic handler
    pub fn remove_handler(&self, priority: i32) -> Result<bool> {
        let mut handlers = self.handlers.write().map_err(|_| {
            Error::Runtime("Failed to acquire panic handlers lock".to_string())
        })?;

        let original_len = handlers.len();
        handlers.retain(|(p, _)| *p != priority);
        
        Ok(handlers.len() != original_len)
    }

    /// Trigger a controlled panic for testing
    pub fn trigger_test_panic(&self, message: &str, severity: PanicSeverity) -> Result<()> {
        let context = PanicContext {
            message: message.to_string(),
            location: Some("test_panic".to_string()),
            goroutine_id: None,
            thread_id: Some(std::thread::current().id()),
            stack_trace: self.capture_stack_trace(),
            severity,
            timestamp: Instant::now(),
            metadata: HashMap::new(),
        };

        self.process_panic(&context)
    }

    /// Get current panic statistics
    pub fn get_statistics(&self) -> Result<PanicStatistics> {
        let stats = self.stats.read().map_err(|_| {
            Error::Runtime("Failed to read panic statistics".to_string())
        })?;

        let mut stats_copy = stats.clone();
        
        // Update panic-free duration
        if let Some(last_panic) = stats_copy.last_panic_time {
            stats_copy.panic_free_duration = last_panic.elapsed();
        } else {
            stats_copy.panic_free_duration = self.start_time.elapsed();
        }

        Ok(stats_copy)
    }

    /// Check if system is currently in panic state
    pub fn is_in_panic(&self) -> bool {
        self.in_panic.load(Ordering::Acquire)
    }

    /// Check if panic runtime is shutdown
    pub fn is_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::Acquire)
    }

    /// Set performance monitor
    pub fn set_performance_monitor(&mut self, monitor: Arc<dyn PerformanceMonitor>) {
        self.performance_monitor = Some(monitor);
    }

    /// Force panic recovery for a specific goroutine
    pub fn force_recovery(&self, goroutine_id: GoroutineId) -> Result<()> {
        // Implement recovery logic for specific goroutine
        if let Some(monitor) = &self.performance_monitor {
            monitor.record_recovery_success(Some(goroutine_id));
        }

        Ok(())
    }

    /// Get configuration
    pub fn get_config(&self) -> &PanicRuntimeConfig {
        &self.config
    }

    // Private methods

    fn handle_panic(&self, panic_info: &PanicInfo<'_>) {
        if self.shutdown.load(Ordering::Acquire) {
            return;
        }

        // Prevent recursive panics
        if self.in_panic.swap(true, Ordering::AcqRel) {
            eprintln!("Recursive panic detected, terminating");
            std::process::abort();
        }

        let context = self.create_panic_context(panic_info);
        
        if let Err(e) = self.process_panic(&context) {
            eprintln!("Failed to process panic: {}", e);
        }

        self.in_panic.store(false, Ordering::Release);
    }

    fn create_panic_context(&self, panic_info: &PanicInfo<'_>) -> PanicContext {
        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        let location = panic_info.location().map(|loc| {
            format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
        });

        PanicContext {
            message,
            location,
            goroutine_id: self.get_current_goroutine_id(),
            thread_id: Some(std::thread::current().id()),
            stack_trace: self.capture_stack_trace(),
            severity: PanicSeverity::Critical, // Default to critical
            timestamp: Instant::now(),
            metadata: HashMap::new(),
        }
    }

    fn process_panic(&self, context: &PanicContext) -> Result<()> {
        // Update statistics
        self.update_statistics(context)?;

        // Log panic based on configuration
        self.log_panic(context);

        // Determine recovery strategy
        let recovery_strategy = self.determine_recovery_strategy(context)?;

        // Record performance metrics
        if let Some(monitor) = &self.performance_monitor {
            monitor.record_panic(context, recovery_strategy);
        }

        // Execute recovery strategy
        self.execute_recovery_strategy(context, recovery_strategy)?;

        Ok(())
    }

    fn determine_recovery_strategy(&self, context: &PanicContext) -> Result<RecoveryStrategy> {
        // Check handlers in priority order
        let handlers = self.handlers.read().map_err(|_| {
            Error::Runtime("Failed to read panic handlers".to_string())
        })?;

        for (_, handler) in handlers.iter() {
            let strategy = handler(context);
            if strategy != self.config.default_recovery_strategy {
                return Ok(strategy);
            }
        }

        // Check if we've exceeded panic limits
        let stats = self.stats.read().map_err(|_| {
            Error::Runtime("Failed to read panic statistics".to_string())
        })?;

        if stats.total_panics >= self.config.max_total_panics as u64 {
            return Ok(RecoveryStrategy::GracefulShutdown);
        }

        if let Some(goroutine_id) = context.goroutine_id {
            if let Some(&panic_count) = stats.panics_by_goroutine.get(&goroutine_id) {
                if panic_count >= self.config.max_panics_per_goroutine as u64 {
                    return Ok(RecoveryStrategy::RestartGoroutine);
                }
            }
        }

        // Use default strategy
        Ok(self.config.default_recovery_strategy)
    }

    fn execute_recovery_strategy(&self, context: &PanicContext, strategy: RecoveryStrategy) -> Result<()> {
        match strategy {
            RecoveryStrategy::Ignore => {
                // Do nothing
            }
            RecoveryStrategy::LogAndContinue => {
                // Already logged, just continue
            }
            RecoveryStrategy::RestartGoroutine => {
                if let Some(goroutine_id) = context.goroutine_id {
                    self.restart_goroutine(goroutine_id)?;
                }
            }
            RecoveryStrategy::RestartScheduler => {
                self.restart_scheduler()?;
            }
            RecoveryStrategy::GracefulShutdown => {
                self.initiate_graceful_shutdown()?;
            }
            RecoveryStrategy::ImmediateTermination => {
                std::process::exit(1);
            }
        }

        // Update recovery strategy statistics
        if let Ok(mut stats) = self.stats.write() {
            *stats.recovery_strategies_used.entry(strategy).or_insert(0) += 1;
        }

        Ok(())
    }

    fn update_statistics(&self, context: &PanicContext) -> Result<()> {
        let mut stats = self.stats.write().map_err(|_| {
            Error::Runtime("Failed to write panic statistics".to_string())
        })?;

        stats.total_panics += 1;
        
        if stats.first_panic_time.is_none() {
            stats.first_panic_time = Some(context.timestamp);
        }
        stats.last_panic_time = Some(context.timestamp);

        // Update per-goroutine statistics
        if let Some(goroutine_id) = context.goroutine_id {
            *stats.panics_by_goroutine.entry(goroutine_id).or_insert(0) += 1;
        }

        // Update per-severity statistics
        *stats.panics_by_severity.entry(context.severity).or_insert(0) += 1;

        // Calculate recent panic rate
        let window_start = context.timestamp - self.config.panic_rate_window;
        let recent_panics = 1; // Simplified - would count panics in window
        stats.recent_panic_rate = recent_panics as f64 / self.config.panic_rate_window.as_secs_f64();

        Ok(())
    }

    fn log_panic(&self, context: &PanicContext) {
        match self.config.log_level {
            PanicLogLevel::None => {}
            PanicLogLevel::Error => eprintln!("PANIC: {}", context.message),
            PanicLogLevel::Warn => eprintln!("PANIC [WARN]: {}", context.message),
            PanicLogLevel::Info => {
                eprintln!("PANIC [INFO]: {} at {:?}", context.message, context.location);
            }
            PanicLogLevel::Debug => {
                eprintln!("PANIC [DEBUG]: {}", context);
            }
        }
    }

    fn capture_stack_trace(&self) -> Vec<String> {
        if !self.config.capture_stack_traces {
            return Vec::new();
        }

        // Simple stack trace capture for now
        vec!["Stack trace capture not implemented yet".to_string()]
    }

    fn get_current_goroutine_id(&self) -> Option<GoroutineId> {
        // This would integrate with the goroutine scheduler to get current goroutine ID
        None // Simplified implementation
    }

    fn restart_goroutine(&self, _goroutine_id: GoroutineId) -> Result<()> {
        // This would integrate with the goroutine scheduler to restart a specific goroutine
        Ok(())
    }

    fn restart_scheduler(&self) -> Result<()> {
        // This would integrate with the goroutine scheduler to restart the entire scheduler
        Ok(())
    }

    fn initiate_graceful_shutdown(&self) -> Result<()> {
        // This would initiate a graceful shutdown of the entire runtime
        Ok(())
    }
}

impl fmt::Display for PanicContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Panic: {}", self.message)?;
        
        if let Some(location) = &self.location {
            write!(f, " at {}", location)?;
        }
        
        if let Some(goroutine_id) = self.goroutine_id {
            write!(f, " [goroutine: {}]", goroutine_id)?;
        }
        
        write!(f, " [severity: {:?}]", self.severity)?;
        
        if !self.stack_trace.is_empty() {
            write!(f, "\nStack trace:")?;
            for (i, frame) in self.stack_trace.iter().enumerate() {
                write!(f, "\n  {}: {}", i, frame)?;
            }
        }
        
        Ok(())
    }
}

// Global panic runtime management

/// Initialize the global panic runtime
pub fn initialize_global_panic_runtime() -> Result<()> {
    initialize_global_panic_runtime_with_config(PanicRuntimeConfig::default())
}

/// Initialize the global panic runtime with custom configuration
pub fn initialize_global_panic_runtime_with_config(config: PanicRuntimeConfig) -> Result<()> {
    let runtime = Arc::new(PanicRuntime::with_config(config));
    
    GLOBAL_PANIC_RUNTIME
        .set(runtime.clone())
        .map_err(|_| Error::Runtime("Global panic runtime already initialized".to_string()))?;

    runtime.initialize()
}

/// Get the global panic runtime
pub fn get_global_panic_runtime() -> Option<Arc<PanicRuntime>> {
    GLOBAL_PANIC_RUNTIME.get().cloned()
}

/// Shutdown the global panic runtime
pub fn shutdown_global_panic_runtime() -> Result<()> {
    if let Some(runtime) = get_global_panic_runtime() {
        runtime.shutdown()
    } else {
        Ok(())
    }
}

// Utility functions

/// Register a global panic handler
pub fn register_global_panic_handler(priority: i32, handler: Arc<PanicHandler>) -> Result<()> {
    get_global_panic_runtime()
        .ok_or_else(|| Error::Runtime("Global panic runtime not initialized".to_string()))?
        .register_handler(priority, handler)
}

/// Get global panic statistics
pub fn get_global_panic_statistics() -> Result<PanicStatistics> {
    get_global_panic_runtime()
        .ok_or_else(|| Error::Runtime("Global panic runtime not initialized".to_string()))?
        .get_statistics()
}

/// Trigger a test panic
pub fn trigger_global_test_panic(message: &str, severity: PanicSeverity) -> Result<()> {
    get_global_panic_runtime()
        .ok_or_else(|| Error::Runtime("Global panic runtime not initialized".to_string()))?
        .trigger_test_panic(message, severity)
}

// Default implementation
impl Default for PanicRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panic_runtime_creation() {
        let runtime = PanicRuntime::new();
        assert!(!runtime.is_in_panic());
        assert!(!runtime.is_shutdown());
    }

    #[test]
    fn test_panic_context_display() {
        let context = PanicContext {
            message: "Test panic".to_string(),
            location: Some("test.rs:123:45".to_string()),
            goroutine_id: Some(42),
            thread_id: None,
            stack_trace: vec!["frame1".to_string(), "frame2".to_string()],
            severity: PanicSeverity::Critical,
            timestamp: Instant::now(),
            metadata: HashMap::new(),
        };

        let display = format!("{}", context);
        assert!(display.contains("Test panic"));
        assert!(display.contains("test.rs:123:45"));
        assert!(display.contains("goroutine: 42"));
    }

    #[test]
    fn test_panic_statistics() {
        let runtime = PanicRuntime::new();
        let stats = runtime.get_statistics().unwrap();
        assert_eq!(stats.total_panics, 0);
    }

    #[test]
    fn test_recovery_strategy_priority() {
        assert!(PanicSeverity::Fatal > PanicSeverity::Critical);
        assert!(PanicSeverity::Critical > PanicSeverity::Recoverable);
    }
}
