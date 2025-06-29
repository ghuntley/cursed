//! Stack trace implementation for CURSED runtime
//!
//! Provides stack tracing capabilities for debugging and error handling,
//! including call stack capture, frame analysis, and debug information.

use crate::error_types::{Error, Result as CursedResult};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

/// Represents a single frame in the call stack
#[derive(Debug, Clone, PartialEq)]
pub struct StackFrame {
    /// Function name
    pub function_name: String,
    /// Source file path
    pub file: Option<String>,
    /// Line number in source
    pub line: Option<usize>,
    /// Column number in source
    pub column: Option<usize>,
    /// Function arguments (if available)
    pub arguments: Vec<String>,
    /// Local variables (if available)
    pub locals: HashMap<String, String>,
    /// Frame type (function, method, closure, etc.)
    pub frame_type: FrameType,
    /// Memory address of the frame
    pub frame_address: Option<usize>,
}

/// Type of stack frame
#[derive(Debug, Clone, PartialEq)]
pub enum FrameType {
    /// Regular function call
    Function,
    /// Method call on an object
    Method,
    /// Closure execution
    Closure,
    /// Goroutine entry point
    Goroutine,
    /// Native/foreign function
    Native,
    /// Built-in function
    Builtin,
}

impl fmt::Display for FrameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameType::Function => write!(f, "function"),
            FrameType::Method => write!(f, "method"),
            FrameType::Closure => write!(f, "closure"),
            FrameType::Goroutine => write!(f, "goroutine"),
            FrameType::Native => write!(f, "native"),
            FrameType::Builtin => write!(f, "builtin"),
        }
    }
}

impl StackFrame {
    /// Create a new stack frame
    pub fn new(function_name: String, frame_type: FrameType) -> Self {
        Self {
            function_name,
            file: None,
            line: None,
            column: None,
            arguments: Vec::new(),
            locals: HashMap::new(),
            frame_type,
            frame_address: None,
        }
    }

    /// Create a frame with source location
    pub fn with_location(
        function_name: String,
        frame_type: FrameType,
        file: String,
        line: usize,
        column: Option<usize>,
    ) -> Self {
        Self {
            function_name,
            file: Some(file),
            line: Some(line),
            column,
            arguments: Vec::new(),
            locals: HashMap::new(),
            frame_type,
            frame_address: None,
        }
    }

    /// Add an argument to the frame
    pub fn add_argument(&mut self, arg: String) {
        self.arguments.push(arg);
    }

    /// Add a local variable to the frame
    pub fn add_local(&mut self, name: String, value: String) {
        self.locals.insert(name, value);
    }

    /// Set the frame address
    pub fn set_address(&mut self, address: usize) {
        self.frame_address = Some(address);
    }

    /// Get formatted location string
    pub fn location_string(&self) -> String {
        match (&self.file, &self.line, &self.column) {
            (Some(file), Some(line), Some(col)) => format!("{}:{}:{}", file, line, col),
            (Some(file), Some(line), None) => format!("{}:{}", file, line),
            (Some(file), None, None) => file.clone(),
            _ => "<unknown>".to_string(),
        }
    }

    /// Get formatted function signature
    pub fn signature_string(&self) -> String {
        if self.arguments.is_empty() {
            format!("{}()", self.function_name)
        } else {
            format!("{}({})", self.function_name, self.arguments.join(", "))
        }
    }
}

impl fmt::Display for StackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let location = self.location_string();
        let signature = self.signature_string();
        
        if location == "<unknown>" {
            write!(f, "{} [{}]", signature, self.frame_type)
        } else {
            write!(f, "{} at {} [{}]", signature, location, self.frame_type)
        }
    }
}

/// Complete stack trace representation
#[derive(Debug, Clone)]
pub struct StackTrace {
    /// Stack frames from bottom to top
    pub frames: Vec<StackFrame>,
    /// Thread ID where trace was captured
    pub thread_id: Option<usize>,
    /// Timestamp when trace was captured
    pub timestamp: std::time::Instant,
    /// Additional context information
    pub context: HashMap<String, String>,
}

impl StackTrace {
    /// Create a new empty stack trace
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            thread_id: None,
            timestamp: std::time::Instant::now(),
            context: HashMap::new(),
        }
    }

    /// Create a stack trace with frames
    pub fn with_frames(frames: Vec<StackFrame>) -> Self {
        Self {
            frames,
            thread_id: None,
            timestamp: std::time::Instant::now(),
            context: HashMap::new(),
        }
    }

    /// Add a frame to the top of the stack
    pub fn push_frame(&mut self, frame: StackFrame) {
        self.frames.push(frame);
    }

    /// Remove the top frame from the stack
    pub fn pop_frame(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }

    /// Get the top frame (current function)
    pub fn top_frame(&self) -> Option<&StackFrame> {
        self.frames.last()
    }

    /// Get all frames
    pub fn all_frames(&self) -> &[StackFrame] {
        &self.frames
    }

    /// Get frame count
    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    /// Set thread ID
    pub fn set_thread_id(&mut self, thread_id: usize) {
        self.thread_id = Some(thread_id);
    }

    /// Add context information
    pub fn add_context(&mut self, key: String, value: String) {
        self.context.insert(key, value);
    }

    /// Get formatted stack trace string
    pub fn format_trace(&self) -> String {
        let mut result = String::new();
        
        // Add header with thread info if available
        if let Some(thread_id) = self.thread_id {
            result.push_str(&format!("Stack trace (thread {}):\\n", thread_id));
        } else {
            result.push_str("Stack trace:\\n");
        }

        // Add each frame (reverse order for traditional stack trace display)
        for (i, frame) in self.frames.iter().rev().enumerate() {
            result.push_str(&format!("  #{}: {}\\n", i, frame));
        }

        // Add context if available
        if !self.context.is_empty() {
            result.push_str("\\nContext:\\n");
            for (key, value) in &self.context {
                result.push_str(&format!("  {}: {}\\n", key, value));
            }
        }

        result
    }

    /// Get frames matching a pattern
    pub fn find_frames(&self, pattern: &str) -> Vec<&StackFrame> {
        self.frames
            .iter()
            .filter(|frame| {
                frame.function_name.contains(pattern) ||
                frame.file.as_ref().map_or(false, |f| f.contains(pattern))
            })
            .collect()
    }

    /// Check if trace contains a specific function
    pub fn contains_function(&self, function_name: &str) -> bool {
        self.frames.iter().any(|frame| frame.function_name == function_name)
    }

    /// Get the deepest function call
    pub fn deepest_function(&self) -> Option<&StackFrame> {
        self.frames.first()
    }

    /// Filter frames by type
    pub fn frames_by_type(&self, frame_type: FrameType) -> Vec<&StackFrame> {
        self.frames
            .iter()
            .filter(|frame| frame.frame_type == frame_type)
            .collect()
    }
}

impl Default for StackTrace {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StackTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_trace())
    }
}

/// Stack trace collector and manager
pub struct StackTraceCollector {
    /// Current active traces by thread
    active_traces: Mutex<HashMap<usize, StackTrace>>,
    /// Maximum stack depth to collect
    max_depth: usize,
    /// Whether to collect local variables
    collect_locals: bool,
    /// Whether to collect function arguments
    collect_arguments: bool,
    /// Statistics
    stats: Mutex<StackTraceStats>,
}

/// Statistics for stack trace operations
#[derive(Debug, Default, Clone)]
pub struct StackTraceStats {
    pub traces_created: usize,
    pub frames_collected: usize,
    pub max_depth_reached: usize,
    pub average_depth: f64,
    pub collection_time_ms: f64,
}

impl StackTraceCollector {
    /// Create a new stack trace collector
    pub fn new() -> Self {
        Self {
            active_traces: Mutex::new(HashMap::new()),
            max_depth: 100, // Default maximum depth
            collect_locals: false, // Expensive, disabled by default
            collect_arguments: true,
            stats: Mutex::new(StackTraceStats::default()),
        }
    }

    /// Create collector with configuration
    pub fn with_config(max_depth: usize, collect_locals: bool, collect_arguments: bool) -> Self {
        Self {
            active_traces: Mutex::new(HashMap::new()),
            max_depth,
            collect_locals,
            collect_arguments,
            stats: Mutex::new(StackTraceStats::default()),
        }
    }

    /// Start tracing for a thread
    pub fn start_trace(&self, thread_id: usize) -> CursedResult<()> {
        let mut traces = self.active_traces.lock().map_err(|_| {
            Error::Runtime("Failed to acquire trace lock".to_string())
        })?;
        
        let mut trace = StackTrace::new();
        trace.set_thread_id(thread_id);
        traces.insert(thread_id, trace);
        
        let mut stats = self.stats.lock().unwrap();
        stats.traces_created += 1;
        
        Ok(())
    }

    /// Stop tracing for a thread and return the trace
    pub fn stop_trace(&self, thread_id: usize) -> CursedResult<Option<StackTrace>> {
        let mut traces = self.active_traces.lock().map_err(|_| {
            Error::Runtime("Failed to acquire trace lock".to_string())
        })?;
        
        Ok(traces.remove(&thread_id))
    }

    /// Push a frame onto the current thread's trace
    pub fn push_frame(&self, thread_id: usize, frame: StackFrame) -> CursedResult<()> {
        let mut traces = self.active_traces.lock().map_err(|_| {
            Error::Runtime("Failed to acquire trace lock".to_string())
        })?;
        
        if let Some(trace) = traces.get_mut(&thread_id) {
            if trace.depth() < self.max_depth {
                trace.push_frame(frame);
                
                let mut stats = self.stats.lock().unwrap();
                stats.frames_collected += 1;
                if trace.depth() > stats.max_depth_reached {
                    stats.max_depth_reached = trace.depth();
                }
            }
        }
        
        Ok(())
    }

    /// Pop a frame from the current thread's trace
    pub fn pop_frame(&self, thread_id: usize) -> CursedResult<Option<StackFrame>> {
        let mut traces = self.active_traces.lock().map_err(|_| {
            Error::Runtime("Failed to acquire trace lock".to_string())
        })?;
        
        if let Some(trace) = traces.get_mut(&thread_id) {
            Ok(trace.pop_frame())
        } else {
            Ok(None)
        }
    }

    /// Get current trace for a thread
    pub fn get_trace(&self, thread_id: usize) -> CursedResult<Option<StackTrace>> {
        let traces = self.active_traces.lock().map_err(|_| {
            Error::Runtime("Failed to acquire trace lock".to_string())
        })?;
        
        Ok(traces.get(&thread_id).cloned())
    }

    /// Capture current stack trace (simplified implementation)
    pub fn capture_trace(&self) -> CursedResult<StackTrace> {
        let start_time = std::time::Instant::now();
        let mut trace = StackTrace::new();
        
        // This is a simplified implementation
        // In a real implementation, you would walk the actual call stack
        // using platform-specific APIs or debug information
        
        // For now, create a placeholder frame
        let frame = StackFrame::new(
            "capture_trace".to_string(),
            FrameType::Function
        );
        trace.push_frame(frame);
        
        let elapsed = start_time.elapsed();
        {
            let mut stats = self.stats.lock().unwrap();
            stats.collection_time_ms += elapsed.as_secs_f64() * 1000.0;
        }
        
        Ok(trace)
    }

    /// Get collector statistics
    pub fn get_stats(&self) -> StackTraceStats {
        self.stats.lock().unwrap().clone()
    }

    /// Clear all active traces
    pub fn clear_all_traces(&self) {
        let mut traces = self.active_traces.lock().unwrap();
        traces.clear();
    }

    /// Get count of active traces
    pub fn active_trace_count(&self) -> usize {
        self.active_traces.lock().unwrap().len()
    }
}

impl Default for StackTraceCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Global stack trace collector instance
static GLOBAL_STACK_TRACE_COLLECTOR: std::sync::LazyLock<StackTraceCollector> = 
    std::sync::LazyLock::new(|| StackTraceCollector::new());

/// Get the global stack trace collector
pub fn get_global_stack_trace_collector() -> &'static StackTraceCollector {
    &GLOBAL_STACK_TRACE_COLLECTOR
}

/// Utility functions for common stack trace operations
pub mod utils {
    use super::*;

    /// Create a stack frame for a function call
    pub fn create_function_frame(
        name: &str,
        file: Option<&str>,
        line: Option<usize>,
        args: &[&str],
    ) -> StackFrame {
        let mut frame = if let (Some(f), Some(l)) = (file, line) {
            StackFrame::with_location(
                name.to_string(),
                FrameType::Function,
                f.to_string(),
                l,
                None,
            )
        } else {
            StackFrame::new(name.to_string(), FrameType::Function)
        };

        for arg in args {
            frame.add_argument(arg.to_string());
        }

        frame
    }

    /// Create a stack frame for a method call
    pub fn create_method_frame(
        class_name: &str,
        method_name: &str,
        file: Option<&str>,
        line: Option<usize>
    ) -> StackFrame {
        let full_name = format!("{}.{}", class_name, method_name);
        
        if let (Some(f), Some(l)) = (file, line) {
            StackFrame::with_location(
                full_name,
                FrameType::Method,
                f.to_string(),
                l,
                None,
            )
        } else {
            StackFrame::new(full_name, FrameType::Method)
        }
    }

    /// Format a minimal stack trace for error messages
    pub fn format_error_trace(trace: &StackTrace, max_frames: usize) -> String {
        let frames_to_show = std::cmp::min(max_frames, trace.depth());
        let mut result = String::new();
        
        for frame in trace.frames.iter().rev().take(frames_to_show) {
            if !result.is_empty() {
                result.push_str(" <- ");
            }
            result.push_str(&frame.function_name);
        }
        
        if trace.depth() > max_frames {
            result.push_str(&format!(" (... {} more frames)", trace.depth() - max_frames));
        }
        
        result
    }

    /// Get the current thread ID (simplified implementation)
    pub fn current_thread_id() -> usize {
        // In a real implementation, this would return the actual thread ID
        // For now, use a hash of the thread ID since as_u64() is unstable
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        std::thread::current().id().hash(&mut hasher);
        hasher.finish() as usize
    }
}

/// Macro for automatic stack frame tracking
#[macro_export]
macro_rules! trace_function {
    ($name:expr) => {
        let _guard = $crate::runtime::stack_trace::FunctionTraceGuard::new($name);
    };
    ($name:expr, $file:expr, $line:expr) => {
        let _guard = $crate::runtime::stack_trace::FunctionTraceGuard::with_location($name, $file, $line);
    };
}

/// RAII guard for automatic function tracing
pub struct FunctionTraceGuard {
    thread_id: usize,
    pushed: bool,
}

impl FunctionTraceGuard {
    /// Create a new trace guard
    pub fn new(function_name: &str) -> Self {
        let thread_id = utils::current_thread_id();
        let frame = StackFrame::new(function_name.to_string(), FrameType::Function);
        
        let collector = get_global_stack_trace_collector();
        let pushed = collector.push_frame(thread_id, frame).is_ok();
        
        Self { thread_id, pushed }
    }

    /// Create a trace guard with location
    pub fn with_location(function_name: &str, file: &str, line: usize) -> Self {
        let thread_id = utils::current_thread_id();
        let frame = StackFrame::with_location(
            function_name.to_string(),
            FrameType::Function,
            file.to_string(),
            line,
            None,
        );
        
        let collector = get_global_stack_trace_collector();
        let pushed = collector.push_frame(thread_id, frame).is_ok();
        
        Self { thread_id, pushed }
    }
}

impl Drop for FunctionTraceGuard {
    fn drop(&mut self) {
        if self.pushed {
            let collector = get_global_stack_trace_collector();
            let _ = collector.pop_frame(self.thread_id);
        }
    }
}

/// Public function that was likely being used by external code
pub fn get_minimal_result() -> CursedResult<String> {
    Ok("CURSED stack trace system initialized".to_string())
}
