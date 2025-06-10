/// Stack Trace System for CURSED Error Handling
///
/// Provides comprehensive stack trace generation and management including:
/// - CURSED function call tracking
/// - Source location resolution
/// - Symbol resolution and debugging information
/// - Integration with Rust backtraces
/// - Thread-safe stack frame management

use crate::error::{Error as CursedError, SourceLocation};
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
    pub function_name: String,
    /// Source location of the call
    pub source_location: Option<SourceLocation>,
    /// Module or package name
    pub module_name: Option<String>,
    /// Local variables visible in this frame
    pub local_variables: HashMap<String, String>,
    /// Function parameters
    pub parameters: HashMap<String, String>,
    /// Frame depth in the call stack
    pub depth: usize,
    /// Timestamp when frame was created
    pub timestamp: SystemTime,
    /// Whether this frame represents a CURSED function or native code
    pub is_cursed_function: bool,
    /// Instruction pointer (if available)
    pub instruction_pointer: Option<usize>,
}

impl CallFrame {
    pub fn new(function_name: String, depth: usize) -> Self {
        CallFrame {
            function_name,
            source_location: None,
            module_name: None,
            local_variables: HashMap::new(),
            parameters: HashMap::new(),
            depth,
            timestamp: SystemTime::now(),
            is_cursed_function: true,
            instruction_pointer: None,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    }

    pub fn with_module(mut self, module: String) -> Self {
        self.module_name = Some(module);
        self
    }

    pub fn with_variable(mut self, name: String, value: String) -> Self {
        self.local_variables.insert(name, value);
        self
    }

    pub fn with_parameter(mut self, name: String, value: String) -> Self {
        self.parameters.insert(name, value);
        self
    }

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
        }
        
        if let Some(location) = &self.source_location {
            write!(f, " at {}", location)?;
        }
        
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
    pub trace_id: u64,
    /// Ordered list of call frames (deepest first)
    pub frames: Vec<CallFrame>,
    /// Thread ID where trace was captured
    pub thread_id: ThreadId,
    /// Goroutine ID if captured in goroutine context
    pub goroutine_id: Option<u64>,
    /// Timestamp when trace was captured
    pub timestamp: SystemTime,
    /// Associated Rust backtrace
    pub rust_backtrace: Option<Backtrace>,
    /// Whether trace capture was truncated
    pub truncated: bool,
    /// Maximum depth that was requested
    pub max_depth: usize,
}

impl StackTrace {
    pub fn new(max_depth: usize) -> Self {
        StackTrace {
            trace_id: crate::runtime::error_handling::next_error_id(),
            frames: Vec::new(),
            thread_id: thread::current().id(),
            goroutine_id: None,
            timestamp: SystemTime::now(),
            rust_backtrace: None,
            truncated: false,
            max_depth,
        }
    }

    pub fn with_goroutine(mut self, goroutine_id: u64) -> Self {
        self.goroutine_id = Some(goroutine_id);
        self
    }

    pub fn with_rust_backtrace(mut self, backtrace: Backtrace) -> Self {
        self.rust_backtrace = Some(backtrace);
        self
    }

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
    }

    /// Get the bottom frame (oldest call)
    pub fn bottom_frame(&self) -> Option<&CallFrame> {
        self.frames.last()
    }

    /// Find frame by function name
    pub fn find_frame(&self, function_name: &str) -> Option<&CallFrame> {
        self.frames.iter().find(|frame| frame.function_name == function_name)
    }

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
        }
        
        for frame in &self.frames {
            writeln!(f, "  {}", frame)?;
        }
        
        if self.truncated {
            writeln!(f, "  ... (truncated at {} frames)", self.max_depth)?;
        }
        
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
    pub function_locations: HashMap<String, SourceLocation>,
    /// Module name to file path mapping
    pub module_files: HashMap<String, String>,
    /// Symbol to function name mapping
    pub symbol_table: HashMap<usize, String>,
    /// Line number to instruction mapping
    pub line_table: HashMap<SourceLocation, usize>,
}

impl DebugInfo {
    pub fn new() -> Self {
        DebugInfo {
            function_locations: HashMap::new(),
            module_files: HashMap::new(),
            symbol_table: HashMap::new(),
            line_table: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, name: String, location: SourceLocation) {
        self.function_locations.insert(name, location);
    }

    pub fn add_module(&mut self, name: String, file_path: String) {
        self.module_files.insert(name, file_path);
    }

    pub fn add_symbol(&mut self, address: usize, name: String) {
        self.symbol_table.insert(address, name);
    }

    pub fn resolve_function_location(&self, function_name: &str) -> Option<&SourceLocation> {
        self.function_locations.get(function_name)
    }

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
    pub max_frames: usize,
    /// Whether to capture Rust backtraces
    pub capture_rust_backtrace: bool,
    /// Whether to capture local variables
    pub capture_variables: bool,
    /// Whether to capture function parameters
    pub capture_parameters: bool,
    /// Whether to resolve symbols
    pub resolve_symbols: bool,
    /// Maximum variable value length to capture
    pub max_variable_length: usize,
}

impl Default for StackTraceConfig {
    fn default() -> Self {
        StackTraceConfig {
            max_frames: 100,
            capture_rust_backtrace: true,
            capture_variables: false, // Can be expensive
            capture_parameters: true,
            resolve_symbols: true,
            max_variable_length: 1000,
        }
    }
}

/// Per-thread call stack state
struct ThreadCallStack {
    /// Current call frames
    frames: Vec<CallFrame>,
    /// Function entry timestamps
    entry_times: Vec<SystemTime>,
    /// Whether currently capturing trace
    capturing: bool,
}

impl ThreadCallStack {
    fn new() -> Self {
        ThreadCallStack {
            frames: Vec::new(),
            entry_times: Vec::new(),
            capturing: false,
        }
    }
}

/// Stack trace management system
pub struct StackTraceManager {
    /// Configuration for trace capture
    config: Arc<RwLock<StackTraceConfig>>,
    /// Per-thread call stacks
    thread_stacks: Arc<Mutex<HashMap<ThreadId, ThreadCallStack>>>,
    /// Debug information for symbol resolution
    debug_info: Arc<RwLock<DebugInfo>>,
    /// Statistics
    stats: Arc<Mutex<StackTraceStatistics>>,
}

/// Stack trace statistics
#[derive(Debug, Default, Clone)]
pub struct StackTraceStatistics {
    /// Total traces captured
    pub total_traces: u64,
    /// Average trace depth
    pub average_depth: f64,
    /// Maximum trace depth seen
    pub max_depth: usize,
    /// Number of truncated traces
    pub truncated_traces: u64,
    /// Total time spent capturing traces
    pub total_capture_time: std::time::Duration,
}

impl StackTraceManager {
    /// Create a new stack trace manager
    pub fn new() -> Self {
        StackTraceManager {
            config: Arc::new(RwLock::new(StackTraceConfig::default())),
            thread_stacks: Arc::new(Mutex::new(HashMap::new())),
            debug_info: Arc::new(RwLock::new(DebugInfo::new())),
            stats: Arc::new(Mutex::new(StackTraceStatistics::default())),
        }
    }

    /// Create manager with custom configuration
    pub fn with_config(config: StackTraceConfig) -> Self {
        StackTraceManager {
            config: Arc::new(RwLock::new(config)),
            thread_stacks: Arc::new(Mutex::new(HashMap::new())),
            debug_info: Arc::new(RwLock::new(DebugInfo::new())),
            stats: Arc::new(Mutex::new(StackTraceStatistics::default())),
        }
    }

    /// Enter a function (push frame onto stack)
    #[instrument(skip(self))]
    pub fn enter_function(
        &self,
        function_name: String,
        module_name: Option<String>,
        source_location: Option<SourceLocation>,
        parameters: HashMap<String, String>,
    ) -> Result<(), CursedError> {
        let thread_id = thread::current().id();
        
        if let Ok(mut stacks) = self.thread_stacks.lock() {
            let stack = stacks.entry(thread_id).or_insert_with(ThreadCallStack::new);
            
            let depth = stack.frames.len();
            let mut frame = CallFrame::new(function_name.clone(), depth);
            
            if let Some(location) = source_location {
                frame = frame.with_location(location);
            }
            
            if let Some(module) = module_name {
                frame = frame.with_module(module);
            }
            
            for (name, value) in parameters {
                frame = frame.with_parameter(name, value);
            }
            
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
    pub fn exit_function(&self, function_name: Option<String>) -> Result<(), CursedError> {
        let thread_id = thread::current().id();
        
        if let Ok(mut stacks) = self.thread_stacks.lock() {
            if let Some(stack) = stacks.get_mut(&thread_id) {
                if let Some(frame) = stack.frames.pop() {
                    stack.entry_times.pop();
                    
                    if let Some(expected_name) = function_name {
                        if frame.function_name != expected_name {
                            warn!(
                                "Function name mismatch: expected {}, got {}",
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
    pub fn capture_stack_trace(&self, goroutine_id: Option<u64>) -> Result<StackTrace, CursedError> {
        let start_time = std::time::Instant::now();
        let thread_id = thread::current().id();
        
        let config = self.config.read().unwrap();
        let mut trace = StackTrace::new(config.max_frames);
        
        if let Some(gid) = goroutine_id {
            trace = trace.with_goroutine(gid);
        }
        
        // Capture Rust backtrace if configured
        if config.capture_rust_backtrace {
            let backtrace = Backtrace::capture();
            trace = trace.with_rust_backtrace(backtrace);
        }
        
        // Copy current stack frames
        if let Ok(stacks) = self.thread_stacks.lock() {
            if let Some(stack) = stacks.get(&thread_id) {
                // Copy frames in reverse order (most recent first)
                for frame in stack.frames.iter().rev() {
                    let mut trace_frame = frame.clone();
                    
                    // Add local variables if configured
                    if config.capture_variables {
                        trace_frame = self.capture_local_variables(trace_frame)?;
                    }
                    
                    // Resolve symbols if configured
                    if config.resolve_symbols {
                        trace_frame = self.resolve_frame_symbols(trace_frame)?;
                    }
                    
                    trace.add_frame(trace_frame);
                }
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
            }
            
            if trace.truncated {
                stats.truncated_traces += 1;
            }
            
            // Update average depth
            let total_depth = stats.average_depth * (stats.total_traces - 1) as f64 + frame_count as f64;
            stats.average_depth = total_depth / stats.total_traces as f64;
        }
        
        debug!("Captured stack trace with {} frames", trace.frames.len());
        Ok(trace)
    }

    /// Add debug information for symbol resolution
    pub fn add_debug_info(&self, function_name: String, location: SourceLocation, module: Option<String>) -> Result<(), CursedError> {
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
    }

    /// Get stack trace statistics
    pub fn get_statistics(&self) -> Result<StackTraceStatistics, CursedError> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::Runtime("Failed to access stack trace statistics".to_string()))
    }

    /// Update configuration
    pub fn update_config<F>(&self, updater: F) -> Result<(), CursedError>
    where
        F: FnOnce(&mut StackTraceConfig),
    {
        if let Ok(mut config) = self.config.write() {
            updater(&mut *config);
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to update stack trace configuration".to_string()))
        }
    }

    // Helper methods

    fn capture_local_variables(&self, mut frame: CallFrame) -> Result<CallFrame, CursedError> {
        // Placeholder implementation - in a real system this would inspect
        // the current execution context for local variables
        
        // For now, just add some mock variables as an example
        frame.local_variables.insert("example_var".to_string(), "example_value".to_string());
        
        Ok(frame)
    }

    fn resolve_frame_symbols(&self, mut frame: CallFrame) -> Result<CallFrame, CursedError> {
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
    function_ptr: *const u8,
    function_len: usize,
    module_ptr: *const u8,
    module_len: usize,
    line: u32,
    column: u32,
    file_ptr: *const u8,
    file_len: usize,
) {
    let function_name = if function_ptr.is_null() || function_len == 0 {
        "unknown".to_string()
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(function_ptr, function_len);
            String::from_utf8_lossy(slice).to_string()
        }
    };

    let module_name = if module_ptr.is_null() || module_len == 0 {
        None
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(module_ptr, module_len);
            Some(String::from_utf8_lossy(slice).to_string())
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

    // Create a default stack trace manager for this call
    // In practice, this would use a global instance
    let manager = StackTraceManager::new();
    let _ = manager.enter_function(function_name, module_name, source_location, HashMap::new());
}

/// Exit a function from compiled code
#[no_mangle]
pub extern "C" fn cursed_stack_exit_function(
    function_ptr: *const u8,
    function_len: usize,
) {
    let function_name = if function_ptr.is_null() || function_len == 0 {
        None
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(function_ptr, function_len);
            Some(String::from_utf8_lossy(slice).to_string())
        }
    };

    let manager = StackTraceManager::new();
    let _ = manager.exit_function(function_name);
}

/// Get current call depth
#[no_mangle]
pub extern "C" fn cursed_get_call_depth() -> u32 {
    let manager = StackTraceManager::new();
    manager.get_call_depth() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_frame_creation() {
        let frame = CallFrame::new("test_function".to_string(), 0)
            .with_location(SourceLocation::new(10, 5))
            .with_module("test_module".to_string())
            .with_parameter("arg1".to_string(), "value1".to_string());

        assert_eq!(frame.function_name, "test_function");
        assert_eq!(frame.depth, 0);
        assert!(frame.source_location.is_some());
        assert_eq!(frame.module_name, Some("test_module".to_string()));
        assert_eq!(frame.parameters.get("arg1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_stack_trace_creation() {
        let mut trace = StackTrace::new(10);
        
        let frame1 = CallFrame::new("func1".to_string(), 0);
        let frame2 = CallFrame::new("func2".to_string(), 1);
        
        trace.add_frame(frame1);
        trace.add_frame(frame2);

        assert_eq!(trace.frames.len(), 2);
        assert_eq!(trace.top_frame().unwrap().function_name, "func1");
        assert_eq!(trace.bottom_frame().unwrap().function_name, "func2");
    }

    #[test]
    fn test_debug_info() {
        let mut debug_info = DebugInfo::new();
        
        let location = SourceLocation::new(15, 20).with_file("test.csd");
        debug_info.add_function("test_func".to_string(), location.clone());
        debug_info.add_module("test_module".to_string(), "test.csd".to_string());
        debug_info.add_symbol(0x1000, "symbol_func".to_string());

        assert_eq!(debug_info.resolve_function_location("test_func"), Some(&location));
        assert_eq!(debug_info.resolve_symbol(0x1000), Some(&"symbol_func".to_string()));
    }

    #[test]
    fn test_stack_trace_manager() {
        let manager = StackTraceManager::new();
        
        // Test function entry/exit
        let params = HashMap::new();
        assert!(manager.enter_function(
            "test_func".to_string(),
            Some("test_module".to_string()),
            Some(SourceLocation::new(10, 5)),
            params
        ).is_ok());

        assert_eq!(manager.get_call_depth(), 1);
        assert_eq!(manager.get_current_function(), Some("test_func".to_string()));

        assert!(manager.exit_function(Some("test_func".to_string())).is_ok());
        assert_eq!(manager.get_call_depth(), 0);
    }

    #[test]
    fn test_stack_trace_capture() {
        let manager = StackTraceManager::new();
        
        // Set up some function calls
        let params = HashMap::new();
        let _ = manager.enter_function(
            "func1".to_string(),
            Some("module1".to_string()),
            Some(SourceLocation::new(10, 5)),
            params.clone()
        );
        
        let _ = manager.enter_function(
            "func2".to_string(),
            Some("module2".to_string()),
            Some(SourceLocation::new(20, 10)),
            params
        );

        // Capture trace
        let trace = manager.capture_stack_trace(Some(123)).unwrap();
        
        assert_eq!(trace.frames.len(), 2);
        assert_eq!(trace.goroutine_id, Some(123));
        assert_eq!(trace.frames[0].function_name, "func2"); // Most recent first
        assert_eq!(trace.frames[1].function_name, "func1");
    }

    #[test]
    fn test_trace_truncation() {
        let config = StackTraceConfig {
            max_frames: 2,
            ..Default::default()
        };
        let manager = StackTraceManager::with_config(config);
        
        // Add more frames than max
        let params = HashMap::new();
        for i in 0..5 {
            let _ = manager.enter_function(
                format!("func{}", i),
                None,
                None,
                params.clone()
            );
        }

        let trace = manager.capture_stack_trace(None).unwrap();
        assert_eq!(trace.frames.len(), 2);
        assert!(trace.truncated);
    }

    #[test]
    fn test_statistics() {
        let manager = StackTraceManager::new();
        
        let stats = manager.get_statistics().unwrap();
        assert_eq!(stats.total_traces, 0);

        // Capture a trace
        let _ = manager.capture_stack_trace(None);

        let updated_stats = manager.get_statistics().unwrap();
        assert_eq!(updated_stats.total_traces, 1);
    }
}
