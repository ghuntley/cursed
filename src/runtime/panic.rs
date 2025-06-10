/// Panic and recovery runtime system for CURSED
///
/// Provides panic handling with customizable behavior, recovery mechanisms,
/// stack frame tracking, and thread-safe operations for concurrent environments.

use crate::error::{Error as CursedError, SourceLocation};
use crate::runtime::debug_info::{EnhancedStackTrace, StackTraceCapture, StackTraceConfig};
use crate::runtime::debug_manager::DebugManager;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, OnceLock};
use std::thread::{self, ThreadId};
use std::time::{Duration, Instant, SystemTime};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::backtrace::{Backtrace, BacktraceStatus};
use std::fmt;
use std::panic::{self, PanicHookInfo};
use std::any::Any;

/// Global panic runtime instance
static PANIC_RUNTIME: OnceLock<Arc<PanicRuntime>> = OnceLock::new();

/// Global panic ID counter for tracking individual panic instances
static PANIC_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generate a unique panic ID
fn next_panic_id() -> u64 {
    PANIC_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Panic severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PanicSeverity {
    /// Recoverable error that can be caught
    Recoverable,
    /// Critical error that should terminate the goroutine
    Critical,
    /// Fatal error that should terminate the entire program
    Fatal,
}

/// Panic category for classification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PanicCategory {
    /// Memory-related panics (out of memory, null pointer, etc.)
    Memory,
    /// Type assertion failures
    TypeAssertion,
    /// Array/slice bounds violations
    BoundsCheck,
    /// Division by zero or other arithmetic errors
    Arithmetic,
    /// Channel operations on closed channels
    Channel,
    /// Goroutine-related panics
    Goroutine,
    /// User-initiated panics (explicit panic calls)
    User,
    /// System-level panics (OS errors, etc.)
    System,
    /// Generic/unknown panic category
    Generic,
}

/// Stack frame information for panic debugging
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Function name (if available)
    pub function_name: Option<String>,
    /// Source file location
    pub source_location: Option<SourceLocation>,
    /// Module or package name
    pub module_name: Option<String>,
    /// Raw instruction pointer
    pub instruction_pointer: Option<usize>,
}

impl StackFrame {
    pub fn new() -> Self {
        StackFrame {
            function_name: None,
            source_location: None,
            module_name: None,
            instruction_pointer: None,
        }
    }

    pub fn with_function(mut self, name: &str) -> Self {
        self.function_name = Some(name.to_string());
        self
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    }

    pub fn with_module(mut self, name: &str) -> Self {
        self.module_name = Some(name.to_string());
        self
    }
}

impl fmt::Display for StackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(func) = &self.function_name {
            if let Some(loc) = &self.source_location {
                write!(f, "{} at {}", func, loc)
            } else {
                write!(f, "{}", func)
            }
        } else if let Some(loc) = &self.source_location {
            write!(f, "unknown function at {}", loc)
        } else {
            write!(f, "unknown location")
        }
    }
}

/// Comprehensive panic information
#[derive(Debug)]
pub struct CursedPanicInfo {
    /// Unique identifier for this panic
    pub panic_id: u64,
    /// Panic message
    pub message: String,
    /// Panic severity level
    pub severity: PanicSeverity,
    /// Panic category
    pub category: PanicCategory,
    /// Thread ID where panic occurred
    pub thread_id: ThreadId,
    /// Goroutine ID if panic occurred in a goroutine
    pub goroutine_id: Option<u64>,
    /// Timestamp when panic occurred
    pub timestamp: SystemTime,
    /// Source location where panic originated
    pub source_location: Option<SourceLocation>,
    /// Stack trace at time of panic
    pub stack_trace: Vec<StackFrame>,
    /// Enhanced stack trace with debug information
    pub enhanced_stack_trace: Option<EnhancedStackTrace>,
    /// Rust backtrace (if available)
    pub rust_backtrace: Option<Backtrace>,
    /// Custom metadata associated with the panic
    pub metadata: HashMap<String, String>,
}

impl CursedPanicInfo {
    pub fn new(message: String, severity: PanicSeverity, category: PanicCategory) -> Self {
        CursedPanicInfo {
            panic_id: next_panic_id(),
            message,
            severity,
            category,
            thread_id: thread::current().id(),
            goroutine_id: None,
            timestamp: SystemTime::now(),
            source_location: None,
            stack_trace: Vec::new(),
            enhanced_stack_trace: None,
            rust_backtrace: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    }

    pub fn with_goroutine(mut self, goroutine_id: u64) -> Self {
        self.goroutine_id = Some(goroutine_id);
        self
    }

    pub fn with_stack_trace(mut self, stack_trace: Vec<StackFrame>) -> Self {
        self.stack_trace = stack_trace;
        self
    }

    pub fn with_enhanced_stack_trace(mut self, enhanced_stack_trace: EnhancedStackTrace) -> Self {
        self.enhanced_stack_trace = Some(enhanced_stack_trace);
        self
    }

    pub fn with_backtrace(mut self, backtrace: Backtrace) -> Self {
        self.rust_backtrace = Some(backtrace);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl fmt::Display for CursedPanicInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Panic #{} [{:?}] {:?}: {}", 
                self.panic_id, self.severity, self.category, self.message)?;
        
        if let Some(location) = &self.source_location {
            writeln!(f, "  at {}", location)?;
        }
        
        if let Some(goroutine_id) = self.goroutine_id {
            writeln!(f, "  in goroutine #{}", goroutine_id)?;
        }
        
        if let Some(enhanced_trace) = &self.enhanced_stack_trace {
            writeln!(f, "Enhanced stack trace:")?;
            writeln!(f, "{}", enhanced_trace)?;
        } else if !self.stack_trace.is_empty() {
            writeln!(f, "Stack trace:")?;
            for (i, frame) in self.stack_trace.iter().enumerate() {
                writeln!(f, "  {}: {}", i, frame)?;
            }
        }
        
        Ok(())
    }
}

/// Recovery action to take when a panic is caught
#[derive(Debug)]
pub enum RecoveryAction {
    /// Continue execution, treating panic as a regular error
    Continue(CursedError),
    /// Terminate the current goroutine cleanly
    TerminateGoroutine,
    /// Restart the current operation
    Retry,
    /// Escalate to a higher-level panic
    Escalate(CursedPanicInfo),
}

/// Recovery handler function type
pub type RecoveryHandler = Box<dyn Fn(&CursedPanicInfo) -> RecoveryAction + Send + Sync>;

/// Panic behavior configuration
#[derive(Debug)]
pub struct PanicConfig {
    /// Whether to capture Rust backtraces
    pub capture_backtraces: bool,
    /// Whether to capture CURSED stack traces
    pub capture_stack_traces: bool,
    /// Maximum stack trace depth
    pub max_stack_depth: usize,
    /// Whether to log panics to stderr
    pub log_to_stderr: bool,
    /// Whether to abort on fatal panics
    pub abort_on_fatal: bool,
    /// Default recovery behavior for uncaught panics
    pub default_recovery: RecoveryAction,
    /// Timeout for recovery operations
    pub recovery_timeout: Duration,
    /// Debug manager for enhanced stack traces
    pub debug_manager: Option<Arc<DebugManager>>,
    /// Stack trace capture configuration
    pub stack_trace_config: StackTraceConfig,
}

impl Default for PanicConfig {
    fn default() -> Self {
        PanicConfig {
            capture_backtraces: true,
            capture_stack_traces: true,
            max_stack_depth: 100,
            log_to_stderr: true,
            abort_on_fatal: true,
            default_recovery: RecoveryAction::TerminateGoroutine,
            recovery_timeout: Duration::from_secs(30),
            debug_manager: None,
            stack_trace_config: StackTraceConfig::default(),
        }
    }
}

/// Per-thread panic state
struct ThreadPanicState {
    /// Currently active panic (if any)
    current_panic: Option<CursedPanicInfo>,
    /// Recovery handler stack
    recovery_handlers: Vec<RecoveryHandler>,
    /// Whether thread is in recovery mode
    in_recovery: bool,
    /// Recovery attempt count
    recovery_attempts: u32,
}

impl ThreadPanicState {
    fn new() -> Self {
        ThreadPanicState {
            current_panic: None,
            recovery_handlers: Vec::new(),
            in_recovery: false,
            recovery_attempts: 0,
        }
    }
}

/// Main panic runtime system
pub struct PanicRuntime {
    /// Configuration for panic behavior
    config: Arc<RwLock<PanicConfig>>,
    /// Per-thread panic states
    thread_states: Arc<Mutex<HashMap<ThreadId, ThreadPanicState>>>,
    /// Global recovery handlers
    global_handlers: Arc<RwLock<Vec<RecoveryHandler>>>,
    /// Panic statistics
    stats: Arc<Mutex<PanicStatistics>>,
    /// Whether the runtime is active
    active: AtomicBool,
}

/// Panic statistics for monitoring
#[derive(Debug, Default, Clone)]
pub struct PanicStatistics {
    /// Total number of panics
    pub total_panics: u64,
    /// Panics by category
    pub panics_by_category: HashMap<PanicCategory, u64>,
    /// Panics by severity
    pub panics_by_severity: HashMap<PanicSeverity, u64>,
    /// Successful recoveries
    pub successful_recoveries: u64,
    /// Failed recovery attempts
    pub failed_recoveries: u64,
    /// Average recovery time
    pub average_recovery_time: Duration,
}

impl PanicRuntime {
    /// Create a new panic runtime with default configuration
    pub fn new() -> Self {
        PanicRuntime {
            config: Arc::new(RwLock::new(PanicConfig::default())),
            thread_states: Arc::new(Mutex::new(HashMap::new())),
            global_handlers: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(Mutex::new(PanicStatistics::default())),
            active: AtomicBool::new(false),
        }
    }

    /// Create panic runtime with custom configuration
    pub fn with_config(config: PanicConfig) -> Self {
        PanicRuntime {
            config: Arc::new(RwLock::new(config)),
            thread_states: Arc::new(Mutex::new(HashMap::new())),
            global_handlers: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(Mutex::new(PanicStatistics::default())),
            active: AtomicBool::new(false),
        }
    }

    /// Initialize the panic runtime system
    pub fn initialize(&self) -> Result<(), CursedError> {
        if self.active.load(Ordering::SeqCst) {
            return Err(CursedError::Runtime("Panic runtime already initialized".to_string()));
        }

        // Set up Rust panic hook to integrate with our system
        let stats_clone = Arc::clone(&self.stats);
        let config_clone = Arc::clone(&self.config);
        
        panic::set_hook(Box::new(move |panic_info| {
            let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };

            let config = config_clone.read().unwrap();
            if config.log_to_stderr {
                eprintln!("CURSED panic: {}", message);
                if let Some(location) = panic_info.location() {
                    eprintln!("  at {}:{}:{}", location.file(), location.line(), location.column());
                }
            }

            // Update statistics
            if let Ok(mut stats) = stats_clone.lock() {
                stats.total_panics += 1;
                *stats.panics_by_category.entry(PanicCategory::System).or_insert(0) += 1;
                *stats.panics_by_severity.entry(PanicSeverity::Critical).or_insert(0) += 1;
            }
        }));

        self.active.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Shutdown the panic runtime system
    pub fn shutdown(&self) -> Result<(), CursedError> {
        if !self.active.load(Ordering::SeqCst) {
            return Ok(());
        }

        // Reset panic hook to default
        let _ = panic::take_hook();

        // Clear all thread states
        if let Ok(mut states) = self.thread_states.lock() {
            states.clear();
        }

        self.active.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Trigger a panic with specified information
    pub fn panic(&self, mut panic_info: CursedPanicInfo) -> ! {
        let thread_id = thread::current().id();
        
        // Capture enhanced stack trace before moving panic_info
        if let Some(enhanced_trace) = self.capture_enhanced_stack_trace(panic_info.goroutine_id) {
            panic_info = panic_info.with_enhanced_stack_trace(enhanced_trace);
        }
        
        // Store values we need before moving panic_info
        let severity = panic_info.severity;
        let message = panic_info.message.clone();
        let category = panic_info.category.clone();
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_panics += 1;
            *stats.panics_by_category.entry(category).or_insert(0) += 1;
            *stats.panics_by_severity.entry(severity).or_insert(0) += 1;
        }

        // Log panic if configured
        let config = self.config.read().unwrap();
        if config.log_to_stderr {
            eprintln!("{}", panic_info);
        }

        // Store panic info in thread state (move ownership)
        if let Ok(mut states) = self.thread_states.lock() {
            let state = states.entry(thread_id).or_insert_with(ThreadPanicState::new);
            state.current_panic = Some(panic_info);
        }

        // Handle based on severity
        match severity {
            PanicSeverity::Fatal if config.abort_on_fatal => {
                std::process::abort();
            }
            _ => {
                panic!("{}", message);
            }
        }
    }

    /// Attempt to recover from a panic
    pub fn recover<T, F>(&self, operation: F) -> Result<T, CursedError>
    where
        F: FnOnce() -> T + std::panic::UnwindSafe,
    {
        let thread_id = thread::current().id();
        let start_time = Instant::now();

        // Set recovery mode
        if let Ok(mut states) = self.thread_states.lock() {
            let state = states.entry(thread_id).or_insert_with(ThreadPanicState::new);
            state.in_recovery = true;
            state.recovery_attempts += 1;
        }

        let result = panic::catch_unwind(operation);
        
        let recovery_time = start_time.elapsed();

        // Process recovery result
        match result {
            Ok(value) => {
                // Successful execution
                if let Ok(mut stats) = self.stats.lock() {
                    stats.successful_recoveries += 1;
                    // Update average recovery time
                    let total_time = stats.average_recovery_time.as_nanos() as u64 * stats.successful_recoveries.saturating_sub(1)
                        + recovery_time.as_nanos() as u64;
                    stats.average_recovery_time = Duration::from_nanos(total_time / stats.successful_recoveries);
                }
                
                // Reset recovery mode
                if let Ok(mut states) = self.thread_states.lock() {
                    if let Some(state) = states.get_mut(&thread_id) {
                        state.in_recovery = false;
                        state.current_panic = None;
                    }
                }
                
                Ok(value)
            }
            Err(panic_payload) => {
                // Panic occurred, attempt recovery
                if let Ok(mut stats) = self.stats.lock() {
                    stats.failed_recoveries += 1;
                }

                // Extract panic message
                let message = if let Some(s) = panic_payload.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic_payload.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic during recovery".to_string()
                };

                // Reset recovery mode
                if let Ok(mut states) = self.thread_states.lock() {
                    if let Some(state) = states.get_mut(&thread_id) {
                        state.in_recovery = false;
                    }
                }

                Err(CursedError::Runtime(format!("Panic recovery failed: {}", message)))
            }
        }
    }

    /// Register a recovery handler for the current thread
    pub fn register_recovery_handler<F>(&self, handler: F) -> Result<(), CursedError>
    where
        F: Fn(&CursedPanicInfo) -> RecoveryAction + Send + Sync + 'static,
    {
        let thread_id = thread::current().id();
        
        if let Ok(mut states) = self.thread_states.lock() {
            let state = states.entry(thread_id).or_insert_with(ThreadPanicState::new);
            state.recovery_handlers.push(Box::new(handler));
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to register recovery handler".to_string()))
        }
    }

    /// Register a global recovery handler
    pub fn register_global_handler<F>(&self, handler: F) -> Result<(), CursedError>
    where
        F: Fn(&CursedPanicInfo) -> RecoveryAction + Send + Sync + 'static,
    {
        if let Ok(mut handlers) = self.global_handlers.write() {
            handlers.push(Box::new(handler));
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to register global handler".to_string()))
        }
    }

    /// Get current panic information for the thread
    pub fn get_current_panic(&self) -> Option<CursedPanicInfo> {
        let thread_id = thread::current().id();
        
        if let Ok(states) = self.thread_states.lock() {
            // Since CursedPanicInfo doesn't implement Clone, we return a reference
            // For now, return None to indicate panic info is not available
            // In a full implementation, we might need to restructure this
            None
        } else {
            None
        }
    }

    /// Check if current thread is in recovery mode
    pub fn is_in_recovery(&self) -> bool {
        let thread_id = thread::current().id();
        
        if let Ok(states) = self.thread_states.lock() {
            states.get(&thread_id).map(|s| s.in_recovery).unwrap_or(false)
        } else {
            false
        }
    }

    /// Get panic statistics
    pub fn get_statistics(&self) -> Result<PanicStatistics, CursedError> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::Runtime("Failed to access panic statistics".to_string()))
    }

    /// Update panic configuration
    pub fn update_config<F>(&self, updater: F) -> Result<(), CursedError>
    where
        F: FnOnce(&mut PanicConfig),
    {
        if let Ok(mut config) = self.config.write() {
            updater(&mut *config);
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to update panic configuration".to_string()))
        }
    }

    /// Create a basic stack trace (placeholder implementation)
    pub fn capture_stack_trace(&self, max_depth: usize) -> Vec<StackFrame> {
        let mut frames = Vec::new();
        
        // For now, create a basic frame with current location
        // In a full implementation, this would walk the actual call stack
        let frame = StackFrame::new()
            .with_function("unknown")
            .with_module("cursed::runtime");
        
        frames.push(frame);
        
        // Limit to max_depth
        frames.truncate(max_depth);
        frames
    }

    /// Capture enhanced stack trace with debug information
    pub fn capture_enhanced_stack_trace(&self, goroutine_id: Option<u64>) -> Option<EnhancedStackTrace> {
        let config = self.config.read().ok()?;
        
        if let Some(debug_manager) = &config.debug_manager {
            let capture = StackTraceCapture::new()
                .with_config(config.stack_trace_config.clone())
                .with_debug_manager(Arc::clone(debug_manager));
            
            capture.capture_with_context(goroutine_id).ok()
        } else {
            // Fallback to basic capture
            let capture = StackTraceCapture::new()
                .with_config(config.stack_trace_config.clone());
            
            capture.capture_with_context(goroutine_id).ok()
        }
    }

    /// Set debug manager for enhanced stack traces
    pub fn set_debug_manager(&self, debug_manager: Arc<DebugManager>) -> Result<(), CursedError> {
        if let Ok(mut config) = self.config.write() {
            config.debug_manager = Some(debug_manager);
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to set debug manager".to_string()))
        }
    }
}

impl Default for PanicRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize the global panic runtime
pub fn initialize_panic_runtime() -> Result<(), CursedError> {
    let runtime = Arc::new(PanicRuntime::new());
    runtime.initialize()?;
    
    PANIC_RUNTIME.set(runtime)
        .map_err(|_| CursedError::Runtime("Failed to initialize panic runtime".to_string()))?;
    
    Ok(())
}

/// Get the global panic runtime
pub fn get_panic_runtime() -> Option<&'static Arc<PanicRuntime>> {
    PANIC_RUNTIME.get()
}

/// Shutdown the global panic runtime
pub fn shutdown_panic_runtime() -> Result<(), CursedError> {
    if let Some(runtime) = get_panic_runtime() {
        runtime.shutdown()
    } else {
        Ok(())
    }
}

// Gen Z slang panic functions for style consistency

/// Trigger a panic with Gen Z slang - "no cap" means "no lie/for real"
pub fn no_cap_panic(message: &str) -> ! {
    let panic_info = CursedPanicInfo::new(
        format!("no cap: {}", message),
        PanicSeverity::Critical,
        PanicCategory::User
    );
    
    if let Some(runtime) = get_panic_runtime() {
        runtime.panic(panic_info);
    } else {
        panic!("no cap: {}", message);
    }
}

/// Trigger a panic indicating something is "sus" (suspicious)
pub fn sus_panic(message: &str) -> ! {
    let panic_info = CursedPanicInfo::new(
        format!("that's sus: {}", message),
        PanicSeverity::Critical,
        PanicCategory::User
    );
    
    if let Some(runtime) = get_panic_runtime() {
        runtime.panic(panic_info);
    } else {
        panic!("that's sus: {}", message);
    }
}

/// Trigger a panic when something is "cap" (lie/false)
pub fn cap_panic(message: &str) -> ! {
    let panic_info = CursedPanicInfo::new(
        format!("cap detected: {}", message),
        PanicSeverity::Critical,
        PanicCategory::User
    );
    
    if let Some(runtime) = get_panic_runtime() {
        runtime.panic(panic_info);
    } else {
        panic!("cap detected: {}", message);
    }
}

/// Trigger a panic when something is "not vibing"
pub fn not_vibing_panic(message: &str) -> ! {
    let panic_info = CursedPanicInfo::new(
        format!("not vibing: {}", message),
        PanicSeverity::Critical,
        PanicCategory::User
    );
    
    if let Some(runtime) = get_panic_runtime() {
        runtime.panic(panic_info);
    } else {
        panic!("not vibing: {}", message);
    }
}

/// Standard panic function for CURSED language
pub fn cursed_panic_with_message(message: &str) -> ! {
    let panic_info = CursedPanicInfo::new(
        message.to_string(),
        PanicSeverity::Critical,
        PanicCategory::User
    );
    
    if let Some(runtime) = get_panic_runtime() {
        runtime.panic(panic_info);
    } else {
        panic!("{}", message);
    }
}

// FFI functions for LLVM integration

/// Trigger a CURSED panic from compiled code
#[no_mangle]
pub extern "C" fn cursed_panic(
    message_ptr: *const u8,
    message_len: usize,
    severity: u8,
    category: u8,
    line: u32,
    column: u32,
    file_ptr: *const u8,
    file_len: usize,
) -> ! {
    // Safety: We trust LLVM-generated code to provide valid pointers and lengths
    let message = if message_ptr.is_null() || message_len == 0 {
        "Unknown panic".to_string()
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(message_ptr, message_len);
            String::from_utf8_lossy(slice).to_string()
        }
    };

    let severity = match severity {
        0 => PanicSeverity::Recoverable,
        1 => PanicSeverity::Critical,
        _ => PanicSeverity::Fatal,
    };

    let category = match category {
        0 => PanicCategory::Memory,
        1 => PanicCategory::TypeAssertion,
        2 => PanicCategory::BoundsCheck,
        3 => PanicCategory::Arithmetic,
        4 => PanicCategory::Channel,
        5 => PanicCategory::Goroutine,
        6 => PanicCategory::User,
        7 => PanicCategory::System,
        _ => PanicCategory::Generic,
    };

    let source_location = if file_ptr.is_null() || file_len == 0 {
        Some(SourceLocation::new(line as usize, column as usize))
    } else {
        unsafe {
            let file_slice = std::slice::from_raw_parts(file_ptr, file_len);
            let file_name = String::from_utf8_lossy(file_slice).to_string();
            Some(SourceLocation::new(line as usize, column as usize).with_file(&file_name))
        }
    };

    let panic_info = CursedPanicInfo::new(message, severity, category)
        .with_location(source_location.unwrap());

    if let Some(runtime) = get_panic_runtime() {
        runtime.panic(panic_info);
    } else {
        // Fallback if runtime not initialized
        panic!("CURSED panic: {}", panic_info.message);
    }
}

/// Attempt recovery from compiled code
#[no_mangle]
pub extern "C" fn cursed_recover() -> u8 {
    if let Some(runtime) = get_panic_runtime() {
        if runtime.is_in_recovery() {
            1 // Recovery in progress
        } else {
            0 // No recovery needed
        }
    } else {
        0 // Runtime not available
    }
}

/// Check if current thread has an active panic
#[no_mangle]
pub extern "C" fn cursed_has_panic() -> u8 {
    if let Some(runtime) = get_panic_runtime() {
        if runtime.get_current_panic().is_some() {
            1 // Panic active
        } else {
            0 // No panic
        }
    } else {
        0 // Runtime not available
    }
}

/// Get panic message (for recovery handlers)
#[no_mangle]
pub extern "C" fn cursed_get_panic_message(
    buffer: *mut u8,
    buffer_len: usize,
) -> usize {
    if buffer.is_null() || buffer_len == 0 {
        return 0;
    }

    let message = if let Some(runtime) = get_panic_runtime() {
        if let Some(panic_info) = runtime.get_current_panic() {
            panic_info.message
        } else {
            return 0;
        }
    } else {
        return 0;
    };

    let message_bytes = message.as_bytes();
    let copy_len = std::cmp::min(message_bytes.len(), buffer_len);
    
    unsafe {
        std::ptr::copy_nonoverlapping(message_bytes.as_ptr(), buffer, copy_len);
    }
    
    copy_len
}

/// Gen Z slang FFI functions for compiled CURSED code

/// Trigger "no cap" panic from compiled code
#[no_mangle]
pub extern "C" fn cursed_no_cap_panic(
    message_ptr: *const u8,
    message_len: usize,
) -> ! {
    let message = if message_ptr.is_null() || message_len == 0 {
        "Something's not right"
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(message_ptr, message_len);
            std::str::from_utf8(slice).unwrap_or("Invalid message")
        }
    };
    
    no_cap_panic(message);
}

/// Trigger "sus" panic from compiled code
#[no_mangle]
pub extern "C" fn cursed_sus_panic(
    message_ptr: *const u8,
    message_len: usize,
) -> ! {
    let message = if message_ptr.is_null() || message_len == 0 {
        "Something suspicious happened"
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(message_ptr, message_len);
            std::str::from_utf8(slice).unwrap_or("Invalid message")
        }
    };
    
    sus_panic(message);
}

/// Trigger "cap" panic from compiled code
#[no_mangle]
pub extern "C" fn cursed_cap_panic(
    message_ptr: *const u8,
    message_len: usize,
) -> ! {
    let message = if message_ptr.is_null() || message_len == 0 {
        "False statement detected"
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(message_ptr, message_len);
            std::str::from_utf8(slice).unwrap_or("Invalid message")
        }
    };
    
    cap_panic(message);
}

/// Trigger "not vibing" panic from compiled code
#[no_mangle]
pub extern "C" fn cursed_not_vibing_panic(
    message_ptr: *const u8,
    message_len: usize,
) -> ! {
    let message = if message_ptr.is_null() || message_len == 0 {
        "Bad vibes detected"
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(message_ptr, message_len);
            std::str::from_utf8(slice).unwrap_or("Invalid message")
        }
    };
    
    not_vibing_panic(message);
}

/// Standard panic function from compiled code
#[no_mangle]
pub extern "C" fn cursed_panic_message(
    message_ptr: *const u8,
    message_len: usize,
) -> ! {
    let message = if message_ptr.is_null() || message_len == 0 {
        "Unknown error"
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(message_ptr, message_len);
            std::str::from_utf8(slice).unwrap_or("Invalid message")
        }
    };
    
    cursed_panic_with_message(message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_panic_info_creation() {
        let panic_info = CursedPanicInfo::new(
            "Test panic".to_string(),
            PanicSeverity::Recoverable,
            PanicCategory::User
        );

        assert_eq!(panic_info.message, "Test panic");
        assert_eq!(panic_info.severity, PanicSeverity::Recoverable);
        assert_eq!(panic_info.category, PanicCategory::User);
        assert!(panic_info.panic_id > 0);
    }

    #[test]
    fn test_runtime_initialization() {
        let runtime = PanicRuntime::new();
        assert!(runtime.initialize().is_ok());
        assert!(runtime.shutdown().is_ok());
    }

    #[test]
    fn test_recovery_success() {
        let runtime = PanicRuntime::new();
        runtime.initialize().unwrap();

        let result = runtime.recover(|| {
            42
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        runtime.shutdown().unwrap();
    }

    #[test]
    fn test_recovery_failure() {
        let runtime = PanicRuntime::new();
        runtime.initialize().unwrap();

        let result = runtime.recover(|| {
            panic!("Test panic");
        });

        assert!(result.is_err());
        runtime.shutdown().unwrap();
    }

    #[test]
    fn test_recovery_handler() {
        let runtime = PanicRuntime::new();
        runtime.initialize().unwrap();

        let handler_called = Arc::new(AtomicBool::new(false));
        let handler_called_clone = Arc::clone(&handler_called);

        let result = runtime.register_recovery_handler(move |_panic_info| {
            handler_called_clone.store(true, Ordering::SeqCst);
            RecoveryAction::Continue(CursedError::Runtime("Recovered".to_string()))
        });

        assert!(result.is_ok());
        runtime.shutdown().unwrap();
    }

    #[test]
    fn test_statistics() {
        let runtime = PanicRuntime::new();
        runtime.initialize().unwrap();

        let stats = runtime.get_statistics().unwrap();
        assert_eq!(stats.total_panics, 0);
        assert_eq!(stats.successful_recoveries, 0);

        runtime.shutdown().unwrap();
    }

    #[test]
    fn test_stack_frame() {
        let frame = StackFrame::new()
            .with_function("test_function")
            .with_location(SourceLocation::new(10, 5))
            .with_module("test_module");

        assert_eq!(frame.function_name, Some("test_function".to_string()));
        assert_eq!(frame.module_name, Some("test_module".to_string()));
        assert!(frame.source_location.is_some());
    }

    #[test]
    fn test_panic_severity_ordering() {
        assert!(PanicSeverity::Recoverable < PanicSeverity::Critical);
        assert!(PanicSeverity::Critical < PanicSeverity::Fatal);
    }

    #[test]
    fn test_global_runtime_initialization() {
        // Note: This test might interfere with other tests if run concurrently
        // In a real environment, the global runtime would be initialized once
        if get_panic_runtime().is_none() {
            assert!(initialize_panic_runtime().is_ok());
            assert!(get_panic_runtime().is_some());
            assert!(shutdown_panic_runtime().is_ok());
        }
    }

    #[test]
    fn test_gen_z_panic_function_creation() {
        // Test that Gen Z slang panic info is created correctly
        let runtime = PanicRuntime::new();
        runtime.initialize().unwrap();

        // We can't directly test the panic functions since they abort execution,
        // but we can test panic info creation and formatting
        let no_cap_info = CursedPanicInfo::new(
            "no cap: test".to_string(),
            PanicSeverity::Critical,
            PanicCategory::User
        );
        assert!(no_cap_info.message.contains("no cap"));

        let sus_info = CursedPanicInfo::new(
            "that's sus: test".to_string(),
            PanicSeverity::Critical,
            PanicCategory::User
        );
        assert!(sus_info.message.contains("that's sus"));

        let cap_info = CursedPanicInfo::new(
            "cap detected: test".to_string(),
            PanicSeverity::Critical,
            PanicCategory::User
        );
        assert!(cap_info.message.contains("cap detected"));

        let not_vibing_info = CursedPanicInfo::new(
            "not vibing: test".to_string(),
            PanicSeverity::Critical,
            PanicCategory::User
        );
        assert!(not_vibing_info.message.contains("not vibing"));

        runtime.shutdown().unwrap();
    }

    #[test]
    fn test_panic_categories() {
        // Test all panic categories are distinct
        assert_ne!(PanicCategory::Memory, PanicCategory::TypeAssertion);
        assert_ne!(PanicCategory::BoundsCheck, PanicCategory::Arithmetic);
        assert_ne!(PanicCategory::Channel, PanicCategory::Goroutine);
        assert_ne!(PanicCategory::User, PanicCategory::System);
        assert_ne!(PanicCategory::Generic, PanicCategory::Memory);
    }

    #[test]
    fn test_panic_info_with_goroutine() {
        let panic_info = CursedPanicInfo::new(
            "Test goroutine panic".to_string(),
            PanicSeverity::Critical,
            PanicCategory::Goroutine
        ).with_goroutine(42);

        assert_eq!(panic_info.goroutine_id, Some(42));
        assert_eq!(panic_info.category, PanicCategory::Goroutine);
    }

    #[test]
    fn test_panic_info_with_metadata() {
        let panic_info = CursedPanicInfo::new(
            "Test panic with metadata".to_string(),
            PanicSeverity::Recoverable,
            PanicCategory::User
        ).with_metadata("context".to_string(), "test_context".to_string())
         .with_metadata("thread".to_string(), "main".to_string());

        assert_eq!(panic_info.metadata.get("context"), Some(&"test_context".to_string()));
        assert_eq!(panic_info.metadata.get("thread"), Some(&"main".to_string()));
    }

    #[test]
    fn test_enhanced_stack_trace_capture() {
        let runtime = PanicRuntime::new();
        runtime.initialize().unwrap();

        // Test basic stack trace capture
        let stack_trace = runtime.capture_stack_trace(5);
        assert!(!stack_trace.is_empty());
        assert!(stack_trace.len() <= 5);

        // Test enhanced stack trace capture (without debug manager)
        let enhanced_trace = runtime.capture_enhanced_stack_trace(None);
        // This may return None if debug manager is not set, which is expected

        runtime.shutdown().unwrap();
    }

    #[test]
    fn test_recovery_action_types() {
        // Test that recovery actions can be created
        let continue_action = RecoveryAction::Continue(
            CursedError::Runtime("Test error".to_string())
        );
        let terminate_action = RecoveryAction::TerminateGoroutine;
        let retry_action = RecoveryAction::Retry;
        
        match continue_action {
            RecoveryAction::Continue(_) => (), // Expected
            _ => panic!("Unexpected recovery action type"),
        }
        
        match terminate_action {
            RecoveryAction::TerminateGoroutine => (), // Expected
            _ => panic!("Unexpected recovery action type"),
        }
        
        match retry_action {
            RecoveryAction::Retry => (), // Expected
            _ => panic!("Unexpected recovery action type"),
        }
    }

    #[test]
    fn test_panic_config_customization() {
        let custom_config = PanicConfig {
            capture_backtraces: false,
            capture_stack_traces: true,
            max_stack_depth: 50,
            log_to_stderr: false,
            abort_on_fatal: false,
            default_recovery: RecoveryAction::Continue(
                CursedError::Runtime("Custom recovery".to_string())
            ),
            recovery_timeout: Duration::from_secs(10),
            debug_manager: None,
            stack_trace_config: StackTraceConfig::default(),
        };

        let runtime = PanicRuntime::with_config(custom_config);
        runtime.initialize().unwrap();
        
        // Test config update
        let result = runtime.update_config(|config| {
            config.max_stack_depth = 25;
            config.log_to_stderr = true;
        });
        
        assert!(result.is_ok());
        runtime.shutdown().unwrap();
    }
}
