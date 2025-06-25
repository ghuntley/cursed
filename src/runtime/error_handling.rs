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
    RecoveryAction, get_panic_runtime
// };
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
/// CursedError propagation context for the `?` operator
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Unique identifier for this error context
    /// Source location where error originated
    /// Chain of error contexts (for nested errors)
    /// Associated goroutine ID (if any)
    /// Thread ID where error occurred
    /// Timestamp when error context was created
    /// Custom metadata for error context
/// Entry in the error chain for tracking error propagation
#[derive(Debug, Clone)]
pub struct ErrorChainEntry {
    /// CursedError message at this level
    /// Source location for this level
    /// Function name where error occurred
    /// Timestamp for this level
impl ErrorContext {
    pub fn new() -> Self {
        ErrorContext {
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    pub fn with_goroutine(mut self, goroutine_id: u64) -> Self {
        self.goroutine_id = Some(goroutine_id);
        self
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    pub fn add_to_chain(&mut self, message: String, location: Option<SourceLocation>, function: Option<String>) {
        self.error_chain.push(ErrorChainEntry {
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
    /// Maximum error chain depth
    /// Whether to log error propagation
    /// Timeout for error propagation operations
    /// Whether to auto-convert errors to panics in certain conditions
impl Default for ErrorPropagationConfig {
    fn default() -> Self {
        ErrorPropagationConfig {
            auto_panic_threshold: Some(100), // Convert to panic after 100 error levels
        }
    }
/// Per-thread error state for tracking error propagation
struct ThreadErrorState {
    /// Current error context stack
    /// Number of active error propagations
    /// Whether thread is in error handling mode
    /// CursedError propagation statistics
    /// Last error timestamp
impl ThreadErrorState {
    fn new() -> Self {
        ThreadErrorState {
        }
    }
/// CursedError handling statistics
#[derive(Debug, Default, Clone)]
pub struct ErrorHandlingStatistics {
    /// Total number of errors handled
    /// Number of successful error propagations
    /// Number of failed error propagations
    /// Number of errors converted to panics
    /// Average error propagation time
    /// Errors by category
    /// CursedError chain depth statistics
/// Main CursedError Handling Runtime System
pub struct ErrorRuntime {
    /// Configuration for error propagation
    /// Per-thread error states
    /// CursedError handling statistics
    /// Integration with panic runtime
    /// Whether the runtime is active
impl ErrorRuntime {
    /// Create a new error runtime with default configuration
    pub fn new() -> Self {
        ErrorRuntime {
        }
    }

    /// Create error runtime with custom configuration
    pub fn with_config(config: ErrorPropagationConfig) -> Self {
        ErrorRuntime {
        }
    }

    /// Initialize the error runtime system
    #[instrument(skip(self))]
    pub fn initialize(&self) -> crate::error::Result<()> {
        if self.active.load(Ordering::SeqCst) {
            return Err(CursedError::Runtime("CursedError runtime already initialized".to_string()));
        info!("Initializing CURSED error handling runtime");
        self.active.store(true, Ordering::SeqCst);
        Ok(())
    /// Shutdown the error runtime system
    #[instrument(skip(self))]
    pub fn shutdown(&self) -> crate::error::Result<()> {
        if !self.active.load(Ordering::SeqCst) {
            return Ok(());
        info!("Shutting down CURSED error handling runtime");

        // Clear all thread states
        if let Ok(mut states) = self.thread_states.lock() {
            states.clear();
        self.active.store(false, Ordering::SeqCst);
        Ok(())
    /// Propagate an error using the `?` operator semantics
    #[instrument(skip(self, error), fields(error_id = %next_error_id()))]
    pub fn propagate_error(
    ) -> crate::error::Result<()> {
        let thread_id = thread::current().id();
        let start_time = Instant::now();

        debug!("Propagating error: {}", error);

        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_errors += 1;
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
            // Add to error chain
            context.add_to_chain(
            );

            // Check chain depth limits
            let config = self.config.read().unwrap();
            if context.error_chain.len() > config.max_chain_depth {
                warn!(
                    config.max_chain_depth
                );
                should_convert_to_panic = true;
            // Check auto-panic threshold
            if let Some(threshold) = config.auto_panic_threshold {
                if state.active_propagations as usize > threshold {
                    warn!(
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
                // Update average chain depth
                let total_depth = stats.average_chain_depth * (stats.successful_propagations - 1) as f64 + chain_depth as f64;
                stats.average_chain_depth = total_depth / stats.successful_propagations as f64;
            state.active_propagations = state.active_propagations.saturating_sub(1);
            state.in_error_handling = false;
        // Convert to panic if necessary
        if should_convert_to_panic {
            self.convert_error_to_panic(error.clone(), source_location, function_name)?;
        Err(error)
    /// Convert an error to a panic (for severe error conditions)
    #[instrument(skip(self, error))]
    pub fn convert_error_to_panic(
    ) -> crate::error::Result<()> {
        warn!("Converting error to panic: {}", error);

        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.errors_to_panics += 1;
        // Create panic info
        let message = if let Some(func) = function_name {
            format!("CursedError in {}: {}", func, error)
        } else {
            format!("CursedError converted to panic: {}", error)

        let category = match &error {

        let mut panic_info = CursedPanicInfo::new(
        );

        if let Some(location) = source_location {
            panic_info = panic_info.with_location(location);
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
    ) -> CursedError {
        match self.propagate_error(error.clone(), source_location, function_name) {
            Ok(()) => {
                // This shouldn't happen as propagate_error always returns Err
                error
            }
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
    /// Update error propagation configuration
    pub fn update_config<F>(&self, updater: F) -> crate::error::Result<()>
    where
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
    /// Create an enhanced error with context information
    pub fn create_contextual_error(
    ) -> CursedError {
        let mut context = ErrorContext::new();
        
        if let Some(location) = source_location {
            context = context.with_location(location);
        context.add_to_chain(
        );
        
        context.add_to_chain(
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
/// Get the global error runtime
pub fn get_error_runtime() -> Option<&'static Arc<ErrorRuntime>> {
    ERROR_RUNTIME.get()
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
) -> u8 {
    // Safety: We trust LLVM-generated code to provide valid pointers and lengths
    let error_message = if error_message_ptr.is_null() || error_message_len == 0 {
        "Unknown error".to_string()
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(error_message_ptr, error_message_len);
            String::from_utf8_lossy(slice).to_string()
        }

    let source_location = if file_ptr.is_null() || file_len == 0 {
        Some(SourceLocation::new(line as usize, column as usize))
    } else {
        unsafe {
            let file_slice = std::slice::from_raw_parts(file_ptr, file_len);
            let file_name = String::from_utf8_lossy(file_slice).to_string();
            Some(SourceLocation::new(line as usize, column as usize).with_file(&file_name))
        }

    let function_name = if function_ptr.is_null() || function_len == 0 {
        None
    } else {
        unsafe {
            let function_slice = std::slice::from_raw_parts(function_ptr, function_len);
            Some(String::from_utf8_lossy(function_slice).to_string())
        }

    // Create appropriate error based on error code
    let error = match error_code {

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
) -> u8 {
    if context_id_out.is_null() || chain_depth_out.is_null() {
        return 0;
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
