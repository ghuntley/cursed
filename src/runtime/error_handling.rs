/// Comprehensive CursedError Handling Runtime for CURSED
///
/// This module provides a complete error handling infrastructure including:
/// - CursedError propagation with the `?` operator
/// - Integration with panic/recovery system
/// - Stack trace generation and management
/// - Thread-safe error coordination
/// - Goroutine-aware error handling

use crate::error::{CursedError, SourceLocation};
use crate::runtime::panic::{
    PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory, 
    RecoveryAction, get_panic_runtime
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, OnceLock};
use std::thread::{self, ThreadId};
use std::time::{Duration, Instant, SystemTime};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::fmt;
use tracing::{debug, error, info, instrument, warn};

/// Global error runtime instance
static ERROR_RUNTIME: OnceLock<Arc<ErrorRuntime>> = OnceLock::new();

/// Global error ID counter for tracking individual error instances
static ERROR_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generate a unique error ID
pub fn next_error_id() -> u64 {
    ERROR_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// CursedError propagation context for the `?` operator
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Unique identifier for this error context
    pub context_id: u64,
    /// Source location where error originated
    pub source_location: Option<SourceLocation>,
    /// Chain of error contexts (for nested errors)
    pub error_chain: Vec<ErrorChainEntry>,
    /// Associated goroutine ID (if any)
    pub goroutine_id: Option<u64>,
    /// Thread ID where error occurred
    pub thread_id: ThreadId,
    /// Timestamp when error context was created
    pub timestamp: SystemTime,
    /// Custom metadata for error context
    pub metadata: HashMap<String, String>,
}

/// Entry in the error chain for tracking error propagation
#[derive(Debug, Clone)]
pub struct ErrorChainEntry {
    /// CursedError message at this level
    pub message: String,
    /// Source location for this level
    pub source_location: Option<SourceLocation>,
    /// Function name where error occurred
    pub function_name: Option<String>,
    /// Timestamp for this level
    pub timestamp: SystemTime,
}

impl ErrorContext {
    pub fn new() -> Self {
        ErrorContext {
            context_id: next_error_id(),
            source_location: None,
            error_chain: Vec::new(),
            goroutine_id: None,
            thread_id: thread::current().id(),
            timestamp: SystemTime::now(),
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

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn add_to_chain(&mut self, message: String, location: Option<SourceLocation>, function: Option<String>) {
        self.error_chain.push(ErrorChainEntry {
            message,
            source_location: location,
            function_name: function,
            timestamp: SystemTime::now(),
        });
    }
}

// impl fmt::Display for ErrorContext {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "CursedError Context #{}", self.context_id)?;
//         
//         if let Some(location) = &self.source_location {
//             writeln!(f, "  at {}", location)?;
//         }
//         
//         if let Some(goroutine_id) = self.goroutine_id {
//             writeln!(f, "  in goroutine #{}", goroutine_id)?;
//         }
//         
//         if !self.error_chain.is_empty() {
//             writeln!(f, "CursedError chain:")?;
//             for (i, entry) in self.error_chain.iter().enumerate() {
//                 write!(f, "  {}: {}", i, entry.message)?;
//                 if let Some(loc) = &entry.source_location {
//                     write!(f, " at {}", loc)?;
//                 }
//                 if let Some(func) = &entry.function_name {
//                     write!(f, " in {}", func)?;
//                 }
//                 writeln!(f)?;
//             }
//         }
//         
//         Ok(())
//     }
// }

/// CursedError propagation configuration
#[derive(Debug, Clone)]
pub struct ErrorPropagationConfig {
    /// Whether to capture stack traces on error propagation
    pub capture_stack_traces: bool,
    /// Maximum error chain depth
    pub max_chain_depth: usize,
    /// Whether to log error propagation
    pub log_propagation: bool,
    /// Timeout for error propagation operations
    pub propagation_timeout: Duration,
    /// Whether to auto-convert errors to panics in certain conditions
    pub auto_panic_threshold: Option<usize>,
}

impl Default for ErrorPropagationConfig {
    fn default() -> Self {
        ErrorPropagationConfig {
            capture_stack_traces: true,
            max_chain_depth: 50,
            log_propagation: false,
            propagation_timeout: Duration::from_secs(5),
            auto_panic_threshold: Some(100), // Convert to panic after 100 error levels
        }
    }
}

/// Per-thread error state for tracking error propagation
struct ThreadErrorState {
    /// Current error context stack
    error_contexts: Vec<ErrorContext>,
    /// Number of active error propagations
    active_propagations: u32,
    /// Whether thread is in error handling mode
    in_error_handling: bool,
    /// CursedError propagation statistics
    propagation_count: u64,
    /// Last error timestamp
    last_error_time: Option<SystemTime>,
}

impl ThreadErrorState {
    fn new() -> Self {
        ThreadErrorState {
            error_contexts: Vec::new(),
            active_propagations: 0,
            in_error_handling: false,
            propagation_count: 0,
            last_error_time: None,
        }
    }
}

/// CursedError handling statistics
#[derive(Debug, Default, Clone)]
pub struct ErrorHandlingStatistics {
    /// Total number of errors handled
    pub total_errors: u64,
    /// Number of successful error propagations
    pub successful_propagations: u64,
    /// Number of failed error propagations
    pub failed_propagations: u64,
    /// Number of errors converted to panics
    pub errors_to_panics: u64,
    /// Average error propagation time
    pub average_propagation_time: Duration,
    /// Errors by category
    pub errors_by_category: HashMap<String, u64>,
    /// CursedError chain depth statistics
    pub max_chain_depth: usize,
    pub average_chain_depth: f64,
}

/// Main CursedError Handling Runtime System
pub struct ErrorRuntime {
    /// Configuration for error propagation
    config: Arc<RwLock<ErrorPropagationConfig>>,
    /// Per-thread error states
    thread_states: Arc<Mutex<HashMap<ThreadId, ThreadErrorState>>>,
    /// CursedError handling statistics
    stats: Arc<Mutex<ErrorHandlingStatistics>>,
    /// Integration with panic runtime
    panic_runtime: Option<Arc<PanicRuntime>>,
    /// Whether the runtime is active
    active: AtomicBool,
}

impl ErrorRuntime {
    /// Create a new error runtime with default configuration
    pub fn new() -> Self {
        ErrorRuntime {
            config: Arc::new(RwLock::new(ErrorPropagationConfig::default())),
            thread_states: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(ErrorHandlingStatistics::default())),
            panic_runtime: get_panic_runtime().map(|rt| Arc::clone(rt)),
            active: AtomicBool::new(false),
        }
    }

    /// Create error runtime with custom configuration
    pub fn with_config(config: ErrorPropagationConfig) -> Self {
        ErrorRuntime {
            config: Arc::new(RwLock::new(config)),
            thread_states: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(ErrorHandlingStatistics::default())),
            panic_runtime: get_panic_runtime().map(|rt| Arc::clone(rt)),
            active: AtomicBool::new(false),
        }
    }

    /// Initialize the error runtime system
    #[instrument(skip(self))]
    pub fn initialize(&self) -> crate::error::Result<()> {
        if self.active.load(Ordering::SeqCst) {
            return Err(CursedError::Runtime("CursedError runtime already initialized".to_string()));
        }

        info!("Initializing CURSED error handling runtime");
        self.active.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Shutdown the error runtime system
    #[instrument(skip(self))]
    pub fn shutdown(&self) -> crate::error::Result<()> {
        if !self.active.load(Ordering::SeqCst) {
            return Ok(());
        }

        info!("Shutting down CURSED error handling runtime");

        // Clear all thread states
        if let Ok(mut states) = self.thread_states.lock() {
            states.clear();
        }

        self.active.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Propagate an error using the `?` operator semantics
    #[instrument(skip(self, error), fields(error_id = %next_error_id()))]
    pub fn propagate_error(
        &self,
        error: CursedError,
        source_location: Option<SourceLocation>,
        function_name: Option<String>,
    ) -> crate::error::Result<()> {
        let thread_id = thread::current().id();
        let start_time = Instant::now();

        debug!("Propagating error: {}", error);

        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_errors += 1;
        }

        // Get or create thread state
        let mut should_convert_to_panic = false;
        if let Ok(mut states) = self.thread_states.lock() {
            let state = states.entry(thread_id).or_insert_with(ThreadErrorState::new);
            state.active_propagations += 1;
            state.propagation_count += 1;
            state.last_error_time = Some(SystemTime::now());
            state.in_error_handling = true;

            // Create error context
            let mut context = ErrorContext::new();
            if let Some(location) = source_location.clone() {
                context = context.with_location(location.clone());
            }

            // Add to error chain
            context.add_to_chain(
                error.to_string(),
                source_location.clone(),
                function_name.clone(),
            );

            // Check chain depth limits
            let config = self.config.read().unwrap();
            if context.error_chain.len() > config.max_chain_depth {
                warn!(
                    "CursedError chain depth ({}) exceeds maximum ({})",
                    context.error_chain.len(),
                    config.max_chain_depth
                );
                should_convert_to_panic = true;
            }

            // Check auto-panic threshold
            if let Some(threshold) = config.auto_panic_threshold {
                if state.active_propagations as usize > threshold {
                    warn!(
                        "Active propagations ({}) exceed threshold ({}), converting to panic",
                        state.active_propagations,
                        threshold
                    );
                    should_convert_to_panic = true;
                }
            }

            state.error_contexts.push(context);

            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.successful_propagations += 1;
                
                let propagation_time = start_time.elapsed();
                let total_time = stats.average_propagation_time.as_nanos() as u64 * stats.successful_propagations.saturating_sub(1)
                    + propagation_time.as_nanos() as u64;
                stats.average_propagation_time = Duration::from_nanos(total_time / stats.successful_propagations);

                // Update chain depth statistics
                let chain_depth = state.error_contexts.last().map(|ctx| ctx.error_chain.len()).unwrap_or(0);
                if chain_depth > stats.max_chain_depth {
                    stats.max_chain_depth = chain_depth;
                }
                
                // Update average chain depth
                let total_depth = stats.average_chain_depth * (stats.successful_propagations - 1) as f64 + chain_depth as f64;
                stats.average_chain_depth = total_depth / stats.successful_propagations as f64;
            }

            state.active_propagations = state.active_propagations.saturating_sub(1);
            state.in_error_handling = false;
        }

        // Convert to panic if necessary
        if should_convert_to_panic {
            self.convert_error_to_panic(error.clone(), source_location, function_name)?;
        }

        Err(error)
    }

    /// Convert an error to a panic (for severe error conditions)
    #[instrument(skip(self, error))]
    pub fn convert_error_to_panic(
        &self,
        error: CursedError,
        source_location: Option<SourceLocation>,
        function_name: Option<String>,
    ) -> crate::error::Result<()> {
        warn!("Converting error to panic: {}", error);

        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.errors_to_panics += 1;
        }

        // Create panic info
        let message = if let Some(func) = function_name {
            format!("CursedError in {}: {}", func, error)
        } else {
            format!("CursedError converted to panic: {}", error)
        };

        let category = match &error {
            CursedError::Type(_) => PanicCategory::TypeAssertion,
            CursedError::Runtime(_) => PanicCategory::System,
            CursedError::Parse(_) | CursedError::Compile(_) => PanicCategory::User,
            _ => PanicCategory::Generic,
        };

        let mut panic_info = CursedPanicInfo::new(
            message,
            PanicSeverity::Critical,
            category,
        );

        if let Some(location) = source_location {
            panic_info = panic_info.with_location(location);
        }

        // Use panic runtime if available
        if let Some(panic_runtime) = &self.panic_runtime {
            panic_runtime.panic(panic_info);
        } else {
            // Fallback panic
            panic!("CURSED error converted to panic: {}", error);
        }
    }

    /// Handle error in current context (for `?` operator implementation)
    #[instrument(skip(self, error))]
    pub fn handle_question_mark_error(
        &self,
        error: CursedError,
        source_location: Option<SourceLocation>,
        function_name: Option<String>,
    ) -> CursedError {
        match self.propagate_error(error.clone(), source_location, function_name) {
            Ok(()) => {
                // This shouldn't happen as propagate_error always returns Err
                error
            }
            Err(propagated_error) => propagated_error,
        }
    }

    /// Check if current thread is in error handling mode
    pub fn is_in_error_handling(&self) -> bool {
        let thread_id = thread::current().id();
        
        if let Ok(states) = self.thread_states.lock() {
            states.get(&thread_id).map(|s| s.in_error_handling).unwrap_or(false)
        } else {
            false
        }
    }

    /// Get current error context for thread
    pub fn get_current_error_context(&self) -> Option<ErrorContext> {
        let thread_id = thread::current().id();
        
        if let Ok(states) = self.thread_states.lock() {
            states.get(&thread_id)
                .and_then(|s| s.error_contexts.last())
                .cloned()
        } else {
            None
        }
    }

    /// Get error handling statistics
    pub fn get_statistics(&self) -> crate::error::Result<()> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::Runtime("Failed to access error handling statistics".to_string()))
    }

    /// Update error propagation configuration
    pub fn update_config<F>(&self, updater: F) -> crate::error::Result<()>
    where
        F: FnOnce(&mut ErrorPropagationConfig),
    {
        if let Ok(mut config) = self.config.write() {
            updater(&mut *config);
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to update error propagation configuration".to_string()))
        }
    }

    /// Clear error context for current thread (for recovery scenarios)
    #[instrument(skip(self))]
    pub fn clear_error_context(&self) {
        let thread_id = thread::current().id();
        
        if let Ok(mut states) = self.thread_states.lock() {
            if let Some(state) = states.get_mut(&thread_id) {
                state.error_contexts.clear();
                state.active_propagations = 0;
                state.in_error_handling = false;
            }
        }
    }

    /// Create an enhanced error with context information
    pub fn create_contextual_error(
        &self,
        base_error: CursedError,
        additional_context: &str,
        source_location: Option<SourceLocation>,
    ) -> CursedError {
        let mut context = ErrorContext::new();
        
        if let Some(location) = source_location {
            context = context.with_location(location);
        }
        
        context.add_to_chain(
            base_error.to_string(),
            context.source_location.clone(),
            None,
        );
        
        context.add_to_chain(
            additional_context.to_string(),
            context.source_location.clone(),
            None,
        );

        // For now, return the base error with additional context
        // In a full implementation, we might create a new error variant
        CursedError::Runtime(format!("{}: {}", additional_context, base_error))
    }
}

impl Default for ErrorRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize the global error runtime
pub fn initialize_error_runtime() -> crate::error::Result<()> {
    let runtime = Arc::new(ErrorRuntime::new());
    runtime.initialize()?;
    
    ERROR_RUNTIME.set(runtime)
        .map_err(|_| CursedError::Runtime("Failed to initialize error runtime".to_string()))?;
    
    Ok(())
}

/// Get the global error runtime
pub fn get_error_runtime() -> Option<&'static Arc<ErrorRuntime>> {
    ERROR_RUNTIME.get()
}

/// Shutdown the global error runtime
pub fn shutdown_error_runtime() -> crate::error::Result<()> {
    if let Some(runtime) = get_error_runtime() {
        runtime.shutdown()
    } else {
        Ok(())
    }
}

// FFI functions for LLVM integration

/// Handle error propagation from compiled code (for `?` operator)
#[no_mangle]
pub extern "C" fn cursed_propagate_error(
    error_message_ptr: *const u8,
    error_message_len: usize,
    error_code: u32,
    line: u32,
    column: u32,
    file_ptr: *const u8,
    file_len: usize,
    function_ptr: *const u8,
    function_len: usize,
) -> u8 {
    // Safety: We trust LLVM-generated code to provide valid pointers and lengths
    let error_message = if error_message_ptr.is_null() || error_message_len == 0 {
        "Unknown error".to_string()
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(error_message_ptr, error_message_len);
            String::from_utf8_lossy(slice).to_string()
        }
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

    let function_name = if function_ptr.is_null() || function_len == 0 {
        None
    } else {
        unsafe {
            let function_slice = std::slice::from_raw_parts(function_ptr, function_len);
            Some(String::from_utf8_lossy(function_slice).to_string())
        }
    };

    // Create appropriate error based on error code
    let error = match error_code {
        1 => CursedError::Parse(error_message),
        2 => CursedError::Compile(error_message),
        3 => CursedError::Type(error_message),
        _ => CursedError::Runtime(error_message),
    };

    if let Some(runtime) = get_error_runtime() {
        match runtime.propagate_error(error, source_location, function_name) {
            Ok(()) => 0, // Success (shouldn't happen)
            Err(_) => 1,  // CursedError propagated
        }
    } else {
        1 // Runtime not available, indicate error
    }
}

/// Check if thread is in error handling mode
#[no_mangle]
pub extern "C" fn cursed_is_in_error_handling() -> u8 {
    if let Some(runtime) = get_error_runtime() {
        if runtime.is_in_error_handling() {
            1
        } else {
            0
        }
    } else {
        0
    }
}

/// Clear error context for current thread
#[no_mangle]
pub extern "C" fn cursed_clear_error_context() {
    if let Some(runtime) = get_error_runtime() {
        runtime.clear_error_context();
    }
}

/// Get error context information
#[no_mangle]
pub extern "C" fn cursed_get_error_context_info(
    context_id_out: *mut u64,
    chain_depth_out: *mut u32,
) -> u8 {
    if context_id_out.is_null() || chain_depth_out.is_null() {
        return 0;
    }

    if let Some(runtime) = get_error_runtime() {
        if let Some(context) = runtime.get_current_error_context() {
            unsafe {
                *context_id_out = context.context_id;
                *chain_depth_out = context.error_chain.len() as u32;
            }
            return 1;
        }
    }
    
    0
}

