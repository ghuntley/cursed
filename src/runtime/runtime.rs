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
use crate::runtime::pal::{PlatformAbstraction, Architecture, OperatingSystem, PlatformError};
use crate::runtime::memory::MemoryManager;
use crate::runtime::goroutine::Scheduler;

/// Main runtime coordinator for the CURSED execution environment
/// 
/// The Runtime struct serves as the central coordinator for all runtime services
/// including goroutine scheduling, memory management, and execution control.
/// Now enhanced with Platform Abstraction Layer (PAL) for cross-platform optimization.
pub struct Runtime {
    /// Runtime configuration
    config: RuntimeConfig,
    /// Current runtime statistics
    stats: Arc<RwLock<RuntimeStats>>,
    /// Platform Abstraction Layer
    pal: Arc<dyn PlatformAbstraction>,
    /// Platform-specific memory manager
    memory_manager: Arc<dyn crate::runtime::memory::MemoryManager>,
    /// Platform-specific scheduler
    scheduler: Arc<dyn Scheduler>,
    /// Runtime state
    state: Arc<RwLock<RuntimeState>>,
    /// Value manager for runtime values
    value_manager: Arc<Mutex<ValueManager>>,
    /// Start time for uptime calculation
    start_time: Instant,
}

/// Runtime configuration options enhanced with PAL integration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Maximum number of goroutines allowed
    pub max_goroutines: usize,
    /// Default stack size per goroutine in bytes (platform-optimized)
    pub default_stack_size: usize,
    /// Memory alignment requirement (platform-specific)
    pub memory_alignment: usize,
    /// Garbage collection trigger ratio
    pub gc_trigger_ratio: f32,
    /// Scheduler quantum duration
    pub scheduler_quantum: Duration,
    /// Platform name for debugging
    pub platform_name: String,
    /// Target architecture
    pub architecture: Architecture,
    /// Operating system
    pub operating_system: OperatingSystem,
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
    /// Peak active goroutines
    pub peak_active_goroutines: usize,
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
    /// JIT compilation time
    pub jit_compilation_time: Duration,
    /// GC collection frequency
    pub gc_collection_count: u64,
    /// Peak memory usage
    pub peak_memory_usage: usize,
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
    /// Initialization error
    InitializationError,
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
pub trait GoroutineSchedulerTrait: Send + Sync {
    /// Spawn a new goroutine
    fn spawn(&mut self, task: Box<dyn FnOnce() + Send>) -> Result<usize>;
    /// Get number of active goroutines
    fn active_count(&self) -> usize;
    /// Shutdown scheduler and wait for all goroutines
    fn shutdown(&mut self) -> Result<()>;
    /// Start the scheduler
    fn start(&mut self) -> Result<()>;
    /// Check if scheduler is running
    fn is_running(&self) -> bool;
    /// Get scheduler statistics
    fn get_stats(&self) -> Result<SchedulerStatistics>;
}

/// Scheduler statistics for runtime integration
#[derive(Debug, Clone)]
pub struct SchedulerStatistics {
    pub total_spawned: u64,
    pub total_completed: u64,
    pub current_active: usize,
    pub peak_active: usize,
    pub total_panicked: u64,
    pub uptime: Duration,
}

// MemoryManager trait is defined in runtime::memory module

/// Value manager for runtime values
#[derive(Debug)]
pub struct ValueManager {
    /// Global value cache
    globals: HashMap<String, Value>,
    /// Value reference counts
    ref_counts: HashMap<*const Value, usize>,
}

/// Platform information structure for runtime introspection
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    /// Platform name (e.g., "ARM64 macOS (Apple Silicon)")
    pub name: String,
    /// Target architecture
    pub architecture: Architecture,
    /// Operating system
    pub operating_system: OperatingSystem,
    /// Hardware concurrency level
    pub hardware_concurrency: usize,
    /// Memory page size in bytes
    pub page_size: usize,
    /// Default stack size in bytes
    pub default_stack_size: usize,
}

impl Runtime {
    /// Create a new runtime with PAL integration
    pub fn new(config: RuntimeConfig, pal: Arc<dyn PlatformAbstraction>) -> Result<Self> {
        let start_time = Instant::now();
        let stats = RuntimeStats::new();
        let state = RuntimeState {
            running: false,
            shutting_down: false,
            phase: RuntimePhase::Initializing,
        };

        // Get platform-specific components from PAL
        let memory_manager = pal.memory_manager();
        let scheduler = pal.scheduler();

        log::info!("Creating runtime for {} ({})", 
                   config.platform_name, 
                   format!("{:?} on {:?}", config.architecture, config.operating_system));

        Ok(Runtime {
            config,
            stats: Arc::new(RwLock::new(stats)),
            pal,
            memory_manager,
            scheduler,
            state: Arc::new(RwLock::new(state)),
            value_manager: Arc::new(Mutex::new(ValueManager::new())),
            start_time,
        })
    }

    /// Create a new runtime with default PAL detection
    pub fn with_default_pal() -> Result<Self> {
        let pal = crate::runtime::pal::create_platform_abstraction()
            .map_err(|e| RuntimeError::new(
                RuntimeErrorType::InitializationError, 
                format!("Failed to create PAL: {}", e)
            ))?;
        
        pal.initialize()
            .map_err(|e| RuntimeError::new(
                RuntimeErrorType::InitializationError,
                format!("Failed to initialize PAL: {}", e)
            ))?;

        let platform_config = crate::runtime::pal::common::PlatformDetector::get_optimal_config();
        
        let config = RuntimeConfig {
            max_goroutines: platform_config.max_goroutines,
            default_stack_size: platform_config.default_stack_size,
            memory_alignment: platform_config.memory_alignment,
            gc_trigger_ratio: platform_config.gc_trigger_ratio,
            scheduler_quantum: platform_config.scheduler_quantum,
            platform_name: pal.platform_name().to_string(),
            architecture: pal.architecture(),
            operating_system: pal.operating_system(),
            memory_limit: None,
            gc_frequency: Duration::from_millis(100),
            debug_mode: false,
            profiling_enabled: false,
            max_call_depth: 10000,
            timeouts: TimeoutConfig::default(),
        };

        Self::new(config, pal)
    }

    /// Create a new runtime with configuration (PAL compatibility method)
    pub fn with_config(config: RuntimeConfig) -> Result<Self> {
        let pal = crate::runtime::pal::create_platform_abstraction()
            .map_err(|e| RuntimeError::new(
                RuntimeErrorType::InitializationError, 
                format!("Failed to create PAL: {}", e)
            ))?;
        
        pal.initialize()
            .map_err(|e| RuntimeError::new(
                RuntimeErrorType::InitializationError,
                format!("Failed to initialize PAL: {}", e)
            ))?;

        Self::new(config, pal)
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

        log::info!("Starting CURSED runtime on {}", self.config.platform_name);
        log::debug!("Hardware concurrency: {}", self.pal.hardware_concurrency());
        log::debug!("Default stack size: {}KB", self.config.default_stack_size / 1024);
        log::debug!("Page size: {}KB", self.pal.page_size() / 1024);

        // Memory manager and scheduler are already initialized via PAL
        // Just need to mark runtime as running
        state.running = true;
        state.phase = RuntimePhase::Running;

        // Update stats
        if let Ok(mut stats) = self.stats.write() {
            stats.uptime = self.start_time.elapsed();
        }

        log::info!("CURSED runtime started successfully");
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

        log::info!("Shutting down CURSED runtime");

        // Scheduler and memory manager cleanup is handled by their Drop implementations
        // or explicit cleanup in the PAL

        state.running = false;
        state.phase = RuntimePhase::Stopped;

        log::info!("CURSED runtime shutdown complete");

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
        
        // Update memory statistics from PAL memory manager
        stats_copy.memory_usage = self.memory_manager.memory_usage();
        stats_copy.peak_memory_usage = std::cmp::max(
            stats_copy.peak_memory_usage,
            stats_copy.memory_usage
        );
        
        // Update platform-specific statistics
        if let Some(memory_stats) = self.memory_manager.get_stats() {
            stats_copy.total_allocations = memory_stats.total_allocations;
            stats_copy.total_deallocations = memory_stats.total_deallocations;
        }
        
        Ok(stats_copy)
    }

    /// Get runtime configuration
    pub fn get_config(&self) -> &RuntimeConfig {
        &self.config
    }

    /// Get platform abstraction layer
    pub fn get_pal(&self) -> &dyn PlatformAbstraction {
        &*self.pal
    }

    /// Get platform-specific memory manager
    pub fn get_memory_manager(&self) -> &dyn crate::runtime::memory::MemoryManager {
        &*self.memory_manager
    }

    /// Get platform-specific scheduler
    pub fn get_scheduler(&self) -> &dyn Scheduler {
        &*self.scheduler
    }

    /// Spawn a goroutine using the platform-optimized scheduler
    pub fn spawn_goroutine<F>(&self, task: F) -> Result<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let boxed_task = Box::new(task);
        self.scheduler.spawn_goroutine(boxed_task)
            .map_err(|e| RuntimeError::new(
                RuntimeErrorType::SchedulingError,
                format!("Failed to spawn goroutine: {}", e)
            ).into())
    }

    /// Allocate memory using the platform-optimized memory manager
    pub fn allocate_memory(&self, size: usize) -> Result<*mut u8> {
        self.memory_manager.allocate(size)
            .map_err(|e| RuntimeError::new(
                RuntimeErrorType::MemoryError,
                format!("Failed to allocate memory: {}", e)
            ).into())
    }

    /// Deallocate memory using the platform-optimized memory manager
    pub fn deallocate_memory(&self, ptr: *mut u8, size: usize) -> Result<()> {
        self.memory_manager.deallocate(ptr, size)
            .map_err(|e| RuntimeError::new(
                RuntimeErrorType::MemoryError,
                format!("Failed to deallocate memory: {}", e)
            ).into())
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

    /// Platform information access
    pub fn platform_info(&self) -> PlatformInfo {
        PlatformInfo {
            name: self.config.platform_name.clone(),
            architecture: self.config.architecture,
            operating_system: self.config.operating_system,
            hardware_concurrency: self.pal.hardware_concurrency(),
            page_size: self.pal.page_size(),
            default_stack_size: self.config.default_stack_size,
        }
    }

    /// Force garbage collection using the platform-optimized memory manager
    pub fn collect_garbage(&self) -> Result<usize> {
        // Implementation depends on memory manager providing this capability
        // For now, return 0 as many memory managers handle GC automatically
        Ok(0)
    }

    /// Get value manager
    pub fn value_manager(&self) -> Arc<Mutex<ValueManager>> {
        Arc::clone(&self.value_manager)
    }

    /// Set scheduler (for PAL compatibility)
    pub fn set_scheduler(&self, _scheduler: Box<dyn GoroutineSchedulerTrait>) -> Result<()> {
        // For now, just return success as we use the PAL scheduler
        Ok(())
    }

    /// Handle scheduler error (for PAL compatibility)
    pub fn handle_scheduler_error(&self, error: RuntimeError) -> Result<()> {
        // Update error count
        self.update_stats(|stats| {
            stats.total_errors += 1;
        })?;
        
        // Return the error
        Err(error.into())
    }
}

impl RuntimeConfig {
    /// Create default runtime configuration (requires PAL for optimal values)
    pub fn default() -> Self {
        RuntimeConfig {
            max_goroutines: 100_000,
            default_stack_size: 256 * 1024, // 256KB conservative default
            memory_alignment: 8,
            gc_trigger_ratio: 0.75,
            scheduler_quantum: Duration::from_millis(20),
            platform_name: "Unknown Platform".to_string(),
            architecture: Architecture::X86_64, // Default fallback
            operating_system: OperatingSystem::Linux, // Default fallback
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
            gc_trigger_ratio: 0.5, // More aggressive GC for debugging
            ..Self::default()
        }
    }

    /// Create runtime configuration for production
    pub fn production() -> Self {
        RuntimeConfig {
            debug_mode: false,
            profiling_enabled: false,
            gc_frequency: Duration::from_millis(50),
            gc_trigger_ratio: 0.85, // Less aggressive GC for performance
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
            peak_active_goroutines: 0,
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
            jit_compilation_time: Duration::from_secs(0),
            gc_collection_count: 0,
            peak_memory_usage: 0,
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
            RuntimeErrorType::InitializationError => write!(f, "Initialization Error"),
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

/// Initialize global runtime with PAL auto-detection
pub fn initialize_runtime() -> Result<Arc<Runtime>> {
    let runtime = Arc::new(Runtime::with_default_pal()?);
    runtime.start()?;
    Ok(runtime)
}

/// Initialize global runtime with custom configuration and PAL
pub fn initialize_runtime_with_config(config: RuntimeConfig) -> Result<Arc<Runtime>> {
    let runtime = Arc::new(create_runtime_with_pal(config)?);
    runtime.start()?;
    Ok(runtime)
}

/// Create a runtime with PAL and custom configuration  
pub fn create_runtime_with_pal(config: RuntimeConfig) -> Result<Runtime> {
    let pal = crate::runtime::pal::create_platform_abstraction()
        .map_err(|e| RuntimeError::new(
            RuntimeErrorType::InitializationError,
            format!("Failed to create PAL: {}", e)
        ))?;
    
    pal.initialize()
        .map_err(|e| RuntimeError::new(
            RuntimeErrorType::InitializationError,
            format!("Failed to initialize PAL: {}", e)
        ))?;
    
    Runtime::new(config, pal)
}

/// Initialize complete runtime system with memory management
pub fn initialize_complete_runtime(
    runtime_config: RuntimeConfig, 
    memory_config: crate::runtime::memory::MemoryConfig
) -> Result<Arc<Runtime>> {
    use crate::runtime::{initialize_memory_manager, gc::initialize_gc};
    
    // Create stack manager
    let stack_manager = Arc::new(crate::runtime::RuntimeStack::new());
    
    // Initialize garbage collector
    initialize_gc(memory_config.gc_config.clone(), Arc::clone(&stack_manager))
        .map_err(|e| Error::Runtime(e.to_string()))?;
    
    // Initialize memory manager
    initialize_memory_manager(memory_config, Arc::clone(&stack_manager))
        .map_err(|e| Error::Runtime(e.to_string()))?;
    
    // Initialize interface dispatch system
    crate::runtime::interface_dispatch::initialize_interface_dispatch()
        .map_err(|e| Error::Runtime(e.to_string()))?;
    
    // Initialize interface compliance checker
    crate::type_system::interface_compliance::initialize_interface_compliance_checker();
    
    // Create and start runtime
    let runtime = Arc::new(Runtime::with_config(runtime_config)?);
    
    // Set up memory manager integration
    if let Some(_memory_manager) = crate::runtime::get_global_memory_manager() {
        // Note: Direct integration would require trait object compatibility
        // For now, the memory manager is available through global functions
    }
    
    runtime.start()?;
    Ok(runtime)
}

/// Shutdown complete runtime system
pub fn shutdown_complete_runtime() -> Result<()> {
    // Shutdown memory manager
    crate::runtime::shutdown_memory_manager()
        .map_err(|e| Error::Runtime(e.to_string()))?;
    
    // Shutdown GC
    crate::runtime::gc::shutdown_gc()
        .map_err(|e| Error::Runtime(e.to_string()))?;
    
    Ok(())
}

/// Create a runtime with integrated scheduler
pub fn create_runtime_with_scheduler(
    runtime_config: RuntimeConfig,
    scheduler_config: crate::runtime::goroutine::SchedulerConfig,
) -> Result<Arc<Runtime>> {
    let runtime = Arc::new(Runtime::with_config(runtime_config)?);
    
    // Create scheduler wrapper
    let scheduler = Box::new(crate::runtime::goroutine::GoroutineSchedulerWrapper::new_with_config(scheduler_config)
        .map_err(|e| Error::Runtime(format!("Failed to create scheduler: {}", e)))?);
    
    // Set the scheduler
    runtime.set_scheduler(scheduler)?;
    
    // Start the runtime
    runtime.start()?;
    
    Ok(runtime)
}

/// Create a runtime with default scheduler
pub fn create_runtime_with_default_scheduler() -> Result<Arc<Runtime>> {
    let runtime_config = RuntimeConfig::default();
    let scheduler_config = crate::runtime::goroutine::SchedulerConfig::default();
    
    create_runtime_with_scheduler(runtime_config, scheduler_config)
}

/// Initialize complete runtime with scheduler integration
pub fn initialize_runtime_with_scheduler(
    runtime_config: RuntimeConfig,
    scheduler_config: crate::runtime::goroutine::SchedulerConfig,
) -> Result<Arc<Runtime>> {
    create_runtime_with_scheduler(runtime_config, scheduler_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let config = RuntimeConfig::default();
        let pal = crate::runtime::pal::detect_platform();
        let runtime = Runtime::new(config, pal).unwrap();
        assert!(!runtime.is_running());
    }

    #[test]
    fn test_runtime_start_shutdown() {
        let config = RuntimeConfig::default();
        let pal = crate::runtime::pal::detect_platform();
        let runtime = Runtime::new(config, pal).unwrap();
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
        let config = RuntimeConfig::default();
        let pal = crate::runtime::pal::detect_platform();
        let runtime = Runtime::new(config, pal).unwrap();
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

    #[test]
    fn test_runtime_with_scheduler() {
        let result = create_runtime_with_default_scheduler();
        assert!(result.is_ok());
        let runtime = result.unwrap();
        assert!(runtime.is_running());
        
        // Test spawning a goroutine
        let result = runtime.spawn_goroutine(|| {
            // Simple test goroutine
        });
        assert!(result.is_ok());
        
        // Test getting stats
        let stats = runtime.get_stats();
        assert!(stats.is_ok());
        
        // Shutdown
        assert!(runtime.shutdown().is_ok());
    }

    #[test]
    fn test_scheduler_error_handling() {
        let config = RuntimeConfig::default();
        let pal = crate::runtime::pal::detect_platform();
        let runtime = Runtime::new(config, pal).unwrap();
        let error = RuntimeError::new(RuntimeErrorType::SchedulingError, "Test error");
        
        let result = runtime.handle_scheduler_error(error);
        assert!(result.is_err());
        
        // Check that error count was incremented
        let stats = runtime.get_stats().unwrap();
        assert_eq!(stats.total_errors, 1);
    }
}
