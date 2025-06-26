//! CURSED Runtime System
//!
//! This module provides the core runtime system for CURSED, including:
//! - Runtime coordinator for goroutines and memory management
//! - Configuration and statistics tracking
//! - Runtime error handling and propagation
//! - Integration with the CURSED execution environment

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::fmt;

use crate::error_types::{Error, Result};
use crate::runtime::value::Value;
use crate::memory::{Tag, Traceable, Visitor};

/// Main runtime coordinator for the CURSED execution environment
/// 
/// The Runtime struct serves as the central coordinator for all runtime services
/// including goroutine scheduling, memory management, and execution control.
pub struct Runtime {
    /// Runtime configuration
    config: RuntimeConfig,
    /// Current runtime statistics
    stats: Arc<RwLock<RuntimeStats>>,
    /// Goroutine scheduler reference
    scheduler: Arc<Mutex<Option<Box<dyn GoroutineScheduler>>>>,
    /// Memory manager reference  
    memory_manager: Arc<Mutex<Option<Box<dyn MemoryManager>>>>,
    /// Runtime state
    state: Arc<RwLock<RuntimeState>>,
    /// Value manager for runtime values
    value_manager: Arc<Mutex<ValueManager>>,
    /// Start time for uptime calculation
    start_time: Instant,
}

/// Runtime configuration options
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Maximum number of goroutines allowed
    pub max_goroutines: usize,
    /// Stack size per goroutine in bytes
    pub goroutine_stack_size: usize,
    /// Memory limit in bytes (None for unlimited)
    pub memory_limit: Option<usize>,
    /// Garbage collection frequency 
    pub gc_frequency: Duration,
    /// Enable debug mode
    pub debug_mode: bool,
    /// Enable performance profiling
    pub profiling_enabled: bool,
    /// Maximum call stack depth
    pub max_call_depth: usize,
    /// Runtime timeouts
    pub timeouts: TimeoutConfig,
}

/// Timeout configuration for runtime operations
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// Goroutine spawn timeout
    pub goroutine_spawn: Duration,
    /// Memory allocation timeout
    pub memory_allocation: Duration,
    /// Runtime shutdown timeout
    pub shutdown: Duration,
}

/// Runtime performance and usage statistics
#[derive(Debug, Clone)]
pub struct RuntimeStats {
    /// Total runtime uptime
    pub uptime: Duration,
    /// Current number of active goroutines
    pub active_goroutines: usize,
    /// Total goroutines created since startup
    pub total_goroutines_created: usize,
    /// Total goroutines completed since startup
    pub total_goroutines_completed: usize,
    /// Current memory usage in bytes
    pub memory_usage: usize,
    /// Peak memory usage in bytes
    pub peak_memory_usage: usize,
    /// Total memory allocations
    pub total_allocations: usize,
    /// Total memory deallocations
    pub total_deallocations: usize,
    /// Garbage collection statistics
    pub gc_stats: GcStats,
    /// Runtime errors encountered
    pub total_errors: usize,
    /// Performance metrics
    pub performance: PerformanceMetrics,
}

/// Garbage collection statistics
#[derive(Debug, Clone)]
pub struct GcStats {
    /// Total GC cycles run
    pub total_cycles: usize,
    /// Time spent in GC
    pub total_gc_time: Duration,
    /// Average GC pause time
    pub average_pause_time: Duration,
    /// Objects collected in last cycle
    pub last_cycle_collected: usize,
    /// Memory freed in last cycle
    pub last_cycle_freed: usize,
}

/// Performance metrics for runtime operations
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Average goroutine spawn time
    pub avg_goroutine_spawn_time: Duration,
    /// Average memory allocation time
    pub avg_allocation_time: Duration,
    /// Function call rate (calls per second)
    pub function_call_rate: f64,
    /// Memory allocation rate (bytes per second)
    pub allocation_rate: f64,
}

/// Runtime error types
#[derive(Debug, Clone)]
pub enum RuntimeErrorType {
    /// Goroutine scheduling error
    SchedulingError,
    /// Memory management error
    MemoryError,
    /// Stack overflow error
    StackOverflow,
    /// Resource exhaustion error
    ResourceExhaustion,
    /// Timeout error
    Timeout,
    /// Configuration error
    ConfigurationError,
    /// Internal runtime error
    InternalError,
    /// Value operation error
    ValueError,
    /// Panic recovery error
    PanicError,
}

/// Runtime error with detailed context
#[derive(Debug, Clone)]
pub struct RuntimeError {
    /// Type of runtime error
    pub error_type: RuntimeErrorType,
    /// Error message
    pub message: String,
    /// Optional source error
    pub source: Option<Box<RuntimeError>>,
    /// Context where error occurred
    pub context: Option<String>,
    /// Goroutine ID where error occurred
    pub goroutine_id: Option<usize>,
    /// Timestamp when error occurred
    pub timestamp: Instant,
}

/// Internal runtime state
#[derive(Debug)]
struct RuntimeState {
    /// Whether runtime is running
    running: bool,
    /// Whether shutdown has been initiated
    shutting_down: bool,
    /// Current execution phase
    phase: RuntimePhase,
}

/// Runtime execution phases
#[derive(Debug, Clone, Copy, PartialEq)]
enum RuntimePhase {
    /// Runtime is initializing
    Initializing,
    /// Runtime is running normally
    Running,
    /// Runtime is shutting down
    ShuttingDown,
    /// Runtime has stopped
    Stopped,
}

/// Trait for goroutine schedulers
pub trait GoroutineScheduler: Send + Sync {
    /// Spawn a new goroutine
    fn spawn(&mut self, task: Box<dyn FnOnce() + Send>) -> Result<usize>;
    /// Get number of active goroutines
    fn active_count(&self) -> usize;
    /// Shutdown scheduler and wait for all goroutines
    fn shutdown(&mut self) -> Result<()>;
}

/// Trait for memory managers
pub trait MemoryManager: Send + Sync {
    /// Allocate memory
    fn allocate(&mut self, size: usize) -> Result<*mut u8>;
    /// Deallocate memory
    fn deallocate(&mut self, ptr: *mut u8, size: usize) -> Result<()>;
    /// Get current memory usage
    fn memory_usage(&self) -> usize;
    /// Force garbage collection
    fn collect_garbage(&mut self) -> Result<usize>;
}

/// Value manager for runtime values
#[derive(Debug)]
pub struct ValueManager {
    /// Global value cache
    globals: HashMap<String, Value>,
    /// Value reference counts
    ref_counts: HashMap<*const Value, usize>,
}

impl Runtime {
    /// Create a new runtime with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(RuntimeConfig::default())
    }

    /// Create a new runtime with specific configuration
    pub fn with_config(config: RuntimeConfig) -> Result<Self> {
        let start_time = Instant::now();
        let stats = RuntimeStats::new();
        let state = RuntimeState {
            running: false,
            shutting_down: false,
            phase: RuntimePhase::Initializing,
        };

        Ok(Runtime {
            config,
            stats: Arc::new(RwLock::new(stats)),
            scheduler: Arc::new(Mutex::new(None)),
            memory_manager: Arc::new(Mutex::new(None)),
            state: Arc::new(RwLock::new(state)),
            value_manager: Arc::new(Mutex::new(ValueManager::new())),
            start_time,
        })
    }

    /// Initialize and start the runtime
    pub fn start(&self) -> Result<()> {
        let mut state = self.state.write().map_err(|_| {
            RuntimeError::new(RuntimeErrorType::InternalError, "Failed to acquire state lock")
        })?;

        if state.running {
            return Err(RuntimeError::new(
                RuntimeErrorType::ConfigurationError,
                "Runtime is already running"
            ).into());
        }

        // Initialize memory manager if not set
        if self.memory_manager.lock().unwrap().is_none() {
            // Would initialize with actual memory manager
        }

        // Initialize scheduler if not set  
        if self.scheduler.lock().unwrap().is_none() {
            // Would initialize with actual scheduler
        }

        state.running = true;
        state.phase = RuntimePhase::Running;

        // Update stats
        if let Ok(mut stats) = self.stats.write() {
            stats.uptime = self.start_time.elapsed();
        }

        Ok(())
    }

    /// Shutdown the runtime gracefully
    pub fn shutdown(&self) -> Result<()> {
        let mut state = self.state.write().map_err(|_| {
            RuntimeError::new(RuntimeErrorType::InternalError, "Failed to acquire state lock")
        })?;

        if !state.running {
            return Ok(());
        }

        state.shutting_down = true;
        state.phase = RuntimePhase::ShuttingDown;

        // Shutdown scheduler
        if let Ok(mut scheduler_guard) = self.scheduler.lock() {
            if let Some(scheduler) = scheduler_guard.as_mut() {
                scheduler.shutdown()?;
            }
        }

        state.running = false;
        state.phase = RuntimePhase::Stopped;

        Ok(())
    }

    /// Check if runtime is running
    pub fn is_running(&self) -> bool {
        self.state.read()
            .map(|state| state.running)
            .unwrap_or(false)
    }

    /// Get current runtime statistics
    pub fn get_stats(&self) -> Result<RuntimeStats> {
        let stats = self.stats.read().map_err(|_| {
            RuntimeError::new(RuntimeErrorType::InternalError, "Failed to read stats")
        })?;
        
        let mut stats_copy = stats.clone();
        stats_copy.uptime = self.start_time.elapsed();
        
        Ok(stats_copy)
    }

    /// Get runtime configuration
    pub fn get_config(&self) -> &RuntimeConfig {
        &self.config
    }

    /// Update runtime statistics
    pub fn update_stats<F>(&self, updater: F) -> Result<()> 
    where 
        F: FnOnce(&mut RuntimeStats),
    {
        let mut stats = self.stats.write().map_err(|_| {
            RuntimeError::new(RuntimeErrorType::InternalError, "Failed to write stats")
        })?;
        
        updater(&mut stats);
        Ok(())
    }

    /// Set goroutine scheduler
    pub fn set_scheduler(&self, scheduler: Box<dyn GoroutineScheduler>) -> Result<()> {
        let mut scheduler_guard = self.scheduler.lock().map_err(|_| {
            RuntimeError::new(RuntimeErrorType::InternalError, "Failed to acquire scheduler lock")
        })?;
        
        *scheduler_guard = Some(scheduler);
        Ok(())
    }

    /// Set memory manager
    pub fn set_memory_manager(&self, manager: Box<dyn MemoryManager>) -> Result<()> {
        let mut manager_guard = self.memory_manager.lock().map_err(|_| {
            RuntimeError::new(RuntimeErrorType::InternalError, "Failed to acquire memory manager lock")
        })?;
        
        *manager_guard = Some(manager);
        Ok(())
    }

    /// Spawn a new goroutine
    pub fn spawn_goroutine<F>(&self, task: F) -> Result<usize>
    where
        F: FnOnce() + Send + 'static,
    {
        let mut scheduler_guard = self.scheduler.lock().map_err(|_| {
            RuntimeError::new(RuntimeErrorType::SchedulingError, "Failed to acquire scheduler")
        })?;

        let scheduler = scheduler_guard.as_mut().ok_or_else(|| {
            RuntimeError::new(RuntimeErrorType::ConfigurationError, "No scheduler configured")
        })?;

        let goroutine_id = scheduler.spawn(Box::new(task))?;

        // Update stats
        self.update_stats(|stats| {
            stats.active_goroutines += 1;
            stats.total_goroutines_created += 1;
        })?;

        Ok(goroutine_id)
    }

    /// Force garbage collection
    pub fn collect_garbage(&self) -> Result<usize> {
        let mut manager_guard = self.memory_manager.lock().map_err(|_| {
            RuntimeError::new(RuntimeErrorType::MemoryError, "Failed to acquire memory manager")
        })?;

        let manager = manager_guard.as_mut().ok_or_else(|| {
            RuntimeError::new(RuntimeErrorType::ConfigurationError, "No memory manager configured")
        })?;

        let collected = manager.collect_garbage()?;

        // Update stats
        self.update_stats(|stats| {
            stats.gc_stats.total_cycles += 1;
            stats.gc_stats.last_cycle_collected = collected;
        })?;

        Ok(collected)
    }

    /// Get value manager
    pub fn value_manager(&self) -> Arc<Mutex<ValueManager>> {
        Arc::clone(&self.value_manager)
    }
}

impl RuntimeConfig {
    /// Create default runtime configuration
    pub fn default() -> Self {
        RuntimeConfig {
            max_goroutines: 10000,
            goroutine_stack_size: 8 * 1024 * 1024, // 8MB
            memory_limit: None,
            gc_frequency: Duration::from_millis(100),
            debug_mode: false,
            profiling_enabled: false,
            max_call_depth: 1000,
            timeouts: TimeoutConfig::default(),
        }
    }

    /// Create runtime configuration for development
    pub fn development() -> Self {
        RuntimeConfig {
            debug_mode: true,
            profiling_enabled: true,
            ..Self::default()
        }
    }

    /// Create runtime configuration for production
    pub fn production() -> Self {
        RuntimeConfig {
            debug_mode: false,
            profiling_enabled: false,
            gc_frequency: Duration::from_millis(50),
            ..Self::default()
        }
    }
}

impl TimeoutConfig {
    /// Create default timeout configuration
    pub fn default() -> Self {
        TimeoutConfig {
            goroutine_spawn: Duration::from_millis(100),
            memory_allocation: Duration::from_millis(50),
            shutdown: Duration::from_secs(30),
        }
    }
}

impl RuntimeStats {
    /// Create new runtime statistics
    pub fn new() -> Self {
        RuntimeStats {
            uptime: Duration::from_secs(0),
            active_goroutines: 0,
            total_goroutines_created: 0,
            total_goroutines_completed: 0,
            memory_usage: 0,
            peak_memory_usage: 0,
            total_allocations: 0,
            total_deallocations: 0,
            gc_stats: GcStats::new(),
            total_errors: 0,
            performance: PerformanceMetrics::new(),
        }
    }
}

impl GcStats {
    /// Create new GC statistics
    pub fn new() -> Self {
        GcStats {
            total_cycles: 0,
            total_gc_time: Duration::from_secs(0),
            average_pause_time: Duration::from_secs(0),
            last_cycle_collected: 0,
            last_cycle_freed: 0,
        }
    }
}

impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new() -> Self {
        PerformanceMetrics {
            avg_goroutine_spawn_time: Duration::from_micros(100),
            avg_allocation_time: Duration::from_micros(50),
            function_call_rate: 0.0,
            allocation_rate: 0.0,
        }
    }
}

impl RuntimeError {
    /// Create a new runtime error
    pub fn new<S: Into<String>>(error_type: RuntimeErrorType, message: S) -> Self {
        RuntimeError {
            error_type,
            message: message.into(),
            source: None,
            context: None,
            goroutine_id: None,
            timestamp: Instant::now(),
        }
    }

    /// Create a runtime error with context
    pub fn with_context<S: Into<String>, C: Into<String>>(
        error_type: RuntimeErrorType,
        message: S,
        context: C,
    ) -> Self {
        RuntimeError {
            error_type,
            message: message.into(),
            source: None,
            context: Some(context.into()),
            goroutine_id: None,
            timestamp: Instant::now(),
        }
    }

    /// Create a runtime error with source
    pub fn with_source<S: Into<String>>(
        error_type: RuntimeErrorType,
        message: S,
        source: RuntimeError,
    ) -> Self {
        RuntimeError {
            error_type,
            message: message.into(),
            source: Some(Box::new(source)),
            context: None,
            goroutine_id: None,
            timestamp: Instant::now(),
        }
    }

    /// Set goroutine ID for this error
    pub fn with_goroutine_id(mut self, goroutine_id: usize) -> Self {
        self.goroutine_id = Some(goroutine_id);
        self
    }
}

impl ValueManager {
    /// Create a new value manager
    pub fn new() -> Self {
        ValueManager {
            globals: HashMap::new(),
            ref_counts: HashMap::new(),
        }
    }

    /// Set a global value
    pub fn set_global(&mut self, name: String, value: Value) {
        self.globals.insert(name, value);
    }

    /// Get a global value
    pub fn get_global(&self, name: &str) -> Option<&Value> {
        self.globals.get(name)
    }

    /// Remove a global value
    pub fn remove_global(&mut self, name: &str) -> Option<Value> {
        self.globals.remove(name)
    }

    /// Get all global variable names
    pub fn global_names(&self) -> Vec<&String> {
        self.globals.keys().collect()
    }
}

// Implement Display for runtime types
impl fmt::Display for RuntimeErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeErrorType::SchedulingError => write!(f, "Scheduling Error"),
            RuntimeErrorType::MemoryError => write!(f, "Memory Error"),
            RuntimeErrorType::StackOverflow => write!(f, "Stack Overflow"),
            RuntimeErrorType::ResourceExhaustion => write!(f, "Resource Exhaustion"),
            RuntimeErrorType::Timeout => write!(f, "Timeout"),
            RuntimeErrorType::ConfigurationError => write!(f, "Configuration Error"),
            RuntimeErrorType::InternalError => write!(f, "Internal Error"),
            RuntimeErrorType::ValueError => write!(f, "Value Error"),
            RuntimeErrorType::PanicError => write!(f, "Panic Error"),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_type, self.message)?;
        
        if let Some(context) = &self.context {
            write!(f, " (context: {})", context)?;
        }
        
        if let Some(goroutine_id) = self.goroutine_id {
            write!(f, " [goroutine: {}]", goroutine_id)?;
        }
        
        if let Some(source) = &self.source {
            write!(f, " caused by: {}", source)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for RuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

// Convert RuntimeError to the main Error type
impl From<RuntimeError> for Error {
    fn from(runtime_error: RuntimeError) -> Self {
        Error::Runtime(runtime_error.to_string())
    }
}

// Compatibility exports for existing code
pub type CursedValue = Value;

/// Initialize global runtime with default configuration
pub fn initialize_runtime() -> Result<Arc<Runtime>> {
    let runtime = Arc::new(Runtime::new()?);
    runtime.start()?;
    Ok(runtime)
}

/// Initialize global runtime with custom configuration
pub fn initialize_runtime_with_config(config: RuntimeConfig) -> Result<Arc<Runtime>> {
    let runtime = Arc::new(Runtime::with_config(config)?);
    runtime.start()?;
    Ok(runtime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let runtime = Runtime::new().unwrap();
        assert!(!runtime.is_running());
    }

    #[test]
    fn test_runtime_start_shutdown() {
        let runtime = Runtime::new().unwrap();
        assert!(runtime.start().is_ok());
        assert!(runtime.is_running());
        assert!(runtime.shutdown().is_ok());
        assert!(!runtime.is_running());
    }

    #[test]
    fn test_runtime_config() {
        let config = RuntimeConfig::development();
        assert!(config.debug_mode);
        assert!(config.profiling_enabled);

        let config = RuntimeConfig::production();
        assert!(!config.debug_mode);
        assert!(!config.profiling_enabled);
    }

    #[test]
    fn test_runtime_stats() {
        let runtime = Runtime::new().unwrap();
        let stats = runtime.get_stats().unwrap();
        assert_eq!(stats.active_goroutines, 0);
        assert_eq!(stats.total_goroutines_created, 0);
    }

    #[test]
    fn test_runtime_error() {
        let error = RuntimeError::new(RuntimeErrorType::MemoryError, "Out of memory");
        assert!(matches!(error.error_type, RuntimeErrorType::MemoryError));
        assert_eq!(error.message, "Out of memory");
    }

    #[test]
    fn test_value_manager() {
        let mut vm = ValueManager::new();
        vm.set_global("test".to_string(), Value::integer(42));
        assert_eq!(vm.get_global("test"), Some(&Value::integer(42)));
        assert!(vm.get_global("nonexistent").is_none());
    }
}
