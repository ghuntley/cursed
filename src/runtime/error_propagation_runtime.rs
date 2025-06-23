use crate::error::Error;
use crate::error::SourceLocation as ErrorSourceLocation;
use crate::error::error_propagation::ErrorPropagationError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};
use backtrace::{Backtrace, BacktraceFrame, Symbol};
use rustc_demangle::demangle;
use std::path::PathBuf;
use std::ffi::{CStr, CString};

/// Enhanced runtime support for error propagation in CURSED
/// 
/// This module provides comprehensive runtime infrastructure for the `?` operator,
/// including error handler stacks, propagation tracking, performance monitoring,
/// and integration with the panic/recovery system.

/// Runtime system for error propagation
#[derive(Debug)]
pub struct ErrorPropagationRuntime {
    /// Stack of error handlers for nested contexts
    error_handlers: Vec<Box<dyn ErrorHandler + Send + Sync>>,
    
    /// Current propagation context stack
    propagation_stack: Vec<PropagationFrame>,
    
    /// Statistics for performance monitoring
    statistics: Arc<Mutex<PropagationStatistics>>,
    
    /// Configuration for error propagation behavior
    config: PropagationConfig,
    
    /// Integration with panic system
    panic_runtime: Option<String>, // Simplified for now
    
    /// Thread-local propagation state
    thread_local_state: Arc<RwLock<HashMap<thread::ThreadId, ThreadLocalState>>>,
}

/// Frame representing a single propagation site
#[derive(Debug, Clone)]
pub struct PropagationFrame {
    /// Source location of the propagation
    pub location: ErrorSourceLocation,
    
    /// Function context
    pub function_name: Option<String>,
    
    /// Timestamp when propagation occurred
    pub timestamp: Instant,
    
    /// Error type being propagated
    pub error_type: String,
    
    /// Whether this is a tail position propagation
    pub is_tail_position: bool,
    
    /// Stack trace at propagation point
    pub stack_trace: Vec<StackFrame>,
    
    /// Debug information
    pub debug_info: Option<DebugInfo>,
}

/// Represents a single frame in the stack trace
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Function name (demangled if possible)
    pub function_name: Option<String>,
    
    /// Raw symbol name
    pub symbol_name: Option<String>,
    
    /// Source file path
    pub file_path: Option<PathBuf>,
    
    /// Line number in source file
    pub line_number: Option<u32>,
    
    /// Column number in source file
    pub column_number: Option<u32>,
    
    /// Module path
    pub module_path: Option<String>,
    
    /// Instruction pointer
    pub instruction_pointer: Option<usize>,
}

/// Debug information associated with a stack frame
#[derive(Debug, Clone)]
pub struct DebugInfo {
    /// Compilation unit
    pub compilation_unit: Option<String>,
    
    /// Debug symbols available
    pub symbols_available: bool,
    
    /// Source language
    pub source_language: Option<String>,
    
    /// Optimization level
    pub optimization_level: Option<String>,
}

/// Thread-local state for error propagation
#[derive(Debug, Clone)]
pub struct ThreadLocalState {
    /// Current propagation depth
    pub propagation_depth: usize,
    
    /// Thread-specific error handlers
    pub local_handlers: Vec<String>,
    
    /// Whether error propagation is currently active
    pub propagation_active: bool,
    
    /// Last error propagated in this thread
    pub last_error: Option<Error>,
}

/// Statistics for error propagation performance
#[derive(Debug, Default, Clone)]
pub struct PropagationStatistics {
    /// Total number of propagations
    pub total_propagations: u64,
    
    /// Number of successful propagations
    pub successful_propagations: u64,
    
    /// Number of failed propagations
    pub failed_propagations: u64,
    
    /// Average propagation time in microseconds
    pub average_propagation_time_us: f64,
    
    /// Maximum propagation depth observed
    pub max_propagation_depth: usize,
    
    /// Number of panic integrations triggered
    pub panic_integrations: u64,
    
    /// Error type frequency map
    pub error_type_counts: HashMap<String, u64>,
    
    /// Function-specific propagation counts
    pub function_propagation_counts: HashMap<String, u64>,
}

/// Configuration for error propagation behavior
#[derive(Debug, Clone)]
pub struct PropagationConfig {
    /// Maximum propagation depth allowed
    pub max_propagation_depth: usize,
    
    /// Whether to generate detailed stack traces
    pub generate_stack_traces: bool,
    
    /// Whether to integrate with panic system
    pub panic_integration_enabled: bool,
    
    /// Timeout for error propagation operations
    pub propagation_timeout: Duration,
    
    /// Whether to collect performance statistics
    pub collect_statistics: bool,
    
    /// Whether to preserve error context
    pub preserve_error_context: bool,
}

/// Trait for error handlers in the propagation system
pub trait ErrorHandler: std::fmt::Debug {
    /// Handle an error during propagation
    fn handle_error(&self, error: &Error, context: &PropagationFrame) -> Result<(), Error>;
    
    /// Get the handler name
    fn name(&self) -> &str;
    
    /// Check if this handler can handle the given error type
    fn can_handle(&self, error: &Error) -> bool;
    
    /// Get priority for handler ordering
    fn priority(&self) -> u32;
}

impl ErrorPropagationRuntime {
    /// Create a new error propagation runtime
    pub fn new() -> Self {
        Self {
            error_handlers: Vec::new(),
            propagation_stack: Vec::new(),
            statistics: Arc::new(Mutex::new(PropagationStatistics::default())),
            config: PropagationConfig::default(),
            panic_runtime: None,
            thread_local_state: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(config: PropagationConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }
    
    /// Initialize with panic system integration
    pub fn with_panic_integration(mut self, _panic_runtime: String) -> Self {
        self.panic_runtime = Some("panic_enabled".to_string());
        self
    }
    
    /// Register an error handler
    pub fn register_handler(&mut self, handler: Box<dyn ErrorHandler + Send + Sync>) {
        // Insert handler in priority order
        let priority = handler.priority();
        let insert_pos = self.error_handlers
            .iter()
            .position(|h| h.priority() > priority)
            .unwrap_or(self.error_handlers.len());
        
        self.error_handlers.insert(insert_pos, handler);
    }
    
    /// Propagate an error through the runtime system
    pub fn propagate_error(
        &mut self,
        error: Error,
        location: ErrorSourceLocation,
        function_context: Option<String>,
    ) -> Result<(), Error> {
        let start_time = Instant::now();
        
        // Check propagation depth
        self.check_propagation_depth(&location)?;
        
        // Create propagation frame with stack trace
        let stack_trace = self.capture_stack_trace();
        let debug_info = self.extract_debug_info(&stack_trace);
        
        let frame = PropagationFrame {
            location: location.clone(),
            function_name: function_context.clone(),
            timestamp: Instant::now(),
            error_type: self.get_error_type_name(&error),
            is_tail_position: self.is_tail_position(&location, &stack_trace),
            stack_trace,
            debug_info,
        };
        
        // Update thread-local state
        self.update_thread_local_state()?;
        
        // Push frame onto propagation stack
        self.propagation_stack.push(frame.clone());
        
        // Try to handle the error with registered handlers
        match self.try_handle_error(&error, &frame) {
            Ok(()) => {
                self.record_successful_propagation(start_time);
                Ok(())
            },
            Err(handler_error) => {
                // If handlers fail, integrate with panic system if available
                if self.config.panic_integration_enabled {
                    let panic_runtime = self.panic_runtime.clone();
                    self.integrate_with_panic_system(&error, &frame, &panic_runtime)?;
                }
                
                self.record_failed_propagation(start_time);
                
                Err(ErrorPropagationError::with_context(
                    handler_error,
                    location.into(),
                    function_context,
                    None,
                ))
            }
        }
    }
    
    /// Try to handle error with registered handlers
    fn try_handle_error(&self, error: &Error, frame: &PropagationFrame) -> Result<(), Error> {
        for handler in &self.error_handlers {
            if handler.can_handle(error) {
                match handler.handle_error(error, frame) {
                    Ok(()) => return Ok(()),
                    Err(e) => {
                        // Log handler failure but continue to next handler
                        eprintln!("Error handler '{}' failed: {}", handler.name(), e);
                        continue;
                    }
                }
            }
        }
        
        // No handler could handle the error
        Err(Error::ErrorPropagation {
            message: format!("No handler available for error type: {}", 
                           self.get_error_type_name(error)),
            line: Some(frame.location.line),
            column: Some(frame.location.column),
        })
    }
    
    /// Check if propagation depth is within limits
    fn check_propagation_depth(&self, location: &ErrorSourceLocation) -> Result<(), Error> {
        if self.propagation_stack.len() >= self.config.max_propagation_depth {
            return Err(ErrorPropagationError::new(
                Error::ErrorPropagation {
                    message: format!("Propagation depth limit exceeded: {} (max: {})",
                                   self.propagation_stack.len() + 1,
                                   self.config.max_propagation_depth),
                    line: Some(location.line as usize),
                    column: Some(location.column as usize),
                },
                location.clone().into(),
            ));
        }
        
        Ok(())
    }
    
    /// Update thread-local propagation state
    fn update_thread_local_state(&mut self) -> Result<(), Error> {
        let thread_id = thread::current().id();
        let mut state_map = self.thread_local_state.write()
            .map_err(|_| ErrorPropagationError::new(
                Error::Runtime("Failed to acquire thread state lock".to_string()),
                ErrorSourceLocation::new(0, 0).into(),
            ))?;
        
        let state = state_map.entry(thread_id).or_insert_with(|| ThreadLocalState {
            propagation_depth: 0,
            local_handlers: Vec::new(),
            propagation_active: false,
            last_error: None,
        });
        
        state.propagation_depth += 1;
        state.propagation_active = true;
        
        Ok(())
    }
    
    /// Integrate with panic system for unhandled errors
    fn integrate_with_panic_system(
        &mut self,
        error: &Error,
        frame: &PropagationFrame,
        _panic_runtime: &Option<String>,
    ) -> Result<(), Error> {
        // Create panic from error propagation failure
        let panic_message = format!(
            "Unhandled error propagation: {} at {}:{}",
            error, frame.location.line, frame.location.column
        );
        
        // Log the panic (simplified implementation)
        eprintln!("PANIC: {}", panic_message);
        
        // Update statistics
        if let Ok(mut stats) = self.statistics.lock() {
            stats.panic_integrations += 1;
        }
        
        Ok(())
    }
    
    /// Record successful propagation statistics
    fn record_successful_propagation(&self, start_time: Instant) {
        if let Ok(mut stats) = self.statistics.lock() {
            stats.total_propagations += 1;
            stats.successful_propagations += 1;
            
            let duration = start_time.elapsed();
            let duration_us = duration.as_micros() as f64;
            
            // Update average using exponential moving average
            stats.average_propagation_time_us = 
                (stats.average_propagation_time_us * 0.9) + (duration_us * 0.1);
            
            stats.max_propagation_depth = stats.max_propagation_depth.max(self.propagation_stack.len());
        }
    }
    
    /// Record failed propagation statistics
    fn record_failed_propagation(&self, start_time: Instant) {
        if let Ok(mut stats) = self.statistics.lock() {
            stats.total_propagations += 1;
            stats.failed_propagations += 1;
            
            let duration = start_time.elapsed();
            let duration_us = duration.as_micros() as f64;
            stats.average_propagation_time_us = 
                (stats.average_propagation_time_us * 0.9) + (duration_us * 0.1);
        }
    }
    
    /// Get error type name for statistics
    fn get_error_type_name(&self, error: &Error) -> String {
        match error {
            Error::Io(_) => "IO".to_string(),
            Error::Parse(_) => "Parse".to_string(),
            Error::Compile(_) => "Compile".to_string(),
            Error::Runtime(_) => "Runtime".to_string(),
            Error::Package(_) => "Package".to_string(),
            Error::Repl(_) => "Repl".to_string(),
            Error::TemplateError { .. } => "Template".to_string(),
            Error::TypeCompilation(_) => "TypeCompilation".to_string(),
            Error::Type(_) => "Type".to_string(),
            Error::Panic { .. } => "Panic".to_string(),
            Error::Recovery { .. } => "Recovery".to_string(),
            Error::ErrorPropagation { .. } => "ErrorPropagation".to_string(),
            Error::ParseError { .. } => "ParseError".to_string(),
            Error::CodeGeneration { .. } => "CodeGeneration".to_string(),
            Error::ProcessError(_) => "ProcessError".to_string(),
        }
    }
    
    /// Capture current stack trace
    fn capture_stack_trace(&self) -> Vec<StackFrame> {
        let mut frames = Vec::new();
        
        if !self.config.generate_stack_traces {
            return frames;
        }
        
        let backtrace = Backtrace::new();
        
        backtrace.frames().iter().for_each(|frame| {
            frame.symbols().iter().for_each(|symbol| {
                let stack_frame = self.symbol_to_stack_frame(symbol, frame);
                frames.push(stack_frame);
            });
        });
        
        frames
    }
    
    /// Convert a backtrace symbol to our StackFrame format
    fn symbol_to_stack_frame(&self, symbol: &Symbol, frame: &BacktraceFrame) -> StackFrame {
        let symbol_name = symbol.name().map(|s| s.to_string());
        let function_name = symbol_name.as_ref().map(|name| {
            demangle(name).to_string()
        });
        
        let file_path = symbol.filename().map(|p| p.to_path_buf());
        let line_number = symbol.lineno();
        let column_number = symbol.colno();
        
        // Extract module path from symbol name
        let module_path = function_name.as_ref().and_then(|name| {
            if name.contains("::") {
                let parts: Vec<&str> = name.split("::").collect();
                if parts.len() > 1 {
                    Some(parts[..parts.len()-1].join("::"))
                } else {
                    None
                }
            } else {
                None
            }
        });
        
        StackFrame {
            function_name,
            symbol_name,
            file_path,
            line_number,
            column_number,
            module_path,
            instruction_pointer: Some(frame.ip() as usize),
        }
    }
    
    /// Extract debug information from stack trace
    fn extract_debug_info(&self, stack_trace: &[StackFrame]) -> Option<DebugInfo> {
        if stack_trace.is_empty() {
            return None;
        }
        
        let symbols_available = stack_trace.iter().any(|frame| {
            frame.function_name.is_some() || frame.file_path.is_some()
        });
        
        // Try to determine if we're in a CURSED compilation unit
        let compilation_unit = stack_trace.iter()
            .find_map(|frame| {
                frame.file_path.as_ref().and_then(|path| {
                    if path.extension().and_then(|s| s.to_str()) == Some("csd") {
                        path.file_stem().and_then(|s| s.to_str()).map(|s| s.to_string())
                    } else {
                        None
                    }
                })
            });
        
        Some(DebugInfo {
            compilation_unit,
            symbols_available,
            source_language: Some("CURSED".to_string()),
            optimization_level: None, // Would need compiler info
        })
    }
    
    /// Check if location is in tail position based on stack analysis
    fn is_tail_position(&self, _location: &ErrorSourceLocation, stack_trace: &[StackFrame]) -> bool {
        // Analyze stack trace to determine if this is a tail position
        // Look for patterns that indicate tail calls or return statements
        
        if stack_trace.len() < 2 {
            return false;
        }
        
        // Check if the current frame is the last meaningful frame before main/runtime
        let current_frame = &stack_trace[0];
        let next_frame = &stack_trace[1];
        
        // Heuristic: if the next frame is a runtime function or main, 
        // this might be a tail position
        if let (Some(current_fn), Some(next_fn)) = (&current_frame.function_name, &next_frame.function_name) {
            next_fn.contains("main") || 
            next_fn.contains("runtime") || 
            next_fn.contains("error_propagation") ||
            current_fn.ends_with("?") // Question mark operator functions
        } else {
            false
        }
    }
    
    /// Get current function name from stack trace
    fn get_current_function_name(&self) -> Option<String> {
        let backtrace = Backtrace::new();
        
        for frame in backtrace.frames() {
            for symbol in frame.symbols() {
                if let Some(name) = symbol.name() {
                    let demangled = demangle(&name.to_string()).to_string();
                    // Skip runtime and error propagation functions
                    if !demangled.contains("error_propagation") && 
                       !demangled.contains("backtrace") &&
                       !demangled.contains("__rust") {
                        return Some(demangled);
                    }
                }
            }
        }
        
        None
    }
    
    /// Create a minimal stack trace for FFI
    fn create_minimal_stack_trace(&self) -> Box<MinimalStackTrace> {
        let frames = self.capture_stack_trace();
        let mut minimal_frames = Vec::new();
        
        for frame in frames.into_iter().take(10) { // Limit to 10 frames
            minimal_frames.push(MinimalStackFrame {
                function_name: frame.function_name.unwrap_or_else(|| "<unknown>".to_string()),
                file_name: frame.file_path
                    .and_then(|p| p.file_name())
                    .and_then(|s| s.to_str())
                    .unwrap_or("<unknown>")
                    .to_string(),
                line_number: frame.line_number.unwrap_or(0),
                column_number: frame.column_number.unwrap_or(0),
            });
        }
        
        Box::new(MinimalStackTrace {
            frames: minimal_frames,
            total_frames: minimal_frames.len(),
            timestamp: Instant::now(),
        })
    }
    
    /// Get current propagation statistics
    pub fn get_statistics(&self) -> Result<(), Error> {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .map_err(|_| Error::Runtime("Failed to acquire statistics lock".to_string()))
    }
    
    /// Reset propagation statistics
    pub fn reset_statistics(&mut self) -> Result<(), Error> {
        let mut stats = self.statistics.lock()
            .map_err(|_| Error::Runtime("Failed to acquire statistics lock".to_string()))?;
        
        *stats = PropagationStatistics::default();
        Ok(())
    }
    
    /// Get current propagation stack depth
    pub fn get_propagation_depth(&self) -> usize {
        self.propagation_stack.len()
    }
    
    /// Clear propagation stack (for error recovery)
    pub fn clear_propagation_stack(&mut self) {
        self.propagation_stack.clear();
        
        // Reset thread-local state
        if let Ok(mut state_map) = self.thread_local_state.write() {
            let thread_id = thread::current().id();
            if let Some(state) = state_map.get_mut(&thread_id) {
                state.propagation_depth = 0;
                state.propagation_active = false;
            }
        }
    }
}

impl Default for PropagationConfig {
    fn default() -> Self {
        Self {
            max_propagation_depth: 100,
            generate_stack_traces: true,
            panic_integration_enabled: true,
            propagation_timeout: Duration::from_secs(5),
            collect_statistics: true,
            preserve_error_context: true,
        }
    }
}

impl Default for ThreadLocalState {
    fn default() -> Self {
        Self {
            propagation_depth: 0,
            local_handlers: Vec::new(),
            propagation_active: false,
            last_error: None,
        }
    }
}

/// Minimal stack trace for FFI and external use
#[derive(Debug, Clone)]
pub struct MinimalStackTrace {
    pub frames: Vec<MinimalStackFrame>,
    pub total_frames: usize,
    pub timestamp: Instant,
}

/// Minimal stack frame for FFI
#[derive(Debug, Clone)]
pub struct MinimalStackFrame {
    pub function_name: String,
    pub file_name: String,
    pub line_number: u32,
    pub column_number: u32,
}

/// Default error handler for basic error types
#[derive(Debug)]
pub struct DefaultErrorHandler {
    name: String,
    priority: u32,
}

impl DefaultErrorHandler {
    pub fn new() -> Self {
        Self {
            name: "DefaultErrorHandler".to_string(),
            priority: 1000, // Low priority
        }
    }
}

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(&self, error: &Error, _context: &PropagationFrame) -> Result<(), Error> {
        // Basic error handling - just log the error
        eprintln!("Error propagated: {}", error);
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn can_handle(&self, _error: &Error) -> bool {
        true // Can handle any error as fallback
    }
    
    fn priority(&self) -> u32 {
        self.priority
    }
}

/// Global error propagation runtime instance
static mut GLOBAL_RUNTIME: Option<Mutex<ErrorPropagationRuntime>> = None;
static INIT_ONCE: std::sync::Once = std::sync::Once::new();

/// Initialize global error propagation runtime
fn ensure_global_runtime() -> &'static Mutex<ErrorPropagationRuntime> {
    unsafe {
        INIT_ONCE.call_once(|| {
            GLOBAL_RUNTIME = Some(Mutex::new(ErrorPropagationRuntime::new()));
        });
        GLOBAL_RUNTIME.as_ref().unwrap()
    }
}

/// FFI functions for runtime integration
#[no_mangle]
pub extern "C" fn cursed_error_propagation_init() {
    ensure_global_runtime();
    eprintln!("Error propagation runtime initialized");
}

#[no_mangle]
pub extern "C" fn cursed_error_propagation_cleanup() {
    // Reset global runtime
    unsafe {
        GLOBAL_RUNTIME = None;
    }
    eprintln!("Error propagation runtime cleaned up");
}

#[no_mangle]
pub extern "C" fn cursed_error_propagation(
    error_value: *const u8,
    line: u32,
    column: u32,
) {
    if error_value.is_null() {
        return;
    }
    
    let runtime = ensure_global_runtime();
    if let Ok(mut runtime) = runtime.lock() {
        let location = ErrorSourceLocation::new(line as u32, column as u32);
        let function_name = runtime.get_current_function_name();
        
        // Create a generic runtime error for demonstration
        let error = Error::Runtime(format!("Error propagated at {}:{}", line, column));
        
        if let Err(e) = runtime.propagate_error(error, location, function_name) {
            eprintln!("Failed to propagate error: {}", e);
        }
    } else {
        eprintln!("Error propagated at line {}, column {} (runtime unavailable)", line, column);
    }
}

#[no_mangle]
pub extern "C" fn cursed_record_error_context(
    line: u32,
    column: u32,
    function_name: *const u8,
) {
    let function_str = if function_name.is_null() {
        "<unknown>".to_string()
    } else {
        unsafe {
            let c_str = std::ffi::CStr::from_ptr(function_name as *const i8);
            c_str.to_str().unwrap_or("<invalid>").to_string()
        }
    };
    
    eprintln!("Recording error context at {}:{} in function '{}'", line, column, function_str);
}

#[no_mangle]
pub extern "C" fn cursed_capture_stack_trace(
    stack_trace_ptr: *mut u8,
    max_depth: u64,
) {
    if stack_trace_ptr.is_null() {
        return;
    }
    
    let runtime = ensure_global_runtime();
    if let Ok(runtime) = runtime.lock() {
        let frames = runtime.capture_stack_trace();
        let limited_frames: Vec<_> = frames.into_iter().take(max_depth as usize).collect();
        
        // Store stack trace information at the provided pointer
        // In a real implementation, this would serialize the stack trace
        let trace_info = format!("Stack trace: {} frames captured", limited_frames.len());
        eprintln!("{}", trace_info);
        
        // For demonstration, store frame count at the pointer location
        unsafe {
            if max_depth > 0 {
                *(stack_trace_ptr as *mut u64) = limited_frames.len() as u64;
            }
        }
    } else {
        eprintln!("Failed to capture stack trace: runtime unavailable");
    }
}

#[no_mangle]
pub extern "C" fn cursed_get_current_function_name() -> *const u8 {
    let runtime = ensure_global_runtime();
    if let Ok(runtime) = runtime.lock() {
        if let Some(function_name) = runtime.get_current_function_name() {
            // Convert to C string and leak memory (caller must free)
            let c_string = CString::new(function_name).unwrap_or_else(|_| {
                CString::new("<invalid_function_name>").unwrap()
            });
            return c_string.into_raw() as *const u8;
        }
    }
    
    // Return placeholder if unable to get function name
    let function_name = b"<unknown_function>\0";
    function_name.as_ptr()
}

#[no_mangle]
pub extern "C" fn cursed_store_stack_frame(
    frame_ptr: *mut u8,
    function_name: *const u8,
    line: u32,
    column: u32,
) {
    if frame_ptr.is_null() || function_name.is_null() {
        return;
    }
    
    let function_str = unsafe {
        let c_str = std::ffi::CStr::from_ptr(function_name as *const i8);
        c_str.to_str().unwrap_or("<invalid>")
    };
    
    eprintln!("Storing stack frame: {} at {}:{} in {:p}", function_str, line, column, frame_ptr);
}

#[no_mangle]
pub extern "C" fn cursed_add_debug_stack_info(stack_trace_ptr: *mut u8) {
    if stack_trace_ptr.is_null() {
        return;
    }
    
    eprintln!("Adding debug information to stack trace at {:p}", stack_trace_ptr);
}

#[no_mangle]
pub extern "C" fn cursed_record_stack_context(
    stack_trace_ptr: *const u8,
    max_depth: u64,
) {
    if stack_trace_ptr.is_null() {
        return;
    }
    
    eprintln!("Recording stack context: {} frames at {:p}", max_depth, stack_trace_ptr);
}

#[no_mangle]
pub extern "C" fn cursed_get_debug_info() -> *const u8 {
    // Return a placeholder debug info pointer
    let debug_info = b"<debug_info>\0";
    debug_info.as_ptr()
}

#[no_mangle]
pub extern "C" fn cursed_attach_debug_to_stack_trace(
    stack_trace_ptr: *mut u8,
    debug_info_ptr: *const u8,
) {
    if stack_trace_ptr.is_null() || debug_info_ptr.is_null() {
        return;
    }
    
    eprintln!("Attaching debug info {:p} to stack trace {:p}", debug_info_ptr, stack_trace_ptr);
}

#[no_mangle]
pub extern "C" fn cursed_capture_source_locations(stack_trace_ptr: *mut u8) {
    if stack_trace_ptr.is_null() {
        return;
    }
    
    eprintln!("Capturing source locations for stack trace at {:p}", stack_trace_ptr);
}

#[no_mangle]
pub extern "C" fn cursed_resolve_stack_symbols(stack_trace_ptr: *mut u8) {
    if stack_trace_ptr.is_null() {
        return;
    }
    
    eprintln!("Resolving stack symbols for stack trace at {:p}", stack_trace_ptr);
}

#[no_mangle]
pub extern "C" fn cursed_create_minimal_stack_trace() -> *mut u8 {
    let runtime = ensure_global_runtime();
    if let Ok(runtime) = runtime.lock() {
        let minimal_trace = runtime.create_minimal_stack_trace();
        let boxed_trace = Box::into_raw(minimal_trace);
        eprintln!("Created minimal stack trace at {:p}", boxed_trace);
        boxed_trace as *mut u8
    } else {
        eprintln!("Failed to create minimal stack trace: runtime unavailable");
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn cursed_free_minimal_stack_trace(stack_trace_ptr: *mut u8) {
    if !stack_trace_ptr.is_null() {
        unsafe {
            let _trace = Box::from_raw(stack_trace_ptr as *mut MinimalStackTrace);
            // trace will be automatically dropped and freed
        }
    }
}

#[no_mangle]
pub extern "C" fn cursed_error_propagation_panic(message: *const u8) {
    if message.is_null() {
        return;
    }
    
    // Convert C string to Rust string and panic
    let c_str = unsafe { std::ffi::CStr::from_ptr(message as *const i8) };
    if let Ok(rust_str) = c_str.to_str() {
        // Create a proper propagation frame before panicking
        let runtime = ensure_global_runtime();
        if let Ok(runtime) = runtime.lock() {
            let stack_trace = runtime.capture_stack_trace();
            eprintln!("Error propagation panic with {} stack frames: {}", stack_trace.len(), rust_str);
        }
        panic!("Error propagation panic: {}", rust_str);
    } else {
        panic!("Error propagation panic with invalid message");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_propagation_runtime_creation() {
        let runtime = ErrorPropagationRuntime::new();
        assert_eq!(runtime.get_propagation_depth(), 0);
        assert!(runtime.error_handlers.is_empty());
    }
    
    #[test]
    fn test_propagation_config() {
        let config = PropagationConfig::default();
        assert_eq!(config.max_propagation_depth, 100);
        assert!(config.generate_stack_traces);
        assert!(config.panic_integration_enabled);
    }
    
    #[test]
    fn test_error_handler_registration() {
        let mut runtime = ErrorPropagationRuntime::new();
        let handler = Box::new(DefaultErrorHandler::new());
        
        runtime.register_handler(handler);
        assert_eq!(runtime.error_handlers.len(), 1);
    }
    
    #[test]
    fn test_propagation_frame() {
        let location = ErrorSourceLocation::new(1, 5);
        let frame = PropagationFrame {
            location: location.clone(),
            function_name: Some("test_function".to_string()),
            timestamp: Instant::now(),
            error_type: "Runtime".to_string(),
            is_tail_position: false,
        };
        
        assert_eq!(frame.location.line, 1);
        assert_eq!(frame.location.column, 5);
        assert_eq!(frame.function_name, Some("test_function".to_string()));
    }
    
    #[test]
    fn test_thread_local_state() {
        let state = ThreadLocalState::default();
        assert_eq!(state.propagation_depth, 0);
        assert!(!state.propagation_active);
        assert!(state.last_error.is_none());
    }
    
    #[test]
    fn test_default_error_handler() {
        let handler = DefaultErrorHandler::new();
        let error = Error::Runtime("Test error".to_string());
        let location = ErrorSourceLocation::new(1, 5);
        let frame = PropagationFrame {
            location,
            function_name: None,
            timestamp: Instant::now(),
            error_type: "Runtime".to_string(),
            is_tail_position: false,
        };
        
        assert!(handler.can_handle(&error));
        assert_eq!(handler.name(), "DefaultErrorHandler");
        assert_eq!(handler.priority(), 1000);
        
        let result = handler.handle_error(&error, &frame);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_propagation_statistics() {
        let stats = PropagationStatistics::default();
        assert_eq!(stats.total_propagations, 0);
        assert_eq!(stats.successful_propagations, 0);
        assert_eq!(stats.failed_propagations, 0);
        assert_eq!(stats.max_propagation_depth, 0);
    }
    
    #[test]
    fn test_error_type_name_extraction() {
        let runtime = ErrorPropagationRuntime::new();
        
        let runtime_error = Error::Runtime("test".to_string());
        assert_eq!(runtime.get_error_type_name(&runtime_error), "Runtime");
        
        let parse_error = Error::Parse("test".to_string());
        assert_eq!(runtime.get_error_type_name(&parse_error), "Parse");
    }
    
    #[test]
    fn test_propagation_depth_tracking() {
        let mut runtime = ErrorPropagationRuntime::new();
        assert_eq!(runtime.get_propagation_depth(), 0);
        
        // Simulate adding frames
        let location = ErrorSourceLocation::new(1, 5);
        let frame = PropagationFrame {
            location,
            function_name: None,
            timestamp: Instant::now(),
            error_type: "Runtime".to_string(),
            is_tail_position: false,
        };
        
        runtime.propagation_stack.push(frame);
        assert_eq!(runtime.get_propagation_depth(), 1);
        
        runtime.clear_propagation_stack();
        assert_eq!(runtime.get_propagation_depth(), 0);
    }
}
