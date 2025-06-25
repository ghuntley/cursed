/// Stack Trace System for CURSED CursedError Handling
///
/// Provides comprehensive stack trace generation and management including:
/// - CURSED function call tracking
/// - Source location resolution
/// - Symbol resolution and debugging information
/// - Integration with Rust backtraces
/// - Thread-safe stack frame management

use crate::error::CursedError;

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, ThreadId};
use std::time::SystemTime;
use std::fmt;
use std::backtrace::{Backtrace, BacktraceStatus};
use tracing::{debug, error, info, instrument, warn};

/// Represents a single frame in a CURSED call stack
#[derive(Debug, Clone)]
pub struct CallFrame {
    /// Function name being called
    /// Source location of the call
    /// Module or package name
    /// Local variables visible in this frame
    /// Function parameters
    /// Frame depth in the call stack
    /// Timestamp when frame was created
    /// Whether this frame represents a CURSED function or native code
    /// Instruction pointer (if available)
impl CallFrame {
    pub fn new(function_name: String, depth: usize) -> Self {
        CallFrame {
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    pub fn with_module(mut self, module: String) -> Self {
        self.module_name = Some(module);
        self
    pub fn with_variable(mut self, name: String, value: String) -> Self {
        self.local_variables.insert(name, value);
        self
    pub fn with_parameter(mut self, name: String, value: String) -> Self {
        self.parameters.insert(name, value);
        self
    pub fn native_frame(function_name: String, depth: usize) -> Self {
        let mut frame = Self::new(function_name, depth);
        frame.is_cursed_function = false;
        frame
    }
}

impl fmt::Display for CallFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.depth, self.function_name)?;
        
        if let Some(module) = &self.module_name {
            write!(f, " ({})", module)?;
        if let Some(location) = &self.source_location {
            write!(f, " at {}", location)?;
        if !self.parameters.is_empty() {
            write!(f, " with parameters: ")?;
            for (i, (name, value)) in self.parameters.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", name, value)?;
            }
        }
        
        Ok(())
    }
}

/// Complete stack trace information
#[derive(Debug)]
pub struct StackTrace {
    /// Unique identifier for this stack trace
    /// Ordered list of call frames (deepest first)
    /// Thread ID where trace was captured
    /// Goroutine ID if captured in goroutine context
    /// Timestamp when trace was captured
    /// Associated Rust backtrace
    /// Whether trace capture was truncated
    /// Maximum depth that was requested
impl StackTrace {
    pub fn new(max_depth: usize) -> Self {
        StackTrace {
        }
    }

    pub fn with_goroutine(mut self, goroutine_id: u64) -> Self {
        self.goroutine_id = Some(goroutine_id);
        self
    pub fn with_rust_backtrace(mut self, backtrace: Backtrace) -> Self {
        self.rust_backtrace = Some(backtrace);
        self
    pub fn add_frame(&mut self, frame: CallFrame) {
        if self.frames.len() < self.max_depth {
            self.frames.push(frame);
        } else {
            self.truncated = true;
        }
    }

    /// Get the top frame (most recent call)
    pub fn top_frame(&self) -> Option<&CallFrame> {
        self.frames.first()
    /// Get the bottom frame (oldest call)
    pub fn bottom_frame(&self) -> Option<&CallFrame> {
        self.frames.last()
    /// Find frame by function name
    pub fn find_frame(&self, function_name: &str) -> Option<&CallFrame> {
        self.frames.iter().find(|frame| frame.function_name == function_name)
    /// Get frames for a specific module
    pub fn frames_for_module(&self, module_name: &str) -> Vec<&CallFrame> {
        self.frames.iter()
            .filter(|frame| frame.module_name.as_ref().map(|m| m == module_name).unwrap_or(false))
            .collect()
    }
}

impl fmt::Display for StackTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Stack trace #{} ({} frames):", self.trace_id, self.frames.len())?;
        
        if let Some(goroutine_id) = self.goroutine_id {
            writeln!(f, "  in goroutine #{}", goroutine_id)?;
        for frame in &self.frames {
            writeln!(f, "  {}", frame)?;
        if self.truncated {
            writeln!(f, "  ... (truncated at {} frames)", self.max_depth)?;
        if let Some(rust_backtrace) = &self.rust_backtrace {
            if rust_backtrace.status() == BacktraceStatus::Captured {
                writeln!(f, "\nRust backtrace:")?;
                writeln!(f, "{}", rust_backtrace)?;
            }
        }
        
        Ok(())
    }
}

/// Debug information for symbol resolution
#[derive(Debug, Clone)]
pub struct DebugInfo {
    /// Function name to source location mapping
    /// Module name to file path mapping
    /// Symbol to function name mapping
    /// Line number to instruction mapping
impl DebugInfo {
    pub fn new() -> Self {
        DebugInfo {
        }
    }

    pub fn add_function(&mut self, name: String, location: SourceLocation) {
        self.function_locations.insert(name, location);
    pub fn add_module(&mut self, name: String, file_path: String) {
        self.module_files.insert(name, file_path);
    pub fn add_symbol(&mut self, address: usize, name: String) {
        self.symbol_table.insert(address, name);
    pub fn resolve_function_location(&self, function_name: &str) -> Option<&SourceLocation> {
        self.function_locations.get(function_name)
    pub fn resolve_symbol(&self, address: usize) -> Option<&String> {
        self.symbol_table.get(&address)
    }
}

impl Default for DebugInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for stack trace capture
#[derive(Debug, Clone)]
pub struct StackTraceConfig {
    /// Maximum number of frames to capture
    /// Whether to capture Rust backtraces
    /// Whether to capture local variables
    /// Whether to capture function parameters
    /// Whether to resolve symbols
    /// Maximum variable value length to capture
impl Default for StackTraceConfig {
    fn default() -> Self {
        StackTraceConfig {
            capture_variables: false, // Can be expensive
        }
    }
/// Per-thread call stack state
struct ThreadCallStack {
    /// Current call frames
    /// Function entry timestamps
    /// Whether currently capturing trace
impl ThreadCallStack {
    fn new() -> Self {
        ThreadCallStack {
        }
    }
/// Stack trace management system
pub struct StackTraceManager {
    /// Configuration for trace capture
    /// Per-thread call stacks
    /// Debug information for symbol resolution
    /// Statistics
/// Stack trace statistics
#[derive(Debug, Default, Clone)]
pub struct StackTraceStatistics {
    /// Total traces captured
    /// Average trace depth
    /// Maximum trace depth seen
    /// Number of truncated traces
    /// Total time spent capturing traces
impl StackTraceManager {
    /// Create a new stack trace manager
    pub fn new() -> Self {
        StackTraceManager {
        }
    }

    /// Create manager with custom configuration
    pub fn with_config(config: StackTraceConfig) -> Self {
        StackTraceManager {
        }
    }

    /// Enter a function (push frame onto stack)
    #[instrument(skip(self))]
    pub fn enter_function(
    ) -> crate::error::Result<()> {
        let thread_id = thread::current().id();
        
        if let Ok(mut stacks) = self.thread_stacks.lock() {
            let stack = stacks.entry(thread_id).or_insert_with(ThreadCallStack::new);
            
            let depth = stack.frames.len();
            let mut frame = CallFrame::new(function_name.clone(), depth);
            
            if let Some(location) = source_location {
                frame = frame.with_location(location);
            if let Some(module) = module_name {
                frame = frame.with_module(module);
            for (name, value) in parameters {
                frame = frame.with_parameter(name, value);
            stack.frames.push(frame);
            stack.entry_times.push(SystemTime::now());
            
            debug!("Entered function: {} (depth: {})", function_name, depth);
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to access thread stack".to_string()))
        }
    }

    /// Exit a function (pop frame from stack)
    #[instrument(skip(self))]
    pub fn exit_function(&self, function_name: Option<String>) -> crate::error::Result<()> {
        let thread_id = thread::current().id();
        
        if let Ok(mut stacks) = self.thread_stacks.lock() {
            if let Some(stack) = stacks.get_mut(&thread_id) {
                if let Some(frame) = stack.frames.pop() {
                    stack.entry_times.pop();
                    
                    if let Some(expected_name) = function_name {
                        if frame.function_name != expected_name {
                            warn!(
                                expected_name, frame.function_name
                            );
                        }
                    }
                    
                    debug!("Exited function: {} (depth: {})", frame.function_name, frame.depth);
                    Ok(())
                } else {
                    warn!("Attempted to exit function but stack is empty");
                    Ok(())
                }
            } else {
                Ok(()) // No stack for thread
            }
        } else {
            Err(CursedError::Runtime("Failed to access thread stack".to_string()))
        }
    }

    /// Capture current stack trace
    #[instrument(skip(self))]
    pub fn capture_stack_trace(&self, goroutine_id: Option<u64>) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        let thread_id = thread::current().id();
        
        let config = self.config.read().unwrap();
        let mut trace = StackTrace::new(config.max_frames);
        
        if let Some(gid) = goroutine_id {
            trace = trace.with_goroutine(gid);
        // Capture Rust backtrace if configured
        if config.capture_rust_backtrace {
            let backtrace = Backtrace::capture();
            trace = trace.with_rust_backtrace(backtrace);
        // Copy current stack frames
        if let Ok(stacks) = self.thread_stacks.lock() {
            if let Some(stack) = stacks.get(&thread_id) {
                // Copy frames in reverse order (most recent first)
                for frame in stack.frames.iter().rev() {
                    let mut trace_frame = frame.clone();
                    
                    // Add local variables if configured
                    if config.capture_variables {
                        trace_frame = self.capture_local_variables(trace_frame)?;
                    // Resolve symbols if configured
                    if config.resolve_symbols {
                        trace_frame = self.resolve_frame_symbols(trace_frame)?;
                    trace.add_frame(trace_frame);
                }
            }
        // Update statistics
        let capture_time = start_time.elapsed();
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_traces += 1;
            stats.total_capture_time += capture_time;
            
            let frame_count = trace.frames.len();
            if frame_count > stats.max_depth {
                stats.max_depth = frame_count;
            if trace.truncated {
                stats.truncated_traces += 1;
            // Update average depth
            let total_depth = stats.average_depth * (stats.total_traces - 1) as f64 + frame_count as f64;
            stats.average_depth = total_depth / stats.total_traces as f64;
        debug!("Captured stack trace with {} frames", trace.frames.len());
        Ok(trace)
    /// Add debug information for symbol resolution
    pub fn add_debug_info(&self, function_name: String, location: SourceLocation, module: Option<String>) -> crate::error::Result<()> {
        if let Ok(mut debug_info) = self.debug_info.write() {
            let file_path = location.file.clone(); // Clone before moving location
            debug_info.add_function(function_name, location);
            
            if let Some(module_name) = module {
                if let Some(file_path) = file_path.as_ref() {
                    debug_info.add_module(module_name, file_path.clone());
                }
            }
            
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to add debug information".to_string()))
        }
    }

    /// Get current call depth for thread
    pub fn get_call_depth(&self) -> usize {
        let thread_id = thread::current().id();
        
        if let Ok(stacks) = self.thread_stacks.lock() {
            stacks.get(&thread_id).map(|s| s.frames.len()).unwrap_or(0)
        } else {
            0
        }
    }

    /// Get current function name
    pub fn get_current_function(&self) -> Option<String> {
        let thread_id = thread::current().id();
        
        if let Ok(stacks) = self.thread_stacks.lock() {
            stacks.get(&thread_id)
                .and_then(|s| s.frames.last())
                .map(|f| f.function_name.clone())
        } else {
            None
        }
    }

    /// Clear stack for current thread
    pub fn clear_stack(&self) {
        let thread_id = thread::current().id();
        
        if let Ok(mut stacks) = self.thread_stacks.lock() {
            if let Some(stack) = stacks.get_mut(&thread_id) {
                stack.frames.clear();
                stack.entry_times.clear();
                stack.capturing = false;
            }
        }
    /// Get stack trace statistics
    pub fn get_statistics(&self) -> crate::error::Result<()> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::Runtime("Failed to access stack trace statistics".to_string()))
    /// Update configuration
    pub fn update_config<F>(&self, updater: F) -> crate::error::Result<()>
    where
    {
        if let Ok(mut config) = self.config.write() {
            updater(&mut *config);
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to update stack trace configuration".to_string()))
        }
    }

    // Helper methods

    fn capture_local_variables(&self, mut frame: CallFrame) -> crate::error::Result<()> {
        // Placeholder implementation - in a real system this would inspect
        // the current execution context for local variables
        
        // For now, just add some mock variables as an example
        frame.local_variables.insert("example_var".to_string(), "example_value".to_string());
        
        Ok(frame)
    fn resolve_frame_symbols(&self, mut frame: CallFrame) -> crate::error::Result<()> {
        if let Ok(debug_info) = self.debug_info.read() {
            // Resolve function location if not already set
            if frame.source_location.is_none() {
                if let Some(location) = debug_info.resolve_function_location(&frame.function_name) {
                    frame.source_location = Some(location.clone());
                }
            }
            
            // Resolve symbols for instruction pointer if available
            if let Some(ip) = frame.instruction_pointer {
                if let Some(symbol) = debug_info.resolve_symbol(ip) {
                    // Update function name if symbol resolution gives a better name
                    if frame.function_name == "unknown" {
                        frame.function_name = symbol.clone();
                    }
                }
            }
        }
        
        Ok(frame)
    }
}

impl Default for StackTraceManager {
    fn default() -> Self {
        Self::new()
    }
}

// FFI functions for LLVM integration

/// Enter a function from compiled code
#[no_mangle]
pub extern "C" fn cursed_stack_enter_function(
) {
    let function_name = if function_ptr.is_null() || function_len == 0 {
        "unknown".to_string()
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(function_ptr, function_len);
            String::from_utf8_lossy(slice).to_string()
        }

    let module_name = if module_ptr.is_null() || module_len == 0 {
        None
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(module_ptr, module_len);
            Some(String::from_utf8_lossy(slice).to_string())
        }

    let source_location = if file_ptr.is_null() || file_len == 0 {
        Some(SourceLocation::new(line as usize, column as usize))
    } else {
        unsafe {
            let file_slice = std::slice::from_raw_parts(file_ptr, file_len);
            let file_name = String::from_utf8_lossy(file_slice).to_string();
            Some(SourceLocation::new(line as usize, column as usize).with_file(&file_name))
        }

    // Create a default stack trace manager for this call
    // In practice, this would use a global instance
    let manager = StackTraceManager::new();
    let _ = manager.enter_function(function_name, module_name, source_location, HashMap::new());
/// Exit a function from compiled code
#[no_mangle]
pub extern "C" fn cursed_stack_exit_function(
) {
    let function_name = if function_ptr.is_null() || function_len == 0 {
        None
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(function_ptr, function_len);
            Some(String::from_utf8_lossy(slice).to_string())
        }

    let manager = StackTraceManager::new();
    let _ = manager.exit_function(function_name);
/// Get current call depth
#[no_mangle]
pub extern "C" fn cursed_get_call_depth() -> u32 {
    let manager = StackTraceManager::new();
    manager.get_call_depth() as u32
